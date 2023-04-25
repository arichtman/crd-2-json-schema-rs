//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]

// use openapiv3::OpenAPI as OtherOpenAPI;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Names {
    plural: String,
    singular: String,
    kind: String, //TODO: TBD, could be enum
    shortNames: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Version {
    name: String,
    served: bool,
    storage: bool,
    // schema: OpenAPI,
    // schema: HashMap<OpenAPI>,
    schema: HashMap<String, OpenAPI>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OpenAPI {
    // r#type: String,
    // properties: OpenAPISpec,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OpenAPISpec {
    // r#type: String,
    //TODO: Possibly adjust the nested hashmap if it's always "type": String
    // properties: HashMap<String, HashMap<String, String>>,
}

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct OpenAPIProperties {
//     HashMap<String, HashMap<String, String>>
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Spec {
    group: String,
    versions: Vec<Version>,
    scope: String,
    names: Names,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Crd {
    apiVersion: String,
    kind: String,
    metadata: HashMap<String, String>,
    spec: Spec,
}

fn main() {
    //TODO: should we use if let here?
    if let Ok(file) = fs::File::open(String::from("resource-definition.yaml")) {
        // let data_structure: OpenAPI = serde_yaml::from_reader(file).unwrap();
        let data_structure: Crd = serde_yaml::from_reader(file).unwrap();
        // if let Ok(file_contents) = fs::read_to_string(String::from("resource-definition.yaml")) {
        // let data_structure = serde_yaml::from_str(&file_contents);
        println!("{:?}", data_structure);
        todo!("main function after data load");
    }
}
