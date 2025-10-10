use std::{fs, path::PathBuf, sync::OnceLock};

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

use crate::infrastructure::error::BasicResult;

pub(super) static LOG_GUARD: OnceLock<non_blocking::WorkerGuard> = OnceLock::new();

pub(super) fn init_log(log_dir: PathBuf) {
    if !log_dir.exists() {
        std::fs::create_dir_all(&log_dir).expect("Failed to create log dir");
    }

    let file_appender = rolling::daily(&log_dir, "app.log");
    let (non_blocking_file, guard) = non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_timer(fmt::time::ChronoUtc::rfc_3339())
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true);

    tracing_subscriber::registry().with(file_layer).init();

    let _ = LOG_GUARD.set(guard);

    tracing::info!("init log success");

    if let Err(err) = cleanup_old_logs(log_dir) {
        tracing::error!("Failed to cleanup old logs: {:?}", err);
    }
}

fn cleanup_old_logs<'a>(log_dir: PathBuf) -> BasicResult<'a, ()> {
    let cutoff_time = Utc::now() - ChronoDuration::days(7);

    for entry in fs::read_dir(&log_dir)? {
        let path = entry?.path();

        if !path.is_file() {
            continue;
        }

        if !path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n.starts_with("app.log"))
        {
            continue;
        }

        if fs::metadata(&path)?
            .created()
            .map(|t| <std::time::SystemTime as Into<DateTime<Utc>>>::into(t) > cutoff_time)?
        {
            continue;
        }

        fs::remove_file(&path)?;
    }

    Ok(())
}
