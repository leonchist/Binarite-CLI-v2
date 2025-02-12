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
pub struct DeleteUserResponse {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "status")]
    pub status: models::ResponseStatus,
}

impl DeleteUserResponse {
    pub fn new(message: String, status: models::ResponseStatus) -> DeleteUserResponse {
        DeleteUserResponse {
            message,
            status,
        }
    }
}

