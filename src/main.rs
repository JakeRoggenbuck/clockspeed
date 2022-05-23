use log::debug;
use structopt::StructOpt;

use config::{config_dir_exists, get_config};
use daemon::{daemon_init, Checker};
use display::{
    print_available_governors, print_cpu_governors, print_cpu_speeds, print_cpu_temp, print_cpus,
    print_freq, print_power, print_turbo, show_config,
};
use error::Error;
use power::{read_battery_charge, read_lid_state, read_power_source};
use settings::Settings;
use system::{
    check_available_governors, check_cpu_freq, check_cpu_name, check_turbo_enabled,
    get_cpu_percent, list_cpu_governors, list_cpu_speeds, list_cpu_temp, list_cpus,
};

pub mod config;
pub mod cpu;
pub mod daemon;
pub mod display;
pub mod error;
pub mod graph;
pub mod logger;
pub mod power;
pub mod settings;
pub mod state;
pub mod system;
pub mod terminal;

#[derive(StructOpt)]
enum GetType {
    /// Get the power
    #[structopt(name = "power")]
    Power {
        #[structopt(short, long)]
        raw: bool,
    },

    /// Get the power
    #[structopt(name = "usage")]
    Usage {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The overall frequency of your cpu
    #[structopt(name = "freq")]
    Freq {
        #[structopt(short, long)]
        raw: bool,
    },

    /// Get whether turbo is enabled or not
    #[structopt(name = "turbo")]
    Turbo {
        #[structopt(short, long)]
        raw: bool,
    },

    /// Get the available governor
    #[structopt(name = "available-govs")]
    AvailableGovs {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The names of the core
    #[structopt(name = "cpus")]
    CPUS {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The speed of the individual cores
    #[structopt(name = "speeds")]
    Speeds {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The temperature of the individual cores
    #[structopt(name = "temp")]
    Temp {
        #[structopt(short, long)]
        raw: bool,
    },

    /// The governors of the individual cores
    #[structopt(name = "govs")]
    Govs {
        #[structopt(short, long)]
        raw: bool,
    },
}

#[derive(StructOpt)]
enum SetType {
    #[structopt(name = "gov")]
    Gov {
        #[structopt()]
        value: String,
    },
}

#[derive(StructOpt)]
#[structopt(
    name = "autoclockspeed",
    about = "Automatic CPU frequency scaler and power saver"
)]
enum ACSCommand {
    /// Get a specific value or status
    #[structopt(name = "get")]
    Get {
        /// The type of value to request
        #[structopt(subcommand)]
        get: GetType,
    },

    /// Set a specific value
    #[structopt(name = "set")]
    Set {
        #[structopt(subcommand)]
        set: SetType,
    },

    /// Show the current config in use
    #[structopt(name = "showconfig", alias = "conf")]
    ShowConfig {},

    /// Run the daemon, this checks and edit your cpu's speed
    #[structopt(name = "run")]
    Run {
        /// Show the information the monitor sub-command outputs
        #[structopt(short, long)]
        quiet: bool,

        /// Milliseconds between update
        #[structopt(short, long, default_value = "1000")]
        delay: u64,

        /// Milliseconds between update
        #[structopt(short = "b", long = "delay-battery", default_value = "5000")]
        delay_battery: u64,

        /// No animations, for systemctl updating issue
        #[structopt(short, long)]
        no_animation: bool,

        /// Graph
        #[structopt(short = "g", long = "--graph")]
        should_graph: bool,

        /// Commit hash
        #[structopt(short, long)]
        commit: bool,
    },

    /// Monitor each cpu, it's min, max, and current speed, along with the governor
    #[structopt(name = "monitor", alias = "monit")]
    Monitor {
        /// Milliseconds between update when on AC
        #[structopt(short, long, default_value = "1000")]
        delay: u64,

        /// Milliseconds between update
        #[structopt(short = "b", long = "delay-battery", default_value = "5000")]
        delay_battery: u64,

        /// No animations, for systemctl updating issue
        #[structopt(short, long)]
        no_animation: bool,

        /// Graph
        #[structopt(short = "g", long = "--graph")]
        should_graph: bool,

        /// Commit hash
        #[structopt(short, long)]
        commit: bool,
    },
}

fn parse_args(config: config::Config) {
    let mut daemon: daemon::Daemon;

    // default settings used by set command
    let set_settings = Settings {
        verbose: true,
        delay_battery: 0,
        delay: 0,
        edit: false,
        no_animation: false,
        should_graph: false,
        commit: false,
        testing: false,
    };

    match ACSCommand::from_args() {
        // Everything starting with "get"
        ACSCommand::Get { get } => match get {
            GetType::Freq { raw } => {
                let f = check_cpu_freq();
                print_freq(f, raw);
            }

            GetType::Power { raw } => match read_lid_state() {
                Ok(lid) => match read_battery_charge() {
                    Ok(bat) => match read_power_source() {
                        Ok(plugged) => {
                            print_power(lid, bat, plugged, raw);
                        }
                        Err(_) => eprintln!("Failed to get read power source"),
                    },
                    Err(_) => eprintln!("Failed to get read battery charger"),
                },
                Err(_) => eprintln!("Failed to get read lid state"),
            },

            GetType::Usage { raw } => match get_cpu_percent() {
                Ok(content) => {
                    println!("{}", content)
                }
                Err(_) => println!("Unable to usage status"),
            },

            GetType::Turbo { raw } => match check_turbo_enabled() {
                Ok(turbo_enabled) => print_turbo(turbo_enabled, raw),
                Err(_) => println!("Failed to get turbo status"),
            },

            GetType::AvailableGovs { raw } => match check_available_governors() {
                Ok(available_governors) => print_available_governors(available_governors, raw),
                Err(_) => println!("Failed to get available governors"),
            },

            GetType::CPUS { raw } => {
                let cpus = list_cpus();
                match check_cpu_name() {
                    Ok(name) => print_cpus(cpus, name, raw),
                    Err(_) => println!("Failed get list of cpus"),
                };
            }

            GetType::Speeds { raw } => {
                let speeds = list_cpu_speeds();
                print_cpu_speeds(speeds, raw);
            }

            GetType::Temp { raw } => {
                let cpu_temp = list_cpu_temp();
                print_cpu_temp(cpu_temp, raw);
            }

            GetType::Govs { raw } => {
                let govs = list_cpu_governors();
                print_cpu_governors(govs, raw);
            }
        },

        // Everything starting with "set"
        ACSCommand::Set { set } => match set {
            SetType::Gov { value } => match daemon_init(set_settings, config) {
                Ok(mut d) => match d.set_govs(value.clone()) {
                    Ok(_) => {}
                    Err(e) => eprint!("Could not set gov, {:?}", e),
                },
                Err(_) => eprint!("Could not run daemon in edit mode"),
            },
        },

        ACSCommand::ShowConfig {} => show_config(),

        // Run command
        ACSCommand::Run {
            quiet,
            delay,
            delay_battery,
            no_animation,
            should_graph,
            commit,
        } => {
            if !config_dir_exists() {
                warn_user!("Config directory '/etc/acs' does not exist!");
            }

            let mut effective_delay_battery = delay_battery;
            if should_graph || delay != 1000 {
                effective_delay_battery = delay;
            }

            let settings = Settings {
                verbose: !quiet,
                delay_battery: effective_delay_battery,
                delay,
                edit: true,
                no_animation,
                should_graph,
                commit,
                testing: false,
            };

            match daemon_init(settings, config) {
                Ok(d) => {
                    daemon = d;
                    daemon.run().unwrap_err();
                }
                Err(_) => eprint!("Could not run daemon in edit mode"),
            }
        }

        // Monitor command
        ACSCommand::Monitor {
            delay,
            delay_battery,
            no_animation,
            should_graph,
            commit,
        } => {
            if !config_dir_exists() {
                warn_user!("Config directory '/etc/acs' does not exist!");
            }

            let mut effective_delay_battery = delay_battery;
            if should_graph || delay != 1000 {
                effective_delay_battery = delay;
            }

            let settings = Settings {
                verbose: true,
                delay,
                delay_battery: effective_delay_battery,
                edit: false,
                no_animation,
                should_graph,
                commit,
                testing: false,
            };

            match daemon_init(settings, config) {
                Ok(d) => {
                    daemon = d;
                    daemon.run().unwrap_err();
                }
                Err(_) => eprint!("Could not run daemon in monitor mode"),
            }
        }
    }
}

fn main() {
    env_logger::init();

    let config: config::Config = get_config();

    parse_args(config);
}
