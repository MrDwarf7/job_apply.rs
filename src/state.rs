use fantoccini::{Client, ClientBuilder, Locator, client};

use crate::config::{AppConfig, ProcessHandleExt, start_driver};
use crate::prelude::*;

pub trait CloseClient {
    fn close_client(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

#[derive(Debug)]
pub struct State {
    pub config: AppConfig,
    pub client: Client,
}

impl State {
    pub fn new(config: AppConfig, client: Client) -> Self {
        Self { config, client }
    }
}

impl ProcessHandleExt for State {
    async fn abort_handle(&self) {
        self.config.driver.driver_process.as_ref().map(|dp| {
            Box::pin(async move {
                debug!("STATE :: Aborting driver process handle...");
                dp.abort_handle().await;
            })
        });
    }

    async fn send_stop_signal(&self) {
        self.config.driver.driver_process.as_ref().map(|dp| {
            Box::pin(async move {
                debug!("STATE :: Sending stop signal to driver process...");
                dp.send_stop_signal().await;
            })
        });
    }
}

impl CloseClient for State {
    async fn close_client(&self) -> Result<()> {
        let fut = Box::pin(async move {
            {
                debug!("Closing fantoccini client...");
                let client = self.client.clone();
                client.close().await.unwrap_or_else(|e| {
                    error!("Failed to close client: {}", e);
                });
            }
        });
        fut.await;
        Ok(())
    }
}

impl TryFrom<(AppConfig, Client)> for State {
    type Error = Error;

    fn try_from(value: (AppConfig, Client)) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            config: value.0,
            client: value.1,
        })
    }
}
