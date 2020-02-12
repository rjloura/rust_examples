// I wanted to go through a chain of Option<_>'s and for each one assign a
// unique error message if the Option was None.  Using .ok_or() I was able to
// treat the chain of Option<_>'s as Result<_, String>'s

use serde_json::json;
use serde_json::Value;
pub type Applications = Vec<ApplicationData>;

#[derive(Default)]
pub struct ApplicationData {
    pub metadata: Option<Value>,
}

fn gen_applications(version: u64) -> Applications {
    let app = ApplicationData {
        metadata: Some(json!({ "MANTAV": version })),
    };

    vec![app]
}

fn gen_missing_mantav() -> Applications {
    let app = ApplicationData {
        metadata: Some(json!({})),
    };

    vec![app]
}

fn gen_missing_metadata() -> Applications {
    let app = ApplicationData { metadata: None };

    vec![app]
}

fn gen_empty_vec() -> Applications {
    vec![]
}

// I wanted to go through a chain of Option<_>'s and for each one assign a unique
// error message if the Option was None.  Using .ok_or() I was able to treat the
// chain of Option<_>'s as Result<_, String>'s
fn validate_version(applications: Applications) {
    let version_validation = applications
        .first()
        .ok_or("Missing manta application".to_string())
        .and_then(|application| {
            application
                .metadata
                .as_ref()
                .ok_or("Missing application metadata".to_string())
        })
        .and_then(|metadata| {
            metadata
                .get("MANTAV")
                .ok_or("Missing MantaV metadata entry".to_string())
        })
        .and_then(|version| {
            version
                .as_u64()
                .ok_or("Version is not a number".to_string())
        })
        .and_then(|ver| {
            if ver < 2 {
                let msg = format!(
                    "Rebalancer requires manta version 2 or \
                     greater.  Found version {}",
                    ver
                );
                Err(msg)
            } else {
                Ok(())
            }
        });

    match version_validation {
        Ok(_) => {
            println!("Valid version");
        }
        Err(e) => {
            println!("Invalid version: {}", e);
        }
    }
}

fn main() {
    let applications = gen_applications(1);
    validate_version(applications);

    let applications = gen_missing_mantav();
    validate_version(applications);

    let applications = gen_missing_metadata();
    validate_version(applications);

    let applications = gen_empty_vec();
    validate_version(applications);

    let applications = gen_applications(2);
    validate_version(applications);
}
