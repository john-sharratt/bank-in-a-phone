use crate::LocalApp;

impl LocalApp {
    pub fn save_state(&mut self, frame: &mut eframe::Frame) {
        if let Some(storage) = frame.storage_mut() {
            eframe::set_value(storage, eframe::APP_KEY, self);
        }
    }
}
