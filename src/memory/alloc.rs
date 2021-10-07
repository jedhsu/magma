pub trait Allocate {
    fn allocate(&self) -> Fn<(), Result> {}
}
