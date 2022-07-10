mod video;
mod controls;

use sycamore::prelude::*;
use sycamore_futures::*;
use crate::AppState;

pub use video::Video;
pub use controls::Controls;

#[component]
pub async fn App<'a, G: Html>(ctx: Scope<'a>) -> View<G> {

    let state = AppState::wrap_state(ctx).await;

    create_effect(ctx, move || {
        state.track();
        spawn_local_scoped(ctx, async move {
            provide_context_ref(ctx, state);
        });
    });

    view!{
        ctx,
        div(class="p-2"){
            Video(state)
        }
    }
}
