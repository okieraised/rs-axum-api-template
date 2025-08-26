use once_cell::sync::OnceCell;

use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection}, // <- use adapter's Pool
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
    AsyncPgConnection,
};
use bb8::RunError;

pub type DbPool = Pool<AsyncPgConnection>;
pub type DbConn = PooledConnection<'static, AsyncPgConnection>;
pub type DbError = RunError<PoolError>;

static DB_POOL: OnceCell<DbPool> = OnceCell::new();

/// Initialize the global async Diesel pool once at startup.
/// Safe to call multiple times; the first successful call wins.
pub async fn init(database_url: &str, max_connections: u32) -> Result<(), DbError> {
    if DB_POOL.get().is_some() {
        return Ok(());
    }
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool = Pool::builder().max_size(max_connections).build(manager).await?;
    let _ = DB_POOL.set(pool);
    Ok(())
}

/// Borrow the global pool (panics if `init` wasn’t called).
#[inline]
pub fn pool() -> &'static DbPool {
    DB_POOL.get().expect("DB pool not initialized; call db::init(...) first")
}

/// Get a pooled async connection.
#[inline]
pub async fn conn() -> Result<DbConn, DbError> {
    pool().get().await
}
