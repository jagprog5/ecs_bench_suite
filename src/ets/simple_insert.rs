use cgmath::*;

#[derive(Copy, Clone)]
struct MyData {
    transform: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    velocity: Vector3<f32>,
}

entity_trait_system::world!(MyWorld, MyData;);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = MyWorld::default();
        for _ in 0..10_000 {
            world.my_data.insert(MyData {
                transform: Matrix4::from_scale(1.0),
                position: Vector3::unit_x(),
                rotation: Vector3::unit_x(),
                velocity: Vector3::unit_x(),
            });
        }
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
