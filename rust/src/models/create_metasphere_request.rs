/*
 * platform_manager
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMetasphereRequest {
    #[serde(rename = "cloud_provider", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<Option<String>>,
    #[serde(rename = "cloud_region", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cloud_region: Option<Option<String>>,
    #[serde(rename = "instance_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<Option<i32>>,
    #[serde(rename = "instance_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance_size: Option<Option<models::InstanceSize>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "template", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub template: Option<Option<String>>,
    #[serde(rename = "terraform_var", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub terraform_var: Option<Option<serde_json::Value>>,
}

impl CreateMetasphereRequest {
    pub fn new(name: String) -> CreateMetasphereRequest {
        CreateMetasphereRequest {
            cloud_provider: None,
            cloud_region: None,
            instance_count: None,
            instance_size: None,
            name,
            template: None,
            terraform_var: None,
        }
    }
}

