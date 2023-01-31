# npm-package
A very light-weight sync and async client for fetching metadata from the npm registry for a npm package. 

## Usage
These examples come from our [examples](examples/) folder

### Using the Async Client
```rust
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
```

### Using the Sync Client
```rust
use npm_package::SyncNpmClient;

fn main() {
    let client = SyncNpmClient::new();
    let package = client.get("is-interactive").unwrap();
    let version_history = package.versions.keys().collect::<Vec<_>>();

    println!("All is-interactive releases on npm: {:?}", version_history);
}
```

