use std::fs::OpenOptions;
use std::path::Path;
use std::fs::read_to_string;
use std::io::Write;
use std::io::Result;

pub const CONFIG_FILE:&str = "config.rdcf";
pub const DEFAULT_RELATIVE_PATH:char = '/';

pub struct RsDocsConfig {
    vault_path:String, //AbsolutePath
    pub file_paths:Vec<(String, (String, String))> //List of paths(Id, (RelativePath, FileName))
}

impl RsDocsConfig {
    pub fn new() -> RsDocsConfig{
        let vault_path = String::new();
        let file_paths:Vec<(String, (String, String))> = Vec::new();
        RsDocsConfig { vault_path, file_paths }
    }
    

    pub fn load_vault_path(&mut self, path:String) {
        self.vault_path = path;
    }
    
    
    pub fn get_vault_path(&mut self) -> String { self.vault_path.clone() }


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
                // println!("{}",self.file_paths[0].clone().1.1)
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

    pub fn write_content(&mut self){
        let mut content = String::new(); 
        for rsf_path in self.file_paths.clone() {
            content += format!("{}:{}:{}\n", 
                                rsf_path.0, 
                                rsf_path.1.0,
                                rsf_path.1.1,).as_str();
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
        .append(false) // Agregar contenido al final en lugar de sobrescribir
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