use crate::form::account_manage::{AccountRegisterRequest, LoginRequest};
use crate::model::account::Entity as AccountEntity;
use crate::model::account::{ActiveModel, Model};
use actix_web::web;
use sea_orm::entity::ActiveValue;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

/**
 * アカウント登録の処理
 * TODO: 先に値があるかを判定する処理を実装する
 */
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

pub async fn login_account(
    conn: &DatabaseConnection,
    request: &web::Json<LoginRequest>,
) -> Result<Model, sea_orm::DbErr> {
    let ret = AccountEntity::find()
        .filter(
            crate::model::account::Column::LoginId
                .eq(request.id.clone())
                .and(crate::model::account::Column::Password.eq(request.password.clone())),
        )
        .one(conn)
        .await?;

    match ret {
        Some(model) => Ok(model),
        None => Err(sea_orm::DbErr::RecordNotFound(
            "Account not found".to_string(),
        )),
    }
}
