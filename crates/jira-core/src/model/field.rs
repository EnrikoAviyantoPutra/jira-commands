use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub schema: Option<Value>,
    /// Allowed values for Select / MultiSelect fields (from createmeta)
    pub allowed_values: Option<Vec<Value>>,
}

/// Typed classification of a Jira field, derived from its schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldKind {
    Text,
    Number,
    DateTime,
    Select,
    MultiSelect,
    User,
    UserArray,
    Url,
    Checkbox,
    Labels,
    CascadingSelect,
    Unknown,
}

impl Field {
    /// Derive the kind of this field from its schema metadata.
    pub fn kind(&self) -> FieldKind {
        let schema = match &self.schema {
            Some(s) => s,
            None => return FieldKind::Unknown,
        };

        let typ = schema.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let items = schema.get("items").and_then(|v| v.as_str()).unwrap_or("");
        let custom = schema.get("custom").and_then(|v| v.as_str()).unwrap_or("");

        match typ {
            "string" if custom.contains("url") => FieldKind::Url,
            "string" => FieldKind::Text,
            "number" => FieldKind::Number,
            "datetime" | "date" => FieldKind::DateTime,
            "boolean" => FieldKind::Checkbox,
            "option" if custom.contains("cascading") => FieldKind::CascadingSelect,
            "option" => FieldKind::Select,
            "array" if items == "option" => FieldKind::MultiSelect,
            "array" if items == "string" => FieldKind::Labels,
            "array" if items == "user" => FieldKind::UserArray,
            "user" => FieldKind::User,
            _ => FieldKind::Unknown,
        }
    }
}

/// Typed value ready to be serialized into a Jira API request body.
#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(String),
    Number(f64),
    /// ISO 8601 date string ("YYYY-MM-DD" or full datetime)
    Date(String),
    /// Select field — match by value string
    SelectName(String),
    /// Select field — match by option ID
    SelectId(String),
    /// Multi-select — list of value strings
    MultiSelect(Vec<String>),
    /// User field — identified by email address
    UserEmail(String),
    /// Labels — plain string list
    Labels(Vec<String>),
    /// Escape hatch for anything else
    Raw(Value),
}

impl FieldValue {
    /// Serialize to the JSON shape Jira expects for each field type.
    pub fn to_api_json(&self) -> Value {
        match self {
            FieldValue::Text(s) => json!(s),
            FieldValue::Number(n) => json!(n),
            FieldValue::Date(d) => json!(d),
            FieldValue::SelectName(v) => json!({ "value": v }),
            FieldValue::SelectId(id) => json!({ "id": id }),
            FieldValue::MultiSelect(vs) => {
                json!(vs.iter().map(|v| json!({ "value": v })).collect::<Vec<_>>())
            }
            FieldValue::UserEmail(e) => json!({ "emailAddress": e }),
            FieldValue::Labels(ls) => json!(ls),
            FieldValue::Raw(v) => v.clone(),
        }
    }
}
