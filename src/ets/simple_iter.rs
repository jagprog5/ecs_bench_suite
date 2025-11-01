use cgmath::*;

/// there's three groups
/// - structure of arrays is fastest (hecs, legion)
/// - middle group (including ets) array of structures
/// - planck_ecs is slowest (some sort of runtime overhead also)

#[derive(Copy, Clone)]
struct MyData {
    transform: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    velocity: Vector3<f32>,
}

pub trait Physics {
    fn update(&mut self);
}

impl Physics for MyData {
    fn update(&mut self) {
        self.position += self.velocity;
    }
}

entity_trait_system::world!(MyWorld, MyData;Physics);

pub struct Benchmark(MyWorld);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = MyWorld::default();
        for _ in 0..10_000 {
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
        self.0.visit_mut_physics(|h| {
            h.update();
        });
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
