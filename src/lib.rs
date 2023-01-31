use reqwest::blocking::Client as SyncClient;
use reqwest::Client as AsyncClient;
use serde::Deserialize;
use std::collections::HashMap;

const NPM_REGISTRY_URL: &str = "https://registry.npmjs.org";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NpmPackage {
    pub _id: String,
    pub _rev: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, NpmPackageVersion>,
    pub time: HashMap<String, String>,
    pub author: NpmPackageAuthor,
    pub repository: Option<NpmPackageRepository>,
    pub readme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NpmPackageVersion {
    pub name: String,
    pub version: String,
    pub homepage: String,
    pub repository: Option<NpmPackageRepository>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(alias = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub scripts: HashMap<String, String>,
    pub author: NpmPackageAuthor,
    pub license: Option<String>,
    pub readme: Option<String>,
    #[serde(alias = "readmeFilename")]
    pub readme_filename: Option<String>,
    pub _id: String,
    pub description: String,
    pub dist: NpmPackageVersionDist,
    #[serde(alias = "_npmVersion")]
    pub _npm_version: Option<String>,
    #[serde(alias = "_npmUser")]
    pub _npm_user: Option<NpmPackageAuthor>,
    pub maintainers: Vec<NpmPackageAuthor>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NpmPackageVersionDist {
    pub shasum: String,
    pub tarball: String,
}

#[derive(Debug, Deserialize)]
pub struct NpmPackageRepository {
    pub r#type: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct NpmPackageAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct SyncNpmClient {
    pub client: SyncClient,
}

impl SyncNpmClient {
    pub fn new() -> Self {
        Self {
            client: SyncClient::new(),
        }
    }

    pub fn get(&self, package_name: &str) -> Result<NpmPackage, reqwest::Error> {
        let package_url = format!("{}/{}", NPM_REGISTRY_URL, package_name);

        let response = self.client.get(&package_url).send()?;
        let package: NpmPackage = response.json()?;

        Ok(package)
    }
}

pub struct AsyncNpmClient {
    pub client: AsyncClient,
}

impl AsyncNpmClient {
    pub fn new() -> Self {
        Self {
            client: AsyncClient::new(),
        }
    }

    pub async fn get(&self, package_name: &str) -> Result<NpmPackage, reqwest::Error> {
        let package_url = format!("{}/{}", NPM_REGISTRY_URL, package_name);

        let response = self.client.get(&package_url).send().await?;
        let package: NpmPackage = response.json().await?;

        Ok(package)
    }
}

#[cfg(test)]
mod tests {
    use crate::SyncNpmClient;

    #[test]
    fn test_sync_get_name() {
        let client = SyncNpmClient::new();
        let package = client.get("webpack").unwrap();

        assert_eq!(package.name, "webpack");
    }

    #[test]
    fn test_sync_getting_a_bad_package_name() {
        let client = SyncNpmClient::new();
        let package = client.get("webpacksadfas");

        assert!(package.is_err());
    }

    #[tokio::test]
    async fn test_async_get_name() {
        let client = crate::AsyncNpmClient::new();
        let package = client.get("webpack").await.unwrap();

        assert_eq!(package.name, "webpack");
    }

    #[tokio::test]
    async fn test_async_getting_a_bad_package_name() {
        let client = crate::AsyncNpmClient::new();
        let package = client.get("webpacksadfas").await;

        assert!(package.is_err());
    }

    #[test]
    fn test_npm_version_field_serde() {
        let client = SyncNpmClient::new();
        let package = client.get("webpack").unwrap();

        assert_eq!(
            package.versions["5.0.0"]._npm_version,
            Some("6.14.8".to_string())
        );
    }

    #[test]
    fn test_readme_version_filename_return_none_serde() {
        let client = SyncNpmClient::new();
        let package = client.get("webpack").unwrap();

        assert_eq!(package.versions["5.0.0"].readme_filename, None);
    }
}
