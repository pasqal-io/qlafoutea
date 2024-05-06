//! Information on a target device.

pub mod c6;

use serde::de::{Error, Unexpected};
use serde::Deserialize;

use crate::pulser::device_schema::{self, ChannelId, PhysicalChannel, RydbergBeam, RydbergEom};
use crate::pulser::layout_schema::Layout;

pub struct Device {
    interaction_coeff: c6::C6Coeff,
    abstract_repr: device_schema::Device,
}
impl Device {
    pub fn interaction_coeff(&self) -> c6::C6Coeff {
        self.interaction_coeff
    }
    pub fn as_abstract_repr(&self) -> device_schema::Device {
        self.abstract_repr.clone()
    }
}

impl Device {
    pub fn analog() -> Self {
        // All this is ported from Pulser's `AnalogDevice` hard-coded implementation.
        let abstract_repr = device_schema::Device::Variant0 {
            version: "1".to_string(),
            schema: None,
            dimensions: device_schema::DeviceVariant0Dimensions::try_from(2.0).unwrap(),
            rydberg_level: 60.,
            max_atom_num: 25.,
            max_radial_distance: 35.,
            min_atom_distance: 5.,
            max_sequence_duration: Some(4_000.),
            interaction_coeff_xy: None,
            max_runs: None,
            is_virtual: false,
            max_layout_filling: 0.5,
            name: "AnalogDevice".to_string(),
            reusable_channels: false,
            supports_slm_mask: false,
            dmm_objects: vec![],
            channels: vec![PhysicalChannel::Variant0 {
                addressing: "Global".to_string(),
                basis: "ground-rydberg".to_string(),
                clock_period: 4.0,
                id: ChannelId("rydberg_global".to_string()),
                max_targets: (),
                fixed_retarget_t: (),
                max_abs_detuning: 40.0 * std::f64::consts::PI,
                max_amp: 4.0 * std::f64::consts::PI,
                min_duration: 16.0,
                mod_bandwidth: Some(8.0),
                max_duration: 100000000.0,
                eom_config: Some(RydbergEom {
                    limiting_beam: RydbergBeam::Red,
                    max_limiting_amp: 60.0 * std::f64::consts::PI,
                    controlled_beams: vec![RydbergBeam::Blue],
                    custom_buffer_time: Some(240.0),
                    intermediate_detuning: 900.0 * std::f64::consts::PI,
                    mod_bandwidth: 40.0,
                    multiple_beam_control: Some(true),
                }),
                min_avg_amp: None,
                min_retarget_interval: (),
            }],
            pre_calibrated_layouts: vec![Layout {
                coordinates: vec![
                    [-20.0, 0.0],
                    [-17.5, -4.330127],
                    [-17.5, 4.330127],
                    [-15.0, -8.660254],
                    [-15.0, 0.0],
                    [-15.0, 8.660254],
                    [-12.5, -12.990381],
                    [-12.5, -4.330127],
                    [-12.5, 4.330127],
                    [-12.5, 12.990381],
                    [-10.0, -17.320508],
                    [-10.0, -8.660254],
                    [-10.0, 0.0],
                    [-10.0, 8.660254],
                    [-10.0, 17.320508],
                    [-7.5, -12.990381],
                    [-7.5, -4.330127],
                    [-7.5, 4.330127],
                    [-7.5, 12.990381],
                    [-5.0, -17.320508],
                    [-5.0, -8.660254],
                    [-5.0, 0.0],
                    [-5.0, 8.660254],
                    [-5.0, 17.320508],
                    [-2.5, -12.990381],
                    [-2.5, -4.330127],
                    [-2.5, 4.330127],
                    [-2.5, 12.990381],
                    [0.0, -17.320508],
                    [0.0, -8.660254],
                    [0.0, 0.0],
                    [0.0, 8.660254],
                    [0.0, 17.320508],
                    [2.5, -12.990381],
                    [2.5, -4.330127],
                    [2.5, 4.330127],
                    [2.5, 12.990381],
                    [5.0, -17.320508],
                    [5.0, -8.660254],
                    [5.0, 0.0],
                    [5.0, 8.660254],
                    [5.0, 17.320508],
                    [7.5, -12.990381],
                    [7.5, -4.330127],
                    [7.5, 4.330127],
                    [7.5, 12.990381],
                    [10.0, -17.320508],
                    [10.0, -8.660254],
                    [10.0, 0.0],
                    [10.0, 8.660254],
                    [10.0, 17.320508],
                    [12.5, -12.990381],
                    [12.5, -4.330127],
                    [12.5, 4.330127],
                    [12.5, 12.990381],
                    [15.0, -8.660254],
                    [15.0, 0.0],
                    [15.0, 8.660254],
                    [17.5, -4.330127],
                    [17.5, 4.330127],
                    [20.0, 0.0],
                ],
                slug: Some("TriangularLatticeLayout(61, 5.0µm)".to_string()),
            }],
        };
        Self::from_abstract_repr(abstract_repr).unwrap()
    }

    fn from_abstract_repr(
        abstract_repr: device_schema::Device,
    ) -> Result<Self, (serde::de::Unexpected<'static>, String)> {
        let rydberg_level_f64 = match abstract_repr {
            device_schema::Device::Variant1 { rydberg_level, .. } => rydberg_level,
            device_schema::Device::Variant0 { rydberg_level, .. } => rydberg_level,
        };
        if !rydberg_level_f64.is_normal()
            || rydberg_level_f64.is_sign_negative()
            || rydberg_level_f64.round() != rydberg_level_f64
        {
            return Err((
                Unexpected::Float(rydberg_level_f64),
                "non-negative integer rydberg_level".to_string(),
            ));
        }
        let rydberg_level_u32 = rydberg_level_f64 as u32;
        let interaction_coeff = match c6::C6Coeff::new(rydberg_level_u32) {
            None => {
                return Err((
                    Unexpected::Float(rydberg_level_f64),
                    format!("a value in [{}, {}]", c6::C6Coeff::MIN, c6::C6Coeff::MAX),
                ))
            }
            Some(coeff) => coeff,
        };
        Ok(Device {
            interaction_coeff,
            abstract_repr,
        })
    }
}

impl<'de> Deserialize<'de> for Device {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let abstract_repr = device_schema::Device::deserialize(deserializer)?;
        Self::from_abstract_repr(abstract_repr)
            .map_err(|(unexpected, message)| D::Error::invalid_value(unexpected, &message.as_str()))
    }
}
