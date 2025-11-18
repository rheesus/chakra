


pub use bevy_ecs::world::World;
pub use bevy_ecs::resource::Resource;
pub use bevy_ecs::change_detection::Res;
pub use bevy_ecs::change_detection::ResMut;
pub use bevy_ecs::schedule::ScheduleLabel;



use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::schedule::Schedules;
use bevy_ecs::system::ScheduleSystem;



pub trait WorldWrapper {
	fn add_systems<M>(
		&mut self,
		label: impl ScheduleLabel,
		system: impl IntoScheduleConfigs<ScheduleSystem, M>
	);

	fn schedule_exists(&self, label: impl ScheduleLabel) -> bool;

	fn run_schedule_if_exists<L>(&mut self, label: L)
		where L: ScheduleLabel + Clone;
}

impl WorldWrapper for World {
	fn add_systems<M>(
		&mut self,
		schedule_label: impl ScheduleLabel,
		systems: impl IntoScheduleConfigs<ScheduleSystem, M>
	) {
		self.get_resource_or_init::<Schedules>()
			.add_systems(schedule_label, systems);
	}

	fn schedule_exists(&self, label: impl ScheduleLabel) -> bool {
		self.get_resource::<Schedules>()
			.map(|s| s.contains(label))
			.unwrap_or(false)
	}

	fn run_schedule_if_exists<L>(&mut self, label: L)
		where L: ScheduleLabel + Clone
	{
		if self.schedule_exists(label.clone()) {
			self.run_schedule(label);
		}
	}
}



