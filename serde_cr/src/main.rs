use serde::{Serialize, Deserialize};


/// JSON 
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserRecord {
    name: String,
    email: Option<String>,
    enabled: bool,
    map_location: Point,
}

impl UserRecord {
    fn new(name: String, email: Option<String>, enabled: bool, map_location: Point) -> Self {
        Self {
            name,
            email,
            enabled,
            map_location
        }
    }
}

fn serialize_user(user: UserRecord) -> String {
    let ser = serde_json::to_string(&user).unwrap();
    println!("Serialized user: {:?}", ser);
    ser
}

fn deserialize_user(user: &str) -> UserRecord {
    let de_ser: UserRecord = serde_json::from_str(&user).unwrap();
    println!("Deserialized user: {}", user);
    de_ser
}


/// BSON


fn main() {
    let point = Point { x: 44.5, y: 78.25 };
    let user = UserRecord::new(
        "And".to_owned(),
        Some("abr@bla.la".to_owned()),
        true,
        point
    );

    let ser = serialize_user(user);

    let de_ser = deserialize_user(&ser);

    println!("End");
    // // Convert the Point to a JSON string.
    // let serialized = serde_json::to_string(&point).unwrap();
    // // Prints serialized = {"x":1,"y":2}
    // println!("serialized = {}", serialized);

    // // Convert the JSON string back to a Point.
    // let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // // Prints deserialized = Point { x: 1, y: 2 }
    // println!("deserialized = {:?}", deserialized);
}
