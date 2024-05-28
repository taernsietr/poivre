use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client,Ws};
use once_cell::sync::Lazy;

pub static SURREALDB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn initialize_database() { 
    _ = SURREALDB.connect::<Ws>("127.0.0.1:8000").await;
    match SURREALDB.use_ns("dev").use_db("dev").await {
        Ok(()) => println!("We're golden, boss"),
        _ => println!("We're shitfaced, boss")
    };
}

