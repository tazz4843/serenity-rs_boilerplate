use std::env;
use tracing::{error, info, warn};

/// This function simply changes the current working directory to the directory where the executable is located.
pub fn set_dir() {
    match env::current_exe() {
        Ok(path) => match path.parent() {
            Some(parent) => {
                if let Err(err) = env::set_current_dir(parent) {
                    warn!("couldn't chdir: {}", err);
                }
            }
            None => error!("couldn't get current executable path"),
        },
        Err(err) => error!("couldn't get current executable location: {}", err),
    }
    match env::current_dir() {
        Ok(dir) => info!("working directory is {}", dir.display()),
        Err(err) => warn!("couldn't get current dir: {}", err),
    }
}
