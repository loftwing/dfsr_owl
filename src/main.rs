use std::ffi::OsString;
use windows_service::service_dispatcher;
use windows_service::{define_windows_service};

define_windows_service!(ffi_service_main, dfsr_owl_main);

fn dfsr_owl_main(arguments: Vec<OsString>) {
    // entry point
}

fn main() -> Result<(), windows_service::Error> {
    //register generated fn with system and start the service, blocks until stopped

    service_dispatcher::start("DFSROwl", ffi_service_main)?;
    Ok(())
}
