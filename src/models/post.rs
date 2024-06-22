use std::time::SystemTime;

use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
