use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Common {
    pub action: String,
    pub sender: Value,
    pub repository: Value,
    pub organization: Value,
    pub installation: Value,
}
