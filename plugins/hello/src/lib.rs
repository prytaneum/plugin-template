use hestia::{Error, Plugin};

struct Entry;

impl Plugin for Entry {
    fn init() -> Result<(), Error> {
        hestia::log::info("hello loaded");
        Ok(())
    }
}

hestia::plugin!(Entry);
