use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres, Sqlite, SqlitePool};

use crate::cli::Args;
use crate::schema::User;

pub mod postgres;
pub mod sqlite;

pub struct DBConn {
    pub psql: Option<Pool<Postgres>>,
    pub sqlite: Option<Pool<Sqlite>>,
    pub args: Args,
}

impl DBConn {
    pub async fn new() -> Result<DBConn, Error> {
        let args = crate::cli::Args::new();

        if args.storage.is_none() {
            let sqlite_pool = SqlitePool::connect("sqlite://graph_fs.db?mode=rwc").await?;
            sqlx::query(sqlite::CREATE_TABLE)
                .execute(&sqlite_pool)
                .await?;

            Ok(DBConn {
                psql: None,
                sqlite: Some(sqlite_pool),
                args,
            })
        } else {
            if args.db_path.is_none() {
                panic!("please supply a valid db connection string")
            }
            let ps_ql_pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(args.clone().db_path.unwrap().as_str())
                .await?;
            sqlx::query(postgres::CREATE_TABLE)
                .execute(&ps_ql_pool)
                .await?;
            Ok(DBConn {
                psql: Some(ps_ql_pool),
                sqlite: None,
                args,
            })
        }
    }

    pub async fn create_user(&self, password: &str) -> Result<(), Error> {
        if self.args.storage.is_none() {
            let _row: (i64,) = sqlx::query_as(sqlite::CRAETE_NEW_USER_SQL)
                .bind(self.args.clone().account_name)
                .bind(self.args.clone().account_email)
                .bind(password)
                .bind(self.args.clone().account_permission)
                .fetch_one(self.sqlite.as_ref().unwrap())
                .await?;
            Ok(())
        } else {
            let _row: (i64,) = sqlx::query_as(postgres::CRAETE_NEW_USER_SQL)
                .bind(self.args.clone().account_name)
                .bind(self.args.clone().account_email)
                .bind(password)
                .bind(self.args.clone().account_permission)
                .fetch_one(self.psql.as_ref().unwrap())
                .await?;
            Ok(())
        }
    }

    pub async fn get_user_by_id(&self, id: &i64) -> Result<Vec<User>, Error> {
        if self.args.storage.is_none() {
            let select_query = sqlx::query_as::<_, User>(sqlite::GET_USER_BY_ID_SQL).bind(id);
            let users: Vec<User> = select_query
                .fetch_all(self.sqlite.as_ref().unwrap())
                .await?;
            Ok(users)
        } else {
            let select_query = sqlx::query_as::<_, User>(postgres::GET_USER_BY_ID_SQL).bind(id);
            let pg_users: Vec<User> = select_query.fetch_all(self.psql.as_ref().unwrap()).await?;
            Ok(pg_users)
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Vec<User>, Error> {
        if self.args.storage.is_none() {
            let select_query = sqlx::query_as::<_, User>(sqlite::GET_USER_BY_EMAIL_SQL).bind(email);
            let users: Vec<User> = select_query
                .fetch_all(self.sqlite.as_ref().unwrap())
                .await?;
            Ok(users)
        } else {
            let select_query =
                sqlx::query_as::<_, User>(postgres::GET_USER_BY_EMAIL_SQL).bind(email);
            let pg_users: Vec<User> = select_query.fetch_all(self.psql.as_ref().unwrap()).await?;
            Ok(pg_users)
        }
    }

    pub async fn update_user_permission(
        &self,
        permission: &str,
        email: &str,
    ) -> Result<bool, Error> {
        if self.args.storage.is_none() {
            let affected = sqlx::query(sqlite::UPDATE_PERMISSION_SQL)
                .bind(permission)
                .bind(email)
                .execute(self.sqlite.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        } else {
            let affected = sqlx::query(postgres::UPDATE_PERMISSION_SQL)
                .bind(permission)
                .bind(email)
                .execute(self.psql.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        }
    }

    pub async fn update_user_password(&self, password: &str, email: &str) -> Result<bool, Error> {
        if self.args.storage.is_none() {
            let affected = sqlx::query(sqlite::UPDATE_PASSWORD_SQL)
                .bind(password)
                .bind(email)
                .execute(self.sqlite.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        } else {
            let affected = sqlx::query(postgres::UPDATE_PASSWORD_SQL)
                .bind(password)
                .bind(email)
                .execute(self.psql.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        }
    }

    pub async fn update_user_name(&self, name: &str, email: &str) -> Result<bool, Error> {
        if self.args.storage.is_none() {
            let affected = sqlx::query(sqlite::UPDATE_NAME_SQL)
                .bind(name)
                .bind(email)
                .execute(self.sqlite.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        } else {
            let affected = sqlx::query(postgres::UPDATE_NAME_SQL)
                .bind(name)
                .bind(email)
                .execute(self.psql.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        }
    }

    pub async fn delete_user(&self, email: &str) -> Result<bool, Error> {
        if self.args.storage.is_none() {
            let affected = sqlx::query(sqlite::DELETE_USER_SQL)
                .bind(email)
                .execute(self.sqlite.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        } else {
            let affected = sqlx::query(postgres::DELETE_USER_SQL)
                .bind(email)
                .execute(self.psql.as_ref().unwrap())
                .await?
                .rows_affected();
            Ok(affected > 0)
        }
    }
}
