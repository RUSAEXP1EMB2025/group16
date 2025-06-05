use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{
        call_1_devices_deviceid_appliances_get, call_1_devices_deviceid_delete_post,
        call_1_devices_deviceid_humidity_offset_post, call_1_devices_deviceid_post,
        call_1_devices_deviceid_temperature_offset_post, call_1_devices_get,
    },
    models::{ApplianceResponse, DeviceResponse},
};

impl Client {
    /// Fetch the list of Remo devices.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_devices(&self) -> APIResult<Vec<DeviceResponse>> {
        let response = call_1_devices_get(self).await?;
        Ok(response)
    }

    /// Update a device.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_device(
        &self,
        device_id: &str,
        name: Option<&str>,
    ) -> APIResult<DeviceResponse> {
        let response = call_1_devices_deviceid_post(self, device_id, name).await?;
        Ok(response)
    }

    /// Fetch the list of appliances registered to the Remo device.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_appliances_in_device(
        &self,
        device_id: &str,
    ) -> APIResult<Vec<ApplianceResponse>> {
        let response = call_1_devices_deviceid_appliances_get(self, device_id).await?;
        Ok(response)
    }

    /// Delete a device.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn delete_device(&self, device_id: &str) -> APIResult<()> {
        call_1_devices_deviceid_delete_post(self, device_id).await?;
        Ok(())
    }

    /// Update a device's humidity offset.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_device_humidity_offset(
        &self,
        device_id: &str,
        offset: Option<f32>,
    ) -> APIResult<DeviceResponse> {
        let response =
            call_1_devices_deviceid_humidity_offset_post(self, device_id, offset).await?;
        Ok(response)
    }

    /// Update a device's temperature offset.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_device_temperature_offset(
        &self,
        device_id: &str,
        offset: Option<f32>,
    ) -> APIResult<DeviceResponse> {
        let response =
            call_1_devices_deviceid_temperature_offset_post(self, device_id, offset).await?;
        Ok(response)
    }
}
