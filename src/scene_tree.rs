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
        self.gd.0.share()
    }

    // pub fn get_current_scene(&mut self) -> TRef<Node> {
    //     unsafe { self.get().current_scene().unwrap().assume_safe() }
    // }

    // pub fn get_root(&mut self) -> TRef<Viewport> {
    //     unsafe { self.get().root().unwrap().assume_safe() }
    // }

    // pub fn add_to_scene<T: SubClass<Node>>(&mut self, node: TRef<T>) {
    //     self.get_current_scene().add_child(node.upcast(), true);
    // }

    // pub fn add_to_root<T: SubClass<Node>>(&mut self, node: TRef<T>) {
    //     self.get_root().add_child(node.upcast(), true);
    // }
}

#[doc(hidden)]
#[derive(Debug)]
pub(crate) struct SceneTreeRefImpl(Gd<SceneTree>);

impl SceneTreeRefImpl {
    fn get_ref() -> Gd<SceneTree> {
        let engine = Engine::singleton();
        engine
            .get_main_loop()
            .and_then(|lp| Some(lp.try_cast::<SceneTree>()?))
            .unwrap()
    }
}

impl Default for SceneTreeRefImpl {
    fn default() -> Self {
        Self(Self::get_ref())
    }
}
