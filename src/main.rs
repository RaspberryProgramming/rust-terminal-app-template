use clap::Parser;

use log::{error, info};

use std::fs::File;
use std::io::prelude::*;

use rust_terminal_app_template::logging;
use rust_terminal_app_template::cli;

fn main() {
    // Use cli::Args to parse cli arguments
    let args = cli::Args::parse();

    // Setup Logging
    let _handle = logging::setup_logging();

    info!("Writing file to {}", args.filename);

    let lorem_ipsum: &str = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam non sapien elit. Vivamus quis vulputate velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Proin volutpat dui id sem sollicitudin, ut auctor nunc venenatis. Praesent aliquam nulla metus, quis porttitor nunc posuere id. Aliquam tortor lorem, viverra in velit non, tincidunt blandit metus. Morbi eu facilisis turpis. Phasellus hendrerit placerat imperdiet. Curabitur nec mauris vitae eros rutrum luctus. Suspendisse potenti. Vestibulum a est aliquet, aliquet dui vel, interdum lorem.

Ut nec tristique lacus. Aliquam sagittis elementum lectus. Nulla facilisi. Vivamus eu est eu felis mollis porttitor eu ut magna. Duis lorem nisi, porttitor ut ante quis, sodales hendrerit libero. Pellentesque suscipit nunc eu diam posuere, et tempus magna hendrerit. Interdum et malesuada fames ac ante ipsum primis in faucibus. Aliquam efficitur vitae neque cursus eleifend. Phasellus luctus mauris ultrices luctus dictum. Nulla sit amet ex sapien. Suspendisse commodo augue non lectus condimentum tempus. Phasellus a lacinia tortor, mollis commodo velit. Fusce ex massa, ultrices sodales est quis, commodo finibus nulla. Nam consectetur tortor nec ultricies sodales. Curabitur vitae dignissim felis.

Nam scelerisque eros id libero ultricies, nec sodales ipsum molestie. In hac habitasse platea dictumst. Nunc non arcu vel leo ornare fringilla. Nam turpis lectus, euismod a aliquam sit amet, pellentesque in dolor. Proin justo sapien, interdum eu libero sit amet, dictum ornare orci. Mauris id nisi fringilla, porta metus facilisis, elementum mi. Vestibulum eu urna nisi.

Proin auctor, augue vel tincidunt finibus, arcu est cursus urna, eget sagittis urna leo in arcu. Sed sollicitudin orci ut dapibus condimentum. Praesent vehicula sagittis sem, vel congue purus ornare ut. Donec quis massa ac lectus efficitur tristique vel pellentesque purus. Mauris pharetra porttitor quam, in cursus ipsum commodo efficitur. Nam nec imperdiet odio, ac viverra elit. Ut vel tellus eget odio euismod placerat et et sem. In hac habitasse platea dictumst. Vestibulum pulvinar fringilla quam non molestie. Donec eleifend odio sit amet metus sollicitudin varius. Fusce ut est sit amet eros porttitor auctor. Proin ac nulla ac orci ornare facilisis non nec turpis. Donec felis ex, scelerisque sit amet libero ut, convallis euismod ligula.

Vestibulum tristique turpis vitae lacus rutrum dictum. Duis magna nibh, facilisis at porttitor nec, euismod eu quam. Vestibulum luctus maximus varius. Phasellus sagittis sollicitudin sodales. Suspendisse potenti. Integer congue turpis at erat sodales dictum. Pellentesque finibus justo ac placerat egestas. Vestibulum bibendum justo turpis, vitae euismod est volutpat euismod. Fusce vitae diam id dui auctor facilisis. ";

    // Write lorem_ipsum to argument file
    let mut file = File::create(args.filename.clone()).unwrap();
    let result = file.write_all(lorem_ipsum.as_bytes());    

    if result.is_ok() {
        info!("Wrote Lorem Ipsum to {} Successfully.", args.filename);
    } else {
        error!("Failed to write Lorem Ipsum to {}.", args.filename);
    }

    
}