use serde::{Serialize, Deserialize, Deserializer};

#[derive(Clone)]
pub enum EventType {
    Hello,
    Goodbye,
}

#[derive(Deserialize, Clone)]
pub struct CustomEvent {
    pub event_type: EventType,
    pub name: String,
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            &EventType::Hello => "hello",
            &EventType::Goodbye => "goodbye",
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hello" => EventType::Hello,
            "goodbye" => EventType::Goodbye,
            _ => panic!("Invalid event type")
        })
    }
}