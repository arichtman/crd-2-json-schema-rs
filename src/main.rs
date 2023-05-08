//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]

use json_schema::{
    JSONSchema, JSONSchemaObject, JSONSchemaObjectBuilder, Properties, SimpleTypes,
    Type as SchemaType,
};
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
    //TODO: I wonder how much difference there is between using ?, unwrap, and expect...
    let file =
        fs::File::open(String::from("resource-definition.yaml")).expect("Error opening file");
    let crd: Crd = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", crd);
    let crd_specs: Vec<Version> = crd
        .spec
        .versions
        .into_iter()
        //TODO: I wonder if there's a better way to get .storage, maybe like flatten + filter?
        .filter(|x| x.storage)
        .collect();
    //TODO: This still feels lousy for testing purposes.
    assert_eq!(crd_specs.len, 1);
    let crd_spec = &crd_specs[0];
    //TODO: Same question for testing...
    let crd_name = crd.metadata.get("name").expect("CRD must have a name");
    println!("{}", crd_name);
    let crd_group = crd.spec.group;
    let crd_spec_name = &crd_spec.name;
    let crd_spec_kind = &crd.spec.names.kind;
    //TODO: Rethink the use of this library. It's a LOT of fiddling around when we can probably
    //   just dynamically populate a data structure and serialize that out to JSON...
    let mut properties: HashMap<String, JSONSchema> = HashMap::new();
    //TODO: Handle returns of builder error
    let api_version_schema_object = JSONSchemaObjectBuilder::default()
        ._type(SchemaType::SimpleTypes(SimpleTypes::String))
        //TODO: Sort out the ownership nonsense
        ._enum(vec![serde_json::value::Value::String(crd_spec.name.clone())])
        .description(String::from(" a string that identifies the version of the schema the object should have. For CRDs this is the crdGroup/version"))
        // crd.spec.versions.keys())))
        .build();
    properties.insert(
        String::from("apiVersion"),
        //TODO: bruh ain't no way this is how it's meant to be
        JSONSchema::JSONSchemaObject(api_version_schema_object.as_ref().unwrap().clone()),
    );
    // let api_property: HashMap<String, JSONSchema> = { String::from("apiVersion"):  };
    let schema = JSONSchemaObjectBuilder::default()
        .schema(String::from("http://json-schema.org/draft-07/schema#"))
        //TODO: fix this borrowing nonsense
        .title(crd_name.to_owned())
        .description(format!(
            "Generated JSON schema for {}'s {} CRD",
            crd_group, crd_spec_kind
        ))
        ._type(SchemaType::SimpleTypes(SimpleTypes::Object))
        .properties(properties)
        .build()
        .unwrap();
    println!("{:?}", schema);
    todo!("convert data structure to schema");
}
