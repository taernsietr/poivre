use std::future::Future;
use std::fmt::Debug;
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
        <AdminTable fetcher=get_all_items />
        <AdminTable fetcher=get_all_users />
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
pub fn AdminTable<T,U,E>(fetcher: impl Fn() -> T + 'static) -> impl IntoView
where
    E: Debug + 'static,
    U: Clone + TableRow + 'static,
    T: Future<Output = Result<Vec<U>,E>> + 'static,
    Result<Vec<U>,E>: Serializable + Clone
{
    let (table, _set_table) = create_signal(());
    let resource = create_resource(move || table.get(), move |_| fetcher());

    view! {
        <Suspense fallback=move || view! { <p>"No data loaded!"</p> }>
            <table class="border-4 border-solid border-black-200 m-4">
                <caption>Users</caption>
                <thead>
                    <tr class="align-left bg-orange-200 border-4 border-solid border-black-200 p-4">
                        { U::headers().map(|header| view! { <th>{header}</th> }).collect_view() }
                    </tr>
                </thead>
                {
                    move || resource.get()
                        .map(|data| {
                            data.unwrap()
                                .into_iter()
                                .map(|row| view! { <TableRow row=row.clone() /> })
                                .collect_view()
                        })
                }
            </table>
        </Suspense>
    }
}

