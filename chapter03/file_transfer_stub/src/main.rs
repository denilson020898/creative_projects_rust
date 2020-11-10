// Test it with the following commands:
// curl -X DELETE http://localhost:8080/datafile.txt
// curl -X GET http://localhost:8080/datafile.txt
// curl -X PUT http://localhost:8080/datafile.txt -d "File contents."
// curl -X POST http://localhost:8080/data -d "File contents."
// curl -X GET http://localhost:8080/a/b
//
// after running the second command, the client should have printed:
// Contents of the file.
//
// After running all five commands, the server should have printed:
// Listening at address 127.0.0.1:8080 ...
// Deleting file "datafile.txt" ... Deleted file "datafile.txt"
// Downloading file "datafile.txt" ... Downloaded file "datafile.txt"
// Uploading file "datafile.txt" ... Uploaded file "datafile.txt"
// Uploading file "data_*.txt" ... Uploaded file "data_17.txt"
// Invalid URI: "/a/b"

use actix_web::{web, web::Path, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io::Write;

fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

fn delete_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("deleting file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Delete the file

    println!("deleted file \"{}\"", filename);
    HttpResponse::Ok()
}

fn download_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("downloading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: read the contents of the file
    let contents = "Contents of the file.\n".to_string();

    println!("downloaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(contents)
}

fn upload_specified_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("uploading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: get from the client the contents to write info the file
    let _contents = "Contents of the file.\n".to_string();

    // TODO: create the file and write the contents into it

    println!("uploaded file \"{}\"", filename);
    HttpResponse::Ok()
}

fn upload_new_file(info: Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("uploading file \"{}*.txt\" ... ", filename);
    flush_stdout();

    // TODO: get from the client the contents to write info the file
    let _contents = "Contents of the file.\n".to_string();

    // TODO: generate new filename and create that file
    let file_id = 17;
    let filename = format!("{}{}.txt", filename, file_id);

    // TODO: write the contents into that file

    println!("uploaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(filename)
}

fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

fn main() -> std::io::Result<()> {
    let server_addr = "127.0.0.1:8080";
    println!("listening at address {} ...", server_addr);
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{filename}")
                    .route(web::delete().to(delete_file))
                    .route(web::get().to(download_file))
                    .route(web::put().to(upload_specified_file))
                    .route(web::post().to(upload_new_file)),
            )
            .default_service(web::route().to(invalid_resource))
    })
    .bind(server_addr)?
    .run()
}
