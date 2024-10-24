use view::extract::extract_args;
mod fetch;
mod search;
mod view;
mod clean;
#[tokio::main]
async fn main() {
    extract_args().await; 
}