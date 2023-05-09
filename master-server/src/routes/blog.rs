use crate::{
    fairings::db::DBConnection,
    models::post::{
        Post,
        NewPost,
    },
    guards::jwt_header::ApiUser, errors::OurError,
};
use rocket::{
    serde::json::Json,
    State,
};
use rocket_db_pools::{
    sqlx::Acquire,
    Connection,
};
use crate::states::JWToken;

#[post(
    "/blogs",
    format = "json",
    data = "<new_blog>",
)]
pub async fn create_new_blog(
    new_blog: Json<NewPost>,
    mut db: Connection<DBConnection>,
    user: ApiUser,
) -> Result<Json<Post>, Json<OurError>> {
    let conn = db.acquire()
        .await
        .unwrap();
    let unwraped_blog = new_blog.into_inner();
    // let myblog = Post::new(
    //     unwraped_blog.title,
    //     unwraped_blog.body,
    //     unwraped_blog.tags,
    // );
    let myblog = Post
        ::create(
            unwraped_blog.title,
            unwraped_blog.body,
            unwraped_blog.tags,
            conn,
            user,
        ).await;
    if myblog.is_err() {
        return Err(Json(myblog.err().unwrap()))
    }
    Ok(Json(myblog.unwrap()))
}