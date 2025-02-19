use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(EnumString, AsRefStr, PartialEq, EnumIter)]
pub enum RsSErrorLogs {
    BE00, //SE = Backend Error
    BE01,
    BE02,
    BE03,
    BE04,
    BE05,
}
#[derive(EnumString, AsRefStr, PartialEq, EnumIter)]
pub enum RsSSuccesLogs {
    BS00, //SS = Server Success
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
}

