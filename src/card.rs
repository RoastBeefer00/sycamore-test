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

    let body_visible = create_signal(cx, false);

    let remove_recipe = move |_| app_state.remove_recipe(recipe.id);
    let replace_recipe = move |_| app_state.replace_recipe(recipe.id);
    let toggle_body_visible = |_| body_visible.set(!body_visible.get().as_ref().clone());

    view! {cx,
        div(class="rounded my-3 mx-auto lg:w-2/3 w-11/12 border-2 border-indigo-700 shadow") {
            // name
            div(class="rounded-t w-full p-3 bg-indigo-900 border-b-2 border-b-indigo-700") {
                p(class="text-2xl text-white font-bold") {
                    (recipe.name)
                }
                // time
                p(class="text-white") {
                    (recipe.time)
                }
                (if *body_visible.get() {
                    view! { cx,
                        button(class="rounded bg-slate-700 text-white p-2 mt-2", on:click=toggle_body_visible) {
                            "Hide"
                        }
                    }
                } else {
                    view! { cx, 
                        button(class="rounded bg-slate-700 text-white p-2 mt-2", on:click=toggle_body_visible) {
                            "Show"
                        }
                    }
                })
                button(class="rounded bg-red-900 text-white p-2 mx-2 mt-2 float-right", on:click=remove_recipe) {
                    "Remove"
                }
                button(class="rounded bg-slate-700 text-white p-2 mt-2 float-right", on:click=replace_recipe) {
                    "Replace"
                }
            }
            (if *body_visible.get() {
                view! { cx, 
                    div(class="bg-indigo-900 p-4 rounded-b") {
                        // ingredients
                        div(class="border-b border-l border-r p-5 bg-gray-300 mb-4 rounded") {
                            p(class="font-bold") {
                                "Ingredients:"
                            }
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
                        div(class="rounded-b border-b border-l border-r p-5 bg-gray-300 rounded") {
                            p(class="font-bold") {
                                "Steps:"
                            }
                            ul(class="list-decimal m-2") {
                                Indexed(
                                    iterable=steps,
                                    view=|cx, step| view! { cx,
                                        li(class="border-b border-indigo-700 m-2") { (step) }    
                                    },
                                )  
                            }
                        }
                    }
                }
            } else {
                view! { cx, }
            })
        }
    }
}
