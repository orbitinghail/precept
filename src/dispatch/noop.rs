use super::{Dispatch, Event};

pub struct NoopDispatch;

impl NoopDispatch {
    pub fn new_boxed() -> Box<dyn Dispatch> {
        Box::new(Self)
    }
}

impl Dispatch for NoopDispatch {
    fn emit(&self, _event: Event) {}

    fn random(&self) -> u64 {
        rand::random()
    }
}
