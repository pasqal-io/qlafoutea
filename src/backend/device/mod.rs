//! Information on a target device.

pub mod c6;
pub mod layout;

use c6::C6Coeff;
use serde::{Deserialize, Serialize};

use layout::Layout;

use crate::backend::pulser::device::{ChannelId, PhysicalChannel, RydbergBeam, RydbergEom};

pub struct Device {
    interaction_coeff: c6::C6Coeff,
    dimensions: u32,
    rydberg_level: u32,
    max_atom_num: u32,

    /// Max distance to the center of the board, in um.
    max_radial_distance_um: u32,
    max_sq_distance_to_center_um_sq: f64,
    min_atom_distance: f64,
    max_sequence_duration: u32,
    is_virtual: bool,
    max_layout_filling: f64,
    name: String,
    channels: Vec<PhysicalChannel>,
    pre_calibrated_layouts: Vec<Layout>,
}
impl Device {
    pub fn interaction_coeff(&self) -> c6::C6Coeff {
        self.interaction_coeff
    }
    pub fn max_sq_distance_to_center(&self) -> f64 {
        self.max_sq_distance_to_center_um_sq
    }
    pub fn min_atom_distance(&self) -> f64 {
        self.min_atom_distance
    }
    pub fn rydberg_level(&self) -> u32 {
        self.rydberg_level
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            version: "1".into(),
            dimensions: self.dimensions,
            max_atom_num: self.max_atom_num,
            rydberg_level: self.rydberg_level,
            max_radial_distance: self.max_radial_distance_um,
            min_atom_distance: self.min_atom_distance,
            max_sequence_duration: self.max_sequence_duration,
            is_virtual: self.is_virtual,
            max_layout_filling: self.max_layout_filling,
            name: self.name.clone(),
            channels: self.channels.clone(),
            pre_calibrated_layouts: self.pre_calibrated_layouts.clone(),
            interaction_coeff_xy: None,
            reusable_channels: false,
            supports_slm_mask: false,
        };
        schema.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Device {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        Ok(Self {
            dimensions: schema.dimensions,
            max_atom_num: schema.max_atom_num,
            rydberg_level: schema.rydberg_level,
            max_radial_distance_um: schema.max_radial_distance,
            min_atom_distance: schema.min_atom_distance,
            max_sequence_duration: schema.max_sequence_duration,
            is_virtual: schema.is_virtual,
            max_layout_filling: schema.max_layout_filling,
            name: schema.name,
            channels: schema.channels.clone(),
            pre_calibrated_layouts: schema.pre_calibrated_layouts.clone(),
            interaction_coeff: C6Coeff::new(schema.rydberg_level).unwrap(),
            max_sq_distance_to_center_um_sq: u64::pow(schema.max_radial_distance as u64, 2) as f64,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Schema {
    version: String,
    dimensions: u32,
    rydberg_level: u32,
    max_atom_num: u32,
    max_radial_distance: u32,
    min_atom_distance: f64,
    max_sequence_duration: u32,
    is_virtual: bool,
    max_layout_filling: f64,
    name: String,
    channels: Vec<PhysicalChannel>,
    pre_calibrated_layouts: Vec<Layout>,
    interaction_coeff_xy: Option<f64>,
    reusable_channels: bool,
    supports_slm_mask: bool,
}

impl Device {
    pub fn analog() -> Self {
        // All this is ported from Pulser's `AnalogDevice` hard-coded implementation.
        let rydberg_level = 60;
        let interaction_coeff = c6::C6Coeff::new(rydberg_level).unwrap();

        let pre_calibrated_layouts = vec![Layout {
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
            ]
            .into(),
        }];
        let channels = vec![PhysicalChannel::Variant0 {
            addressing: "Global".to_string(),
            basis: "ground-rydberg".to_string(),
            clock_period: 4.0,
            id: ChannelId("ising".to_string()),
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
        }];

        let max_radial_distance_um = 35;
        let max_sq_distance_to_center_um_sq = (max_radial_distance_um as f64).powi(2);
        Self {
            interaction_coeff,
            dimensions: 2,
            rydberg_level,
            max_atom_num: 25,
            max_radial_distance_um,
            min_atom_distance: 5.,
            max_sequence_duration: 4_000,
            max_sq_distance_to_center_um_sq,
            is_virtual: false,
            max_layout_filling: 0.5,
            name: "AnalogDevice".into(),
            channels,
            pre_calibrated_layouts,
        }
    }
}
