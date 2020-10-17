use std::collections::HashMap;

use crate::interpreter::error;
use crate::object::BuildinFunction;
use crate::object::ChaiObject;

const NULL: ChaiObject = ChaiObject::Null;

#[derive(Clone)]
pub struct Std {
    buildinfunctions: HashMap<String, BuildinFunction>,
}

fn len(object: Vec<ChaiObject>) -> ChaiObject {
    if object.len() != 1 {
        return error(&format!("Expected 1 argument, found {}", object.len()));
    }
    match &object[0] {
        ChaiObject::String(string) => ChaiObject::Integer(string.len() as i32),
        ChaiObject::Array(array) => ChaiObject::Integer(array.len() as i32),
        _ => error(&format!("Expected String, found {}", &object[0])),
    }
}

fn push(object: Vec<ChaiObject>) -> ChaiObject {
    if object.len() != 2 {
        return error(&format!("Expected 2 argument, found {}", object.len()));
    }

    if let ChaiObject::Array(mut array) = object[0].clone() {
        array.push(object[1].clone());
        return ChaiObject::Array(array);
    }

    error("First argument should be an array")
}

fn serialize_chai_object(object: ChaiObject) -> String {
    match object {
        ChaiObject::Integer(integer) => integer.to_string(),
        ChaiObject::Boolean(boolean) => boolean.to_string(),
        ChaiObject::String(string) => string,
        ChaiObject::Array(array) => {
            let mut array_items: Vec<String> = Vec::new();
            for item in array {
                array_items.push(serialize_chai_object(item));
            }
            "[".to_string() + &array_items.join(",") + &"]".to_string()
        }
        _ => format!("{}", object),
    }
}

fn print(object: Vec<ChaiObject>) -> ChaiObject {
    if object.len() == 0 {
        return NULL;
    }

    let mut print_string: Vec<String> = Vec::new();
    for obj in object {
        print_string.push(serialize_chai_object(obj));
    }

    ChaiObject::Print(print_string.join(" "))
}

impl Std {
    pub fn load() -> Self {
        let mut buildinfunctions: HashMap<String, BuildinFunction> = HashMap::new();
        buildinfunctions.insert("len".into(), len);
        buildinfunctions.insert("push".into(), push);
        buildinfunctions.insert("print".into(), print);
        Std { buildinfunctions }
    }

    pub fn get(self, name: &str) -> Option<BuildinFunction> {
        self.buildinfunctions.get(name.into()).cloned()
    }
}
