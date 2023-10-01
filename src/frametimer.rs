pub fn init() {}

pub fn wait_for_frame() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
