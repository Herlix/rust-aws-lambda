use lambda::{handler_fn, Context};
use simple_logger::SimpleLogger;
use log::info;
type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use aws_lambda_events::event::s3::S3Event;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().init()?;

    lambda::run(handler_fn(resize)).await?;
    Ok(())
}

async fn resize(event: S3Event, _: Context) -> Result<String, Error> {

    let e = format!("Msg: {:?}", &event);
    info!("{}", &e);
    Ok(e)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;

//     #[tokio::test]
//     async fn hello_handles() {
//         let event = S3Event
//         assert_eq!(
//             resize(event.clone(), Context::default())
//                 .await
//                 .expect("expected Ok(_) value"),
//             event
//         )
//     }
// }
