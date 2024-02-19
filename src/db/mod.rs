use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres, Sqlite, SqlitePool};

pub mod postgres;
pub mod sqlite;

pub struct DBConn {
    pub psql: Option<Pool<Postgres>>,
    pub sqlite: Option<Pool<Sqlite>>,
}

impl DBConn {
    async fn new() -> Result<DBConn, Error> {
        let args = crate::cli::Args::new();

        if args.storage.is_none() {
            let sqlite_pool = SqlitePool::connect("sqlite://graph_fs.db?mode=rwc").await?;
            Ok(DBConn {
                psql: None,
                sqlite: Some(sqlite_pool),
            })
        } else {
            let ps_ql_pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(args.db_path.unwrap().as_str())
                .await?;
            Ok(DBConn {
                psql: Some(ps_ql_pool),
                sqlite: None,
            })
        }
    }
}
