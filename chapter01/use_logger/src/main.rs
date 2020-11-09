use log::{debug, error, info, warn};

fn main() {
    env_logger::init();
    error!("error message");
    warn!("warning message");
    info!("information message");
    debug!("debugging message");
}
