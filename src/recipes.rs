use sycamore::prelude::*;
use rand::Rng;
use crate::card::*;
use uuid::Uuid;

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

#[derive(Clone, PartialEq, Eq)]
pub struct AppState {
    pub db: RcSignal<Vec<Recipe>>,
    pub recipes: RcSignal<Vec<Recipe>>,
    pub filter: RcSignal<Filter>,
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
}

#[component]
pub fn Show_Recipes<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let recipes = create_memo(cx, || app_state.recipes
                              .get()
                              .iter()
                              .filter(|_| true)
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
    let set_filter = |filter| app_state.filter.set(filter);

    view! { cx,
        option(
            class=if selected() { "selected" } else { "" },
            on:click=move |_| set_filter(filter),
            ) {
            (format!("{filter:?}"))
        }
    }
}
