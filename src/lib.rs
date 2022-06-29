//! PipeVision is a simple command-line tool
//! to inspect and monitor the progress of
//! data flowing through a pipe.
//!
//! It shows the processing time and
//! the speed of data flowing through the pipeline.

pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const CHUNK_SIZE: usize = 16 * 1024;
