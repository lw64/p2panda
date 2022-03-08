// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::BTreeMap;

/// CDDL types.
#[derive(Clone, Debug, Copy)]
pub enum Type {
    Bool,
    Int,
    Float,
    Tstr,
    Relation,
}

/// CDDL types to string representation.
impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Bool => "bool",
            Type::Int => "int",
            Type::Float => "float",
            Type::Tstr => "tstr",
            Type::Relation => "tstr .regexp \"[0-9a-f]{68}\"",
        }
        .to_string()
    }
}

/// CDDL field types.
#[derive(Clone, Debug)]
pub enum Field {
    String(String),
    Type(Type),
    Struct(Group),
}

/// Format each different data type into a string.
impl ToString for Field {
    fn to_string(&self) -> String {
        match self {
            Field::String(str) => str.to_string(),
            Field::Type(cddl_type) => cddl_type.to_string(),
            Field::Struct(group) => group.to_string(),
        }
    }
}

/// Struct for building and representing CDDL groups.
///
/// CDDL uses groups to define reusable data structures they can be merged or used in Vectors,
/// Tables and Structs.
#[derive(Clone, Debug)]
pub struct Group {
    #[allow(dead_code)] // Remove when module is used.
    name: String,
    fields: BTreeMap<String, Field>,
}

impl Group {
    /// Create a new CDDL group.
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: BTreeMap::new(),
        }
    }

    /// Add a field to the group.
    pub fn add_field(&mut self, key: &str, field_type: Field) {
        self.fields.insert(key.to_owned(), field_type);
    }
}

impl ToString for Group {
    fn to_string(&self) -> String {
        let map = &self.fields;
        let mut cddl_str = "( ".to_string();
        for (count, value) in map.iter().enumerate() {
            // For every element except the first, add a comma
            if count != 0 {
                cddl_str += ", ";
            }
            cddl_str += &format!("{}: {}", value.0, value.1.to_string());
        }
        cddl_str += " )";
        cddl_str
    }
}

/// CddlGenerator struct for programmatically creating CDDL strings.
#[derive(Clone, Debug)]
pub struct CddlGenerator {
    name: String,
    fields: BTreeMap<String, Field>,
}

impl CddlGenerator {
    /// Create a new blank CDDL.
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: BTreeMap::new(),
        }
    }

    /// Add a field definition.
    pub fn add_operation_field(&mut self, key: String, field_type: Type) {
        // Match passed type and map it to our OperationFields type and CDDL types
        let type_string = match field_type {
            Type::Tstr => "\"str\"",
            Type::Int => "\"int\"",
            Type::Float => "\"float\"",
            Type::Bool => "\"bool\"",
            Type::Relation => "\"relation\"",
        };

        // Create an operation field group and add fields
        let mut operation_fields = Group::new(key.to_owned());
        operation_fields.add_field("type", Field::String(type_string.to_owned()));
        operation_fields.add_field("value", Field::Type(field_type));

        // Format operation fields group as a struct
        let operation_fields = Field::Struct(operation_fields);

        // Insert new operation field. If this was created from a cddl string `fields` will be None
        self.fields.insert(key, operation_fields);
    }
}

impl ToString for CddlGenerator {
    fn to_string(&self) -> String {
        let mut cddl_str = "".to_string();
        cddl_str += &format!("{} = {{ ", self.name);
        for (count, value) in self.fields.iter().enumerate() {
            if count != 0 {
                cddl_str += ", ";
            }
            cddl_str += &format!("{}: {{ {} }}", value.0, value.1.to_string());
        }
        cddl_str += " }";
        cddl_str
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{OperationFields, OperationValue};

    use super::{CddlGenerator, Type};

    pub const PERSON_CDDL: &str = r#"person = { age: { ( type: "int", value: int ) }, name: { ( type: "str", value: tstr ) } }"#;

    #[test]
    pub fn cddl_builder() {
        // Instantiate new empty CDDL named "person"
        let mut person = CddlGenerator::new("person".to_owned());

        // Add two operation fields to the CDDL
        person.add_operation_field("name".to_owned(), Type::Tstr);
        person.add_operation_field("age".to_owned(), Type::Int);

        // Create a new "person" operation
        let mut me = OperationFields::new();
        me.add("name", OperationValue::Text("Sam".to_owned()))
            .unwrap();
        me.add("age", OperationValue::Integer(35)).unwrap();

        // Validate operation fields against person CDDL
        assert_eq!(person.to_string(), PERSON_CDDL);
    }
}