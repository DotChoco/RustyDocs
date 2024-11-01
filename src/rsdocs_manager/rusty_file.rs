use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;
use super::rsdocs_manager::RustyHashser;

pub const FILE_EXTENSION:&str = ".rdcs";
pub const FILE_DEFAULT_NAME:&str = "Untitled";


pub struct RustyFile {
    pub content:String, //Content of RustyFile
    pub path:String, //RelativePath
    pub id:String //Id
}

impl RustyFile {
    
    pub fn new()->RustyFile{
        let content = String::new();
        let id = String::new();
        let path = String::new();

        RustyFile { content, id, path }
    }


    pub fn create(&mut self, folder_path:&str, 
        files:Vec<String>){

        self.id = RustyHashser::new().make_hash();
        if self.path == String::new() {
            self.path = FILE_DEFAULT_NAME.to_string();
            self.path += &self.id.clone()[self.id.len() - 12..];
        }

        //Get path
        let full_path = format!("{}{}{}",folder_path,self.path, FILE_EXTENSION);
        let path = Path::new(full_path.as_str());
        
        //Check if the id actually exists
        let mut exist_id = false;
        if files != Vec::<String>::new() {
            exist_id = files.iter()
                            .any(|item| *item == self.id);
        }
        
        //don't exist Path and don't exist the Id
        if !path.exists() && !exist_id {
            let _ = self.write_data(full_path, 
                self.path.clone(),
                    String::new());
        }
    }


    fn write_data(&mut self, full_path:String, 
        path:String, content:String)-> Result<()> {

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false) // Agregar contenido al final en lugar de sobrescribir
        .open(full_path.as_str())?;
        
        let mut _file_content = String::new();
        if content == String::new() {
            _file_content = format!(".H1 {}", path);
        }else {
            _file_content = format!(".H1 {}\n{}", path, content);
        }

        file.write_all(_file_content.as_bytes())?;

        println!("Texto agregado. {}\n{}", path, full_path);

        Ok(())
    }


    pub fn read(&mut self){
        
    }

    
    pub fn update(&mut self, folder_path:&str, content:String){
        //Get path
        let full_path = format!("{}{}{}",folder_path,self.path, FILE_EXTENSION);
        let _ = self.write_data(full_path,
                    self.path.clone(),
                    content);
    }

}

