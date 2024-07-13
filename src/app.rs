use ratatui::widgets::TableState;
use std::error;
use systemctl::*;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,
    pub services: Vec<Unit>,
    pub table_state: TableState,
}

impl Default for App {
    fn default() -> Self {
        if let Ok(service_names) = list_units(None, None, None) {
            let mut services: Vec<Unit> = service_names
                .into_iter()
                .map(|name| {
                    if let Ok(unit) = Unit::from_systemctl(&name) {
                        unit
                    } else {
                        Unit::default()
                    }
                })
                .filter(|service| service.name != "".to_string())
                .collect();
            services.sort_by_key(|k| k.name.clone().to_lowercase());
            services.dedup();
            Self {
                running: true,
                counter: 0,
                services,
                table_state: TableState::default().with_selected(0),
            }
        } else {
            Self {
                running: true,
                counter: 0,
                services: Vec::new(),
                table_state: TableState::default().with_selected(0),
            }
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
