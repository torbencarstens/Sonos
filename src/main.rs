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

    let dispatcher = match file {
        Some(path) => {
            dispatcher.chain(fern::log_file(path)?)
        }
        None => dispatcher.chain(std::io::stdout())
    };

    dispatcher.apply()?;

    Ok(())
}
