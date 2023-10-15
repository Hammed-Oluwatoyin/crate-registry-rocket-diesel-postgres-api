use argon2::{password_hash::{SaltString, rand_core::OsRng}, PasswordHasher};
use diesel::{PgConnection, Connection};
use argon2::Argon2;


use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}, auth};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load DB url from env");
    PgConnection::establish(&database_url)
        .expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection();

   let password_hash = auth::hash_password(password).unwrap();
let new_user = NewUser {username, password: password_hash};


    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    println!("Role assigned {:?}", roles);
}

pub fn list_users() {
      let mut c = load_db_connection();

    let users = UserRepository::find_with_roles(&mut c).unwrap();
    for user in users {
        println!("{:?}", user);
    }


}

pub fn delete_user(_id: i32) {

}
