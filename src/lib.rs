#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod database;
mod filtering;
mod graph;
mod model;
mod sidepanel;
mod utils;
mod visualization_controller;
pub use app::ComplexityVisualizerApp;
