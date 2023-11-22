use serde::{Serialize, Deserialize};
use rmp_serde::{Serializer, Deserializer};
use std::io::Cursor;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Data {
    data: Vec<u8>,
}