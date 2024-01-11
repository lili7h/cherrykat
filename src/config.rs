use config::{Config, ConfigError, File, FileFormat};
use std::io::ErrorKind;
use std::path::Path;
use std::fs;


const LOCAL_LOC: &str = "config/cherrykat.toml";
const ETC_LOC: &str = "/etc/cherrykat/config.toml";


pub fn init_config() -> Result<Config, ConfigError> {
    if !Path::new(ETC_LOC).try_exists().expect("Could not verify existance of /etc/ log file") {
        if let Err(err) = fs::create_dir("/etc/cherrykat/") {
            if err.kind() != ErrorKind::AlreadyExists {
                panic!("{}", err);
            }
        }

        if let Err(err) = fs::File::create(ETC_LOC) {
            if err.kind() == ErrorKind::PermissionDenied {
                panic!("Could not create /etc/ log file, permission denied.");
            }
        } else {
            if let Err(err) = fs::copy(LOCAL_LOC, ETC_LOC) {
                panic!("{}", err);
            }
        }
    }

    Config::builder()
        .set_default("working.working_dir", "/tmp/cherrykat/working/")?
        .set_default("working.output_dir", "/tmp/cherrykat/compressed/")?
        .set_default("watching.size_limit", 1000000000 as i64)?
        .set_default("watching.age_limit", 3600 as i64)?
        .set_default("watching.stale_limit", 1200 as i64)?
        .set_default("compression.compression_level", 9 as i64)?
        .set_default("hashing.algorithm", "sha256")?
        .set_default("working.watching_dirs", vec!["/tmp/ingest"])?
        .set_default("working.recursive", true)?
        .set_default("logging.output", "/var/log/cherrykat.log")?
        .add_source(File::new(LOCAL_LOC, FileFormat::Toml))
        .add_source(File::new(ETC_LOC, FileFormat::Toml))
        .build()
}

