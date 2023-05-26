use sycamore::prelude::*;
use uuid::Uuid;
use crate::recipes::AppState;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

#[component(inline_props)]
pub fn RecipeCard<G: Html> (cx: Scope, recipe: RcSignal<Recipe>) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let recipe = create_ref(cx, recipe);
    let ingredients = create_signal(cx, recipe.get().ingredients.clone());
    let steps = create_signal(cx, recipe.get().steps.clone());

    let remove_recipe = move |_| app_state.remove_recipe(recipe.get().id);

    view! {cx,
        div(class="rounded my-3 mx-auto lg:w-2/3 w-11/12 border-2 border-indigo-700 shadow") {
            // name
            div(class="rounded-t w-full p-3 bg-indigo-900 border-b-2 border-b-indigo-700") {
                p(class="text-2xl text-white") {
                    (recipe.get().name)
                }
                // time
                p(class="text-white") {
                    (recipe.get().time)
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
