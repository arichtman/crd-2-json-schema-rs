//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]

//TODO: decide if we're using this or not
use openapiv3::OpenAPI as OutputOpenAPI;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
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
    schema: HashMap<String, OpenAPI>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct OpenAPI {
    r#type: String,
    //TODO: Same potential optimization if it's always spec
    properties: HashMap<String, OpenAPISpec>,
}

//TODO: Could this be directly our OpenAPI library struct?
#[derive(Serialize, Deserialize, PartialEq, Debug, JsonSchema)]
struct OpenAPISpec {
    r#type: String,
    //TODO: Possibly adjust the nested hashmap if it's always "type": String
    properties: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Spec {
    group: String,
    versions: Vec<Version>,
    scope: Scope,
    names: Names,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Scope {
    Namespaced,
    Cluster,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Crd {
    apiVersion: String,
    kind: String,
    metadata: HashMap<String, String>,
    spec: Spec,
}

//TODO: wip, build out or delete
impl From<Crd> for OutputOpenAPI {
    fn from(crd: Crd) -> OutputOpenAPI {
        // serde_json::from_value(crd.spec.versions[0].schema);
        todo!();
    }
}

fn main() {
    //TODO: should we use if let here?
    if let Ok(file) = fs::File::open(String::from("resource-definition.yaml")) {
        let data_structure: Crd = serde_yaml::from_reader(file).unwrap();
        println!("{:?}", data_structure);
        if let Some(crd_name) = data_structure.metadata.get("name") {
            println!("{}", crd_name);
        };
        //TODO: I _think_ the if let is better, but I'm not sure what the idiomatic way to test this is...
        if !data_structure.metadata.contains_key("name") {
            panic!()
        };
        //TODO: This works but is based on the struct, not the actual CRD
        //  But if we're generating based on structs, and the structs have to be known at compile time...
        //  I'm missing something here...
        let schema = schema_for!(OpenAPISpec);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        todo!("main function after data load");
    }
}
