use ahash::AHashMap;
use diesel_async::{
    pooled_connection::bb8::{Pool, PooledConnection},
    AsyncPgConnection,
};
use std::sync::Arc;

pub type Backend = diesel::pg::Pg;
pub type MyDbConnection = AsyncPgConnection;
pub type DbPool = Pool<MyDbConnection>;
pub type ArcDbPool = Arc<DbPool>;
pub type DbPoolConnection<'a> = PooledConnection<'a, MyDbConnection>;

// the max is actually u16::MAX but we see that when the size is too big we get an overflow error so reducing it a bit
const MAX_DIESEL_PARAM_SIZE: usize = (u16::MAX / 2) as usize;

/// Returns the entry for the config hashmap, or the default field count for the insert
/// Given diesel has a limit of how many parameters can be inserted in a single operation (u16::MAX),
/// we default to chunk an array of items based on how many columns are in the table.
pub fn get_config_table_chunk_size<T: field_count::FieldCount>(
    table_name: &str,
    per_table_chunk_sizes: &AHashMap<String, usize>,
) -> usize {
    per_table_chunk_sizes
        .get(table_name)
        .copied()
        .unwrap_or_else(|| MAX_DIESEL_PARAM_SIZE / T::field_count())
}
