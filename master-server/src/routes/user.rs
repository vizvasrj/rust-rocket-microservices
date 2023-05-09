use crate::{fairings::db::DBConnection, models::user::{NewUser, LoginUser}, errors::OurError, guards::jwt_header::ApiUser, jwt::token::Token};
use rocket::{serde::json::Json, State};
use sqlx::PgConnection;
use crate::models::user::User;
use rocket_db_pools::{sqlx::Acquire, Connection};
use crate::states::JWToken;
use crate::models::user_status::UserStatus;
use uuid::Uuid;

#[post(
    "/users",
    format = "json",
    data = "<new_user>",
)]
pub async fn create_user<'r>(
    mut db: Connection<DBConnection>,
    new_user: Json<NewUser<'r>>,
    jwt_secret: &State<JWToken>
) -> Result<Json<User>, Json<OurError>> {
    println!("jwt_secret = {}", jwt_secret.secret);
    let user = new_user.into_inner();
    let conn = db.acquire().await.map_err(|e| {
        log::error!("{}", e)
    });
    let get_user = User::create(conn.unwrap(), user, &jwt_secret.secret)
        .await
        .map_err(|e| {
            Json(e)
        });
    
    // Ok(Json(get_user.unwrap()))
    let u = match get_user {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    Ok(Json(u))
}

pub fn test_in_routes() {}

#[get(
    "/user/<uuid>",
)]
pub async fn get_user_by_uuid(
    mut db: Connection<DBConnection>,
    uuid: &str,
) -> Result<Json<User>, Json<OurError>> {
    println!("in route?????");
    let conn = db.acquire().await
        .map_err(|e| {
            log::error!("{}", e)
        });
        // unwrap wrong here.../
    let user = User::find_user_by_uuid(conn.unwrap(), uuid).await;
    let u = match user {
        Ok(user) => user,
        Err(e) => return Err(Json(e)),
    };
    Ok(Json(u))
}

#[get(
    "/me",
)]
pub async fn get_user_by_token(
    // mut db: Connection<DBConnection>,
    _token: ApiUser,
) -> Result<Json<User>, &'static str> {

    println!("in route api ????? {:?}", _token.user);
    // let _conn = db.acquire().await
    //     .map_err(|e| {
    //         log::error!("{}", e)
    //     });
        // unwrap wrong here.../
    // let user = User::find_user_by_uuid(conn.unwrap(), uuid).await;
    // let u = match user {
    //     Ok(user) => user,
    //     Err(e) => return Err(Json(e)),
    // };
    Ok(Json(_token.user))
}


#[post(
    "/login",
    data = "<login_user>"
)]
pub async fn login(
    login_user: Json<LoginUser<'_>>,
    mut db: Connection<DBConnection>,
    jwt_secret: &State<JWToken>,
) -> Result<Json<User>, Json<OurError>> {
    // let y = User::update_auth_token().await;
    let conn = db.acquire()
        .await
        .map_err(|e| {
            log::error!("{}", e)
        });
    if conn.is_err() {
        return Err(
            // "".to_string()
            Json(OurError::new_bad_request_error(String::new(), None))
        )
    }
    let connection = conn.unwrap();
    let user_result = User
        ::auth(
            login_user.into_inner(), 
            connection, 
            jwt_secret.secret.as_bytes()
        ).await;
    if user_result.is_err() {
        return Err(Json(
            user_result.err().unwrap()
        ));
        // return Err("".to_string());
    }

    Ok(Json(user_result.unwrap()))
}