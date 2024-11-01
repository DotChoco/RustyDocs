use std::path::Path;
use std::fs::read_to_string;
use std::io::Write;
use std::io::Result;

pub const CONFIG_FILE:&str = "config.rdcf";
pub const DEFAULT_RELATIVE_PATH:char = '/';

pub struct RsDocsConfig {
    pub vault_path:String, //AbsolutePath
    content:String, //Content of config file
    pub file_paths:Vec<(String, (String, String))> //List of paths(Id, (RelativePath, FileName))
}

impl RsDocsConfig {
    pub fn new() -> RsDocsConfig{
        let vault_path = String::new();
        let content = String::new();
        let file_paths:Vec<(String, (String, String))> = Vec::new();
        RsDocsConfig { vault_path, content, file_paths }
    }
    

    pub fn load_vault_path(&mut self, path:String) {
        self.vault_path = path;
    }


    pub fn load_config_file(&mut self) {
        let path = format!("{}/{}",self.vault_path.clone(),CONFIG_FILE);
        
        // Leer el contenido del archivo
        match read_to_string(path) {
            Ok(content) => {
                let lines:Vec<_> = content.lines().collect();
                let mut items:Vec<(String, (String, String))> = Vec::new();
                
                for line in lines {
                    let item:Vec<&str> = line.split(':').collect();
                    items.push(
                        (item[0].to_string(),
                        (item[1].to_string(),
                        item[2].to_string()
                    )));
                }

                self.file_paths = items;
                println!("{}",self.file_paths[0].clone().1.1)
            }

            Err(e) => { println!("Error al leer el archivo: {}", e); }
        }
    }


    pub fn get_ids(&mut self)->Vec<String>{
        let ids: Vec<String> = self.file_paths
            .iter()
            .map(|(first, _)| first.clone()) // Extrae el primer elemento de cada tupla
            .collect();
        ids
    }




}