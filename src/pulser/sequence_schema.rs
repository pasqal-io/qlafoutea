#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::unit_arg)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::device_schema::Device;
use super::layout_schema::Layout;
use super::register_schema::Atom;
use super::register_schema::QubitId;

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "The two-level-system basis addressable by a given channel."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The two-level-system basis addressable by a given channel.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ground-rydberg\","]
#[doc = "    \"digital\","]
#[doc = "    \"XY\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Basis {
    #[serde(rename = "ground-rydberg")]
    GroundRydberg,
    #[serde(rename = "digital")]
    Digital,
    #[serde(rename = "XY")]
    Xy,
}
impl From<&Basis> for Basis {
    fn from(value: &Basis) -> Self {
        value.clone()
    }
}
impl ToString for Basis {
    fn to_string(&self) -> String {
        match *self {
            Self::GroundRydberg => "ground-rydberg".to_string(),
            Self::Digital => "digital".to_string(),
            Self::Xy => "XY".to_string(),
        }
    }
}
impl std::str::FromStr for Basis {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ground-rydberg" => Ok(Self::GroundRydberg),
            "digital" => Ok(Self::Digital),
            "XY" => Ok(Self::Xy),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for Basis {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Basis {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Basis {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A Blackman window of a specified max value and area."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A Blackman window of a specified max value and area.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"kind\","]
#[doc = "    \"max_val\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"blackman_max\""]
#[doc = "    },"]
#[doc = "    \"max_val\": {"]
#[doc = "      \"description\": \"The waveform peak value.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlackmanMaxWaveform {
    #[doc = "The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values."]
    pub area: ParametrizedNum,
    pub kind: String,
    #[doc = "The waveform peak value."]
    pub max_val: ParametrizedNum,
}
impl From<&BlackmanMaxWaveform> for BlackmanMaxWaveform {
    fn from(value: &BlackmanMaxWaveform) -> Self {
        value.clone()
    }
}
impl BlackmanMaxWaveform {
    pub fn builder() -> builder::BlackmanMaxWaveform {
        Default::default()
    }
}
#[doc = "A Blackman window of a specified duration and area."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A Blackman window of a specified duration and area.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"duration\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The waveform duration (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"blackman\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlackmanWaveform {
    #[doc = "The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values."]
    pub area: ParametrizedNum,
    #[doc = "The waveform duration (in ns)."]
    pub duration: ParametrizedNum,
    pub kind: String,
}
impl From<&BlackmanWaveform> for BlackmanWaveform {
    fn from(value: &BlackmanWaveform) -> Self {
        value.clone()
    }
}
impl BlackmanWaveform {
    pub fn builder() -> builder::BlackmanWaveform {
        Default::default()
    }
}
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
#[doc = "Name of declared channel."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of declared channel.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelName(pub String);
impl std::ops::Deref for ChannelName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ChannelName> for String {
    fn from(value: ChannelName) -> Self {
        value.0
    }
}
impl From<&ChannelName> for ChannelName {
    fn from(value: &ChannelName) -> Self {
        value.clone()
    }
}
impl From<String> for ChannelName {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for ChannelName {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for ChannelName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "CompositeWaveform"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"waveforms\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"composite\""]
#[doc = "    },"]
#[doc = "    \"waveforms\": {"]
#[doc = "      \"description\": \"List of waveforms to compose one after another, in specified order.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Waveform\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompositeWaveform {
    pub kind: String,
    #[doc = "List of waveforms to compose one after another, in specified order."]
    pub waveforms: Vec<Waveform>,
}
impl From<&CompositeWaveform> for CompositeWaveform {
    fn from(value: &CompositeWaveform) -> Self {
        value.clone()
    }
}
impl CompositeWaveform {
    pub fn builder() -> builder::CompositeWaveform {
        Default::default()
    }
}
#[doc = "A waveform of constant value."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A waveform of constant value.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"duration\","]
#[doc = "    \"kind\","]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The waveform duration (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"constant\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"The constant modulation value (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstantWaveform {
    #[doc = "The waveform duration (in ns)."]
    pub duration: ParametrizedNum,
    pub kind: String,
    #[doc = "The constant modulation value (in rad/µs)."]
    pub value: ParametrizedNum,
}
impl From<&ConstantWaveform> for ConstantWaveform {
    fn from(value: &ConstantWaveform) -> Self {
        value.clone()
    }
}
impl ConstantWaveform {
    pub fn builder() -> builder::ConstantWaveform {
        Default::default()
    }
}
#[doc = "CustomWaveform"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"samples\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"custom\""]
#[doc = "    },"]
#[doc = "    \"samples\": {"]
#[doc = "      \"description\": \"List of waveform value samples, one per timestep.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CustomWaveform {
    pub kind: String,
    #[doc = "List of waveform value samples, one per timestep."]
    pub samples: Vec<f64>,
}
impl From<&CustomWaveform> for CustomWaveform {
    fn from(value: &CustomWaveform) -> Self {
        value.clone()
    }
}
impl CustomWaveform {
    pub fn builder() -> builder::CustomWaveform {
        Default::default()
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
#[doc = "Expression argument"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expression argument\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableRef\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExprBinary\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExprUnary\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ExprArgument {
    Variant0(f64),
    Variant1(Vec<f64>),
    Variant2(VariableRef),
    Variant3(ExprBinary),
    Variant4(Box<ExprUnary>),
}
impl From<&ExprArgument> for ExprArgument {
    fn from(value: &ExprArgument) -> Self {
        value.clone()
    }
}
impl From<f64> for ExprArgument {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<f64>> for ExprArgument {
    fn from(value: Vec<f64>) -> Self {
        Self::Variant1(value)
    }
}
impl From<VariableRef> for ExprArgument {
    fn from(value: VariableRef) -> Self {
        Self::Variant2(value)
    }
}
impl From<ExprBinary> for ExprArgument {
    fn from(value: ExprBinary) -> Self {
        Self::Variant3(value)
    }
}
impl From<Box<ExprUnary>> for ExprArgument {
    fn from(value: Box<ExprUnary>) -> Self {
        Self::Variant4(value)
    }
}
#[doc = "Simple binary expression involving variables and constants.\n\nThe array access behaviour depends on expression:\n- index:   - the lhs array is indexed using rhs indices, resulting in an array of the same length as rhs.   - out of bounds indexing is a runtime error   - NOTE: Pulser only supports variable references on lhs of index expression.           This limitation might be lifted in the future.\n- everything else:   - the expression is applied element-wise   - operating on arrays of different lengths is a runtime error"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Simple binary expression involving variables and constants.\\n\\nThe array access behaviour depends on expression:\\n- index:   - the lhs array is indexed using rhs indices, resulting in an array of the same length as rhs.   - out of bounds indexing is a runtime error   - NOTE: Pulser only supports variable references on lhs of index expression.           This limitation might be lifted in the future.\\n- everything else:   - the expression is applied element-wise   - operating on arrays of different lengths is a runtime error\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expression\","]
#[doc = "    \"lhs\","]
#[doc = "    \"rhs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expression\": {"]
#[doc = "      \"description\": \"Expresion operation\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"add\","]
#[doc = "        \"sub\","]
#[doc = "        \"mul\","]
#[doc = "        \"div\","]
#[doc = "        \"mod\","]
#[doc = "        \"pow\","]
#[doc = "        \"index\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lhs\": {"]
#[doc = "      \"description\": \"Left-hand side of an operation\","]
#[doc = "      \"$ref\": \"#/definitions/ExprArgument\""]
#[doc = "    },"]
#[doc = "    \"rhs\": {"]
#[doc = "      \"description\": \"Right-hand side of an operation\","]
#[doc = "      \"$ref\": \"#/definitions/ExprArgument\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExprBinary {
    #[doc = "Expresion operation"]
    pub expression: ExprBinaryExpression,
    #[doc = "Left-hand side of an operation"]
    pub lhs: Box<ExprArgument>,
    #[doc = "Right-hand side of an operation"]
    pub rhs: Box<ExprArgument>,
}
impl From<&ExprBinary> for ExprBinary {
    fn from(value: &ExprBinary) -> Self {
        value.clone()
    }
}
impl ExprBinary {
    pub fn builder() -> builder::ExprBinary {
        Default::default()
    }
}
#[doc = "Expresion operation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expresion operation\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"add\","]
#[doc = "    \"sub\","]
#[doc = "    \"mul\","]
#[doc = "    \"div\","]
#[doc = "    \"mod\","]
#[doc = "    \"pow\","]
#[doc = "    \"index\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ExprBinaryExpression {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "sub")]
    Sub,
    #[serde(rename = "mul")]
    Mul,
    #[serde(rename = "div")]
    Div,
    #[serde(rename = "mod")]
    Mod,
    #[serde(rename = "pow")]
    Pow,
    #[serde(rename = "index")]
    Index,
}
impl From<&ExprBinaryExpression> for ExprBinaryExpression {
    fn from(value: &ExprBinaryExpression) -> Self {
        value.clone()
    }
}
impl ToString for ExprBinaryExpression {
    fn to_string(&self) -> String {
        match *self {
            Self::Add => "add".to_string(),
            Self::Sub => "sub".to_string(),
            Self::Mul => "mul".to_string(),
            Self::Div => "div".to_string(),
            Self::Mod => "mod".to_string(),
            Self::Pow => "pow".to_string(),
            Self::Index => "index".to_string(),
        }
    }
}
impl std::str::FromStr for ExprBinaryExpression {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "mul" => Ok(Self::Mul),
            "div" => Ok(Self::Div),
            "mod" => Ok(Self::Mod),
            "pow" => Ok(Self::Pow),
            "index" => Ok(Self::Index),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ExprBinaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ExprBinaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ExprBinaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Simple arithmetic binary expression involving variables and constants."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Simple arithmetic binary expression involving variables and constants.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expression\","]
#[doc = "    \"lhs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expression\": {"]
#[doc = "      \"description\": \"Expresion operation\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"neg\","]
#[doc = "        \"abs\","]
#[doc = "        \"floor\","]
#[doc = "        \"ceil\","]
#[doc = "        \"round\","]
#[doc = "        \"sqrt\","]
#[doc = "        \"exp\","]
#[doc = "        \"log2\","]
#[doc = "        \"log\","]
#[doc = "        \"sin\","]
#[doc = "        \"cos\","]
#[doc = "        \"tan\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"lhs\": {"]
#[doc = "      \"description\": \"Argument of an unary operation\","]
#[doc = "      \"$ref\": \"#/definitions/ExprArgument\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExprUnary {
    #[doc = "Expresion operation"]
    pub expression: ExprUnaryExpression,
    #[doc = "Argument of an unary operation"]
    pub lhs: ExprArgument,
}
impl From<&ExprUnary> for ExprUnary {
    fn from(value: &ExprUnary) -> Self {
        value.clone()
    }
}
impl ExprUnary {
    pub fn builder() -> builder::ExprUnary {
        Default::default()
    }
}
#[doc = "Expresion operation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expresion operation\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"neg\","]
#[doc = "    \"abs\","]
#[doc = "    \"floor\","]
#[doc = "    \"ceil\","]
#[doc = "    \"round\","]
#[doc = "    \"sqrt\","]
#[doc = "    \"exp\","]
#[doc = "    \"log2\","]
#[doc = "    \"log\","]
#[doc = "    \"sin\","]
#[doc = "    \"cos\","]
#[doc = "    \"tan\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ExprUnaryExpression {
    #[serde(rename = "neg")]
    Neg,
    #[serde(rename = "abs")]
    Abs,
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "ceil")]
    Ceil,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "exp")]
    Exp,
    #[serde(rename = "log2")]
    Log2,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "sin")]
    Sin,
    #[serde(rename = "cos")]
    Cos,
    #[serde(rename = "tan")]
    Tan,
}
impl From<&ExprUnaryExpression> for ExprUnaryExpression {
    fn from(value: &ExprUnaryExpression) -> Self {
        value.clone()
    }
}
impl ToString for ExprUnaryExpression {
    fn to_string(&self) -> String {
        match *self {
            Self::Neg => "neg".to_string(),
            Self::Abs => "abs".to_string(),
            Self::Floor => "floor".to_string(),
            Self::Ceil => "ceil".to_string(),
            Self::Round => "round".to_string(),
            Self::Sqrt => "sqrt".to_string(),
            Self::Exp => "exp".to_string(),
            Self::Log2 => "log2".to_string(),
            Self::Log => "log".to_string(),
            Self::Sin => "sin".to_string(),
            Self::Cos => "cos".to_string(),
            Self::Tan => "tan".to_string(),
        }
    }
}
impl std::str::FromStr for ExprUnaryExpression {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "neg" => Ok(Self::Neg),
            "abs" => Ok(Self::Abs),
            "floor" => Ok(Self::Floor),
            "ceil" => Ok(Self::Ceil),
            "round" => Ok(Self::Round),
            "sqrt" => Ok(Self::Sqrt),
            "exp" => Ok(Self::Exp),
            "log2" => Ok(Self::Log2),
            "log" => Ok(Self::Log),
            "sin" => Ok(Self::Sin),
            "cos" => Ok(Self::Cos),
            "tan" => Ok(Self::Tan),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ExprUnaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ExprUnaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ExprUnaryExpression {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Mathematical expression involving variables and constants.\n\nThe expression is evaluated in the context of any parametrizable field.\n\nIf the context requires an integer value, the float result is rounded at the end. If the expression type differs from expected by the context (e.g. channel_name), it is a runtime error. If an expression result array length differs from expected, a it is a runtime error."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Mathematical expression involving variables and constants.\\n\\nThe expression is evaluated in the context of any parametrizable field.\\n\\nIf the context requires an integer value, the float result is rounded at the end. If the expression type differs from expected by the context (e.g. channel_name), it is a runtime error. If an expression result array length differs from expected, a it is a runtime error.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExprBinary\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ExprUnary\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Expression {
    Binary(ExprBinary),
    Unary(ExprUnary),
}
impl From<&Expression> for Expression {
    fn from(value: &Expression) -> Self {
        value.clone()
    }
}
impl From<ExprBinary> for Expression {
    fn from(value: ExprBinary) -> Self {
        Self::Binary(value)
    }
}
impl From<ExprUnary> for Expression {
    fn from(value: ExprUnary) -> Self {
        Self::Unary(value)
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
#[doc = "HardcodedDevice"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Chadoq2\","]
#[doc = "    \"IroiseMVP\","]
#[doc = "    \"MockDevice\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum HardcodedDevice {
    Chadoq2,
    #[serde(rename = "IroiseMVP")]
    IroiseMvp,
    MockDevice,
}
impl From<&HardcodedDevice> for HardcodedDevice {
    fn from(value: &HardcodedDevice) -> Self {
        value.clone()
    }
}
impl ToString for HardcodedDevice {
    fn to_string(&self) -> String {
        match *self {
            Self::Chadoq2 => "Chadoq2".to_string(),
            Self::IroiseMvp => "IroiseMVP".to_string(),
            Self::MockDevice => "MockDevice".to_string(),
        }
    }
}
impl std::str::FromStr for HardcodedDevice {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Chadoq2" => Ok(Self::Chadoq2),
            "IroiseMVP" => Ok(Self::IroiseMvp),
            "MockDevice" => Ok(Self::MockDevice),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for HardcodedDevice {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for HardcodedDevice {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for HardcodedDevice {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Creates a waveform from interpolation of a set of data points. Uses pchip interpolation algorithm."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Creates a waveform from interpolation of a set of data points. Uses pchip interpolation algorithm.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"duration\","]
#[doc = "    \"kind\","]
#[doc = "    \"times\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The waveform duration (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"interpolated\""]
#[doc = "    },"]
#[doc = "    \"times\": {"]
#[doc = "      \"description\": \"Fractions of the total duration (between 0 and 1), indicating where to place each value on the time axis. The array size must be the same as `values` array size.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNumArray\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"Values of the interpolation points (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNumArray\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InterpolatedWaveform {
    #[doc = "The waveform duration (in ns)."]
    pub duration: ParametrizedNum,
    pub kind: String,
    #[doc = "Fractions of the total duration (between 0 and 1), indicating where to place each value on the time axis. The array size must be the same as `values` array size."]
    pub times: ParametrizedNumArray,
    #[doc = "Values of the interpolation points (in rad/µs)."]
    pub values: ParametrizedNumArray,
}
impl From<&InterpolatedWaveform> for InterpolatedWaveform {
    fn from(value: &InterpolatedWaveform) -> Self {
        value.clone()
    }
}
impl InterpolatedWaveform {
    pub fn builder() -> builder::InterpolatedWaveform {
        Default::default()
    }
}
#[doc = "A Kaiser window of a specified max value, area and beta parameter."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A Kaiser window of a specified max value, area and beta parameter.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"beta\","]
#[doc = "    \"kind\","]
#[doc = "    \"max_val\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"beta\": {"]
#[doc = "      \"description\": \"The beta parameter of the Kaiser window. A typical value is 14.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"kaiser_max\""]
#[doc = "    },"]
#[doc = "    \"max_val\": {"]
#[doc = "      \"description\": \"The waveform peak value.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KaiserMaxWaveform {
    #[doc = "The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values."]
    pub area: ParametrizedNum,
    #[doc = "The beta parameter of the Kaiser window. A typical value is 14."]
    pub beta: ParametrizedNum,
    pub kind: String,
    #[doc = "The waveform peak value."]
    pub max_val: ParametrizedNum,
}
impl From<&KaiserMaxWaveform> for KaiserMaxWaveform {
    fn from(value: &KaiserMaxWaveform) -> Self {
        value.clone()
    }
}
impl KaiserMaxWaveform {
    pub fn builder() -> builder::KaiserMaxWaveform {
        Default::default()
    }
}
#[doc = "A Kaiser window of a specified duration, area and beta parameter."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A Kaiser window of a specified duration, area and beta parameter.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"area\","]
#[doc = "    \"beta\","]
#[doc = "    \"duration\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"area\": {"]
#[doc = "      \"description\": \"The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"beta\": {"]
#[doc = "      \"description\": \"The beta parameter of the Kaiser window. A typical value is 14.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The waveform duration (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"kaiser\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KaiserWaveform {
    #[doc = "The integral of the waveform. Can be negative, in which case it takes the positive waveform and changes the sign of all its values."]
    pub area: ParametrizedNum,
    #[doc = "The beta parameter of the Kaiser window. A typical value is 14."]
    pub beta: ParametrizedNum,
    #[doc = "The waveform duration (in ns)."]
    pub duration: ParametrizedNum,
    pub kind: String,
}
impl From<&KaiserWaveform> for KaiserWaveform {
    fn from(value: &KaiserWaveform) -> Self {
        value.clone()
    }
}
impl KaiserWaveform {
    pub fn builder() -> builder::KaiserWaveform {
        Default::default()
    }
}
#[doc = "MappableQubit"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"qid\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default_trap\": {"]
#[doc = "      \"description\": \"An optional default trap ID.\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"qid\": {"]
#[doc = "      \"description\": \"The ID of the qubit.\","]
#[doc = "      \"$ref\": \"#/definitions/QubitId\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MappableQubit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_trap: Option<f64>,
    #[doc = "The ID of the qubit."]
    pub qid: QubitId,
}
impl From<&MappableQubit> for MappableQubit {
    fn from(value: &MappableQubit) -> Self {
        value.clone()
    }
}
impl MappableQubit {
    pub fn builder() -> builder::MappableQubit {
        Default::default()
    }
}
#[doc = "OpAddDmmDet"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dmm_name\","]
#[doc = "    \"op\","]
#[doc = "    \"protocol\","]
#[doc = "    \"waveform\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dmm_name\": {"]
#[doc = "      \"description\": \"The name of the DMM.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"add_dmm_detuning\""]
#[doc = "    },"]
#[doc = "    \"protocol\": {"]
#[doc = "      \"description\": \"Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\\n\\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\\n\\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\\n\\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"min-delay\","]
#[doc = "        \"no-delay\","]
#[doc = "        \"wait-for-all\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"waveform\": {"]
#[doc = "      \"description\": \"The waveform to add to the detuning of the DMM.\","]
#[doc = "      \"$ref\": \"#/definitions/Waveform\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpAddDmmDet {
    #[doc = "The name of the DMM."]
    pub dmm_name: ChannelName,
    pub op: String,
    #[doc = "Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\n\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\n\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\n\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse."]
    pub protocol: OpAddDmmDetProtocol,
    #[doc = "The waveform to add to the detuning of the DMM."]
    pub waveform: Waveform,
}
impl From<&OpAddDmmDet> for OpAddDmmDet {
    fn from(value: &OpAddDmmDet) -> Self {
        value.clone()
    }
}
impl OpAddDmmDet {
    pub fn builder() -> builder::OpAddDmmDet {
        Default::default()
    }
}
#[doc = "Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\n\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\n\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\n\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\\n\\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\\n\\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\\n\\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"min-delay\","]
#[doc = "    \"no-delay\","]
#[doc = "    \"wait-for-all\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OpAddDmmDetProtocol {
    #[serde(rename = "min-delay")]
    MinDelay,
    #[serde(rename = "no-delay")]
    NoDelay,
    #[serde(rename = "wait-for-all")]
    WaitForAll,
}
impl From<&OpAddDmmDetProtocol> for OpAddDmmDetProtocol {
    fn from(value: &OpAddDmmDetProtocol) -> Self {
        value.clone()
    }
}
impl ToString for OpAddDmmDetProtocol {
    fn to_string(&self) -> String {
        match *self {
            Self::MinDelay => "min-delay".to_string(),
            Self::NoDelay => "no-delay".to_string(),
            Self::WaitForAll => "wait-for-all".to_string(),
        }
    }
}
impl std::str::FromStr for OpAddDmmDetProtocol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "min-delay" => Ok(Self::MinDelay),
            "no-delay" => Ok(Self::NoDelay),
            "wait-for-all" => Ok(Self::WaitForAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for OpAddDmmDetProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OpAddDmmDetProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OpAddDmmDetProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Aligns multiple channels in time.\n\nIntroduces delays that align the provided channels with the one that finished the latest, such that the next action added to any of them will start right after the latest channel has finished."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Aligns multiple channels in time.\\n\\nIntroduces delays that align the provided channels with the one that finished the latest, such that the next action added to any of them will start right after the latest channel has finished.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channels\","]
#[doc = "    \"op\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channels\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"align\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpAlign {
    pub channels: Vec<ChannelName>,
    pub op: String,
}
impl From<&OpAlign> for OpAlign {
    fn from(value: &OpAlign) -> Self {
        value.clone()
    }
}
impl OpAlign {
    pub fn builder() -> builder::OpAlign {
        Default::default()
    }
}
#[doc = "OpConfigDetMap"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"detuning_map\","]
#[doc = "    \"dmm_id\","]
#[doc = "    \"op\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"detuning_map\": {"]
#[doc = "      \"description\": \"DetuningMap to associate with the DMM channel.\","]
#[doc = "      \"$ref\": \"#/definitions/WeightMap\""]
#[doc = "    },"]
#[doc = "    \"dmm_id\": {"]
#[doc = "      \"description\": \"ID of the DMM channel to configure.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"config_detuning_map\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpConfigDetMap {
    #[doc = "DetuningMap to associate with the DMM channel."]
    pub detuning_map: WeightMap,
    #[doc = "ID of the DMM channel to configure."]
    pub dmm_id: ChannelId,
    pub op: String,
}
impl From<&OpConfigDetMap> for OpConfigDetMap {
    fn from(value: &OpConfigDetMap) -> Self {
        value.clone()
    }
}
impl OpConfigDetMap {
    pub fn builder() -> builder::OpConfigDetMap {
        Default::default()
    }
}
#[doc = "OpConfigSlm"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dmm_id\","]
#[doc = "    \"op\","]
#[doc = "    \"qubits\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dmm_id\": {"]
#[doc = "      \"description\": \"ID of the DMM channel to use for the SLM mask.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"config_slm_mask\""]
#[doc = "    },"]
#[doc = "    \"qubits\": {"]
#[doc = "      \"description\": \"Qubit ID's to mask during the first global pulse of the sequence.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/QubitId\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpConfigSlm {
    #[doc = "ID of the DMM channel to use for the SLM mask."]
    pub dmm_id: ChannelId,
    pub op: String,
    #[doc = "Qubit ID's to mask during the first global pulse of the sequence."]
    pub qubits: Vec<QubitId>,
}
impl From<&OpConfigSlm> for OpConfigSlm {
    fn from(value: &OpConfigSlm) -> Self {
        value.clone()
    }
}
impl OpConfigSlm {
    pub fn builder() -> builder::OpConfigSlm {
        Default::default()
    }
}
#[doc = "Adds extra fixed delay before starting the pulse."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Adds extra fixed delay before starting the pulse.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel\","]
#[doc = "    \"op\","]
#[doc = "    \"time\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"Channel on which to insert a delay\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"delay\""]
#[doc = "    },"]
#[doc = "    \"time\": {"]
#[doc = "      \"description\": \"Delay time\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpDelay {
    #[doc = "Channel on which to insert a delay"]
    pub channel: ChannelName,
    pub op: String,
    #[doc = "Delay time"]
    pub time: ParametrizedNum,
}
impl From<&OpDelay> for OpDelay {
    fn from(value: &OpDelay) -> Self {
        value.clone()
    }
}
impl OpDelay {
    pub fn builder() -> builder::OpDelay {
        Default::default()
    }
}
#[doc = "OpDisableEom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel\","]
#[doc = "    \"op\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"The name of the channel to take out of EOM mode.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"correct_phase_drift\": {"]
#[doc = "      \"description\": \"Performs a phase shift to correct for the phase drift that occured since the last pulse (or the start of the EOM mode, if no pulse was added).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"disable_eom_mode\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpDisableEom {
    #[doc = "The name of the channel to take out of EOM mode."]
    pub channel: ChannelName,
    #[doc = "Performs a phase shift to correct for the phase drift that occured since the last pulse (or the start of the EOM mode, if no pulse was added)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_phase_drift: Option<bool>,
    pub op: String,
}
impl From<&OpDisableEom> for OpDisableEom {
    fn from(value: &OpDisableEom) -> Self {
        value.clone()
    }
}
impl OpDisableEom {
    pub fn builder() -> builder::OpDisableEom {
        Default::default()
    }
}
#[doc = "OpEnableEom"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amp_on\","]
#[doc = "    \"channel\","]
#[doc = "    \"detuning_on\","]
#[doc = "    \"op\","]
#[doc = "    \"optimal_detuning_off\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amp_on\": {"]
#[doc = "      \"description\": \"The amplitude of the EOM pulses (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"The name of the channel to put in EOM mode.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"correct_phase_drift\": {"]
#[doc = "      \"description\": \"Performs a phase shift to correct for the phase drift incurred while turning on the EOM mode.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"detuning_on\": {"]
#[doc = "      \"description\": \"The detuning of the EOM pulses (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"enable_eom_mode\""]
#[doc = "    },"]
#[doc = "    \"optimal_detuning_off\": {"]
#[doc = "      \"description\": \"The optimal value of detuning when there is no pulse being played (in rad/µs). It will choose the closest value among the existing options.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpEnableEom {
    #[doc = "The amplitude of the EOM pulses (in rad/µs)."]
    pub amp_on: ParametrizedNum,
    #[doc = "The name of the channel to put in EOM mode."]
    pub channel: ChannelName,
    #[doc = "Performs a phase shift to correct for the phase drift incurred while turning on the EOM mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_phase_drift: Option<bool>,
    #[doc = "The detuning of the EOM pulses (in rad/µs)."]
    pub detuning_on: ParametrizedNum,
    pub op: String,
    #[doc = "The optimal value of detuning when there is no pulse being played (in rad/µs). It will choose the closest value among the existing options."]
    pub optimal_detuning_off: ParametrizedNum,
}
impl From<&OpEnableEom> for OpEnableEom {
    fn from(value: &OpEnableEom) -> Self {
        value.clone()
    }
}
impl OpEnableEom {
    pub fn builder() -> builder::OpEnableEom {
        Default::default()
    }
}
#[doc = "OpEomPulse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel\","]
#[doc = "    \"duration\","]
#[doc = "    \"op\","]
#[doc = "    \"phase\","]
#[doc = "    \"post_phase_shift\","]
#[doc = "    \"protocol\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"The name of the channel to add the pulse to.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"correct_phase_drift\": {"]
#[doc = "      \"description\": \"Performs a phase shift to correct for the phase drift that occured since the last pulse (or the start of the EOM mode, if adding the first pulse).\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The duration of the pulse (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"add_eom_pulse\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"The pulse phase (in radians).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"post_phase_shift\": {"]
#[doc = "      \"description\": \"A phase shift (in radians) immediately after the end of the pulse.\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"protocol\": {"]
#[doc = "      \"description\": \"Stipulates how to deal with eventual conflicts with other channels.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"min-delay\","]
#[doc = "        \"no-delay\","]
#[doc = "        \"wait-for-all\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpEomPulse {
    #[doc = "The name of the channel to add the pulse to."]
    pub channel: ChannelName,
    #[doc = "Performs a phase shift to correct for the phase drift that occured since the last pulse (or the start of the EOM mode, if adding the first pulse)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_phase_drift: Option<bool>,
    #[doc = "The duration of the pulse (in ns)."]
    pub duration: ParametrizedNum,
    pub op: String,
    #[doc = "The pulse phase (in radians)."]
    pub phase: ParametrizedNum,
    #[doc = "A phase shift (in radians) immediately after the end of the pulse."]
    pub post_phase_shift: ParametrizedNum,
    #[doc = "Stipulates how to deal with eventual conflicts with other channels."]
    pub protocol: OpEomPulseProtocol,
}
impl From<&OpEomPulse> for OpEomPulse {
    fn from(value: &OpEomPulse) -> Self {
        value.clone()
    }
}
impl OpEomPulse {
    pub fn builder() -> builder::OpEomPulse {
        Default::default()
    }
}
#[doc = "Stipulates how to deal with eventual conflicts with other channels."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stipulates how to deal with eventual conflicts with other channels.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"min-delay\","]
#[doc = "    \"no-delay\","]
#[doc = "    \"wait-for-all\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OpEomPulseProtocol {
    #[serde(rename = "min-delay")]
    MinDelay,
    #[serde(rename = "no-delay")]
    NoDelay,
    #[serde(rename = "wait-for-all")]
    WaitForAll,
}
impl From<&OpEomPulseProtocol> for OpEomPulseProtocol {
    fn from(value: &OpEomPulseProtocol) -> Self {
        value.clone()
    }
}
impl ToString for OpEomPulseProtocol {
    fn to_string(&self) -> String {
        match *self {
            Self::MinDelay => "min-delay".to_string(),
            Self::NoDelay => "no-delay".to_string(),
            Self::WaitForAll => "wait-for-all".to_string(),
        }
    }
}
impl std::str::FromStr for OpEomPulseProtocol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "min-delay" => Ok(Self::MinDelay),
            "no-delay" => Ok(Self::NoDelay),
            "wait-for-all" => Ok(Self::WaitForAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for OpEomPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OpEomPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OpEomPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Adds a separate phase shift to atoms. If possible, OpPulse phase and post_phase_shift are preferred."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Adds a separate phase shift to atoms. If possible, OpPulse phase and post_phase_shift are preferred.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"basis\","]
#[doc = "    \"op\","]
#[doc = "    \"phi\","]
#[doc = "    \"targets\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"basis\": {"]
#[doc = "      \"description\": \"Phase shift basis\","]
#[doc = "      \"$ref\": \"#/definitions/Basis\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"phase_shift\""]
#[doc = "    },"]
#[doc = "    \"phi\": {"]
#[doc = "      \"description\": \"The intended phase shift (in rads).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"targets\": {"]
#[doc = "      \"description\": \"Target atom indices\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpPhaseShift {
    #[doc = "Phase shift basis"]
    pub basis: Basis,
    pub op: String,
    #[doc = "The intended phase shift (in rads)."]
    pub phi: ParametrizedNum,
    #[doc = "Target atom indices"]
    pub targets: Vec<ParametrizedNum>,
}
impl From<&OpPhaseShift> for OpPhaseShift {
    fn from(value: &OpPhaseShift) -> Self {
        value.clone()
    }
}
impl OpPhaseShift {
    pub fn builder() -> builder::OpPhaseShift {
        Default::default()
    }
}
#[doc = "Pulse is a modulation of a frequency signal in amplitude and/or frequency, with a specific phase, over a given duration.\n\nNote:     We define the ``amplitude`` of a pulse to be its Rabi frequency, `ω`, in rad/µs.     Equivalently, the ``detuning`` is `Δ`, also in rad/µs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Pulse is a modulation of a frequency signal in amplitude and/or frequency, with a specific phase, over a given duration.\\n\\nNote:     We define the ``amplitude`` of a pulse to be its Rabi frequency, `ω`, in rad/µs.     Equivalently, the ``detuning`` is `Δ`, also in rad/µs.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"amplitude\","]
#[doc = "    \"channel\","]
#[doc = "    \"detuning\","]
#[doc = "    \"op\","]
#[doc = "    \"phase\","]
#[doc = "    \"post_phase_shift\","]
#[doc = "    \"protocol\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"amplitude\": {"]
#[doc = "      \"description\": \"Pulse amplitude waveform (in rad/µs)\","]
#[doc = "      \"$ref\": \"#/definitions/Waveform\""]
#[doc = "    },"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"Device channel to use for this pulse.\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"detuning\": {"]
#[doc = "      \"description\": \"Shift in frequency from the channel's central frequency over time (in rad/µs)\","]
#[doc = "      \"$ref\": \"#/definitions/Waveform\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"pulse\""]
#[doc = "    },"]
#[doc = "    \"phase\": {"]
#[doc = "      \"description\": \"The pulse phase (in radians)\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"post_phase_shift\": {"]
#[doc = "      \"description\": \"A phase shift (in radians) immediately after the end of the pulse\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"protocol\": {"]
#[doc = "      \"description\": \"Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\\n\\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\\n\\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\\n\\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"min-delay\","]
#[doc = "        \"no-delay\","]
#[doc = "        \"wait-for-all\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpPulse {
    #[doc = "Pulse amplitude waveform (in rad/µs)"]
    pub amplitude: Waveform,
    #[doc = "Device channel to use for this pulse."]
    pub channel: ChannelName,
    #[doc = "Shift in frequency from the channel's central frequency over time (in rad/µs)"]
    pub detuning: Waveform,
    pub op: String,
    #[doc = "The pulse phase (in radians)"]
    pub phase: ParametrizedNum,
    #[doc = "A phase shift (in radians) immediately after the end of the pulse"]
    pub post_phase_shift: ParametrizedNum,
    #[doc = "Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\n\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\n\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\n\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse."]
    pub protocol: OpPulseProtocol,
}
impl From<&OpPulse> for OpPulse {
    fn from(value: &OpPulse) -> Self {
        value.clone()
    }
}
impl OpPulse {
    pub fn builder() -> builder::OpPulse {
        Default::default()
    }
}
#[doc = "Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\n\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\n\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\n\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Stipulates how to deal with eventual conflicts with other channels, specifically in terms of having multiple channels act on the same target simultaneously.\\n\\n- ``'min-delay'``: Before adding the pulse, introduces the   smallest possible delay that avoids all exisiting conflicts.\\n\\n- ``'no-delay'``: Adds the pulse to the channel, regardless of   existing conflicts.\\n\\n- ``'wait-for-all'``: Before adding the pulse, adds a delay   that idles the channel until the end of the other channels'   latest pulse.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"min-delay\","]
#[doc = "    \"no-delay\","]
#[doc = "    \"wait-for-all\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OpPulseProtocol {
    #[serde(rename = "min-delay")]
    MinDelay,
    #[serde(rename = "no-delay")]
    NoDelay,
    #[serde(rename = "wait-for-all")]
    WaitForAll,
}
impl From<&OpPulseProtocol> for OpPulseProtocol {
    fn from(value: &OpPulseProtocol) -> Self {
        value.clone()
    }
}
impl ToString for OpPulseProtocol {
    fn to_string(&self) -> String {
        match *self {
            Self::MinDelay => "min-delay".to_string(),
            Self::NoDelay => "no-delay".to_string(),
            Self::WaitForAll => "wait-for-all".to_string(),
        }
    }
}
impl std::str::FromStr for OpPulseProtocol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "min-delay" => Ok(Self::MinDelay),
            "no-delay" => Ok(Self::NoDelay),
            "wait-for-all" => Ok(Self::WaitForAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for OpPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OpPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OpPulseProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Adds a waveform to the pulse."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Adds a waveform to the pulse.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channel\","]
#[doc = "    \"op\","]
#[doc = "    \"target\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"channel\": {"]
#[doc = "      \"description\": \"Channel to retarget. Must be local\","]
#[doc = "      \"$ref\": \"#/definitions/ChannelName\""]
#[doc = "    },"]
#[doc = "    \"op\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"target\""]
#[doc = "    },"]
#[doc = "    \"target\": {"]
#[doc = "      \"description\": \"New target atom index (or indices)\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ParametrizedNumArray\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpTarget {
    #[doc = "Channel to retarget. Must be local"]
    pub channel: ChannelName,
    pub op: String,
    #[doc = "New target atom index (or indices)"]
    pub target: OpTargetTarget,
}
impl From<&OpTarget> for OpTarget {
    fn from(value: &OpTarget) -> Self {
        value.clone()
    }
}
impl OpTarget {
    pub fn builder() -> builder::OpTarget {
        Default::default()
    }
}
#[doc = "New target atom index (or indices)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"New target atom index (or indices)\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNumArray\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpTargetTarget {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<ParametrizedNum>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<ParametrizedNumArray>,
}
impl From<&OpTargetTarget> for OpTargetTarget {
    fn from(value: &OpTargetTarget) -> Self {
        value.clone()
    }
}
impl OpTargetTarget {
    pub fn builder() -> builder::OpTargetTarget {
        Default::default()
    }
}
#[doc = "Sequence operation. All operations are performed in specified order."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sequence operation. All operations are performed in specified order.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpAlign\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpDelay\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpTarget\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpPulse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpPhaseShift\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpEnableEOM\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpDisableEOM\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpEOMPulse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpConfigSLM\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpConfigDetMap\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpAddDmmDet\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Operation {
    Align(OpAlign),
    Delay(OpDelay),
    Target(OpTarget),
    Pulse(OpPulse),
    PhaseShift(OpPhaseShift),
    EnableEom(OpEnableEom),
    DisableEom(OpDisableEom),
    EomPulse(OpEomPulse),
    ConfigSlm(OpConfigSlm),
    ConfigDetMap(OpConfigDetMap),
    AddDmmDet(OpAddDmmDet),
}
impl From<&Operation> for Operation {
    fn from(value: &Operation) -> Self {
        value.clone()
    }
}
impl From<OpAlign> for Operation {
    fn from(value: OpAlign) -> Self {
        Self::Align(value)
    }
}
impl From<OpDelay> for Operation {
    fn from(value: OpDelay) -> Self {
        Self::Delay(value)
    }
}
impl From<OpTarget> for Operation {
    fn from(value: OpTarget) -> Self {
        Self::Target(value)
    }
}
impl From<OpPulse> for Operation {
    fn from(value: OpPulse) -> Self {
        Self::Pulse(value)
    }
}
impl From<OpPhaseShift> for Operation {
    fn from(value: OpPhaseShift) -> Self {
        Self::PhaseShift(value)
    }
}
impl From<OpEnableEom> for Operation {
    fn from(value: OpEnableEom) -> Self {
        Self::EnableEom(value)
    }
}
impl From<OpDisableEom> for Operation {
    fn from(value: OpDisableEom) -> Self {
        Self::DisableEom(value)
    }
}
impl From<OpEomPulse> for Operation {
    fn from(value: OpEomPulse) -> Self {
        Self::EomPulse(value)
    }
}
impl From<OpConfigSlm> for Operation {
    fn from(value: OpConfigSlm) -> Self {
        Self::ConfigSlm(value)
    }
}
impl From<OpConfigDetMap> for Operation {
    fn from(value: OpConfigDetMap) -> Self {
        Self::ConfigDetMap(value)
    }
}
impl From<OpAddDmmDet> for Operation {
    fn from(value: OpAddDmmDet) -> Self {
        Self::AddDmmDet(value)
    }
}
#[doc = "Numeric scalar value that can be parametrized"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Numeric scalar value that can be parametrized\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParametrizedNum {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<f64>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Expression>,
}
impl From<&ParametrizedNum> for ParametrizedNum {
    fn from(value: &ParametrizedNum) -> Self {
        value.clone()
    }
}
impl ParametrizedNum {
    pub fn builder() -> builder::ParametrizedNum {
        Default::default()
    }
}
#[doc = "Numeric array value that can be parametrized"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Numeric array value that can be parametrized\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Expression\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/VariableRef\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParametrizedNumArray {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<Vec<f64>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<Expression>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<VariableRef>,
}
impl From<&ParametrizedNumArray> for ParametrizedNumArray {
    fn from(value: &ParametrizedNumArray) -> Self {
        value.clone()
    }
}
impl ParametrizedNumArray {
    pub fn builder() -> builder::ParametrizedNumArray {
        Default::default()
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
#[doc = "Pulser import/export data structure."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Pulser import/export data structure.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"channels\","]
#[doc = "        \"device\","]
#[doc = "        \"measurement\","]
#[doc = "        \"name\","]
#[doc = "        \"operations\","]
#[doc = "        \"register\","]
#[doc = "        \"variables\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$schema\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channels\": {"]
#[doc = "          \"description\": \"Channels declared in this Sequence.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"device\": {"]
#[doc = "          \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Device\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"layout\": {"]
#[doc = "          \"description\": \"The trap layout underlying the register.\","]
#[doc = "          \"$ref\": \"#/definitions/Layout\""]
#[doc = "        },"]
#[doc = "        \"magnetic_field\": {"]
#[doc = "          \"description\": \"The magnetic field components in x, y and z (in Gauss)\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 3,"]
#[doc = "          \"minItems\": 3"]
#[doc = "        },"]
#[doc = "        \"measurement\": {"]
#[doc = "          \"description\": \"Type of measurement to perform after all pulses are executed\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Basis\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"User-assigned sequence name. Can be autogenerated on export if not provided.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"operations\": {"]
#[doc = "          \"description\": \"Sequence of pulses, delays and target changes, performed in specified order.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Operation\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"register\": {"]
#[doc = "          \"description\": \"A 2D register containing a set of atoms.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Atom\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"slm_mask_targets\": {"]
#[doc = "          \"description\": \"The qubits to mask during the first global pulse of the sequence.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/QubitId\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"variables\": {"]
#[doc = "          \"description\": \"Variables and expressions that can be used in expressions or parametrized values.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/Variable\""]
#[doc = "          }"]
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
#[doc = "        \"device\","]
#[doc = "        \"layout\","]
#[doc = "        \"measurement\","]
#[doc = "        \"name\","]
#[doc = "        \"operations\","]
#[doc = "        \"register\","]
#[doc = "        \"variables\","]
#[doc = "        \"version\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$schema\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"channels\": {"]
#[doc = "          \"description\": \"Channels declared in this Sequence.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"device\": {"]
#[doc = "          \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Device\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"layout\": {"]
#[doc = "          \"description\": \"The trap layout underlying the register.\","]
#[doc = "          \"$ref\": \"#/definitions/Layout\""]
#[doc = "        },"]
#[doc = "        \"magnetic_field\": {"]
#[doc = "          \"description\": \"The magnetic field components in x, y and z (in Gauss)\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 3,"]
#[doc = "          \"minItems\": 3"]
#[doc = "        },"]
#[doc = "        \"measurement\": {"]
#[doc = "          \"description\": \"Type of measurement to perform after all pulses are executed\","]
#[doc = "          \"anyOf\": ["]
#[doc = "            {"]
#[doc = "              \"$ref\": \"#/definitions/Basis\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"null\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"User-assigned sequence name. Can be autogenerated on export if not provided.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"operations\": {"]
#[doc = "          \"description\": \"Sequence of pulses, delays and target changes, performed in specified order.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/Operation\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"register\": {"]
#[doc = "          \"description\": \"A  list of qubit IDs.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/MappableQubit\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"slm_mask_targets\": {"]
#[doc = "          \"description\": \"The qubits to mask during the first global pulse of the sequence.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/definitions/QubitId\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"variables\": {"]
#[doc = "          \"description\": \"Variables and expressions that can be used in expressions or parametrized values.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$ref\": \"#/definitions/Variable\""]
#[doc = "          }"]
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
pub struct PulserSequence {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<PulserSequenceSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<PulserSequenceSubtype1>,
}
impl From<&PulserSequence> for PulserSequence {
    fn from(value: &PulserSequence) -> Self {
        value.clone()
    }
}
impl PulserSequence {
    pub fn builder() -> builder::PulserSequence {
        Default::default()
    }
}
#[doc = "PulserSequenceSubtype0"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channels\","]
#[doc = "    \"device\","]
#[doc = "    \"measurement\","]
#[doc = "    \"name\","]
#[doc = "    \"operations\","]
#[doc = "    \"register\","]
#[doc = "    \"variables\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"channels\": {"]
#[doc = "      \"description\": \"Channels declared in this Sequence.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Device\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"description\": \"The trap layout underlying the register.\","]
#[doc = "      \"$ref\": \"#/definitions/Layout\""]
#[doc = "    },"]
#[doc = "    \"magnetic_field\": {"]
#[doc = "      \"description\": \"The magnetic field components in x, y and z (in Gauss)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 3,"]
#[doc = "      \"minItems\": 3"]
#[doc = "    },"]
#[doc = "    \"measurement\": {"]
#[doc = "      \"description\": \"Type of measurement to perform after all pulses are executed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Basis\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"User-assigned sequence name. Can be autogenerated on export if not provided.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"operations\": {"]
#[doc = "      \"description\": \"Sequence of pulses, delays and target changes, performed in specified order.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Operation\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"register\": {"]
#[doc = "      \"description\": \"A 2D register containing a set of atoms.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Atom\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"slm_mask_targets\": {"]
#[doc = "      \"description\": \"The qubits to mask during the first global pulse of the sequence.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/QubitId\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"variables\": {"]
#[doc = "      \"description\": \"Variables and expressions that can be used in expressions or parametrized values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/Variable\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"1\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PulserSequenceSubtype0 {
    #[doc = "Channels declared in this Sequence."]
    pub channels: std::collections::HashMap<String, ChannelId>,
    #[doc = "A valid device in which to execute the Sequence"]
    pub device: PulserSequenceSubtype0Device,
    #[doc = "The trap layout underlying the register."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,
    #[doc = "The magnetic field components in x, y and z (in Gauss)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnetic_field: Option<[f64; 3usize]>,
    #[doc = "Type of measurement to perform after all pulses are executed"]
    pub measurement: Option<Basis>,
    #[doc = "User-assigned sequence name. Can be autogenerated on export if not provided."]
    pub name: String,
    #[doc = "Sequence of pulses, delays and target changes, performed in specified order."]
    pub operations: Vec<Operation>,
    #[doc = "A 2D register containing a set of atoms."]
    pub register: Vec<Atom>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The qubits to mask during the first global pulse of the sequence."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slm_mask_targets: Vec<QubitId>,
    #[doc = "Variables and expressions that can be used in expressions or parametrized values."]
    pub variables: std::collections::HashMap<String, Variable>,
    pub version: String,
}
impl From<&PulserSequenceSubtype0> for PulserSequenceSubtype0 {
    fn from(value: &PulserSequenceSubtype0) -> Self {
        value.clone()
    }
}
impl PulserSequenceSubtype0 {
    pub fn builder() -> builder::PulserSequenceSubtype0 {
        Default::default()
    }
}
#[doc = "A valid device in which to execute the Sequence"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Device\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PulserSequenceSubtype0Device {
    HardcodedDevice(HardcodedDevice),
    Device(Box<Device>),
}
impl From<&PulserSequenceSubtype0Device> for PulserSequenceSubtype0Device {
    fn from(value: &PulserSequenceSubtype0Device) -> Self {
        value.clone()
    }
}
impl From<HardcodedDevice> for PulserSequenceSubtype0Device {
    fn from(value: HardcodedDevice) -> Self {
        Self::HardcodedDevice(value)
    }
}
impl From<Device> for PulserSequenceSubtype0Device {
    fn from(value: Device) -> Self {
        Self::Device(Box::new(value))
    }
}
#[doc = "PulserSequenceSubtype1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"channels\","]
#[doc = "    \"device\","]
#[doc = "    \"layout\","]
#[doc = "    \"measurement\","]
#[doc = "    \"name\","]
#[doc = "    \"operations\","]
#[doc = "    \"register\","]
#[doc = "    \"variables\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"channels\": {"]
#[doc = "      \"description\": \"Channels declared in this Sequence.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/ChannelId\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"device\": {"]
#[doc = "      \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Device\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"layout\": {"]
#[doc = "      \"description\": \"The trap layout underlying the register.\","]
#[doc = "      \"$ref\": \"#/definitions/Layout\""]
#[doc = "    },"]
#[doc = "    \"magnetic_field\": {"]
#[doc = "      \"description\": \"The magnetic field components in x, y and z (in Gauss)\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 3,"]
#[doc = "      \"minItems\": 3"]
#[doc = "    },"]
#[doc = "    \"measurement\": {"]
#[doc = "      \"description\": \"Type of measurement to perform after all pulses are executed\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Basis\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"User-assigned sequence name. Can be autogenerated on export if not provided.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"operations\": {"]
#[doc = "      \"description\": \"Sequence of pulses, delays and target changes, performed in specified order.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Operation\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"register\": {"]
#[doc = "      \"description\": \"A  list of qubit IDs.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/MappableQubit\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"slm_mask_targets\": {"]
#[doc = "      \"description\": \"The qubits to mask during the first global pulse of the sequence.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/QubitId\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"variables\": {"]
#[doc = "      \"description\": \"Variables and expressions that can be used in expressions or parametrized values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/Variable\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"1\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PulserSequenceSubtype1 {
    #[doc = "Channels declared in this Sequence."]
    pub channels: std::collections::HashMap<String, ChannelId>,
    #[doc = "A valid device in which to execute the Sequence"]
    pub device: PulserSequenceSubtype1Device,
    #[doc = "The trap layout underlying the register."]
    pub layout: Layout,
    #[doc = "The magnetic field components in x, y and z (in Gauss)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnetic_field: Option<[f64; 3usize]>,
    #[doc = "Type of measurement to perform after all pulses are executed"]
    pub measurement: Option<Basis>,
    #[doc = "User-assigned sequence name. Can be autogenerated on export if not provided."]
    pub name: String,
    #[doc = "Sequence of pulses, delays and target changes, performed in specified order."]
    pub operations: Vec<Operation>,
    #[doc = "A  list of qubit IDs."]
    pub register: Vec<MappableQubit>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The qubits to mask during the first global pulse of the sequence."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slm_mask_targets: Vec<QubitId>,
    #[doc = "Variables and expressions that can be used in expressions or parametrized values."]
    pub variables: std::collections::HashMap<String, Variable>,
    pub version: String,
}
impl From<&PulserSequenceSubtype1> for PulserSequenceSubtype1 {
    fn from(value: &PulserSequenceSubtype1) -> Self {
        value.clone()
    }
}
impl PulserSequenceSubtype1 {
    pub fn builder() -> builder::PulserSequenceSubtype1 {
        Default::default()
    }
}
#[doc = "A valid device in which to execute the Sequence"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A valid device in which to execute the Sequence\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/HardcodedDevice\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Device\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PulserSequenceSubtype1Device {
    HardcodedDevice(HardcodedDevice),
    Device(Box<Device>),
}
impl From<&PulserSequenceSubtype1Device> for PulserSequenceSubtype1Device {
    fn from(value: &PulserSequenceSubtype1Device) -> Self {
        value.clone()
    }
}
impl From<HardcodedDevice> for PulserSequenceSubtype1Device {
    fn from(value: HardcodedDevice) -> Self {
        Self::HardcodedDevice(value)
    }
}
impl From<Device> for PulserSequenceSubtype1Device {
    fn from(value: Device) -> Self {
        Self::Device(Box::new(value))
    }
}
#[doc = "A linear ramp waveform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A linear ramp waveform.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"duration\","]
#[doc = "    \"kind\","]
#[doc = "    \"start\","]
#[doc = "    \"stop\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration\": {"]
#[doc = "      \"description\": \"The waveform duration (in ns).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ramp\""]
#[doc = "    },"]
#[doc = "    \"start\": {"]
#[doc = "      \"description\": \"The initial value (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    },"]
#[doc = "    \"stop\": {"]
#[doc = "      \"description\": \"The final value (in rad/µs).\","]
#[doc = "      \"$ref\": \"#/definitions/ParametrizedNum\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RampWaveform {
    #[doc = "The waveform duration (in ns)."]
    pub duration: ParametrizedNum,
    pub kind: String,
    #[doc = "The initial value (in rad/µs)."]
    pub start: ParametrizedNum,
    #[doc = "The final value (in rad/µs)."]
    pub stop: ParametrizedNum,
}
impl From<&RampWaveform> for RampWaveform {
    fn from(value: &RampWaveform) -> Self {
        value.clone()
    }
}
impl RampWaveform {
    pub fn builder() -> builder::RampWaveform {
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
#[doc = "Variable representing a typed value assigned during sequence build. variables can be used in expressions and parametrized values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Variable representing a typed value assigned during sequence build. variables can be used in expressions and parametrized values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"Variable type\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"int\","]
#[doc = "        \"float\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"description\": \"Default variable value. The default array length determines the variable array size.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Variable {
    #[doc = "Variable type"]
    #[serde(rename = "type")]
    pub type_: VariableType,
    #[doc = "Default variable value. The default array length determines the variable array size."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<f64>,
}
impl From<&Variable> for Variable {
    fn from(value: &Variable) -> Self {
        value.clone()
    }
}
impl Variable {
    pub fn builder() -> builder::Variable {
        Default::default()
    }
}
#[doc = "Name of declared variable."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of declared variable.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct VariableName(pub String);
impl std::ops::Deref for VariableName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VariableName> for String {
    fn from(value: VariableName) -> Self {
        value.0
    }
}
impl From<&VariableName> for VariableName {
    fn from(value: &VariableName) -> Self {
        value.clone()
    }
}
impl From<String> for VariableName {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for VariableName {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for VariableName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "References a declared variable by name."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"References a declared variable by name.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"variable\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"variable\": {"]
#[doc = "      \"description\": \"variable name, must reference declared variable\","]
#[doc = "      \"$ref\": \"#/definitions/VariableName\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VariableRef {
    #[doc = "variable name, must reference declared variable"]
    pub variable: VariableName,
}
impl From<&VariableRef> for VariableRef {
    fn from(value: &VariableRef) -> Self {
        value.clone()
    }
}
impl VariableRef {
    pub fn builder() -> builder::VariableRef {
        Default::default()
    }
}
#[doc = "Variable type"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Variable type\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"int\","]
#[doc = "    \"float\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VariableType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "float")]
    Float,
}
impl From<&VariableType> for VariableType {
    fn from(value: &VariableType) -> Self {
        value.clone()
    }
}
impl ToString for VariableType {
    fn to_string(&self) -> String {
        match *self {
            Self::Int => "int".to_string(),
            Self::Float => "float".to_string(),
        }
    }
}
impl std::str::FromStr for VariableType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for VariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for VariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for VariableType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Modulation waveform of any kind"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Modulation waveform of any kind\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CompositeWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CustomWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ConstantWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/RampWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlackmanWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/BlackmanMaxWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InterpolatedWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/KaiserWaveform\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/KaiserMaxWaveform\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Waveform {
    CompositeWaveform(CompositeWaveform),
    CustomWaveform(CustomWaveform),
    ConstantWaveform(ConstantWaveform),
    RampWaveform(RampWaveform),
    BlackmanWaveform(BlackmanWaveform),
    BlackmanMaxWaveform(BlackmanMaxWaveform),
    InterpolatedWaveform(InterpolatedWaveform),
    KaiserWaveform(KaiserWaveform),
    KaiserMaxWaveform(KaiserMaxWaveform),
}
impl From<&Waveform> for Waveform {
    fn from(value: &Waveform) -> Self {
        value.clone()
    }
}
impl From<CompositeWaveform> for Waveform {
    fn from(value: CompositeWaveform) -> Self {
        Self::CompositeWaveform(value)
    }
}
impl From<CustomWaveform> for Waveform {
    fn from(value: CustomWaveform) -> Self {
        Self::CustomWaveform(value)
    }
}
impl From<ConstantWaveform> for Waveform {
    fn from(value: ConstantWaveform) -> Self {
        Self::ConstantWaveform(value)
    }
}
impl From<RampWaveform> for Waveform {
    fn from(value: RampWaveform) -> Self {
        Self::RampWaveform(value)
    }
}
impl From<BlackmanWaveform> for Waveform {
    fn from(value: BlackmanWaveform) -> Self {
        Self::BlackmanWaveform(value)
    }
}
impl From<BlackmanMaxWaveform> for Waveform {
    fn from(value: BlackmanMaxWaveform) -> Self {
        Self::BlackmanMaxWaveform(value)
    }
}
impl From<InterpolatedWaveform> for Waveform {
    fn from(value: InterpolatedWaveform) -> Self {
        Self::InterpolatedWaveform(value)
    }
}
impl From<KaiserWaveform> for Waveform {
    fn from(value: KaiserWaveform) -> Self {
        Self::KaiserWaveform(value)
    }
}
impl From<KaiserMaxWaveform> for Waveform {
    fn from(value: KaiserMaxWaveform) -> Self {
        Self::KaiserMaxWaveform(value)
    }
}
#[doc = "Associates weights to trap coordinates. The sum of the provided weights must be equal to 1."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Associates weights to trap coordinates. The sum of the provided weights must be equal to 1.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"traps\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"slug\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"traps\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/WeightedTrap\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WeightMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub traps: Vec<WeightedTrap>,
}
impl From<&WeightMap> for WeightMap {
    fn from(value: &WeightMap) -> Self {
        value.clone()
    }
}
impl WeightMap {
    pub fn builder() -> builder::WeightMap {
        Default::default()
    }
}
#[doc = "WeightedTrap"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"weight\","]
#[doc = "    \"x\","]
#[doc = "    \"y\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"weight\": {"]
#[doc = "      \"description\": \"The weight on the site.\","]
#[doc = "      \"type\": \"number\""]
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
pub struct WeightedTrap {
    pub weight: f64,
    pub x: f64,
    pub y: f64,
}
impl From<&WeightedTrap> for WeightedTrap {
    fn from(value: &WeightedTrap) -> Self {
        value.clone()
    }
}
impl WeightedTrap {
    pub fn builder() -> builder::WeightedTrap {
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
    pub struct BlackmanMaxWaveform {
        area: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
        max_val: Result<super::ParametrizedNum, String>,
    }
    impl Default for BlackmanMaxWaveform {
        fn default() -> Self {
            Self {
                area: Err("no value supplied for area".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                max_val: Err("no value supplied for max_val".to_string()),
            }
        }
    }
    impl BlackmanMaxWaveform {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn max_val<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.max_val = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_val: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BlackmanMaxWaveform> for super::BlackmanMaxWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: BlackmanMaxWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                kind: value.kind?,
                max_val: value.max_val?,
            })
        }
    }
    impl From<super::BlackmanMaxWaveform> for BlackmanMaxWaveform {
        fn from(value: super::BlackmanMaxWaveform) -> Self {
            Self {
                area: Ok(value.area),
                kind: Ok(value.kind),
                max_val: Ok(value.max_val),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BlackmanWaveform {
        area: Result<super::ParametrizedNum, String>,
        duration: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
    }
    impl Default for BlackmanWaveform {
        fn default() -> Self {
            Self {
                area: Err("no value supplied for area".to_string()),
                duration: Err("no value supplied for duration".to_string()),
                kind: Err("no value supplied for kind".to_string()),
            }
        }
    }
    impl BlackmanWaveform {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BlackmanWaveform> for super::BlackmanWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: BlackmanWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                duration: value.duration?,
                kind: value.kind?,
            })
        }
    }
    impl From<super::BlackmanWaveform> for BlackmanWaveform {
        fn from(value: super::BlackmanWaveform) -> Self {
            Self {
                area: Ok(value.area),
                duration: Ok(value.duration),
                kind: Ok(value.kind),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CompositeWaveform {
        kind: Result<String, String>,
        waveforms: Result<Vec<super::Waveform>, String>,
    }
    impl Default for CompositeWaveform {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                waveforms: Err("no value supplied for waveforms".to_string()),
            }
        }
    }
    impl CompositeWaveform {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn waveforms<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Waveform>>,
            T::Error: std::fmt::Display,
        {
            self.waveforms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for waveforms: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CompositeWaveform> for super::CompositeWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: CompositeWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                waveforms: value.waveforms?,
            })
        }
    }
    impl From<super::CompositeWaveform> for CompositeWaveform {
        fn from(value: super::CompositeWaveform) -> Self {
            Self {
                kind: Ok(value.kind),
                waveforms: Ok(value.waveforms),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstantWaveform {
        duration: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
        value: Result<super::ParametrizedNum, String>,
    }
    impl Default for ConstantWaveform {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConstantWaveform {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ConstantWaveform> for super::ConstantWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: ConstantWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                kind: value.kind?,
                value: value.value?,
            })
        }
    }
    impl From<super::ConstantWaveform> for ConstantWaveform {
        fn from(value: super::ConstantWaveform) -> Self {
            Self {
                duration: Ok(value.duration),
                kind: Ok(value.kind),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomWaveform {
        kind: Result<String, String>,
        samples: Result<Vec<f64>, String>,
    }
    impl Default for CustomWaveform {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                samples: Err("no value supplied for samples".to_string()),
            }
        }
    }
    impl CustomWaveform {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn samples<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<f64>>,
            T::Error: std::fmt::Display,
        {
            self.samples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for samples: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CustomWaveform> for super::CustomWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: CustomWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                samples: value.samples?,
            })
        }
    }
    impl From<super::CustomWaveform> for CustomWaveform {
        fn from(value: super::CustomWaveform) -> Self {
            Self {
                kind: Ok(value.kind),
                samples: Ok(value.samples),
            }
        }
    }
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
    pub struct ExprBinary {
        expression: Result<super::ExprBinaryExpression, String>,
        lhs: Result<Box<super::ExprArgument>, String>,
        rhs: Result<Box<super::ExprArgument>, String>,
    }
    impl Default for ExprBinary {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
                lhs: Err("no value supplied for lhs".to_string()),
                rhs: Err("no value supplied for rhs".to_string()),
            }
        }
    }
    impl ExprBinary {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ExprBinaryExpression>,
            T::Error: std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expression: {}", e));
            self
        }
        pub fn lhs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::ExprArgument>>,
            T::Error: std::fmt::Display,
        {
            self.lhs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lhs: {}", e));
            self
        }
        pub fn rhs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Box<super::ExprArgument>>,
            T::Error: std::fmt::Display,
        {
            self.rhs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rhs: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExprBinary> for super::ExprBinary {
        type Error = super::error::ConversionError;
        fn try_from(value: ExprBinary) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
                lhs: value.lhs?,
                rhs: value.rhs?,
            })
        }
    }
    impl From<super::ExprBinary> for ExprBinary {
        fn from(value: super::ExprBinary) -> Self {
            Self {
                expression: Ok(value.expression),
                lhs: Ok(value.lhs),
                rhs: Ok(value.rhs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExprUnary {
        expression: Result<super::ExprUnaryExpression, String>,
        lhs: Result<super::ExprArgument, String>,
    }
    impl Default for ExprUnary {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
                lhs: Err("no value supplied for lhs".to_string()),
            }
        }
    }
    impl ExprUnary {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ExprUnaryExpression>,
            T::Error: std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expression: {}", e));
            self
        }
        pub fn lhs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ExprArgument>,
            T::Error: std::fmt::Display,
        {
            self.lhs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lhs: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ExprUnary> for super::ExprUnary {
        type Error = super::error::ConversionError;
        fn try_from(value: ExprUnary) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
                lhs: value.lhs?,
            })
        }
    }
    impl From<super::ExprUnary> for ExprUnary {
        fn from(value: super::ExprUnary) -> Self {
            Self {
                expression: Ok(value.expression),
                lhs: Ok(value.lhs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InterpolatedWaveform {
        duration: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
        times: Result<super::ParametrizedNumArray, String>,
        values: Result<super::ParametrizedNumArray, String>,
    }
    impl Default for InterpolatedWaveform {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                times: Err("no value supplied for times".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl InterpolatedWaveform {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn times<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNumArray>,
            T::Error: std::fmt::Display,
        {
            self.times = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for times: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNumArray>,
            T::Error: std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InterpolatedWaveform> for super::InterpolatedWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: InterpolatedWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                kind: value.kind?,
                times: value.times?,
                values: value.values?,
            })
        }
    }
    impl From<super::InterpolatedWaveform> for InterpolatedWaveform {
        fn from(value: super::InterpolatedWaveform) -> Self {
            Self {
                duration: Ok(value.duration),
                kind: Ok(value.kind),
                times: Ok(value.times),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KaiserMaxWaveform {
        area: Result<super::ParametrizedNum, String>,
        beta: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
        max_val: Result<super::ParametrizedNum, String>,
    }
    impl Default for KaiserMaxWaveform {
        fn default() -> Self {
            Self {
                area: Err("no value supplied for area".to_string()),
                beta: Err("no value supplied for beta".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                max_val: Err("no value supplied for max_val".to_string()),
            }
        }
    }
    impl KaiserMaxWaveform {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn beta<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.beta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beta: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn max_val<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.max_val = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_val: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<KaiserMaxWaveform> for super::KaiserMaxWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: KaiserMaxWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                beta: value.beta?,
                kind: value.kind?,
                max_val: value.max_val?,
            })
        }
    }
    impl From<super::KaiserMaxWaveform> for KaiserMaxWaveform {
        fn from(value: super::KaiserMaxWaveform) -> Self {
            Self {
                area: Ok(value.area),
                beta: Ok(value.beta),
                kind: Ok(value.kind),
                max_val: Ok(value.max_val),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KaiserWaveform {
        area: Result<super::ParametrizedNum, String>,
        beta: Result<super::ParametrizedNum, String>,
        duration: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
    }
    impl Default for KaiserWaveform {
        fn default() -> Self {
            Self {
                area: Err("no value supplied for area".to_string()),
                beta: Err("no value supplied for beta".to_string()),
                duration: Err("no value supplied for duration".to_string()),
                kind: Err("no value supplied for kind".to_string()),
            }
        }
    }
    impl KaiserWaveform {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn beta<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.beta = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for beta: {}", e));
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<KaiserWaveform> for super::KaiserWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: KaiserWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                beta: value.beta?,
                duration: value.duration?,
                kind: value.kind?,
            })
        }
    }
    impl From<super::KaiserWaveform> for KaiserWaveform {
        fn from(value: super::KaiserWaveform) -> Self {
            Self {
                area: Ok(value.area),
                beta: Ok(value.beta),
                duration: Ok(value.duration),
                kind: Ok(value.kind),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct MappableQubit {
        default_trap: Result<Option<f64>, String>,
        qid: Result<super::QubitId, String>,
    }
    impl Default for MappableQubit {
        fn default() -> Self {
            Self {
                default_trap: Ok(Default::default()),
                qid: Err("no value supplied for qid".to_string()),
            }
        }
    }
    impl MappableQubit {
        pub fn default_trap<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.default_trap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_trap: {}", e));
            self
        }
        pub fn qid<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::QubitId>,
            T::Error: std::fmt::Display,
        {
            self.qid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qid: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MappableQubit> for super::MappableQubit {
        type Error = super::error::ConversionError;
        fn try_from(value: MappableQubit) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_trap: value.default_trap?,
                qid: value.qid?,
            })
        }
    }
    impl From<super::MappableQubit> for MappableQubit {
        fn from(value: super::MappableQubit) -> Self {
            Self {
                default_trap: Ok(value.default_trap),
                qid: Ok(value.qid),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpAddDmmDet {
        dmm_name: Result<super::ChannelName, String>,
        op: Result<String, String>,
        protocol: Result<super::OpAddDmmDetProtocol, String>,
        waveform: Result<super::Waveform, String>,
    }
    impl Default for OpAddDmmDet {
        fn default() -> Self {
            Self {
                dmm_name: Err("no value supplied for dmm_name".to_string()),
                op: Err("no value supplied for op".to_string()),
                protocol: Err("no value supplied for protocol".to_string()),
                waveform: Err("no value supplied for waveform".to_string()),
            }
        }
    }
    impl OpAddDmmDet {
        pub fn dmm_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.dmm_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dmm_name: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn protocol<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OpAddDmmDetProtocol>,
            T::Error: std::fmt::Display,
        {
            self.protocol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol: {}", e));
            self
        }
        pub fn waveform<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Waveform>,
            T::Error: std::fmt::Display,
        {
            self.waveform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for waveform: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpAddDmmDet> for super::OpAddDmmDet {
        type Error = super::error::ConversionError;
        fn try_from(value: OpAddDmmDet) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                dmm_name: value.dmm_name?,
                op: value.op?,
                protocol: value.protocol?,
                waveform: value.waveform?,
            })
        }
    }
    impl From<super::OpAddDmmDet> for OpAddDmmDet {
        fn from(value: super::OpAddDmmDet) -> Self {
            Self {
                dmm_name: Ok(value.dmm_name),
                op: Ok(value.op),
                protocol: Ok(value.protocol),
                waveform: Ok(value.waveform),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpAlign {
        channels: Result<Vec<super::ChannelName>, String>,
        op: Result<String, String>,
    }
    impl Default for OpAlign {
        fn default() -> Self {
            Self {
                channels: Err("no value supplied for channels".to_string()),
                op: Err("no value supplied for op".to_string()),
            }
        }
    }
    impl OpAlign {
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ChannelName>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpAlign> for super::OpAlign {
        type Error = super::error::ConversionError;
        fn try_from(value: OpAlign) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channels: value.channels?,
                op: value.op?,
            })
        }
    }
    impl From<super::OpAlign> for OpAlign {
        fn from(value: super::OpAlign) -> Self {
            Self {
                channels: Ok(value.channels),
                op: Ok(value.op),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpConfigDetMap {
        detuning_map: Result<super::WeightMap, String>,
        dmm_id: Result<super::ChannelId, String>,
        op: Result<String, String>,
    }
    impl Default for OpConfigDetMap {
        fn default() -> Self {
            Self {
                detuning_map: Err("no value supplied for detuning_map".to_string()),
                dmm_id: Err("no value supplied for dmm_id".to_string()),
                op: Err("no value supplied for op".to_string()),
            }
        }
    }
    impl OpConfigDetMap {
        pub fn detuning_map<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::WeightMap>,
            T::Error: std::fmt::Display,
        {
            self.detuning_map = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detuning_map: {}", e));
            self
        }
        pub fn dmm_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelId>,
            T::Error: std::fmt::Display,
        {
            self.dmm_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dmm_id: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpConfigDetMap> for super::OpConfigDetMap {
        type Error = super::error::ConversionError;
        fn try_from(value: OpConfigDetMap) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                detuning_map: value.detuning_map?,
                dmm_id: value.dmm_id?,
                op: value.op?,
            })
        }
    }
    impl From<super::OpConfigDetMap> for OpConfigDetMap {
        fn from(value: super::OpConfigDetMap) -> Self {
            Self {
                detuning_map: Ok(value.detuning_map),
                dmm_id: Ok(value.dmm_id),
                op: Ok(value.op),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpConfigSlm {
        dmm_id: Result<super::ChannelId, String>,
        op: Result<String, String>,
        qubits: Result<Vec<super::QubitId>, String>,
    }
    impl Default for OpConfigSlm {
        fn default() -> Self {
            Self {
                dmm_id: Err("no value supplied for dmm_id".to_string()),
                op: Err("no value supplied for op".to_string()),
                qubits: Err("no value supplied for qubits".to_string()),
            }
        }
    }
    impl OpConfigSlm {
        pub fn dmm_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelId>,
            T::Error: std::fmt::Display,
        {
            self.dmm_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dmm_id: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn qubits<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::QubitId>>,
            T::Error: std::fmt::Display,
        {
            self.qubits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for qubits: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpConfigSlm> for super::OpConfigSlm {
        type Error = super::error::ConversionError;
        fn try_from(value: OpConfigSlm) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                dmm_id: value.dmm_id?,
                op: value.op?,
                qubits: value.qubits?,
            })
        }
    }
    impl From<super::OpConfigSlm> for OpConfigSlm {
        fn from(value: super::OpConfigSlm) -> Self {
            Self {
                dmm_id: Ok(value.dmm_id),
                op: Ok(value.op),
                qubits: Ok(value.qubits),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpDelay {
        channel: Result<super::ChannelName, String>,
        op: Result<String, String>,
        time: Result<super::ParametrizedNum, String>,
    }
    impl Default for OpDelay {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                op: Err("no value supplied for op".to_string()),
                time: Err("no value supplied for time".to_string()),
            }
        }
    }
    impl OpDelay {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpDelay> for super::OpDelay {
        type Error = super::error::ConversionError;
        fn try_from(value: OpDelay) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                op: value.op?,
                time: value.time?,
            })
        }
    }
    impl From<super::OpDelay> for OpDelay {
        fn from(value: super::OpDelay) -> Self {
            Self {
                channel: Ok(value.channel),
                op: Ok(value.op),
                time: Ok(value.time),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpDisableEom {
        channel: Result<super::ChannelName, String>,
        correct_phase_drift: Result<Option<bool>, String>,
        op: Result<String, String>,
    }
    impl Default for OpDisableEom {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                correct_phase_drift: Ok(Default::default()),
                op: Err("no value supplied for op".to_string()),
            }
        }
    }
    impl OpDisableEom {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn correct_phase_drift<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.correct_phase_drift = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for correct_phase_drift: {}",
                    e
                )
            });
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpDisableEom> for super::OpDisableEom {
        type Error = super::error::ConversionError;
        fn try_from(value: OpDisableEom) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                correct_phase_drift: value.correct_phase_drift?,
                op: value.op?,
            })
        }
    }
    impl From<super::OpDisableEom> for OpDisableEom {
        fn from(value: super::OpDisableEom) -> Self {
            Self {
                channel: Ok(value.channel),
                correct_phase_drift: Ok(value.correct_phase_drift),
                op: Ok(value.op),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpEnableEom {
        amp_on: Result<super::ParametrizedNum, String>,
        channel: Result<super::ChannelName, String>,
        correct_phase_drift: Result<Option<bool>, String>,
        detuning_on: Result<super::ParametrizedNum, String>,
        op: Result<String, String>,
        optimal_detuning_off: Result<super::ParametrizedNum, String>,
    }
    impl Default for OpEnableEom {
        fn default() -> Self {
            Self {
                amp_on: Err("no value supplied for amp_on".to_string()),
                channel: Err("no value supplied for channel".to_string()),
                correct_phase_drift: Ok(Default::default()),
                detuning_on: Err("no value supplied for detuning_on".to_string()),
                op: Err("no value supplied for op".to_string()),
                optimal_detuning_off: Err("no value supplied for optimal_detuning_off".to_string()),
            }
        }
    }
    impl OpEnableEom {
        pub fn amp_on<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.amp_on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amp_on: {}", e));
            self
        }
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn correct_phase_drift<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.correct_phase_drift = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for correct_phase_drift: {}",
                    e
                )
            });
            self
        }
        pub fn detuning_on<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.detuning_on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detuning_on: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn optimal_detuning_off<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.optimal_detuning_off = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for optimal_detuning_off: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<OpEnableEom> for super::OpEnableEom {
        type Error = super::error::ConversionError;
        fn try_from(value: OpEnableEom) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                amp_on: value.amp_on?,
                channel: value.channel?,
                correct_phase_drift: value.correct_phase_drift?,
                detuning_on: value.detuning_on?,
                op: value.op?,
                optimal_detuning_off: value.optimal_detuning_off?,
            })
        }
    }
    impl From<super::OpEnableEom> for OpEnableEom {
        fn from(value: super::OpEnableEom) -> Self {
            Self {
                amp_on: Ok(value.amp_on),
                channel: Ok(value.channel),
                correct_phase_drift: Ok(value.correct_phase_drift),
                detuning_on: Ok(value.detuning_on),
                op: Ok(value.op),
                optimal_detuning_off: Ok(value.optimal_detuning_off),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpEomPulse {
        channel: Result<super::ChannelName, String>,
        correct_phase_drift: Result<Option<bool>, String>,
        duration: Result<super::ParametrizedNum, String>,
        op: Result<String, String>,
        phase: Result<super::ParametrizedNum, String>,
        post_phase_shift: Result<super::ParametrizedNum, String>,
        protocol: Result<super::OpEomPulseProtocol, String>,
    }
    impl Default for OpEomPulse {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                correct_phase_drift: Ok(Default::default()),
                duration: Err("no value supplied for duration".to_string()),
                op: Err("no value supplied for op".to_string()),
                phase: Err("no value supplied for phase".to_string()),
                post_phase_shift: Err("no value supplied for post_phase_shift".to_string()),
                protocol: Err("no value supplied for protocol".to_string()),
            }
        }
    }
    impl OpEomPulse {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn correct_phase_drift<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.correct_phase_drift = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for correct_phase_drift: {}",
                    e
                )
            });
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {}", e));
            self
        }
        pub fn post_phase_shift<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.post_phase_shift = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for post_phase_shift: {}",
                    e
                )
            });
            self
        }
        pub fn protocol<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OpEomPulseProtocol>,
            T::Error: std::fmt::Display,
        {
            self.protocol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpEomPulse> for super::OpEomPulse {
        type Error = super::error::ConversionError;
        fn try_from(value: OpEomPulse) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                correct_phase_drift: value.correct_phase_drift?,
                duration: value.duration?,
                op: value.op?,
                phase: value.phase?,
                post_phase_shift: value.post_phase_shift?,
                protocol: value.protocol?,
            })
        }
    }
    impl From<super::OpEomPulse> for OpEomPulse {
        fn from(value: super::OpEomPulse) -> Self {
            Self {
                channel: Ok(value.channel),
                correct_phase_drift: Ok(value.correct_phase_drift),
                duration: Ok(value.duration),
                op: Ok(value.op),
                phase: Ok(value.phase),
                post_phase_shift: Ok(value.post_phase_shift),
                protocol: Ok(value.protocol),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpPhaseShift {
        basis: Result<super::Basis, String>,
        op: Result<String, String>,
        phi: Result<super::ParametrizedNum, String>,
        targets: Result<Vec<super::ParametrizedNum>, String>,
    }
    impl Default for OpPhaseShift {
        fn default() -> Self {
            Self {
                basis: Err("no value supplied for basis".to_string()),
                op: Err("no value supplied for op".to_string()),
                phi: Err("no value supplied for phi".to_string()),
                targets: Err("no value supplied for targets".to_string()),
            }
        }
    }
    impl OpPhaseShift {
        pub fn basis<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Basis>,
            T::Error: std::fmt::Display,
        {
            self.basis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for basis: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn phi<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.phi = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phi: {}", e));
            self
        }
        pub fn targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ParametrizedNum>>,
            T::Error: std::fmt::Display,
        {
            self.targets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for targets: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpPhaseShift> for super::OpPhaseShift {
        type Error = super::error::ConversionError;
        fn try_from(value: OpPhaseShift) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                basis: value.basis?,
                op: value.op?,
                phi: value.phi?,
                targets: value.targets?,
            })
        }
    }
    impl From<super::OpPhaseShift> for OpPhaseShift {
        fn from(value: super::OpPhaseShift) -> Self {
            Self {
                basis: Ok(value.basis),
                op: Ok(value.op),
                phi: Ok(value.phi),
                targets: Ok(value.targets),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpPulse {
        amplitude: Result<super::Waveform, String>,
        channel: Result<super::ChannelName, String>,
        detuning: Result<super::Waveform, String>,
        op: Result<String, String>,
        phase: Result<super::ParametrizedNum, String>,
        post_phase_shift: Result<super::ParametrizedNum, String>,
        protocol: Result<super::OpPulseProtocol, String>,
    }
    impl Default for OpPulse {
        fn default() -> Self {
            Self {
                amplitude: Err("no value supplied for amplitude".to_string()),
                channel: Err("no value supplied for channel".to_string()),
                detuning: Err("no value supplied for detuning".to_string()),
                op: Err("no value supplied for op".to_string()),
                phase: Err("no value supplied for phase".to_string()),
                post_phase_shift: Err("no value supplied for post_phase_shift".to_string()),
                protocol: Err("no value supplied for protocol".to_string()),
            }
        }
    }
    impl OpPulse {
        pub fn amplitude<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Waveform>,
            T::Error: std::fmt::Display,
        {
            self.amplitude = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for amplitude: {}", e));
            self
        }
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn detuning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Waveform>,
            T::Error: std::fmt::Display,
        {
            self.detuning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for detuning: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn phase<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.phase = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for phase: {}", e));
            self
        }
        pub fn post_phase_shift<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.post_phase_shift = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for post_phase_shift: {}",
                    e
                )
            });
            self
        }
        pub fn protocol<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OpPulseProtocol>,
            T::Error: std::fmt::Display,
        {
            self.protocol = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpPulse> for super::OpPulse {
        type Error = super::error::ConversionError;
        fn try_from(value: OpPulse) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                amplitude: value.amplitude?,
                channel: value.channel?,
                detuning: value.detuning?,
                op: value.op?,
                phase: value.phase?,
                post_phase_shift: value.post_phase_shift?,
                protocol: value.protocol?,
            })
        }
    }
    impl From<super::OpPulse> for OpPulse {
        fn from(value: super::OpPulse) -> Self {
            Self {
                amplitude: Ok(value.amplitude),
                channel: Ok(value.channel),
                detuning: Ok(value.detuning),
                op: Ok(value.op),
                phase: Ok(value.phase),
                post_phase_shift: Ok(value.post_phase_shift),
                protocol: Ok(value.protocol),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpTarget {
        channel: Result<super::ChannelName, String>,
        op: Result<String, String>,
        target: Result<super::OpTargetTarget, String>,
    }
    impl Default for OpTarget {
        fn default() -> Self {
            Self {
                channel: Err("no value supplied for channel".to_string()),
                op: Err("no value supplied for op".to_string()),
                target: Err("no value supplied for target".to_string()),
            }
        }
    }
    impl OpTarget {
        pub fn channel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ChannelName>,
            T::Error: std::fmt::Display,
        {
            self.channel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channel: {}", e));
            self
        }
        pub fn op<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.op = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for op: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OpTargetTarget>,
            T::Error: std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpTarget> for super::OpTarget {
        type Error = super::error::ConversionError;
        fn try_from(value: OpTarget) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channel: value.channel?,
                op: value.op?,
                target: value.target?,
            })
        }
    }
    impl From<super::OpTarget> for OpTarget {
        fn from(value: super::OpTarget) -> Self {
            Self {
                channel: Ok(value.channel),
                op: Ok(value.op),
                target: Ok(value.target),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpTargetTarget {
        subtype_0: Result<Option<super::ParametrizedNum>, String>,
        subtype_1: Result<Option<super::ParametrizedNumArray>, String>,
    }
    impl Default for OpTargetTarget {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl OpTargetTarget {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ParametrizedNum>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ParametrizedNumArray>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OpTargetTarget> for super::OpTargetTarget {
        type Error = super::error::ConversionError;
        fn try_from(value: OpTargetTarget) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::OpTargetTarget> for OpTargetTarget {
        fn from(value: super::OpTargetTarget) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ParametrizedNum {
        subtype_0: Result<Option<f64>, String>,
        subtype_1: Result<Option<super::Expression>, String>,
    }
    impl Default for ParametrizedNum {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl ParametrizedNum {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<f64>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Expression>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ParametrizedNum> for super::ParametrizedNum {
        type Error = super::error::ConversionError;
        fn try_from(value: ParametrizedNum) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::ParametrizedNum> for ParametrizedNum {
        fn from(value: super::ParametrizedNum) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ParametrizedNumArray {
        subtype_0: Result<Option<Vec<f64>>, String>,
        subtype_1: Result<Option<super::Expression>, String>,
        subtype_2: Result<Option<super::VariableRef>, String>,
    }
    impl Default for ParametrizedNumArray {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
            }
        }
    }
    impl ParametrizedNumArray {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<f64>>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Expression>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::VariableRef>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ParametrizedNumArray> for super::ParametrizedNumArray {
        type Error = super::error::ConversionError;
        fn try_from(value: ParametrizedNumArray) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
            })
        }
    }
    impl From<super::ParametrizedNumArray> for ParametrizedNumArray {
        fn from(value: super::ParametrizedNumArray) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
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
    pub struct PulserSequence {
        subtype_0: Result<Option<super::PulserSequenceSubtype0>, String>,
        subtype_1: Result<Option<super::PulserSequenceSubtype1>, String>,
    }
    impl Default for PulserSequence {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl PulserSequence {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PulserSequenceSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::PulserSequenceSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PulserSequence> for super::PulserSequence {
        type Error = super::error::ConversionError;
        fn try_from(value: PulserSequence) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::PulserSequence> for PulserSequence {
        fn from(value: super::PulserSequence) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PulserSequenceSubtype0 {
        channels: Result<std::collections::HashMap<String, super::ChannelId>, String>,
        device: Result<super::PulserSequenceSubtype0Device, String>,
        layout: Result<Option<super::Layout>, String>,
        magnetic_field: Result<Option<[f64; 3usize]>, String>,
        measurement: Result<Option<super::Basis>, String>,
        name: Result<String, String>,
        operations: Result<Vec<super::Operation>, String>,
        register: Result<Vec<super::Atom>, String>,
        schema: Result<Option<String>, String>,
        slm_mask_targets: Result<Vec<super::QubitId>, String>,
        variables: Result<std::collections::HashMap<String, super::Variable>, String>,
        version: Result<String, String>,
    }
    impl Default for PulserSequenceSubtype0 {
        fn default() -> Self {
            Self {
                channels: Err("no value supplied for channels".to_string()),
                device: Err("no value supplied for device".to_string()),
                layout: Ok(Default::default()),
                magnetic_field: Ok(Default::default()),
                measurement: Err("no value supplied for measurement".to_string()),
                name: Err("no value supplied for name".to_string()),
                operations: Err("no value supplied for operations".to_string()),
                register: Err("no value supplied for register".to_string()),
                schema: Ok(Default::default()),
                slm_mask_targets: Ok(Default::default()),
                variables: Err("no value supplied for variables".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl PulserSequenceSubtype0 {
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::ChannelId>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn device<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PulserSequenceSubtype0Device>,
            T::Error: std::fmt::Display,
        {
            self.device = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device: {}", e));
            self
        }
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
        pub fn magnetic_field<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<[f64; 3usize]>>,
            T::Error: std::fmt::Display,
        {
            self.magnetic_field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for magnetic_field: {}", e));
            self
        }
        pub fn measurement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Basis>>,
            T::Error: std::fmt::Display,
        {
            self.measurement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measurement: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn operations<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Operation>>,
            T::Error: std::fmt::Display,
        {
            self.operations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operations: {}", e));
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
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn slm_mask_targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::QubitId>>,
            T::Error: std::fmt::Display,
        {
            self.slm_mask_targets = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for slm_mask_targets: {}",
                    e
                )
            });
            self
        }
        pub fn variables<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::Variable>>,
            T::Error: std::fmt::Display,
        {
            self.variables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variables: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PulserSequenceSubtype0> for super::PulserSequenceSubtype0 {
        type Error = super::error::ConversionError;
        fn try_from(value: PulserSequenceSubtype0) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channels: value.channels?,
                device: value.device?,
                layout: value.layout?,
                magnetic_field: value.magnetic_field?,
                measurement: value.measurement?,
                name: value.name?,
                operations: value.operations?,
                register: value.register?,
                schema: value.schema?,
                slm_mask_targets: value.slm_mask_targets?,
                variables: value.variables?,
                version: value.version?,
            })
        }
    }
    impl From<super::PulserSequenceSubtype0> for PulserSequenceSubtype0 {
        fn from(value: super::PulserSequenceSubtype0) -> Self {
            Self {
                channels: Ok(value.channels),
                device: Ok(value.device),
                layout: Ok(value.layout),
                magnetic_field: Ok(value.magnetic_field),
                measurement: Ok(value.measurement),
                name: Ok(value.name),
                operations: Ok(value.operations),
                register: Ok(value.register),
                schema: Ok(value.schema),
                slm_mask_targets: Ok(value.slm_mask_targets),
                variables: Ok(value.variables),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PulserSequenceSubtype1 {
        channels: Result<std::collections::HashMap<String, super::ChannelId>, String>,
        device: Result<super::PulserSequenceSubtype1Device, String>,
        layout: Result<super::Layout, String>,
        magnetic_field: Result<Option<[f64; 3usize]>, String>,
        measurement: Result<Option<super::Basis>, String>,
        name: Result<String, String>,
        operations: Result<Vec<super::Operation>, String>,
        register: Result<Vec<super::MappableQubit>, String>,
        schema: Result<Option<String>, String>,
        slm_mask_targets: Result<Vec<super::QubitId>, String>,
        variables: Result<std::collections::HashMap<String, super::Variable>, String>,
        version: Result<String, String>,
    }
    impl Default for PulserSequenceSubtype1 {
        fn default() -> Self {
            Self {
                channels: Err("no value supplied for channels".to_string()),
                device: Err("no value supplied for device".to_string()),
                layout: Err("no value supplied for layout".to_string()),
                magnetic_field: Ok(Default::default()),
                measurement: Err("no value supplied for measurement".to_string()),
                name: Err("no value supplied for name".to_string()),
                operations: Err("no value supplied for operations".to_string()),
                register: Err("no value supplied for register".to_string()),
                schema: Ok(Default::default()),
                slm_mask_targets: Ok(Default::default()),
                variables: Err("no value supplied for variables".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl PulserSequenceSubtype1 {
        pub fn channels<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::ChannelId>>,
            T::Error: std::fmt::Display,
        {
            self.channels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for channels: {}", e));
            self
        }
        pub fn device<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::PulserSequenceSubtype1Device>,
            T::Error: std::fmt::Display,
        {
            self.device = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device: {}", e));
            self
        }
        pub fn layout<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Layout>,
            T::Error: std::fmt::Display,
        {
            self.layout = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for layout: {}", e));
            self
        }
        pub fn magnetic_field<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<[f64; 3usize]>>,
            T::Error: std::fmt::Display,
        {
            self.magnetic_field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for magnetic_field: {}", e));
            self
        }
        pub fn measurement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Basis>>,
            T::Error: std::fmt::Display,
        {
            self.measurement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for measurement: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn operations<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Operation>>,
            T::Error: std::fmt::Display,
        {
            self.operations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operations: {}", e));
            self
        }
        pub fn register<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::MappableQubit>>,
            T::Error: std::fmt::Display,
        {
            self.register = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for register: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn slm_mask_targets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::QubitId>>,
            T::Error: std::fmt::Display,
        {
            self.slm_mask_targets = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for slm_mask_targets: {}",
                    e
                )
            });
            self
        }
        pub fn variables<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::Variable>>,
            T::Error: std::fmt::Display,
        {
            self.variables = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variables: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PulserSequenceSubtype1> for super::PulserSequenceSubtype1 {
        type Error = super::error::ConversionError;
        fn try_from(value: PulserSequenceSubtype1) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                channels: value.channels?,
                device: value.device?,
                layout: value.layout?,
                magnetic_field: value.magnetic_field?,
                measurement: value.measurement?,
                name: value.name?,
                operations: value.operations?,
                register: value.register?,
                schema: value.schema?,
                slm_mask_targets: value.slm_mask_targets?,
                variables: value.variables?,
                version: value.version?,
            })
        }
    }
    impl From<super::PulserSequenceSubtype1> for PulserSequenceSubtype1 {
        fn from(value: super::PulserSequenceSubtype1) -> Self {
            Self {
                channels: Ok(value.channels),
                device: Ok(value.device),
                layout: Ok(value.layout),
                magnetic_field: Ok(value.magnetic_field),
                measurement: Ok(value.measurement),
                name: Ok(value.name),
                operations: Ok(value.operations),
                register: Ok(value.register),
                schema: Ok(value.schema),
                slm_mask_targets: Ok(value.slm_mask_targets),
                variables: Ok(value.variables),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RampWaveform {
        duration: Result<super::ParametrizedNum, String>,
        kind: Result<String, String>,
        start: Result<super::ParametrizedNum, String>,
        stop: Result<super::ParametrizedNum, String>,
    }
    impl Default for RampWaveform {
        fn default() -> Self {
            Self {
                duration: Err("no value supplied for duration".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                start: Err("no value supplied for start".to_string()),
                stop: Err("no value supplied for stop".to_string()),
            }
        }
    }
    impl RampWaveform {
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for duration: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn start<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.start = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for start: {}", e));
            self
        }
        pub fn stop<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ParametrizedNum>,
            T::Error: std::fmt::Display,
        {
            self.stop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stop: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RampWaveform> for super::RampWaveform {
        type Error = super::error::ConversionError;
        fn try_from(value: RampWaveform) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration: value.duration?,
                kind: value.kind?,
                start: value.start?,
                stop: value.stop?,
            })
        }
    }
    impl From<super::RampWaveform> for RampWaveform {
        fn from(value: super::RampWaveform) -> Self {
            Self {
                duration: Ok(value.duration),
                kind: Ok(value.kind),
                start: Ok(value.start),
                stop: Ok(value.stop),
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
    #[derive(Clone, Debug)]
    pub struct Variable {
        type_: Result<super::VariableType, String>,
        value: Result<Vec<f64>, String>,
    }
    impl Default for Variable {
        fn default() -> Self {
            Self {
                type_: Err("no value supplied for type_".to_string()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Variable {
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VariableType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<f64>>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Variable> for super::Variable {
        type Error = super::error::ConversionError;
        fn try_from(value: Variable) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                type_: value.type_?,
                value: value.value?,
            })
        }
    }
    impl From<super::Variable> for Variable {
        fn from(value: super::Variable) -> Self {
            Self {
                type_: Ok(value.type_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VariableRef {
        variable: Result<super::VariableName, String>,
    }
    impl Default for VariableRef {
        fn default() -> Self {
            Self {
                variable: Err("no value supplied for variable".to_string()),
            }
        }
    }
    impl VariableRef {
        pub fn variable<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::VariableName>,
            T::Error: std::fmt::Display,
        {
            self.variable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variable: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<VariableRef> for super::VariableRef {
        type Error = super::error::ConversionError;
        fn try_from(value: VariableRef) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                variable: value.variable?,
            })
        }
    }
    impl From<super::VariableRef> for VariableRef {
        fn from(value: super::VariableRef) -> Self {
            Self {
                variable: Ok(value.variable),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeightMap {
        slug: Result<Option<String>, String>,
        traps: Result<Vec<super::WeightedTrap>, String>,
    }
    impl Default for WeightMap {
        fn default() -> Self {
            Self {
                slug: Ok(Default::default()),
                traps: Err("no value supplied for traps".to_string()),
            }
        }
    }
    impl WeightMap {
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
        pub fn traps<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::WeightedTrap>>,
            T::Error: std::fmt::Display,
        {
            self.traps = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for traps: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<WeightMap> for super::WeightMap {
        type Error = super::error::ConversionError;
        fn try_from(value: WeightMap) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                slug: value.slug?,
                traps: value.traps?,
            })
        }
    }
    impl From<super::WeightMap> for WeightMap {
        fn from(value: super::WeightMap) -> Self {
            Self {
                slug: Ok(value.slug),
                traps: Ok(value.traps),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeightedTrap {
        weight: Result<f64, String>,
        x: Result<f64, String>,
        y: Result<f64, String>,
    }
    impl Default for WeightedTrap {
        fn default() -> Self {
            Self {
                weight: Err("no value supplied for weight".to_string()),
                x: Err("no value supplied for x".to_string()),
                y: Err("no value supplied for y".to_string()),
            }
        }
    }
    impl WeightedTrap {
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
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
    impl std::convert::TryFrom<WeightedTrap> for super::WeightedTrap {
        type Error = super::error::ConversionError;
        fn try_from(value: WeightedTrap) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                weight: value.weight?,
                x: value.x?,
                y: value.y?,
            })
        }
    }
    impl From<super::WeightedTrap> for WeightedTrap {
        fn from(value: super::WeightedTrap) -> Self {
            Self {
                weight: Ok(value.weight),
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
}
