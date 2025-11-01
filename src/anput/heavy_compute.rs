use anput::{third_party::moirai::Jobs, view::WorldView, world::World};
use cgmath::*;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark<const LOCKING: bool>(World, Jobs);

impl<const LOCKING: bool> Benchmark<LOCKING> {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 0..1000 {
            let _ = world.spawn((
                Matrix4::<f32>::from_angle_x(Rad(1.2)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            ));
        }

        Self(world, Jobs::default())
    }

    pub fn run(&mut self) {
        let view = WorldView::new::<(Position, Matrix4<f32>)>(&self.0);

        self.1
            .broadcast(move |ctx| {
                let entities =
                    view.entities_work_group(ctx.work_group_index, ctx.work_groups_count, 10);

                let access = view.lookup::<LOCKING, (&mut Position, &mut Matrix4<f32>)>(entities);

                for (pos, mat) in access {
                    for _ in 0..100 {
                        *mat = mat.invert().unwrap();
                    }

                    pos.0 = mat.transform_vector(pos.0);
                }
            })
            .unwrap()
            .wait()
            .unwrap();
    }
}
