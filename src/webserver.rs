use std::env;
use std::fs;
use std::net::SocketAddr;

use bytes::Bytes;
use futures::executor::block_on;
use futures_util::TryStreamExt;
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::body::Frame;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::{tokio::TokioIo, TokioExecutor};
use hyper_util::server::conn::auto;
use tokio::{fs::File, net::TcpListener};
use tokio_util::io::ReaderStream;

pub async fn start_server() -> anyhow::Result<()> {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = 
                auto::Builder::new(TokioExecutor::new())
                    .serve_connection(io, service_fn(response))
                    .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn response (
    req: Request<hyper::body::Incoming>,
) -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> 
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => simple_file_send(path).await,
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<BoxBody<Bytes, std::io::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(block_on(simple_file_send("404.html")).unwrap().into_body())
        .unwrap()
}

async fn simple_file_send(filename: &str) 
    -> hyper::Result<Response<BoxBody<Bytes, std::io::Error>>> 
{
    let mut filename = filename.to_string();
    if filename.starts_with("/") {
        filename = filename[1..].to_string();
    }
    let mut path = env::current_dir().unwrap().join("docs").join(filename);
    println!("Serving File: {}", path.display());

    let is_dir: bool;
    match fs::metadata(&path) {
        Err(_) => {
            eprintln!("No such file: {}", path.display());
            return Ok(not_found());
        },
        Ok(o) => is_dir = o.is_dir()
    }

    if is_dir {
        path.push("index.html");
    }

    // Open file for reading
    let file = File::open(&path).await;
    if file.is_err() {
        eprintln!("ERROR: Unable to open file: {}", path.display());
        return Ok(not_found());
    }

    let file: File = file.unwrap();

    // Wrap to a tokio_util::io::ReaderStream
    let reader_stream = ReaderStream::new(file);

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = stream_body.boxed();

    // Send response
    let mut response = Response::builder()
        .status(StatusCode::OK);


    if let Some(mimetype) = mime_guess::from_path(path).first() {
        let mut output: String = mimetype.type_().as_str().to_string();
        output += "/";
        output += mimetype.subtype().into();
        if let Some(suffix) = mimetype.suffix() {
            output += "+";
            output += suffix.into();
        }
        response = response.header("Content-Type", output);
    }

    Ok(response.body(boxed_body).unwrap())
}
