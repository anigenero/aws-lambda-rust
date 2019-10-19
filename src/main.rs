#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;
use std::error::Error;

mod event;

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())

}

fn my_handler(e: event::CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if let event::EventType::Hello = e.event_type {
        Ok(CustomOutput {
            message: format!("Hello, {}!", e.name)
        })
    } else {
        Ok(CustomOutput {
            message: format!("Goodbye, {}!", e.name)
        })
    }
}