use std::fs;

use super::{ config_rsf::rsdocs_config::*, doc_files::rusty_file::*, server_rsdocs::rs_server_cons::* };


#[derive(serde::Serialize)]
pub struct RsDocsManger {
    pub rs_conf:RsDocsConfig,
}

impl RsDocsManger {
    
    pub fn new() -> RsDocsManger{
        let rs_conf = RsDocsConfig::new();
        
        RsDocsManger { rs_conf }
    }


    pub fn init(&mut self, vault_path:String) {
        self.rs_conf.load_config_file(vault_path);
    }


    pub fn add_new_file(&mut self, folder_path:String){
        let mut _def_path = String::new();
        let mut rusty_file = RustyFile::new();

        if folder_path == String::new() ||
            folder_path == DEFAULT_RELATIVE_PATH.to_string() {
            _def_path = format!("{}{}",self.rs_conf.get_vault_path(), DEFAULT_RELATIVE_PATH);
        } else {
            _def_path = format!("{}{}{}",self.rs_conf.get_vault_path(), DEFAULT_RELATIVE_PATH, folder_path);
        }
        
        //File Creation
        rusty_file.create(_def_path.as_str(),self.rs_conf.get_ids());
        
        //Get FileName
        let mut _rsf_path_trimed:Vec<&str> = rusty_file.path.split('/').collect();
        let rsf_name = _rsf_path_trimed
                                    .get(_rsf_path_trimed.len() - 1)
                                    .unwrap();
        

        //Add the new file to path list
        let mut new_path =RSDocsPaths::new(); 
        
        new_path.id = rusty_file.id;
        new_path.relative_path = format!("{}{}", 
                    DEFAULT_RELATIVE_PATH.to_string(),
                    (*rsf_name).to_string());
        
        self.rs_conf.file_paths.push(new_path);
        
        //Overwrite config file
        self.rs_conf.write_content();
    }


    pub fn load_file(&mut self) -> String {
        let mut rusty_file = RustyFile::new();

        //Load RustyFiles
        for rs_paths in self.rs_conf.file_paths.clone() {
            rusty_file.id = rs_paths.id;
            rusty_file.path = format!("{}{}", 
                        self.rs_conf.get_vault_path(), 
                        rs_paths.relative_path);
            rusty_file.read();
        }
        rusty_file.content
    }


    pub fn create_folder(&mut self, path:String)->String{
        match fs::create_dir(path) {
            Ok(_) => {
                println!("Carpeta creada exitosamente.");
                return RsSSuccesLogs::S01.as_ref().to_string();
            },
            Err(e) => {
                eprintln!("Error al crear la carpeta: {}", e);
                return RsSErrorLogs::SE01.as_ref().to_string();
            },
        }
    }
}








