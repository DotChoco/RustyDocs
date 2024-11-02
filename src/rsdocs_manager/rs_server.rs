use serde::{Deserialize, Serialize};


use super::rs_server_cons::*;

//AW = Action Word
const AW_LV:&str = "LV"; //Load Vault
const AW_CV:&str = "public"; //Create Vault
const AW_CRSF:&str = "private"; //Create Rusty File
const AW_RRSF:&str = "namespace"; //Read Rusty File
const AW_HTMLC:&str = "using"; //HTML Convert
const AW_PDFC:&str = "class"; //PDF Convert


#[derive(Debug, Serialize, Deserialize)]
pub struct RsSResponse {
    body:String,
    pub log:String
}

impl RsSResponse {
    pub fn new() -> RsSResponse{
        let body = String::new();
        let log = String::new();
        RsSResponse { body, log }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RsCRequest {
    action:String,
    body:String,
}

impl RsCRequest {
    pub fn new() -> RsCRequest {
        let action = String::new();
        let body = String::new();
        RsCRequest { action, body, }
    }


    pub fn run_action(&mut self, json:String) -> RsSResponse{
        let mut rss_response = RsSResponse::new();
        
        // Deserialize JSON
        let request:RsCRequest  = serde_json::from_str(json.as_str()).expect("Error deserializing JSON");

        if !self.action_exists(&request.action){
            rss_response.body = String::new();
            rss_response.log = RsSErrorLogs::S00.as_ref().to_string();
        }

        rss_response
    }

    pub fn action_exists(&mut self, action:&str) -> bool {
        matches!(action, AW_CV | AW_CRSF| AW_HTMLC 
                | AW_LV | AW_PDFC | AW_RRSF)
    }


}


