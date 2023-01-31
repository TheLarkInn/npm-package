use npm_package::SyncNpmClient;

fn main() {
    let client = SyncNpmClient::new();
    let package = client.get("is-interactive").unwrap();
    let version_history = package.versions.keys().collect::<Vec<_>>();

    println!("All is-interactive releases on npm: {:?}", version_history);
}
