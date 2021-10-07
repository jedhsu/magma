pub trait Storage

// Represents a runtime that holds state.
pub trait Interpreting {
    pub type V;
}

pub trait Serialize {
    pub fn from_file();
    pub fn into_file();
}

/// Binds a variable to an optimizing runtime.
pub trait Bind {
    pub fn unbind();
    pub fn bind();
}

pub trait Copy {

}

/// An environment that holds references to variables.
pub struct Referencing {
    /// An optional path describing the associated path.
    path: Option<Path>,
}
