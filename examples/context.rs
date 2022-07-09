use sycamore::{
    context::{ContextProvider, ContextProviderProps, use_context},
    prelude::*,
};

pub enum MyState {
    First,
    Second,
}

#[derive(Clone)]
pub struct MyContext {
    pub state: Signal<MyState>,
}

#[component(FirstState<G>)]
pub fn first_state() -> View<G> {
    let context = use_context::<MyContext>();
    let on_click = cloned!((context) => move |_| {
        context.state.set(MyState::Second);
    });
 
    view! {
        button(on:click=on_click) {
            "Switch to second state"
        }
    }
}

#[component(SecondState<G>)]
pub fn second_state() -> View<G> {
    let context = use_context::<MyContext>();
    let on_click = cloned!((context) => move |_| {
        context.state.set(MyState::First);
    });
 
    view! {
        button(on:click=on_click) {
            "Switch to first state"
        }
    }
}

#[component(App<G>)]
pub fn app() -> View<G> {
    view! {
        ContextProvider(ContextProviderProps {
            value: MyContext { state: Signal::new(MyState::First) },
            children: || {
                let context = use_context::<MyContext>();
                view! {
                    (match &*context.state.get() {
                        MyState::First => view! {
                            FirstState()
                        },
                        MyState::Second => view! {
                            SecondState()
                        },
                    })
                }
            }
        })
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| view! { App() });
}
