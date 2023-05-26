use crate::blog::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
                <Routes>
                <Route path="" view=|cx| view! { cx, <Home/> }/>
                <Route path="/counter" view=|cx| view! { cx, <Counter/> }/>
                <Blog prefix="/blog" />
                </Routes>
        </Router>
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <ul>
            <li><A href="/counter">"Counter"</A></li>
            <li><A href="/blog">"Blog"</A></li>
        </ul>
    }
}

#[component]
fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    view! { cx,
        <h1>"Counter"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
