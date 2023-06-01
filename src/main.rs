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

mod sidebar;
use sidebar::*;

mod groceries;
use groceries::GroceriesModal;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // Initialize AppState
    let app_state = AppState {
        recipes: create_rc_signal(Vec::new()),
        db: create_rc_signal(Vec::new()),
        filter: create_rc_signal(Filter::Title),
        maxTime: create_rc_signal(String::from("55")),
        modal: create_rc_signal(false),
    };
    let app_state = provide_context(cx, app_state);
    let search = create_signal(cx, String::new());

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

    let handle_keyup = |event: KeyboardEvent| {
        if event.key() == "Enter" {
            app_state.add_from_search(search.get().as_ref().clone());
            search.set("".to_string());
        }
    };
    view! { cx,
        // Nav {}
        Sidebar {}
        div(class="p-4 sm:ml-64 mt-16 sm:mt-28") {
            div(class="flex mx-auto my-3 lg:w-2/3 w-4/5") {
                input(class="flex-initial w-full shadow rounded appearance-none border-8 border-indigo-700 p-2", type="search", placeholder="Press 'Enter' to search by...", bind:value=search, on:keyup=handle_keyup)
            }
            Show_Recipes {}
            (if app_state.modal.get().as_ref().clone() {
                view! { cx,
                    GroceriesModal {}
                }
            } else {
                view! { cx, 
                }
            })
        }
    }
}


fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    sycamore::render(|cx| view! { cx, App {} });
}
