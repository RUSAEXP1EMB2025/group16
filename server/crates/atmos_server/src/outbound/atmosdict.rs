use atmos_dict::{Atmosdict, error::AtmosdictError};
use std::{collections::HashSet, sync::Arc};

use crate::domain::ports::AtmosdictRepository;

impl AtmosdictRepository for Arc<Atmosdict> {
    async fn get_all_atmoswords(&self) -> Result<HashSet<String>, AtmosdictError> {
        self.get_all().await
    }

    async fn get_pos_neg_atmoswords(
        &self,
    ) -> Result<(HashSet<String>, HashSet<String>), AtmosdictError> {
        self.get_pos_neg().await
    }
}
