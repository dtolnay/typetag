// cargo run --example web_event

use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "type")]
trait WebEvent {
    fn inspect(&self);
}

#[derive(Serialize, Deserialize)]
struct PageLoad;

#[typetag::serde]
impl WebEvent for PageLoad {
    fn inspect(&self) {
        println!("200 milliseconds or bust");
    }
}

#[derive(Serialize, Deserialize)]
struct Click {
    x: i32,
    y: i32,
}

#[typetag::serde]
impl WebEvent for Click {
    fn inspect(&self) {
        println!("negative space between the ads: x={} y={}", self.x, self.y);
    }
}

fn main() -> serde_json::Result<()> {
    let page_load = PageLoad;
    let event = &page_load as &dyn WebEvent;
    let json = serde_json::to_string(event)?;
    println!("PageLoad json: {}", json);
    let de: Box<dyn WebEvent> = serde_json::from_str(&json)?;
    de.inspect();

    println!();

    let click = Click { x: 10, y: 10 };
    let event = &click as &dyn WebEvent;
    let json = serde_json::to_string(event)?;
    println!("Click json: {}", json);
    let de: Box<dyn WebEvent> = serde_json::from_str(&json)?;
    de.inspect();

    Ok(())
}
