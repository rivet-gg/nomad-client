// Copyright (c) HashiCorp, Inc.
// SPDX-License-Identifier: MPL-2.0

/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeResources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Box<crate::models::NodeCpuResources>>,
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::NodeDeviceResource>>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<Box<crate::models::NodeDiskResources>>,
    #[serde(rename = "MaxDynamicPort", skip_serializing_if = "Option::is_none")]
    pub max_dynamic_port: Option<i32>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<crate::models::NodeMemoryResources>>,
    #[serde(rename = "MinDynamicPort", skip_serializing_if = "Option::is_none")]
    pub min_dynamic_port: Option<i32>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
}

impl NodeResources {
    pub fn new() -> NodeResources {
        NodeResources {
            cpu: None,
            devices: None,
            disk: None,
            max_dynamic_port: None,
            memory: None,
            min_dynamic_port: None,
            networks: None,
        }
    }
}

