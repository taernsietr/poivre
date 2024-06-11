use std::future::Future;
use serde::{Serialize,Deserialize};
use leptos::*;
use leptos_meta::Title;
use crate::{
    resources::{
        users::User,
        items::Item
    },
    shared::TableRow
};

/// Server endpoint for returning all database users (debugging purposes only).
#[server(GetAllUsers)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    Result::Ok(User::mock_user_list())
}

/// Server endpoint for returning all database items (debugging purposes only).
#[server(GetAllItems)]
pub async fn get_all_items() -> Result<Vec<Item>, ServerFnError> {
    Result::Ok(Item::mock_item_list())
}

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <Title text="Poivre - Admin"/>
        <UserTable />
        <ItemTable />
    }
}

/// A table row for a list of entries queried from the database.
#[component]
pub fn TableRow(row: impl TableRow) -> impl IntoView {
    view! {
        <tr class="bg-orange-200 border-4 border-solid border-black-200 p-4">
            {
                row.into_row().map(|field| view! {
                    <td class="align-left">{ field }</td>
                }).collect_view()
            }
        </tr>
    }
}

#[component]
pub fn TestTable<T>(source: impl Fn() -> T + 'static) -> impl IntoView
where
    T: Future + 'static,
    T::Output: Serializable + Clone + 'static
{
    let (table, _set_table) = create_signal(());
    let resource = Resource::new(move || table, |_| source());

    view! {
        <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
            <table class="border-4 border-solid border-black-200 m-4">
                <caption>Users</caption>
                <thead>
                    <tr class="align-left bg-orange-200 border-4 border-solid border-black-200 p-4">
                        <th>Image</th>
                        <th>ID</th>
                        <th>Username</th>
                        <th>First Name</th>
                        <th>Last Name</th>
                        <th>Date of Birth</th>
                    </tr>
                </thead>
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


/// Table listing users
#[component]
pub fn UserTable() -> impl IntoView {
    let (table, _set_table) = create_signal(());
    let resource = Resource::new(move || table, |_| get_all_users());

    view! {
        <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
            <table class="border-4 border-solid border-black-200 m-4">
                <caption>Users</caption>
                <thead>
                    <tr class="align-left bg-orange-200 border-4 border-solid border-black-200 p-4">
                        <th>Image</th>
                        <th>ID</th>
                        <th>Username</th>
                        <th>First Name</th>
                        <th>Last Name</th>
                        <th>Date of Birth</th>
                    </tr>
                </thead>
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

/// The table for the list of items queried from the database.
#[component]
pub fn ItemTable() -> impl IntoView {
    let (table, _set_table) = create_signal(());
    let resource = Resource::new(move || table, |_| get_all_items());

    view! {
        <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
            <table class="border-4 border-solid border-black-200 m-4">
                <caption>Items</caption>
                <thead>
                    <tr class="bg-orange-200 border-4 border-solid border-black-200 p-4">
                        <th class="align-left">Image</th>
                        <th class="align-left">ID</th>
                        <th class="align-left">Name</th>
                        <th class="align-left">Category</th>
                        <th class="align-left">Descriptors</th>
                    </tr>
                </thead>
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

