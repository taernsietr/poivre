use std::future::Future;
use std::fmt::Debug;
use leptos::*;
use leptos_meta::Title;
use leptos_router::A;
use crate::{
    db::setup::SURREALDB, resources::{
        items::{Item,ItemBuilder}, users::User
    }, shared::PoivreTableRow
};

/// Server endpoint for returning all database users (debugging purposes only).
#[server(GetAllUsers)]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    //Result::Ok(User::mock_user_list())
    match SURREALDB
        .select::<Vec<User>>("users")
        .await {
            Ok(users) => { dbg!(&users); Ok(users) },
            Err(e) => Err(ServerFnError::from(e))
        }
}

/// Server endpoint for returning all database items (debugging purposes only).
#[server(GetAllItems)]
pub async fn get_all_items() -> Result<Vec<Item>, ServerFnError> {
    Result::Ok(Item::mock_item_list())
}

/// Server endpoint for adding a new database item.
#[server(AddItem)]
pub async fn add_item() -> Result<(), ServerFnError> {
    todo!()
}

/// Server endpoint for updating (editing) a database item.
#[server(UpdateItem)]
pub async fn update_item() -> Result<(), ServerFnError> {
    todo!()
}

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <Title text="Poivre - Admin"/>
        <A href="/items/add">"Add Item"</A>
        <AdminTable fetcher=get_all_items />
        <AdminTable fetcher=get_all_users />
    }
}

/// A table row for a list of entries queried from the database.
#[component]
pub fn AdminTableRow(row: impl PoivreTableRow + 'static) -> impl IntoView {
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
pub fn AdminTable<T,U,E>(fetcher: impl Fn() -> T + 'static) -> impl IntoView
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
                <caption>Users</caption>
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
                                .map(|row| view! { <AdminTableRow row=row.clone() /> })
                                .collect_view()
                        })
                }
            </table>
        </Suspense>
    }
}

