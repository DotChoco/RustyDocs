use super::{rusty_file::*, rsdocs_config::*};



pub struct RsDocsManger {
    pub rs_conf:RsDocsConfig,
}

impl RsDocsManger {
    
    pub fn new() -> RsDocsManger{
        let rs_conf = RsDocsConfig::new();
        RsDocsManger { rs_conf }
    }

    pub fn init(&mut self, vault_path:String) {
    
        // let mut rs_conf = RsDocsConfig::new();
        // self.rs_conf.load_vault_path("C:/Users/carlo/Downloads/Docs".to_string());
        self.rs_conf.load_vault_path(vault_path);
        self.rs_conf.load_config_file();

    }


    pub fn add_new_file(&mut self, folder_path:String){
        let mut _def_path = String::new();
        let mut rusty_file = RustyFile::new();

        if folder_path == String::new(){
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
        self.rs_conf.file_paths.push(( rusty_file.id,
            ( DEFAULT_RELATIVE_PATH.to_string(), (*rsf_name).to_string())));
        
        //Overwrite config file
        self.rs_conf.write_content();
    }

    pub fn load_file(&mut self) -> String {
        let mut rusty_file = RustyFile::new();

        //Load RustyFiles
        for rs_paths in self.rs_conf.file_paths.clone() {
            rusty_file.id = rs_paths.0;
            rusty_file.path = format!("{}{}{}",self.rs_conf.get_vault_path(), rs_paths.1.0, rs_paths.1.1);
            rusty_file.read();
        }
        rusty_file.content
    }


}








