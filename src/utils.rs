use bevy::{
    ecs::{schedule::{SystemConfig, SystemConfigs}, system::SystemParam},
    prelude::*,
};
use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

/// Bevy Resource that is available when the app is updated through `_process` callback
#[derive(Resource)]
pub struct GodotVisualFrame;

/// Bevy Resource that is available when the app is updated through `_physics_process` callback
#[derive(Resource)]
pub struct GodotPhysicsFrame;

/// Adds `as_physics_system` that schedules a system only for the physics frame
pub trait AsPhysicsSystem<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_physics_system(self) -> SystemConfigs;
}

impl<Params, T: IntoSystem<(), (), Params>> AsPhysicsSystem<Params> for T {
    fn as_physics_system(self) -> SystemConfigs {
        self.run_if(resource_exists::<GodotPhysicsFrame>)
    }
}

/// Adds `as_visual_system` that schedules a system only for the frame
pub trait AsVisualSystem<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_visual_system(self) -> SystemConfigs;
}

impl<Params, T: IntoSystem<(), (), Params>> AsVisualSystem<Params> for T {
    fn as_visual_system(self) -> SystemConfigs {
        self.run_if(resource_exists::<GodotVisualFrame>)
    }
}

/// SystemParam to keep track of an independent delta time
///
/// Not every system runs on a Bevy update and Bevy can be updated multiple
/// during a "frame".
#[derive(SystemParam)]
pub struct SystemDeltaTimer<'w, 's> {
    last_time: Local<'s, Option<Instant>>,
    #[system_param(ignore)]
    marker: PhantomData<&'w ()>,
}

impl<'w, 's> SystemDeltaTimer<'w, 's> {
    /// Returns the time passed since the last invocation
    pub fn delta(&mut self) -> Duration {
        let now = Instant::now();
        let last_time = self.last_time.unwrap_or(now);

        *self.last_time = Some(now);

        now - last_time
    }

    pub fn delta_seconds(&mut self) -> f32 {
        self.delta().as_secs_f32()
    }

    pub fn delta_seconds_f64(&mut self) -> f64 {
        self.delta().as_secs_f64()
    }
}