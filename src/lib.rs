#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod database;
mod graph;
mod model;
mod sidepanel;
mod visualization_controller;
pub use app::ComplexityVisualizerApp;
