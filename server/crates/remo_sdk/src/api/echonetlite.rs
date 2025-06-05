use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{
        call_1_echonetlite_appliances_applianceid_refresh_post,
        call_1_echonetlite_appliances_applianceid_set_post, call_1_echonetlite_appliances_get,
    },
    models::EchonetLiteApplianceResponse,
};

impl Client {
    /// Fetch the list of ECHONET Lite appliances.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_echonet_lite_appliances(&self) -> APIResult<EchonetLiteApplianceResponse> {
        let response = call_1_echonetlite_appliances_get(self).await?;
        Ok(response)
    }

    /// Notify Remo E / Remo E lite to refresh one or more ECHONET Lite properties.
    ///
    /// This endpoint is subject to ECHONET Lite specific rate limiting. See the rate limiting section of the documentation for more details.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `echonetlite.*.read`
    ///
    #[inline]
    pub async fn refresh_echonet_lite(
        &self,
        appliance_id: &str,
        epc: Option<&str>,
    ) -> APIResult<()> {
        call_1_echonetlite_appliances_applianceid_refresh_post(self, appliance_id, epc).await?;
        Ok(())
    }

    /// Set one ECHONET Lite property.
    ///
    /// This endpoint is subject to ECHONET Lite specific rate limiting. See the rate limiting section of the documentation for more details.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `echonetlite.*.write`
    ///
    #[inline]
    pub async fn set_echonet_lite_property(
        &self,
        appliance_id: &str,
        epc: Option<&str>,
        val: Option<&str>,
    ) -> APIResult<()> {
        call_1_echonetlite_appliances_applianceid_set_post(self, appliance_id, epc, val).await?;
        Ok(())
    }
}
