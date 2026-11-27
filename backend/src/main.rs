mod db;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL fehlt in .env");
    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await?;
    
    sqlx::migrate! ("./migrations").run(&pool).await?;
    println!("Migrationen angewendet");
    Ok(())
}