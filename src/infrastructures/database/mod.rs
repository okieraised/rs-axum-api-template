use std::time::Duration;
use once_cell::sync::OnceCell;

use bb8::RunError;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::bb8::{Pool, PooledConnection}, // <- use adapter's Pool
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
};

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConn = PooledConnection<'static, AsyncPgConnection>;
pub type DbError = RunError<PoolError>;

static DB_POOL: OnceCell<DbPool> = OnceCell::new();

/// Initialize the global async Diesel pool once at startup.
/// Safe to call multiple times; the first successful call wins.
pub async fn init_database_connection(
    database_url: &str, max_connections: u32,
) -> Result<(), DbError> {
    if DB_POOL.get().is_some() {
        return Ok(());
    }
    let manager =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder()
        .connection_timeout(Duration::from_secs(2))
        .max_size(max_connections)
        .build(manager)
        .await?;
    let _ = DB_POOL.set(pool);
    Ok(())
}

/// Borrow the global pool (panics if `init` wasn’t called).
#[inline]
pub fn pool() -> &'static DbPool {
    DB_POOL
        .get()
        .expect("DB pool not initialized; call db::init(...) first")
}

/// Get a pooled async connection.
#[inline]
pub async fn conn() -> Result<DbConn, DbError> {
    pool().get().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::sql_types::BigInt;
    use diesel::{QueryableByName, sql_query};
    use diesel_async::RunQueryDsl;
    use serial_test::serial;

    #[derive(Debug, QueryableByName)]
    struct RowCount {
        #[diesel(sql_type = BigInt)]
        total: i64,
    }

    #[tokio::test(flavor = "multi_thread")]
    #[serial]
    async fn test_new_database_connection() {
        let url: &str =
            "";
        init_database_connection(&url, 5).await.expect("init ok");

        let mut c = conn().await.expect("get pooled connection");
        let RowCount { total } =
            sql_query("SELECT COUNT(*) AS total FROM records_v2")
                .get_result::<RowCount>(&mut c)
                .await
                .expect("count query");

        println!("records_v2 count = {}", total);
        assert!(total >= 0);
    }
}
