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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceSize {
    #[serde(rename = "s")]
    S,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "l")]
    L,
    #[serde(rename = "xl")]
    Xl,

}

impl std::fmt::Display for InstanceSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::S => write!(f, "s"),
            Self::M => write!(f, "m"),
            Self::L => write!(f, "l"),
            Self::Xl => write!(f, "xl"),
        }
    }
}

impl Default for InstanceSize {
    fn default() -> InstanceSize {
        Self::S
    }
}

