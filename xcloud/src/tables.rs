#![allow(unused)]

#[derive(sqlx::FromRow)]
pub struct User<'a> {
    id: i32,
    username: &'a str,
    password: String,
    email: &'a str,
    preferences: UserPreferences<'a>,
}

#[derive(sqlx::FromRow)]
pub struct UserPreferences<'a> {
    pub theme: &'a str,
    pub screen_reader: bool,
}

#[derive(sqlx::FromRow)]
pub struct BimObject<'a> {
    pub id: i32,
    pub name: &'a str,
}

impl<'a> User<'a> {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password, &self.password)
    }
}

// example
// fn main() {
//     let password = "password123";
//     let user = User {
//         id: 1,
//         username: "user",
//         password: User::hash_password(password).unwrap(),
//         email: "",
//         preferences: UserPreferences {
//             theme: "dark",
//             screen_reader: false,
//         },
//     };

//     user.verify_password(password);

//     BimObject {
//         id: 1,
//         name: "Cube",
//     };
// }
