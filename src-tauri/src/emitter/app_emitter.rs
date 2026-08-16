use crate::app_error::errors::{AppError, AppResult};
use crate::AppState;
use tauri::{Emitter, State};

pub struct AppEmitter;

pub trait AppEvent {
    fn as_str(&self) -> &'static str;
}

/// Represents events that require action from frontend, its not critical but it indicates to frontend new data
/// is available and other stuff related
/// Example : NewFileindexed -> in this case backend just informed frontend that a new file was indexed, but hasnt sent the file path
/// so frontend now knows that has a new file and need to trigger or not a callback to handle the situation
#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionRequiredEvent {
    NewFilesIndexed,
}

/// Represents events that send data and has no required action from frontend.
/// This kind of event is the product of an already completed action, and it's used to notify frontend about a result of an action.
///
/// Ex: Scan Completed -> in this case u don't have to take any action, but u can use this event to update the UI and show the user that the scan is completed
/// and has some data from the event, you can take the data to update UI.
#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResultEvent {
    ScanStarted,
    ScanCompleted,
}

impl AppEvent for ActionRequiredEvent {
    fn as_str(&self) -> &'static str {
        match self {
            ActionRequiredEvent::NewFilesIndexed => "new_files_indexed",
        }
    }
}

impl AppEvent for ResultEvent {
    fn as_str(&self) -> &'static str {
        match self {
            ResultEvent::ScanStarted => "scan_started",
            ResultEvent::ScanCompleted => "scan_completed",
        }
    }
}

impl AppEmitter {
    pub  fn emit<E, P>(state: State<'_, AppState>, event: E, payload: P) -> AppResult<()>
    where
        E: AppEvent,
        P: serde::Serialize + Clone,
    {
        match state.handle.emit(event.as_str(), payload) {
            Ok(data) => {}
            Err(e) => {
                return Err(AppError::Generic(format!(
                    "Error while sending emitter {}",
                    e
                )))
            }
        }

        Ok(())
    }
}
