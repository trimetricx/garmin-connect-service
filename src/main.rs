use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // build our application with a route
    let (router, api) = OpenApiRouter::new()
        .routes(routes!(handler))
        .split_for_parts();
    let router =
        router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()));
    println!("visit api docs at http://localhost:3000/swagger-ui");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, router.into_make_service()).await;
}
#[utoipa::path(get, path= "/user", responses((status = OK, body = String)))]
async fn handler() -> String {
    String::from("hello")
}
