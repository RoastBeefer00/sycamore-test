use sycamore::prelude::*;
use std::panic;
use web_sys::KeyboardEvent;
use console_error_panic_hook;
use serde_json;

mod card;
use card::*;

mod nav;
use nav::Nav;
mod db;
use db::RECIPES;

mod recipes;
use recipes::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // Initialize AppState
    let app_state = AppState {
        recipes: create_rc_signal(Vec::new()),
        db: create_rc_signal(Vec::new()),
    };
    let app_state = provide_context(cx, app_state);
    let search = create_signal(cx, String::new());

    let handle_keyup = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            app_state.add_from_search(&search.get().as_ref().clone());
        }
    };

    // Add recipes to database
    let json = serde_json::from_str(RECIPES);
    let recipes_no_id: Vec<RecipeNoId> = match json {
        Ok(recipes) => {recipes},
        Err(err) => {panic!("Something went wrong: {}", err)}
    };
    app_state.db.set(recipes_no_id
           .into_iter()
           .map(|recipe| recipe.add_id())
           .collect());

    let recipes_empty = create_selector(cx, || app_state.recipes.get().is_empty());
    view! { cx,
        Nav {}
        div(class="flex mx-auto my-3 lg:w-2/3 w-4/5") {
            input(class="flex-initial mx-6 w-full shadow rounded appearance-none border-8 border-indigo-700 p-2", type="search", placeholder="Press 'Enter' to search...", bind:value=search, on:keyup=handle_keyup)
        }
        ((!*recipes_empty.get()).then(|| view! { cx,
                Show_Recipes {}
        }))
    }
}


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    sycamore::render(|cx| view! { cx, App {} });
}
