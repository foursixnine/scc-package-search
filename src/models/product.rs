/*
 * SUSE Package Search
 *
 * An API to find what products that packages are contained in.
 *
 * The version of the OpenAPI document: 4.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "free", skip_serializing_if = "Option::is_none")]
    pub free: Option<bool>,
    #[serde(rename = "edition", skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
}

impl Product {
    pub fn new() -> Product {
        Product {
            id: None,
            name: None,
            identifier: None,
            r#type: None,
            free: None,
            edition: None,
            architecture: None,
        }
    }
}
