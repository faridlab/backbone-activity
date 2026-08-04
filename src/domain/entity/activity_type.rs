use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "activity_type", rename_all = "snake_case")]
pub enum ActivityType {
    Whatsapp,
    Call,
    Email,
    Meeting,
    Visit,
    Task,
    Note,
    Other,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::Call => write!(f, "call"),
            Self::Email => write!(f, "email"),
            Self::Meeting => write!(f, "meeting"),
            Self::Visit => write!(f, "visit"),
            Self::Task => write!(f, "task"),
            Self::Note => write!(f, "note"),
            Self::Other => write!(f, "other"),
        }
    }
}

impl FromStr for ActivityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "whatsapp" => Ok(Self::Whatsapp),
            "call" => Ok(Self::Call),
            "email" => Ok(Self::Email),
            "meeting" => Ok(Self::Meeting),
            "visit" => Ok(Self::Visit),
            "task" => Ok(Self::Task),
            "note" => Ok(Self::Note),
            "other" => Ok(Self::Other),
            _ => Err(format!("Unknown ActivityType variant: {}", s)),
        }
    }
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Whatsapp
    }
}
