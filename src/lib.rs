use std::thread;

pub const LOCAL: &str = "127.0.0.1:6000";
pub const MSG_SIZE: usize = 32;

pub fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}