use enigo::{Key, Enigo, KeyboardControllable};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait EnigoTrait {
    fn key_click(&mut self, key: Key);
}

pub struct EnigoWrapper {
    enigo: Enigo,
}

impl EnigoWrapper {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }
}

impl EnigoTrait for EnigoWrapper {
    fn key_click(&mut self, key: Key) {
        self.enigo.key_click(key);
    }
}
