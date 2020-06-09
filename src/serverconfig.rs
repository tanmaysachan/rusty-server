use clap::{App, load_yaml};

pub struct Config {
    pub url: String,
    pub port: i32,
}

impl Config {
    pub fn get_config() -> Config {
        let mut port = 8080;
        if let Some(port_arg) = App::from(load_yaml!("cli.yml"))
                                          .get_matches()
                                          .value_of("port") {

            let port_arg = port_arg.parse::<i32>();

            if let Err(e) = port_arg {
                eprintln!("Error: {}", e);
                println!("Unable to assign port, falling back to default");
            } else {
                port = port_arg.unwrap();
            }
        }

        Config {
            url: String::from("localhost:") + &port.to_string()[..],
            port: port,
        }
    }
}
