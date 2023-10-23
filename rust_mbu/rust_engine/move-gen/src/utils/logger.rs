use flexi_logger::{WriteMode, DeferredNow};
use log::Record;

pub fn configure_logger() {
    flexi_logger::Logger::try_with_env_or_str("trace")
        .unwrap()
        .write_mode(WriteMode::SupportCapture)
        .format(log_format)
        .set_palette("203;215;193;113;213".to_string())
        .start()
        .unwrap();
}

pub fn log_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    let line = record.line().unwrap_or(0);
    let path = record.module_path().unwrap_or("<unnamed>");
    let log = record.args();
    let style = flexi_logger::style(level);

    let msg = if level == log::Level::Info {
        format!("[{level:5}] {log}") 
    } else {
        format!("[{level:5}] [{path}|{line}]: {log}") 
    };

    write!(w, "{}", style.paint(&msg))
}
