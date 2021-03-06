use super::models::{Factoid, FactoidEnum};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RFactoid {
    pub key: String,
    pub val: RFactoidValue,
}

#[derive(Serialize, Deserialize)]
pub struct RFactoidValue {
    pub intent: Option<String>,
    pub message: String,
    pub editor: RFactoidEditor,
    pub time: String,
    pub frozen: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RFactoidEditor {
    pub nickname: String,
    pub username: String,
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
pub struct WinError {
    pub code: String,
    pub name: String,
    pub desc: String,
}

impl From<Factoid> for RFactoid {
    fn from(factoid: Factoid) -> Self {
        let time = DateTime::<Utc>::from_utc(factoid.timestamp, Utc)
            .to_rfc3339_opts(SecondsFormat::Millis, true);
        let intent = match factoid.intent {
            FactoidEnum::Act => Some("act".into()),
            FactoidEnum::Say => Some("say".into()),
            FactoidEnum::Alias => Some("alias".into()),
            FactoidEnum::Forget => None,
        };

        RFactoid {
            key: factoid.label,
            val: RFactoidValue {
                intent,
                message: factoid.description,
                editor: RFactoidEditor {
                    nickname: factoid.nickname,
                    username: "".into(),
                    hostname: "".into(),
                },
                time,
                frozen: factoid.locked,
            },
        }
    }
}
