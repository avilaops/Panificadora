use delpopolo_infrastructure::{Database, Config};

pub struct AppState {
    pub database: Database,
    pub config: Config,
}

impl AppState {
    pub fn new(database: Database, config: Config) -> Self {
        Self {
            database,
            config,
        }
    }
}
