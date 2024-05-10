use crate::prelude::*;
use bevy::ecs::system::SystemParam;
use godot::engine::Engine;
use std::marker::PhantomData;

#[derive(SystemParam)]
pub struct SceneTreeRef<'w, 's> {
    gd: NonSendMut<'w, SceneTreeRefImpl>,
    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> SceneTreeRef<'w, 's> {
    pub fn get(&mut self) -> Gd<SceneTree> {
        self.gd.0.clone()
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct SceneTreeRefImpl(Gd<SceneTree>);

impl SceneTreeRefImpl {
    fn get_ref() -> Gd<SceneTree> {
        Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>()
    }
}

impl Default for SceneTreeRefImpl {
    fn default() -> Self {
        Self(Self::get_ref())
    }
}
