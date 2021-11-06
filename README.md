# Packer

Packer is a simple key value messagepack store
Inspired by [kwik](https://deno.land/x/kwik/)
It uses your local storage

## Example

```rs
use packer::Packer:
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32
}

let store = Packer::<User>::new(Some("data".to_owned()));
store.set("user1", User {name: "useer1", age: 16});
let user = store.get("user1");
```
