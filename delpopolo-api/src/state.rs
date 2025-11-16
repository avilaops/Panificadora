use delpopolo_infrastructure::{Database, Cache, Config};

pub struct AppState {
    pub database: Database,
    pub cache: Cache,
    pub config: Config,
}

impl AppState {
    pub fn new(database: Database, cache: Cache, config: Config) -> Self {
        Self {
            database,
            cache,
            config,
        }
    }
}
