use leptos::*;
use leptos_meta::Title;
use crate::{
  db::{
    user::get_all_users,
    item::get_all_items
  },
  components::table::PoivreTable
};

#[component]
pub fn AdminDashboard() -> impl IntoView {
  view! {
    <Title text="Poivre - Administrator Dashboard"/>
    <PoivreTable fetcher=get_all_items caption="Items" />
    <PoivreTable fetcher=get_all_users caption="Users"/>
  }
}

