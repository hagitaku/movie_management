use crate::form::movie_create::MovieCreateRequest;
use crate::model::movie::{ActiveModel, Model};
use actix_web::web;
use chrono::prelude::*;
use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection};

/**
 * 映画登録の処理
 */
pub async fn insert_movie(
    conn: &DatabaseConnection,
    request: &web::Json<MovieCreateRequest>,
) -> Result<Model, sea_orm::DbErr> {
    // created_at に入れる値を取得
    // フォーマットは yyy-mm-dd
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let data: ActiveModel = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(request.title.clone()),
        description: ActiveValue::Set(request.description.clone()),
        created_user_id: ActiveValue::Set(request.created_user_id),
        created_at: ActiveValue::Set(today.to_string()),
    };

    let ret: Result<Model, sea_orm::DbErr> = data.insert(conn).await;

    return ret;
}

pub async fn register_movie(pool: &PgPool, req: CreateMovieRequest) -> Result<Movie, sqlx::Error> {
    let movie = sqlx::query_as::<_, Movie>(
        "INSERT INTO movies (title, description, release_date, rating, created_at) 
         VALUES ($1, $2, $3, $4, NOW()) 
         RETURNING id, title, description, release_date, rating, created_at",
    )
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.release_date)
    .bind(req.rating)
    .fetch_one(pool)
    .await?;

    Ok(movie)
}
