use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Res {
    // The endpoint returns userId, but serde converts it to "user_id" for us
    // with this field attribute.
    //
    // Alternatively we could do:
    //      #[serde(alias = "userId"))]
    #[serde(rename(deserialize = "userId"))]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

fn main() -> Result<(), failure::Error> {
    // Lets hope this endpoint doesn't change format
    let mut ret = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")?;
    dbg!(&ret);

    let jret = ret.json::<Res>()?;
    dbg!(jret);
    Ok(())
}
