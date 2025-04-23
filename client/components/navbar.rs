use std::future::Future;
use std::fmt::Debug;
use leptos::*;
use leptos_router::A;
use crate::resources::shared::Displayable;

/// Navbar item, that is, a button redirecting to another page.
#[component]
fn NavbarItem(href: &'static str, inner_html: &'static str) -> impl IntoView {
  view! {
  <A href={ href }>
    <div class="navbar-item">
      { inner_html }
    </div>
  </A>
  }
}

/// Main navbar. Should appear in most if not all pages.
/// TODO: If we are logged in, we should show profile links instead of the login link
#[component]
pub fn Navbar(entries: Vec<(&'static str, &'static str)>) -> impl IntoView {
  view! {
    <nav>
      <div class="navbar-links-container">
        <img
          class="navbar-img" 
          src="logo-64.jpg"
          alt="Poivre logo: a cartoonish pepper mill and salt shaker"
        />
        {
          entries
            .into_iter()
            .map(|entry| view! {
              <NavbarItem
                href={ entry.1 }
                inner_html={ entry.0 }
              />
            })
            .collect_view()
        }
      </div>
    </nav>
  }
}

