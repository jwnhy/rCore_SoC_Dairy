mod timer;
mod handler;
pub mod context;

pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}