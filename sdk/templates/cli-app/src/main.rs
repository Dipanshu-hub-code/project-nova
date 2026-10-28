fn main() {
    match nova_std::log::write_log("Hello from a Nova SDK application template") {
        Ok(()) => println!("Message sent to Nova."),
        Err(error) => {
            eprintln!("Nova logging is unavailable in this build: {error:?}");
            std::process::exit(1);
        }
    }
}
