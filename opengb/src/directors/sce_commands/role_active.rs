use crate::directors::sce_director::{SceCommand, SceState};

use crate::directors::SceneManagerExtensions;
use imgui::Ui;
use radiance::scene::SceneManager;

#[derive(Clone)]
pub struct SceCommandRoleActive {
    role_id: i32,
    active: i32,
}

impl SceCommand for SceCommandRoleActive {
    fn update(
        &mut self,
        scene_manager: &mut dyn SceneManager,
        ui: &mut Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        scene_manager
            .core_scene_mut_or_fail()
            .get_role_entity_mut(self.role_id)
            .set_active(self.active != 0);
        true
    }
}

impl SceCommandRoleActive {
    pub fn new(role_id: i32, active: i32) -> Self {
        Self { role_id, active }
    }
}
