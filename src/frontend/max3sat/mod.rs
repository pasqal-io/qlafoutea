use std::{borrow::Cow, collections::HashMap, fmt::Display, ops::Not, sync::Arc};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{backend::qubo::Constraints, runtime::run::Sample};

pub type Input = Disjunction;

/// A problem formulated as a AND of OR of variables/negated variables.
///
/// The solution space is the mapping between the variables used in the problem
/// and booleans. A solution to the problem is valid iff the disjunction evaluates
/// to `true`.
#[derive(Deserialize, Serialize)]
pub struct Disjunction {
    /// A logical "and" operation between a number of conjunctions.
    ///
    /// It evaluates to `true` for a solution iff all the conjunctions evaluate to `true`.
    pub and: Vec<Conjunction>,
}
impl Disjunction {
    pub fn eval(&self, env: &Env) -> bool {
        self.and.iter().all(|literal| literal.eval(env))
    }
}

/// A logical conjunction, e.g. a OR of variables/negated variables.
#[derive(Deserialize, Serialize)]
pub struct Conjunction {
    /// A logical "or" operation between a number of literals.
    ///
    /// It evaluates to `true` for a solution iff at least one of the literals evaluates to `true`.
    pub or: [Literal; 3],
}
impl Conjunction {
    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.or.iter().map(|literal| &literal.variable)
    }
    pub fn eval(&self, env: &Env) -> bool {
        self.or.iter().any(|literal| literal.eval(env))
    }
}

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
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
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
        self.and.iter().flat_map(|c| c.variables()).dedup()
    }

    pub fn ordered_variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables().sorted().dedup()
    }

    pub fn to_qubo(&self) -> Constraints {
        // Collect variables and assign to each an index.
        let variables: HashMap<&Variable, usize> = self
            .ordered_variables()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        let num_nodes = self.and.len() + variables.len();
        let mut constraints = Constraints::new(num_nodes);
        for (conj_offset, conjunction) in self.and.iter().enumerate() {
            // A conjunction Ci = L1 && L2 && L3 is compiled into
            // - ((1 + Ci)(L1 + L2 + L3) - L1.L2 - L1.L3 - L2.L3 - 2 . Ci)
            // -L1 + -L2 + -L3 + -Ci.L1 + -Ci.L2 + -Ci.L3 + L1.L2 + L1.L3 + L2.L3 + 2.Ci
            let conj_index = conj_offset + self.and.len() - 1;
            let mut delta_conj_var = 2.0;
            eprintln!("conjunction {} => index {}", conj_offset, conj_index);
            for (literal_1_index, literal_1) in conjunction.or.iter().enumerate() {
                let var_1_index = variables.get(&literal_1.variable).cloned().unwrap();
                eprintln!("literal {} => index {}", literal_1.variable.0, var_1_index);

                // Encode term `-Lj`.
                let delta_var_1 = if literal_1.positive { -1.0 } else { 1.0 };
                constraints
                    .delta_at(var_1_index, var_1_index, delta_var_1)
                    .unwrap();

                // Encode term `Lj.Lk` for k > j.
                for literal_2 in conjunction.or.iter().skip(literal_1_index + 1) {
                    let var_2_index = variables.get(&literal_2.variable).cloned().unwrap();
                    let (delta_product_2, delta_var_1, delta_var_2) =
                        match (literal_1.positive, literal_2.positive) {
                            (true, true) => {
                                //Vj.Vk
                                (1.0, 0.0, 0.0)
                            }
                            (true, false) => {
                                // (Vj.(1 - Vk)) = Vj - Vj.Vk
                                (-1.0, 1.0, 0.0)
                            }
                            (false, true) => {
                                // ((1 - Vj).Vk) = Vk - Vj.Vk
                                (-1.0, 0.0, 1.0)
                            }
                            (false, false) => {
                                // (1 - Vj).(1 - Vk) = 1 - Vj - Vk + Vj.Vk
                                (1.0, -1.0, -1.0)
                            }
                        };
                    constraints
                        .delta_at(var_1_index, var_1_index, delta_var_1)
                        .unwrap();
                    constraints
                        .delta_at(var_2_index, var_2_index, delta_var_2)
                        .unwrap();
                    constraints
                        .delta_at(var_1_index, var_2_index, delta_product_2)
                        .unwrap();
                }

                // Encode term `-Ci.Lj`
                eprintln!("product3 {}, {}", var_1_index, conj_index);
                let (delta_prod_3, additional_delta_conj_var) = if literal_1.positive {
                    (-1.0, 0.0)
                } else {
                    (1.0, -1.0)
                };
                constraints
                    .delta_at(var_1_index, conj_index, delta_prod_3)
                    .unwrap();

                delta_conj_var += additional_delta_conj_var;
            }

            // Encode term `2.Ci`
            eprintln!("C[{}]", conj_offset);
            constraints
                .delta_at(conj_index, conj_index, delta_conj_var)
                .unwrap();
        }
        constraints
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
        and: vec![
            Conjunction {
                or: [x_1.positive(), x_2.positive(), x_3.positive()],
            },
            Conjunction {
                or: [x_1.negative(), x_2.positive(), x_3.positive()],
            },
            Conjunction {
                or: [x_1.positive(), x_2.negative(), x_3.positive()],
            },
            Conjunction {
                or: [x_1.negative(), x_2.positive(), x_3.negative()],
            },
        ],
    };
    let constraints = input.to_qubo();
    assert_eq!(constraints.num_nodes(), 7); // 3 SAT variables, 4 terms.
    let expected = Constraints::from_const([
        [0., 0., 0., 0., 0., 0., 0.],
        [-2., 1., 0., 0., 0., 0., 0.],
        [2., 0., -1., 0., 0., 0., 0.],
        [-1., -1., -1., 2., 0., 0., 0.],
        [1., -1., -1., 0., 1., 0., 0.],
        [-1., 1., -1., 0., 0., 1., 0.],
        [1., -1., 1., 0., 0., 0., 0.],
    ]);
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
        for result in results {
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
            if self.eval(&env) {
                println!("=> 1 [PASS]");
            } else {
                println!("=> 0 [FAIL]");
            }
        }
        Ok(())
    }
}
