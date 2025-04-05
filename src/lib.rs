#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod database;
mod graph;
mod model;
mod sidepanel;
mod utils;
mod visualization_controller;
mod filtering;
pub use app::ComplexityVisualizerApp;
