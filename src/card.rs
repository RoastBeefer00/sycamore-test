use sycamore::prelude::*;
use uuid::Uuid;
use crate::recipes::AppState;
use serde::{Serialize, Deserialize};

#[derive(Props, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Recipe {
    pub name: String,
    pub time: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecipeNoId {
    pub name: String,
    pub time: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
}

impl RecipeNoId {
    pub fn add_id(&self) -> Recipe {
        Recipe { name: self.name.clone(), time: self.time.clone(), ingredients: self.ingredients.clone(), steps: self.steps.clone(), id: Uuid::new_v4() }
    }
}

#[component]
pub fn RecipeCard<G: Html> (cx: Scope, recipe: Recipe) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let ingredients = create_signal(cx, recipe.ingredients);
    let steps = create_signal(cx, recipe.steps);

    create_effect(cx, || println!("Something changed {:?}", app_state.recipes.get()));
    let remove_recipe = move |_| app_state.remove_recipe(recipe.id);

    view! {cx,
        div(class="rounded my-3 mx-auto lg:w-2/3 w-11/12 border-2 border-indigo-700 shadow") {
            // name
            div(class="rounded-t w-full p-3 bg-indigo-900 border-b-2 border-b-indigo-700") {
                p(class="text-2xl text-white") {
                    (recipe.name)
                }
                // time
                p(class="text-white") {
                    (recipe.time)
                }
                button(class="rounded bg-red-700 text-white", on:click=remove_recipe) {
                    "Remove"
                }
            }
            // ingredients
            div(class="border-b border-l border-r p-5 bg-gray-300") {
                "Ingredients:"
                ul(class="list-disc m-2") {
                   Indexed(
                        iterable=ingredients,
                        view=|cx, ingredient| view! { cx,
                        li { (ingredient) }    
                        },
                    ) 
                }
            }
            // steps
            div(class="rounded-b border-b border-l border-r p-5 bg-gray-300") {
                "Steps:"
                ul(class="list-decimal m-2") {
                   Indexed(
                        iterable=steps,
                        view=|cx, step| view! { cx,
                        li { (step) }    
                        },
                    )  
                }
            }
        }
    }
}
