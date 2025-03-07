#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod database;
mod graph;
mod model;
mod sidepanel;
mod visualization_controller;
mod utils;
pub use app::ComplexityVisualizerApp;
