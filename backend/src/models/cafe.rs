use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cafe)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cafe {
    pub id: i32,
    pub location: String,
    pub address: String,
    pub date: NaiveDateTime,
}
