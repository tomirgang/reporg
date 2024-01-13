use crate::models::user::User;

pub fn filter_user_by_role(
    users: Vec<User>,
    roles: i32
) -> Vec<User> {
    users.into_iter().filter(|u| {
        u.roles & roles > 0
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions::Role;


    #[test]
    fn test_filter_by_role() {
        let users = vec![
            User {
                id: 1,
                name: "A".to_string(),
                email: "a@b.de".to_string(),
                phone: None,
                notifications: true,
                roles: Role::Supporter as i32,
            },
            User {
                id: 2,
                name: "A".to_string(),
                email: "a@b.de".to_string(),
                phone: None,
                notifications: true,
                roles: Role::Organizer as i32 | Role::Guest as i32,
            },
        ];

        let filtered = filter_user_by_role(
            users, Role::Organizer as i32 | Role::Admin as i32);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, 2)
    }
}
