use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use crate::rsdocs_manager::RsDocsManger;
use crate::config_rsf::rsdocs_config::DEFAULT_RELATIVE_PATH;
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
        let mut rss_response: RsSResponse = RsSResponse::new();
        let mut doc_manager = RsDocsManger::new();
        println!("{}", json);
        
        // Deserialize JSON
        let request:RsCRequest = serde_json::from_str(&json).unwrap();
        self.auto_fill(&request);
        
        if !doc_manager.init(request.path.clone()) {
            
            rss_response.log = RsSErrorLogs::SE01.as_ref().to_string();
            return rss_response;
        }

        let destination_path:String = format!("{}{}",request.path, request.body);
        match request.action.as_str() {
            AW_LV => {
                rss_response.log = RsSSuccesLogs::SS00.as_ref().to_string();
                let body = serde_json::to_string(&doc_manager.rs_conf.file_paths).unwrap();
                rss_response.body = body;
            },
            AW_CF => {
                rss_response.log = doc_manager.create_folder(destination_path.clone());
            },
            AW_CRSF =>{
                //Get FileName
                let mut _rsf_path_trimed:Vec<&str> = request.body.split('/').collect();
                let rsf_name = _rsf_path_trimed
                                            .get(_rsf_path_trimed.len() - 1)
                                            .unwrap();
                let _name = rsf_name.to_string();
                let mut _relative_path = String::new();
                
                if _rsf_path_trimed.len() > 2 {
                    _relative_path = _rsf_path_trimed.remove(_rsf_path_trimed.len() - 1).to_string();
                }else {
                    _relative_path = format!("{}",DEFAULT_RELATIVE_PATH);
                }
                
                rss_response.log = doc_manager.add_new_file(request.path.clone(),_relative_path, _name);
            }
            _ =>{ rss_response.log = RsSErrorLogs::SE00.as_ref().to_string();
                rss_response.body = String::new();}
        }

        // if self.is_success_log(&rss_response.log) {
        //     rss_response.body = destination_path;
        // }
        rss_response
    }


    fn auto_fill(&mut self, request:&RsCRequest){
        self.action = request.action.clone();
        self.client = request.client.clone();
        self.body = request.body.clone();
        self.path = request.path.clone();
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


