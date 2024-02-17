use crate::db_connector::DB; // DB変数をインポートします。

use actix_web::{get, Responder};
use sea_orm::error::DbErr;
use std::result::Result;
use tokio::runtime::Runtime;

#[get("/")]
pub async fn hello() -> impl Responder {
    match health_check() {
        Ok(_) => "Ok".to_string(),
        Err(_) => "NG".to_string(),
    }
}

pub fn health_check() -> Result<(), DbErr> {
    let db = DB.lock().unwrap();
    let rt = Runtime::new().unwrap();
    rt.block_on(async { ping(&db).await })
}

async fn ping(db: &sea_orm::prelude::DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await?;
    Ok(())
}
