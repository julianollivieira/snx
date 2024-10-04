use crate::app::App;

/// Boots the application.
pub async fn boot<A: App>() {
    A::serve().await
}
