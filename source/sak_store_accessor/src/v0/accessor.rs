use sak_mrs::SakMRS;
use std::sync::Arc;

pub struct StoreAccessor {
    mrs: SakMRS,
}

impl StoreAccessor {
    pub fn new(mrs: SakMRS) -> Self {
        StoreAccessor { mrs }
    }

    pub fn get_mrs_data(&self) -> usize {
        123123
    }
}
