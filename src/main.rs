mod lingua;
mod eg;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::{Map, Value};
use quick_xml::{events::Event, Reader};
use lingua::ga::gramadoir::grammatical_errors;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input { teacs: String }

fn xml_error_to_json(s: &str) -> Value {
    let mut rdr = Reader::from_str(s);
    
    rdr.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut obj = Map::new();

    loop {
        match rdr.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                for a in e.attributes().with_checks(false) {
                    if let Ok(a) = a {
                        let k = String::from_utf8_lossy(a.key.as_ref()).to_string();
                        let v = a.unescape_value().map(|v| v.into_owned()).unwrap_or_default();
                        obj.insert(k, Value::String(v));
                    }
                }
                break;
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    Value::Object(obj)
}

#[post("/api/gramadoir/1.0")]
async fn check(payload: web::Json<Input>) -> impl Responder {
    match grammatical_errors(&payload.teacs) {
        Ok(xml_errs) => {
            let json_errs: Vec<Value> = xml_errs.into_iter().map(|s| xml_error_to_json(&s)).collect();
            HttpResponse::Ok().json(json_errs)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e })),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Gramadoir Rust server on 0.0.0.0:5000…");
    HttpServer::new(|| App::new().service(check))
        .workers(1)                // <- keep Perl on one thread
        .bind(("0.0.0.0", 5000))?
        .run()
        .await
}
