use serde::{Deserialize, Serialize};
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
        //TODO: Is there any point using match here?
        if crd_specs.len() == 1 {
            //TODO: Find out why we have to clone twice
            Some(crd_specs[0].clone())
        } else {
            None
        }
    }
    //TODO: Use static lifetime or just clone?
    pub fn get_name(&self) -> Option<&String> {
        self.metadata.get("name")
    }
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

#[derive(Debug)]
pub struct CRDJsonSchema {
    title: String,
    schema_version: String,
    kind: String,
    schema: HashMap<String, OpenAPI>,
}

impl CRDJsonSchema {
    fn get_description(&self) -> String {
        //TODO: We're not actually storing the group granularly?
        format!(
            "Generated JSON schema for {}'s {} CRD",
            self.schema_version, self.schema_version
        )
    }
}

impl From<Crd> for CRDJsonSchema {
    fn from(crd: Crd) -> Self {
        let spec = crd
            .get_current_version()
            .expect("CRD versions must contain at least one marked for storage");
        CRDJsonSchema {
            //TODO: See about ownership and references here
            title: crd
                .get_name()
                .expect("CRD name must be present")
                .to_string(),
            schema_version: format!("{}/{}", crd.get_spec_group(), spec.name),
            kind: crd.get_spec_names_kind(),
            schema: spec.schema,
        }
    }
}

impl fmt::Display for CRDJsonSchema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO: Printing logic
        write!(f, "{{\"$schema\": \"http://json-schema.org/draft-07/schema#\",
            \"title\": \"{}\",
            \"description\": \"{}\",
            \"type\": \"object\",
            \"properties\": {{
                \"apiVersion\": {{
                    \"type\": \"string\",
                    \"enum\": \"{}\",
                    \"description\": \"a string that identifies the version of the schema the object should have. For CRDs this is the crdGroup/version\"
                }},
                \"kind\": {{
                        \"type\": \"string\",
                        \"const\": \"{}\",
                        \"description\": \"a string the identifies the kind of resource that this document represents\"
                    }},
                \"metadata\": {{
                    \"type\": \"object\",
                    \"properties\": {{
                        \"name\": {{
                            \"type\": \"string\",
                            \"description\": \"a string that uniquely identifies this object within the current namespace\"
                        }},
                        \"labels\": {{
                            \"type\": \"object\",
                            \"description\": \"a map of string keys and values that can be used to organize and categorize objects, for more details see https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/\"
                        }},
                        \"annotations\": {{
                            \"type\": \"object\",
                            \"description\": \"a map of string keys and values that can be used by external tooling to store and retrieve arbitrary metadata about this object, for more details see https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations/\"
                        }}
                    }},
                \"required\": [\"name\"]
                }},
            \"spec\": {{}}
            }},
            \"required\": [\"apiVersion\", \"kind\", \"metadata\", \"spec\"],
            \"allOf\": [ {{ \"if\": {{ \"properties\": {{ \"apiVersion\": {{ \"const\": \"{}\" }}}}}}}}]
            }}", self.title, self.get_description(), self.schema_version, self.kind, self.schema_version)
    }
}
