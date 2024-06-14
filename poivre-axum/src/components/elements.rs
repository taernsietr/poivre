use leptos::*;
use leptos_router::A;
use crate::shared::PoivreCard;

/// Navbar item, that is, a button redirecting to another page.
#[component]
fn NavbarItem(href: String, inner_html: String) -> impl IntoView {
    view! {
        <A href={href}>
            <div class="flex-initial border-4 border-solid border-black-200 p-2 mx-2 my-6 bg-orange-500 hover:bg-orange-100 hover:fg-orange-950 transition duration-300 ease-in-out">
                {inner_html}
            </div>
        </A>
    }
}

/// Main navbar. Should appear in most if not all pages.
/// TODO: If we are logged in, we should show profile links instead of the login link
#[component]
pub fn Navbar() -> impl IntoView {
    let navbar_links: [(&str,&str); 4] = [
        ("/", "Home"),
        ("/login", "Login"),
        ("/signup", "Sign Up"),
        ("/admin", "Admin")
    ];

    view! {
        <nav class="float w-full border-4 border-solid justify-around bg-orange-500">
            <div class="flex flex-row">
                <img
                    class="flex-initial p-2 m-2" 
                    src="logo-64.jpg"
                    alt="Poivre logo: a cartoonish pepper mill and salt shaker"
                />
                {
                    navbar_links
                        .into_iter()
                        .map(|link| view! { <NavbarItem href={link.0.to_string()} inner_html={link.1.to_string()} /> })
                        .collect_view()
                }
            </div>
        </nav>
    }
}

/// Container for generic cards
#[component]
pub fn CardList(cards: Vec<impl PoivreCard + 'static>) -> impl IntoView {
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
pub fn Card(object: impl PoivreCard + 'static) -> impl IntoView {
    view! {
        <div>
            <A href={object.url()}>
                <img
                    class="" 
                    src={object.img()}
                    alt={object.alt_text()}
                />
                <p>{object.card_name()}</p>
            </A>
        </div>
    }
}

