use std::future::Future;
use std::fmt::Debug;
use leptos::*;
use leptos_router::A;
use crate::resources::shared::Displayable;

/// A table row for a list of entries queried from the database.
#[component]
pub fn PoivreTableRow(row: impl Displayable + 'static) -> impl IntoView {
  view! {
    <tr class="table-row">
      {
        row
          .row_values()
          .map(|field| view! { <td>{ field }</td> })
          .collect_view()
      }
    </tr>
  }
}

#[component]
pub fn PoivreTable<T,U,E>(fetcher: impl Fn() -> T + 'static, caption: &'static str) -> impl IntoView
where
  E: Debug + 'static,
  U: Clone + Debug + Displayable + 'static,
  T: Future<Output = Result<Vec<U>,E>> + 'static,
  Result<Vec<U>,E>: Serializable + Clone
{
  let (table, _set_table) = create_signal(());
  let resource = create_resource(move || table.get(), move |_| fetcher());

  view! {
  <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
    <table class="table">
      <caption>{ caption }</caption>
      <thead>
        <tr class="table-row">
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

