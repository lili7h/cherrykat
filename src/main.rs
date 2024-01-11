use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

mod config;
mod watching;

fn main() {
    let service_config = match config::init_config() {
        Ok(config) => config,
        Err(err) => panic!("{}", err),
    };
    let log_path = service_config.get_string("logging.output").expect("logging.output not defined/valid").clone();
    let _guard = init_tracing(log_path);
    let watch_paths = service_config.get_array("working.watching_dirs").expect("working.watching_dirs not valid").clone();
    let mut watchers = watching::Watchers::new(service_config);
    for watch_path in watch_paths {
        if let Err(err) = watchers
            .register_watcher(
                watch_path.into_string()
                .expect("watching_dirs values are strigs")
                .into()
            ) {
                tracing::error!("Couldn't initialise watcher on path because: {}", err);
        }
    }

}

fn init_tracing(log_path: String) -> Option<WorkerGuard> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info,hyper::proto=warn");
    }

    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_filter(EnvFilter::from_default_env()),
    );

    match std::fs::File::create(log_path) {
        Ok(latest_log) => {
            let (file_writer, guard) = tracing_appender::non_blocking(latest_log);
            subscriber
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_ansi(false)
                        .with_writer(file_writer.with_max_level(tracing::Level::TRACE)),
                )
                .init();
            Some(guard)
        }
        Err(e) => {
            subscriber.init();
            tracing::error!(
                "Failed to create log file, continuing without persistent logs: {}",
                e
            );
            None
        }
    }
}