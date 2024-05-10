use crate::prelude::*;
use godot::{
    engine::Resource, obj::{bounds::DynMemory, Bounds}, sys,
    obj::RawGd
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

#[derive(Debug, Resource)]
pub struct ErasedGdResource {
    resource_id: InstanceId,
}
struct MyGd<T: GodotClass> {
    raw: RawGd<T>,
}

fn maybe_inc_ref<T: GodotClass>(gd: &Gd<T>) {
    let mygd: MyGd<T> = unsafe {
        std::mem::transmute(gd.clone())
    };
    let mut raw = mygd.raw;
    <Object as Bounds>::DynMemory::maybe_inc_ref(&mut raw);
}

fn try_maybe_inc_ref<T: GodotClass>(gd: &Option<Gd<T>>) {
    if let Some(gd) = gd {
        let mygd: MyGd<T> = unsafe {
            std::mem::transmute(gd.clone())
        };
        let mut raw = mygd.raw;
        <Object as Bounds>::DynMemory::maybe_inc_ref(&mut raw);
    }
}

fn maybe_dec_ref<T: GodotClass>(gd: &Gd<T>) -> bool {
    let mygd: MyGd<T> = unsafe {
        std::mem::transmute(gd.clone())
    };
    let mut raw = mygd.raw;
    unsafe {
        <Object as Bounds>::DynMemory::maybe_dec_ref(&mut raw)
    }
}

fn try_maybe_dec_ref<T: GodotClass>(gd: &Option<Gd<T>>) -> bool {
    if let Some(gd) = gd {
        let mygd: MyGd<T> = unsafe {
            std::mem::transmute(gd.clone())
        };
        let mut raw = mygd.raw;
        unsafe {
            <Object as Bounds>::DynMemory::maybe_dec_ref(&mut raw)
        }
    } else {
        false
    }
}

impl ErasedGdResource {
    pub fn get(&mut self) -> Gd<Resource> {
        self.try_get().unwrap()
    }

    pub fn try_get(&mut self) -> Option<Gd<Resource>> {
        Gd::try_from_instance_id(self.resource_id).ok()
    }

    pub fn new(reference: Gd<Resource>) -> Self {
        // StaticRefCount::maybe_inc_ref(&reference.share());
        maybe_inc_ref(&reference);

        Self {
            resource_id: reference.instance_id(),
        }
    }
}

impl Clone for ErasedGdResource {
    fn clone(&self) -> Self {
        // StaticRefCount::maybe_inc_ref::<Resource>(
        //     &Gd::try_from_instance_id(self.resource_id).unwrap(),
        // );
        try_maybe_inc_ref::<Resource>(&Gd::try_from_instance_id(self.resource_id).ok());

        Self {
            resource_id: self.resource_id.clone(),
        }
    }
}

impl Drop for ErasedGdResource {
    fn drop(&mut self) {
        let gd = self.get();
        // let is_last = StaticRefCount::maybe_dec_ref(&gd); // may drop
        let is_last = maybe_dec_ref(&gd); // may drop
        if is_last {
            unsafe {
                sys::interface_fn!(object_destroy)(gd.obj_sys());
            }
        }
    }
}
