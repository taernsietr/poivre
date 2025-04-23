use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Poivre - Home"/>
        <p>"Hi there!"</p>
    }
}

