pub trait Variable<I, G> {
    /// Associated type for the ident.
    pub type I;

    /// Associated type for the group (combine>).
    pub type G;
}
