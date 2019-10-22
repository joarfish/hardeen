//! # Parameters
//!
//! Each ProcessorComponent has a different set of parameters. The kind of parameters is restricted
//! to a number of `ParameterTypes`. All parameters be set from a &str and converted to a String. This
//! allows for a common interface to setting and getting parameters.

use serde::Serialize;
use std::string::ToString;
use std::str::FromStr;
use std::num::ParseFloatError;
use std::fmt;
use std::vec::Vec;

use crate::geometry::Position;

#[derive(Clone, Serialize)]
pub struct ProcessorParameter {
    pub param_type: &'static str,
    pub param_name: &'static str
}


#[derive(Clone, Serialize)]
pub struct PositionList(pub Vec<Position>);

impl FromStr for PositionList {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords : Vec<&str> = s.trim().split(';').collect();
        let mut result : Vec<Position> = Vec::new();

        for i in 0..coords.len() {
            let position = Position::from_str(coords[i])?;
            result.push(position);
        }

        Ok(PositionList(result))
    }
}

impl fmt::Display for PositionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for p in self.0.iter() {
            result += &format!("{},{};", p.0, p.1);
        }
        write!(f, "{}", &result)
    }
}

#[derive(Serialize, Clone)]
pub enum ParameterType {
    Integer,
    UnsignedInteger,
    Float,
    Boolean,
    Position,
    String,
    PositionList
}

#[derive(Serialize, Clone)]
pub enum Parameter {
    Integer(i32),
    UnsignedInteger(u32),
    Float(f32),
    Boolean(bool),
    Position(f32, f32),
    String(String),
    PositionList(PositionList)
}

impl Parameter {

    pub fn set(&mut self, value: &str) {
        match self {
            Parameter::Integer(i) => *i = value.parse::<i32>().unwrap(),
            Parameter::Float(f) => *f = value.parse::<f32>().unwrap(),
            _ => panic!("")
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Parameter::Integer(_) => "Integer",
            Parameter::Float(_) => "Float",
            _ => panic!("")
        }
    }

    pub fn from_string(value: &str, parameter_type: &str) -> Self {
        match parameter_type {
            "Integer" => {
                Parameter::Integer(value.parse::<i32>().unwrap())
            },
            "Unsigned" => {
                Parameter::UnsignedInteger(value.parse::<u32>().unwrap())
            },
            "Float" => {
                Parameter::Float(value.parse::<f32>().unwrap())
            },
            "Boolean" => {
                Parameter::Boolean(match value {
                    "true" => true,
                    "false" => false,
                    &_ => panic!("Invalid Parameter Value!")
                })
            },
            &_ => {
                panic!("Invalid Parameter Type!")
            }
        }
    }
}



impl ToString for Parameter {
    fn to_string(&self) -> String {
        match self {
            Parameter::Integer(i) => i.to_string(),
            Parameter::UnsignedInteger(u) => u.to_string(),
            Parameter::Float(f) => f.to_string(),
            Parameter::Boolean(b) => String::from(if *b { "true" } else { "false" }),
            Parameter::Position(x, y) => format!("({},{})", x, y),
            Parameter::PositionList(list) => format!("{}", list),
            Parameter::String(string) => string.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float() {
        let mut m = Parameter::Float(10.0);
        m.set("20.0");
        m.to_string();

        if let Parameter::Float(f) = m {
            assert_eq!(f, 20.0);
        }
        else {
            assert!(false);
        }
    }
}


