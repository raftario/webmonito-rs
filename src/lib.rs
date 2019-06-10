pub mod config;
pub mod pings;
pub mod urls;

pub fn verbose_println(log: bool, message: &str) {
    if log {
        println!(message);
    }
}
