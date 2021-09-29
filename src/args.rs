use anyhow::{Context, Result};
use clap::{App, Arg};

pub(crate) struct Args {
    inner: clap::ArgMatches<'static>,
}

impl Args {
    pub fn parse() -> Result<Self> {
        let inner = App::new("temper-usb")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Queries a TEMPer USB device to get temperatures.\n\
                    If no arguments are given, prints the temperature to stdout.")
            .arg(
                Arg::with_name("http")
                    .long("http")
                    .short("h")
                    .help("Serve temperatures via HTTP"),
            )
            .arg(
                Arg::with_name("http-port")
                    .long("http-port")
                    .short("p")
                    .takes_value(true)
                    .value_name("PORT")
                    //                    .default_value("80") //Default values are fixed in clap-3
                    .requires("http")
                    .help("HTTP port to listen on (default 80)"),
            )
            .arg(
                Arg::with_name("mqtt")
                    .long("mqtt")
                    .short("m")
                    .requires("mqtt-server")
                    .requires("mqtt-topic")
                    .help("Publish readings at regular intervals over MQTT"),
            )
            .arg(
                Arg::with_name("mqtt-server")
                    .long("mqtt-server")
                    .short("s")
                    .takes_value(true)
                    .value_name("SERVER")
                    .requires("mqtt")
                    .help(
                        "IP address or host name, and port number of the MQTT\n\
                        server, for example localhost:1833",
                    ),
            )
            .arg(
                Arg::with_name("mqtt-topic")
                    .long("mqtt-topic")
                    .short("t")
                    .takes_value(true)
                    .value_name("TOPIC")
                    .requires("mqtt")
                    .help("MQTT topic onto which to publish readings"),
            )
            .arg(
                Arg::with_name("mqtt-frequency")
                    .long("mqtt-frequency")
                    .short("f")
                    .takes_value(true)
                    .value_name("FREQUENCY")
                    //                    .default_value("30") //Default values are fixed in clap-3
                    .requires("mqtt")
                    .help("Frequency of MQTT readings to send (in seconds, default 30)"),
            )
            .get_matches();
        Ok(Self { inner })
    }

    pub fn http(&self) -> bool {
        self.inner.is_present("http")
    }

    pub fn http_port(&self) -> usize {
        self.inner
            .value_of("http-port")
            .unwrap()
            .parse()
            .context("Invalid HTTP port")
            .unwrap()
    }

    pub fn mqtt(&self) -> bool {
        self.inner.is_present("mqtt")
    }

    pub fn mqtt_server(&self) -> &str {
        self.inner.value_of("mqtt-server").unwrap()
    }

    pub fn mqtt_topic(&self) -> &str {
        self.inner.value_of("mqtt-topic").unwrap()
    }

    pub fn mqtt_frequency(&self) -> usize {
        self.inner
            .value_of("mqtt-frequency")
            .unwrap()
            .parse()
            .context("Invalid MQTT frequency")
            .unwrap()
    }
}
