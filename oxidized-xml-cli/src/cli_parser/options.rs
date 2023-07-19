use std::io::{stdin, StdinLock};

pub struct Opts {
    pub xpath: String,
    pub xml_buffer: String,
    // pub stdin_handle: StdinLock<'a>,
}

impl Opts {
    pub fn new() -> Self {
        Self {
            // stdin_handle: stdin().lock(),
            xpath: String::new(),
            xml_buffer: String::new(),
        }
    }
}
