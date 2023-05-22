use sycamore::prelude::*;

mod card;
use card::*;

mod nav;
use nav::Nav;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let mut db: Vec<Recipe> = Vec::new();
    let recipes: &Signal<Vec<Recipe>> = create_signal(cx, Vec::new()); 
    let search = create_signal(cx, String::new());

    // let search_recipes = |_| recipes.set(get_recipes_from_search(search.get(), db));

    let recipe = Recipe {
        name: "Tomato Sandwich".into(),
        time: "2 minutes".into(),
        ingredients: vec!["1 tomato".into(), "2 slices of bread".into()],
        steps: vec!["Slice the tomato".into(), "Place the tomato between the slices of bread".into()],
    };

    db.push(Recipe {
        name: "Tomato Sandwich".into(),
        time: "2 minutes".into(),
        ingredients: vec!["1 tomato".into(), "2 slices of bread".into()],
        steps: vec!["Slice the tomato".into(), "Place the tomato between the slices of bread".into()],
    });

    view! { cx,
        Nav {}
        div(class="flex mx-auto my-3 w-2/3") {
            input(class="mx-6 w-full shadow appearance-none border p-2", type="search", placeholder="Search...", bind:value=search)
            button(class="border bg-purple-300 m-2 p-2 rounded") { "Search" }
        }

        RecipeCard(name=recipe.name, time=recipe.time, ingredients=recipe.ingredients, steps=recipe.steps)
        Indexed(
            iterable=recipes,
            view=|cx, recipe| view! { cx,
                RecipeCard(name=recipe.name, time=recipe.time, ingredients=recipe.ingredients, steps=recipe.steps)
            },
        )
    }
}


fn main() {
    sycamore::render(|cx| view! { cx, App {} });
}
