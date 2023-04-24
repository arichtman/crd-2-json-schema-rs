//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code
)]

use openapiv3::OpenAPI;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::fs;

#[derive(Debug)]
struct Names {
    plural: String,
    singular: String,
    kind: String, //TODO: TBD, could be enum
    short_names: Vec<String>,
}

#[derive(Debug)]
struct Version {
    name: String,
    served: bool,
    storage: bool,
    schema: OpenAPI,
}

#[derive(Debug)]
struct Spec {
    group: String,
    versions: Vec<Version>,
    scope: String,
    names: Names,
}

#[derive(Debug)] //, Deserialize)]
struct Crd {
    api_version: String,
    kind: String,
    metadata: Vec<(String, String)>,
    spec: Spec,
}

fn main() {
    //TODO: should we use if let here?
    if let Ok(file) = fs::File::open(String::from("resource-definition.yaml")) {
        let data_structure: OpenAPI = serde_yaml::from_reader(file).unwrap();
        // let data_structure: Crd = serde_yaml::from_reader(file).unwrap();
        // if let Ok(file_contents) = fs::read_to_string(String::from("resource-definition.yaml")) {
        // let data_structure = serde_yaml::from_str(&file_contents);
        println!("{:?}", data_structure);
        todo!();
    }
}
