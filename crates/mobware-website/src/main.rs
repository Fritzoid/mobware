use leptos::prelude::*;
use leptos::mount::mount_to_body;

mod header;
use header::Header;

#[component]
fn App() -> impl IntoView {

    view! {
        <div>
            <Header />
        </div>
    }
}



fn main() {
    mount_to_body(App);
}
