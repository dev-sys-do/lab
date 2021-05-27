use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
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
const DEFAULT_ROOT: &str = "/tmp/pima";
const DEFAULT_SERVER: &str = "pima 0.1";
const DEFAULT_404: &str = "404.html";

#[derive(Debug, Clone)]
struct PimaServer {
    ip: String,
    port: u16,
    root: String,
}

impl PimaServer {
    fn new(ip: &str, port: u16, root: &str) -> Self {
        PimaServer {
            ip: String::from(ip),
            port,
            root: String::from(root),
        }
    }

    fn listen(&self) -> std::result::Result<TcpListener, PimaError> {
        println!(
            "Listening for incoming clients on {}:{}",
            self.ip, self.port
        );

        TcpListener::bind(format!("{}:{}", self.ip, self.port)).map_err(PimaError::IoError)
    }

    fn http_response(&self, req_data: &[u8]) -> std::result::Result<Response, PimaError> {
        let not_found = Body::new(
            fs::read_to_string(format!("{}/{}", self.root, DEFAULT_404))
                .map_err(PimaError::IoError)?,
        );
        let request = Request::try_from(req_data).map_err(PimaError::HttpRequestError)?;

        let path = request.uri().get_abs_path();
        let method = request.method();

        println!("HTTP request: method [{:?}] path [{}]", method, path);

        let (status, body) = match method {
            Method::Get => {
                fs::read_to_string(format!("{}/{}", format!("{}/static", self.root), path))
                    .map_or((StatusCode::NotFound, Some(not_found)), |f| {
                        (StatusCode::OK, Some(Body::new(f)))
                    })
            }
            _ => (StatusCode::MethodNotAllowed, None),
        };

        let mut response = Response::new(Version::Http11, status);
        response.set_server(DEFAULT_SERVER);
        if let Some(body) = body {
            response.set_body(body);
        }

        Ok(response)
    }

    fn handle_http_connection(&self, mut stream: &TcpStream) -> std::result::Result<(), PimaError> {
        let mut stream_data = [0; 1024];

        println!(
            "New client {:?}",
            stream.peer_addr().map_err(PimaError::IoError)?
        );

        stream.read(&mut stream_data).map_err(PimaError::IoError)?;

        let response = self.http_response(&stream_data)?;
        response
            .write_all(&mut stream)
            .map_err(PimaError::IoError)?;

        Ok(())
    }
}

fn main() -> std::result::Result<(), PimaError> {
    let pima_server = PimaServer::new(DEFAULT_IP, DEFAULT_PORT, DEFAULT_ROOT);
    let server = pima_server.listen()?;

    for stream in server.incoming() {
        let stream = stream.map_err(PimaError::IoError)?;
        let pima = pima_server.clone();

        let _: JoinHandle<std::result::Result<(), PimaError>> =
            thread::spawn(move || pima.handle_http_connection(&stream));
    }

    Ok(())
}
