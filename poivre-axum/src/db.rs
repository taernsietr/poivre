use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client,Ws};
use once_cell::sync::Lazy;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn initialize_database() { 
    _ = DB.connect::<Ws>("127.0.0.1:8000").await;
    match DB.use_ns("dev").use_db("dev").await {
        Ok(()) => println!("We're golden, boss"),
        _ => println!("We're shitfaced, boss")
    };
}

pub mod queries {
    pub static GET_ITEM_BY_ID: &str = "SELECT * FROM ONLY items:($id)";
}
