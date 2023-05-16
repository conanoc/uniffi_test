use tokio::sync::{ RwLock, Mutex };
use std::{ sync::Arc };
use sqlx::{
    migrate::MigrateDatabase, Sqlite, SqlitePool, pool::PoolConnection,
};

uniffi::include_scaffolding!("bug_finder");

const DB_URL: &str = "sqlite:///tmp/test.db";

pub struct Store {
    pool: RwLock<Option<SqlitePool>>,
}

pub struct Session {
    connection: Mutex<PoolConnection<Sqlite>>,
}

impl Drop for Session {
    fn drop(&mut self) {
        println!("Session dropped and connection pool will be released");
    }
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

    let pool = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);")
                    .execute(&pool).await.unwrap();
    println!("Create table result: {:?}", result);

    Arc::new(Store { pool: RwLock::new(Some(pool)) })
}

#[uniffi::export(async_runtime = "tokio")]
impl Store {
    pub async fn session(&self) -> Arc<Session> {
        let pool = self.pool.read().await;
        let conn = pool.as_ref().unwrap().acquire().await.unwrap();
        Arc::new(Session { connection: Mutex::new(conn) })
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
        let mut conn = self.connection.lock().await;
        let count: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(conn.as_mut())
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
