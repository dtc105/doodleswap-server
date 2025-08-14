use actix_web::web::Data;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct AppState {
    pub pool: PgPool,
}

pub async fn init() -> Data<AppState> {
    let database_url: String = std::env::var("DATABASE_URL").expect("DB_PATH must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database.");

    println!("Connected to database! ");

    Data::new(AppState { pool })
}
