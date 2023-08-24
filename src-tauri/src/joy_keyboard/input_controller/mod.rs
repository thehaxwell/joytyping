use std::time::Duration;

use self::enigo_wrapper::EnigoTrait;

use super::keys_config::KeyClickConfig;

pub mod enigo_wrapper;

const DELAY_DURATION: Duration = Duration::from_millis(500);
pub struct InputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<KeyClickConfig>,
    delay_from_this_instant: Option<std::time::Instant>,
}

impl InputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
            delay_from_this_instant: None,
        }
    }

    pub fn key_down(&mut self, key_to_click: KeyClickConfig) {
        self.active_key = Some(key_to_click);
        self.delay_from_this_instant = None;
        self.trigger_input();
        self.delay_from_this_instant = Some(std::time::Instant::now());
    }

    pub fn key_up(&mut self) {
        self.active_key = None;
        self.delay_from_this_instant = None;
    }

    fn allow_input(&self) -> bool {
        if let Some(instant) = self.delay_from_this_instant {
            if instant.elapsed() <= DELAY_DURATION {
                return false;
            }
        }
        true
    }

    pub fn trigger_input(&mut self) {
        if let Some(key_to_click) = &self.active_key {
            if !self.allow_input() {
                return;
            }
            for key_to_click_modifier in key_to_click.modifiers {
                if let Some(modifier) = key_to_click_modifier {
                    println!("-> {:?}",modifier);
                    self.enigo.key_down(modifier);
                }
            }

            if let Some(key) = key_to_click.key {
                println!("-> {:?}",key);
                self.enigo.key_click(key);
            }

            for key_to_click_modifier in key_to_click.modifiers {
                if let Some(modifier) = key_to_click_modifier {
                    println!("-> {:?}",modifier);
                    self.enigo.key_up(modifier);
                }
            }
            self.delay_from_this_instant = None;
        }
    }
}
