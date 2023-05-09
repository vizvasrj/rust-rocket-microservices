use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use jwt_simple::{prelude::{
    HS256Key,
    Claims, Duration, MACLike, JWTClaims,
}, reexports::anyhow::Ok};
use sqlx::PgConnection;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::models::user::User;
use crate::errors::OurError;


pub struct UserToken {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimToken {
    pub username: String,
    pub uuid: Uuid,
}

#[async_trait]
pub trait Token {
    fn get_jwt_token(
        username: &String,
        uuid: &Uuid,
        jwt_secret: &[u8],
    ) -> String;

    fn get_jwt_refresh_token(
        username: &String,
        uuid: &Uuid,
        jwt_secret: &[u8],
    ) -> String;

    fn verify_jwt_token(
        token: String,
        jwt_secret: &[u8]
    ) -> Result<JWTClaims<ClaimToken>, jwt_simple::Error>;

    async fn update_token_to_db(
        uuid: &Uuid,
        token: &String,
        refresh_token: &String,
        conn: &mut PgConnection,
    ) -> Result<User, OurError>;

    fn generate_tokens(
        uuid: &Uuid,
        jwt_secret: &[u8],
        username: &String,
    ) -> UserToken ;

    // fn verify_password_hash(
    //     password: String,
    //     password_hash: String,
    // ) -> Result<(), OurError>;
    
    async fn do_somethine() -> u8;

}

#[async_trait]
impl Token for User {
    fn get_jwt_token(
        username: &String, 
        uuid: &Uuid, 
        jwt_secret: &[u8]
    ) -> String {
        let key = HS256Key::from_bytes(jwt_secret);
        let username = username.to_owned();
        let uuid = uuid.to_owned();
        let custom_claims = ClaimToken {
            username,
            uuid,
        };
        let claims = Claims
            ::with_custom_claims::<ClaimToken>(
                custom_claims,
                Duration::from_secs(30),
            );
        let token = key.authenticate(claims).unwrap();
        token
    }

    fn get_jwt_refresh_token(
        username: &String, 
        uuid: &Uuid, 
        jwt_secret: &[u8]
    ) -> String {
        let key = HS256Key::from_bytes(jwt_secret);
        let username = username.to_owned();
        let uuid = uuid.to_owned();
        let custom_claims = ClaimToken {
            username,
            uuid,
        };
        let claims = Claims
            ::with_custom_claims::<ClaimToken>(
                custom_claims,
                Duration::from_mins(4),
            );
        let token = key.authenticate(claims).unwrap();
        token
    }

    fn verify_jwt_token(token: String, jwt_secret: &[u8]) -> Result<JWTClaims<ClaimToken>, jwt_simple::Error> {
        let key = HS256Key::from_bytes(jwt_secret);
        // let create_user
        let claims = key.verify_token
            ::<ClaimToken>(
                token.as_str(), 
                None,
            );
        if claims.is_err() {
            let c = claims.err();
            return Err(c.unwrap());
        }


        
        // let myclaim = claims.unwrap();
        Ok(claims.unwrap())
    }

    // update token on every logins?? 
    async fn update_token_to_db(
            uuid: &Uuid,
            token: &String,
            refresh_token: &String,
            conn: &mut PgConnection,
        ) -> Result<User, OurError> {
        let query_str = r#"
        UPDATE users
        SET token = $1,
            refresh_token = $2,
            updated_at = $3
        WHERE uuid = $4
        RETURNING *
        "#;
        let user = sqlx::query_as::<_, User>(query_str)
            .bind(token)
            .bind(refresh_token)
            .bind(chrono::Utc::now())
            .bind(uuid)
            .fetch_one(conn)
            .await
            .map_err(|e| {
                // log::error!("{}", e)
                OurError::string_error(
                    e.to_string()
                )
                // e.to_string()
            });
        user
    }

    async fn do_somethine() -> u8 {
        8
    }

    fn generate_tokens(
            uuid: &Uuid,
            jwt_secret: &[u8],
            username: &String,
        ) -> UserToken {
        let token = Self::get_jwt_token(username, uuid, jwt_secret);
        let refresh_token = Self::get_jwt_refresh_token(username, uuid, jwt_secret);
        return UserToken {token, refresh_token};
    }

    // take password as "asdfgh",
    // and take hash from database "argon:&asdwwasd...LONG"


}