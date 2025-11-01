pub trait Checker {
    fn the_value(&mut self) -> &mut f32;
}

// Macro to generate a list of simple tuple structs with f32 fields
macro_rules! define_structs {
    ( $( $name:ident ),+ $(,)? ) => {
        $(
            #[derive(Copy, Clone, Debug)]
            pub struct $name(pub f32);

            impl Checker for $name {
                fn the_value(&mut self) -> &mut f32 {
                    &mut self.0
                }
            }
        )+
    };
}

define_structs!(
    A, B, C, DD, E, FF, G, H, I, J, K, L, M, N, O, P, Q, R, S, TT, U, VV, W, X, Y, Z
);

entity_trait_system::world!(
    MyWorld,
    A, B, C, DD, E, FF, G, H, I, J, K, L, M, N, O, P, Q, R, S, TT, U, VV, W, X, Y, Z;Checker
);

pub struct Benchmark {
    world: MyWorld,
}

impl Benchmark {
    /// Build a world and populate it:
    /// for each of A..Z, insert 20 entries; also insert 20 `Data` entries.
    pub fn new() -> Self {
        let mut world = MyWorld::default();
        let n = 20usize;

        macro_rules! populate {
            ( $( $name:ident ),+ ) => {
                $(
                    for _ in 0..n {
                        world.arena_mut::<$name>().insert($name(1.0));
                    }
                )+
            };
        }

        populate!(
            A, B, C, DD, E, FF, G, H, I, J, K, L, M, N, O, P, Q, R, S, TT, U, VV, W, X, Y, Z
        );

        Self { world }
    }

    pub fn run(&mut self) {
        self.world.visit_mut_checker(|e| {
            *e.the_value() *= 2.0;
        });
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
