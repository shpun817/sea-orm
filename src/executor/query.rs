use crate::{debug_print, DbErr};
use std::fmt;

#[derive(Debug)]
pub struct QueryResult {
    pub(crate) row: QueryResultRow,
}

pub(crate) enum QueryResultRow {
    #[cfg(feature = "sqlx-mysql")]
    SqlxMySql(sqlx::mysql::MySqlRow),
    #[cfg(feature = "sqlx-postgres")]
    SqlxPostgres(sqlx::postgres::PgRow),
    #[cfg(feature = "sqlx-sqlite")]
    SqlxSqlite(sqlx::sqlite::SqliteRow),
    #[cfg(feature = "mock")]
    Mock(crate::MockRow),
}

pub enum TryGetError {
    DbErr(DbErr),
    Null,
}

impl From<TryGetError> for DbErr {
    fn from(e: TryGetError) -> DbErr {
        match e {
            TryGetError::DbErr(e) => e,
            TryGetError::Null => DbErr::Query("error occurred while decoding: Null".to_owned()),
        }
    }
}

pub trait TryGetable {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError>
    where
        Self: Sized;
}

// QueryResult //

impl QueryResult {
    pub fn try_get<T>(&self, pre: &str, col: &str) -> Result<T, DbErr>
    where
        T: TryGetable,
    {
        Ok(T::try_get(self, pre, col)?)
    }
}

impl fmt::Debug for QueryResultRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            #[cfg(feature = "sqlx-mysql")]
            Self::SqlxMySql(row) => write!(f, "{:?}", row),
            #[cfg(feature = "sqlx-postgres")]
            Self::SqlxPostgres(_) => panic!("QueryResultRow::SqlxPostgres cannot be inspected"),
            #[cfg(feature = "sqlx-sqlite")]
            Self::SqlxSqlite(_) => panic!("QueryResultRow::SqlxSqlite cannot be inspected"),
            #[cfg(feature = "mock")]
            Self::Mock(row) => write!(f, "{:?}", row),
        }
    }
}

// TryGetable //

impl<T: TryGetable> TryGetable for Option<T> {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match T::try_get(res, pre, col) {
            Ok(v) => Ok(Some(v)),
            Err(TryGetError::Null) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

macro_rules! try_getable_all {
    ( $type: ty ) => {
        impl TryGetable for $type {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                let column = format!("{}{}", pre, col);
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        TryGetError::Null
                    }),
                }
            }
        }
    };
}

macro_rules! try_getable_unsigned {
    ( $type: ty ) => {
        impl TryGetable for $type {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                let column = format!("{}{}", pre, col);
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(_) => {
                        panic!("{} unsupported by sqlx-postgres", stringify!($type))
                    }
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        TryGetError::Null
                    }),
                }
            }
        }
    };
}

macro_rules! try_getable_mysql {
    ( $type: ty ) => {
        impl TryGetable for $type {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                let column = format!("{}{}", pre, col);
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(_) => {
                        panic!("{} unsupported by sqlx-postgres", stringify!($type))
                    }
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(_) => {
                        panic!("{} unsupported by sqlx-sqlite", stringify!($type))
                    }
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        TryGetError::Null
                    }),
                }
            }
        }
    };
}

macro_rules! try_getable_postgres {
    ( $type: ty ) => {
        impl TryGetable for $type {
            fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
                let column = format!("{}{}", pre, col);
                match &res.row {
                    #[cfg(feature = "sqlx-mysql")]
                    QueryResultRow::SqlxMySql(_) => {
                        panic!("{} unsupported by sqlx-mysql", stringify!($type))
                    }
                    #[cfg(feature = "sqlx-postgres")]
                    QueryResultRow::SqlxPostgres(row) => {
                        use sqlx::Row;
                        row.try_get::<Option<$type>, _>(column.as_str())
                            .map_err(crate::sqlx_error_to_query_err)
                            .and_then(|opt| opt.ok_or_else(TryGetError::Null))
                    }
                    #[cfg(feature = "sqlx-sqlite")]
                    QueryResultRow::SqlxSqlite(_) => {
                        panic!("{} unsupported by sqlx-sqlite", stringify!($type))
                    }
                    #[cfg(feature = "mock")]
                    QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
                        debug_print!("{:#?}", e.to_string());
                        TryGetError::Null
                    }),
                }
            }
        }
    };
}

try_getable_all!(bool);
try_getable_all!(i8);
try_getable_all!(i16);
try_getable_all!(i32);
try_getable_all!(i64);
try_getable_unsigned!(u8);
try_getable_unsigned!(u16);
try_getable_all!(u32);
try_getable_mysql!(u64);
try_getable_all!(f32);
try_getable_all!(f64);
try_getable_all!(String);

#[cfg(feature = "with-json")]
try_getable_all!(serde_json::Value);

#[cfg(feature = "with-chrono")]
try_getable_all!(chrono::NaiveDateTime);

#[cfg(feature = "with-chrono")]
try_getable_postgres!(chrono::DateTime<chrono::FixedOffset>);

#[cfg(feature = "with-rust_decimal")]
use rust_decimal::Decimal;

#[cfg(feature = "with-rust_decimal")]
impl TryGetable for Decimal {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        let column = format!("{}{}", pre, col);
        match &res.row {
            #[cfg(feature = "sqlx-mysql")]
            QueryResultRow::SqlxMySql(row) => {
                use sqlx::Row;
                row.try_get::<Option<Decimal>, _>(column.as_str())
                    .map_err(crate::sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-postgres")]
            QueryResultRow::SqlxPostgres(row) => {
                use sqlx::Row;
                row.try_get::<Option<Decimal>, _>(column.as_str())
                    .map_err(crate::sqlx_error_to_query_err)
            }
            #[cfg(feature = "sqlx-sqlite")]
            QueryResultRow::SqlxSqlite(row) => {
                use sqlx::Row;
                let val: Option<f64> = row
                    .try_get(column.as_str())
                    .map_err(crate::sqlx_error_to_query_err)?;
                use rust_decimal::prelude::FromPrimitive;
                match val {
                    Some(v) => Decimal::from_f64(v)
                        .ok_or_else(|| DbErr::Query("Failed to convert f64 into Decimal".to_owned())),
                    None => Err(TryGetError::Null)
                }
            }
            #[cfg(feature = "mock")]
            QueryResultRow::Mock(row) => row.try_get(column.as_str()).map_err(|e| {
                debug_print!("{:#?}", e.to_string());
                TryGetError::Null
            }),
        }
    }
}

#[cfg(feature = "with-uuid")]
try_getable_all!(uuid::Uuid);
