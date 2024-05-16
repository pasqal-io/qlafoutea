use super::Constraints;
use serde::Deserialize;
use serde_yaml::with::singleton_map;

/// The (de)serialization format for qubo constraints.
#[derive(Deserialize)]
#[serde(tag = "version")]
pub enum Format {
    /// Version one of the serialization format.
    #[serde(rename = "0.1.0", with = "singleton_map")]
    V1(Constraints),
}

impl Format {
    pub fn as_constraints(&self) -> &Constraints {
        match *self {
            Self::V1(ref c) => c,
        }
    }
}

/*
// Very early draft on Format v2.
impl<'de> serde::Deserialize<'de> for Format {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Aux {
            constraints: Vec<Vec<f64>>,
        }
        let aux = Aux::deserialize(deserializer)?;
        let num_nodes = aux.constraints.len();
        let mut constraints = vec![];
        for (index, line) in aux.constraints.iter().enumerate() {
            if line.len() + index != num_nodes {
                return Err(D::Error::invalid_value(Unexpected::Str(format!("{}", line.iter().format(", ")).as_str()), &format!("since this is a QUBO with {num_nodes} nodes, line {index} should specify {} constraints",
                num_nodes - index).as_str()));
            }
            for i in 1..=index {
                constraints.push(aux.constraints[0][i]);
            }
            constraints.extend(line);
        }
        Ok(Constraints {
            data: constraints,
            num_nodes,
        })
    }
}
 */
