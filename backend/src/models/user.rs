use crate::permissions::Role;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use crate::entities::user;
use crate::error::ReporgError;
use crate::utils::filter_user_by_role;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub notifications: bool,
    pub roles: i32,
}

impl User {
    fn from(model: user::Model) -> Result<User, ReporgError> {
        Ok(User {
            id: model.id,
            name: model.name,
            email: model.email,
            phone: model.phone,
            notifications: model.notification,
            roles: model.roles,
        })
    }

    fn from_opt(model: Option<user::Model>) -> Result<Option<User>, ReporgError> {
        match model {
            Some(m) => {
                let user = User::from(m)?;
                Ok(Some(user))
            },
            None => Ok(None),
        }
    }

    fn from_list(models: Vec<user::Model>) -> Result<Vec<User>, ReporgError> {
        let mut result = Vec::new();

        for m in models.into_iter() {
            let cafe = User::from(m)?;
            result.push(cafe);
        }

        Ok(result)
    }

    pub async fn delete(
        &self,
        db: &DatabaseConnection,
    ) -> Result<DeleteResult, ReporgError> {
        let entry = user::ActiveModel {
            id: ActiveValue::Set(1), // The primary key must be set
            ..Default::default()
        };

        entry.delete(db).await
        .map_err(|e| ReporgError::from(&e))
    }

    pub async fn find(
        user_id: i32,
        db: &DatabaseConnection,
    ) -> Result<Option<User>, ReporgError> {
        let user: Option<user::Model> = user::Entity::find_by_id(user_id).one(db).await
        .map_err(|e| ReporgError::from(&e))?;

        User::from_opt(user)
    }

    pub async fn find_by_email(
        user_email: &str,
        db: &DatabaseConnection,
    ) -> Result<Option<User>, ReporgError> {
        let user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(user_email))
        .one(db).await
        .map_err(|e| ReporgError::from(&e))?;

        User::from_opt(user)
    }

    pub async fn list(
        limit: u64,
        db: &DatabaseConnection,
        roles: &Option<Vec<Role>>
    ) -> Result<Vec<User>, ReporgError> {
        let users: Vec<user::Model> = user::Entity::find().limit(limit).all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        let users = User::from_list(users)?;

        Ok(match roles {
            Some(roles) => filter_user_by_role(users, roles),
            None => users,
        })
    }

    pub async fn page(
        offset: u64,
        limit: u64,
        db: &DatabaseConnection,
        roles: &Option<Vec<Role>>
    ) -> Result<Vec<User>, ReporgError> {
        let users: Vec<user::Model> = user::Entity::find().offset(offset).limit(limit).all(db).await
        .map_err(|e| ReporgError::from(&e))?;

        let users = User::from_list(users)?;

        Ok(match roles {
            Some(roles) => filter_user_by_role(users, roles),
            None => users,
        })
    }

    pub async fn update(
        &self,
        new_values: &mut NewUser,
        db: &DatabaseConnection,
    ) -> Result<User, ReporgError> {
        let user = user::ActiveModel {
            id: ActiveValue::Set(self.id),
            name: ActiveValue::Set(new_values.name.to_owned()),
            email: ActiveValue::Set(new_values.email.to_owned()),
            phone: ActiveValue::Set(new_values.phone.to_owned()),
            notification: ActiveValue::Set(new_values.notifications),
            roles: ActiveValue::Set(new_values.roles),
        };
        let result = user.update(db).await
        .map_err(|e| ReporgError::from(&e))?;

        User::from(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub notifications: bool,
    pub roles: i32,
}

impl NewUser {
    pub fn new(name: &str, email: &str, phone: Option<&str>) -> NewUser {
        let phone = match phone {
            Some(p) => Some(p.to_string()),
            None => None,
        };

        NewUser {
            name: name.to_string(),
            email: email.to_string(),
            phone,
            notifications: true,
            roles: 0,
        }
    }

    pub async fn save(&mut self, db: &DatabaseConnection) -> Result<User, ReporgError> {
        let user = user::ActiveModel {
            name: ActiveValue::Set(self.name.to_owned()),
            email: ActiveValue::Set(self.email.to_owned()),
            phone: ActiveValue::Set(self.phone.to_owned()),
            notification: ActiveValue::Set(self.notifications),
            roles: ActiveValue::Set(self.roles),
            ..Default::default()
        };

        let res = user::Entity::insert(user).exec(db).await
        .map_err(|e| ReporgError::from(&e))?;

        match User::find(res.last_insert_id, db).await? {
            Some(c) => Ok(c),
            None => Err(ReporgError::new(&format!("[NewUser.save] user with ID {} not found.", res.last_insert_id))),
        }
    }
}

pub trait Roles {
    fn set_roles(&mut self, roles: i32);

    fn get_roles(&self) -> i32;

    fn is_role(&self, role: Role) -> bool {
        self.get_roles() & role as i32 > 0
    }

    fn set_role(&mut self, role: Role, value: bool) {
        let roles = if value {
            self.get_roles() | role as i32
        } else {
            self.get_roles() & !(role as i32)
        };
        self.set_roles(roles);
    }

    fn is_admin(&self) -> bool {
        self.is_role(Role::Admin)
    }

    fn set_admin(&mut self, value: bool) {
        self.set_role(Role::Admin, value);
    }

    fn is_organizer(&self) -> bool {
        self.is_role(Role::Admin)
    }

    fn set_organizer(&mut self, value: bool) {
        self.set_role(Role::Admin, value);
    }

    fn is_supporter(&self) -> bool {
        self.is_role(Role::Admin)
    }

    fn set_supporter(&mut self, value: bool) {
        self.set_role(Role::Admin, value);
    }

    fn is_guest(&self) -> bool {
        self.is_role(Role::Admin)
    }

    fn set_guest(&mut self, value: bool) {
        self.set_role(Role::Admin, value);
    }

    fn get_roles_list(&self) -> Vec<Role> {
        let mut roles = Vec::new();
        if self.is_admin() {
            roles.push(Role::Admin);
        }
        if self.is_organizer() {
            roles.push(Role::Organizer);
        }
        if self.is_supporter() {
            roles.push(Role::Supporter);
        }
        if self.is_guest() {
            roles.push(Role::Guest);
        }

        roles
    }
}

impl Roles for User {
    fn set_roles(&mut self, roles: i32) {
        self.roles = roles;
    }

    fn get_roles(&self) -> i32 {
        self.roles
    }
}

impl Roles for NewUser {
    fn set_roles(&mut self, roles: i32) {
        self.roles = roles;
    }

    fn get_roles(&self) -> i32 {
        self.roles
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::establish_connection;

    async fn dummy_user(db: &DatabaseConnection) -> (NewUser, User) {
        let mut new_user = NewUser::new("Jane Doe", "jane.doe@example.com", Some("09123 456 789"));

        let db_user = new_user
            .save(db).await
            .expect("Creation of dummy user failed.");

        (new_user, db_user)
    }

    async fn dummy_user_with_email(email: &str, db: &DatabaseConnection) -> (NewUser, User) {
        let mut new_user = NewUser::new("Jane Doe", email, Some("09123 456 789"));

        let db_user = new_user
            .save(db).await
            .expect("Creation of dummy user with email failed.");

        (new_user, db_user)
    }

    #[tokio::test]
    async fn insert_new_user() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (new_user, db_user) = dummy_user(&db).await;

        assert!(db_user.id > 0);
        assert_eq!(new_user.name, db_user.name);
        assert_eq!(new_user.email, db_user.email);
        assert_eq!(new_user.notifications, true);
        assert_eq!(new_user.roles, 0);
    }

    #[tokio::test]
    async fn delete_user() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_user) = dummy_user(&db).await;

        let user_id = db_user.id;

        match db_user.delete(&db).await {
            Ok(res) => assert_eq!(res.rows_affected, 1),
            Err(e) => panic!("{}", e),
        }

        match User::find(user_id, &db).await {
            Ok(opt_user) => {
                match opt_user {
                    Some(_) => panic!("User was not deleted!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn find_user() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_user) = dummy_user(&db).await;

        let user_id = db_user.id;

        match User::find(user_id, &db).await {
            Ok(opt_user) => match opt_user {
                Some(db_user) => assert_eq!(db_user.id, user_id),
                None => panic!("User was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn find_user_by_email() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_user) = dummy_user(&db).await;

        let user_email = db_user.email;

        match User::find_by_email(&user_email, &db).await {
            Ok(opt_user) => match opt_user {
                Some(db_user) => assert_eq!(db_user.email, user_email),
                None => panic!("User was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn find_user_invalid_id() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, db_user) = dummy_user(&db).await;

        let user_id = db_user.id + 1;

        match User::find(user_id, &db).await {
            Ok(opt_user) => {
                match opt_user {
                    Some(_) => panic!("Wrong user was found!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn user_email_must_be_unique() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_user(&db).await;

        let mut new_user = NewUser::new("John Doe", "jane.doe@example.com", Some("09123 456 789"));

        let result = new_user.save(&db).await;

        if let Ok(_) = result {
            panic!("Two users with same email");
        }
    }

    #[tokio::test]
    async fn list_users() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_user(&db).await;
        let (_, _) = dummy_user_with_email("john.doe@example.com", &db).await;

        match User::list(100, &db, &None).await {
            Ok(users) => assert_eq!(users.len(), 2),
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn page_users() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        let (_, _) = dummy_user(&db).await;
        let (_, user_obj) = dummy_user_with_email("john.doe@example.com", &db).await;
        let (_, _) = dummy_user_with_email("jim.doe@example.com", &db).await;

        match User::page(1, 1, &db, &None).await {
            Ok(users) => {
                assert_eq!(users.len(), 1);
                assert_eq!(user_obj.id, users[0].id);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[tokio::test]
    async fn update_user() {
        let db = establish_connection("sqlite::memory:").await.unwrap();
        
        let (mut new_user, db_user) = dummy_user(&db).await;

        new_user.name = String::from("Max Mustermann");
        new_user.email = String::from("max.mustermann@web.de");
        new_user.phone = Some(String::from("0151 1234 5678"));
        new_user.notifications = false;
        new_user.roles = 15;

        let db_user = db_user
            .update(&mut new_user, &db).await
            .expect("Update of user failed!");

        assert_eq!(db_user.name, new_user.name);
        assert_eq!(db_user.email, new_user.email);
        assert_eq!(db_user.phone, new_user.phone);
        assert_eq!(db_user.notifications, new_user.notifications);
        assert_eq!(db_user.roles, new_user.roles);

        let db_user = User::find(db_user.id, &db).await
            .expect("User not found!")
            .unwrap();

        assert_eq!(db_user.name, new_user.name);
        assert_eq!(db_user.email, new_user.email);
        assert_eq!(db_user.phone, new_user.phone);
        assert_eq!(db_user.notifications, new_user.notifications);
        assert_eq!(db_user.roles, new_user.roles);
    }
}
