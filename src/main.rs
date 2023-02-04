//                  %%%%%%%@%%%%%%
//               %%%%%%%%%%%@%%%%%%%%%%
//            %%%%%%%%%%%%%%@%%%%%%%%%%%
//           %%%%%%%%%             %%%%
//          %%%%%%%         %                           @@@@          @@@@@@@@@      @@@@@@@@@@
//         %%%%%%%         %%%                         @@@@@@       @@@@     @@@@   @@@     @@@@
//         %%%%%%         %%%%%                       @@@  @@@     @@@@             @@@@
//        @@@@@@@       %%%%%%%%%                    @@@    @@@    @@@@               @@@@@@@@
//         %%%%%%       %%%%%%%%%                   @@@@@@@@@@@@   @@@@                     @@@@
//         %%%%%%%         %%%                     @@@@      @@@@   @@@@     @@@@  @@@@     @@@@
//          %%%%%%%                               @@@@        @@@@    @@@@@@@@@      @@@@@@@@@@
//           %%%%%%%%%             %%%%%
//             %%%%%%%%%%%%%@%%%%%%%%%%%%
//               %%%%%%%%%%%@%%%%%%%%%%%
//                    %%%%%%@%%%%%%
//
//
// Automatic CPU frequency scaler and power saver
//
// https://github.com/jakeroggenbuck/auto-clock-speed
// https://autoclockspeed.org
// https://crates.io/crates/autoclockspeed
// https://github.com/autoclockspeed

#![allow(clippy::uninlined_format_args)]

use args::parse_args;
use config::get_config;
use error::Error;
use log::debug;

pub mod args;
pub mod config;
pub mod cpu;
pub mod csv;
pub mod daemon;
pub mod display;
pub mod error;
pub mod gov;
pub mod graph;
pub mod interactive;
pub mod interface;
pub mod logger;
<<<<<<< HEAD
pub mod network;
=======
pub mod msr;
>>>>>>> ef4c35dc9cff0abe0ecdee2e2cf30a5c23e2e422
pub mod power;
pub mod settings;
pub mod setup;
pub mod sysfs;
pub mod system;
pub mod terminal;
pub mod thermal;

fn main() {
    env_logger::init();

    setup::setup();

    let config: config::Config = get_config();

    parse_args(config);
}
