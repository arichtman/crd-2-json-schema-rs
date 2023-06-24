use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Names {
    plural: String,
    singular: String,
    kind: String, //TODO: TBD, could be enum
    shortNames: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Version {
    name: String,
    served: bool,
    storage: bool,
    schema: HashMap<String, OpenAPI>,
}

//TODO: See if we can use the OpenAPI crate instead of our own
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OpenAPI {
    r#type: String,
    //TODO: Same potential optimization if it's always spec
    properties: HashMap<String, OpenAPISpec>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OpenAPISpec {
    r#type: String,
    //TODO: Possibly adjust the nested hashmap if it's always "type": String
    properties: HashMap<String, HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Spec {
    group: String,
    versions: Vec<Version>,
    scope: Scope,
    names: Names,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Scope {
    Namespaced,
    Cluster,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Crd {
    apiVersion: String,
    kind: String,
    metadata: HashMap<String, String>,
    spec: Spec,
}

impl Crd {
    //TODO: I chose an Option here instead of Result
    //  A) cause wrapping an error type was pissing me off
    //  B) It's not really recoverable or shouldn't be
    //      outside the scope of this module
    pub fn get_current_version(&self) -> Option<Version> {
        let crd_specs: Vec<Version> = self
            .spec
            .versions
            //TODO: interators are consuming so it would move self
            .clone()
            .into_iter()
            //TODO: I wonder if there's a better way to get .storage, maybe like flatten + filter?
            .filter(|x| x.storage)
            //TODO: We can possibly use take(1) here and return immediately
            .collect();
        if crd_specs.len() == 1 {
            //TODO: Find out why we have to clone twice
            Some(crd_specs[0].clone())
        } else {
            None
        }
    }
    pub fn get_name(&self) -> Option<&String> {
        self.metadata.get("name")
    }
    //TODO: Use static lifetime or just clone?
    pub fn get_spec_group(&self) -> String {
        self.spec.group.clone()
    }
    pub fn get_spec_name(&self) -> String {
        self.get_current_version().unwrap().name
    }
    pub fn get_spec_names_kind(&self) -> String {
        self.spec.names.kind.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CRDJsonSchema(serde_json::Value);

impl From<Crd> for CRDJsonSchema {
    fn from(crd: Crd) -> CRDJsonSchema {
        // TODO: Should this error be handled here or do we consider a CRD invalid if no versions stored?
        // let spec = crd
        // .get_current_version()
        // .expect("CRD versions must contain at least one marked for storage");
        let version = crd.get_current_version().unwrap();
        CRDJsonSchema(json!({
        "schema": "http://json-schema.org/draft-07/schema#",
        "title": crd.get_name(),
        // TODO: work out version display normally
        "description": format!("Generated JSON schema for {:?}'s {:?} CRD", version, version),
        "type": "object",
        "properties": {
            "apiVersion": {
                "type": "string",
                "enum": version,
                "description": "a string that identifies the version of the schema the object should have. For CRDs this is the crdGroup/version"
            },
            "kind": {
                    "type": "string",
                    "const": crd.kind,
                    "description": "a string the identifies the kind of resource that this document represents"
                },
            "metadata": {
                "required": [ "name" ],
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "a string that uniquely identifies this object within the current namespace"
                    },
                    "labels": {
                        "type": "object",
                        "description": "a map of string keys and values that can be used to organize and categorize objects, for more details see https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/"
                    },
                    "annotations": {
                        "type": "object",
                        "description": "a map of string keys and values that can be used by external tooling to store and retrieve arbitrary metadata about this object, for more details see https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/"
                    }
                }
            }
        },
        "spec": {},
        "required": [ "apiVersion", "kind", "metadata", "spec" ],
        // TODO: This wacky dynamic bit, somehow
        // "allOf":
        }))
    }
}
//             \"allOf\": [ {{ \"if\": {{ \"properties\": {{ \"apiVersion\": {{ \"const\": \"{}\" }}}}}}}}]
//             }}", self.title, self.get_description(), self.schema_version, self.kind, self.schema_version)
