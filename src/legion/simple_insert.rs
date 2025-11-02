use cgmath::*;
use legion::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::default();

        world.extend(
            (
                vec![Transform(Matrix4::from_scale(1.0)); crate::INSTANCES_COUNT],
                vec![Position(Vector3::unit_x()); crate::INSTANCES_COUNT],
                vec![Rotation(Vector3::unit_x()); crate::INSTANCES_COUNT],
                vec![Velocity(Vector3::unit_x()); crate::INSTANCES_COUNT],
            )
                .into_soa(),
        );
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
