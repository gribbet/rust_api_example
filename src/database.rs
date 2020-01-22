use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

type Connection = PooledConnection<ConnectionManager<PgConnection>>;

impl Database {
    pub fn connect(database_url: &'_ str) -> Result<Database, PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Ok(Database {
            pool: Pool::builder().build(manager)?,
        })
    }

    pub fn get(&self) -> Result<Connection, PoolError> {
        self.pool.get()
    }
}
