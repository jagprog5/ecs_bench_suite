use cgmath::*;

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
struct MyData {
    transform: Matrix4<f32>,
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    velocity: Vector3<f32>,
}

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
        let serialized = ron::to_string(&self.0).unwrap();
        let _de_json: MyWorld = ron::from_str(&serialized).unwrap();
        // let serialized = ron::ser::to_string(&self.0).unwrap();
        // let mut deserializer = ron::de::Deserializer::from_str(&serialized).unwrap();
        // let _deserialized: MyWorld = serde::Deserialize::deserialize(&mut deserializer).unwrap();
    }
}
