use heapless::Vec;
use nanofish::{HttpHandler, HttpResponse, ResponseBody, SmallHttpServer, StatusCode};

#[embassy_executor::task]
pub async fn webserver_task(network_stack: embassy_net::Stack<'static>) {
    let mut server = SmallHttpServer::new(80);
    let handler = WebServerHandler;

    server.serve(network_stack, handler).await;
}

struct WebServerHandler;

impl HttpHandler for WebServerHandler {
    async fn handle_request(
        &self,
        request: &nanofish::HttpRequest<'_>,
    ) -> Result<nanofish::HttpResponse<'_>, nanofish::Error> {
        match request.path {
            "/" => Ok(HttpResponse {
                status_code: StatusCode::Ok,
                headers: Vec::new(),
                body: ResponseBody::Text("<h1>Hello World!</h1>"),
            }),
            "/api/status" => Ok(HttpResponse {
                status_code: StatusCode::Ok,
                headers: Vec::new(),
                body: ResponseBody::Text("{\"status\":\"ok\"}"),
            }),
            _ => Ok(HttpResponse {
                status_code: StatusCode::NotFound,
                headers: Vec::new(),
                body: ResponseBody::Text("Not Found"),
            }),
        }
    }
}
