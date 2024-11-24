use strum_macros::{AsRefStr, EnumIter, EnumString};


//AW = Action Word
pub const AW_LV:&str = "LV"; //Load Vault
pub const AW_CV:&str = "CV"; //Create Vault
pub const AW_UV:&str = "UV"; //Update Vault


//Rusty Files AW
pub const AW_CRSF:&str = "CRSF"; //Create Rusty File
pub const AW_DRSF:&str = "DRSF"; //Delete Rusty File
pub const AW_URSF:&str = "URSF"; //Update Rusty File
pub const AW_RRSF:&str = "RRSF"; //Read Rusty File


//Folders AW
pub const AW_CF:&str = "CF"; //Create Folder
pub const AW_DF:&str = "DF"; //Delete Folder
pub const AW_UFN:&str = "UFN"; //Update Folder Name


//Converters AW
pub const AW_HTMLC:&str = "HTMLC"; //HTML Convert
pub const AW_PDFC:&str = "PDFC"; //PDF Convert



#[derive(EnumString, AsRefStr, PartialEq, EnumIter)]
pub enum RsSErrorLogs {
    SE00, //SE = Server Error
    SE01,
    SE02,
    SE03,
    SE04,
    SE05,
}
#[derive(EnumString, AsRefStr, PartialEq, EnumIter)]
pub enum RsSSuccesLogs {
    SS00, //SS = Server Success
    S01,
    S02,
    S03,
    S04,
    S05,
    S06,
}



