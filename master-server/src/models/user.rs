use std::collections::BTreeMap;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use jwt::SignWithKey;
use rand_core::OsRng;
use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;
use tokio::sync::futures;
use zxcvbn::zxcvbn;
use crate::{errors::OurError, states::JWToken, jwt::token::Token};
// use rocket::serde::json::Json;
// use rocket::serde;
use super::user_status::UserStatus;
use rocket_db_pools::sqlx::PgConnection;
use jwt_simple::prelude::*;

use rocket::form::{self, Error as FormError};
use regex::Regex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::FromRow;
use argon2::PasswordHash;
use argon2::PasswordVerifier;




pub struct UserLogin<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password1: String,
    pub password2: String,
    pub phone: u64,
}

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: String,
    pub refresh_token: String,
    pub description: Option<String>,
    pub token: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: UserStatus,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsUser {
    pub uuid: Uuid,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser<'r> {
    // #[field(validate = len(5..20).or_else(msg!("min 5 max 20")))]
    pub username: &'r str,
    // #[field(validate = validate_email().or_else(msg!("invalid email")))]
    pub email: &'r str,
    // #[field(validate = validate_password().or_else(msg!("weak password")))]
    pub password: &'r str,
    // #[field(validate = eq(self.password).or_else(msg!("password confirmation mismatch")))]
    pub password_confirmation: &'r str,
    
}

impl User {
    pub async fn find_user_by_uuid(
        conn: &mut PgConnection, uuid: &str
    ) -> Result<Self, OurError> {
        let parsed_uuid = Uuid::parse_str(uuid)
            .map_err(OurError::from_uuid_error)?;
        let query_str = "SELECT * FROM users WHERE uuid = $1";
        let row = sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(conn)
            .await
            .map_err(|e| {
                OurError::sqlx_bad_request_error(e.to_string(), Some(Box::new(e)))
            });
        // let row = sqlx::que
        row
    
    }

    pub async fn find_user_by_username(
        conn: &mut PgConnection, username: String
    ) -> Result<Self, OurError> {
        let query_str = "SELECT * FROM users WHERE username = $1";
        let row = sqlx::query_as::<_, Self>(query_str)
            .bind(username)
            .fetch_one(conn)
            .await
            .map_err(|e| {
                // OurError::sqlx_bad_request_error(e.to_string(), Some(Box::new(e)))
                OurError::string_error(e.to_string())
            });
        // let row = sqlx::que
        row
    
    }

    pub async fn update_auth_token(
        _conn: &mut PgConnection,
    ) -> Result<(), ()> {
        return Ok(());
    }

    pub async fn create<'r>(
        conn: &mut PgConnection,
        new_user: NewUser<'r>,
        jwt_secret: &String,
    ) -> Result<Self, OurError> {
        let uuid = Uuid::new_v4();
        let username = new_user.username;
        let email = new_user.email;
        let password = new_user.password;
        let confirm_password = new_user.password_confirmation;
        println!(
            "e={}, p1={}, p2={}, u={:?}, n={}",
            email, password, confirm_password, uuid, username,
        );

        let num = some_simple().await;
        println!("num = {}", num);

        let pass_strength = validate_password(password).await;
            // .await
            // .map_err(|e| {
            //     OurError::new_bad_request_error(e.to_string(), None)
            // });
        if pass_strength.is_err() {
            let e = pass_strength.err().unwrap();
            return Err(OurError::new_bad_request_error(e.to_string(), None));
        }
        
        if !password.eq(confirm_password) {
            return Err(
                OurError::new_bad_request_error(
                    "password did not matched".to_string(),
                    None,
                )
            );
        }

        let valid_email = validate_email(email).await;
        if valid_email.is_err() {
            return Err(
                OurError::new_bad_request_error(
                    valid_email.err().unwrap().to_string(),
                    None,
                )
            )
        }

        // create password hash
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt);
        if password_hash.is_err() {
            return Err(
                OurError::new_internal_server_error(
                    "Something went wrong".to_string(),
                    Some(Box::new(password_hash.err().unwrap())),
                )
            );
        }

        // let auth_error = || {
        //     OurError::new_bad_request_error("Some Error".to_string(), None)
        // };
        // let key: Hmac<Sha256> =
        //     Hmac::new_from_slice(jwt_secret.as_bytes()).map_err(|_| auth_error()).unwrap();

        // let mut claims = BTreeMap::new();
        // claims.insert("user_uuid", uuid);

        // let token = claims.sign_with_key(&key).map_err(|_| auth_error())?;

        let jwt_token = User::get_jwt_token(&username.to_string(), &uuid, jwt_secret.as_bytes());
        let refresh_token = User::get_jwt_refresh_token(&username.to_string(), &uuid, jwt_secret.as_bytes());

        
        //     .map_err(|e| {
        //         OurError::new_internal_server_error(
        //             "Something went wrong".to_string(),
        //             Some(Box::new(e)),
        //         )
        //     });

        // match pass_strength {
        //     Ok(()) => {},
        //     Err(e) => {
        //         for error in e.iter() {
        //             error!("{}", e);
        //         }
                
        //         return Err(OurError::new_bad_request_error("weak password".to_string(), Some(Box::new(e))))
        //     }
        // }
        
        // if pass_strength.is_err() {
        //     return Err(OurError::new_bad_request_error(
        //         e,
        //         Some(Box::new(pass_strength.err().unwrap())),
        //     ))
        // }

        let password_hash = password_hash.unwrap().to_string();
        let token = jwt_token;
        let refresh_token = refresh_token;
        let created_at = chrono::Utc::now();
        let updated_at = chrono::Utc::now();
        let status = UserStatus::Active;
        let query_str = r#"
        INSERT INTO users (uuid, username, password_hash, email, refresh_token, token, created_at, updated_at, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#;
        let user = sqlx::query_as::<_, Self>(query_str)
            .bind(uuid)
            .bind(username)
            .bind(password_hash)
            .bind(email)
            .bind(refresh_token)
            .bind(token)
            .bind(created_at)
            .bind(updated_at)
            .bind(status)
            .fetch_one(conn)
            .await
            .map_err(|e|{
                return OurError::sqlx_bad_request_error(e.to_string(), Some(Box::new(e)))
            });


        user
        // Ok(User {
        //     uuid: uuid,
        //     username: username.to_string(),
        //     password_hash: "".to_string(),
        //     email: email.to_string(),
        //     refresh_token: "".to_string(),
        //     description: None,
        //     token: "".to_string(),
        //     created_at: chrono::Utc::now(),
        //     updated_at: chrono::Utc::now(),
        //     status: UserStatus::Active,
        // })
    }

    pub fn verify_password_hash(
        password: &String,
        password_hash: &String,
    ) -> Result<(), OurError> {
        let aragon = Argon2::default();
        let refrence_hash = PasswordHash::new(password_hash.as_str())
            .map_err(|e| {
                OurError::new_internal_server_error(
                    e.to_string(),
                    Some(Box::new(e)),
                )
            });
        if refrence_hash.is_err() {
            return Err(
                refrence_hash.err().unwrap()
            );
        }
        
        let verify = aragon.verify_password(password.as_bytes(), &refrence_hash.unwrap())
            .map_err(|e| {
                OurError::new_internal_server_error(
                    e.to_string(),
                    Some(Box::new(e)),
                )
            });
        verify
        // Ok(())

    }

    pub fn demo() -> Self {
            User {
            uuid: Uuid::new_v4(),
            username: "username".to_string(),
            password_hash: "".to_string(),
            email: "email".to_string(),
            refresh_token: "".to_string(),
            description: None,
            token: "".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status: UserStatus::Active,
        }

    }

  



}

// pub fn get_user_by_id(id: Uuid) -> Result<Json<User>, Json<OurError>> {
    
// }

async fn validate_email(email: &str) -> Result<(), &'static str> {
    const EMAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
    let email_regex = Regex::new(EMAIL_REGEX).unwrap();
    if !email_regex.is_match(email) {
        return Err("email is not valid");
    } else {
        return Ok(());
    }
}

async fn validate_password(password: &str) -> Result<(), &'static str> {
    let entropy = zxcvbn(password, &[]);
    if entropy.is_err() || entropy.unwrap().score() < 3 {
        return Err("weak password")
    } else {
        return Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginUser<'r> {
    pub username: &'r str,
    pub password: &'r str,
}


impl<'r> User {
    pub async fn auth(
        login_user: LoginUser<'r>, 
        conn: &mut PgConnection,
        jwt_secret: &[u8],
    ) -> Result<Self, OurError> {
        println!("loginuser = {:?}", login_user);
        let password = login_user.password.to_string();
        let username = login_user.username.to_string();

        let user_data = User
            ::find_user_by_username(conn, username)
                .await;
                // .map_err(|e| {
                //     e.to_string()
                // });
        
        if user_data.is_err() {
            return Err(user_data.err().unwrap())
        }
        
        // // TODO validate users password.
        let user = user_data.unwrap();
        let password_hash = &user.password_hash;
        let verify_password = User
            ::verify_password_hash(&password, password_hash);
                // .map_err(|e| {
                //     e.to_string()
                // });
        if verify_password.is_err() {
            return Err(verify_password.err().unwrap())
        }

        let get_tokens = User
            ::generate_tokens(&user.uuid, jwt_secret, &user.username);
        let update_tokens_user = User
            ::update_token_to_db(
                &user.uuid, 
                &get_tokens.token, 
                &get_tokens.refresh_token, 
                conn
            )
                .await;
                // .map_err(|e| {
                //     // e
                // });

        // let 

        

        update_tokens_user

        // Ok(User {
        //     username: "".to_string(),
        //     email: "".to_string(),
        //     password_hash: "".to_string(),
        //     created_at: chrono::Utc::now(),
        //     updated_at: chrono::Utc::now(),
        //     uuid: Uuid::new_v4(),
        //     refresh_token: "".to_string(),
        //     token: "".to_string(),
        //     description: None,
        //     status: UserStatus::Active,
        // })
    }
}

pub async fn some_simple() -> usize { 8 }