use crate::{models::layout::{CardinalLevers, SingleCardinalLever}, gamepad::InputEvent};

use super::CardinalLeversMoveDetectorTrait;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg(test)]
mod tests;

#[cfg_attr(test, automock)]
pub trait MouseTrait {
    fn set_mouse_controls(&mut self,
      cardinal_levers: Option<CardinalLevers>,);
    fn tick(&mut self) -> Option<InputEvent>;
    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32);
}

pub struct Mouse {
   mouse_cursor_move_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
   mouse_scroll_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
}

impl Mouse {
    pub fn new(
       mouse_cursor_move_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
       mouse_scroll_detector: Box<dyn CardinalLeversMoveDetectorTrait>,
    ) -> Self {
        Mouse{
            mouse_cursor_move_detector,
            mouse_scroll_detector,
        }
    }
}

impl MouseTrait for Mouse {
    fn set_mouse_controls(
        &mut self,
        cardinal_levers: Option<CardinalLevers>,
        ){
            if let Some(CardinalLevers { left_stick, right_stick }) 
                = cardinal_levers {
                self.mouse_cursor_move_detector.set_mouse_controls(
                    match left_stick.clone() {
                        Some(SingleCardinalLever::ControlMouseCursor(
                           mouse_control)) 
                        => Some(mouse_control),
                        _ => None,
                    },
                    match right_stick.clone() {
                        Some(SingleCardinalLever::ControlMouseCursor(
                           mouse_control)) 
                        => Some(mouse_control),
                        _ => None,
                    });


                self.mouse_scroll_detector.set_mouse_controls(
                    match left_stick {
                        Some(SingleCardinalLever::ControlMouseScrollwheel(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    },
                    match right_stick {
                        Some(SingleCardinalLever::ControlMouseScrollwheel(
                           mouse_control)) 
                        => Some(mouse_control.clone()),
                        _ => None,
                    });
            }
            else {
                self.mouse_cursor_move_detector.set_mouse_controls(None,None);
                self.mouse_scroll_detector.set_mouse_controls(None,None);
            }
    }

    fn tick(&mut self) -> Option<InputEvent>{
        if let Some((x,y)) = self.mouse_cursor_move_detector.tick() {
            return Some(InputEvent::MoveMouseCursor(x,y))
        }
        if let Some((x,y)) = self.mouse_scroll_detector.tick() {
            return Some(InputEvent::MouseScroll(x,y))
        }

        None
    }

    fn axis_changed(&mut self, axis: gilrs::ev::Axis, value: f32){
        self.mouse_cursor_move_detector.axis_changed(axis,value);
        self.mouse_scroll_detector.axis_changed(axis,value);
    }
}
