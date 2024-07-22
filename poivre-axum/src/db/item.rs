use leptos::*;
use leptos_router::*;
use std::fmt::Debug;
use wasm_bindgen::UnwrapThrowExt;
use crate::{
    db::setup::SURREALDB,
    resources::{
        category::Category,
        item::Item
    }
};

#[derive(Params, PartialEq)]
struct ItemParams {
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
pub async fn add_item() -> Result<(), ServerFnError> {
  todo!()
}

/// Server endpoint for updating (editing) a database item.
#[server(UpdateItem)]
pub async fn update_item() -> Result<(), ServerFnError> {
  todo!()
}

#[server(GetItem)]
pub async fn get_item() -> Result<Item, ServerFnError> {
    let params = use_params::<ItemParams>();
    let id = move || {
        params.with(
            |params| {
                params.as_ref()
                    .map(|params| params.id.clone())
                    .unwrap_or_default()
            }
        )
    };

    // handle this error properly
    let id = id().unwrap_throw();

    SURREALDB
        .select(("items", id))
        .await?
        .ok_or(surrealdb::error::Db::NoRecordFound.into())
}

//pub fn mock_item_list() -> Vec<Item> {
//    vec!(
//        Item {
//            id: "0000000000".to_string(),
//            name: ItemName::SingleName("Apple".to_string()),
//            image: Image::NoImage,
//            category: Category::Ingredient,
//            description: "A sweet fruit, usually red.".to_string(),
//            descriptors: vec!("fruit".to_string(), "sweet".to_string(), "aromatic".to_string()),
//            associated_cuisines: vec!("".to_string())
//        },
//        Item {
//            id: "0000000001".to_string(),
//            name: ItemName::SingleName("Banana".to_string()),
//            image: Image::HasImage("some/url".to_string()),
//            category: Category::Ingredient,
//            description: "A sweet fruit, yellow, with a thick peel.".to_string(),
//            descriptors: vec!("fruit".to_string(), "sweet".to_string(), "aromatic".to_string()),
//            associated_cuisines: vec!("".to_string())
//        },
//        Item {
//            id: "0000000002".to_string(),
//            name: ItemName::SingleName("Pizza".into()),
//            image: Image::HasImage("some/url".to_string()),
//            category: Category::Dish,
//            description: "One of the most popular dishes worldwide with roots in Italy, pizzas are flat disks of dough (though ocasionally found in other shapes), usually covered with tomato sauce and melted cheese, and may have any number of toppings added to it.".to_string(),
//            descriptors: vec!("bread".to_string(), "gluten".to_string(), "savory".to_string(), "baked".to_string()),
//            associated_cuisines: vec!("italian".to_string(), "fast-food".to_string())
//        }
//    )
//}
