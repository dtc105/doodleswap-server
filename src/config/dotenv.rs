use std::fs;

use dotenv::dotenv;

pub fn init() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();

    let upload_path =
        std::env::var("UPLOAD_PATH").expect("Environment variable `UPLOAD_PATH` must be defined.");
    let _ = fs::create_dir_all(upload_path);

    println!("Environment variables loaded! 󰑓");
}
