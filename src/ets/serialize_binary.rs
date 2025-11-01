use cgmath::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct MyData {
    transform: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    velocity: Vector3<f32>,
}

// Declare the ECS world with MyData arena
entity_trait_system::world!(MyWorld, MyData;);

pub struct Benchmark(MyWorld);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = MyWorld::default();

        for _ in 0..1000 {
            world.my_data.insert(MyData {
                transform: Matrix4::from_scale(1.0),
                position: Vector3::unit_x(),
                rotation: Vector3::unit_x(),
                velocity: Vector3::unit_x(),
            });
        }

        Self(world)
    }

    pub fn run(&mut self) {
        // Serialize the entire world to bincode bytes
        let serialized = bincode::serialize(&self.0).unwrap();

        // Deserialize back into a new world
        let _deserialized: MyWorld = bincode::deserialize(&serialized).unwrap();
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
