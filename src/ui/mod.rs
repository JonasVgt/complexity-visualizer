#[cfg(debug_assertions)]
use crate::ui::debugging::DebugAppState;

#[cfg(debug_assertions)]
pub mod debugging;

pub mod filtering;
pub mod graph;
pub mod sidepanel;

pub struct AppState {
    #[cfg(debug_assertions)]
    debug: DebugAppState,
}

impl AppState {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        {
            
            AppState {
                debug: DebugAppState::new(),
            }
        }

        #[cfg(not(debug_assertions))]
        {
            AppState {}
        }
    }
}
