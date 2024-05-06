#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::unit_arg)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::error;
use super::layout_schema::Layout;

#[doc = "Atom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"x\","]
#[doc = "    \"y\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Name of the atom.\","]
#[doc = "      \"$ref\": \"#/definitions/QubitId\""]
#[doc = "    },"]
#[doc = "    \"x\": {"]
#[doc = "      \"description\": \"x-position in µm\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"y\": {"]
#[doc = "      \"description\": \"y-position in µm\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Atom {
    #[doc = "Name of the atom."]
    pub name: QubitId,
    pub x: f64,
    pub y: f64,
}
impl From<&Atom> for Atom {
    fn from(value: &Atom) -> Self {
        value.clone()
    }
}
impl Atom {
    pub fn builder() -> builder::Atom {
        Default::default()
    }
}

#[doc = "Name for a qubit."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name for a qubit.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct QubitId(pub String);
impl std::ops::Deref for QubitId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<QubitId> for String {
    fn from(value: QubitId) -> Self {
        value.0
    }
}
impl From<&QubitId> for QubitId {
    fn from(value: &QubitId) -> Self {
        value.clone()
    }
}
impl From<String> for QubitId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for QubitId {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for QubitId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Register"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"register\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"layout\": {"]
#[doc = "      \"description\": \"The trap layout underlying the register.\","]
#[doc = "      \"$ref\": \"#/definitions/Layout\""]
#[doc = "    },"]
#[doc = "    \"register\": {"]
#[doc = "      \"description\": \"A 2D register containing a set of atoms.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Atom\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Register {
    #[doc = "The trap layout underlying the register."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[doc = "A 2D register containing a set of atoms."]
    pub register: Vec<Atom>,
}
impl From<&Register> for Register {
    fn from(value: &Register) -> Self {
        value.clone()
    }
}
impl Register {
    pub fn builder() -> builder::Register {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Atom {
        name: Result<super::QubitId, String>,
        x: Result<f64, String>,
        y: Result<f64, String>,
    }
    impl Default for Atom {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl Atom {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::QubitId>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Atom> for super::Atom {
        type Error = super::error::ConversionError;
        fn try_from(value: Atom) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::Atom> for Atom {
        fn from(value: super::Atom) -> Self {
            Self {
                name: Ok(value.name),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Register {
        layout: Result<Option<super::Layout>, String>,
        register: Result<Vec<super::Atom>, String>,
    }
    impl Default for Register {
        fn default() -> Self {
            Self {
                layout: Ok(Default::default()),
                register: Err("no value supplied for register".to_string()),
            }
        }
    }
    impl Register {
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Layout>>,
            T::Error: std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn register<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Atom>>,
            T::Error: std::fmt::Display,
        {
            self.register = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for register: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Register> for super::Register {
        type Error = super::error::ConversionError;
        fn try_from(value: Register) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                layout: value.layout?,
                register: value.register?,
            })
        }
    }
    impl From<super::Register> for Register {
        fn from(value: super::Register) -> Self {
            Self {
                layout: Ok(value.layout),
                register: Ok(value.register),
            }
        }
    }
}
