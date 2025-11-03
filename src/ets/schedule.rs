pub trait HasAB {
    fn swap_ab(&mut self);
}

pub trait HasCD {
    fn swap_cd(&mut self);
}

pub trait HasCE {
    fn swap_ce(&mut self);
}

pub struct AB {
    pub a: f32,
    pub b: f32,
}

pub struct ABC {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

pub struct ABCD {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

pub struct ABCE {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub e: f32,
}

impl HasAB for AB {
    fn swap_ab(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

impl HasAB for ABC {
    fn swap_ab(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

impl HasAB for ABCD {
    fn swap_ab(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

impl HasAB for ABCE {
    fn swap_ab(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

impl HasCD for ABCD {
    fn swap_cd(&mut self) {
        std::mem::swap(&mut self.c, &mut self.d);
    }
}

impl HasCE for ABCE {
    fn swap_ce(&mut self) {
        std::mem::swap(&mut self.c, &mut self.e);
    }
}

// explicit outer parallelism
pub trait HasABAndCD {
    fn swap_ab_and_cd(&mut self);
}

impl<T> HasABAndCD for T
where
    T: HasAB,
    T: HasCD,
{
    fn swap_ab_and_cd(&mut self) {
        self.swap_ab();
        self.swap_cd();
    }
}

entity_trait_system::world!(MyWorld, AB, ABC, ABCD, ABCE; HasABAndCD, HasCE);

pub struct Benchmark(MyWorld);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = MyWorld::default();

        (0..crate::INSTANCES_COUNT).for_each(|_| {
            world.a_b.insert(AB { a: 0.0, b: 0.0 });
        });
        (0..crate::INSTANCES_COUNT).for_each(|_| {
            world.a_b_c.insert(ABC {
                a: 0.0,
                b: 0.0,
                c: 0.0,
            });
        });
        (0..crate::INSTANCES_COUNT).for_each(|_| {
            world.a_b_c_d.insert(ABCD {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
            });
        });
        (0..crate::INSTANCES_COUNT).for_each(|_| {
            world.a_b_c_e.insert(ABCE {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                e: 0.0,
            });
        });
        Self(world)
    }

    pub fn run(&mut self) {
        self.0.par_visit_mut_has_a_b_and_c_d(|e| e.swap_ab_and_cd());
        self.0.par_visit_mut_has_c_e(|e| e.swap_ce());
    }
}

#[test]
fn test() {
    Benchmark::new().run();
}
