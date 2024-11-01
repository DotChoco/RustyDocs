
use std::fs::read_to_string;

pub struct HTMLConverter{
    vault_path:String,
    items:Vec<(String, String)>
}

impl HTMLConverter {
    pub fn new(vault_path:String)->HTMLConverter{
        let items:Vec<(String, String)> = Vec::new();
        HTMLConverter{
            vault_path,
            items,
        }
    }

}