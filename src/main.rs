use rsdocs_manager::rsdocs_manager::RsDocsManger;


mod pdf_converter;
mod html_converter;
mod rsdocs_manager;

fn main() {
    
    RsDocsManger::init();
    
}


fn _test_file_html(){
    // let path = "C:\\Users\\carlo\\Downloads\\Dev\\RustyDocs.rdcs".to_string();
    // let mut html = html_doc::HTMLDoc::new(path);
    // html.make_file();
}

