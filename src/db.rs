use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;


pub async fn init_db() -> Result<Surreal<Db>, surrealdb::Error> {
    let db = Surreal::new::<RocksDb>("my_secure_db").await?;

    db.use_ns("test").await?;
    db.use_db("test").await?;
    
    Ok(db)
}