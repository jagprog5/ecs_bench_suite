use cgmath::*;

#[derive(Copy, Clone)]
struct MyData {
    transform: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    velocity: Vector3<f32>,
}

pub trait DoCompute {
    fn compute(&mut self);
}

impl DoCompute for MyData {
    fn compute(&mut self) {
        for _ in 0..100 {
            self.transform = self.transform.invert().unwrap();
        }
        self.position = self.transform.transform_vector(self.position);
    }
}

entity_trait_system::world!(MyWorld, MyData; DoCompute);

pub struct Benchmark(MyWorld);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = MyWorld::default();

        for _ in 0..1000 {
            world.my_data.insert(MyData {
                transform: Matrix4::<f32>::from_angle_x(Rad(1.2)),
                position: Vector3::unit_x(),
                rotation: Vector3::unit_x(),
                velocity: Vector3::unit_x(),
            });
        }

        Self(world)
    }

    pub fn run(&mut self) {
        self.0.par_visit_mut_do_compute(|e| e.compute());
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
