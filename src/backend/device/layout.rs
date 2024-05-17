use std::sync::Arc;

use serde::Serialize;

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
            slug: "TriangularLatticeLayout(61, 5.0µm)",
        };
        schema.serialize(serializer)
    }
}

#[derive(Serialize)]
struct Schema {
    coordinates: Arc<[[f64; 2]]>,
    slug: &'static str,
}
