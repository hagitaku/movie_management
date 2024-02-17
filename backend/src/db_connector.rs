use sea_orm::DatabaseConnection;
use std::sync::Mutex;

const DATABASE_URL: &str = "mysql://user:password@localhost:3306";

static DB: Mutex<DatabaseConnection> = Mutex::new(DatabaseConnection::new());

pub async fn db_init() {
    DB = DatabaseConnection::connect(DATABASE_URL).await?;
    ping(db)
}

async fn ping(db: &DatabaseConnection) -> Result<()> {
    db.ping().await;
    Ok(());
}
