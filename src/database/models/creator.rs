use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/**
 * The type of creator.
 * `Writer` is a "normal" creator, while `Publisher` is more like an admin.
 */
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "creator_role", rename_all = "lowercase")]
pub enum CreatorRole {
    Publisher,
    Writer,
}

/**
 * A `Creator` is someone who can write articles on the site.
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
    /// `display_name` may use any characters.
    pub display_name: String,
    pub username: String,
    pub password: String,
    pub biography: String,
    pub joined_at: DateTime<Utc>,
    pub role: CreatorRole,
}

impl Default for Creator {
    fn default() -> Self {
        Creator {
            display_name: "No Name".to_string(),
            username: "no_name".to_string(),
            password: "".to_string(),
            biography: "Empty biography.".to_string(),
            joined_at: Utc::now(),
            role: CreatorRole::Writer,
        }
    }
}

impl Creator {
    pub fn create_writer(username: &str, display_name: &str, password: &str) -> Self {
        Creator {
            username: username.to_string(),
            display_name: display_name.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    pub fn create_publisher(username: &str, display_name: &str, password: &str) -> Self {
        Creator {
            username: username.to_string(),
            display_name: display_name.to_string(),
            password: password.to_string(),
            role: CreatorRole::Publisher,
            ..Default::default()
        }
    }

    pub fn is_publisher(&self) -> bool {
        matches!(self.role, CreatorRole::Publisher)
    }
}
