use anput::world::World;
use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark<const LOCKING: bool>(World);

impl<const LOCKING: bool> Benchmark<LOCKING> {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 0..10000 {
            let _ = world.spawn((
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            ));
        }

        Self(world)
    }

    pub fn run(&mut self) {
        for (velocity, position) in self.0.query::<LOCKING, (&Velocity, &mut Position)>() {
            position.0 += velocity.0;
        }
    }
}
