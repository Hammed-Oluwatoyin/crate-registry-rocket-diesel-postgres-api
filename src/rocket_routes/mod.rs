pub mod rustaceans;
pub mod crates;

use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);
