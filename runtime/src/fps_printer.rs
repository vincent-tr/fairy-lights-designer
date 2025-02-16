use std::sync::Mutex;

use wasm_timer::SystemTime;

pub struct FpsPrinter {
    data: Mutex<Option<Data>>,
}

struct Data {
    last_time: SystemTime,
    frame_count: usize,
}

impl FpsPrinter {
    pub const fn new() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }

    pub fn init(&self) {
        self.data.lock().unwrap().replace(Data {
            last_time: SystemTime::now(),
            frame_count: 0,
        });
    }

    pub fn tick(&self) {
        let mut data_lock = self.data.lock().unwrap();
        let data = data_lock.as_mut().unwrap();

        data.frame_count += 1;

        let now = SystemTime::now();
        let elapsed = now.duration_since(data.last_time).unwrap();

        if elapsed.as_secs() >= 1 {
            log::info!("FPS: {}", data.frame_count);
            data.frame_count = 0;
            data.last_time = now;
        }
    }
}
