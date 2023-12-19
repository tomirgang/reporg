use crate::models::schema::cafe;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = cafe)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Cafe {
    pub id: i32,
    pub location: String,
    pub address: String,
    pub date: NaiveDateTime,
}

impl Cafe {
    pub fn delete(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;
        Ok(diesel::delete(cafe.filter(id.eq(self.id))).execute(connection)?)
    }

    pub fn find(
        cafe_id: i32,
        connection: &mut SqliteConnection,
    ) -> Result<Option<Cafe>, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;
        let mut results = cafe
            .limit(1)
            .filter(id.eq(cafe_id))
            .select(Cafe::as_select())
            .load(connection)?;

        let cafe_object = if results.len() > 0 {
            Some(results.remove(0))
        } else {
            None
        };

        Ok(cafe_object)
    }

    pub fn list(
        limit: i64,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<Cafe>, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;

        cafe.limit(limit).select(Cafe::as_select()).load(connection)
    }

    pub fn page(
        offset: i64,
        limit: i64,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<Cafe>, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;

        cafe.offset(offset)
            .limit(limit)
            .select(Cafe::as_select())
            .load(connection)
    }

    pub fn past_cafes(
        connection: &mut SqliteConnection,
    ) -> Result<Vec<Cafe>, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;

        cafe.filter(date.lt(Utc::now().naive_utc()))
            .select(Cafe::as_select())
            .load(connection)
    }

    pub fn future_cafes(
        connection: &mut SqliteConnection,
    ) -> Result<Vec<Cafe>, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;

        cafe.filter(date.ge(Utc::now().naive_utc()))
            .select(Cafe::as_select())
            .load(connection)
    }

    pub fn update(
        &self,
        new_values: &NewCafe,
        connection: &mut SqliteConnection,
    ) -> Result<Cafe, diesel::result::Error> {
        use crate::models::schema::cafe::dsl::*;

        let db_cafe = diesel::update(cafe)
            .filter(id.eq(self.id))
            .set(new_values)
            .returning(Cafe::as_returning())
            .get_result(connection)?;
        Ok(db_cafe)
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = cafe)]
pub struct NewCafe {
    pub location: String,
    pub address: String,
    pub date: NaiveDateTime,
}

impl NewCafe {
    pub fn new(location: String, address: String, date: NaiveDateTime) -> NewCafe {
        NewCafe {
            location: location,
            address: address,
            date: date,
        }
    }

    pub fn save(&self, connection: &mut SqliteConnection) -> Result<Cafe, diesel::result::Error> {
        let db_cafe = diesel::insert_into(cafe::table)
            .values(self)
            .returning(Cafe::as_returning())
            .get_result(connection)?;
        Ok(db_cafe)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::establish_connection;

    fn dummy_cafe(
        cafe_date: Option<NaiveDateTime>,
        connection: &mut SqliteConnection,
    ) -> (NewCafe, Cafe) {
        let cafe_date = match cafe_date {
            None => Utc::now().naive_utc(),
            Some(cafe_date) => cafe_date,
        };

        let new_cafe = NewCafe::new(
            String::from("Haus des Gastes"),
            String::from("Maria-Dorothea-Straße 8, 91161 Hilpoltstein"),
            cafe_date,
        );

        let db_cafe = new_cafe
            .save(connection)
            .expect("Creation of dummy cafe failed.");

        (new_cafe, db_cafe)
    }

    #[test]
    fn insert_new_cafe() {
        let mut connection = establish_connection().get().unwrap();
        let (new_cafe, db_cafe) = dummy_cafe(None, &mut connection);

        assert!(db_cafe.id > 0);
        assert_eq!(new_cafe.location, db_cafe.location);
        assert_eq!(new_cafe.address, db_cafe.address);
        assert_eq!(new_cafe.date, db_cafe.date);
    }

    #[test]
    fn delete_cafe() {
        let mut connection = establish_connection().get().unwrap();
        let (_, db_cafe) = dummy_cafe(None, &mut connection);

        let cafe_id = db_cafe.id;

        match db_cafe.delete(&mut connection) {
            Ok(cnt) => assert_eq!(cnt, 1),
            Err(e) => panic!("{}", e),
        }

        match Cafe::find(cafe_id, &mut connection) {
            Ok(opt_cafe) => {
                match opt_cafe {
                    Some(_) => panic!("Cafe was not deleted!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn find_cafe() {
        let mut connection = establish_connection().get().unwrap();
        let (_, db_cafe) = dummy_cafe(None, &mut connection);

        let cafe_id = db_cafe.id;

        match Cafe::find(cafe_id, &mut connection) {
            Ok(opt_cafe) => match opt_cafe {
                Some(db_cafe) => assert_eq!(db_cafe.id, cafe_id),
                None => panic!("Cafe was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn find_cafe_invalid_id() {
        let mut connection = establish_connection().get().unwrap();
        let (_, db_cafe) = dummy_cafe(None, &mut connection);

        let cafe_id = db_cafe.id + 1;

        match Cafe::find(cafe_id, &mut connection) {
            Ok(opt_cafe) => {
                match opt_cafe {
                    Some(_) => panic!("Wrong cafe was found!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn list_cafes() {
        let mut connection = establish_connection().get().unwrap();
        let (_, _) = dummy_cafe(None, &mut connection);
        let (_, _) = dummy_cafe(None, &mut connection);

        match Cafe::list(100, &mut connection) {
            Ok(cafes) => assert_eq!(cafes.len(), 2),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn page_cafes() {
        let mut connection = establish_connection().get().unwrap();
        let (_, _) = dummy_cafe(None, &mut connection);
        let (_, cafe_obj) = dummy_cafe(None, &mut connection);
        let (_, _) = dummy_cafe(None, &mut connection);

        match Cafe::page(1, 1, &mut connection) {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 1);
                assert_eq!(cafe_obj.id, cafes[0].id);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn past_cafes() {
        let mut connection = establish_connection().get().unwrap();
        let (_, _) = dummy_cafe(None, &mut connection);
        let (_, _) = dummy_cafe(None, &mut connection);
        let (_, _) = dummy_cafe(None, &mut connection);

        match Cafe::past_cafes(&mut connection) {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 3);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn future_cafes() {
        use chrono::{Datelike, Timelike};

        let mut connection = establish_connection().get().unwrap();

        let now = Utc::now().naive_utc();
        let future1 = now.with_year(now.year() + 1).unwrap();
        let future2 = if now.minute() < 58 {
            now.with_minute(now.minute() + 2).unwrap()
        } else {
            now.with_year(now.year() + 2).unwrap()
        };

        let (_, _) = dummy_cafe(None, &mut connection);
        let (_, _) = dummy_cafe(Some(future1), &mut connection);
        let (_, _) = dummy_cafe(Some(future2), &mut connection);

        match Cafe::future_cafes(&mut connection) {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 2);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn update_cafe() {
        use chrono::Datelike;
        let mut connection = establish_connection().get().unwrap();

        let (mut new_cafe, db_cafe) = dummy_cafe(None, &mut connection);

        new_cafe.location = String::from("AWO Soziales Kompetenz - Zentrum");
        new_cafe.address = String::from("Sankt-Jakob-Straße 12, 91161 Hilpoltstein");
        let now = Utc::now().naive_utc();
        new_cafe.date = now.with_year(now.year() + 1).unwrap();

        let db_cafe = db_cafe
            .update(&new_cafe, &mut connection)
            .expect("Update of cafe failed!");

        assert_eq!(db_cafe.location, new_cafe.location);
        assert_eq!(db_cafe.address, new_cafe.address);
        assert_eq!(db_cafe.date, new_cafe.date);

        let db_cafe = Cafe::find(db_cafe.id, &mut connection)
            .expect("Cafe not found!")
            .unwrap();

        assert_eq!(db_cafe.location, new_cafe.location);
        assert_eq!(db_cafe.address, new_cafe.address);
        assert_eq!(db_cafe.date, new_cafe.date);
    }
}
