use specs::prelude::*;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_worker::*;

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
    });
}

#[wasm_bindgen]
pub struct BBBB;

#[wasm_bindgen]
impl BBBB {
    #[wasm_bindgen(constructor)]
    pub fn new(object: &JsValue) -> Result<BBBB, JsValue> {
        Ok(BBBB)
    }

    #[wasm_bindgen]
    pub fn something() {}
}

pub struct Comp {
    a: usize,
    b: usize,
}

impl Component for Comp {
    type Storage = VecStorage<Self>;
}

pub struct Sys;

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
