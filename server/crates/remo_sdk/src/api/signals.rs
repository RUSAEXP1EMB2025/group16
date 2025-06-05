use crate::{client::Client, error::APIResult};
use remo_api::{
    apis::default_api::{
        call_1_appliances_applianceid_light_post,
        call_1_appliances_applianceid_light_projector_post,
        call_1_appliances_applianceid_signal_orders_post,
        call_1_appliances_applianceid_signals_get, call_1_appliances_applianceid_signals_post,
        call_1_appliances_applianceid_tv_post, call_1_signals_signalid_delete_post,
        call_1_signals_signalid_post, call_1_signals_signalid_send_post,
    },
    models::{LightState, Signal, TvState},
};

impl Client {
    /// Update a signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn update_signal(
        &self,
        signal_id: &str,
        image: Option<&str>,
        name: Option<&str>,
    ) -> APIResult<Signal> {
        let response = call_1_signals_signalid_post(self, signal_id, image, name).await?;
        Ok(response)
    }

    /// Delete a signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn delete_signal(&self, signal_id: &str) -> APIResult<()> {
        call_1_signals_signalid_delete_post(self, signal_id).await?;
        Ok(())
    }

    /// Send a signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `sendir`
    ///
    #[inline]
    pub async fn send_signal(&self, signal_id: &str) -> APIResult<()> {
        call_1_signals_signalid_send_post(self, signal_id).await?;
        Ok(())
    }

    /// Reorder signals.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn reorder_signals(
        &self,
        appliance_id: &str,
        signals: Option<&str>,
    ) -> APIResult<()> {
        call_1_appliances_applianceid_signal_orders_post(self, appliance_id, signals).await?;
        Ok(())
    }

    /// Fetch the list of signals.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.read`
    ///
    #[inline]
    pub async fn get_signals(&self, appliance_id: &str) -> APIResult<Vec<Signal>> {
        let response = call_1_appliances_applianceid_signals_get(self, appliance_id).await?;
        Ok(response)
    }

    /// Create a new signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `basic.write`
    ///
    #[inline]
    pub async fn create_signal(
        &self,
        appliance_id: &str,
        image: Option<&str>,
        message: Option<&str>,
        name: Option<&str>,
    ) -> APIResult<Signal> {
        let response =
            call_1_appliances_applianceid_signals_post(self, appliance_id, image, message, name)
                .await?;
        Ok(response)
    }

    /// Send light signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `sendir`
    ///
    #[inline]
    pub async fn send_signal_to_light(
        &self,
        appliance_id: &str,
        button: Option<&str>,
    ) -> APIResult<LightState> {
        let response = call_1_appliances_applianceid_light_post(self, appliance_id, button).await?;
        Ok(response)
    }

    /// Send light_projector signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `sendir`
    ///
    #[inline]
    pub async fn send_signal_to_light_projector(
        &self,
        appliance_id: &str,
        button: Option<&str>,
    ) -> APIResult<()> {
        call_1_appliances_applianceid_light_projector_post(self, appliance_id, button).await?;
        Ok(())
    }

    /// Send TV signal.
    ///
    /// # Requirements
    /// - OAuth2 scopes: `sendir`
    ///
    #[inline]
    pub async fn send_signal_to_tv(
        &self,
        appliance_id: &str,
        button: Option<&str>,
    ) -> APIResult<TvState> {
        let response = call_1_appliances_applianceid_tv_post(self, appliance_id, button).await?;
        Ok(response)
    }
}
