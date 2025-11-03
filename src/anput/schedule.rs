use anput::{
    query::Query,
    scheduler::{GraphScheduler, GraphSchedulerPlugin, SystemParallelize},
    systems::SystemContext,
    third_party::moirai::Jobs,
    universe::Universe,
    world::World,
};
use std::error::Error;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab<const LOCKING: bool>(context: SystemContext) -> Result<(), Box<dyn Error>> {
    let (world, query) = context.fetch::<(&World, Query<LOCKING, (&mut A, &mut B)>)>()?;

    for (a, b) in query.query(world) {
        std::mem::swap(&mut a.0, &mut b.0);
    }

    Ok(())
}

fn cd<const LOCKING: bool>(context: SystemContext) -> Result<(), Box<dyn Error>> {
    let (world, query) = context.fetch::<(&World, Query<LOCKING, (&mut C, &mut D)>)>()?;

    for (c, d) in query.query(world) {
        std::mem::swap(&mut c.0, &mut d.0);
    }

    Ok(())
}

fn ce<const LOCKING: bool>(context: SystemContext) -> Result<(), Box<dyn Error>> {
    let (world, query) = context.fetch::<(&World, Query<LOCKING, (&mut C, &mut E)>)>()?;

    for (c, e) in query.query(world) {
        std::mem::swap(&mut c.0, &mut e.0);
    }

    Ok(())
}

pub struct Benchmark<const LOCKING: bool>(Universe, Jobs);

impl<const LOCKING: bool> Benchmark<LOCKING> {
    pub fn new() -> Self {
        let mut universe = Universe::default().with_plugin(
            GraphSchedulerPlugin::<LOCKING>::default()
                .plugin_setup(|plugin| {
                    plugin
                        .system_setup(ab::<LOCKING>, |system| {
                            system.local(SystemParallelize::AnyWorker)
                        })
                        .system_setup(cd::<LOCKING>, |system| {
                            system.local(SystemParallelize::AnyWorker)
                        })
                })
                .system_setup(ce::<LOCKING>, |system| {
                    system.local(SystemParallelize::AnyWorker)
                }),
        );

        for _ in 0..crate::INSTANCES_COUNT {
            universe.simulation.spawn((A(0.0), B(0.0))).unwrap();
        }
        for _ in 0..crate::INSTANCES_COUNT {
            universe.simulation.spawn((A(0.0), B(0.0), C(0.0))).unwrap();
        }
        for _ in 0..crate::INSTANCES_COUNT {
            universe
                .simulation
                .spawn((A(0.0), B(0.0), C(0.0), D(0.0)))
                .unwrap();
        }
        for _ in 0..10 {
            universe
                .simulation
                .spawn((A(0.0), B(0.0), C(0.0), E(0.0)))
                .unwrap();
        }

        Self(universe, Jobs::default())
    }

    pub fn run(&mut self) {
        let _ = GraphScheduler::<LOCKING>.run(&self.1, &mut self.0);
    }
}

#[test]
fn test_locking() {
    Benchmark::<true>::new().run();
}

#[test]
fn test_lockfree() {
    Benchmark::<false>::new().run();
}
