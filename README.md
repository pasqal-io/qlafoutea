# Qlafoutea

Qlafoutea \kla.fu.ti\ 
- (clafoutis) a delicious crustless pie prepared from pourable batter and black cherries
- (this piece of software) an ongoing experiment on designing a quantum-based calculator 
    for developers and scientists who wish to take advantage of quantum algorithms without
    having to study quantum mechanics.

Qlafoutea was born from the observation that most (if not all) of the tools currently
available to develop quantum algorithms assume:

1. That the user of these tools has achieved PhD-level in quantum mechanics (in addition
    to their own field).
2. That the user of these tools can easily navigate (and generally, cares about) the
    distinction hardware currently available for usage, hardware still in development,
    hardware that only exists on paper, the various kinds of emulators, and general
    quantum hype.
3. That the user of these tools is interested in building algorithms from scratch
    rather than interested in the result or the performance.

## Building & running

You'll need [a recent version of Rust](https://rustup.rs/).

To build & run (and display the command-line options):

```sh
$ cargo run -- --help
```

To run tests:

```sh
$ python3.10 -m venv venv # You only need to do this the first time.
$ . venv/bin/activate
$ cargo test
```


## What it does

As of this writing, qlafoutea supports two features:

1. compiling a source QUBO file;
2. running a compiled QUBO file using its quantum simulator.

To compile

```sh
$ cargo run -- backend path-to-your-source-file.yaml
```

This will produce a compiled file, currently in the same JSON format as used by [Pulser](https://pulser.readthedocs.io/).

To run

```sh
$ cargo run -- run path-to-your-compiled-file.json
```

This will output a CSV files indicating how often each bitstring has been encountered during the execution of the compiled QUBO file.


## Is that it?

Yes, this is a very early stage project, for the time being, this is it.


# What is QUBO? Why QUBO?

[QUBO](https://en.wikipedia.org/wiki/Quadratic_unconstrained_binary_optimization), or Quadratic Unconstrained Binary Optimization, is one optimization problem that has two interesting properties:

1. it is very easy to execute on analog quantum architectures;
2. many problems [can easily be compiled to QUBO](https://blog.xa0.de/post/List-of-QUBO-formulations/).

We figure that this can be a good candidate for an intermediate language for compiling some problems for quantum architectures.
