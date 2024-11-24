use std::{fs, path::Path};

use crate::config_rsf::rsdocs_config::
    {RSDocsPaths, RsDocsConfig, DEFAULT_RELATIVE_PATH};
    
use crate::doc_files::rusty_file::RustyFile;
use crate::server_rsdocs::rs_server_cons::
    {RsSErrorLogs, RsSSuccesLogs};


#[derive(serde::Serialize)]
pub struct RsDocsManger {
    pub rs_conf:RsDocsConfig,
}

impl RsDocsManger {
    
    pub fn new() -> RsDocsManger{
        let rs_conf = RsDocsConfig::new();
        RsDocsManger { rs_conf }
    }


    pub fn init(&mut self, vault_path:String)->bool{
        if Path::new(vault_path.as_str()).exists() {
            self.rs_conf.load_config_file(vault_path);
            return true;
        }
        false
    }


    pub fn add_new_file(&mut self, vault:String,
        mut _folder_path:String, mut _name:String)->String{
        let path = format!("{}{}", vault,_folder_path);
        
        if !Path::new(path.as_str()).exists() {
            return RsSErrorLogs::SE03.as_ref().to_string();
        }
        println!("{}", path);

        let mut _relative_path = String::new();
        let mut rusty_file = RustyFile::new();
        
        
        //Make Folder Path
        if _folder_path == String::new() 
        || _folder_path == DEFAULT_RELATIVE_PATH.to_string() {
            _relative_path = format!("{}", DEFAULT_RELATIVE_PATH);
        } else {
            _relative_path = format!("{}{}", _folder_path, DEFAULT_RELATIVE_PATH);
        }

        //File Creation
        rusty_file.create(_name.clone(),
                    _relative_path.as_str(),
                    self.rs_conf.get_vault_path(),
                        self.rs_conf.get_ids());
        

        //Add the new file to path list
        let mut new_path = RSDocsPaths::new(); 
        
        new_path.id = rusty_file.id;
        new_path.relative_path = format!("{}{}", 
                    _folder_path,
                    _name);
        
        self.rs_conf.file_paths.push(new_path);
        
        //Overwrite config file
        self.rs_conf.write_content();
        return RsSSuccesLogs::S03.as_ref().to_string();
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
                return RsSErrorLogs::SE02.as_ref().to_string();
            },
        }
    }


}








