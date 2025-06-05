use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{call_1_users_me_get, call_1_users_me_post},
    models::UserResponse,
};

impl Client {
    /// Fetch the authenticated user's information.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_user(&self) -> APIResult<UserResponse> {
        let response = call_1_users_me_get(self).await?;
        Ok(response)
    }

    /// Update authenticated user's information.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_user(
        &self,
        country: Option<&str>,
        distance: Option<&str>,
        nickname: Option<&str>,
        temp: Option<&str>,
    ) -> APIResult<UserResponse> {
        let response = call_1_users_me_post(self, country, distance, nickname, temp).await?;
        Ok(response)
    }
}
