use chrono::{DateTime};
use sqlx::{PgConnection, FromRow, Error::RowNotFound};
use uuid::Uuid;
use crate::fairings::db::DBConnection;
use crate::{errors::OurError, guards::jwt_header::ApiUser};
use super::bool_wrapper::BoolWrapper;

use super::{tags::Tags, pagination::Pagination};
use serde::{
    Serialize,
    Deserialize,
};
use rocket_db_pools::{
    Connection,
    sqlx::Acquire,
};

#[derive(FromRow)]
pub struct SqlxPost {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Serialize, Debug, Deserialize, FromRow)]
pub struct Post {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
    pub tags: Vec<Tags>,
}



#[derive(Debug, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}

impl Post {
    pub fn new(
        title: String,
        body: String,
        tags: Vec<String>,
    ) -> Self {
        let mut mytags = Vec::with_capacity(5);
        for x in tags {
            mytags.push(Tags::from(x))
        }

        Post {
            uuid: Uuid::new_v4(),
            user_uuid: Uuid::new_v4(),
            title,
            body,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            tags: mytags,
        }
    }

    async fn add(
        uuid: Uuid,
        title: String,
        body: String,
        conn: &mut PgConnection
    ) -> Result<SqlxPost, OurError> {
        let query_str = r#"
        INSERT INTO posts (
            uuid, user_uuid, title, body,
            created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#;
        let post = sqlx::query_as
            ::<_, SqlxPost>(query_str)
                .bind(Uuid::new_v4())
                .bind(uuid)
                .bind(title)
                .bind(body)
                .bind(chrono::Utc::now())
                .bind(chrono::Utc::now())
                .fetch_one(conn)
                .await
                .map_err(|e| {
                    OurError::from_sqlx_error(e)
                });
        post

    }

    // cancel transaction if any error happens while this all
    pub async fn create(
        title: String,
        body: String,
        tags: Vec<String>,
        conn: &mut PgConnection,
        user: ApiUser,
        // jwt_secret: &[u8],
    ) -> Result<Self, OurError> {
        let post = Post
            ::add(
                user.user.uuid,
                title,
                body,
                conn,
            )
                .await;
        // get uuid
        let mut tags_uuids = Vec::new();
        for x in tags {
            let u = Tags::exists(x.clone(), conn)
                .await;
            match u {
                Ok(tag) => {
                    tags_uuids.push(tag)
                },
                Err(e) => match e {
                    RowNotFound => {
                        let tag = Tags
                            ::create(x, conn)
                                .await;
                                // .map_err(|e| {
                                //     OurError::from_sqlx_error(e)
                                // });
                        if tag.is_err() {
                            return Err(tag.err().unwrap());
                        }
                        tags_uuids.push(tag.unwrap())
                    },
                    _ => return Err(
                        OurError::from_sqlx_error(e)
                    ),
                }
            }
        }

        if post.is_err() {
            return Err(post.err().unwrap())
        }
        
        // TODO add post_tag many to many fields/
        let post = post.unwrap();
        for x in tags_uuids.iter() {
            let many = Post
                ::post_tag_add(
                    post.uuid, 
                    x.uuid, 
                    conn
                ).await;
            if many.is_err() {
                return Err(many.err().unwrap())
            }
        }


        // let post = post.map(|p| {
        Ok(Post {
            uuid: post.uuid,
            title: post.title,
            body: post.body,
            user_uuid: post.user_uuid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            tags: tags_uuids,
        })
        // });
        // post
    }

    pub async fn post_tag_add(
        post_uuid: Uuid, 
        tag_uuid: Uuid,
        conn: &mut PgConnection,
    ) -> Result<(), OurError> {
        let query_str = r#"
        INSERT INTO post_tag (post_uuid, tag_uuid)
        VALUES ($1, $2)
        "#;
        let s = sqlx
            ::query(query_str)
            .bind(post_uuid)
            .bind(tag_uuid)
            .execute(conn)
            .await
            .map_err(|e| {
                OurError::from_sqlx_error(e)
            });
        if s.is_err() {
            return Err(s.err().unwrap())
        }
        Ok(())
    }

    // this function will find specific user uuid posts
    // with pagination
    pub async fn find_all_posts_with_uuid_pagination(
        db: &mut Connection<DBConnection>,
        user_uuid: &Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<SqlxPost>, Option<Pagination>), OurError> {
        let query_str = r#"
        SELECT * 
        FROM posts
        WHERE 
            user_uuid = $1 AND created_at < $2
        ORDER BY created_at DESC
        LIMIT $3
        "#;
        let conn = db.acquire().await.unwrap();
        let posts = sqlx
            ::query_as::<_, SqlxPost>(query_str)
            .bind(user_uuid)
            .bind(pagination.next)
            .bind(pagination.limit as i32)
            .fetch_all(conn)
            .await
            .map_err(|e| OurError::from_sqlx_error(e));
            // .aw
        if posts.is_err() {
            return Err(posts.err().unwrap())
        }
        let posts = posts.unwrap();
        let mut new_pagination: Option<Pagination> = None;
        if posts.len() == pagination.limit {
            let query_str = r#"
            SELECT EXISTS
            (
                SELECT 1 
                FROM posts 
                WHERE
                    created_at < $1 AND user_uuid = $2
                ORDER BY created_at DESC LIMIT 1
            )
            "#;
            let conn = db.acquire().await.unwrap();
            let exists = sqlx
                ::query_as::<_, BoolWrapper>(query_str)
                .bind(posts.last().unwrap().created_at)
                .bind(posts.last().unwrap().user_uuid)
                .fetch_one(conn)
                .await
                .map_err(|e| OurError::from_sqlx_error(e))?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: posts.last().unwrap().created_at.to_owned(),
                    limit: pagination.limit,
                });
            }
        }
        
        Ok((posts, new_pagination))
    }

}
