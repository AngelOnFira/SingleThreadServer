//! Server module is responsible for the start-up and handling of server requests

use std::net::TcpListener;
use std::net::TcpStream;

use std::io::prelude::*;

pub mod file_reader;
pub mod http;
pub mod method_logic;

mod logger;
mod setting;

pub use logger::log;

use http::method::Method;
use http::response::response::Response;
use http::response::response_status_code::ResponseStatusCode;

use setting::ServerSetting;
use std::collections::HashMap;

/// start up server on specified address and port
///
/// # Examples
/// ```
/// let logic: MethodLogic = MethodLogic {
///     get: MethodLogic::default_get_logic(),
///     head: MethodLogic::default_not_allowed_logic(),
///     post: MethodLogic::default_not_allowed_logic(),
///     put: MethodLogic::default_not_allowed_logic(),
///     delete: MethodLogic::default_not_allowed_logic(),
///     connect: MethodLogic::default_not_allowed_logic(),
///     option: MethodLogic::default_not_allowed_logic(),
///     trace: MethodLogic::default_not_allowed_logic(),
///     patch: MethodLogic::default_not_allowed_logic(),
/// };
///
/// server::start(logic);
/// ```
pub fn start(method_action: method_logic::MethodLogic) {
    logger::set_up();

    let server_setting = ServerSetting::load();

    println!("{:?}", server_setting);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        server_setting.address, server_setting.port
    ))
    .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &method_action, &server_setting);
    }
}

/// handle_connection is responsible for addressing the general steps required to parse and respond to HTTP requests
fn handle_connection(
    mut stream: TcpStream,
    method_action: &method_logic::MethodLogic,
    server_settings: &ServerSetting,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let method: Method;
    let meta_data: HashMap<String, String>;
    let response: Response;

    match http::request::parse(buffer) {
        Ok(val) => {
            method = val.0;
            meta_data = val.1;

            log("Method", format!("{:?}", method));

            response = (match &method {
                Method::Get { .. } => method_action.get.clone(),
                Method::Head { .. } => method_action.head.clone(),
                Method::Post { .. } => method_action.post.clone(),
                Method::Put { .. } => method_action.put.clone(),
                Method::Delete { .. } => method_action.delete.clone(),
                Method::Connect { .. } => method_action.connect.clone(),
                Method::Options { .. } => method_action.option.clone(),
                Method::Trace { .. } => method_action.trace.clone(),
                Method::Patch { .. } => method_action.patch.clone(),
            })(method, &server_settings, &meta_data);
        }
        Err(err) => {
            log("Parse Failure", format!("Failure:{:?}", err));
            response = Response {
                status: ResponseStatusCode::BadRequest,
                body: Option::None,
            };
        }
    }

    log("response", format!("{:?}", response.status));

    response.as_bytes().iter().try_for_each(|byte| {
        if let Err(err) = stream.write(&[*byte]) {
            println!("{}", err);
            log("Send Error", format!("{}", err));
            return None;
        }
        Some(())
    });

    stream.flush().unwrap();
}
