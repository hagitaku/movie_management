use once_cell::sync::Lazy;
use sea_orm::Database;
use std::sync::Mutex;
use tokio::runtime::Runtime;

const DATABASE_URL: &str = "mysql://user:password@localhost:3306";

// DBへの接続。Lazyで初期化を遅延させている
// Mutexで排他制御を行う
// pubをつけて外部からアクセスできるようにしている
pub static DB: Lazy<
    Mutex<sea_orm::prelude::DatabaseConnection>,
    fn() -> Mutex<sea_orm::prelude::DatabaseConnection>,
> = Lazy::new(|| {
    let rt: Runtime = Runtime::new().unwrap();
    let db: sea_orm::prelude::DatabaseConnection =
        rt.block_on(async { Database::connect(DATABASE_URL).await.unwrap() });
    Mutex::new(db)
});

pub fn db_init() {
    let _db: std::sync::MutexGuard<'_, sea_orm::prelude::DatabaseConnection> = DB.lock().unwrap();
}
