use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{
        call_1_homes_get, call_1_homes_homeid_delete_post, call_1_homes_homeid_devices_get,
        call_1_homes_homeid_invites_post, call_1_homes_homeid_kick_post,
        call_1_homes_homeid_location_delete_post, call_1_homes_homeid_location_post,
        call_1_homes_homeid_location_state_update_post, call_1_homes_homeid_owner_post,
        call_1_homes_homeid_part_post, call_1_homes_homeid_post,
        call_1_homes_homeid_transfer_tohomeid_post, call_1_homes_homeid_users_get,
        call_1_homes_post, call_1_invites_invitetoken_get, call_1_invites_invitetoken_post,
    },
    models::{DeviceResponse, HomeInvite, HomeResponse, UserAndRole},
};

impl Client {
    /// Fetch the list of homes.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.read`
    ///
    #[inline]
    pub async fn get_homes(&self) -> APIResult<Vec<HomeResponse>> {
        let response = call_1_homes_get(self).await?;
        Ok(response)
    }

    /// Create a new home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn create_home(&self, name: Option<&str>) -> APIResult<HomeResponse> {
        let response = call_1_homes_post(self, name).await?;
        Ok(response)
    }

    /// Update a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn update_home(&self, home_id: &str, name: Option<&str>) -> APIResult<HomeResponse> {
        let response = call_1_homes_homeid_post(self, home_id, name).await?;
        Ok(response)
    }

    /// Delete a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn delete_home(&self, home_id: &str) -> APIResult<()> {
        call_1_homes_homeid_delete_post(self, home_id).await?;
        Ok(())
    }

    /// Fetch the list of devices in a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.read`
    ///
    #[inline]
    pub async fn get_devices_in_home(&self, home_id: &str) -> APIResult<Vec<DeviceResponse>> {
        let response = call_1_homes_homeid_devices_get(self, home_id).await?;
        Ok(response)
    }

    /// Create a new home invite.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn create_new_invite(&self, home_id: &str) -> APIResult<HomeInvite> {
        let response = call_1_homes_homeid_invites_post(self, home_id).await?;
        Ok(response)
    }

    /// Kick a user from a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn kick_user(&self, home_id: &str, user: Option<&str>) -> APIResult<()> {
        call_1_homes_homeid_kick_post(self, home_id, user).await?;
        Ok(())
    }

    /// Update a home's location.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn update_home_location(&self, home_id: &str) -> APIResult<HomeResponse> {
        let response = call_1_homes_homeid_location_post(self, home_id).await?;
        Ok(response)
    }

    /// Delete a home's location.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn delete_home_location(&self, home_id: &str) -> APIResult<HomeResponse> {
        let response = call_1_homes_homeid_location_delete_post(self, home_id).await?;
        Ok(response)
    }

    /// Update the user's location state for a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn update_user_location_state_in_home(&self, home_id: &str) -> APIResult<()> {
        call_1_homes_homeid_location_state_update_post(self, home_id).await?;
        Ok(())
    }

    /// Change the owner of the home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn change_owner(&self, home_id: &str, user: Option<&str>) -> APIResult<HomeResponse> {
        let response = call_1_homes_homeid_owner_post(self, home_id, user).await?;
        Ok(response)
    }

    /// Part from a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn part_from_home(&self, home_id: &str) -> APIResult<()> {
        call_1_homes_homeid_part_post(self, home_id).await?;
        Ok(())
    }

    /// Transfer devices to another home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn transfer_devices(
        &self,
        home_id: &str,
        to_home_id: &str,
        devices: Option<&str>,
    ) -> APIResult<()> {
        call_1_homes_homeid_transfer_tohomeid_post(self, home_id, to_home_id, devices).await?;
        Ok(())
    }

    /// Fetch the list of users in a home.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.read`
    ///
    #[inline]
    pub async fn get_users_in_home(&self, home_id: &str) -> APIResult<Vec<UserAndRole>> {
        let response = call_1_homes_homeid_users_get(self, home_id).await?;
        Ok(response)
    }

    /// Show a home invite.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.read`
    ///
    #[inline]
    pub async fn show_invite(&self, invite_token: &str) -> APIResult<HomeInvite> {
        let response = call_1_invites_invitetoken_get(self, invite_token).await?;
        Ok(response)
    }

    /// Accept a home invite.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `home.write`
    ///
    #[inline]
    pub async fn accept_invite(&self, invite_token: &str) -> APIResult<HomeResponse> {
        let response = call_1_invites_invitetoken_post(self, invite_token).await?;
        Ok(response)
    }
}
