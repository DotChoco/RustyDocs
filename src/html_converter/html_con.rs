use crate::html_converter::cons;

use super::cons::{Cons, DFLAGS, L_CTEM, L_OTEM, W_CCMT, W_CT, W_OCMT, W_OT};
use std::{ fs::read_to_string, process::id, ptr::null, vec};
pub struct HTMLConverter{
    file_path:String,
    flags:Vec<(DFLAGS, String)>
}

impl HTMLConverter {
    pub fn new(file_path:String)->HTMLConverter{
        let flags:Vec<(DFLAGS, String)> = Vec::new();
        HTMLConverter{
            file_path,
            flags
        }
    }

    pub fn make_file(&mut self){
        // Leer el contenido del archivo
        match read_to_string(self.file_path.clone()) {
            Ok(content) => {

                let lines:Vec<_> = content.lines().collect();
                
                for line in lines {
                    // println!("{}",line);
                    self.get_flag(line);
                }
                self.html_struct();
            }
            Err(e) => {
                println!("Error al leer el archivo: {}", e);
            }
        }
    }

    fn get_flag(&mut self, line: &str){
        let chars: Vec<char> = line.chars().collect();
        let mut word: String = String::new();
        let mut index: usize = 0;
        let mut letter: char = '\0';
        let mut ascii:u8 = u8::MIN;

        if chars.len() == 0 {
            self.data_flag(line, 0, ".P".to_string());
        }
        else {
            //Iterate per character in the line
            while index < chars.len() {
            letter = chars[index];
            ascii = letter as u8;
            
            //last element from the string 
            if index == chars.len() - 1 {
                self.data_flag(line, 0, ".P".to_string());
                break;
            }
            
            //comment detect
            if index < chars.len() - 3{
                if ascii == Cons::DOT as u8 && 
                chars[index + 1] as u8 == Cons::DOT as u8 {
                        self.data_flag(line, 0, "..".to_string());
                        break;
                }
            }

            //the item will add to "word" while the item are not a SPACE
            if ascii != Cons::SPACE as u8 {
                word += letter.to_string().as_str();
            }
            else {
                if cons::get_flag(word.clone()) != DFLAGS::NONE {
                    self.data_flag(line, index, word.clone());
                    break;
                }
            }

            index += 1;
        }
        }

    }


    fn data_flag(&mut self, line: &str, index: usize, word:String){
        let mut data = String::new();
        let chars: Vec<char> = line.chars().collect();

        for item in chars.iter().enumerate().skip(index) {
            data += item.1.to_string().as_str();
        }

        self.flags.push((cons::get_flag(word.clone()), data.clone()));

        // println!("+{}+:**{}**", cons::get_flag(word.clone()).as_ref(), data);
    }

    fn html_struct(&mut self){
        let mut html_line = String::new();
        let mut html_doc:Vec<String> = Vec::new();
        html_doc.push(L_OTEM.to_string());
        for tag in &self.flags {
            if tag.0 == DFLAGS::CMT {
                html_line += &format!("{}{}{}", W_OCMT, tag.1, W_CCMT);
                html_doc.push(html_line.clone());
                html_line = String::new();
            }
            else {
                html_line += W_OT;
                html_line += &format!("{}>{}", tag.0.as_ref(), tag.1);
                html_line += W_CT;
                html_line += &format!("{}>", tag.0.as_ref());
                html_doc.push(html_line.clone());
                html_line = String::new();
            }
        }
        html_doc.push(L_CTEM.to_string());
        
        for data in html_doc {
            println!("{}", data)
        }
    }

}