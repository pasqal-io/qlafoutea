//! Information on a target device.

pub mod c6;
pub mod layout;
use serde::Serialize;

use layout::Layout;

use crate::pulser::device::{ChannelId, PhysicalChannel, RydbergBeam, RydbergEom};

pub struct Device {
    interaction_coeff: c6::C6Coeff,
    dimensions: u32,
    rydberg_level: u32,
    max_atom_num: u32,
    max_radial_distance: u32,
    min_atom_distance: f64,
    max_sequence_duration: u32,
    is_virtual: bool,
    max_layout_filling: f64,
    name: &'static str,
    channels: Vec<PhysicalChannel>,
    pre_calibrated_layouts: Vec<Layout>,
}
impl Device {
    pub fn interaction_coeff(&self) -> c6::C6Coeff {
        self.interaction_coeff
    }
}

impl Serialize for Device {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            version: "1",
            dimensions: self.dimensions,
            max_atom_num: self.max_atom_num,
            rydberg_level: self.rydberg_level,
            max_radial_distance: self.max_radial_distance,
            min_atom_distance: self.min_atom_distance,
            max_sequence_duration: self.max_sequence_duration,
            is_virtual: self.is_virtual,
            max_layout_filling: self.max_layout_filling,
            name: self.name,
            channels: self.channels.clone(),
            pre_calibrated_layouts: self.pre_calibrated_layouts.clone(),
            interaction_coeff_xy: None,
            reusable_channels: false,
            supports_slm_mask: false,
        };
        schema.serialize(serializer)
    }
}

#[derive(Serialize)]
struct Schema {
    version: &'static str,
    dimensions: u32,
    rydberg_level: u32,
    max_atom_num: u32,
    max_radial_distance: u32,
    min_atom_distance: f64,
    max_sequence_duration: u32,
    is_virtual: bool,
    max_layout_filling: f64,
    name: &'static str,
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

        Self {
            interaction_coeff,
            dimensions: 2,
            rydberg_level,
            max_atom_num: 25,
            max_radial_distance: 35,
            min_atom_distance: 5.,
            max_sequence_duration: 4_000,
            is_virtual: false,
            max_layout_filling: 0.5,
            name: "AnalogDevice",
            channels,
            pre_calibrated_layouts,
        }
    }
}
