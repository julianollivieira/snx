use blog::app::App;

#[tokio::main]
async fn main() {
    snx::boot::<App>().await
}
