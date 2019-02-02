use harsh::{Harsh, HarshBuilder};

pub struct Shortener {
    generator: Harsh,
}

impl Shortener {
    pub fn new() -> Shortener {
        let harsh = HarshBuilder::new()
            .alphabet("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
            .init()
            .unwrap();
        Shortener { generator: harsh }
    }

    pub fn get_short(&mut self, id: u64) -> String {
        let hashed = self.generator.encode(&[id]).unwrap();
        hashed
    }
}
