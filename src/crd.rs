use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Names {
    plural: String,
    singular: String,
    pub kind: String, //TODO: TBD, could be enum
    shortNames: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Version {
    pub name: String,
    served: bool,
    pub storage: bool,
    schema: HashMap<String, OpenAPI>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OpenAPI {
    r#type: String,
    //TODO: Same potential optimization if it's always spec
    properties: HashMap<String, OpenAPISpec>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OpenAPISpec {
    r#type: String,
    //TODO: Possibly adjust the nested hashmap if it's always "type": String
    properties: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Spec {
    pub group: String,
    pub versions: Vec<Version>,
    scope: Scope,
    pub names: Names,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Scope {
    Namespaced,
    Cluster,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Crd {
    apiVersion: String,
    kind: String,
    pub metadata: HashMap<String, String>,
    pub spec: Spec,
}
