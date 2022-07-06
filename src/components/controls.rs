use sycamore::prelude::*;

use crate::AppState;


#[component]
pub fn Controls<G: Html>(ctx: Scope) -> View<G> {

    let app_state = use_context::<AppState>(ctx);
    // let devices = app_state.devices.get().video_devices();

    view! {ctx,
        div(class="absolute bottom-2 p-5 w-full") {
            div(class="flex justfy-center") {
                div(class="mb-4 xl:w-496") {
                    // button(class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded") {
                    //     "Start"
                    // }
                    select(class="form-select appearance-none
                    block
                    w-full
                    px-3
                    py-1.5
                    text-base
                    font-normal
                    text-gray-700
                    bg-white bg-clip-padding bg-no-repeat
                    border border-solid border-gray-300
                    rounded
                    transition
                    ease-in-out
                    m-0
                    focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none", aria-label="Default select example") {
                        option(value="") {
                            "Select"
                        }
                        option(value="user") {
                            "User"
                        }
                        option(value="environment") {
                            "Environment"
                        }
                        // KeyedList::new(devices).map(|(id, device)| {
                        //     option(value=id) {
                        //         device.name
                        //     }
                        // })
                    }   
                }
            }
        }
    }
}