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
pub struct MetasphereOutputs {
    #[serde(rename = "output", deserialize_with = "Option::deserialize")]
    pub output: Option<serde_json::Value>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl MetasphereOutputs {
    pub fn new(output: Option<serde_json::Value>, updated_at: String) -> MetasphereOutputs {
        MetasphereOutputs {
            output,
            updated_at,
        }
    }
}

