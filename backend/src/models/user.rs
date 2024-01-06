use crate::models::schema::user;
use crate::permissions::Role;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notifications: bool,
    pub roles: i32,
}

impl User {
    pub fn delete(
        &self,
        connection: &mut SqliteConnection,
    ) -> Result<usize, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;
        Ok(diesel::delete(user.filter(id.eq(self.id))).execute(connection)?)
    }

    pub fn find(
        user_id: i32,
        connection: &mut SqliteConnection,
    ) -> Result<Option<User>, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;
        let mut results = user
            .limit(1)
            .filter(id.eq(user_id))
            .select(User::as_select())
            .load(connection)?;

        let user_object = if results.len() > 0 {
            Some(results.remove(0))
        } else {
            None
        };

        Ok(user_object)
    }

    pub fn find_by_email(
        user_email: &str,
        connection: &mut SqliteConnection,
    ) -> Result<Option<User>, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;
        let mut results = user
            .limit(1)
            .filter(email.eq(user_email))
            .select(User::as_select())
            .load(connection)?;

        let user_object = if results.len() > 0 {
            Some(results.remove(0))
        } else {
            None
        };

        Ok(user_object)
    }

    pub fn list(
        limit: i64,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;

        user.limit(limit).select(User::as_select()).load(connection)
    }

    pub fn page(
        offset: i64,
        limit: i64,
        connection: &mut SqliteConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;

        user.offset(offset)
            .limit(limit)
            .select(User::as_select())
            .load(connection)
    }

    pub fn update(
        &self,
        new_values: &NewUser,
        connection: &mut SqliteConnection,
    ) -> Result<User, diesel::result::Error> {
        use crate::models::schema::user::dsl::*;

        let db_user = diesel::update(user)
            .filter(id.eq(self.id))
            .set(new_values)
            .returning(User::as_returning())
            .get_result(connection)?;
        Ok(db_user)
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notifications: bool,
    pub roles: i32,
}

impl NewUser {
    pub fn new(name: &str, email: &str, phone: &str) -> NewUser {
        NewUser {
            name: name.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            notifications: true,
            roles: 0,
        }
    }

    pub fn save(&self, connection: &mut SqliteConnection) -> Result<User, diesel::result::Error> {
        let db_user = diesel::insert_into(user::table)
            .values(self)
            .returning(User::as_returning())
            .get_result(connection)?;
        Ok(db_user)
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

    fn dummy_user(connection: &mut SqliteConnection) -> (NewUser, User) {
        let new_user = NewUser::new("Jane Doe", "jane.doe@example.com", "09123 456 789");

        let db_user = new_user
            .save(connection)
            .expect("Creation of dummy user failed.");

        (new_user, db_user)
    }

    fn dummy_user_with_email(email: &str, connection: &mut SqliteConnection) -> (NewUser, User) {
        let new_user = NewUser::new("Jane Doe", email, "09123 456 789");

        let db_user = new_user
            .save(connection)
            .expect("Creation of dummy user with email failed.");

        (new_user, db_user)
    }

    #[test]
    fn insert_new_user() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (new_user, db_user) = dummy_user(&mut connection);

        assert!(db_user.id > 0);
        assert_eq!(new_user.name, db_user.name);
        assert_eq!(new_user.email, db_user.email);
        assert_eq!(new_user.notifications, true);
        assert_eq!(new_user.roles, 0);
    }

    #[test]
    fn delete_user() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, db_user) = dummy_user(&mut connection);

        let user_id = db_user.id;

        match db_user.delete(&mut connection) {
            Ok(cnt) => assert_eq!(cnt, 1),
            Err(e) => panic!("{}", e),
        }

        match User::find(user_id, &mut connection) {
            Ok(opt_user) => {
                match opt_user {
                    Some(_) => panic!("User was not deleted!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn find_user() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, db_user) = dummy_user(&mut connection);

        let user_id = db_user.id;

        match User::find(user_id, &mut connection) {
            Ok(opt_user) => match opt_user {
                Some(db_user) => assert_eq!(db_user.id, user_id),
                None => panic!("User was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn find_user_by_email() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, db_user) = dummy_user(&mut connection);

        let user_email = db_user.email;

        match User::find_by_email(&user_email, &mut connection) {
            Ok(opt_user) => match opt_user {
                Some(db_user) => assert_eq!(db_user.email, user_email),
                None => panic!("User was not found!"),
            },
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn find_user_invalid_id() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, db_user) = dummy_user(&mut connection);

        let user_id = db_user.id + 1;

        match User::find(user_id, &mut connection) {
            Ok(opt_user) => {
                match opt_user {
                    Some(_) => panic!("Wrong user was found!"),
                    None => {} // Ok
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn user_email_must_be_unique() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, _) = dummy_user(&mut connection);

        let new_user = NewUser::new("John Doe", "jane.doe@example.com", "09123 456 789");

        let result = new_user.save(&mut connection);

        if let Ok(_) = result {
            panic!("Two users with same email");
        }
    }

    #[test]
    fn list_users() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, _) = dummy_user(&mut connection);
        let (_, _) = dummy_user_with_email("john.doe@example.com", &mut connection);

        match User::list(100, &mut connection) {
            Ok(users) => assert_eq!(users.len(), 2),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn page_users() {
        let mut connection = establish_connection(":memory:").get().unwrap();
        let (_, _) = dummy_user(&mut connection);
        let (_, user_obj) = dummy_user_with_email("john.doe@example.com", &mut connection);
        let (_, _) = dummy_user_with_email("jim.doe@example.com", &mut connection);

        match User::page(1, 1, &mut connection) {
            Ok(users) => {
                assert_eq!(users.len(), 1);
                assert_eq!(user_obj.id, users[0].id);
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn update_user() {
        let mut connection = establish_connection(":memory:").get().unwrap();

        let (mut new_user, db_user) = dummy_user(&mut connection);

        new_user.name = String::from("Max Mustermann");
        new_user.email = String::from("max.mustermann@web.de");
        new_user.phone = String::from("0151 1234 5678");
        new_user.notifications = false;
        new_user.roles = 15;

        let db_user = db_user
            .update(&new_user, &mut connection)
            .expect("Update of user failed!");

        assert_eq!(db_user.name, new_user.name);
        assert_eq!(db_user.email, new_user.email);
        assert_eq!(db_user.phone, new_user.phone);
        assert_eq!(db_user.notifications, new_user.notifications);
        assert_eq!(db_user.roles, new_user.roles);

        let db_user = User::find(db_user.id, &mut connection)
            .expect("User not found!")
            .unwrap();

        assert_eq!(db_user.name, new_user.name);
        assert_eq!(db_user.email, new_user.email);
        assert_eq!(db_user.phone, new_user.phone);
        assert_eq!(db_user.notifications, new_user.notifications);
        assert_eq!(db_user.roles, new_user.roles);
    }
}
