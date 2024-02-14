/*
 * SUSE Package Search
 *
 * An API to find what products that packages are contained in.
 *
 * The version of the OpenAPI document: 4.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Clone)]
pub struct Configuration {
    pub server_url: String,
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            server_url: "https://scc.suse.com".to_owned(),
            base_path: "https://scc.suse.com/api/package_search".to_owned(),
            user_agent: Some("QE-Core/OpenAPI-Generator/4.0.0/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}