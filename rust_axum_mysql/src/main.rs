/**
    axum: https://docs.rs/axum/latest/axum/index.html
*/
use axum::{
    Router, 
    routing::{ get }, 
    response::{IntoResponse, Response}, 
    Json,
    middleware::{self, Next},
    http::{HeaderMap, StatusCode},
    extract::Request,
};

use tower::ServiceBuilder;

/**
    IntoResponse (is a trait) automatically converts your return values to proper HTTP responses. 
    What do you mean by that?

    Without IntoResponse you need to manually add status, header etc.
        
        use axum::http::{Response, StatusCode};

        async fn manual_handler() -> Response<String> {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/plain")
                .body("Hello, World!".to_string())
                .unwrap()
        }

    With the IntoResponse the above code can be reduced as below
        
        async fn simple_handler() -> impl IntoResponse {
            "Hello, World!"  // Automatically becomes 200 OK with text/plain
        }

    IntoResponse is a zero-cost abstraction - the conversion happens at compile time, not runtime. 
    The trait methods are typically inlined, so there's no performance penalty for using this convenient API.
*/

async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Rust Server is running";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

/**
    Why do we need to wrap json responses with axum::Json?

    axum::Json wrapper is needed because:
        - HTTP Content-Type: Sets the correct content-type: application/json header
        - Serialization: Automatically converts Rust structs to JSON strings
        - Type Safety: Provides compile-time guarantees about JSON serialization
        - Protocol Compliance: Ensures proper HTTP response format
*/

async fn hello_world() -> impl IntoResponse {
    const MESSAGE: &str = "Hello World";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    
    Json(json_response)
}

/**
    What is axum::routing::Trace? How is it different from tracing?
        axum::routing::trace refers specifically to routing HTTP TRACE method requests to a handler, 
        not request tracing or logging.
        It is used to assign a handler to respond to HTTP TRACE requests, 
        similar to get or post routing functions.

        If you want to trace (log and instrument) all requests and responses in an Axum app for observability 
        (which is usually what "trace requests" means), you should use tower_http::trace::TraceLayer

        What is HTTP TRACE? (Interesting)
            The HTTP TRACE method is one of the less commonly used HTTP verbs. 
            It is defined in the HTTP/1.1 specification and is intended for diagnostic purposes. 
            
            TRACE asks a web server to return the received request (including headers and body) as the response, 
            allowing the client to see how the request was received by the server along the path. 
            This can be useful for debugging proxies, firewalls, or custom network setups to 
            determine if and how intermediate components are modifying the request.

    How to trace requests?
        To trace requests in Axum, the recommended approach is to use the tracing crate 
        along with tower-http's TraceLayer middleware.
    
    What is tower & tower_http?

        tower is a library of modular, reusable components and middleware for building robust networking clients 
        and servers. It is protocol-agnostic, meaning its middleware works across various protocols and does not 
        have awareness of specific protocol details like HTTP status codes or headers. For example, a timeout 
        middleware in tower can apply to any service regardless of whether it uses HTTP or some other protocol.

        tower_http is built on top of tower and provides HTTP-specific middleware and utilities. It uses the http 
        and http-body crates, making it compatible with HTTP clients and servers like hyper, tonic, and warp. This 
        means tower_http has middleware that understands HTTP features such as headers, status codes, compression, 
        CORS, tracing, and request validation. It offers middleware tailored to HTTP applications, such as setting 
        response headers, compressing response bodies, propagating headers, and more
*/

async fn route_layer_middleware(headers: HeaderMap,req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("Request headers: {:?}", headers);
    Ok(next.run(req).await)
}

async fn layer_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("Layer middleware");
    Ok(next.run(req).await)
}

/**
    Understanding middleware
    
    let app: Router = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(hello_world))
        .route_layer(middleware::from_fn(route_layer_middleware))
        .route("/sample", get(health_check))
        .layer(middleware::from_fn(layer_middleware))
        .route("/sample2", get(health_check));

    what is the difference between .layer and .route_layer?
        .layer() is a global middleware
        .route_layer() if a route specific middleware

    When to use ServiceBuilder? What is the use of it?
        1. Performance Optimisation: 
            ServiceBuilder creates a single, optimized middleware stack 
            instead of wrapping each layer individually

            // Less efficient - creates nested wrapper for each layer
            .layer(A).layer(B).layer(C)

            // More efficient - single optimized stack  
            .layer(ServiceBuilder::new().layer(A).layer(B).layer(C))

        2. Better Type infernce: 
            ServiceBuilder helps Rust's type system handle complex middleware stacks more efficiently, 
            reducing compilation time and improving error messages.

        3. Conditional middleware
            
            let mut builder = ServiceBuilder::new().layer(CorsLayer::permissive());

            if config.enable_auth {
                builder = builder.layer(AuthLayer::new());
            }

            if config.enable_compression {
                builder = builder.layer(CompressionLayer::new());
            }

        4. Reusability:
            You can define the the service builder within a function and return it. And reuse 
            multiple times the same middlewares.

        5. Clear Organisation

*/

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(hello_world))
        .route_layer(middleware::from_fn(route_layer_middleware))
        .route("/sample", get(health_check))
        .route("/sample2", get(health_check))
        .layer(middleware::from_fn(layer_middleware));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await;
    
    match listener {
        Ok(listener) => {
            println!("Server is running on http://127.0.0.1:3001");
            axum::serve(listener, app).await.unwrap();
        }
        Err(e) => {
            eprintln!("Error binding to port 3000: {}", e);
        }
    }
}