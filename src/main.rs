use html_converter::html_con;

mod pdf_converter;
mod html_converter;

fn main() {
    let path = "C:\\Users\\carlo\\Downloads\\Dev\\RustyDocs.rdcs".to_string();
    let mut html = html_con::HTMLConverter::new(path);
    html.make_file();
}
