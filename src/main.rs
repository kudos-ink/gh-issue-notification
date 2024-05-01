use aws_lambda_events::event::ses::SimpleEmailEvent;
use lambda_runtime::{
    run, service_fn,
    tracing::{self, error, info},
    Error, LambdaEvent,
};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    owner: String,
    repo: String,
    issue_number: u64,
}

async fn function_handler(event: LambdaEvent<SimpleEmailEvent>) -> Result<Response, Error> {
    info!("Event payload: {:?}", event.payload);
    let records = event.payload.records;

    let message_id = records
        .first()
        .ok_or_else(|| {
            error!("No email record found");
            Error::from("No email record found")
        })?
        .ses
        .mail
        .message_id
        .as_ref()
        .ok_or_else(|| {
            error!("No message id found");
            Error::from("No message id found")
        })?;

    let trimmed = message_id.trim_matches(&['<', '>']);
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() < 4 {
        error!("Malformed messaged id: {message_id}");
        return Err(Error::from(format!("Malformed messaged id: {message_id}")));
    }

    let owner = parts[0].to_string();
    let repo = parts[1].to_string();

    let issue_number = parts[3].parse::<u64>().map_err(|e| {
        error!("Failed to parse issue number: {}", e);
        Error::from(format!("Error parsing issue number: {}", e))
    })?;

    info!("Output - owner: {owner}, repo: {repo}, issue_number: {issue_number}");
    Ok(Response {
        owner,
        repo,
        issue_number,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
