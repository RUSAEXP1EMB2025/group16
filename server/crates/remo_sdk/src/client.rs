use remo_api::apis::configuration::{ApiKey, BasicAuth, Configuration};
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct Client(Configuration);

impl Deref for Client {
    type Target = Configuration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Client {
    #[inline]
    pub fn new(
        basic_auth: Option<BasicAuth>,
        oauth_access_token: Option<String>,
        bearer_access_token: Option<String>,
        api_key: Option<ApiKey>,
    ) -> Self {
        Client(Configuration {
            basic_auth,
            oauth_access_token,
            bearer_access_token,
            api_key,
            ..Default::default()
        })
    }
}
