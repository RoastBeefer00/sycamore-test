use sycamore::prelude::*;
use crate::recipes::AppState;

#[component]
pub fn Sidebar<G: Html> (cx: Scope) -> View<G> {
    let app_state = use_context::<AppState>(cx);   
    let sidebar_class = create_signal(cx, String::new());
    let sidebar_open = create_signal(cx, false);
    let sidebar_class_closed="fixed top-0 left-0 z-40 w-64 pt-20 h-screen transition-transform -translate-x-full sm:translate-x-0";
    let sidebar_class_open="fixed top-0 left-0 z-40 w-64 pt-20 h-screen transition-transform -translate-x-0 sm:translate-x-0";

    sidebar_class.set(sidebar_class_closed.to_string());

    let toggle_sidebar = move |_| {
        if sidebar_open.get().as_ref().clone() {
            sidebar_class.set(sidebar_class_closed.to_string());
            sidebar_open.set(false);
        } else {
            sidebar_class.set(sidebar_class_open.to_string());
            sidebar_open.set(true);
        }
    };
    let remove_all_recipes = move |_| app_state.remove_all_recipes();
    view! { cx,
        nav(class="fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-slate-800 dark:border-indigo-700") {
          div(class="px-3 py-3 lg:px-5 lg:pl-3") { 
            div(class="flex items-center justify-between") {
              div(class="flex items-center justify-start") {
                  button (on:click=toggle_sidebar, data-drawer-target="separator-sidebar", data-drawer-toggle="separator-sidebar", aria-controls="separator-sidebar", type="button", class="inline-flex items-center p-2 mt-2 ml-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600") {
                      span (class="sr-only") {
                          "Open sidebar"
                      }
                      svg (class="w-6 h-6", aria-hidden="true", fill="currentColor", viewBox="0 0 20 20", xmlns="http://www.w3.org/2000/svg") {
                          path (clip-rule="evenodd", fill-rule="evenodd", d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z") {}
                      }
                  }
                  h1(class="text-indigo-700 lg:text-7xl text-4xl m-2 border-b border-b-indigo-700 text-center") {
                      "COOK"
                  }
              }
            }
          }
        }

        aside (id="separator-sidebar", class=(sidebar_class.get()), aria-label="Sidebar") {
            div (class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-slate-800 border-r border-r-indigo-700") {
                ul (class="pt-4 mt-4 space-y-2 font-medium") {
                    li {
                        a (href="#", on:click=remove_all_recipes, class="flex items-center p-2 text-white transition duration-75 rounded-lg bg-red-900 sm:bg-transparent hover:bg-red-900 group") {
                            span (class="ml-4") {
                                "Remove All"
                            }
                        }
                    }
                }
            }
        }
    }
}
