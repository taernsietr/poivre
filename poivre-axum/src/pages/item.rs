use leptos::*;
use leptos_router::*;
use leptos_meta::Title;
use std::fmt::Debug;
use wasm_bindgen::UnwrapThrowExt;
use crate::db::item::{GetItem,AddItem};
use crate::resources::shared::Displayable;

#[component]
pub fn ItemLanding() -> impl IntoView {
    view! { <p>"LANDING"</p> }
}

/// Form for adding a new item (Admin only)
#[component]
pub fn AddItem() -> impl IntoView {
  let add_item = create_server_action::<AddItem>();

  view! {
    <Title text="Poivre - Add Item"/>
    <ActionForm action=add_item>
      <label class="m-4 p-4 flex-auto">
        "Image: "
        <input
          type="text"
          name="item-image"
        />
      </label>
      <label class="m-4 p-4 flex-auto">
        "Item Name: "
        <input
          type="text"
          name="item-name"
        />
      </label>
      <label class="m-4 p-4 flex-auto">
        "Category: "
        <select name="item-category" >
          <option value="ingredient">Ingredient</option>
          <option value="dish">Dish</option>
          <option value="beverage">Beverage</option>
        </select>
      </label>
      <label class="m-4 p-4 flex-auto">
        "Description: "
        <input
          type="text"
          name="item-description"
        />
      </label>
      <label class="m-4 p-4 flex-auto">
        "Descriptors: "
        <input
          type="text"
          name="item-descriptors"
          multiple
        />
      </label>
      <label class="m-4 p-4 flex-auto">
        "Associated Cuisines: "
        <input
          type="text"
          name="item-cuisines"
          multiple
        />
      </label>
      <input type="submit" value="Add item" />
    </ActionForm>
  }
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
        <p>{ item.display_name() }</p>
        <p>{ item.category() }</p>
        <p>{ item.description() }</p>
        <p>{ item.descriptors() }</p>
      </div>
    </Suspense>
  }
}

