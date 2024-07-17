use std::future::Future;
use std::fmt::Debug;
use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::{
  db::{
    setup::SURREALDB,
    user::get_all_users,
    item::get_all_items
  },
  resources::{
     item::{Item,ItemBuilder},
     user::User,
     shared::PoivreTableRow
  }
};

#[component]
pub fn Dashboard() -> impl IntoView {
  view! {
    <Title text="Poivre - Admin"/>
    <A href="/items/add">"Add Item"</A>
    <PoivreTable fetcher=get_all_items caption="Items" />
    <PoivreTable fetcher=get_all_users caption="Users"/>
  }
}

/// A table row for a list of entries queried from the database.
#[component]
pub fn PoivreTableRow(row: impl PoivreTableRow + 'static) -> impl IntoView {
  view! {
  <tr class="bg-orange-200 border-4 border-solid border-black-200 p-4 hover:bg-orange-400">
    {
      row
        .row_values()
        .map(|field| view! { <td class="align-left">{ field }</td> })
        .collect_view()
    }
  </tr>
  }
}

#[component]
pub fn PoivreTable<T,U,E>(fetcher: impl Fn() -> T + 'static, caption: &'static str) -> impl IntoView
where
  E: Debug + 'static,
  U: Clone + Debug + PoivreTableRow + 'static,
  T: Future<Output = Result<Vec<U>,E>> + 'static,
  Result<Vec<U>,E>: Serializable + Clone
{
  let (table, _set_table) = create_signal(());
  let resource = create_resource(move || table.get(), move |_| fetcher());

  view! {
  <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
    <table class="border-4 border-solid border-black-200 m-4">
    <caption>{caption}</caption>
    <thead>
      <tr class="align-left bg-orange-200 border-4 border-solid border-black-200 p-4">
      {
        U::headers()
          .map(|header| view! { <th>{header}</th> })
          .collect_view()
      }
      </tr>
    </thead>
    {
      move || resource.get()
        .map(|data| {
          data.unwrap()
          .into_iter()
          .map(|row| view! { <PoivreTableRow row=row.clone() /> })
          .collect_view()
      })
    }
    </table>
  </Suspense>
  }
}

