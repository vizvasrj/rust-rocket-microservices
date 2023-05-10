use log::error;
use rocket::{request::{FromRequest, Outcome}, Request};

use crate::{models::user::User, fairings::db::DBConnection, jwt::token::Token, states::JWToken};
use serde::Serialize;
use rocket::http::Status;
use rocket_db_pools::{sqlx::Acquire, Connection};
// use sqlx::PgPool;
// use rocket_db_pools::sqlx::Postgres;
// use sqlx::pool::PoolConnection;

#[derive(Serialize, Debug)]
pub struct ApiUser {
    pub user: User
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("from request ...");
        let error = Outcome::Failure((Status::Unauthorized, ()));
        let parsed_token = req.headers().get_one("token");
        if parsed_token.is_none() {
            return error;
        }
        let parsed_db = req.guard::<Connection<DBConnection>>().await;
        if !parsed_db.is_success() {
            return error;
        }
        let mut db = parsed_db.unwrap();
        let parsed_conn = db.acquire().await;
        if parsed_conn.is_err() {
            return error;
        }
        let conn = parsed_conn.unwrap();


        let parsed_secret = req.rocket().state::<JWToken>();
        if parsed_secret.is_none() {
            return error;
        }
        
        let secret = &parsed_secret.unwrap().secret;
        



        let claims = User::verify_jwt_token(parsed_token.unwrap().to_string(), secret.as_bytes());
        if claims.is_err() {
            error!("{}", claims.err().unwrap());
            return error;
        }
        let claim = claims.unwrap();
        let uuid = claim.custom.uuid.to_string();

        let user = User::find_user_by_uuid(conn, uuid.as_str()).await;
        let u = match user {
            Ok(user) => user,
            Err(e) => {
                error!("{}", e);
                return error
            },
        };
        Outcome::Success(ApiUser { user: u })
    }
}
