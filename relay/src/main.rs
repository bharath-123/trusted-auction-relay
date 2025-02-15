use protobuf_core::generated::relay_protos::helloworld::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use tonic::{service::interceptor, transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    // The stack of middleware that our service will be wrapped in
    let layer = tower::ServiceBuilder::new()
        // Interceptors can be also be applied as middleware
        .into_inner();

    Server::builder()
        .layer(layer)
        .trace_fn(|_| tracing::info_span!("greeter"))
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
