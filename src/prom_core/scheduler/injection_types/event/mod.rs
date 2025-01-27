pub mod queue;
pub mod reader;
pub mod writer;

/// Marker trait for the scheduler
pub trait Event: 'static {}
