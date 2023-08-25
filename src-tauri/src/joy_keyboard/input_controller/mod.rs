use std::time::Duration;
use self::enigo_wrapper::EnigoTrait;
use super::keys_config::KeyClickConfig;

pub mod enigo_wrapper;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg(test)]
mod tests;

const DELAY_DURATION: Duration = Duration::from_millis(500);

#[cfg_attr(test, automock)]
pub trait InputControllerTrait {
    fn key_down(&mut self, key_to_click: KeyClickConfig);
    fn key_up(&mut self);
    fn trigger_input(&mut self);
}

pub struct InputController {
    enigo: Box<dyn EnigoTrait>,
    active_key: Option<KeyClickConfig>,
    instant_to_start_delay_from: Option<std::time::Instant>,
}

impl InputController {
    pub fn new(
        enigo: Box<dyn EnigoTrait>,
    ) -> Self {
        Self{
            enigo,
            active_key: None,
            instant_to_start_delay_from: None,
        }
    }

    fn allow_input(&self) -> bool {
        if let Some(instant) = self.instant_to_start_delay_from {
            if instant.elapsed() <= DELAY_DURATION {
                return false;
            }
        }
        true
    }

}

impl InputControllerTrait for InputController {
    // The key is immediately triggered and then after DELAY_DURATION
    // it will be allowed to be triggered many times by calling trigger_input()
    // until key_up() is called (or key_down() is called again).
    fn key_down(&mut self, key_to_click: KeyClickConfig) {
        self.active_key = Some(key_to_click);
        self.instant_to_start_delay_from = None;
        self.trigger_input();
        self.instant_to_start_delay_from = Some(std::time::Instant::now());
    }

    fn key_up(&mut self) {
        self.active_key = None;
        self.instant_to_start_delay_from = None;
    }

    fn trigger_input(&mut self) {
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
            self.instant_to_start_delay_from = None;
        }
    }
}
