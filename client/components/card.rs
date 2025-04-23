use std::future::Future;
use std::fmt::Debug;
use leptos::*;
use leptos_router::A;
use crate::resources::shared::Displayable;

/// Container for generic cards
#[component]
pub fn CardList(cards: Vec<impl Displayable + 'static>) -> impl IntoView {
  view! {
    <div>
      {
        cards
          .iter()
          .map(|card| view! { <Card object=card.clone() /> })
          .collect_view()
      }
    </div>
  }
}

/// Generic card element for showing entries
#[component]
pub fn Card(object: impl Displayable + 'static) -> impl IntoView {
  view! {
    <div>
      <A href={ object.url() }>
        <img
          class="" 
          src={ object.image() }
          alt={ object.alt_text() }
        />
        <p>{ object.display_name() }</p>
      </A>
    </div>
  }
}

