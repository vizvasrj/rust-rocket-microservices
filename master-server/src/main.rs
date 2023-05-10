#[macro_use]
extern crate rocket;
// use std::panic;

// use rocket::fairing::AdHoc;
// use rocket::figment::Figment;
// use rocket::figment::providers::{Format, Env};
use rocket::{Build, Rocket, 
    // State
};
use rocket::serde::json::Json;
use rocket_db_pools::Database;
// use rocket_db_pools::sqlx::postgres::PgPoolOptions;
use master_server::errors::OurError;
use rocket::serde::{Serialize, Deserialize};
use master_server::fairings::db::DBConnection;
use master_server::states::JWToken;
use dotenv;
// use sqlx::PgPool;
use master_server::routes::{blog, user};
// use figment::{Figment, providers::{Env}};


#[allow(dead_code)]
#[derive(Deserialize)]
struct Config {
    database: Databases,
    secret_key: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Databases {
    main_connection: MainConection,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct MainConection{
    url: String,
}

#[derive(Serialize)]
pub struct Test {
    pub name: String,
    pub age: usize,
}

#[get("/")]
pub async fn hello_world() -> Result<Json<Test>, Json<OurError>> {
    // let mut conn = db.acquire().await.unwrap();
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&mut conn).await.unwrap();
    // println!("demo test here?? {}", row.0);

    Ok(Json(Test { name: "smename".to_string(), age: 11 }))
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv::dotenv().expect("Failed to load .env");
    // figment
    // let config = Figment::new()
    //     .merge(Env::prefixed("ROCKET_"))
    //     .extract::<Config>();
    // if config.is_err() {
    //     println!("err = {}", config.err().unwrap().to_string());
    //     panic!("a") 
    // }
        // .unwrap();


    // let secret = std::env::var("SECRET_KEY").unwrap();
    
    // let warn = "";
    
    // let fairing = AdHoc::on_ignite("database",move |r| async {
    //     let db_user = std::env::var("POSTGRES_USER").unwrap();
    //     let db_pass = std::env::var("POSTGRES_PASSWORD").unwrap();
    //     let db_name = std::env::var("POSTGRES_DB").unwrap();
    //     let db_host = std::env::var("POSTGRES_HOST").unwrap();
    //     let db_url = format!("postgres://{}:{}@{}/{}", db_user, db_pass, db_host, db_name);
    
    //     let pool = PgPoolOptions::new()
    //         .max_connections(5)
    //         .connect(&db_url)
    //         .await
    //         .expect("Failed to connect to database postgres");

    //     r.manage(pool)
    // });

    let our_rocket = rocket::build()
        // .attach(fairing)
        .attach(DBConnection::init())
        .mount(
            "/", routes![
                hello_world,
                user::create_user,
                user::get_user_by_uuid,
                user::get_user_by_token,
                user::login,
                blog::create_new_blog,

                ]
            );

        let config: Config = our_rocket
            .figment()
            .extract()
            .expect("incorrect Rocket.toml configuration");
    
        let secret_key = JWToken {
            secret: config.secret_key.to_string(),
        };
    
        let finel_rocket = our_rocket.manage(secret_key);
    
    
        finel_rocket
    }
    