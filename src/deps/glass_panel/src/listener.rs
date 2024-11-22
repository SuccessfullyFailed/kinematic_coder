use crate::DrawableData;
use std::rc::Rc;

#[derive(Clone)]
pub enum ListenerType {
	Hover,
	NonHover,
	LeftDown,
	LeftUp,
	RightDown,
	RightUp,
	DragLeft,
	DragRight
}

pub type ListenerCallback = Rc<dyn Fn([isize; 2], [isize; 2], &str, &mut DrawableData)>; // Parameters are the mouse position relative to the element, the absolute position, and the Settings of the element.

#[derive(Clone)]
pub struct Listener {
	trigger_type:ListenerType,
	handler:Box<ListenerCallback>,
	handler_data:Vec<isize>
}
impl Listener {

	/// Create a new listener.
	pub fn new(trigger_type:ListenerType, handler:ListenerCallback) -> Listener {
		Listener {
			trigger_type,
			handler: Box::new(handler),
			handler_data: Vec::new()
		}
	}

	/// Check if the trigger is active. If so, activate the handler.
	pub fn update(&mut self, mouse_position:&[usize; 2], parent_position:&[usize; 4], mouse_down:&[bool; 2], initial:&[bool; 2], listener_name:&str, element_data:&mut DrawableData) {

		// Figure out the state of the trigger.
		let mp:[isize; 2] = [mouse_position[0] as isize, mouse_position[1] as isize];
		let pp:[isize; 4] = [parent_position[0] as isize, parent_position[1] as isize, parent_position[2] as isize, parent_position[3] as isize];
		let rmp:[isize; 2] = [mp[0] - pp[0], mp[1] - pp[1]];
		let hovering:bool = mp[0] >= pp[0] && mp[1] >= pp[1] && mp[0] <= (pp[0] + pp[2]) && mp[1] <= (pp[1] + pp[3]);

		// Handle listeners according to its type.
		match self.trigger_type {
			ListenerType::Hover => {
				if hovering {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::NonHover => {
				if !hovering {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::LeftDown => {
				if hovering &&  mouse_down[0] && initial[0] {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::LeftUp => {
				if hovering && !mouse_down[0] && initial[0] {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::RightDown => {
				if hovering &&  mouse_down[1] && initial[1] {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::RightUp => {
				if hovering && !mouse_down[1] && initial[1] {
					(self.handler)([rmp[0], rmp[1]], mp, listener_name, element_data);
				}
			},
			ListenerType::DragLeft => {
				if hovering && mouse_down[0] && initial[0] {
					self.handler_data = vec![1, mp[0], mp[1]];
				}
				if mouse_down[0] && !initial[0] && !self.handler_data.is_empty() && self.handler_data[0] == 1 {
					(self.handler)([mp[0] - self.handler_data[1], mp[1] - self.handler_data[2]], mp, listener_name, element_data);
					self.handler_data = vec![1, mp[0], mp[1]];
				}
				if !mouse_down[0] && !self.handler_data.is_empty() {
					self.handler_data = Vec::new();
				}
			},
			ListenerType::DragRight => {
				if hovering && mouse_down[1] && initial[1] {
					self.handler_data = vec![1, mp[0], mp[1]];
				}
				if mouse_down[1] && !initial[1] && !self.handler_data.is_empty() && self.handler_data[0] == 1 {
					(self.handler)([mp[0] - self.handler_data[1], mp[1] - self.handler_data[2]], mp, listener_name, element_data);
					self.handler_data = vec![1, mp[0], mp[1]];
				}
				if !mouse_down[1] && !self.handler_data.is_empty() {
					self.handler_data = Vec::new();
				}
			}
		};
	}
}