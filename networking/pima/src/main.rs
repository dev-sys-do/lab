use std::fs;
use std::io::Read;
use std::net::TcpListener;
use std::thread;
use std::thread::JoinHandle;

use micro_http::{Body, Method, Request, RequestError, Response, StatusCode, Version};

#[derive(Debug)]
pub enum PimaError {
    IoError(std::io::Error),

    HttpRequestError(RequestError),
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_IP: &str = "0.0.0.0";
const DEFAULT_SERVER: &str = "pima 0.1";
const DEFAULT_404: &str = "/tmp/pima/404.html";
const DEFAULT_STATIC_HTML: &str = "/tmp/pima/static";

fn http_response(req_data: &[u8]) -> std::result::Result<Response, PimaError> {
    let not_found = Body::new(fs::read_to_string(DEFAULT_404).map_err(PimaError::IoError)?);
    let request = Request::try_from(req_data).map_err(PimaError::HttpRequestError)?;

    let path = request.uri().get_abs_path();
    let method = request.method();

    println!("HTTP request: method [{:?}] path [{}]", method, path);

    let (status, body) = match method {
        Method::Get => fs::read_to_string(format!("{}/{}", DEFAULT_STATIC_HTML, path))
            .map_or((StatusCode::NotFound, Some(not_found)), |f| {
                (StatusCode::OK, Some(Body::new(f)))
            }),
        _ => (StatusCode::MethodNotAllowed, None),
    };

    let mut response = Response::new(Version::Http11, status);
    response.set_server(DEFAULT_SERVER);
    if let Some(body) = body {
        response.set_body(body);
    }

    Ok(response)
}

fn main() -> std::result::Result<(), PimaError> {
    let server = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT))
        .map_err(PimaError::IoError)?;

    println!(
        "Waiting for incoming clients on {}:{}",
        DEFAULT_IP, DEFAULT_PORT
    );

    for stream in server.incoming() {
        let mut stream = stream.map_err(PimaError::IoError)?;

        let _: JoinHandle<std::result::Result<(), PimaError>> = thread::spawn(move || {
            let mut client_data = [0; 1024];

            println!(
                "New client {:?}",
                stream.peer_addr().map_err(PimaError::IoError)?
            );

            stream.read(&mut client_data).map_err(PimaError::IoError)?;

            let response = http_response(&client_data)?;
            response
                .write_all(&mut stream)
                .map_err(PimaError::IoError)?;

            Ok(())
        });
    }

    Ok(())
}
