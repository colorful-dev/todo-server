use diesel::pg::PgConnection;
use dotenvy::dotenv;
// use diesel::r2d2::ConnectionManager;
use diesel::r2d2::ConnectionManager;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn sql_connect() -> Pool {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // PgConnection::establish(&database_url)
    //     .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
