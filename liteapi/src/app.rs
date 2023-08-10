use rustc_hash::FxHashMap;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::string::ToString;
use threadpool::ThreadPool;
use crate::{Handler, Routes};
use crate::{is_http_status_code, parse_query_string, extract_method_and_path};
use crate::http::{Response, StatusCode, QueryParams};
use futures_executor::block_on;
use num_cpus;
use crate::openapi::openapi;

fn handle_client(mut stream: TcpStream, routes: Routes) {
    // Read request from client
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    // Generate response
    let response = block_on(handle_request(request.to_string(), routes));

    // Send response to client
    stream.write(response.as_bytes()).unwrap();

    // Close client connection
    drop(stream);
}

async fn handle_request(request: String, routes: Routes) -> String {
    // Extract headers from request
    let headers: Vec<&str> = request.split("\r\n").collect();

    // Extract method and path from headers
    let Some((method, path)) = extract_method_and_path(headers).await else {
        return "Something went wrong!".to_owned();
    };

    // Extract query pairs from path
    let query_pairs = parse_query_string(&path).await;

    let stripped_path: &str = path.split('?').next().unwrap_or(&path);

    // Check if the route is an OpenAPI route and if then return directly
    if stripped_path == "/openapi.json/".to_owned() || stripped_path == "/openapi.json" {
        unsafe {
            return format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{openapi}", openapi = openapi(&routes).await)
        }
    }

    handle_route(method, query_pairs, stripped_path, routes).await
}

async fn handle_route(method: String, query_pairs: QueryParams, path: &str, routes: Routes) -> String {
    // Check if a route exists for the requested path and method
    let route = routes.get(path);

    if route.is_none() {
        // Check if a route exists for the requested path with an additional '/' at the end
        let route = routes.get(&format!("{}/", path));
        handle_route_response(route, method, query_pairs).await
    } else {
        handle_route_response(route, method, query_pairs).await
    }
}

#[allow(unused_assignments)]
async fn handle_route_response(route: Option<&(String, Handler)>, method: String, query_params: QueryParams) -> String {
    match route {
        None => StatusCode::NotFound.parse(), // Send 404 if the path doesnt exist

        Some((route_method, handler)) => {
            if route_method.to_owned() != method {
                StatusCode::MethodNotAllowed.parse() // Send 405 if the method is not allowed on the path
            } else {
                let response = handler(query_params);

                // Content type to later use as a header to specify what we want to send
                let mut content_type = String::new();

                // Extract content type and content of response
                let response = match response {
                    Response::Json(content) => {
                        content_type = "application/json".to_owned();
                        content
                    },

                    Response::Html(content) => {
                        content_type = "text/html".to_owned();
                        content
                    },

                    Response::Plain(content) => {
                        content_type = "text/plain".to_owned();
                        content
                    },

                    Response::Css(content) => {
                        content_type = "text/css".to_owned();
                        content
                    },

                    Response::Javascript(content) => {
                        content_type = "application/javascript".to_owned();
                        content
                    },

                    Response::Jpeg(content) => {
                        content_type = "image/jpeg".to_owned();
                        content
                    },

                    Response::Png(content) => {
                        content_type = "image/png".to_owned();
                        content
                    },

                    Response::FormData(content) => {
                        content_type = "multipart/form-data".to_owned();
                        content
                    },
                };

                // Check if the response is a http status code and if send it directly without formatting
                if is_http_status_code(&response).await {
                    return response;
                }

                format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", content_type, response) // Send 200 if the method and path are correct
            }
        }
    }
}

pub struct LiteAPI {
    pub(crate) routes: Routes,
}

#[allow(dead_code)]
impl LiteAPI {
    pub async fn new() -> Self {
        Self {
            routes: FxHashMap::default(),
        }
    }

    async fn add_route(&mut self, path: &str, method: &str, handler: Handler) {
        self.routes.insert(path.to_string(), (method.to_string(), handler));
    }

    pub async fn get(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "GET", handler).await;
        return self
    }

    pub async fn post(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "POST", handler).await;
        return self
    }

    pub async fn put(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "PUT", handler).await;
        return self
    }

    pub async fn delete(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "DELETE", handler).await;
        return self
    }

    pub async fn patch(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "PATCH", handler).await;
        return self
    }

    pub async fn head(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "HEAD", handler).await;
        return self
    }

    pub async fn trace(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "TRACE", handler).await;
        return self
    }

    pub async fn connect(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "CONNECT", handler).await;
        return self
    }

    pub async fn options(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_route(path, "OPTIONS", handler).await;
        return self
    }

    pub async fn run(&self) {
        // Setup listener
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        // Log the startup
        println!("INFO:     API Server at http://127.0.0.1:7878");

        // Initialize thread pool to enhance speed
        let pool = ThreadPool::new(num_cpus::get());

        // Listen for incoming requests
        loop {
            let (stream, _) = listener.accept().unwrap();
            let routes = self.routes.clone();

            pool.execute(move || {
                handle_client(stream, routes);
            });
        }
    }

    #[allow(unused_variables)]
    pub async fn host(&self, host: &str) {
        // Like run method but you can define your host
    }
}
