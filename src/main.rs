extern crate serde_json;

use serde_json::ser::PrettyFormatter;
use std::io;

fn main() {
    let mut deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let mut serializer =
        serde_json::Serializer::with_formatter(io::stdout(), PrettyFormatter::new());

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
}
