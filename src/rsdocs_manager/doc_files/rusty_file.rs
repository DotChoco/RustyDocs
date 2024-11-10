use std::fs::read_to_string;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Result;

use chrono::Utc;
use rand::Rng;

pub const RSFILE_EXTENSION:&str = ".rdcs";
pub const FILE_DEFAULT_NAME:&str = "Untitled";

/// Combination of Date(MM/DD/YYYY), Hour(HH:MM) 
/// and the Path where it will create
struct RustyHashser { hash:String } 

impl RustyHashser {
    pub fn new()
    ->RustyHashser{
        let hash = String::new();
        RustyHashser{ hash }
    }

    pub fn make_hash(&mut self)->String{
        
        //Get date with hour 
        //ex: 2024-10-31T18:00:19.396961+00:00
        let mut now_date = Utc::now().to_rfc3339();
        now_date = now_date[..26].to_string();

        // Generate a integer number between 10 to 100
        let rnd = rand::thread_rng().gen_range(100..=999);
        let rnd2 = rand::thread_rng().gen_range(100..=999);
        let rnd3 = rand::thread_rng().gen_range(100..=999);
        
        let mut hash = format!("{}",now_date);
        let chars:Vec<char> = hash.chars().collect();
        hash = String::new();

        for item in chars {
            hash += format!("{:X}", item as u32).as_str();
        }
        hash += format!("{:X}", rnd).as_str();
        hash += format!("{:X}", rnd2).as_str();
        hash += format!("{:X}", rnd3).as_str();
        
        hash
    }

}

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

        RustyFile { content, path , id }
    }


    pub fn create(&mut self, mut file_name:String ,relative_path:&str, 
        vault_path:String, files:Vec<String>){
        
        self.id = RustyHashser::new().make_hash();
        if file_name == String::new() {
            if self.path == String::new() {
                file_name = FILE_DEFAULT_NAME.to_string();
                file_name += &self.id.clone()[self.id.len() - 12..];
            }
        }
        
        
        //Concat path
        self.path = format!("{}{}", vault_path, relative_path);
        let full_path = format!("{}{}{}", self.path, file_name, RSFILE_EXTENSION);
        let path = Path::new(full_path.as_str());
        
        println!("{}", path.to_str().unwrap());

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
                file_name);
        }
    }


    fn write_data(&mut self, full_path:String, 
        path:String, rsf_title:String)-> Result<()> {

        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false) // Agregar contenido al final en lugar de sobrescribir
        .open(full_path.as_str())?;
        
        let mut _file_content = String::new();
        _file_content = format!(".H1 {}", rsf_title);

        file.write_all(_file_content.as_bytes())?;

        println!("Texto agregado. {}\n{}", path, full_path);

        Ok(())
    }


    pub fn read(&mut self){
        let path = format!("{}{}" ,self.path.clone(),RSFILE_EXTENSION);
        
        // Leer el contenido del archivo
        match read_to_string(path.clone()) {
            Ok(file_content) => {
                self.content = file_content;
                
                println!("{}:{}\n{}",path, self.id.clone(),self.content.clone())
            }

            Err(e) => { println!("Error al leer el archivo: {}", e); }
        }
    }

    
    pub fn update(&mut self, folder_path:&str, content:String){
        //Get path
        let full_path = format!("{}{}{}",folder_path,self.path, RSFILE_EXTENSION);
        let _ = self.write_data(full_path,
                    self.path.clone(),
                    content);
    }

}

