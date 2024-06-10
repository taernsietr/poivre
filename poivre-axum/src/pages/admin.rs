use leptos::*;
use leptos_meta::Title;
use crate::components::{
    users::UserTable,
    items::ItemTable
};

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <Title text="Poivre - Admin"/>
        <UserTable />
        <ItemTable />
    }
}

