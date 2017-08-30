#[macro_use]
extern crate log;
extern crate fern;

fn main() {
    let _ = setup_logging(log::LogLevelFilter::Debug, None);
    debug!("Hey");
    info!("Hey");
    warn!("Hey");
    error!("Hey");
}

fn setup_logging(level: log::LogLevelFilter, file: Option<std::path::PathBuf>) -> Result<(), fern::InitError> {
    let dispatcher = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}]\t[{}: {}]\t{}",
            record.level(),
            record.target(),
            record.location().file(),
            message
        ))
    }).level(level);


    let stderr_dispatcher = fern::Dispatch::new().filter(|data| {
        match data.level().cmp(&log::LogLevelFilter::Warn.to_log_level().unwrap()) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => true,
            _ => false
        }
    }).chain(std::io::stderr());

    let stdout_dispatcher = fern::Dispatch::new().filter(|data| {
        match data.level().cmp(&log::LogLevelFilter::Warn.to_log_level().unwrap()) {
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => false,
            _ => true
        }
    });


    let stdout_dispatcher = match file {
        Some(path) => {
            stdout_dispatcher.chain(fern::log_file(path)?)
        }
        None => stdout_dispatcher.chain(std::io::stdout())
    };

    let dispatcher = dispatcher.chain(stderr_dispatcher).chain(stdout_dispatcher);
    dispatcher.apply()?;

    Ok(())
}
