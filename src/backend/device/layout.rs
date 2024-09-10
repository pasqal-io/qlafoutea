use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Layout {
    pub coordinates: Arc<[[f64; 2]]>,
}

impl Serialize for Layout {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let schema = Schema {
            coordinates: self.coordinates.clone(),
            slug: "TriangularLatticeLayout(61, 5.0µm)".into(),
        };
        schema.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Layout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let schema = Schema::deserialize(deserializer)?;
        Ok(Self {
            coordinates: schema.coordinates,
        })
    }
}

#[derive(Deserialize, Serialize)]
struct Schema {
    coordinates: Arc<[[f64; 2]]>,
    slug: String,
}
