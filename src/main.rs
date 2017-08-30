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
        data.level() <= log::LogLevelFilter::Warn
    }).chain(std::io::stderr());

    let stdout_dispatcher = fern::Dispatch::new().filter(|data| {
        data.level() > log::LogLevelFilter::Warn
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

#[cfg(test)]
mod tests {
    use log;
    use super::setup_logging;

    #[test]
    fn setup() {
        assert!(match setup_logging(log::LogLevelFilter::Debug, None) {
            Ok(_) => true,
            Err(_) => false
        });
        
        // Only one dispatcher can be registered
        assert!(match setup_logging(log::LogLevelFilter::Debug, None) {
            Ok(_) => false,
            Err(_) => true
        });
    }
}
