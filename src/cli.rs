use clap::{arg, command};
use std::{env, fmt, io, time::Duration};

#[derive(Debug, Clone)]
pub struct Arg {
    file: String,
    verbose: bool,
    addr: String,
    polling_duration: Duration,
}

const DEFAULT_POLLING_DURATION: u64 = 300;

impl Arg {
    pub fn new() -> io::Result<Self> {
        let matches = command!()
            .arg(arg!([file] "File to watch.").required(true))
            .arg(
                clap::Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .value_parser(["1", "0"])
                    .default_value("0")
                    .default_missing_value("1")
                    .num_args(0..=1)
                    .required(false)
                    .help("Verbose"),
            )
            .arg(
                clap::Arg::new("port")
                    .short('p')
                    .long("port")
                    .default_value("8080")
                    .help("Port to serve pdf")
                    .required(false),
            )
            .arg(
                clap::Arg::new("duration")
                    .short('d')
                    .long("duration")
                    .default_value("300")
                    .required(false)
                    .help("Polling duration in milliseconds"),
            )
            .get_matches();

        let file = matches.get_one::<String>("file").unwrap().to_owned();
        let verbose = match matches.get_one::<String>("verbose") {
            Some(val) => val.eq("1"),
            None => false,
        };

        let port = if let Some(p) = matches.get_one::<String>("port") {
            p
        } else {
            "8080"
        };

        let dur = if let Some(d) = matches.get_one::<String>("duration") {
            d.parse::<u64>()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))
        } else {
            Ok(DEFAULT_POLLING_DURATION)
        }?;

        return Ok(Arg {
            file,
            verbose,
            addr: format!("0.0.0.0:{}", port),
            polling_duration: Duration::from_millis(dur),
        });
    }

    pub fn write_stdin(&self, s: impl fmt::Display) {
        if self.verbose {
            println!("{}", s);
        }
    }

    pub fn host(&self) -> &str {
        return self.addr.as_str();
    }

    pub fn file(&self) -> &str {
        return self.file.as_str();
    }

    pub fn polling_duration(&self) -> Duration {
        return self.polling_duration;
    }
}
