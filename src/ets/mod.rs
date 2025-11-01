#![allow(clippy::len_without_is_empty)]

pub mod frag_iter;
pub mod heavy_compute;
pub mod schedule;
pub mod serialize_binary;
pub mod serialize_text;
pub mod simple_insert;
pub mod simple_iter;
// ETS can't add or remove components because it doesn't work with components at
// all. some alternatives:
//  - a trait which indicates if a property exists or not
//  - if the component is sparse, then an auxilliary data structure can contain
//    entity IDS, which then can be worked with
