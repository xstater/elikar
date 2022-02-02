use futures::{Future, executor::block_on};

pub trait Spawner {
    fn spawn<F>(&mut self,f : F)
    where F : Future<Output = ()> + Send + 'static;

    fn spawn_local<F>(&mut self,f : F)
    where F : Future<Output = ()> + 'static;

    fn block_on<F>(&self,f : F) -> F::Output
    where F : Future {
        block_on(f)
    }
}
