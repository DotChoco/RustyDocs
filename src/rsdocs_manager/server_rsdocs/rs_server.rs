use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use crate::rsdocs_manager::rsdocs_manager::RsDocsManger;
use super::rs_server_cons::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct RsSResponse {
    body:String,
    pub log:String
}

//Rusty Server Response
impl RsSResponse {
    pub fn new() -> RsSResponse{
        let body = String::new();
        let log = String::new();
        RsSResponse { body, log }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RsCRequest {
    pub client:String,
    action:String,
    path:String,
    body:String,
}

//Rusty Client Request
impl RsCRequest {
    pub fn new() -> RsCRequest {
        let client = String::new();
        let action = String::new();
        let path = String::new();
        let body = String::new();

        RsCRequest { client, action, path, body }
    }


    pub fn run_action(&mut self, json:String) -> RsSResponse{
        let mut rss_response = RsSResponse::new();
        println!("{}", json);

        // Deserialize JSON
        let request:RsCRequest = serde_json::from_str(json.as_str()).unwrap();
        
        self.auto_fill(&request);
        
        if !self.action_exists(&request.action){
            rss_response.body = String::new();
            rss_response.log = RsSErrorLogs::SE00.as_ref().to_string();
        } else {
            rss_response = self.exec_action(request);
        }

        rss_response
    }

    fn exec_action(&mut self, request:RsCRequest) ->RsSResponse{
        let mut rss_response: RsSResponse = RsSResponse::new();
        let mut doc_manager = RsDocsManger::new();
        match request.action.as_str() {
            AW_LV => {
                doc_manager.init(request.path);
                let body = serde_json::to_string(&doc_manager.rs_conf.file_paths).unwrap();
                rss_response.body = body;
                rss_response.log = RsSSuccesLogs::SS00.as_ref().to_string();
            },
            AW_CF => {
                let path:String = format!("{}/{}",request.path,request.body);
                rss_response.log = doc_manager.create_folder(path.clone());
                if self.is_success_log(rss_response.log.as_str()) {
                    rss_response.body = path;
                }
            },
            _ =>{}
        }
        rss_response
    }



    fn auto_fill(&mut self, request:&RsCRequest){
        self.action = request.action.clone();
        self.client = request.client.clone();
        self.body = request.body.clone();
        self.path = request.path.clone();
    }

    
    pub fn action_exists(&mut self, action:&str) -> bool {
        matches!(action, AW_CV | AW_CRSF| AW_HTMLC 
                | AW_LV | AW_PDFC | AW_RRSF| AW_CF| 
                AW_CRSF | AW_DF)
    }


    pub fn http_response_code(&mut self, rs_log:String) -> String{
        let mut _http_code = String::new();
        if self.is_success_log(rs_log.as_str()) {
            _http_code = "200".to_string();
        }
        else {
            _http_code = "400".to_string();
        }

        _http_code
    }

    fn is_success_log(&mut self, log:&str)-> bool {
        let is_success = RsSSuccesLogs::iter()
            .any(|rss_sl| rss_sl.as_ref() == log);
        is_success
    }

}


