use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
#[serde(untagged)]
pub enum OneOfLessThanGreaterThan{
    Clock {
        #[serde(rename = "type")]
        r#type: String,
        limit: Option<f64>,
        increment: Option<f64>,
        show: Option<String>,
    },
    Correspondence {
        #[serde(rename = "type")]
        r#type: String,
        daysPerTurn: Option<f64>,
    },
    Unlimited {
        #[serde(rename = "type")]
        r#type: String,
    },
}
