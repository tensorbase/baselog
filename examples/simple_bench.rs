use log::*;
use baselog::*;

use std::fs::File;

fn main() {
    CombinedLogger::init(vec![
        TestLogger::new(LevelFilter::Info, Config::default()),
    ])
    .unwrap();
    
    info!("This only appears in the log file");
}
