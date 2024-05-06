#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::unit_arg)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::error;
use super::layout_schema::Layout;

#[doc = "Hardware channel ID in the Device."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Hardware channel ID in the Device.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelId(pub String);
impl std::ops::Deref for ChannelId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ChannelId> for String {
    fn from(value: ChannelId) -> Self {
        value.0
    }
}
impl From<&ChannelId> for ChannelId {
    fn from(value: &ChannelId) -> Self {
        value.clone()
    }
}
impl From<String> for ChannelId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ChannelId {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for ChannelId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Device"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"channels\","]
#[doc = "        \"dimensions\","]
#[doc = "        \"interaction_coeff_xy\","]
#[doc = "        \"is_virtual\","]
#[doc = "        \"max_atom_num\","]
#[doc = "        \"max_layout_filling\","]
#[doc = "        \"max_radial_distance\","]
#[doc = "        \"min_atom_distance\","]
#[doc = "        \"name\","]
#[doc = "        \"pre_calibrated_layouts\","]
#[doc = "        \"reusable_channels\","]
#[doc = "        \"rydberg_level\","]
#[doc = "        \"supports_slm_mask\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$schema\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channels\": {"]
#[doc = "          \"description\": \"The available channels on the device.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/PhysicalChannel\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"dimensions\": {"]
#[doc = "          \"description\": \"The maximum dimension of the supported trap arrays.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"enum\": ["]
#[doc = "            2,"]
#[doc = "            3"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dmm_objects\": {"]
#[doc = "          \"description\": \"The DMM subclass instances specifying each channel in the device.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/PhysicalDMMChannel\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"interaction_coeff_xy\": {"]
#[doc = "          \"description\": \"Coefficient setting the interaction stregth between atoms in different Rydberg states. Needed only if the device has a Microwave channel (otherwise can be null).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"is_virtual\": {"]
#[doc = "          \"description\": \"Marks the device as physical (ie non-virtual).\","]
#[doc = "          \"type\": \"boolean\","]
#[doc = "          \"const\": false"]
#[doc = "        },"]
#[doc = "        \"max_atom_num\": {"]
#[doc = "          \"description\": \"Maximum number of atoms supported.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_layout_filling\": {"]
#[doc = "          \"description\": \"The largest fraction of a layout that can be filled with atoms.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_radial_distance\": {"]
#[doc = "          \"description\": \"Maximum distance an atom can be from the center of the array (in µm).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_runs\": {"]
#[doc = "          \"description\": \"The maximum number of runs allowed on the device. Only used for backend execution.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_sequence_duration\": {"]
#[doc = "          \"description\": \"The maximum allowed duration for a sequence (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_atom_distance\": {"]
#[doc = "          \"description\": \"The closest together two atoms can be (in μm).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"A unique name for the device.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"pre_calibrated_layouts\": {"]
#[doc = "          \"description\": \"Register layouts already calibrated on the device.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Layout\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"reusable_channels\": {"]
#[doc = "          \"description\": \"Whether each channel can be declared multiple times on the same pulse sequence.\","]
#[doc = "          \"type\": \"boolean\","]
#[doc = "          \"const\": false"]
#[doc = "        },"]
#[doc = "        \"rydberg_level\": {"]
#[doc = "          \"description\": \"The principal quantum number of the used Rydberg level.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"supports_slm_mask\": {"]
#[doc = "          \"description\": \"Whether the device has an SLM mask.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"1\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"channels\","]
#[doc = "        \"dimensions\","]
#[doc = "        \"interaction_coeff_xy\","]
#[doc = "        \"is_virtual\","]
#[doc = "        \"max_atom_num\","]
#[doc = "        \"max_layout_filling\","]
#[doc = "        \"max_radial_distance\","]
#[doc = "        \"min_atom_distance\","]
#[doc = "        \"name\","]
#[doc = "        \"reusable_channels\","]
#[doc = "        \"rydberg_level\","]
#[doc = "        \"supports_slm_mask\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$schema\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channels\": {"]
#[doc = "          \"description\": \"The available channels on the device.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/GenericChannel\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"dimensions\": {"]
#[doc = "          \"description\": \"The maximum dimension of the supported trap arrays.\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"enum\": ["]
#[doc = "            2,"]
#[doc = "            3"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"dmm_objects\": {"]
#[doc = "          \"description\": \"The DMM subclass instances specifying each channel in the device.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/DMMChannel\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"interaction_coeff_xy\": {"]
#[doc = "          \"description\": \"Coefficient setting the interaction stregth between atoms in different Rydberg states. Needed only if the device has a Microwave channel (otherwise can be null).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"is_virtual\": {"]
#[doc = "          \"description\": \"Marks the device as virtual.\","]
#[doc = "          \"type\": \"boolean\","]
#[doc = "          \"const\": true"]
#[doc = "        },"]
#[doc = "        \"max_atom_num\": {"]
#[doc = "          \"description\": \"Maximum number of atoms supported.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_layout_filling\": {"]
#[doc = "          \"description\": \"The largest fraction of a layout that can be filled with atoms.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_radial_distance\": {"]
#[doc = "          \"description\": \"Maximum distance an atom can be from the center of the array (in µm).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_runs\": {"]
#[doc = "          \"description\": \"The maximum number of runs allowed on the device. Only used for backend execution.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_sequence_duration\": {"]
#[doc = "          \"description\": \"The maximum allowed duration for a sequence (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_atom_distance\": {"]
#[doc = "          \"description\": \"The closest together two atoms can be (in μm).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"A unique name for the device.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"reusable_channels\": {"]
#[doc = "          \"description\": \"Whether each channel can be declared multiple times on the same pulse sequence.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"rydberg_level\": {"]
#[doc = "          \"description\": \"The principal quantum number of the used Rydberg level.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"supports_slm_mask\": {"]
#[doc = "          \"description\": \"Whether the device has an SLM mask.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"version\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"1\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Device {
    Variant0 {
        #[doc = "The available channels on the device."]
        channels: Vec<PhysicalChannel>,
        dimensions: DeviceVariant0Dimensions,
        #[doc = "The DMM subclass instances specifying each channel in the device."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        dmm_objects: Vec<PhysicalDmmChannel>,
        #[doc = "Coefficient setting the interaction stregth between atoms in different Rydberg states. Needed only if the device has a Microwave channel (otherwise can be null)."]
        interaction_coeff_xy: Option<f64>,
        #[doc = "Marks the device as physical (ie non-virtual)."]
        is_virtual: bool,
        max_atom_num: f64,
        max_layout_filling: f64,
        max_radial_distance: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_runs: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_sequence_duration: Option<f64>,
        min_atom_distance: f64,
        #[doc = "A unique name for the device."]
        name: String,
        #[doc = "Register layouts already calibrated on the device."]
        pre_calibrated_layouts: Vec<Layout>,
        #[doc = "Whether each channel can be declared multiple times on the same pulse sequence."]
        reusable_channels: bool,
        rydberg_level: f64,
        #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
        schema: Option<String>,
        #[doc = "Whether the device has an SLM mask."]
        supports_slm_mask: bool,
        version: String,
    },
    Variant1 {
        #[doc = "The available channels on the device."]
        channels: Vec<GenericChannel>,
        dimensions: DeviceVariant1Dimensions,
        #[doc = "The DMM subclass instances specifying each channel in the device."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        dmm_objects: Vec<DmmChannel>,
        #[doc = "Coefficient setting the interaction stregth between atoms in different Rydberg states. Needed only if the device has a Microwave channel (otherwise can be null)."]
        interaction_coeff_xy: Option<f64>,
        #[doc = "Marks the device as virtual."]
        is_virtual: bool,
        #[doc = "Maximum number of atoms supported."]
        max_atom_num: Option<f64>,
        max_layout_filling: f64,
        #[doc = "Maximum distance an atom can be from the center of the array (in µm)."]
        max_radial_distance: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_runs: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_sequence_duration: Option<f64>,
        min_atom_distance: f64,
        #[doc = "A unique name for the device."]
        name: String,
        #[doc = "Whether each channel can be declared multiple times on the same pulse sequence."]
        reusable_channels: bool,
        rydberg_level: f64,
        #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
        schema: Option<String>,
        #[doc = "Whether the device has an SLM mask."]
        supports_slm_mask: bool,
        version: String,
    },
}
impl From<&Device> for Device {
    fn from(value: &Device) -> Self {
        value.clone()
    }
}
#[doc = "DeviceVariant0Dimensions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The maximum dimension of the supported trap arrays.\","]
#[doc = "  \"type\": \"number\","]
#[doc = "  \"enum\": ["]
#[doc = "    2,"]
#[doc = "    3"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Serialize)]
pub struct DeviceVariant0Dimensions(f64);
impl std::ops::Deref for DeviceVariant0Dimensions {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl From<DeviceVariant0Dimensions> for f64 {
    fn from(value: DeviceVariant0Dimensions) -> Self {
        value.0
    }
}
impl From<&DeviceVariant0Dimensions> for DeviceVariant0Dimensions {
    fn from(value: &DeviceVariant0Dimensions) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<f64> for DeviceVariant0Dimensions {
    type Error = self::error::ConversionError;
    fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
        if ![2_f64, 3_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for DeviceVariant0Dimensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "DeviceVariant1Dimensions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The maximum dimension of the supported trap arrays.\","]
#[doc = "  \"type\": \"number\","]
#[doc = "  \"enum\": ["]
#[doc = "    2,"]
#[doc = "    3"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Serialize)]
pub struct DeviceVariant1Dimensions(f64);
impl std::ops::Deref for DeviceVariant1Dimensions {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl From<DeviceVariant1Dimensions> for f64 {
    fn from(value: DeviceVariant1Dimensions) -> Self {
        value.0
    }
}
impl From<&DeviceVariant1Dimensions> for DeviceVariant1Dimensions {
    fn from(value: &DeviceVariant1Dimensions) -> Self {
        value.clone()
    }
}
impl std::convert::TryFrom<f64> for DeviceVariant1Dimensions {
    type Error = self::error::ConversionError;
    fn try_from(value: f64) -> Result<Self, self::error::ConversionError> {
        if ![2_f64, 3_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> serde::Deserialize<'de> for DeviceVariant1Dimensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A DMM channel that can be physical or virtual."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A DMM channel that can be physical or virtual.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addressing\","]
#[doc = "    \"basis\","]
#[doc = "    \"bottom_detuning\","]
#[doc = "    \"clock_period\","]
#[doc = "    \"eom_config\","]
#[doc = "    \"fixed_retarget_t\","]
#[doc = "    \"id\","]
#[doc = "    \"max_abs_detuning\","]
#[doc = "    \"max_amp\","]
#[doc = "    \"max_duration\","]
#[doc = "    \"max_targets\","]
#[doc = "    \"min_duration\","]
#[doc = "    \"min_retarget_interval\","]
#[doc = "    \"mod_bandwidth\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addressing\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"Global\""]
#[doc = "    },"]
#[doc = "    \"basis\": {"]
#[doc = "      \"description\": \"The addressed basis name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ground-rydberg\""]
#[doc = "    },"]
#[doc = "    \"bottom_detuning\": {"]
#[doc = "      \"description\": \"Minimum possible detuning per trap (in rad/µs), must be below zero.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"clock_period\": {"]
#[doc = "      \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"eom_config\": {"]
#[doc = "      \"description\": \"Configuration of an associated EOM.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"fixed_retarget_t\": {"]
#[doc = "      \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier of the channel within its device.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "    },"]
#[doc = "    \"max_abs_detuning\": {"]
#[doc = "      \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"max_amp\": {"]
#[doc = "      \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"const\": 0"]
#[doc = "    },"]
#[doc = "    \"max_duration\": {"]
#[doc = "      \"description\": \"The longest duration an instruction can take.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"max_targets\": {"]
#[doc = "      \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"min_avg_amp\": {"]
#[doc = "      \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"min_duration\": {"]
#[doc = "      \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"min_retarget_interval\": {"]
#[doc = "      \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"mod_bandwidth\": {"]
#[doc = "      \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"total_bottom_detuning\": {"]
#[doc = "      \"description\": \"Minimum possible detuning of the whole DMM channel (in rad/µs), must be below zero.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DmmChannel {
    pub addressing: String,
    #[doc = "The addressed basis name."]
    pub basis: String,
    #[doc = "Minimum possible detuning per trap (in rad/µs), must be below zero."]
    pub bottom_detuning: Option<f64>,
    pub clock_period: f64,
    #[doc = "Configuration of an associated EOM."]
    pub eom_config: (),
    #[doc = "Time taken to change the target (in ns)."]
    pub fixed_retarget_t: (),
    #[doc = "The identifier of the channel within its device."]
    pub id: ChannelId,
    #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
    pub max_abs_detuning: (),
    pub max_amp: f64,
    #[doc = "The longest duration an instruction can take."]
    pub max_duration: Option<f64>,
    #[doc = "How many atoms can be locally addressed at once by the same beam."]
    pub max_targets: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_avg_amp: Option<f64>,
    pub min_duration: f64,
    #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
    pub min_retarget_interval: (),
    #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
    pub mod_bandwidth: Option<f64>,
    #[doc = "Minimum possible detuning of the whole DMM channel (in rad/µs), must be below zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_bottom_detuning: Option<f64>,
}
impl From<&DmmChannel> for DmmChannel {
    fn from(value: &DmmChannel) -> Self {
        value.clone()
    }
}
impl DmmChannel {
    pub fn builder() -> builder::DmmChannel {
        Default::default()
    }
}
#[doc = "A Channel that can be physical or virtual."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A Channel that can be physical or virtual.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ground-rydberg\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/RydbergEOM\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"digital\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"XY\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ground-rydberg\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/RydbergEOM\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"digital\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"XY\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum GenericChannel {
    Variant0 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: Option<RydbergEom>,
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant1 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant2 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant3 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: Option<RydbergEom>,
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant4 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant5 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
        max_abs_detuning: Option<f64>,
        #[doc = "Maximum pulse amplitude (in rad/µs)."]
        max_amp: Option<f64>,
        #[doc = "The longest duration an instruction can take."]
        max_duration: Option<f64>,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
}
impl From<&GenericChannel> for GenericChannel {
    fn from(value: &GenericChannel) -> Self {
        value.clone()
    }
}

#[doc = "PhysicalChannel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ground-rydberg\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/RydbergEOM\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"digital\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Global\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"XY\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"ground-rydberg\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/RydbergEOM\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"digital\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"addressing\","]
#[doc = "        \"basis\","]
#[doc = "        \"clock_period\","]
#[doc = "        \"eom_config\","]
#[doc = "        \"fixed_retarget_t\","]
#[doc = "        \"id\","]
#[doc = "        \"max_abs_detuning\","]
#[doc = "        \"max_amp\","]
#[doc = "        \"max_duration\","]
#[doc = "        \"max_targets\","]
#[doc = "        \"min_duration\","]
#[doc = "        \"min_retarget_interval\","]
#[doc = "        \"mod_bandwidth\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"addressing\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"Local\""]
#[doc = "        },"]
#[doc = "        \"basis\": {"]
#[doc = "          \"description\": \"The addressed basis name.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"XY\""]
#[doc = "        },"]
#[doc = "        \"clock_period\": {"]
#[doc = "          \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"eom_config\": {"]
#[doc = "          \"description\": \"Configuration of an associated EOM.\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        \"fixed_retarget_t\": {"]
#[doc = "          \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"The identifier of the channel within its device.\","]
#[doc = "          \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "        },"]
#[doc = "        \"max_abs_detuning\": {"]
#[doc = "          \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_amp\": {"]
#[doc = "          \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_duration\": {"]
#[doc = "          \"description\": \"The longest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"max_targets\": {"]
#[doc = "          \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_avg_amp\": {"]
#[doc = "          \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_duration\": {"]
#[doc = "          \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"min_retarget_interval\": {"]
#[doc = "          \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"mod_bandwidth\": {"]
#[doc = "          \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum PhysicalChannel {
    Variant0 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: Option<RydbergEom>,
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant1 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant2 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        #[doc = "Time taken to change the target (in ns)."]
        fixed_retarget_t: (),
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        #[doc = "How many atoms can be locally addressed at once by the same beam."]
        max_targets: (),
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
        min_retarget_interval: (),
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant3 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: Option<RydbergEom>,
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        max_targets: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant4 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        max_targets: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
    Variant5 {
        addressing: String,
        #[doc = "The addressed basis name."]
        basis: String,
        clock_period: f64,
        #[doc = "Configuration of an associated EOM."]
        eom_config: (),
        fixed_retarget_t: f64,
        #[doc = "The identifier of the channel within its device."]
        id: ChannelId,
        max_abs_detuning: f64,
        max_amp: f64,
        max_duration: f64,
        max_targets: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min_avg_amp: Option<f64>,
        min_duration: f64,
        min_retarget_interval: f64,
        #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
        mod_bandwidth: Option<f64>,
    },
}
impl From<&PhysicalChannel> for PhysicalChannel {
    fn from(value: &PhysicalChannel) -> Self {
        value.clone()
    }
}
#[doc = "PhysicalDmmChannel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addressing\","]
#[doc = "    \"basis\","]
#[doc = "    \"bottom_detuning\","]
#[doc = "    \"clock_period\","]
#[doc = "    \"eom_config\","]
#[doc = "    \"fixed_retarget_t\","]
#[doc = "    \"id\","]
#[doc = "    \"max_abs_detuning\","]
#[doc = "    \"max_amp\","]
#[doc = "    \"max_duration\","]
#[doc = "    \"max_targets\","]
#[doc = "    \"min_duration\","]
#[doc = "    \"min_retarget_interval\","]
#[doc = "    \"mod_bandwidth\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addressing\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"Global\""]
#[doc = "    },"]
#[doc = "    \"basis\": {"]
#[doc = "      \"description\": \"The addressed basis name.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ground-rydberg\""]
#[doc = "    },"]
#[doc = "    \"bottom_detuning\": {"]
#[doc = "      \"description\": \"Minimum possible detuning per trap (in rad/µs), must be below zero.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"clock_period\": {"]
#[doc = "      \"description\": \"The duration of a clock cycle (in ns).\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"eom_config\": {"]
#[doc = "      \"description\": \"Configuration of an associated EOM.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"fixed_retarget_t\": {"]
#[doc = "      \"description\": \"Time taken to change the target (in ns).\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier of the channel within its device.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "    },"]
#[doc = "    \"max_abs_detuning\": {"]
#[doc = "      \"description\": \"Maximum possible detuning (in rad/µs), in absolute value.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"max_amp\": {"]
#[doc = "      \"description\": \"Maximum pulse amplitude (in rad/µs).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"const\": 0"]
#[doc = "    },"]
#[doc = "    \"max_duration\": {"]
#[doc = "      \"description\": \"The longest duration an instruction can take.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"max_targets\": {"]
#[doc = "      \"description\": \"How many atoms can be locally addressed at once by the same beam.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"min_avg_amp\": {"]
#[doc = "      \"description\": \"The minimum average amplitude of a pulse (when not zero).\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"min_duration\": {"]
#[doc = "      \"description\": \"The shortest duration an instruction can take.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"min_retarget_interval\": {"]
#[doc = "      \"description\": \"Minimum time required between the ends of two target instructions (in ns).\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    \"mod_bandwidth\": {"]
#[doc = "      \"description\": \"The modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"total_bottom_detuning\": {"]
#[doc = "      \"description\": \"Minimum possible detuning of the whole DMM channel (in rad/µs), must be below zero.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalDmmChannel {
    pub addressing: String,
    #[doc = "The addressed basis name."]
    pub basis: String,
    pub bottom_detuning: f64,
    pub clock_period: f64,
    #[doc = "Configuration of an associated EOM."]
    pub eom_config: (),
    #[doc = "Time taken to change the target (in ns)."]
    pub fixed_retarget_t: (),
    #[doc = "The identifier of the channel within its device."]
    pub id: ChannelId,
    #[doc = "Maximum possible detuning (in rad/µs), in absolute value."]
    pub max_abs_detuning: (),
    pub max_amp: f64,
    pub max_duration: f64,
    #[doc = "How many atoms can be locally addressed at once by the same beam."]
    pub max_targets: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_avg_amp: Option<f64>,
    pub min_duration: f64,
    #[doc = "Minimum time required between the ends of two target instructions (in ns)."]
    pub min_retarget_interval: (),
    #[doc = "The modulation bandwidth at -3dB (50% reduction), in MHz."]
    pub mod_bandwidth: Option<f64>,
    #[doc = "Minimum possible detuning of the whole DMM channel (in rad/µs), must be below zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_bottom_detuning: Option<f64>,
}
impl From<&PhysicalDmmChannel> for PhysicalDmmChannel {
    fn from(value: &PhysicalDmmChannel) -> Self {
        value.clone()
    }
}
impl PhysicalDmmChannel {
    pub fn builder() -> builder::PhysicalDmmChannel {
        Default::default()
    }
}
#[doc = "RydbergBeam"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"RED\","]
#[doc = "    \"BLUE\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RydbergBeam {
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "BLUE")]
    Blue,
}
impl From<&RydbergBeam> for RydbergBeam {
    fn from(value: &RydbergBeam) -> Self {
        value.clone()
    }
}
impl ToString for RydbergBeam {
    fn to_string(&self) -> String {
        match *self {
            Self::Red => "RED".to_string(),
            Self::Blue => "BLUE".to_string(),
        }
    }
}
impl std::str::FromStr for RydbergBeam {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "RED" => Ok(Self::Red),
            "BLUE" => Ok(Self::Blue),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RydbergBeam {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RydbergBeam {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RydbergBeam {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "RydbergEom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"controlled_beams\","]
#[doc = "    \"intermediate_detuning\","]
#[doc = "    \"limiting_beam\","]
#[doc = "    \"max_limiting_amp\","]
#[doc = "    \"mod_bandwidth\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"controlled_beams\": {"]
#[doc = "      \"description\": \"The beams that can be switched on/off with an EOM.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/RydbergBeam\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"custom_buffer_time\": {"]
#[doc = "      \"description\": \"A custom wait time to enforce during EOM buffers.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"intermediate_detuning\": {"]
#[doc = "      \"description\": \"The detuning between the two beams, in rad/µs.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"limiting_beam\": {"]
#[doc = "      \"description\": \"The beam with the smallest amplitude range.\","]
#[doc = "      \"$ref\": \"#/definitions/RydbergBeam\""]
#[doc = "    },"]
#[doc = "    \"max_limiting_amp\": {"]
#[doc = "      \"description\": \"The maximum amplitude the limiting beam can reach, in rad/µs.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"mod_bandwidth\": {"]
#[doc = "      \"description\": \"The EOM modulation bandwidth at -3dB (50% reduction), in MHz.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"multiple_beam_control\": {"]
#[doc = "      \"description\": \"Whether both EOMs can be used simultaneously or not.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RydbergEom {
    #[doc = "The beams that can be switched on/off with an EOM."]
    pub controlled_beams: Vec<RydbergBeam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_buffer_time: Option<f64>,
    pub intermediate_detuning: f64,
    #[doc = "The beam with the smallest amplitude range."]
    pub limiting_beam: RydbergBeam,
    pub max_limiting_amp: f64,
    pub mod_bandwidth: f64,
    #[doc = "Whether both EOMs can be used simultaneously or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_beam_control: Option<bool>,
}
impl From<&RydbergEom> for RydbergEom {
    fn from(value: &RydbergEom) -> Self {
        value.clone()
    }
}
impl RydbergEom {
    pub fn builder() -> builder::RydbergEom {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DmmChannel {
        addressing: Result<String, String>,
        basis: Result<String, String>,
        bottom_detuning: Result<Option<f64>, String>,
        clock_period: Result<f64, String>,
        eom_config: Result<(), String>,
        fixed_retarget_t: Result<(), String>,
        id: Result<super::ChannelId, String>,
        max_abs_detuning: Result<(), String>,
        max_amp: Result<f64, String>,
        max_duration: Result<Option<f64>, String>,
        max_targets: Result<(), String>,
        min_avg_amp: Result<Option<f64>, String>,
        min_duration: Result<f64, String>,
        min_retarget_interval: Result<(), String>,
        mod_bandwidth: Result<Option<f64>, String>,
        total_bottom_detuning: Result<Option<f64>, String>,
    }
    impl Default for DmmChannel {
        fn default() -> Self {
            Self {
                addressing: Err("no value supplied for addressing".to_string()),
                basis: Err("no value supplied for basis".to_string()),
                bottom_detuning: Err("no value supplied for bottom_detuning".to_string()),
                clock_period: Err("no value supplied for clock_period".to_string()),
                eom_config: Err("no value supplied for eom_config".to_string()),
                fixed_retarget_t: Err("no value supplied for fixed_retarget_t".to_string()),
                id: Err("no value supplied for id".to_string()),
                max_abs_detuning: Err("no value supplied for max_abs_detuning".to_string()),
                max_amp: Err("no value supplied for max_amp".to_string()),
                max_duration: Err("no value supplied for max_duration".to_string()),
                max_targets: Err("no value supplied for max_targets".to_string()),
                min_avg_amp: Ok(Default::default()),
                min_duration: Err("no value supplied for min_duration".to_string()),
                min_retarget_interval: Err(
                    "no value supplied for min_retarget_interval".to_string()
                ),
                mod_bandwidth: Err("no value supplied for mod_bandwidth".to_string()),
                total_bottom_detuning: Ok(Default::default()),
            }
        }
    }
    impl DmmChannel {
        pub fn addressing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.addressing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addressing: {}", e));
            self
        }
        pub fn basis<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.basis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for basis: {}", e));
            self
        }
        pub fn bottom_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.bottom_detuning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bottom_detuning: {}", e));
            self
        }
        pub fn clock_period<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.clock_period = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clock_period: {}", e));
            self
        }
        pub fn eom_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.eom_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eom_config: {}", e));
            self
        }
        pub fn fixed_retarget_t<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.fixed_retarget_t = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_retarget_t: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn max_abs_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.max_abs_detuning = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_abs_detuning: {}",
                    e
                )
            });
            self
        }
        pub fn max_amp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.max_amp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_amp: {}", e));
            self
        }
        pub fn max_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.max_duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_duration: {}", e));
            self
        }
        pub fn max_targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.max_targets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_targets: {}", e));
            self
        }
        pub fn min_avg_amp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.min_avg_amp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_avg_amp: {}", e));
            self
        }
        pub fn min_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.min_duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_duration: {}", e));
            self
        }
        pub fn min_retarget_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.min_retarget_interval = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for min_retarget_interval: {}",
                    e
                )
            });
            self
        }
        pub fn mod_bandwidth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.mod_bandwidth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_bandwidth: {}", e));
            self
        }
        pub fn total_bottom_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.total_bottom_detuning = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for total_bottom_detuning: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<DmmChannel> for super::DmmChannel {
        type Error = super::error::ConversionError;
        fn try_from(value: DmmChannel) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                addressing: value.addressing?,
                basis: value.basis?,
                bottom_detuning: value.bottom_detuning?,
                clock_period: value.clock_period?,
                eom_config: value.eom_config?,
                fixed_retarget_t: value.fixed_retarget_t?,
                id: value.id?,
                max_abs_detuning: value.max_abs_detuning?,
                max_amp: value.max_amp?,
                max_duration: value.max_duration?,
                max_targets: value.max_targets?,
                min_avg_amp: value.min_avg_amp?,
                min_duration: value.min_duration?,
                min_retarget_interval: value.min_retarget_interval?,
                mod_bandwidth: value.mod_bandwidth?,
                total_bottom_detuning: value.total_bottom_detuning?,
            })
        }
    }
    impl From<super::DmmChannel> for DmmChannel {
        fn from(value: super::DmmChannel) -> Self {
            Self {
                addressing: Ok(value.addressing),
                basis: Ok(value.basis),
                bottom_detuning: Ok(value.bottom_detuning),
                clock_period: Ok(value.clock_period),
                eom_config: Ok(value.eom_config),
                fixed_retarget_t: Ok(value.fixed_retarget_t),
                id: Ok(value.id),
                max_abs_detuning: Ok(value.max_abs_detuning),
                max_amp: Ok(value.max_amp),
                max_duration: Ok(value.max_duration),
                max_targets: Ok(value.max_targets),
                min_avg_amp: Ok(value.min_avg_amp),
                min_duration: Ok(value.min_duration),
                min_retarget_interval: Ok(value.min_retarget_interval),
                mod_bandwidth: Ok(value.mod_bandwidth),
                total_bottom_detuning: Ok(value.total_bottom_detuning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PhysicalDmmChannel {
        addressing: Result<String, String>,
        basis: Result<String, String>,
        bottom_detuning: Result<f64, String>,
        clock_period: Result<f64, String>,
        eom_config: Result<(), String>,
        fixed_retarget_t: Result<(), String>,
        id: Result<super::ChannelId, String>,
        max_abs_detuning: Result<(), String>,
        max_amp: Result<f64, String>,
        max_duration: Result<f64, String>,
        max_targets: Result<(), String>,
        min_avg_amp: Result<Option<f64>, String>,
        min_duration: Result<f64, String>,
        min_retarget_interval: Result<(), String>,
        mod_bandwidth: Result<Option<f64>, String>,
        total_bottom_detuning: Result<Option<f64>, String>,
    }
    impl Default for PhysicalDmmChannel {
        fn default() -> Self {
            Self {
                addressing: Err("no value supplied for addressing".to_string()),
                basis: Err("no value supplied for basis".to_string()),
                bottom_detuning: Err("no value supplied for bottom_detuning".to_string()),
                clock_period: Err("no value supplied for clock_period".to_string()),
                eom_config: Err("no value supplied for eom_config".to_string()),
                fixed_retarget_t: Err("no value supplied for fixed_retarget_t".to_string()),
                id: Err("no value supplied for id".to_string()),
                max_abs_detuning: Err("no value supplied for max_abs_detuning".to_string()),
                max_amp: Err("no value supplied for max_amp".to_string()),
                max_duration: Err("no value supplied for max_duration".to_string()),
                max_targets: Err("no value supplied for max_targets".to_string()),
                min_avg_amp: Ok(Default::default()),
                min_duration: Err("no value supplied for min_duration".to_string()),
                min_retarget_interval: Err(
                    "no value supplied for min_retarget_interval".to_string()
                ),
                mod_bandwidth: Err("no value supplied for mod_bandwidth".to_string()),
                total_bottom_detuning: Ok(Default::default()),
            }
        }
    }
    impl PhysicalDmmChannel {
        pub fn addressing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.addressing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for addressing: {}", e));
            self
        }
        pub fn basis<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.basis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for basis: {}", e));
            self
        }
        pub fn bottom_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.bottom_detuning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bottom_detuning: {}", e));
            self
        }
        pub fn clock_period<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.clock_period = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for clock_period: {}", e));
            self
        }
        pub fn eom_config<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.eom_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eom_config: {}", e));
            self
        }
        pub fn fixed_retarget_t<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.fixed_retarget_t = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for fixed_retarget_t: {}",
                    e
                )
            });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn max_abs_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.max_abs_detuning = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_abs_detuning: {}",
                    e
                )
            });
            self
        }
        pub fn max_amp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.max_amp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_amp: {}", e));
            self
        }
        pub fn max_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.max_duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_duration: {}", e));
            self
        }
        pub fn max_targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.max_targets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_targets: {}", e));
            self
        }
        pub fn min_avg_amp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.min_avg_amp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_avg_amp: {}", e));
            self
        }
        pub fn min_duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.min_duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_duration: {}", e));
            self
        }
        pub fn min_retarget_interval<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<()>,
            T::Error: std::fmt::Display,
        {
            self.min_retarget_interval = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for min_retarget_interval: {}",
                    e
                )
            });
            self
        }
        pub fn mod_bandwidth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.mod_bandwidth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_bandwidth: {}", e));
            self
        }
        pub fn total_bottom_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.total_bottom_detuning = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for total_bottom_detuning: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<PhysicalDmmChannel> for super::PhysicalDmmChannel {
        type Error = super::error::ConversionError;
        fn try_from(value: PhysicalDmmChannel) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                addressing: value.addressing?,
                basis: value.basis?,
                bottom_detuning: value.bottom_detuning?,
                clock_period: value.clock_period?,
                eom_config: value.eom_config?,
                fixed_retarget_t: value.fixed_retarget_t?,
                id: value.id?,
                max_abs_detuning: value.max_abs_detuning?,
                max_amp: value.max_amp?,
                max_duration: value.max_duration?,
                max_targets: value.max_targets?,
                min_avg_amp: value.min_avg_amp?,
                min_duration: value.min_duration?,
                min_retarget_interval: value.min_retarget_interval?,
                mod_bandwidth: value.mod_bandwidth?,
                total_bottom_detuning: value.total_bottom_detuning?,
            })
        }
    }
    impl From<super::PhysicalDmmChannel> for PhysicalDmmChannel {
        fn from(value: super::PhysicalDmmChannel) -> Self {
            Self {
                addressing: Ok(value.addressing),
                basis: Ok(value.basis),
                bottom_detuning: Ok(value.bottom_detuning),
                clock_period: Ok(value.clock_period),
                eom_config: Ok(value.eom_config),
                fixed_retarget_t: Ok(value.fixed_retarget_t),
                id: Ok(value.id),
                max_abs_detuning: Ok(value.max_abs_detuning),
                max_amp: Ok(value.max_amp),
                max_duration: Ok(value.max_duration),
                max_targets: Ok(value.max_targets),
                min_avg_amp: Ok(value.min_avg_amp),
                min_duration: Ok(value.min_duration),
                min_retarget_interval: Ok(value.min_retarget_interval),
                mod_bandwidth: Ok(value.mod_bandwidth),
                total_bottom_detuning: Ok(value.total_bottom_detuning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RydbergEom {
        controlled_beams: Result<Vec<super::RydbergBeam>, String>,
        custom_buffer_time: Result<Option<f64>, String>,
        intermediate_detuning: Result<f64, String>,
        limiting_beam: Result<super::RydbergBeam, String>,
        max_limiting_amp: Result<f64, String>,
        mod_bandwidth: Result<f64, String>,
        multiple_beam_control: Result<Option<bool>, String>,
    }
    impl Default for RydbergEom {
        fn default() -> Self {
            Self {
                controlled_beams: Err("no value supplied for controlled_beams".to_string()),
                custom_buffer_time: Ok(Default::default()),
                intermediate_detuning: Err(
                    "no value supplied for intermediate_detuning".to_string()
                ),
                limiting_beam: Err("no value supplied for limiting_beam".to_string()),
                max_limiting_amp: Err("no value supplied for max_limiting_amp".to_string()),
                mod_bandwidth: Err("no value supplied for mod_bandwidth".to_string()),
                multiple_beam_control: Ok(Default::default()),
            }
        }
    }
    impl RydbergEom {
        pub fn controlled_beams<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::RydbergBeam>>,
            T::Error: std::fmt::Display,
        {
            self.controlled_beams = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for controlled_beams: {}",
                    e
                )
            });
            self
        }
        pub fn custom_buffer_time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.custom_buffer_time = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for custom_buffer_time: {}",
                    e
                )
            });
            self
        }
        pub fn intermediate_detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.intermediate_detuning = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for intermediate_detuning: {}",
                    e
                )
            });
            self
        }
        pub fn limiting_beam<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RydbergBeam>,
            T::Error: std::fmt::Display,
        {
            self.limiting_beam = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limiting_beam: {}", e));
            self
        }
        pub fn max_limiting_amp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.max_limiting_amp = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_limiting_amp: {}",
                    e
                )
            });
            self
        }
        pub fn mod_bandwidth<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.mod_bandwidth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_bandwidth: {}", e));
            self
        }
        pub fn multiple_beam_control<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.multiple_beam_control = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for multiple_beam_control: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<RydbergEom> for super::RydbergEom {
        type Error = super::error::ConversionError;
        fn try_from(value: RydbergEom) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                controlled_beams: value.controlled_beams?,
                custom_buffer_time: value.custom_buffer_time?,
                intermediate_detuning: value.intermediate_detuning?,
                limiting_beam: value.limiting_beam?,
                max_limiting_amp: value.max_limiting_amp?,
                mod_bandwidth: value.mod_bandwidth?,
                multiple_beam_control: value.multiple_beam_control?,
            })
        }
    }
    impl From<super::RydbergEom> for RydbergEom {
        fn from(value: super::RydbergEom) -> Self {
            Self {
                controlled_beams: Ok(value.controlled_beams),
                custom_buffer_time: Ok(value.custom_buffer_time),
                intermediate_detuning: Ok(value.intermediate_detuning),
                limiting_beam: Ok(value.limiting_beam),
                max_limiting_amp: Ok(value.max_limiting_amp),
                mod_bandwidth: Ok(value.mod_bandwidth),
                multiple_beam_control: Ok(value.multiple_beam_control),
            }
        }
    }
}
