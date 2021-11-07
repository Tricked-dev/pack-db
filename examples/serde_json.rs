use pack_db::PackDb;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

fn main() {
    let db = PackDb::<Value>::new(Some("test".to_owned()));
    db.set(
        "data",
        json!({
            "data": "more data",
            "nested_values": {
                "data2": "data",
                "more": {
                    "nested": {
                        "values": {
                            "are": {
                                "here": []
                            }
                        }
                    }
                }
            },

        }),
    )
    .unwrap();

    let data = db.get("data").unwrap();

    println!("{:#}", data);
}
