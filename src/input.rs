use bevy::prelude::*;
use godot::{engine::InputEvent, prelude::*};
use std::sync::mpsc::Receiver;

pub struct GodotInputPlugin;
impl Plugin for GodotInputPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(forward_input_events)
        //     .init_non_send_resource::<Events<Gd<InputEvent>>>();
    }
}

fn forward_input_events(reciever: NonSend<Receiver<Gd<InputEvent>>>) {
    reciever
        .try_iter()
        .for_each(|x| println!("{:?}", x.get_class()))
}
