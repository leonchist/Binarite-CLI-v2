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
pub struct MetasphereDb {
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: String,
    #[serde(rename = "cloud_region")]
    pub cloud_region: String,
    #[serde(rename = "created_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Option<String>>,
    #[serde(rename = "deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<Option<String>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "instance_count")]
    pub instance_count: i32,
    #[serde(rename = "instance_size")]
    pub instance_size: models::InstanceSize,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: models::MetasphereStatus,
    #[serde(rename = "template")]
    pub template: String,
    #[serde(rename = "terraform_var", deserialize_with = "Option::deserialize")]
    pub terraform_var: Option<serde_json::Value>,
    #[serde(rename = "updated_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Option<String>>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl MetasphereDb {
    pub fn new(cloud_provider: String, cloud_region: String, id: i32, instance_count: i32, instance_size: models::InstanceSize, name: String, status: models::MetasphereStatus, template: String, terraform_var: Option<serde_json::Value>, uuid: uuid::Uuid) -> MetasphereDb {
        MetasphereDb {
            cloud_provider,
            cloud_region,
            created_at: None,
            deleted_at: None,
            id,
            instance_count,
            instance_size,
            name,
            status,
            template,
            terraform_var,
            updated_at: None,
            uuid,
        }
    }
}

