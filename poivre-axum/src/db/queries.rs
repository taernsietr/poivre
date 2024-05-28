use leptos::ServerFnError;
use crate::{
    db::setup::SURREALDB,
    resources::{items::{Item, ItemChanges}, users::{User, UserChanges}}
};

static GET_ITEM_BY_ID:           &str = "SELECT * FROM ONLY items:($id)";
static GET_ITEM_BY_NAME:         &str = "SELECT * FROM ONLY items:($name)";
static GET_ITEMS_BY_NAME:        &str = "SELECT * FROM items:($name)";
static GET_ITEMS_BY_CATEGORY:    &str = "SELECT * FROM items:($category)";
static GET_ITEMS_BY_DESCRIPTOR:  &str = "SELECT * FROM items:($descriptor)";
static GET_ALL_ITEMS:            &str = "SELECT * FROM items";
static ADD_ITEM:                 &str = "CREATE items:uuid() SET name = ($name), image = ($image), category = ($category), descriptors = ($descriptors)";
static UPDATE_ITEM:              &str = "";
static DELETE_ITEM:              &str = "";
static GET_USER:                 &str = "SELECT * FROM ONLY users:($id)";
static ADD_USER:                 &str = "CREATE users:uuid() SET name = ($name), profile_image = ($profile_image)";
static UPDATE_USER:              &str = "";
static DELETE_USER:              &str = "";

pub async fn get_item_by_id(id: String) -> Result<Item, ServerFnError> {
    let temp: Option<Item> = SURREALDB.query(GET_ITEM_BY_ID)
        .bind(("id",id))
        .await?
        .take(0)?;
    temp.ok_or_else(|| surrealdb::error::Db::NoRecordFound.into())
}

pub async fn get_item_by_name(name: String) -> Result<Option<Item>, ServerFnError> { todo!() }
pub async fn get_items_by_name(name: String) -> Result<Option<Vec<Item>>, ServerFnError> { todo!() }
pub async fn get_items_by_category(category: String) -> Result<Option<Vec<Item>>, ServerFnError> { todo!() }
pub async fn get_items_by_descriptor(descriptor: String) -> Result<Option<Vec<Item>>, ServerFnError> { todo!() }
pub async fn get_all_items() -> Result<Option<Vec<Item>>, ServerFnError> { todo!() }
pub async fn get_user(id: String) -> Result<Option<User>, ServerFnError> { todo!() }
pub async fn add_item(item: Item) -> Result<(), ServerFnError> { todo!() }
pub async fn add_user(user: User) -> Result<(), ServerFnError> { todo!() }
pub async fn update_item(id: String, data: ItemChanges) -> Result<(), ServerFnError> { todo!() }
pub async fn update_user(id: String, data: UserChanges) -> Result<(), ServerFnError> { todo!() }
pub async fn delete_item(id: String, soft: bool) -> Result<(), ServerFnError> { todo!() }
pub async fn delete_user(id: String, soft: bool) -> Result<(), ServerFnError> { todo!() }
