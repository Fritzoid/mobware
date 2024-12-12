use leptos::prelude::*;
use leptos::mount::mount_to_body;

mod header;
use header::Header;

#[component]
fn App() -> impl IntoView {

    view! {
        <div>
            <Header />
            <main>
                <h2>
                    "Welcome to Mobware"
                </h2>
                <p>
                    "Mobware is a software development company that specializes in mobile applications."
                </p>
            </main>
        </div>
    }
}



fn main() {
    mount_to_body(App);
}
