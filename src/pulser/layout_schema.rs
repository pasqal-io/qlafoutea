#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::unit_arg)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::pulser::error;

#[doc = "Layout with the positions of the traps. A selection of up to 50% of these traps makes up the Register."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Layout with the positions of the traps. A selection of up to 50% of these traps makes up the Register.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"coordinates\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"coordinates\": {"]
#[doc = "      \"description\": \"The trap coordinates in µm.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"array\","]
#[doc = "        \"items\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"maxItems\": 2,"]
#[doc = "        \"minItems\": 2"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"slug\": {"]
#[doc = "      \"description\": \"An optional name for the layout.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    #[doc = "The trap coordinates in µm."]
    pub coordinates: Vec<[f64; 2usize]>,
    #[doc = "An optional name for the layout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}
impl From<&Layout> for Layout {
    fn from(value: &Layout) -> Self {
        value.clone()
    }
}
impl Layout {
    pub fn builder() -> builder::Layout {
        Default::default()
    }
}

pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Layout {
        coordinates: Result<Vec<[f64; 2usize]>, String>,
        slug: Result<Option<String>, String>,
    }
    impl Default for Layout {
        fn default() -> Self {
            Self {
                coordinates: Err("no value supplied for coordinates".to_string()),
                slug: Ok(Default::default()),
            }
        }
    }
    impl Layout {
        pub fn coordinates<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<[f64; 2usize]>>,
            T::Error: std::fmt::Display,
        {
            self.coordinates = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for coordinates: {}", e));
            self
        }
        pub fn slug<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.slug = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for slug: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Layout> for super::Layout {
        type Error = super::error::ConversionError;
        fn try_from(value: Layout) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                coordinates: value.coordinates?,
                slug: value.slug?,
            })
        }
    }
    impl From<super::Layout> for Layout {
        fn from(value: super::Layout) -> Self {
            Self {
                coordinates: Ok(value.coordinates),
                slug: Ok(value.slug),
            }
        }
    }
}