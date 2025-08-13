use dotenv::dotenv;

pub fn init() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv().ok();
    env_logger::init();
    println!("Environment variables loaded! 󰑓");
}
