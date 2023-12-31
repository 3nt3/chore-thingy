use std::{cell::RefCell, rc::Rc, time::Duration};

use leptos::{leptos_dom::console_log, *};
use leptos_meta::*;
use leptos_router::*;

use wbg_rand::{wasm_rng, Rng};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (chore, set_chore) = create_signal::<Option<String>>(cx, None);
    let chores = vec![
        "main floor",
        "floor under desk",
        "desk",
        "bed area",
        "nightstand",
        "stuff corner",
        "couch",
        "bring out garbage",
    ];

    let (loading, set_loading) = create_signal::<bool>(cx, false);

    let chores_ref = Rc::new(RefCell::new(chores));

    let on_click = move |_| {
        set_loading.set(true);
        let cloned_chores = Rc::clone(&chores_ref);
        let random_number = wasm_rng().gen_range(0, cloned_chores.borrow().len());
        let binding = cloned_chores.borrow();
        let random_chore = binding.get(random_number);
        set_chore.set(random_chore.map(|s| s.to_string()));

        set_timeout(move || set_loading.set(false), Duration::from_millis(200))
    };

    view! { cx,
        <div class="h-screen w-full flex flex-col justify-center items-center pointer text-slate-50 bg-black" on:click=on_click>
            {move || if loading.get() {
                view! { cx, <p>"Loading..."</p> }
            } else {
                match chore.get() {
                    Some(chore) => view! { cx, <p class="text-9xl font-bold">{chore}</p> },
                    None => view! { cx, <p>"Click anywhere to get a chore!"</p> },
                }
            }}
        </div>
    }
}
