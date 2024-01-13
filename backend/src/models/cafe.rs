use chrono::NaiveDateTime;
use chrono::Utc;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::entities::cafe;
use crate::error::ReporgError;

#[derive(Serialize, Deserialize)]
pub struct Cafe {
    pub id: i32,
    pub location: String,
    pub address: String,
    pub date: NaiveDateTime,
}

impl Cafe {
    fn from(model: cafe::Model) -> Result<Cafe, ReporgError> {
        let date = NaiveDateTime::parse_from_str(&model.date, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| ReporgError::from(&e))?;

        Ok(Cafe {
            id: model.id,
            location: model.location,
            address: model.address,
            date,
        })
    }

    fn from_opt(model: Option<cafe::Model>) -> Result<Option<Cafe>, ReporgError> {
        match model {
            Some(m) => {
                let cafe = Cafe::from(m)?;
                Ok(Some(cafe))
            },
            None => Ok(None),
        }
    }

    fn from_list(models: Vec<cafe::Model>) -> Result<Vec<Cafe>, ReporgError> {
        let mut result = Vec::new();

        for m in models.into_iter() {
            let cafe = Cafe::from(m)?;
            result.push(cafe);
        }

        Ok(result)
    }

    pub async fn delete(
        &self,
        db: &DatabaseConnection,
    ) -> Result<DeleteResult, ReporgError> {
        let entry = cafe::ActiveModel {
            id: ActiveValue::Set(1), // The primary key must be set
            ..Default::default()
        };

        entry.delete(db).await
        .map_err(|e| ReporgError::from(&e))
    }

    pub async fn find(
        cafe_id: i32,
        db: &DatabaseConnection,
    ) -> Result<Option<Cafe>, ReporgError> {
        let cafe: Option<cafe::Model> = cafe::Entity::find_by_id(cafe_id).one(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Cafe::from_opt(cafe)
    }

    pub async fn list(
        limit: u64,
        db: &DatabaseConnection,
    ) -> Result<Vec<Cafe>, ReporgError> {
        let cafes: Vec<cafe::Model> = cafe::Entity::find().limit(limit).all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Cafe::from_list(cafes)
    }

    pub async fn page(
        offset: u64,
        limit: u64,
        db: &DatabaseConnection,
    ) -> Result<Vec<Cafe>, ReporgError> {
        let cafes: Vec<cafe::Model> = cafe::Entity::find().offset(offset).limit(limit).all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Cafe::from_list(cafes)
    }

    pub async fn past_cafes(
        db: &DatabaseConnection,
    ) -> Result<Vec<Cafe>, ReporgError> {
        let cafes: Vec<cafe::Model> = cafe::Entity::find()
        .filter(cafe::Column::Date.lt(Utc::now().naive_utc()))
        .all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Cafe::from_list(cafes)
    }

    pub async fn future_cafes(
        db: &DatabaseConnection,
    ) -> Result<Vec<Cafe>, ReporgError> {
        let cafes: Vec<cafe::Model> = cafe::Entity::find()
        .filter(cafe::Column::Date.gte(Utc::now().naive_utc()))
        .all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Ok(Cafe::from_list(cafes)?)
    }

    pub async fn update(
        &self,
        new_values: &NewCafe,
        db: &DatabaseConnection,
    ) -> Result<Cafe, ReporgError> {
        let mut cafe = new_values.to_model();
        cafe.id = ActiveValue::Set(self.id);
        let result = cafe.update(db).await
        .map_err(|e| ReporgError::from(&e))?;

        Cafe::from(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewCafe {
    pub location: String,
    pub address: String,
    pub date: NaiveDateTime,
}

impl NewCafe {
    pub fn new(location: &str, address: &str, date: NaiveDateTime) -> NewCafe {
        NewCafe {
            location: location.to_string(),
            address: address.to_string(),
            date: date,
        }
    }

    fn to_model(&self) -> cafe::ActiveModel {
        let date = self.date.format("%Y-%m-%d %H:%M:%S").to_string();
        
        cafe::ActiveModel {
            location: ActiveValue::Set(self.location.clone()),
            address: ActiveValue::Set(self.address.clone()),
            date: ActiveValue::Set(date),
            ..Default::default()
        }
    }

    pub async fn save(&mut self, db: &DatabaseConnection) -> Result<Cafe, ReporgError> {
        let cafe = self.to_model();

        let res = cafe::Entity::insert(cafe).exec(db).await
        .map_err(|e| ReporgError::from(&e))?;

        match Cafe::find(res.last_insert_id, db).await? {
            Some(c) => Ok(c),
            None => Err(ReporgError::new(&format!("[NewCafe.save] cafe with ID {} not found.", res.last_insert_id))),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;
    use crate::models::establish_connection;

    async fn dummy_cafe(
        cafe_date: Option<NaiveDateTime>,
        db: &DatabaseConnection,
    ) -> (NewCafe, Cafe) {
        let cafe_date = match cafe_date {
            None => {
                let date = Utc::now().naive_utc();
                date.with_nanosecond(0).unwrap()
            },
            Some(cafe_date) => cafe_date,
        };

        let mut new_cafe = NewCafe::new(
            "Haus des Gastes",
            "Maria-Dorothea-Straße 8, 91161 Hilpoltstein",
            cafe_date,
        );

        let db_cafe = new_cafe
            .save(db).await
            .expect("Creation of dummy cafe failed.");

        (new_cafe, db_cafe)
    }

    #[tokio::test]
    async fn insert_new_cafe() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (new_cafe, db_cafe) = dummy_cafe(None, &db).await;

        assert!(db_cafe.id > 0);
        assert_eq!(new_cafe.location, db_cafe.location);
        assert_eq!(new_cafe.address, db_cafe.address);
        assert_eq!(new_cafe.date, db_cafe.date);
    }

    #[tokio::test]
    async fn delete_cafe() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_cafe) = dummy_cafe(None, &db).await;

        let cafe_id = db_cafe.id;

        match db_cafe.delete(&db).await {
            Ok(res) => assert_eq!(res.rows_affected, 1),
            Err(e) => panic!("{}", e),
        }

        match Cafe::find(cafe_id, &db).await {
            Ok(opt_cafe) => {
                match opt_cafe {
                    Some(_) => panic!("Cafe was not deleted!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn find_cafe() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_cafe) = dummy_cafe(None, &db).await;

        let cafe_id = db_cafe.id;

        match Cafe::find(cafe_id, &db).await {
            Ok(opt_cafe) => match opt_cafe {
                Some(db_cafe) => assert_eq!(db_cafe.id, cafe_id),
                None => panic!("Cafe was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn find_cafe_invalid_id() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_cafe) = dummy_cafe(None, &db).await;

        let cafe_id = db_cafe.id + 1;

        match Cafe::find(cafe_id, &db).await {
            Ok(opt_cafe) => {
                match opt_cafe {
                    Some(_) => panic!("Wrong cafe was found!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn list_cafes() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_cafe(None, &db).await;
        let (_, _) = dummy_cafe(None, &db).await;

        match Cafe::list(100, &db).await {
            Ok(cafes) => assert_eq!(cafes.len(), 2),
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn page_cafes() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_cafe(None, &db).await;
        let (_, cafe_obj) = dummy_cafe(None, &db).await;
        let (_, _) = dummy_cafe(None, &db).await;

        match Cafe::page(1, 1, &db).await {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 1);
                assert_eq!(cafe_obj.id, cafes[0].id);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn past_cafes() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_cafe(None, &db).await;
        let (_, _) = dummy_cafe(None, &db).await;
        let (_, _) = dummy_cafe(None, &db).await;

        match Cafe::past_cafes(&db).await {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 3);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn future_cafes() {
        use chrono::{Datelike, Timelike};

        let db = establish_connection("sqlite::memory:").await.unwrap();
        
        let now = Utc::now().naive_utc();
        let future1 = now.with_year(now.year() + 1).unwrap();
        let future2 = if now.minute() < 58 {
            now.with_minute(now.minute() + 2).unwrap()
        } else {
            now.with_year(now.year() + 2).unwrap()
        };

        let (_, _) = dummy_cafe(None, &db).await;
        let (_, _) = dummy_cafe(Some(future1), &db).await;
        let (_, _) = dummy_cafe(Some(future2), &db).await;

        match Cafe::future_cafes(&db).await {
            Ok(cafes) => {
                assert_eq!(cafes.len(), 2);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn update_cafe() {
        use chrono::Datelike;
        let db = establish_connection("sqlite::memory:").await.unwrap();
        
        let (mut new_cafe, db_cafe) = dummy_cafe(None, &db).await;

        new_cafe.location = String::from("AWO Soziales Kompetenz - Zentrum");
        new_cafe.address = String::from("Sankt-Jakob-Straße 12, 91161 Hilpoltstein");
        let now = Utc::now().naive_utc();
        new_cafe.date = now.with_year(now.year() + 1).unwrap()
        .with_nanosecond(0).unwrap();

        let db_cafe = db_cafe
            .update(&new_cafe, &db).await
            .expect("Update of cafe failed!");

        assert_eq!(db_cafe.location, new_cafe.location);
        assert_eq!(db_cafe.address, new_cafe.address);
        assert_eq!(db_cafe.date, new_cafe.date);

        let db_cafe = Cafe::find(db_cafe.id, &db).await
            .expect("Cafe not found!")
            .unwrap();

        assert_eq!(db_cafe.location, new_cafe.location);
        assert_eq!(db_cafe.address, new_cafe.address);
        assert_eq!(db_cafe.date, new_cafe.date);
    }
}
