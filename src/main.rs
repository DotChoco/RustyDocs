//Tokio Imports
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Importar las extensiones de lectura y escritura asincrónica
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::Notify;

//RustyDocs Imports
use rsdocs_manager::server_rsdocs::rs_server::*;

//RustyDocs Mods
mod rsdocs_manager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    //Open the port and set the ip that use to listen
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    let shutdown_notify = Arc::new(Notify::new());

    // Clones para que puedas usarlos en otros lugares si es necesario
    let shutdown_signal_main = Arc::clone(&shutdown_signal);
    let shutdown_notify_main = Arc::clone(&shutdown_notify);

    let _body_index = 6;
    
    println!("Server Open");
    // Ejecuta el servidor en un bucle
    tokio::spawn({
        let shutdown_notify_main = Arc::clone(&shutdown_notify_main);
        async move {
            while !shutdown_signal_main.load(Ordering::Relaxed) {
                tokio::select! {
                    Ok((mut socket, _)) = listener.accept() => {
                        // Clones dentro de la conexión para evitar conflictos de ownership
                        let shutdown_signal = Arc::clone(&shutdown_signal);
                        let shutdown_notify = Arc::clone(&shutdown_notify);
                        tokio::spawn(async move {
                            let mut buffer = [0; 1024];
                            if let Ok(n) = socket.read(&mut buffer).await {
                                
                                //Get Request
                                let request = String::from_utf8_lossy(&buffer[..n]);
                                let req_lines:Vec<&str> = request.split("\r\n").collect(); 
                                
                                //Get the Verbose of request
                                let verbose = get_verbose(req_lines.clone(), 0);
                                
                                //Get the body of request
                                let body = get_body(req_lines.clone());
                                
                                // Shutdown server if the json contains the "SHUTDOWN" action
                                if body.contains("\"action\":\"SD\"") {
                                    println!("Server Shutdown");
                                    shutdown_signal.store(true, Ordering::Relaxed);
                                    shutdown_notify.notify_one();
                                } 
                                
                                //else the server will response to the client
                                else {
                                    let response = action_by_verbose(verbose, body.to_string());
                                    let _ = socket.write_all(response.as_bytes()).await;
                                }


                            }
                        });
                    }
                    _ = shutdown_notify_main.notified() => {
                        break;
                    }
                }
            }
        }
    });

    // Espera a que el servidor finalice
    shutdown_notify_main.notified().await;
    Ok(())
    
}


fn get_verbose(req_lines:Vec<&str>, index:usize)->String{
    let _req_verbose = *(req_lines.get(index).unwrap());
    let _req_verbose_vec:Vec<&str> = _req_verbose.split('/').collect();
    let verbose:String = (_req_verbose_vec.get(index).unwrap())
                                        .to_string().replace(" ", "");
    verbose
}


fn get_body(req_lines:Vec<&str>)->String{
    let mut body = String::new();
    let mut json_detected = false;

    for line in req_lines {
        if line.contains("{") {
            json_detected = true;

        }
    
        if json_detected {
            body += line;
        }
    }

    body
}


fn action_by_verbose(_verbose:String, body:String) -> String {
    let mut rs_request = RsCRequest::new();
    let mut _http_response = String::new();

    //Make the action
    let _rs_response = rs_request.run_action(body);
    
    //Convert the response to json
    let response = serde_json::to_string(&_rs_response).unwrap();

    //http code like "200"
    let http_res_code = rs_request.http_response_code(_rs_response.log);


    //struct for the http response
    if rs_request.client == "RustyDocsGUI"{
        _http_response = format!("{}", response);
    }
    else{
        _http_response = format!(
            "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            http_res_code,
            response.len(),
            response
        );
    }
    
    
    _http_response
}


fn _test_file_html(){
    // let path = "C:\\Users\\carlo\\Downloads\\Dev\\RustyDocs.rdcs".to_string();
    // let mut html = html_doc::HTMLDoc::new(path);
    // html.make_file();
}

