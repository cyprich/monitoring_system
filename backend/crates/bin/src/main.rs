use std::{thread::sleep, time::Duration};

pub fn main() {
    let mut system = match data_collection::System::new() {
        Some(val) => val,
        None => {
            let systems = data_collection::get_supported_systems();
            panic!(
                "System not supported! Only these systems are currently supported: {}",
                systems.join(", ")
            )
        }
    };

    loop {
        dbg!(system.get_data());
        sleep(Duration::from_secs(1));
    }
}
