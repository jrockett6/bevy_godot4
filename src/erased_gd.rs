use bevy::prelude::Component;
use godot::{
    classes::{Node, Object, Resource},
    obj::{Bounds, Gd, GodotClass, Inherits, InstanceId, RawGd, bounds::DynMemory},
    sys,
};

#[derive(Debug, Component, Clone)]
pub struct ErasedGd {
    instance_id: InstanceId,
}

impl ErasedGd {
    pub fn get<T: Inherits<Node>>(&mut self) -> Gd<T> {
        self.try_get()
            .unwrap_or_else(|| panic!("failed to get godot ref as {}", std::any::type_name::<T>()))
    }

    /// # SAFETY
    /// The caller must uphold the contract of the constructors to ensure exclusive access
    pub fn try_get<T: Inherits<Node>>(&mut self) -> Option<Gd<T>> {
        Gd::try_from_instance_id(self.instance_id).ok()
    }

    /// # SAFETY
    /// When using ErasedGodotRef as a Bevy Resource or Component, do not create duplicate references
    /// to the same instance because Godot is not completely thread-safe.
    ///
    /// TODO
    /// Could these type bounds be more flexible to accomodate other types that are not ref-counted
    /// but don't inherit Node
    pub fn new<T: Inherits<Node>>(reference: Gd<T>) -> Self {
        Self {
            instance_id: reference.instance_id(),
        }
    }
}

#[derive(Debug, bevy::prelude::Resource)]
pub struct ErasedGdResource {
    resource_id: InstanceId,
}

/// used to access raw (RawGd) object
struct Gd_<T: GodotClass> {
    raw: RawGd<T>,
}

fn maybe_inc_ref<T: GodotClass>(gd: &mut Gd<T>) {
    let gd_: &mut Gd_<T> = unsafe { std::mem::transmute(gd) };
    <Object as Bounds>::DynMemory::maybe_inc_ref(&mut gd_.raw);
}

fn maybe_inc_ref_opt<T: GodotClass>(gd: &mut Option<Gd<T>>) {
    if let Some(gd) = gd {
        let gd_: &mut Gd_<T> = unsafe { std::mem::transmute(gd) };
        <Object as Bounds>::DynMemory::maybe_inc_ref(&mut gd_.raw);
    }
}

fn maybe_dec_ref<T: GodotClass>(gd: &mut Gd<T>) -> bool {
    let gd_: &mut Gd_<T> = unsafe { std::mem::transmute(gd) };
    unsafe { <Object as Bounds>::DynMemory::maybe_dec_ref(&mut gd_.raw) }
}

impl ErasedGdResource {
    pub fn get(&mut self) -> Gd<Resource> {
        self.try_get().unwrap()
    }

    pub fn try_get(&mut self) -> Option<Gd<Resource>> {
        Gd::try_from_instance_id(self.resource_id).ok()
    }

    pub fn new(mut reference: Gd<Resource>) -> Self {
        maybe_inc_ref(&mut reference);

        Self {
            resource_id: reference.instance_id(),
        }
    }
}

impl Clone for ErasedGdResource {
    fn clone(&self) -> Self {
        maybe_inc_ref_opt::<Resource>(&mut Gd::try_from_instance_id(self.resource_id).ok());

        Self {
            resource_id: self.resource_id,
        }
    }
}

impl Drop for ErasedGdResource {
    fn drop(&mut self) {
        let mut gd = self.get();
        let is_last = maybe_dec_ref(&mut gd); // may drop
        if is_last {
            unsafe {
                sys::interface_fn!(object_destroy)(gd.obj_sys());
            }
        }
    }
}
