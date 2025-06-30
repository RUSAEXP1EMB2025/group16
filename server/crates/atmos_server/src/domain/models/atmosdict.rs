use atmos_dict::error::AtmosdictError;

use crate::inbound::http::api::ApiError;

impl From<AtmosdictError> for ApiError {
    fn from(e: AtmosdictError) -> Self {
        ApiError::InternalServer(e.to_string())
    }
}
