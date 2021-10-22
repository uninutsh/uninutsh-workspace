mod internals;

pub struct Window {
    internals: Option<internals::Data>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        let internals = Some(internals::Data::new(title));
        Window { internals }
    }
    pub fn event_loop(mut self) {
        let internals = self.internals.take().unwrap();
        internals.event_loop(self);
    }
}
