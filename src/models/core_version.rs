/*
 * Vigilia
 *
 * API for the Vigilia Service
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoreVersion {
    #[serde(rename = "serviceVersion")]
    pub service_version: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
}

impl CoreVersion {
    pub fn new(service_version: String, api_version: String) -> CoreVersion {
        CoreVersion {
            service_version,
            api_version,
        }
    }
}

