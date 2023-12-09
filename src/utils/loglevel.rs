#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    All,
    Debug,
    Info,
    Warning,
    Severe,
    Off,
}

impl LogLevel {
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::All => "ALL".to_string(),
            LogLevel::Debug => "DEBUG".to_string(),
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warning => "WARNING".to_string(),
            LogLevel::Severe => "SEVERE".to_string(),
            LogLevel::Off => "OFF".to_string(),
        }
    }
}
