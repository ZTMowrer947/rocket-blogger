use rocket_db_pools::Database;
use rocket_db_pools::diesel::PgPool;

#[derive(Database)]
#[database("rocket-blogger")]
pub struct Blogger(PgPool);
