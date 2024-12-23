use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div>
            <form>
                <label for="username">Username</label>
                <input type="text" id="username" name="username" />
                <label for="password">Password</label>
                <input type="password" id="password" name="password" />
                <button type="submit">Login</button>
            </form>
        </div>
    }
}