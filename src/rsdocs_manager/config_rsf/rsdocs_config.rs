use std::fs::OpenOptions;
use std::fs::read_to_string;
use std::io::Write;
use std::io::Result;

pub const CONFIG_FILE:&str = "config.rdcf";
pub const DEFAULT_RELATIVE_PATH:char = '/';

#[derive(serde::Serialize)]
pub struct RsDocsConfig {
    vault_path:String, //AbsolutePath
    pub file_paths:Vec<RSDocsPaths> //List of paths(Id, (RelativePath, FileName))
}

#[derive(serde::Serialize, Clone)]
pub struct RSDocsPaths{
    pub id:String,
    pub relative_path:String,
}

impl RSDocsPaths {
    pub fn new() -> RSDocsPaths{
        let id = String::new();
        let relative_path = String::new();

        RSDocsPaths { id, relative_path }
    }
}


impl RsDocsConfig {
    pub fn new() -> RsDocsConfig{
        let vault_path = String::new();
        let file_paths:Vec<RSDocsPaths> = Vec::new();
        RsDocsConfig { vault_path, file_paths }
    }
    

    pub fn get_vault_path(&mut self) -> String { self.vault_path.clone() }


    pub fn load_config_file(&mut self, path:String) {
        self.vault_path = path;
        
        let path = format!("{}/{}",self.vault_path.clone(),CONFIG_FILE);
        
        // Leer el contenido del archivo
        match read_to_string(path) {
            Ok(content) => {
                let mut paths:Vec<RSDocsPaths> = Vec::new();
                let mut item_path = RSDocsPaths::new();
                
                for line in content.lines() {
                    let item:Vec<&str> = line.split(':').collect();
                    
                    item_path.id = item[0].to_string();
                    item_path.relative_path = item[1].to_string();

                    paths.push(item_path.clone());
                }

                self.file_paths = paths;
            }

            Err(e) => { println!("Error al leer el archivo: {}", e); }
        }
    }


    pub fn get_ids(&mut self)->Vec<String>{
        let mut ids: Vec<String> = Vec::new();
        for data in self.file_paths.clone() {
            ids.push(data.id);
        }
        ids
    }


    pub fn write_content(&mut self){
        let mut content = String::new(); 
        for rsf_path in self.file_paths.clone() {
            content += format!("{}:{}\n", 
                                rsf_path.id, 
                                rsf_path.relative_path).as_str();
        }
        let _ = self.write_data(content);
    }
    
    
    fn write_data(&mut self,content:String)-> Result<()> {
        let path  = format!("{}{}{}",
                            self.vault_path.clone(),
                            DEFAULT_RELATIVE_PATH.to_string(),
                            CONFIG_FILE);
        

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            // .append(false) // Agregar contenido al final en lugar de sobrescribir
            .open(path.as_str())?;
        
        let mut _file_content = String::new();
        if content != String::new() {
            _file_content = format!("{}", content);
        }

        file.write_all(_file_content.as_bytes())?;

        println!("Path Rewrite. {}\n{}", path, content);

        Ok(())
    }


}