use sycamore::prelude::*;
use rand::Rng;
use crate::{card::*, groceries::{get_ingredient_quantity, get_ingredient_measurement, get_ingredient_item}};
use uuid::Uuid;
use std::fmt;
use crate::groceries::Ingredient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    Title,
    Ingredients
}

impl Default for Filter {
    fn default() -> Self {
        Self::Title
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Filter::Title => write!(f, "Title"),
            Filter::Ingredients => write!(f, "Ingredients"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AppState {
    pub db: RcSignal<Vec<Recipe>>,
    pub recipes: RcSignal<Vec<Recipe>>,
    pub filter: RcSignal<Filter>,
    pub maxTime: RcSignal<String>,
    pub modal: RcSignal<bool>,
}

impl AppState {
    pub fn replace_recipe(&self, id: Uuid) {
        let index = self.recipes.modify().iter().position(|recipe| recipe.id == id).unwrap();
        self.recipes.modify().insert(index, self.get_random_recipe());
        self.remove_recipe(id)
    }

    pub fn get_random_recipe(&self) -> Recipe {
        let mut rng = rand::thread_rng();
        let db_len = self.db.get().as_ref().clone().len() -1;
        let random_index = rng.gen_range(0..db_len);
        let random_recipe = self.db.get().as_ref().clone()[random_index].clone();

        random_recipe
    }

    pub fn remove_recipe(&self, id: Uuid) {
        self.recipes.modify().retain(|recipe| recipe.id != id)
    }

    pub fn remove_all_recipes(&self) {
        self.recipes.modify().clear()
    }

    pub fn add_from_search(&self, search: String) {
        let mut to_add: Vec<Recipe> = self.db
            .get()
            .as_ref()
            .clone()
            .into_iter()
            .filter(|recipe| match *self.filter.get() {
                Filter::Title => recipe.name.to_lowercase().contains(search.to_lowercase().as_str()),
                Filter::Ingredients => recipe.ingredients.iter().any(|ingredient| ingredient.to_lowercase().contains(search.to_lowercase().as_str())),
            })
            .collect();

        to_add = [self.recipes.get().as_ref().clone(), to_add].concat();
        self.recipes.set(to_add)
    }

    pub fn toggle_modal(&self) {
        self.modal.set(!*self.modal.get())
    }

    pub fn get_ingredients(&self) -> Vec<Ingredient> {
        let mut ingredients = Vec::new();
        for recipe in self.recipes.get().as_ref().clone().iter() {
            for ingredient in recipe.ingredients.iter() {
                ingredients.push(Ingredient {
                    quantity: get_ingredient_quantity(ingredient.clone()),
                    measurement: get_ingredient_measurement(ingredient.clone()),
                    item: get_ingredient_item(ingredient.clone()),
                });
            }
        }

        ingredients
    }
}

#[component]
pub fn Show_Recipes<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let recipes = create_memo(cx, || app_state.recipes
                              .get()
                              .iter()
                              .filter(|recipe| {
                                let time_string = String::from(recipe.time.get(0..3).unwrap());
                                let recipe_time = match time_string.trim().parse::<i32>() {
                                    Ok(time) => time,
                                    Err(err) => panic!("Something went wrong converting {}: {}", time_string, err),
                                };
                                let max_time = match app_state.maxTime.get().as_ref().clone().parse::<i32>() {
                                    Ok(time) => time,
                                    Err(err) => panic!("Something went wrong converting {}: {}", app_state.maxTime.get().as_ref().clone(), err),
                                };
                                recipe_time <= max_time
                              })
                              .cloned()
                              .collect::<Vec<_>>());
    view! { cx,
            Keyed(
                iterable=recipes,
                view=|cx, recipe| view! { cx,
                RecipeCard(name=recipe.name, time=recipe.time, ingredients=recipe.ingredients, steps=recipe.steps, id=recipe.id)
                },
                key=|recipe| recipe.id,
            )
    }
}

#[component(inline_props)]
pub fn RecipeFilter<G: Html>(cx: Scope, filter: Filter) -> View<G> {
    let app_state = use_context::<AppState>(cx);
    let selected = move || filter == *app_state.filter.get();
    // let set_filter = |filter| app_state.filter.set(filter);

    view! { cx,
        option(
            class=if selected() { "selected" } else { "" },
            ) {
            (filter.to_string())
        }
    }
}
