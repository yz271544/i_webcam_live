mod video;

use sycamore::prelude::*;
use tracing::info;
pub use video::Video;

use crate::AppState;


#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    let state = AppState::new().await;
    info!("state: {:?}", state);
    provide_context(ctx, state);
    view!{
        ctx,
        div(class="container p-2"){
            Video()
        }
    }
}
