use std::ops::Deref;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use fantoccini::{Client, ClientBuilder};
use futures::FutureExt; // For .fuse()
use serde::{Deserialize, Serialize};
use tokio::net::unix::SocketAddr;
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

use crate::config::ProcessHandleExt;
use crate::config::driver_type::DriverType;
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverConfig {
    pub driver_type: DriverType,
    pub driver_path: PathBuf,

    pub driver_port: Option<u16>, // we will default it to 4444

    pub driver_extra_args: Option<Vec<String>>,

    pub background_driver_check_delay_secs: Option<u64>,

    #[serde(skip)]
    pub driver_process: Option<DriverProcessHandle>,
}

#[derive(Debug, Clone)]
pub struct DriverProcessHandle {
    pub handle:  Arc<tokio::task::JoinHandle<()>>,
    pub stop_tx: Arc<tokio::sync::mpsc::Sender<()>>,
}

impl ProcessHandleExt for DriverProcessHandle {
    async fn abort_handle(&self) {
        // Abort the task
        Box::pin(async move {
            debug!("DRIVER_PROCESS_HANDLE :: Aborting driver process handle...");
            self.deref().abort()
        })
        .await;
    }

    async fn send_stop_signal(&self) {
        Box::pin(async move {
            debug!("DRIVER_PROCESS_HANDLE :: Sending stop signal to driver process...");
            self.stop_tx.send(()).await.unwrap_or_else(|e| {
                error!("Failed to send stop signal to driver process: {}", e);
            });
        })
        .await;
    }
}

impl Deref for DriverProcessHandle {
    type Target = tokio::task::JoinHandle<()>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl DriverProcessHandle {
    pub fn new(
        handle: tokio::task::JoinHandle<()>,
        stop_tx: tokio::sync::mpsc::Sender<()>,
    ) -> Self {
        Self {
            handle:  Arc::new(handle),
            stop_tx: Arc::new(stop_tx),
        }
    }
}

impl DriverConfig {
    pub fn new(driver_type: DriverType) -> Self {
        let driver_path = which::which(match driver_type {
            DriverType::Chrome | DriverType::Chromium => "chromedriver",
        })
        .unwrap_or_else(|_| panic!("Failed to find driver executable for {:?}", driver_type));

        Self {
            driver_type,
            driver_path,
            driver_port: None,
            driver_extra_args: None,
            background_driver_check_delay_secs: Some(1),
            driver_process: None,
        }
    }
}

impl Default for DriverConfig {
    fn default() -> Self {
        Self::new(DriverType::Chrome)
    }
}

async fn pre_check_driver_port(port: u16) -> bool {
    let local = format!("{}:{}", crate::constants::LOCAL_HOST_ADDR_STR, port);
    tokio::net::TcpListener::bind(local).await.is_err()
}

/// Starts the driver process based on the provided DriverConfig.
/// Additionally, updates the config.driver_process with the process handle.
pub async fn start_driver(driver: &mut DriverConfig) -> Result<Client> {
    // HACK: Should find a better way to do this properly
    if let Some(port) = driver.driver_port
        && pre_check_driver_port(port).await
    {
        return ClientBuilder::native()
            .connect(&format!("http://localhost:{}", port))
            .await
            .map_err(|e| {
                Error::Generic(format!(
                    "Failed to connect to existing driver at port {}: {}",
                    port, e
                ))
            });
    }

    let port = driver.driver_port.unwrap_or(4444);
    let mut cmd = tokio::process::Command::new(&driver.driver_path);
    // Assuming standard args for port; adjust based on driver_type if needed (e.g., match on driver_type)
    // cmd.arg("--port").arg(port.to_string());
    cmd.args(
        [
            driver.driver_extra_args.clone().unwrap_or_default(),
            vec![format!("--port={}", port)],
        ]
        .concat(),
    );

    let (stop_tx, stop_rx) = tokio::sync::mpsc::channel::<()>(1);
    // oneshot::channel::<()>();

    // TODO: Make this configurable via DriverConfig or a param (e.g., add pub check_interval_secs: u64 to DriverConfig)
    // let check_interval_secs: u64 = 1; // User-set external param; hardcoded for now
    let check_interval_secs: u64 = driver.background_driver_check_delay_secs.unwrap_or(5);

    let task = spawn_driver(cmd, stop_rx, check_interval_secs).await?;

    let process_handle = DriverProcessHandle::new(task, stop_tx);
    // dbg!(&process_handle);
    driver.driver_process = Some(process_handle);

    let client = ClientBuilder::native()
        .connect(&format!("http://localhost:{}", port))
        .await
        .map_err(|e| {
            Error::Generic(format!("Failed to connect to driver at port {}: {}", port, e))
        })?;

    Ok(client)
}

/// Actually spawns the driver process and monitors it in the background.
///
async fn spawn_driver(
    mut cmd: Command,
    mut stop_rx: tokio::sync::mpsc::Receiver<()>,
    check_interval_secs: u64,
) -> Result<JoinHandle<()>> {
    Ok(tokio::spawn(async move {
        let mut child = cmd
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to spawn driver process");

        let stop_rx = stop_rx.recv().fuse();
        tokio::pin!(stop_rx);
        let mut interval = tokio::time::interval(Duration::from_secs(check_interval_secs));

        // Skip the immediate first tick if desired; or not.
        interval.tick().await;

        (&mut stop_rx).await;
        loop {
            tokio::select! {

                _ = &mut stop_rx => {
                    // Signal received: terminate and wait

                    if let Err(e) = child.kill().await {
                        error!("Failed to kill driver process: {}", e);
                    }
                    match child.wait().await {
                        Ok(status) => info!("Driver process terminated with status: {:?}", status),
                        Err(e) => error!("Failed to wait on driver process after kill: {}", e),
                    }
                    break;
                }

                _ = interval.tick() => {
                    // Check if child exited unexpectedly (non-blocking)
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            if !status.success() {
                                error!("Driver process exited unexpectedly with non-zero status: {}", status);
                            } else {
                                info!("Driver process exited normally with status: {}", status);
                            }
                            break;
                        }
                        Ok(None) => {
                            // Still running, continue
                        }
                        Err(e) => {
                            error!("Error checking driver process status: {}", e);
                            break;
                        }
                    }
                }

                else => {
                    unreachable!("{}", "Unexpected branch in driver process monitoring loop.");
                }
            }
        }
        child
            .start_kill()
            .expect("Failed to start `kill` on child driver process");
        drop(child);
    }))
}
