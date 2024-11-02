use strum_macros::{EnumString, AsRefStr};

#[derive(EnumString, AsRefStr, PartialEq)]
pub enum RsSErrorLogs {
    S00,
    S01,
    S02,
}