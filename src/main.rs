//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]
//TODO: When to use which of these?
//TODO: Should we be making these public?
//  I figured they were internal use only by main.rs
mod crd;
use crd::{CRDJsonSchema, Crd, Version};
use serde_yaml;
use std::fs;

fn main() {
    //TODO: I wonder how much difference there is between using ?, unwrap, and expect...
    let file =
        fs::File::open(String::from("resource-definition.yaml")).expect("Error opening file");
    let crd: Crd = serde_yaml::from_reader(file).unwrap();
    // println!("{:?}", crd);
    let crd_spec = crd.get_current_version().unwrap();
    //TODO: Same question for testing...
    let crd_name = crd.get_name().expect("CRD must have a name");
    // println!("{}", crd_name);
    let crd_group = crd.get_spec_group();
    let crd_spec_name = crd.get_spec_name();
    let crd_spec_kind = crd.get_spec_names_kind();
    // So we want a write out
    // I don't think that needs more than an implementation of Display
    // However we want maybe a struct that represents the output jsonschema
    // But I don't think we want a fully typed one cause that's a mountain of boring boilerplate
    // But then we can implement From<Crd> for our jsonschema struct
    // Which should make our call site nice and keep the transformation logic alongside the data model
    // Or maybe I'm talking out my ass, who knows, I'm learning
    let output = CRDJsonSchema::from(crd);
    println!("{}", output);
}
