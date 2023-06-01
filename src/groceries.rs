use sycamore::prelude::*;
use crate::recipes::AppState;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub quantity: String,
    pub measurement: String,
    pub item: String,
}

pub fn get_ingredient_quantity(ingredient: String) -> String {
    let re = Regex::new(r"^\d*[^a-zA-Z \*]?\d*").unwrap();

    let result = re.find(ingredient.as_str());

    match result {
        Some(quantity) => String::from(quantity.as_str()),
        _ => String::from("")
    } 
}

pub fn get_ingredient_measurement(ingredient: String) -> String {
    let re = Regex::new(r"tbsps?|tsps?|cups?|cans?|packages?|packets?|ozs?|pounds?").unwrap();

    let result = re.find(ingredient.as_str());

    match result {
        Some(measurement) => String::from(measurement.as_str()),
        _ => String::from("")
    } 
}

pub fn get_ingredient_item(ingredient: String) -> String {
    let measurement = get_ingredient_measurement(ingredient.clone());
    if measurement == String::from("") {
        let re = Regex::new(r"\\*?[a-zA-Z].*").unwrap();
        let result = re.find(ingredient.as_str());

        match result {
            Some(item) => String::from(item.as_str()),
            _ => String::from("")
        } 
    } else {
        let ingredient_without_measurement = ingredient.replace(measurement.as_str(), "");
        let re = Regex::new(r"\\*?[a-zA-Z].*").unwrap();
        let result = re.find(ingredient_without_measurement.as_str());

        match result {
            Some(item) => String::from(item.as_str()),
            _ => String::from("")
        } 
    }
}

pub fn combine_ingredients(ingredients: Vec<Ingredient>) -> Vec<Ingredient> {
    let mut ingredients = ingredients.clone();
    if ingredients.len() > 2 {

        ingredients.sort_by_key(|ingredient| ingredient.item.clone());
        let mut i = 0;
        let mut max = ingredients.len() - 2;
        while i < max {
            let j = i + 1; 
            let ingredient_i = ingredients.get(i).unwrap();
            let ingredient_j = ingredients.get(j).unwrap();
            if ingredient_i.item.contains(ingredient_j.item.as_str()) || ingredient_j.item.contains(ingredient_i.item.as_str()) {
                if ingredient_i.measurement.contains(ingredient_j.measurement.as_str()) || ingredient_j.measurement.contains(ingredient_i.measurement.as_str()) {
                    let mut insert = Ingredient {
                        quantity: String::from(""),
                        measurement: String::from(""),
                        item: String::from(""),
                    }; 
                    if ingredient_i.quantity != String::from("") {
                        if ingredient_i.item.len() > ingredient_j.item.len() {
                            insert.item = ingredient_j.item.clone();
                        } else {
                            insert.item = ingredient_i.item.clone();
                        }

                        if ingredient_i.measurement.len() > ingredient_j.measurement.len() {
                            insert.measurement = ingredient_j.measurement.clone();
                        } else {
                            insert.measurement = ingredient_i.measurement.clone();
                        }


                        if ingredient_i.quantity.contains(".") || ingredient_j.quantity.contains(".") {
                            let float_i = ingredient_i.quantity.parse::<f32>().unwrap();
                            let float_j = ingredient_j.quantity.parse::<f32>().unwrap();
                            let added = float_i + float_j;
                            insert.quantity = added.to_string();
                        } else {
                            let int_i = ingredient_i.quantity.parse::<i32>().unwrap();
                            let int_j = ingredient_j.quantity.parse::<i32>().unwrap();
                            let added = int_i + int_j;
                            insert.quantity = added.to_string();
                        }
                    }
                    ingredients.remove(j);
                    if insert.item != String::from("") {
                        ingredients.insert(j, insert);
                        ingredients.remove(i);
                    }
                    i = i - 1;
                    max = max - 1;
                }
            } 
            i = i + 1;
        }
        ingredients
    } else {
        ingredients
    }
}

#[component]
pub fn GroceriesModal<G: Html> (cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);

    let ingredient_list = create_signal(cx, app_state.get_ingredients());

    let combined_ingredients = create_memo(cx, || {
        let mut combined = combine_ingredients(ingredient_list.get().as_ref().clone());
        combined.sort_by_key(|ingredient| ingredient.item.clone());
        combined
    });

    let toggle_modal = move |_| app_state.toggle_modal();
    view! { cx,
        div(id="defaultModal", tabindex="-1", aria-hidden="true", class="flex items-center justify-center z-50 w-full p-4 overflow-x-hidden overflow-y-auto inset-0 h-[calc(100%-1rem)] max-h-full") {
            div(class="relative w-full max-w-2xl max-h-full") {
                // <!-- Modal content -->
                div(class="relative bg-white rounded-lg shadow dark:bg-gray-800") { 
                    // <!-- Modal header -->
                    div(class="flex items-start justify-between p-4 border-b rounded-t dark:border-gray-600") {
                        h3(class="text-xl font-semibold text-gray-900 dark:text-white") {
                            "Grocery List"
                        }
                        button(type="button", on:click=toggle_modal, class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white", data-modal-hide="defaultModal") {
                            svg(aria-hidden="true", class="w-5 h-5", fill="currentColor", viewBox="0 0 20 20", xmlns="http://www.w3.org/2000/svg") {
                                path(fill-rule="evenodd", d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z", clip-rule="evenodd") {}
                            }
                            span(class="sr-only") {
                                "Close modal"
                            }
                        }
                    }
                    // <!-- Modal body -->
                    div(class="p-6 space-y-6") {
                        /* p(class="text-base leading-relaxed text-gray-500 dark:text-gray-400") {
                            "With less than a month to go before the European Union enacts new consumer privacy laws for its citizens, companies around the world are updating their terms of service agreements to comply."
                        }
                        p(class="text-base leading-relaxed text-gray-500 dark:text-gray-400") {
                            "The European Union’s General Data Protection Regulation (G.D.P.R.) goes into effect on May 25 and is meant to ensure a common set of data rights in the European Union. It requires organizations to notify users as soon as possible of high-risk data breaches that could personally affect them."
                        } */
                        Indexed(
                            iterable=combined_ingredients,
                            view=|cx, ingredient| view! { cx, 
                                input(id="ingredient", type="checkbox") {
                                }
                                label(for="ingredient", class="text-white") {
                                    " "(ingredient.quantity) " " (ingredient.measurement) " " (ingredient.item)
                                }
                                br {}
                            },
                        )

                    }
                }
            }
        }
    }
}
