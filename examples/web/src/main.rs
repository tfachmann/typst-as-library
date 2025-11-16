use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>Hello, World!</div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
