use std::{borrow::Cow, collections::HashMap, fmt::Display, ops::Not, sync::Arc};

use anyhow::anyhow;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    backend::qubo::{Constraints, ConstraintsCollector},
    runtime::run::Sample,
};

#[derive(Deserialize, Serialize, Default)]
pub struct Input {
    #[serde(flatten)]
    conjunction: And,

    #[serde(default)]
    restrict: HashMap<Variable, bool>,
}

/// A problem formulated as a AND of OR of variables/negated variables.
///
/// The solution space is the mapping between the variables used in the problem
/// and booleans. A solution to the problem is valid iff the conjunction evaluates
/// to `true`.
#[derive(Deserialize, Serialize, Default)]
pub struct And {
    /// A logical "and" operation between a number of conjunctions.
    ///
    /// It evaluates to `true` for a solution iff all the conjunctions evaluate to `true`.
    pub and: Vec<Or>,
}
impl And {
    pub fn eval(&self, env: &Env) -> bool {
        self.and.iter().all(|literal| literal.eval(env))
    }
}

/// A logical disjunction, e.g. a OR of variables/negated variables.
#[derive(Deserialize, Serialize)]
pub struct Or {
    /// A logical "or" operation between a number of literals.
    ///
    /// It evaluates to `true` for a solution iff at least one of the literals evaluates to `true`.
    pub or: Vec<Literal>,
}
impl Or {
    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.or.iter().map(|literal| &literal.variable)
    }
    pub fn eval(&self, env: &Env) -> bool {
        self.or.iter().any(|literal| literal.eval(env))
    }
}

#[derive(Clone, Debug)]
/// Either a variable or a negated variable.
pub struct Literal {
    /// The name of the variable.
    pub variable: Variable,

    /// If `true`, this literal represents the variable itself and it evaluates to `true` iff
    /// the variable is assigned to `true`. Respectively, if `false`, this literal represents
    /// the negation of the variable and it evaluates to `true` iff the variable is assigned
    /// to `false`.
    pub positive: bool,
}
impl Literal {
    pub fn eval(&self, env: &Env) -> bool {
        if self.positive {
            self.variable.eval(env)
        } else {
            self.variable.eval(env).not()
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.positive {
            self.variable.0.fmt(f)
        } else {
            write!(f, "!{}", self.variable.0)
        }
    }
}
impl<'de> Deserialize<'de> for Literal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const NOT: &str = "NOT ";
        let text = String::deserialize(deserializer)?;
        let (positive, name) = match text.strip_prefix(NOT) {
            None => (true, Cow::from(text)),
            Some(suffix) => (false, Cow::from(suffix)),
        };
        Ok(Literal {
            positive,
            variable: Variable(name.trim().into()),
        })
    }
}
impl Serialize for Literal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.positive {
            self.variable.0.serialize(serializer)
        } else {
            format!("NOT {}", self.variable.0).serialize(serializer)
        }
    }
}

/// The name of a variable.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Deserialize, Serialize, Debug)]
pub struct Variable(Arc<str>);
impl Variable {
    pub fn positive(&self) -> Literal {
        Literal {
            variable: self.clone(),
            positive: true,
        }
    }

    pub fn negative(&self) -> Literal {
        Literal {
            variable: self.clone(),
            positive: false,
        }
    }

    pub fn eval(&self, env: &Env) -> bool {
        *env.0.get(self).unwrap()
    }
}

pub struct Env(HashMap<Variable, bool>);

impl Input {
    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.conjunction
            .and
            .iter()
            .flat_map(|c| c.variables())
            .dedup()
    }

    pub fn ordered_variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables().sorted().dedup()
    }

    pub fn to_qubo(&self) -> Result<Constraints, anyhow::Error> {
        // Collect variables and assign to each an index.
        let variables: HashMap<&Variable, usize> = self
            .ordered_variables()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();

        // Original variables.
        let mut names = self
            .ordered_variables()
            .map(|var| var.0.clone())
            .collect_vec();

        let mut collector = ConstraintsCollector::new(variables.len());
        for disjunction in self.conjunction.and.iter() {
            let mut indexed: arrayvec::ArrayVec<(Literal, usize), 4> = disjunction
                .or
                .iter()
                .map(|lit| {
                    let index = variables
                        .get(&lit.variable)
                        .cloned()
                        .expect("Missing variable");
                    (lit.clone(), index)
                })
                .collect();
            let self_constraint = |collector: &mut ConstraintsCollector,
                                   indexed: &[(Literal, usize)],
                                   local_index: usize,
                                   multiplier: f64| {
                let (lit, index) = &indexed[local_index];
                if lit.positive {
                    collector.add_constraint(*index, *index, multiplier);
                } else {
                    collector.add_constraint(*index, *index, -multiplier);
                }
            };
            let cross_constraint = |collector: &mut ConstraintsCollector,
                                    indexed: &[(Literal, usize)],
                                    i: usize,
                                    j: usize,
                                    multiplier: f64| {
                let (left_lit, left_index) = &indexed[i];
                let (right_lit, right_index) = &indexed[j];
                if left_lit.positive == right_lit.positive {
                    collector.add_constraint(*left_index, *right_index, multiplier);
                } else {
                    collector.add_constraint(*left_index, *right_index, -multiplier);
                }
            };
            match disjunction.or.len() {
                1 => {
                    // Single literal L.
                    //
                    // Any solution to 3SAT must first maximize L, so encode L as itself.
                    self_constraint(&mut collector, &indexed, 0, 1.0);
                }
                2 => {
                    // A disjunction K = L1 \/ L2
                    //
                    // Any solution to 3SAT must first maximize  L1 + L2 - L1.L2

                    // Encode L1 + L2
                    for i in 0..indexed.len() {
                        self_constraint(&mut collector, &indexed, i, 1.0);
                    }
                    // Encode - L1.L2
                    cross_constraint(&mut collector, &indexed, 0, 1, -1.0);
                }
                3 => {
                    // A disjunction K = L1 \/ L2 \/ L3
                    //
                    // K == true has the same truth table as
                    // K' = L1 + L2 + L3 - L1.L2 - L2.L3 - L1.L3 + L1.L2.L3 == 1
                    //
                    // Finding a solution to K == true is therefore equivalent to maximizing K'
                    //
                    // We introduce a new variable C. This is equivalent to maximizing
                    // K' = L1 + L2 + L3 - L1.L2 - L2.L3 - L1.L3 + C.(L1 + L2 + L3 - 2)
                    //
                    // or, equivalently
                    //      L1 + L2 + L3 - L1.L2 - L2.L3 - L1.L3 + C.L1 + C.L2 + C.L3 - 2.C

                    // Encode Li
                    for i in 0..indexed.len() {
                        self_constraint(&mut collector, &indexed, i, 1.0);
                    }

                    // Encode -Li.Lj
                    for i in 0..indexed.len() {
                        for j in i + 1..indexed.len() {
                            cross_constraint(&mut collector, &indexed, i, j, -1.0);
                        }
                    }

                    // Introduce variable C.
                    //
                    // Let's not do it before encoding Li or -Li.Lj otherwise we'll also add additional constraints.
                    let c_index = collector.add_variable();
                    let c_name: Arc<str> = format!("w_{}", c_index).into();
                    let c_literal = Literal {
                        positive: true,
                        variable: Variable(c_name.clone()),
                    };
                    let c_local_index = indexed.len();
                    indexed.push((c_literal, c_index));
                    assert_eq!(indexed.len(), 4);
                    eprintln!("Created new variable {c_name} with local index {c_local_index}, global index {c_index}");

                    // Encode + C.Li
                    names.push(c_name);
                    for i in 0..indexed.len() {
                        cross_constraint(&mut collector, &indexed, i, c_local_index, 1.0);
                    }

                    // Encode -2.C
                    self_constraint(&mut collector, &indexed, c_local_index, -2.0);
                }
                _ => {
                    return Err(anyhow!(
                        "Invalid size, expected 1-3, got {}",
                        disjunction.or.len()
                    ));
                }
            }
        }
        Ok(collector.collect(names))
    }
}

/// Test against the sample at https://canvas.auckland.ac.nz/courses/14782/files/574983/download?verifier=1xqRikUjTEBwm8PnObD8YVmKdeEhZ9Ui8axW8HwP&wrap=1
#[test]
fn test_to_qubo() {
    //(x_1 OR x_2 OR x_3) AND (NEG(x_1) OR x_2 OR x_3) AND (x_1 OR NEG(x_2) OR x_3) AND (NEG(x_1) OR x_2 OR NEG(x_3))
    let x_1 = Variable("x_1".into());
    let x_2 = Variable("x_2".into());
    let x_3 = Variable("x_3".into());
    let input = Input {
        conjunction: And {
            and: vec![
                Or {
                    or: [x_1.positive(), x_2.positive(), x_3.positive()].into(),
                },
                Or {
                    or: [x_1.negative(), x_2.positive(), x_3.positive()].into(),
                },
                Or {
                    or: [x_1.positive(), x_2.negative(), x_3.positive()].into(),
                },
                Or {
                    or: [x_1.negative(), x_2.positive(), x_3.negative()].into(),
                },
            ],
        },
        ..Default::default()
    };
    let constraints = input.to_qubo().unwrap();
    assert_eq!(constraints.num_nodes(), 7); // 3 SAT variables, 4 terms.
    let expected = Constraints::from_const(
        [
            [0., 0., 0., 0., 0., 0., 0.],
            [-2., 1., 0., 0., 0., 0., 0.],
            [2., 0., -1., 0., 0., 0., 0.],
            [-1., -1., -1., 2., 0., 0., 0.],
            [1., -1., -1., 0., 1., 0., 0.],
            [-1., 1., -1., 0., 0., 1., 0.],
            [1., -1., 1., 0., 0., 0., 0.],
        ],
        vec![
            "x1".to_string().into(),
            "x2".to_string().into(),
            "x3".to_string().into(),
            "_0".to_string().into(),
            "_1".to_string().into(),
            "_2".to_string().into(),
            "_3".to_string().into(),
        ],
    );
    for i in 0..constraints.num_nodes() {
        for j in 0..constraints.num_nodes() {
            assert_eq!(
                constraints.at(i, j).unwrap(),
                expected.at(i, j).unwrap(),
                "({}, {})",
                i,
                j
            );
        }
    }
}

impl Input {
    /// Do... something with the results.
    pub fn handle_results(&self, results: &[Sample]) -> Result<(), anyhow::Error> {
        assert!(!results.is_empty());
        let variables = self.ordered_variables().collect_vec();
        let mut env = Env(HashMap::new());
        'per_result: for result in results {
            for (c, var) in result.bitstring.chars().zip(variables.iter()) {
                let measure = c == '1';
                if let Some(restriction) = self.restrict.get(var) {
                    if measure != *restriction {
                        continue 'per_result;
                    }
                }
            }
            eprintln!("Instances {}", result.instances);
            let mut writer = csv::Writer::from_writer(std::io::stdout());
            for (c, var) in result.bitstring.chars().zip(variables.iter()) {
                // Show result.
                #[derive(Serialize)]
                struct Record<'a> {
                    variable: &'a str,
                    value: char,
                }
                let record = Record {
                    variable: var.0.as_ref(),
                    value: c,
                };
                writer.serialize(record)?;

                // Prepare to check result.
                env.0.insert((*var).clone(), c == '1');
            }

            // Double-check that the result is actually meaningful.
            if self.conjunction.eval(&env) {
                println!("=> 1 [PASS]");
            } else {
                println!("=> 0 [FAIL]");
            }
        }
        Ok(())
    }
}
