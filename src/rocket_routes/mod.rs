pub mod authorization;
pub mod rustaceans;
pub mod crates;

use diesel::PgConnection;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::fairing::{Fairing, Info, Kind};


use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Database, Connection}; 

use crate::models::{User, RoleCode};
use crate::repositories::{UserRepository, RoleRepository};



#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);


pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}


pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>().await
           .expect("Cannot retrieve logged in user in request guard");
        let db = request.guard::<DbConn>().await
           .expect("Cannot connect to postgres in request guard");
        let editor_option = db.run(|c| {
            match RoleRepository::find_by_user(c, &user) {
                Ok(roles) => {
                    log::info!("Assigned roles {:?}", roles);
                    let is_editor = roles.iter().any(|r| match r.code {
                        RoleCode::Admin => true,
                        RoleCode::Editor => true,
                        _ => false,
                    });
                    log::info!("Is editor is {:?}", is_editor);
                    is_editor.then_some(EditorUser(user))
                },
                _ => None
            }
        }).await;

        match editor_option {
            Some(editor) => Outcome::Success(editor),
            _ => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Just to add CORS header via the fairing.
}


pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}





#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = request.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header  {
            let mut cache = request.guard::<Connection<CacheConn>>().await
                .expect("Cannot connect to redis in request guard");
            let db = request.guard::<DbConn>().await
                .expect("Cannot connect to postgres in request guard");
            let result = cache.get::<_, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ()))
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}

