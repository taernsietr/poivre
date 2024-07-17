use leptos::*;
use leptos_router::*;
use std::fmt::Debug;
use wasm_bindgen::UnwrapThrowExt;
use crate::db::item::GetItem;

#[component]
pub fn ItemLanding() -> impl IntoView {
    view! { <p>"LANDING"</p> }
}

#[component]
pub fn AddItem() -> impl IntoView { 
    view! { <p>"ADD ITEM"</p> }
}

#[component]
pub fn EditItem() -> impl IntoView {
    view! { <p>"EDIT ITEM"</p> }
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

