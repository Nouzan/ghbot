use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Common {
    pub action: String,
    pub sender: Option<Value>,
    pub repository: Option<Value>,
    pub organization: Option<Value>,
    pub installation: Option<Value>,
}
