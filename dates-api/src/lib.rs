wit_bindgen::generate!({
    world: "dates-api",
    path: "wit",
    generate_all,
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::{
    Fields, IncomingRequest, Method, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

use std::fs;

const DATES_DIR: &str = "/dates";

struct Component;

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let method = request.method();
        let path_with_query = request.path_with_query().unwrap_or_default();
        let path = path_with_query
            .split('?')
            .next()
            .unwrap_or("")
            .to_string();

        let body_bytes = match method {
            Method::Post => read_body(&request),
            _ => vec![],
        };

        let (status, content_type, body) = route(&method, &path, &body_bytes);
        send_response(response_out, status, content_type, body.as_bytes());
    }
}

fn route(method: &Method, path: &str, body: &[u8]) -> (u16, &'static str, String) {
    match (method, path) {
        (Method::Get, "/dates") | (Method::Get, "/dates/") => list_dates(),
        (Method::Post, p) if p.starts_with("/dates/") => {
            let key = &p["/dates/".len()..];
            add_date(key, body)
        }
        (Method::Delete, p) if p.starts_with("/dates/") => {
            let key = &p["/dates/".len()..];
            delete_date(key)
        }
        (Method::Get, "/") | (Method::Get, "") => help(),
        _ => (404, "text/plain", "Not Found".to_string()),
    }
}

fn help() -> (u16, &'static str, String) {
    (
        200,
        "application/json",
        r#"{"routes":["GET /dates","POST /dates/{YYYYMMDD}","DELETE /dates/{YYYYMMDD}"]}"#
            .to_string(),
    )
}

fn list_dates() -> (u16, &'static str, String) {
    match fs::read_dir(DATES_DIR) {
        Ok(entries) => {
            let mut dates: Vec<String> = entries
                .flatten()
                .filter_map(|e| {
                    let path = e.path();
                    if !path.is_file() {
                        return None;
                    }
                    let name = path.file_name()?.to_str()?.to_string();
                    if name.len() < 8 {
                        return None;
                    }
                    let key = &name[..8];
                    let label = fs::read_to_string(&path).ok()?;
                    Some(format!(
                        r#"{{"key":"{}","label":"{}"}}"#,
                        key,
                        json_escape(label.trim())
                    ))
                })
                .collect();
            dates.sort();
            (200, "application/json", format!("[{}]", dates.join(",")))
        }
        Err(e) => (500, "text/plain", format!("Error reading dates directory: {e}")),
    }
}

fn add_date(key: &str, body: &[u8]) -> (u16, &'static str, String) {
    if key.len() != 8 || !key.chars().all(|c| c.is_ascii_digit()) {
        return (
            400,
            "text/plain",
            "Key must be 8-digit YYYYMMDD format".to_string(),
        );
    }
    let label = String::from_utf8_lossy(body).to_string();
    let file_path = format!("{DATES_DIR}/{key}.txt");
    match fs::write(&file_path, &label) {
        Ok(_) => (201, "text/plain", format!("Created {key}")),
        Err(e) => (500, "text/plain", format!("Error writing file: {e}")),
    }
}

fn delete_date(key: &str) -> (u16, &'static str, String) {
    let file_path = format!("{DATES_DIR}/{key}.txt");
    match fs::remove_file(&file_path) {
        Ok(_) => (200, "text/plain", format!("Deleted {key}")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            (404, "text/plain", "Date not found".to_string())
        }
        Err(e) => (500, "text/plain", format!("Error deleting file: {e}")),
    }
}

fn read_body(request: &IncomingRequest) -> Vec<u8> {
    let Ok(body) = request.consume() else {
        return vec![];
    };
    let Ok(stream) = body.stream() else {
        return vec![];
    };
    let mut bytes = vec![];
    loop {
        match stream.blocking_read(4096) {
            Ok(chunk) if !chunk.is_empty() => bytes.extend_from_slice(&chunk),
            _ => break,
        }
    }
    bytes
}

fn send_response(response_out: ResponseOutparam, status: u16, content_type: &str, body: &[u8]) {
    let headers = Fields::new();
    let _ = headers.set(
        &"content-type".to_string(),
        &[content_type.as_bytes().to_vec()],
    );
    let response = OutgoingResponse::new(headers);
    let _ = response.set_status_code(status);
    let out_body = response.body().unwrap();
    ResponseOutparam::set(response_out, Ok(response));
    let stream = out_body.write().unwrap();
    let _ = stream.blocking_write_and_flush(body);
    drop(stream);
    OutgoingBody::finish(out_body, None).unwrap();
}

fn json_escape(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            c => vec![c],
        })
        .collect()
}

export!(Component);
