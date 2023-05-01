//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::cmp::Ordering;
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

fn main() {
    //TODO: should we use if let here?
    if let Ok(file) = fs::File::open(String::from("resource-definition.yaml")) {
        let crd: Crd = serde_yaml::from_reader(file).unwrap();
        println!("{:?}", crd);
        let crd_spec: Vec<Version> = crd
            .spec
            .versions
            .into_iter()
            .filter(|x| x.storage)
            .collect();
        //TODO: would it be better here to only check success condition with an if
        //  and throw a generic panic for all non-1 cases?
        match crd_spec.len().cmp(&1) {
            //TODO: These side effects kinda smell....
            //TODO: Preeeetty sure panicking is lousy for testing, even then the text coupling is poor.
            Ordering::Less => panic!("No specs found marked as storage"),
            Ordering::Equal => (),
            Ordering::Greater => {
                panic!("One and only one version must be marked as the storage version")
            }
        };
        if let Some(crd_name) = crd.metadata.get("name") {
            println!("{}", crd_name);
        };

        todo!("convert data structure to schema");
    }
}
