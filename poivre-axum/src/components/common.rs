use leptos::*;
use leptos_router::A;
use crate::resources::shared::Displayable;

/// Navbar item, that is, a button redirecting to another page.
#[component]
fn NavbarItem(href: &'static str, inner_html: &'static str) -> impl IntoView {
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
pub fn Navbar(entries: Vec<(&'static str, &'static str)>) -> impl IntoView {
  view! {
    <nav class="float w-full border-4 border-solid justify-around bg-orange-500">
      <div class="flex flex-row">
        <img
          class="flex-initial p-2 m-2" 
          src="logo-64.jpg"
          alt="Poivre logo: a cartoonish pepper mill and salt shaker"
        />
        {
          entries
            .into_iter()
            .map(|entry| view! {
                <NavbarItem
                  href={entry.1}
                  inner_html={entry.0}
                />
            })
            .collect_view()
        }
      </div>
    </nav>
  }
}

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
      <A href={object.url()}>
        <img
          class="" 
          src={object.image()}
          alt={object.alt_text()}
        />
        <p>{object.display_name()}</p>
      </A>
    </div>
  }
}

// Generic button element
//#[component]
//pub fn PoivreButton
