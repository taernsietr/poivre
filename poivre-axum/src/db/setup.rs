use leptos::logging;
use once_cell::sync::Lazy;
use surrealdb::{
    Surreal,
    opt::auth::Root,
    engine::remote::ws::{
        Client,
        Ws
    }
};

/// Initiate a SurrealDB Client statically, so it can be accessed globally. The actual connection
/// is made in connect_database.
pub static SURREALDB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
const DATE_FORMAT: &str = "%H:%M:%S";

/// Connect to the SurrealDB database. TODO: move ip elsewhere and set up dev/prod settings
pub async fn connect_database() { 
    match SURREALDB
        .connect::<Ws>("127.0.0.1:8000")
        .await {
            Ok(()) => logging::log!("[{}] [poivre-axum] Connected to SurrealDB.", chrono::Local::now().format(DATE_FORMAT)),
            Err(e) => logging::log!("[{}] [poivre-axum] Unable to connect to SurrealDB.\n{}", chrono::Local::now().format(DATE_FORMAT), e)
        };
    match SURREALDB
        .signin(Root { username: "root", password: "root" })
        .await {
            Ok(_) => logging::log!("[{}] [poivre-axum] Authenticated succesfully.", chrono::Local::now().format(DATE_FORMAT)),
            Err(e) => logging::log!("[{}] [poivre-axum] Unable to authenticate.\n{}", chrono::Local::now().format(DATE_FORMAT), e)
    };
    match SURREALDB
        .use_ns("dev")
        .use_db("dev")
        .await {
            Ok(()) => logging::log!("[{}] [poivre-axum] Using namespace dev, database dev.", chrono::Local::now().format(DATE_FORMAT)),
            Err(e) => logging::log!("[{}] [poivre-axum] Unable to access namespace or database.\n{}", chrono::Local::now().format(DATE_FORMAT), e)
    };
}

pub async fn setup_database() {
    let scaffold_tables = "
        BEGIN TRANSACTION;
            DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
                DEFINE FIELD email ON TABLE users TYPE string
                    ASSERT string::is::email($value);
                DEFINE FIELD username ON TABLE users TYPE string;
                DEFINE FIELD password ON TABLE users TYPE string;
                DEFINE FIELD image_url ON TABLE users TYPE option<string>;
                DEFINE FIELD first_name ON TABLE users TYPE string;
                DEFINE FIELD last_name ON TABLE users TYPE string;
                DEFINE FIELD birth_year ON TABLE users TYPE datetime;
                DEFINE FIELD friends on TABLE users TYPE option<set<record<users>>>;
                DEFINE FIELD created_on ON TABLE users TYPE datetime VALUE time::now() READONLY;
                DEFINE FIELD updated_on ON TABLE users TYPE datetime;
            DEFINE TABLE IF NOT EXISTS items SCHEMAFULL;
                DEFINE FIELD name ON TABLE items TYPE set<string>;
                DEFINE FIELD category ON TABLE items TYPE string;
                DEFINE FIELD image_url ON TABLE items TYPE string;
                DEFINE FIELD description ON TABLE items TYPE string;
                DEFINE FIELD descriptors ON TABLE items TYPE set<string>;
                DEFINE FIELD associated_cuisines ON TABLE items TYPE option<set<string>>;
                DEFINE FIELD created_on ON TABLE users TYPE datetime VALUE time::now() READONLY;
                DEFINE FIELD updated_on ON TABLE users TYPE datetime;
            DEFINE TABLE IF NOT EXISTS cuisines SCHEMAFULL;
                DEFINE FIELD name ON TABLE cuisines TYPE string;
                DEFINE FIELD image_url ON TABLE cuisines TYPE option<string>;
                DEFINE FIELD description ON TABLE cuisines TYPE string;
                DEFINE FIELD created_on ON TABLE users TYPE datetime VALUE time::now() READONLY;
                DEFINE FIELD updated_on ON TABLE users TYPE datetime;
            DEFINE TABLE IF NOT EXISTS preferences SCHEMALESS;
                DEFINE FIELD preference ON preferences TYPE string ASSERT $value IN ['love', 'like', 'ambivalent', 'dislike', 'allergic', 'prohibited'];
                DEFINE FIELD condition ON preferences TYPE option<array<object>>; 
                DEFINE FIELD created_on ON TABLE users TYPE datetime VALUE time::now() READONLY;
                DEFINE FIELD updated_on ON TABLE users TYPE datetime;
            DEFINE TABLE IF NOT EXISTS is_ingredient_of SCHEMALESS;
        COMMIT TRANSACTION;
        ";

    match SURREALDB
        .query(scaffold_tables)
        .await {
            Ok(_) => logging::log!("[{}] [poivre-axum] Tables created successfully.", chrono::Local::now().format(DATE_FORMAT)),
            Err(e) => logging::log!("[{}] [poivre-axum] Unable to create tables.\n{}", chrono::Local::now().format(DATE_FORMAT), e)
        };
}
