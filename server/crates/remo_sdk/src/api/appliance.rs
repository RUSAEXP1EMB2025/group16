use crate::{client::Client, error::APIResult, Deserializable};
use remo_api::{
    apis::default_api::{
        call_1_appliance_orders_post, call_1_appliances_applianceid_aircon_settings_post,
        call_1_appliances_applianceid_delete_post, call_1_appliances_applianceid_post,
        call_1_appliances_get, call_1_appliances_post, call_1_detectappliance_post,
    },
    models::{AirconSettingsResponse, ApplianceModelAndParam, ApplianceResponse},
};

impl Client {
    /// Reorder appliances.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn reorder_appliances(&self, appliances: Option<&str>) -> APIResult<()> {
        let response = call_1_appliance_orders_post(self, appliances).await?;
        response.deserialize().await
    }

    /// Fetch the list of appliances.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_appliances(&self) -> APIResult<Vec<ApplianceResponse>> {
        let response = call_1_appliances_get(self).await?;
        Ok(response)
    }

    /// Create a new appliance.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn create_appliance(
        &self,
        device: Option<&str>,
        image: Option<&str>,
        model: Option<&str>,
        model_type: Option<&str>,
        nickname: Option<&str>,
    ) -> APIResult<ApplianceResponse> {
        let response =
            call_1_appliances_post(self, device, image, model, model_type, nickname).await?;
        Ok(response)
    }

    /// Update an appliance.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_appliance(
        &self,
        appliance_id: &str,
        image: Option<&str>,
        nickname: Option<&str>,
    ) -> APIResult<ApplianceResponse> {
        let response =
            call_1_appliances_applianceid_post(self, appliance_id, image, nickname).await?;
        Ok(response)
    }

    /// Delete an appliance.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn delete_appliance(&self, appliance_id: &str) -> APIResult<()> {
        call_1_appliances_applianceid_delete_post(self, appliance_id).await?;
        Ok(())
    }

    #[inline]
    pub async fn update_aircon_settings(
        &self,
        applianceid: &str,
        air_direction: Option<&str>,
        air_direction_h: Option<&str>,
        air_volume: Option<&str>,
        button: Option<&str>,
        operation_mode: Option<&str>,
        temperature: Option<&str>,
        temperature_unit: Option<&str>,
    ) -> APIResult<AirconSettingsResponse> {
        let response = call_1_appliances_applianceid_aircon_settings_post(
            self,
            applianceid,
            air_direction,
            air_direction_h,
            air_volume,
            button,
            operation_mode,
            temperature,
            temperature_unit,
        )
        .await?;
        Ok(response)
    }

    /// Delete an appliance.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `detectappliance`
    ///
    #[inline]
    pub async fn find_best_matching_air_conditioner(
        &self,
        device: Option<&str>,
        message: Option<&str>,
    ) -> APIResult<Vec<ApplianceModelAndParam>> {
        let response = call_1_detectappliance_post(self, device, message).await?;
        Ok(response)
    }
}
