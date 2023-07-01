use sycamore::prelude::*;
use crate::recipes::*;
use web_sys::KeyboardEvent;

#[component]
pub fn Sidebar<G: Html> (cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);   
    let sidebar_class = create_signal(cx, String::new());
    let sidebar_open = create_signal(cx, false);
    let recipe_counter = create_signal(cx, String::new());
    let sidebar_class_closed="fixed top-0 left-0 z-20 w-64 pt-20 h-screen transition-transform -translate-x-full sm:translate-x-0";
    let sidebar_class_open="fixed top-0 left-0 z-20 w-64 pt-20 h-screen transition-transform -translate-x-0 sm:translate-x-0";

    sidebar_class.set(sidebar_class_closed.to_string());

    /* let times_slice = app_state.db
        .get()
        .iter()
        .map(|recipe| recipe.time.to_lowercase().replace(" min", ""))
        .collect::<Vec<_>>();

    panic!("{:?}", times_slice); */

    let all_times = app_state.db
        .get()
        .iter()
        .map(|recipe| recipe.time.to_lowercase().replace(" min", ""))
        .collect::<Vec<_>>();
    let min_time = all_times.iter().min().unwrap().clone();
    let max_time = all_times.iter().max().unwrap().clone();
    let slider_value = create_signal(cx, max_time.clone());
    let update_time_state = move |_| app_state.maxTime.set(slider_value.get().as_ref().clone());

    let toggle_sidebar = move |_| {
        if sidebar_open.get().as_ref().clone() {
            sidebar_class.set(sidebar_class_closed.to_string());
            sidebar_open.set(false);
        } else {
            sidebar_class.set(sidebar_class_open.to_string());
            sidebar_open.set(true);
        }
    };

    // let toggle_modal = move |_| app_state.toggle_modal();
    let toggle_modal_and_sidebar = move |_| {
        sidebar_class.set(sidebar_class_closed.to_string());
        sidebar_open.set(false);
        app_state.toggle_modal();
    };

    let remove_all_recipes = move |_| app_state.remove_all_recipes();
    let get_random_recipes = move |_| {
        if recipe_counter.get().as_ref().clone() != String::new() {
            let amount = recipe_counter.get().as_ref().clone().parse::<i32>().unwrap();

            for n in 1..=amount {
                app_state.recipes.modify().push(app_state.get_random_recipe());
            }
        }
    };
    let toggle_filter = move |_| match *app_state.filter.get() {
        Filter::Title => app_state.filter.set(Filter::Ingredients),
        Filter::Ingredients => app_state.filter.set(Filter::Title),
    };
    let handle_keyup = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            get_random_recipes;
        }
    };
    view! { cx,
        nav(class="fixed top-0 z-30 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-indigo-700") {
          div(class="px-3 py-3 lg:px-5 lg:pl-3") { 
            div(class="flex items-center justify-between") {
              div(class="flex items-center justify-start sm:justify-center w-full") {
                  button (on:click=toggle_sidebar, data-drawer-target="separator-sidebar", data-drawer-toggle="separator-sidebar", aria-controls="separator-sidebar", type="button", class="inline-flex items-center p-2 mt-2 ml-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600") {
                      span (class="sr-only") {
                          "Open sidebar"
                      }
                      svg (class="w-6 h-6", aria-hidden="true", fill="currentColor", viewBox="0 0 20 20", xmlns="http://www.w3.org/2000/svg") {
                          path (clip-rule="evenodd", fill-rule="evenodd", d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z") {}
                      }
                  }
                  h1(class="text-indigo-700 lg:text-7xl text-4xl m-2 border-b border-b-indigo-700 text-center font-extrabold") {
                      "R&J Meals"
                  }
              }
            }
          }
        }

        aside (id="separator-sidebar", class=(sidebar_class.get()), aria-label="Sidebar") {
            div (class="h-full px-3 sm:py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800 border-r border-r-indigo-700") {
                ul (class="pt-4 sm:mt-4 space-y-2 font-medium") {
                    li {
                        // Modal toggle
                        a (href="#", on:click=toggle_modal_and_sidebar, class="flex items-center p-2 text-white transition duration-75 rounded-lg bg-indigo-700 sm:bg-transparent hover:bg-indigo-700 group") { span (class="ml-4") {
                                "Grocery List"
                            }
                        }
                    }
                    li(class="border-b border-b-indigo-700") {
                        a (href="#", on:click=remove_all_recipes, class="flex items-center p-2 mb-2 text-white transition duration-75 rounded-lg bg-red-900 sm:bg-transparent hover:bg-red-900 group") {
                            span (class="ml-4") {
                                "Remove All"
                            }
                        }
                    }
                    li(class="border-b border-b-indigo-700") {
                        label(for="filters", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                            "Search by:"
                        }
                        select(id="filters", on:change=toggle_filter, class="bg-gray-50 mb-2 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-indigo-700 focus:border-indigo-700 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-indigo-700 dark:focus:border-indigo-700") {
                            RecipeFilter(filter=Filter::Title)
                            RecipeFilter(filter=Filter::Ingredients)
                        }

                    }
                    li(class="border-b border-b-indigo-700"){
                        label(for="steps-range", class="block mb-2 text-sm font-medium text-gray-900 dark:text-white") {
                            "Max time: " (slider_value.get()) " minutes"
                        }
                        input(id="steps-range", type="range", on:change=update_time_state, bind:value=slider_value, min=min_time, max=max_time, value="30", step="5", class="w-full h-2 mb-2 bg-gray-200 rounded-lg appearance-none cursor-pointer dark:bg-gray-700") {}
                    }
                    li {
                        label(for="exampleFormControlInputNumber", class="pointer-events-none mb-0 max-w-[90%] truncate pt-[0.37rema] leading-[1.6] text-white transition-all duration-200 ease-out peer-focus:-translate-y-[0.9rem] peer-focus:scale-[0.8] peer-focus:text-primary peer-data-[te-input-state-active]:-translate-y-[0.9rem] peer-data-[te-input-state-active]:scale-[0.8] motion-reduce:transition-none dark:text-neutral-200 dark:peer-focus:text-primary") {
                            "Add " (recipe_counter.get())  " random recipes:"
                        }
                        div(class="justify-between") {
                            input(on:keyup=handle_keyup, type="number", bind:value=recipe_counter, min=0, class="peer min-h-[auto] w-3/4 rounded border-0 bg-gray-700 px-3 py-[0.32rem] leading-[1.6] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 peer-focus:text-primary data-[te-input-state-active]:placeholder:opacity-100 motion-reduce:transition-none dark:text-white dark:placeholder:text-white dark:peer-focus:text-primary [&:not([data-te-input-placeholder-active])]:placeholder:opacity-0", id="exampleFormControlInputNumber", placeholder="Example label") {}
                            button(class="p-1.5 rounded bg-gray-700 text-white float-right", on:click=get_random_recipes) {
                                "Add"
                            }
                        }
                    }
                }
            }
        }
    }
}
