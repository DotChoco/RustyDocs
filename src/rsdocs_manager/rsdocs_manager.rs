
use chrono::Utc;
use rand::Rng;
use super::{rusty_file::*, rsdocs_config::*};

/// Combination of Date(MM/DD/YYYY), Hour(HH:MM) 
/// and the Path where it will create
pub struct RustyHashser { hash:String } 

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

pub struct RsDocsManger;

impl RsDocsManger {
    
    pub fn init(){
    
    // let mut _da = RustyFile::new();
    // _da.path = "Untitled438322167221".to_string();
    // _da.update(
    //     "C:/Users/carlo/Downloads/Docs",
    //     ".H2 Como estas\n.P Me la pelas".to_string());
    
    let mut _a = RsDocsConfig::new();
    _a.load_vault_path("C:/Users/carlo/Downloads/Docs".to_string());
    _a.load_config_file();
    


    let def_path = format!("{}{}",_a.vault_path, DEFAULT_RELATIVE_PATH);
    let _da = RustyFile::new()
                .create(
                    def_path.as_str(),
                    _a.get_ids());
    }

    

}








