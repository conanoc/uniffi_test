use tokio::sync::RwLock;
use std::{ sync::Arc, time::Duration, str::FromStr };
use sqlx::{
    sqlite::{
        SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode,
        SqlitePoolOptions, SqliteSynchronous,
    },
    migrate::MigrateDatabase, Sqlite, SqlitePool, pool::PoolConnection,
};

uniffi::include_scaffolding!("bug_finder");

const DB_URL: &str = "sqlite:///tmp/test.db";

pub struct Store {
    pool: RwLock<Option<SqlitePool>>,
}

pub struct Session {
    connection: PoolConnection<Sqlite>,
}

#[uniffi::export(async_runtime = "tokio")]
pub async fn create_store() -> Arc<Store> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let conn_opts = SqliteConnectOptions::from_str(DB_URL).unwrap()
        .create_if_missing(true)
        .auto_vacuum(SqliteAutoVacuum::Incremental)
        .busy_timeout(Duration::from_secs(5))
        .journal_mode(SqliteJournalMode::Wal)
        .locking_mode(SqliteLockingMode::Normal)
        .shared_cache(true)
        .synchronous(SqliteSynchronous::Full);
    let pool = SqlitePoolOptions::default()
        .min_connections(1)
        .max_connections(4)
        .test_before_acquire(false)
        .connect_with(conn_opts)
        .await
        .unwrap();
    // let pool = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);")
                    .execute(&pool).await.unwrap();
    println!("Create table result: {:?}", result);
    Arc::new(Store { pool: RwLock::new(Some(pool)) })
}

#[uniffi::export(async_runtime = "tokio")]
impl Store {
    pub async fn session(&self) -> Arc<Session> {
        println!("In count");
        let pool = self.pool.read().await;
        let conn = pool.as_ref().unwrap().acquire().await.unwrap();
        Arc::new(Session { connection: conn })
    }

    pub async fn close(&self) {
        println!("In close");
        let pool = self.pool.write().await.take();
        println!("before close");
        pool.unwrap().close().await;
        println!("after close");
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Session {
    pub async fn count(&self) -> u32 {
        println!("In count");
        let count: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&mut self.connection)
            .await
            .unwrap();
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::future;

    #[test]
    fn close_test() {
        future::block_on(async_compat::Compat::new(async {
            let store = create_store().await;
            let session = store.session().await;
            let count = session.count().await;
            println!("count: {}", count);
            drop(session);
            store.close().await;
        }));
    }
}
