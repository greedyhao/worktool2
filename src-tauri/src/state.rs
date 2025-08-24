
use crate::speed_test::SpeedTestHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

// 应用的全局状态
pub struct AppState {
    pub speed_test_handle: Arc<Mutex<Option<SpeedTestHandle>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            speed_test_handle: Arc::new(Mutex::new(None)),
        }
    }
}
