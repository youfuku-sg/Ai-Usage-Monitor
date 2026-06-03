use std::sync::{Arc, RwLock};

use crate::models::AppUsageData;

pub type SharedState = Arc<RwLock<Option<AppUsageData>>>;
