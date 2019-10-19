#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;
use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    pub input: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    result: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())

}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        result: format!("Message was: {}", e.input)
    })
}