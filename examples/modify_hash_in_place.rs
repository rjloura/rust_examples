// This example shows two ways to modify a hash element in place

use std::collections::HashMap;

#[derive(Debug, Default)]
struct MultiFields {
    fieldone: String,
    fieldtwo: String,
}

#[derive(Debug)]
struct WithHashMap {
    hash: HashMap<String, MultiFields>,
}

impl WithHashMap {
    fn update(&mut self) {
        if let Some(elem) = self.hash.get_mut("key") {
            elem.fieldone = String::from("Modified In Place");
        }
    }

    fn update_or_insert(&mut self) {
        let ent = self
            .hash
            .entry(String::from("no entry"))
            .or_insert(MultiFields::default());
        ent.fieldone = String::from("Modified In Place");
    }
}

fn main() {
    let mut hash: HashMap<String, MultiFields> = HashMap::new();
    hash.insert(
        String::from("key"),
        MultiFields {
            fieldone: String::from("field one"),
            fieldtwo: String::from("field two"),
        },
    );

    let mut with = WithHashMap { hash };

    with.update();
    with.update_or_insert();
    dbg!(with);
}
