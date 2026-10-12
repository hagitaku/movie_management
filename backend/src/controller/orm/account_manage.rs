use crate::form::account_manage::{AccountRegisterRequest, LoginRequest};
use crate::model::account::Entity as AccountEntity;
use crate::model::account::{ActiveModel, Model};
use crate::model::session::{ActiveModel as SessionActiveModel, Entity as SessionEntity};
use actix_web::web;
use chrono::{Duration, Utc};
use sea_orm::entity::ActiveValue;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};

pub async fn insert_account(
    conn: &DatabaseConnection,
    request: &web::Json<AccountRegisterRequest>,
) -> Result<Model, sea_orm::DbErr> {
    let data = ActiveModel {
        id: ActiveValue::NotSet,
        login_id: ActiveValue::Set(request.id.clone()),
        password: ActiveValue::Set(request.password.clone()),
    };

    data.insert(conn).await
}

pub async fn login_account(
    conn: &DatabaseConnection,
    request: &web::Json<LoginRequest>,
) -> Result<Model, sea_orm::DbErr> {
    AccountEntity::find()
        .filter(
            crate::model::account::Column::LoginId
                .eq(request.id.clone())
                .and(crate::model::account::Column::Password.eq(request.password.clone())),
        )
        .one(conn)
        .await?
        .ok_or_else(|| sea_orm::DbErr::RecordNotFound("Account not found".to_owned()))
}

/// 同じアカウントの既存セッションを無効化して、新しいセッションを保存する。
pub async fn replace_session(
    conn: &DatabaseConnection,
    account_id: u32,
    token: &str,
) -> Result<(), sea_orm::DbErr> {
    let transaction = conn.begin().await?;

    SessionEntity::delete_many()
        .filter(crate::model::session::Column::AccountId.eq(account_id))
        .exec(&transaction)
        .await?;

    SessionActiveModel {
        token: ActiveValue::Set(token.to_owned()),
        account_id: ActiveValue::Set(account_id),
        expiration: ActiveValue::Set((Utc::now() + Duration::hours(24)).naive_utc()),
    }
    .insert(&transaction)
    .await?;

    transaction.commit().await
}

pub async fn account_id_from_session(
    conn: &DatabaseConnection,
    token: &str,
) -> Result<Option<u32>, sea_orm::DbErr> {
    let session = SessionEntity::find_by_id(token).one(conn).await?;

    match session {
        Some(session) if session.expiration > Utc::now().naive_utc() => {
            Ok(Some(session.account_id))
        }
        Some(_) => {
            SessionEntity::delete_by_id(token).exec(conn).await?;
            Ok(None)
        }
        None => Ok(None),
    }
}
