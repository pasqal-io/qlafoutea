#![allow(dead_code)]
#![allow(clippy::unused_unit)]

use anyhow::Context;
use wasmtime::{Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc, Val};

use crate::backend::pulser::sequence::Sequence;

// Bindings for Pulser studio.

pub struct Runner {
    engine: Engine,
    module: Module,
}
impl Runner {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut config = Config::new();
        #[cfg(debug_assertions)]
        {
            config.debug_info(true).wasm_backtrace(true);
        }

        let engine = Engine::new(&config)
            .context("Could not initialize engine to compile Pulser studio simulator")?;

        // FIXME: Cache compilation.
        let module = Module::new(&engine, include_bytes!("pulser_wasm_bg.wasm"))
            .context("Failed to compile Pulser studio simulator")?;
        Ok(Runner { engine, module })
    }
    pub fn simulator(&self) -> Result<Simulator, anyhow::Error> {
        //let _foo = || unimplemented!();
        let mut linker = Linker::new(&self.engine);

        /*
          (import "wbg" "__wbg_new_d3138911a89329b0" (func (;7;) (type 9)))
          (import "wbg" "__wbg_buffer_5e74a88a1424a2e0" (func (;9;) (type 3)))
          (import "wbg" "__wbg_newwithbyteoffsetandlength_f6c2c5e40f6f5bda" (func (;10;) (type 2)))
          (import "wbg" "__wbg_new_86a3fd385f9bcaf2" (func (;11;) (type 3)))
          (import "wbg" "__wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f" (func (;12;) (type 2)))
          (import "wbg" "__wbg_new_f5438c0cea22a3aa" (func (;13;) (type 3)))
          (import "wbg" "__wbindgen_throw" (func (;14;) (type 1)))
          (import "wbg" "__wbindgen_memory" (func (;15;) (type 9))
        */

        linker.func_wrap(
            "wbg",
            "__wbindgen_string_new",
            |mut _caller: Caller<'_, State>, _a: i32, _b: i32| -> () {
                unimplemented!();
            },
        )?;

        linker.func_wrap(
            "wbg",
            "__wbg_set_327aa1a19c3f2018",
            |_: i32, _: i32, _: i32| -> i32 {
                //getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
                unimplemented!();
            },
        )?;

        linker.func_wrap(
            "wbg",
            "__wbindgen_object_drop_ref",
            |mut caller: Caller<'_, State>, index: i32| {
                caller.data_mut().drop_ref(index);
            },
        )?;
        linker.func_wrap(
            "wbg",
            "__wbindgen_number_new",
            |mut caller: Caller<'_, State>, value: f64| -> i32 {
                caller
                    .data_mut()
                    .add_heap_object(Value::Wasm(Val::F64(f64::to_bits(value))))
                    as i32
            },
        )?;
        linker.func_wrap(
            "wbg",
            "__wbindgen_object_clone_ref",
            |mut caller: Caller<'_, State>, idx: i32| -> i32 {
                let value = caller.data_mut().heap[idx as usize].unwrap();
                caller.data_mut().add_heap_object(value) as i32
            },
        )?;
        linker.func_wrap(
            "wbg",
            "__wbg_new_693216e109162396",
            |mut caller: Caller<'_, State>| -> i32 {
                caller.data_mut().add_heap_object(Value::Error) as i32
            },
        )?;
        linker.func_wrap(
            "wbg",
            "__wbg_stack_0ddaca5d1abfb52f",
            |_caller: Caller<'_, State>, _idx: i32, _offset: i32| -> () {
                /*
                       var ret = getObject(arg1).stack;
                       var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                       var len0 = WASM_VECTOR_LEN;
                       getInt32Memory0()[arg0 / 4 + 1] = len0;
                       getInt32Memory0()[arg0 / 4 + 0] = ptr0;
                */
                // As far as I can tell, this stores the stack of a JS error in memory that we never actually
                // look at. Let's see what happens if we do nothing here!
                panic!("__wbg_stack_0ddaca5d1abfb52f");
            },
        )?;

        linker.func_wrap(
            "wbg",
            "__wbg_error_09919627ac0992f5",
            |_caller: Caller<'_, State>, _idx: i32, _offset: i32| -> () {
                // This should probably just display an error.
                panic!("__wbg_error_09919627ac0992f5");
            },
        )?;

        let mut store = Store::new(&self.engine, State::new());
        let instance = linker
            .instantiate(&mut store, &self.module)
            .context("Failed to initialize instance of Pulser studio simulator")?;
        let exports = Exports {
            __wbg_simbuilder_free: instance
                .get_typed_func(&mut store, "__wbg_simbuilder_free")
                .unwrap(),
            simbuilder_new: instance
                .get_typed_func(&mut store, "simbuilder_new")
                .unwrap(),
            simbuilder_set_step_time: instance
                .get_typed_func(&mut store, "simbuilder_set_step_time")
                .unwrap(),
            simbuilder_set_rydberg_level: instance
                .get_typed_func(&mut store, "simbuilder_set_rydberg_level")
                .unwrap(),
            simbuilder_add_atom: instance
                .get_typed_func(&mut store, "simbuilder_add_atom")
                .unwrap(),
            simbuilder_add_run: instance
                .get_typed_func(&mut store, "simbuilder_add_run")
                .unwrap(),
            simbuilder_build: instance
                .get_typed_func(&mut store, "simbuilder_build")
                .unwrap(),
            __wbg_simdata_free: instance
                .get_typed_func(&mut store, "__wbg_simdata_free")
                .unwrap(),
            simdata_step: instance.get_typed_func(&mut store, "simdata_step").unwrap(),
            simdata_get_data: instance
                .get_typed_func(&mut store, "simdata_get_data")
                .unwrap(),
            __wbindgen_malloc: instance
                .get_typed_func(&mut store, "__wbindgen_malloc")
                .unwrap(),
            __wbindgen_free: instance
                .get_typed_func(&mut store, "__wbindgen_free")
                .unwrap(),
            __wbindgen_realloc: instance
                .get_typed_func(&mut store, "__wbindgen_realloc")
                .unwrap(),
        };
        Simulator::new(instance, exports, store)
    }
}

#[derive(Clone)]
enum Value {
    Wasm(Val),
    Error, // FIXME: This might need a property `stack`.
}

enum Cell {
    Empty { next: usize },
    Full(Value),
}
impl Cell {
    pub fn unwrap(&self) -> Value {
        match self {
            Cell::Full(val) => val.clone(),
            _ => panic!(),
        }
    }
}
struct State {
    heap: Vec<Cell>,
    next: usize,
}
impl State {
    pub fn new() -> Self {
        // heap.push(undefined, null, true, false);
        let heap = Vec::with_capacity(32);
        State {
            next: heap.len(),
            heap,
        }
    }
    pub fn drop_ref(&mut self, index: i32) -> Value {
        let index = index as usize;
        let cell = &mut self.heap[index];
        if index < 36 {
            // Hardcoded value, generated by wasm-bindgen
            return cell.unwrap();
        }
        let mut swap = Cell::Empty { next: self.next };
        std::mem::swap(cell, &mut swap);
        self.next = index;
        cell.unwrap()
    }
    pub fn add_heap_object(&mut self, value: Value) -> usize {
        if self.next == self.heap.len() {
            self.heap.push(Cell::Empty {
                next: self.heap.len() + 1,
            });
        }
        let idx = self.next;
        match self.heap[idx] {
            Cell::Full(_) => panic!(),
            Cell::Empty { next } => {
                self.heap[idx] = Cell::Full(value);
                self.next = next;
            }
        }
        idx
    }
    pub fn take_object(&mut self, index: i32) -> Value {
        self.drop_ref(index)
    }
}

struct Exports {
    /// func 118
    ///
    /// simbuilder:InnerRef -> void
    __wbg_simbuilder_free: TypedFunc<i32, ()>,

    /// func 101
    ///
    /// void -> simbuilder:InnerRef
    simbuilder_new: TypedFunc<(), i32>,

    /// func 157
    ///
    /// simbuilder:InnerRef, f32 -> void
    simbuilder_set_step_time: TypedFunc<(i32, f32), ()>,

    /// func 143
    ///
    /// simbuilder:InnerRef, level:i32 -> void
    simbuilder_set_rydberg_level: TypedFunc<(i32, i32), ()>,

    /// func 112
    ///
    /// simbuilder:InnerRef, x:f32, y:f32 -> void
    simbuilder_add_atom: TypedFunc<(i32, f32, f32), ()>,

    /// func 107
    ///
    /// simbuilder:InnerRef, basis:i32 !isLikeNone:i32 address||0:i32 f32ArrayPtr:i32 WASM_VECTOR_LEN:i32 -> void
    simbuilder_add_run: TypedFunc<(i32, i32, i32, i32, i32, i32), ()>,

    /// func 58
    ///
    /// simbuilder:InnerRef -> simdata:InnerRef
    simbuilder_build: TypedFunc<i32, i32>,

    /// func 36
    ///
    /// simdata:InnerRef -> void
    __wbg_simdata_free: TypedFunc<i32, ()>,

    /// func 77
    ///
    /// simdata:InnerRef, microsteps:i32 -> bool
    simdata_step: TypedFunc<(i32, i32), i32>,

    /// func 148
    ///
    /// simdata:InnerRef, step:i32 -> InnerRef:SimulationData
    simdata_get_data: TypedFunc<(i32, i32), i32>,

    /// func 158
    ///
    /// size:i32 -> InnerRef
    __wbindgen_malloc: TypedFunc<i32, i32>,

    /// func 197
    ///
    /// InnerRef:i32, len:i32 -> void
    __wbindgen_free: TypedFunc<(i32, i32), ()>,

    /// func 175
    ///
    /// i32, i32, i32 -> i32
    __wbindgen_realloc: TypedFunc<(i32, i32, i32), i32>,
}

pub struct Simulator {
    instance: Instance,
    exports: Exports,
    store: Store<State>,
    /// The in-store address for the builder.
    builder_addr: i32,
}
impl Simulator {
    fn new(
        instance: Instance,
        exports: Exports,
        mut store: Store<State>,
    ) -> Result<Self, anyhow::Error> {
        let builder_addr = exports.simbuilder_new.call(&mut store, ())?;
        Ok(Self {
            instance,
            exports,
            store,
            builder_addr,
        })
    }

    pub fn simulate_sequence(&mut self, sequence: Sequence) -> Result<(), anyhow::Error> {
        self.exports.simbuilder_set_rydberg_level.call(
            &mut self.store,
            (self.builder_addr, sequence.device().rydberg_level() as i32),
        )?;
        for atom in sequence.register().coordinates.as_ref() {
            self.exports.simbuilder_add_atom.call(
                &mut self.store,
                (
                    self.builder_addr,
                    atom.0.x.into_inner() as f32,
                    atom.0.y.into_inner() as f32,
                ),
            )?;
        }
        unimplemented!()
    }
}
