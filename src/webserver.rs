use std::env;
use std::fs;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming as IncomingBody;
use hyper::service::Service;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::{tokio::TokioIo, TokioExecutor};
use hyper_util::server::conn::auto;
use tokio::net::TcpListener;

use crate::config::*;

pub async fn start_server(config: Config) -> anyhow::Result<()> {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let server = Server {
            dir: config.out_dir.clone(),
        };

        tokio::task::spawn(async move {
            let server = server.clone();
            if let Err(err) = auto::Builder::new(TokioExecutor::new())
                .serve_connection(io, server)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Server {
    dir: String,
}

impl Service<Request<IncomingBody>> for Server {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(
            status: StatusCode,
            data: Vec<u8>,
            mime: &str,
        ) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder()
                .status(status)
                .header("Content-Type", mime)
                .body(Full::new(Bytes::from(data)))
                .unwrap())
        }

        let dir = self.dir.clone();
        fn get404(dir: &String, filename: &str) -> Vec<u8> {
            eprintln!("No such file: {}", filename);
            let path404 = env::current_dir().unwrap().join(dir).join("404.html");
            std::fs::read(path404).unwrap()
        }

        let mut filename = match (req.method(), req.uri().path()) {
            (&Method::GET, path) => path.to_string(),
            _ => {
                return Box::pin(async move {
                    mk_response(StatusCode::NOT_FOUND, get404(&dir, ""), "text/html")
                })
            }
        };

        if filename.starts_with('/') {
            filename = filename[1..].to_string();
        }
        let mut path = env::current_dir().unwrap().join(&self.dir).join(&filename);

        let is_dir: bool = match fs::metadata(&path) {
            Err(_) => {
                return Box::pin(async move {
                    mk_response(StatusCode::NOT_FOUND, get404(&dir, &filename), "text/html")
                });
            }
            Ok(o) => o.is_dir(),
        };

        if is_dir {
            path.push("index.html");
        }

        let output = match std::fs::read(&path) {
            Ok(o) => o,
            Err(_) => {
                return Box::pin(async move {
                    mk_response(StatusCode::NOT_FOUND, get404(&dir, &filename), "text/html")
                });
            }
        };

        let mimestring;
        if let Some(mimetype) = mime_guess::from_path(&path).first() {
            let mut output: String = mimetype.type_().as_str().to_string();
            output += "/";
            output += mimetype.subtype().into();
            if let Some(suffix) = mimetype.suffix() {
                output += "+";
                output += suffix.into();
            }
            mimestring = output;
        } else {
            mimestring = "application/octet-stream".to_string();
        }

        println!("Serving File: {}", filename);
        Box::pin(async move { mk_response(StatusCode::OK, output, &mimestring) })
    }
}
