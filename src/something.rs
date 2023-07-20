// use serde::{ Serialize, Deserialize };
// use std::{collections::HashMap, net::SocketAddr};

// //struct for a http request
// pub struct Request {
//     pub method: HttpMethod,
//     pub uri: String,
//     pub version: String,
//     pub headers: HashMap<String, String>,
//     pub body: Option<String>
// }

// //struct for HttpResponse
// #[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
// pub struct Response {
//     pub version: String,
//     pub status_code: u16,
//     pub status_text: String,
//     pub headers: HashMap<String, String>
// }


// pub enum HttpStatusCode {
//     #[default]
//     Success = 200, 
//     BadRequest = 400, 
//     Unauthorized = 401, 
//     Forbidden = 403, 
//     NotFound = 404, 
//     MethodNotAllowed = 405, 
//     NotAcceptable = 406, 
//     Conflict = 409,
//     InternalServerError = 500,
//     NotImplemented = 501,
//     BadGateway = 502,
//     ServiceUnavailable = 503
// }

// #[derive(Debug, Clone)]
// pub enum HttpError {
//     BadRequest(Response, HttpStatusCode, &'static str),
//     Unauthorized(Response, HttpStatusCode, &'static str), 
//     Forbidden(Response, HttpStatusCode, &'static str), 
//     NotFound(Response, HttpStatusCode, &'static str), 
//     MethodNotAllowed(Response, HttpStatusCode, &'static str), 
//     NotAcceptable(Response, HttpStatusCode, &'static str), 
//     Conflict(Response, HttpStatusCode, &'static str),
//     InternalServerError(Response, HttpStatusCode, &'static str),
//     NotImplemented(Response, HttpStatusCode, &'static str),
//     BadGateway(Response, HttpStatusCode, &'static str),
//     ServiceUnavailable(Response, HttpStatusCode, &'static str)
// }

// #[serive(Eq, Hash, PartialEq, Clone)]
// struct Route {
//     method: HttpMethod,
//     path: String
// }

// pub trait Middleware: Send + Sync {
//     fn on_request<'a>(&self, request: Request) -> FutureRequest<'a>;
//     fn on_response<'a>(&self, response: Request) -> FutureResponse<'a>;
// }

// #[derive(Clone)]
// pub struct Server {
//     address: SocketAddr,
//     routes: HashMap<Route, RouteHandler>
// }
// impl Server {
//     pub async fn run(&self) -> std::io::Result<()> {
//         let address = self.address;
//         let listener = TcpListener::bind(address).await?;
//         println!("{} listening on {}", "server".green(), address.to_string().red());

//         loop {
//             let (mut stream, _) = listener.accept().await?;
//             let routes = self.routes.clone();
//             let middleware = Arc::clone(&self.middleware);        
            
//             tokio::spawn(async move {
//                 let mut buffer= [0, 1024];
//                 let _ = stream.read(&mut buffer).await.unwrap();
//                 let request = parse_request(&buffer).unwrap();
//                 let future_response = handle_route
//             });
//         }
//     }
// }
// async fn handle_route<'a>(request: Request, routes: &'a HashMap<Route, RouteHandler>, middleware: &'a)
// fn main() {

// }