//! A simple example of the specs ecs running on wasm.
//! Runs only on firefox nightly with the shared memory option enabled (in about:config).

use specs::prelude::*;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_worker::*;

/// Runs the test.
/// We create a rayon thread pool from the given worker pool and concurrency (desired thread
/// count), then we create a shred/specs world and dispatcher and insert a ton of entities, then we
/// run the dispatcher a couple of times.
#[wasm_bindgen]
pub fn run_test(concurrency: usize, pool: &WorkerPool) {
    init_panic_hook();
    let thread_pool = Arc::new(new_thread_pool(concurrency, pool));
    pool.run(move || {
        let mut world = World::new();
        let mut dispatch = DispatcherBuilder::new()
            .with_pool(thread_pool)
            .with(Sys, "sys", &[])
            .build();

        world.register::<Comp>();

        for _ in 0..100000 {
            world.create_entity().with(Comp { a: 2, b: 3 }).build();
        }

        for _ in 0..3 {
            dispatch.dispatch(&mut world);
            world.maintain();
        }
    }).unwrap();
}

struct Comp {
    a: usize,
    b: usize,
}

impl Component for Comp {
    type Storage = VecStorage<Self>;
}

struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (ReadStorage<'a, Comp>,);
    fn run(&mut self, (comps,): Self::SystemData) {
        let sum = (&comps,)
            .par_join()
            .map(|(comp,)| comp.a + comp.b)
            .reduce(|| 0, |a, b| a + b);
        console_log!("sum: {}", sum);
    }
}
