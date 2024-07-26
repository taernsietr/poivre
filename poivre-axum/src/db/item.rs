use leptos::*;
use leptos_router::*;
use std::fmt::Debug;
use wasm_bindgen::UnwrapThrowExt;
use surrealdb::sql::Thing;
use crate::{
  db::setup::SURREALDB,
  resources::{
    category::Category,
    item::Item,
    shared::{Image,ItemName}
  }
};

/// Parameters for querying an item by its SurrealDB ID
#[derive(Params, PartialEq)]
struct ItemQueryById {
  pub id: Option<String>
}

/// Server endpoint for returning all database items (debugging purposes only).
#[server(GetAllItems)]
pub async fn get_all_items() -> Result<Vec<Item>, ServerFnError> {
  match SURREALDB
    .select::<Vec<Item>>("items")
    .await { 
      Ok(items) => {
        dbg!(&items);
        Ok(items)
      },
      Err(e) => Err(ServerFnError::from(e))
  }
}

/// Server endpoint for adding a new database item.
#[server(AddItem)]
pub async fn add_item(
  name: String,
  image: Image,
  category: Category,
  description: String,
  descriptors: Vec<String>,
  associated_cuisines: Vec<String>
) -> Result<(), ServerFnError> {

  let form_data = Item::new(
    name,
    image,
    category,
    description,
    descriptors,
    associated_cuisines
  )?;

  let response: Result<Vec<Item>,surrealdb::Error> = SURREALDB
    .create("items")
    .content(form_data)
    .await;
  
  Ok(())
}

/// Server endpoint for updating (editing) a database item.
#[server(UpdateItem)]
pub async fn update_item(
  id: Thing,
  name: Option<String>,
  image: Option<Image>,
  category: Option<Category>,
  description: Option<String>,
  descriptors: Option<Vec<String>>,
  associated_cuisines: Option<Vec<String>>
) -> Result<(), ServerFnError> {
  let item_id = get_item().await?;

  Ok(())
}

/// Server endpoint for getting a single item. Can fail if an invalid id or no ID is provided, or
/// if the id finds no existing item.
#[server(GetItem)]
pub async fn get_item() -> Result<Item, ServerFnError> {
  let params = use_params::<ItemQueryById>();

  let id = move || {
    params.with(|params| {
      params.as_ref()
        .map(|params| params.id.clone())
        .unwrap_or_default()
      }
    )
  };

  match id() {
    Some(item) if !item.is_empty() => {
      SURREALDB
        .select(("items", item))
        .await?
        .ok_or(surrealdb::error::Db::NoRecordFound.into())
    },
    _ => Err(ServerFnError::new("Invalid item ID"))
  }
}

