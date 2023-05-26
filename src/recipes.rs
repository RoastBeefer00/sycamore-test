use sycamore::prelude::*;
use crate::card::*;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AppState {
    pub db: RcSignal<Vec<Recipe>>,
    pub recipes: RcSignal<Vec<Recipe>>,
}

#[component]
pub fn Show_Recipes<G: Html>(cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let recipes = create_memo(cx, || app_state.recipes.get().iter().filter(|_| true).cloned().collect::<Vec<_>>());
    // let recipes = create_signal(cx, app_state.recipes.get().as_ref().clone());
    // let recipes = create_selector(cx, || app_state.recipes.get());
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

impl AppState {
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
            .filter(|recipe| recipe.name.to_lowercase().contains(search.to_lowercase().as_str()))
            .collect();
        /* for recipe in to_add.iter() {
            self.recipes.modify().push(recipe.clone());
        } */
        to_add = [self.recipes.get().as_ref().clone(), to_add].concat();
        to_add.sort_by_key(|recipe| recipe.id);
        to_add.dedup_by_key(|recipe| recipe.id);
        self.recipes.set(to_add)
    }
}
