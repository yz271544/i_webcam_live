mod video;

use sycamore::prelude::*;
use sycamore_futures::*;
use tracing::info;
pub use video::Video;

use crate::AppState;

#[component]
pub fn App<G: Html>(ctx: Scope<'_>) -> View<G> {

    spawn_local_scoped(ctx, async move {
        let state = AppState::new().await;
        info!("state: {:?}", state);
        provide_context(ctx, state);
    });

    view!{
        ctx,
        div(class="container p-2"){
            Video()
        }
    }
}
