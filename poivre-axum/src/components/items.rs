use std::fmt::Debug;
use leptos::*;
use leptos_router::*;
use wasm_bindgen::UnwrapThrowExt;
use crate::db::setup::SURREALDB;
use crate::resources::items::Item;

#[derive(Params, PartialEq)]
struct ItemParams {
    pub id: Option<String>
}

/// A table row for the list of items queried from the database.
#[component]
pub fn TableRow(row: Item) -> impl IntoView {
    view! {
        <tr>
            <td>{ row.image()       }</td>
            <td>{ row.id()          }</td>
            <td>{ row.name()        }</td>
            <td>{ row.category()    }</td>
            <td>{ row.descriptors() }</td>
        </tr>
    }
}

/// The table for the list of items queried from the database.
#[component]
pub fn ItemTable() -> impl IntoView {
    let (table, _set_table) = create_signal(());
    let resource = Resource::new(move || table, |_| get_all_items());

    view! {
        <Suspense fallback=move || view! { "<p>No data loaded!</p>" }>
            <table>
                <tr>
                    <th>Image</th>
                    <th>ID</th>
                    <th>Name</th>
                    <th>Category</th>
                    <th>Descriptors</th>
                </tr>
                {
                    move || resource.get()
                        .map(|data| {
                            data.unwrap()
                                .iter()
                                .map(|row| view! { <TableRow row=row.clone() /> }).collect_view()
                        })
                }
            </table>
        </Suspense>
    }
}

/// Server endpoint for returning all database items (debugging purposes only).
#[server(GetAllItems)]
pub async fn get_all_items() -> Result<Vec<Item>, ServerFnError> {
    Result::Ok(Item::mock_item_list())
}

#[server(GetItem, "/internal")]
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

/// Item description card, showing all possible information for a selected item.
#[component]
pub fn ItemDescription() -> impl IntoView {
    let action = create_server_action::<GetItem>();
    let item = action
        .value()    // Return RwSignal
        .get()      // Return Option<Result<T,E>> from signal
        .unwrap()   // Return Result<T,E> from Option
        .unwrap();

    view! {
        <Suspense fallback=move || view! { <ItemDescriptionFallback /> }>
            <div>
                <p>{ item.image() }</p>
                <p>{ item.id() }</p>
                <p>{ item.name() }</p>
                <p>{ item.category() }</p>
                <p>{ item.description() }</p>
                <p>{ item.descriptors() }</p>
            </div>
        </Suspense>
    }
}

#[component]
pub fn ItemDescriptionFallback() -> impl IntoView {
    view! {
        <p>"Loading item info..."</p>
    }
}
