use tonic::{transport::Server, Request, Response, Status};

use payments::payment_server::{Payment, PaymentServer};
use payments::{PaymentRequest, PaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct PaymentService {}

#[tonic::async_trait]
impl Payment for PaymentService {
    async fn send_payment(&self, request: Request<PaymentRequest>) -> Result<Response<PaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let reply = payments::PaymentResponse {
            success: true,
            message: format!("Sent {}BTC to {}.", request.amount, request.to).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payments = PaymentService::default();

    Server::builder()
        .add_service(PaymentServer::new(payments))
        .serve(addr)
        .await?;

    Ok(())
}
