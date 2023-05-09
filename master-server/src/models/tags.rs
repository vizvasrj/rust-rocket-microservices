use sqlx::{FromRow, PgConnection, Error::RowNotFound};
use uuid::Uuid;
use chrono::DateTime;
use serde::{
    Serialize,
    Deserialize,
};

use crate::errors::OurError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tags {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<chrono::Utc>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TestTags {
    pub uuid: Option<Uuid>,
}

impl From<String> for Tags {
    fn from(name: String) -> Self {
        Tags {
            uuid: Uuid::new_v4(),
            name: name,
            created_at: chrono::Utc::now(),
        }
    }

}

pub async fn exists(
    name: String, 
    conn: &mut PgConnection,
) -> Result<Tags, sqlx::Error> {
    let query_str = r#"
    SELECT * FROM tags
    WHERE name = $1
    "#;
    let tag = sqlx::query_as
        ::<_, Tags>(query_str)
            .bind(name)
            .fetch_one(conn)
            .await;
            // .map_err(|e| {
            //     OurError::from_sqlx_error(e)
            // });
    tag

}


impl Tags {
    pub async fn exists(
        name: String, 
        conn: &mut PgConnection,
    ) -> Result<Tags, sqlx::Error> {
        let query_str = r#"
        SELECT * FROM tags
        WHERE name = $1
        "#;
        let tag = sqlx::query_as
            ::<_, Tags>(query_str)
                .bind(name)
                .fetch_one(conn)
                .await;
                // .map_err(|e| {
                //     OurError::from_sqlx_error(e)
                // });
        tag
    
    }

    pub async fn create(
        name: String,
        conn: &mut PgConnection,
    ) -> Result<Self, OurError> {
        let query_str = r#"
        INSERT INTO tags (uuid, name)
        VALUES ($1, $2)
        RETURNING *
        "#;
        let tag = sqlx::query_as
            ::<_, Tags>(query_str)
            .bind(Uuid::new_v4())
            .bind(name)
            .fetch_one(conn)
            .await
            .map_err(|e| {
                OurError::from_sqlx_error(e)
            });

        tag
    }

}
