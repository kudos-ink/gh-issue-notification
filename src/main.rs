use aws_lambda_events::{event::ses::SimpleEmailEvent, ses::SimpleEmailRecord};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    msg: String,
    success: bool,
}

async fn function_handler(event: LambdaEvent<SimpleEmailEvent>) -> Result<Response, Error> {
    let records = event.payload.records;
    let message;

    if let Some(record) = records.first() {
        if let Some(message_id) = &record.ses.mail.message_id {
            let resp = Response {
                msg: format!("The message id is: {}", message_id),
                success: true,
            };
            return Ok(resp);
        } else {
            message = "No message id found".to_string();
        }
    } else {
        message = "No mail record found".to_string();
    }
    Ok(Response {
        msg: message,
        success: false,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
