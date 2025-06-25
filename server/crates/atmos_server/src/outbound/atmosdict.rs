use atmos_dict::Atmosdict;

use crate::domain::{models::atmosdict::GetAtmosdictError, ports::AtmosdictRepository};

impl AtmosdictRepository for Atmosdict {
    async fn get_atmos_dict(&self) -> Result<Vec<String>, GetAtmosdictError> {
        self.get_all().map_err(|_| GetAtmosdictError::Sample)
    }
}
