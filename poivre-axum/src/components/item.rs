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
        <Suspense fallback=move || view! { <p>"Loading item info..."</p> }>
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

