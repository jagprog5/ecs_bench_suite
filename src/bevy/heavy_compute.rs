use bevy_ecs::prelude::*;
use cgmath::{Transform as TransformTrait, *};

#[derive(Component, Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Component, Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Component, Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Component, Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..1000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let mut query = self.0.query::<(&mut Position, &mut Transform)>();

        query
            .par_iter_mut(&mut self.0)
            .for_each(|(mut pos, mut trans)| {
                for _ in 0..100 {
                    trans.0 = trans.0.invert().unwrap();
                }

                pos.0 = trans.0.transform_vector(pos.0);
            });
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
