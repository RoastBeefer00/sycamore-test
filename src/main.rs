use sycamore::prelude::*;
use uuid::Uuid;
use web_sys::{HtmlInputElement, KeyboardEvent};

mod card;
use card::*;

mod nav;
use nav::Nav;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let db: &Signal<Vec<Recipe>> = create_signal(cx, Vec::new()); 
    let recipes: &Signal<Vec<Recipe>> = create_signal(cx, Vec::new()); 
    let search = create_signal(cx, String::new());

    db.set(vec![Recipe {
        name: "Tomato Sandwich".into(),
        time: "2 minutes".into(),
        ingredients: vec!["1 tomato".into(), "2 slices of bread".into()],
        steps: vec!["Slice the tomato".into(), "Place the tomato between the slices of bread".into()],
        id: Uuid::new_v4(),
    },
    Recipe {
        name: "Tofu Sandwich".into(),
        time: "3 minutes".into(),
        ingredients: vec!["1 block of tofu".into(), "2 slices of bread".into()],
        steps: vec!["Slice the tofu".into(), "Place the tofu between the slices of bread".into()],
        id: Uuid::new_v4(),
    }]);

    // let search_recipes = move |_| recipes.set(get_recipes_from_search(search.get().as_ref().clone(), &db.get().as_ref().clone()));

    let handle_keyup = |event: KeyboardEvent| {
        if event.key() == "Enter" {
            recipes.set(get_recipes_from_search(search.get().as_ref().clone(), &db.get().as_ref().clone()));
        }
    };

    view! { cx,
        Nav {}
        div(class="flex mx-auto my-3 w-2/3") {
            input(class="flex-initial mx-6 w-full shadow rounded appearance-none border-8 border-indigo-700 p-2", type="search", placeholder="Press 'Enter' to search...", bind:value=search, on:keyup=handle_keyup)
            // button(class="flex-initial border bg-purple-300 mx-2 p-2 rounded", on:click=search_recipes) { "Search" }
        }

        Keyed(
            iterable=recipes,
            view=|cx, recipe| view! { cx,
                RecipeCard(name=recipe.name, time=recipe.time, ingredients=recipe.ingredients, steps=recipe.steps, id=recipe.id)
            },
            key=|recipe| recipe.id,
        )
    }
}


fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
