use atmos_dict::{Atmosdict, error::AtmosdictError};
use std::{collections::HashSet, sync::Arc};

use crate::domain::ports::AtmosdictRepository;

impl AtmosdictRepository for Arc<Atmosdict> {
    async fn get_all_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError> {
        self.get_all().await
    }

    async fn get_positive_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError> {
        self.get_positive().await
    }

    async fn get_negative_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError> {
        self.get_positive().await
    }
}
