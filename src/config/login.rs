use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginConfig {
    pub login_url: String,
    pub username:  String,
    pub password:  String,
}

impl LoginConfig {
    fn new(username: String, password: String, login_url: String) -> Self {
        Self {
            username,
            password,
            login_url,
        }
    }
}

impl Default for LoginConfig {
    fn default() -> Self {
        let maybe_env_username = std::env::var("APP_USERNAME").ok();
        let maybe_env_password = std::env::var("APP_PASSWORD").ok();
        let maybe_env_login_url = std::env::var("APP_LOGIN_URL").ok();

        if let (Some(username), Some(password), Some(login_url)) =
            (maybe_env_username, maybe_env_password, maybe_env_login_url)
        {
            Self::new(username, password, login_url)
        } else {
            Self::new(
                String::from("your_username_here"),
                String::from("your_password_here"),
                String::from("put_login_url_here"),
            )
        }
    }
}
