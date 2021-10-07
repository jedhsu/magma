use tch::Init;


pub trait Assign<E>: Store<E> + Allocate<E> where E: Environment {
    fn deref_or_store(&self, shape: &[u32], init: Initializer) {
        match init {
            Initializer::Zeros => tch::or_var(&self, 
        }
    }

    fn deref_or_alloc(&self, init: Initializer) {}
}
