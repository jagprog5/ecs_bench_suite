use anput::world::World;

macro_rules! create_entities {
    ($world:ident; $( $variants:ident ),*) => {
        $(
            struct $variants(f32);
            for _ in 0..20 {
                let _ = $world.spawn( ($variants(0.0), Data(1.0)) );
            }
        )*
    };
}

struct Data(f32);

pub struct Benchmark<const LOCKING: bool>(World);

impl<const LOCKING: bool> Benchmark<LOCKING> {
    pub fn new() -> Self {
        let mut world = World::default();

        create_entities!(world; A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

        Self(world)
    }

    pub fn run(&mut self) {
        for data in self.0.query::<LOCKING, &mut Data>() {
            data.0 *= 2.0;
        }
    }
}
