use notify::{PollWatcher, Config, Error, Event, Watcher, RecursiveMode};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use anyhow::Result;
use config;


pub struct ConfiguredWatcher {
    rx: Receiver<Result<Event, Error>>,
    watcher: PollWatcher,
}

impl ConfiguredWatcher {
    pub fn new(config: Config) -> Result<Self> {
        let (tx, rx) = channel();

        let mut watcher = PollWatcher::new(tx, config)?;
        
        return Ok(ConfiguredWatcher {
            rx,
            watcher
        })
    }

    pub fn watch(&mut self, path: PathBuf, mode: RecursiveMode) -> Result<()> {
        self.watcher.watch(&path, mode)?;
        Ok(())
    }
}

pub struct Watchers {
    watchers: Vec<ConfiguredWatcher>,
    service_config: config::Config,
}

impl Watchers {
    pub fn new(service_config: config::Config) -> Self {
        return Watchers {
            watchers: Vec::new(),
            service_config
        }
    }

    pub fn register_watcher(&mut self, path: PathBuf) -> Result<()> {
        let config = Config::default()
            .with_poll_interval(Duration::from_secs(3)) // Manual polling is required for net mounted drives
            .with_compare_contents(false); // This can have a big performance impact!
        let mut new_watcher: ConfiguredWatcher = ConfiguredWatcher::new(config)?;
        let mode = if self.service_config.get_bool("working.recursive")? {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        new_watcher.watch(path, mode)?;
        self.watchers.push(new_watcher);
        Ok(())
    }
}