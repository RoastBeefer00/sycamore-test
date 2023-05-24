use sycamore::prelude::*;
use uuid::Uuid;
use itertools::Itertools;

#[derive(Props, Clone, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub name: String,
    pub time: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
    pub id: Uuid,
}

pub fn get_recipes_from_search(search: String, db: &Vec<Recipe>, recipes: &Signal<Vec<Recipe>>) -> Vec<Recipe> {
    let current_recipes = recipes.get().as_ref().clone();
    let to_add = db.clone().into_iter().filter(|recipe| recipe.name.to_lowercase().contains(search.to_lowercase().as_str())).collect();
    let mut ret = [current_recipes, to_add].concat();
    ret.sort_by_key(|recipe| recipe.id);
    ret.dedup_by_key(|recipe| recipe.id);
    ret
}

pub fn remove_recipe(recipes: &Signal<Vec<Recipe>>, id: Uuid) -> Vec<Recipe> {
    let current_recipes = recipes.get().as_ref().clone();
    let ret = current_recipes.into_iter().filter(|recipe| recipe.id != id).collect();
    ret
}

#[component]
pub fn RecipeCard<G: Html> (cx: Scope, recipe: Recipe) -> View<G> {
    let ingredients = create_signal(cx, recipe.ingredients);
    let steps = create_signal(cx, recipe.steps);

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
            div(class="text-white") {
                (recipe.id.to_string())
            }
        }
    }
}
