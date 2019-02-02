pub trait Cache {
    // Store an entry and return an ID
    fn store(&mut self, url: &str) -> String;
    // Store a named entry
    fn store_named(&mut self, url: &str, name: &str) -> String;
    // Look up a previously stored entry
    fn lookup(&self, id: &str) -> Option<String>;
}
