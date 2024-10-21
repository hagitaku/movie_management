use crate::form::account_manage::AccountRegisterRequest;
use crate::model::account::{ActiveModel, Model};
use actix_web::web;
use sea_orm::entity::ActiveValue;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

pub async fn insert_account(
    conn: &DatabaseConnection,
    request: &web::Json<AccountRegisterRequest>,
) -> Result<Model, sea_orm::DbErr> {
    let data: ActiveModel = ActiveModel {
        id: ActiveValue::NotSet,
        login_id: ActiveValue::Set(request.id.clone()),
        password: ActiveValue::Set(request.password.clone()),
    };

    let ret: Result<crate::model::account::Model, sea_orm::DbErr> = data.insert(conn).await;

    return ret;
}
