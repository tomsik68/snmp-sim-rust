/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AgentsGet400Response {
    #[serde(rename = "error")]
    pub error: String,
}

impl AgentsGet400Response {
    pub fn new(error: String) -> AgentsGet400Response {
        AgentsGet400Response {
            error,
        }
    }
}


