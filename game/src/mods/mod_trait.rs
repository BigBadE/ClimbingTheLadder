use std::sync::Arc;
use crate::util::register::ThingRegister;

pub trait ModMain {
    fn finish_loading(&mut self);

    fn handle_event(&mut self, event: Arc<dyn ThingRegister>);
}