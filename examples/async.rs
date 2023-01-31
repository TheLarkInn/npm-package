use npm_package::AsyncNpmClient;
use tokio;

#[tokio::main]
async fn main() {
    let client = AsyncNpmClient::new();
    let is_wsl_package = client.get("is-wsl").await.unwrap();

    println!(
        "Description of is-wsl from the npm registry: {}",
        is_wsl_package.description
    );
}
