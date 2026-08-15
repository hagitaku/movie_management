use crate::form::movie_create::MovieCreateRequest;
use crate::model::movie::{ActiveModel, Model};
use actix_web::web;
use chrono::Local;
use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection};

pub async fn insert_movie(
    conn: &DatabaseConnection,
    request: &web::Json<MovieCreateRequest>,
    account_id: u32,
) -> Result<Model, sea_orm::DbErr> {
    let data = ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(request.title.clone()),
        description: ActiveValue::Set(request.description.clone()),
        memo: ActiveValue::Set(request.memo.clone()),
        created_user_id: ActiveValue::Set(account_id),
        created_at: ActiveValue::Set(Local::now().naive_local()),
    };

    data.insert(conn).await
}
