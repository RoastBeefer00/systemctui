use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use systemctl::*;

fn get_services() -> Vec<Unit> {
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
        services
    } else {
        Vec::new()
    }
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let selected = app.table_state.selected().unwrap_or(0);
            if selected == app.services.len() - 1 {
                app.table_state.select(Some(0))
            } else {
                app.table_state.select(Some(selected + 1))
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let selected = app.table_state.selected().unwrap_or(0);
            if selected == 0 {
                app.table_state.select(Some(app.services.len() - 1))
            } else {
                app.table_state.select(Some(selected - 1))
            }
        }
        KeyCode::Char('s') => {
            if let Some(index) = app.table_state.selected() {
                if let Some(service) = app.services.get(index) {
                    let _ = service.start();
                    app.services = get_services();
                }
            }
        }
        KeyCode::Char('p') => {
            if let Some(index) = app.table_state.selected() {
                if let Some(service) = app.services.get(index) {
                    let _ = service.stop();
                    app.services = get_services();
                }
            }
        }
        KeyCode::Char('r') => {
            if let Some(index) = app.table_state.selected() {
                if let Some(service) = app.services.get(index) {
                    let _ = service.restart();
                    app.services = get_services();
                }
            }
        }
        KeyCode::Char('e') => {
            if let Some(index) = app.table_state.selected() {
                if let Some(service) = app.services.get(index) {
                    let _ = service.enable();
                    app.services = get_services();
                }
            }
        }
        KeyCode::Char('d') => {
            if let Some(index) = app.table_state.selected() {
                if let Some(service) = app.services.get(index) {
                    let _ = service.disable();
                    app.services = get_services();
                }
            }
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
