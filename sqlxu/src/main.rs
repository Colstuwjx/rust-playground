// simple usage
// use sqlx::mysql::MySqlPoolOptions;

// #[actix_rt::main]
// async fn main() -> Result<(), sqlx::Error> {
//     // Create a connection pool
//     let pool = MySqlPoolOptions::new()
//         .max_connections(5)
//         .connect("mysql://root:test1234@localhost:3306/mbb").await?;

//     // Make a simple query to return the given parameter
//     let row: (i64,) = sqlx::query_as("SELECT 148")
//         .bind(150_i64)
//         .fetch_one(&pool).await?;

//     assert_eq!(row.0, 150);

//     Ok(())
// }

use sqlx::{Database as SqlxDatabase, Error as SqlxError, MySqlPool, pool::{Pool, PoolOptions}};

pub enum DBConnType {
    Mysql,
}

#[derive(Clone)]
pub enum InferPool {
    Mysql(MySqlPool),
}

impl InferPool {
    pub async fn init_pool<T>(conn_type: DBConnType, conn_url: &str) -> Result<Pool<T>, SqlxError>
    where
        T: SqlxDatabase + 'static
    {
        match conn_type {
            DBConnType::Mysql => {
                PoolOptions::<T>::new()
                .max_connections(5)
                .connect(conn_url).await
            },
        }
    }
}

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    let pool = InferPool::init_pool(
        DBConnType::Mysql, "mysql://root:test1234@localhost:3306/mbb"
    ).await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT 148")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}
