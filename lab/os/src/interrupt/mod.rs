mod timer;
mod handler;
mod context;
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}