use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Debug, Hash, Eq, PartialEq, Clone, States)]
pub struct PlayerInputPreUpdate;

impl Plugin for PlayerInputPreUpdate {
    fn build(&self, app: &mut App) {
        app.add_schedule(Schedule::new(PlayerInputPreUpdate));
        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_after(PreUpdate, PlayerInputPreUpdate);
    }
}
