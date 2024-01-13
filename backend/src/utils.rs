use crate::models::user::User;
use crate::permissions::Role;

pub fn filter_user_by_role(
    users: Vec<User>,
    roles: &Vec<Role>
) -> Vec<User> {
    let roles = roles_to_bits(roles);

    users.into_iter().filter(|u| {
        u.roles & roles > 0
    }).collect()
}

pub fn roles_to_bits(
    roles: &Vec<Role>
) -> i32 {
    roles.into_iter().fold(0, |acc, r| {
        acc | *r as i32
    })
}

pub fn roles_to_list(roles_value: i32) -> Vec<Role> {
    let mut roles = Vec::new();

    if roles_value & Role::Admin as i32 > 0 {
        roles.push(Role::Admin);
    }
    if roles_value & Role::Organizer as i32 > 0 {
        roles.push(Role::Organizer);
    }
    if roles_value & Role::Supporter as i32 > 0 {
        roles.push(Role::Supporter);
    }
    if roles_value & Role::Guest as i32 > 0 {
        roles.push(Role::Guest);
    }

    roles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roles_to_bits() {
        assert_eq!(roles_to_bits(&vec![Role::Admin]), Role::Admin as i32);
        assert_eq!(roles_to_bits(&vec![Role::Organizer]), Role::Organizer as i32);
        assert_eq!(roles_to_bits(&vec![Role::Supporter]), Role::Supporter as i32);
        assert_eq!(roles_to_bits(&vec![Role::Guest]), Role::Guest as i32);

        assert_eq!(
            roles_to_bits(&vec![Role::Guest, Role::Supporter]),
            Role::Guest as i32 + Role::Supporter as i32);
    }

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

        let filtered = filter_user_by_role(users, &vec![Role::Organizer, Role::Admin]);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, 2)
    }

    #[test]
    fn test_roles_to_list() {
        assert_eq!(roles_to_list(Role::Admin as i32), vec![Role::Admin]);
        assert_eq!(roles_to_list(Role::Organizer as i32), vec![Role::Organizer]);
        assert_eq!(roles_to_list(Role::Supporter as i32), vec![Role::Supporter]);
        assert_eq!(roles_to_list(Role::Guest as i32), vec![Role::Guest]);
        
        assert_eq!(
            roles_to_list(Role::Guest as i32 + Role::Supporter as i32),
            vec![Role::Supporter, Role::Guest]
        );
    }
}
