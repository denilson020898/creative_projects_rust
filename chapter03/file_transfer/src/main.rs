// test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data-d "File contents."
// curl -X GET http://localhost:8080/a/b

use actix_web::{web, web::Path, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::{
    future::{ok, Future},
    Stream,
};
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn flust_stdout() {
    std::io::stdout().flush().unwrap();
}

fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deleting file: \"{}\" ... ", filename);
    flust_stdout();

    match std::fs::remove_file(&filename) {
        Ok(_) => {
            println!("Deleted file: \"{}\"", filename);
            HttpResponse::Ok()
        }
        Err(error) => {
            println!("Failed to delete file: \"{}\" -> {}", filename, error);
            HttpResponse::NotFound()
        }
    }
}

fn download_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Downloading file: \"{}\" ... ", filename);
    flust_stdout();

    let read_file_contents = |filename: &str| -> std::io::Result<String> {
        use std::io::Read;
        let mut contents = String::new();
        File::open(filename)?.read_to_string(&mut contents)?;
        Ok(contents)
    };

    match read_file_contents(&filename) {
        Ok(contents) => {
            println!("Downloaded file: \"{}\"", filename);
            HttpResponse::Ok().content_type("text/plain").body(contents)
        }
        Err(error) => {
            println!("Failed to read file: \"{}\" -> {}", filename, error);
            HttpResponse::NotFound().finish()
        }
    }
}

fn upload_specified_file(
    payload: web::Payload,
    info: Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let filename = info.0.clone();
    print!("Uploading file: \"{}\" ... ", filename);
    flust_stdout();

    // get the content async-ly from the client
    payload
        .map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(move |contents| {
            let f = File::create(&filename);
            if f.is_err() {
                println!("Failed to create file: \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }

            if f.unwrap().write_all(&contents).is_err() {
                println!("Failed to create file: \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }
            println!("Uploaded file: \"{}\"", filename);
            ok(HttpResponse::Ok().finish())
        })
}

fn upload_new_file(
    payload: web::Payload,
    info: Path<(String,)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let filename_prefix = info.0.clone();
    print!("Uploading file: \"{}\" ... ", filename_prefix);
    flust_stdout();

    // get the content async-ly from the client
    payload
        .map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
        })
        .and_then(move |contents| {
            let mut rng = rand::thread_rng();
            let mut attemps = 0;
            let mut file;
            let mut filename;
            const MAX_ATTEMPS: u32 = 100;

            loop {
                attemps += 1;
                if attemps > MAX_ATTEMPS {
                    println!(
                        "failed to create new file with prefix: \"{}\" after {} attemps.",
                        filename_prefix, MAX_ATTEMPS
                    );
                    return ok(HttpResponse::NotFound().into());
                }

                filename = format!("{}{:03}.txt", filename_prefix, rng.gen_range(0, 1000));
                file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&filename);
                if file.is_ok() {
                    break;
                }
            }

            if file.unwrap().write_all(&contents).is_err() {
                println!("Failed to write file: \"{}\"", filename);
                return ok(HttpResponse::NotFound().into());
            }

            println!("Uploaded file: \"{}\"", filename);
            ok(HttpResponse::Ok().content_type("text/plain").body(filename))
        })
}

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}
fn main() -> std::io::Result<()> {
    let server_addr = "127.0.0.1:8080";
    println!("Listening at addres: {} ...", server_addr);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to_async(upload_specified_file))
                    .route(web::post().to_async(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_addr)?
    .run()
}
