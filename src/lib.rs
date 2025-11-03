#![allow(dead_code)]
#![allow(clippy::new_without_default)]
// required by ets
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod anput;
pub mod bevy;
pub mod ets;
pub mod hecs;
pub mod legion;
pub mod legion_packed;
pub mod planck_ecs;
pub mod shipyard;
pub mod specs;

pub const INSTANCES_COUNT: usize = 10000;
