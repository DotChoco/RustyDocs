//Tokio Imports
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Importar las extensiones de lectura y escritura asincrónica

//RustyDocs Imports
use rsdocs_manager::rsdocs_manager::RsDocsManger;
use rsdocs_manager::rs_server::RsCRequest;

//RustyDocs Mods
mod pdf_converter;
mod html_converter;
mod rsdocs_manager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut _rsdm =  RsDocsManger::new();

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            if let Ok(n) = socket.read(&mut buffer).await {
                
                //Get Request
                let request = String::from_utf8_lossy(&buffer[..n]);
                let req_lines:Vec<&str> = request.split("\r\n").collect(); 
                
                
                //Get the Verbose of request
                let _req_verbose = *(req_lines.get(0).unwrap());
                let _req_verbose_vec:Vec<&str> = _req_verbose.split('/').collect();
                let verbose:String = (_req_verbose_vec.get(0)
                                                        .unwrap())
                                                        .to_string()
                                                        .replace(" ", "");
                
                
                //Get the body of request
                let mut body = String::new();
                for line in req_lines.iter().enumerate() {
                    if line.0 >= 9 {
                        body += *line.1;
                    }
                }
                println!("{}", body);
                
                //Response to the client
                let response = action_by_verbose(verbose, body);
                let _ = socket.write_all(response.as_bytes()).await;
                
                // let response = json!({"message": "Hello from Rust!"}).to_string();
            }
        });
    }
    
}



fn action_by_verbose(_verbose:String, body:String) -> String {
    let mut _rs_request = RsCRequest::new();
    let _rs_response = _rs_request.run_action(body);

    let mut json_response = String::new();
    
    if true {
        json_response = serde_json::to_string(&_rs_response).unwrap();
    }

    json_response
}


fn _test_file_html(){
    // let path = "C:\\Users\\carlo\\Downloads\\Dev\\RustyDocs.rdcs".to_string();
    // let mut html = html_doc::HTMLDoc::new(path);
    // html.make_file();
}

