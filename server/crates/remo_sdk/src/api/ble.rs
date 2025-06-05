use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{
        call_1_ble_appliances_applianceid_private_macros_get,
        call_1_ble_private_macros_privatemacroid_exec_post,
    },
    models::BlePrivateMacroResponse,
};

impl Client {
    /// Fetch the list of BLE private macros.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    #[inline]
    pub async fn get_ble_private_macros(
        &self,
        appliance_id: &str,
    ) -> APIResult<Vec<BlePrivateMacroResponse>> {
        let response =
            call_1_ble_appliances_applianceid_private_macros_get(self, appliance_id).await?;
        Ok(response)
    }

    /// Send a BLE private macro control request.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `sendir`
    #[inline]
    pub async fn send_ble_private_macro_control_request(
        &self,
        private_macro_id: &str,
    ) -> APIResult<()> {
        call_1_ble_private_macros_privatemacroid_exec_post(self, private_macro_id).await?;
        Ok(())
    }
}
