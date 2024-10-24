use view::extract::extract_args;
mod clean;
mod fetch;
mod search;
mod view;
#[tokio::main]
async fn main() {
    extract_args().await;
}
