use mustache::MapBuilder;
use serde::Deserialize;

use std::io::{BufReader, Error, Seek, SeekFrom, Write};
use std::fs::File;


static TEMPLATE_PATH: &str = "./examples/template";
static TEST_CONFIG_FILE: &str = "./examples/config.json";

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Shard {
    pub host: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
struct Config {
    pub domain_name: String,
    pub shards: Vec<Shard>,
    pub snaplinks_cleanup_required: Option<bool>,
}
    
fn main() -> Result<(), Error> {
    // Ignore missing file.
    std::fs::remove_file(TEST_CONFIG_FILE).unwrap_or(());

	let vars = MapBuilder::new()
        .insert_str("DOMAIN_NAME", "fake.joyent.us")
        .insert_bool("SNAPLINKS_CLEANUP_REQUIRED", true)
        // A Vec of unnamed Maps (Objects)
        .insert_vec("INDEX_MORAY_SHARDS", |builder| {
            builder
                .push_map(|bld| {
                    bld
                        .insert_str("host", "1.fake.joyent.us")
                        .insert_bool("last", true)
                })
        })
        .build();

	let template_str = std::fs::read_to_string(TEMPLATE_PATH)?;

    let config_data = mustache::compile_str(&template_str)
        .and_then(|t| {
            t.render_data_to_string(&vars)
        }).expect("render template");

	let mut file = File::create(TEST_CONFIG_FILE)?;

	file.write_all(config_data.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;
    drop(file);

    let file = File::open(TEST_CONFIG_FILE).expect("open file");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("deserialize");

    dbg!(config);
    Ok(())
}
