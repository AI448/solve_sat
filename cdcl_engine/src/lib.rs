// #![feature(try_trait_v2)]

mod core_engine;
mod engine;
mod outer_engine;
mod theory;
mod types;

pub use {
    core_engine::{CoreEngine, CoreEngineExplainKey},
    engine::{EngineAddConstraintTrait, EngineTrait},
    outer_engine::OuterEngine,
    theory::{TheoryAddConstraintTrait, TheoryTrait},
    types::{Boolean, Literal, LiteralArray, PropagationResult, Reason},
};
