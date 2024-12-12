use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <h1>
                "Mobware"
            </h1>
            <nav>
                <a href="/">
                    "Home"
                </a>
                <a href="/about">
                    "About"
                </a>
                <a href="/contact">
                    "Contact"
                </a>
            </nav>
        </header>
    }
}