use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CAgent, L2CValue},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

static mut timer: [f32; 8] = [0.0; 8];

unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("PizaM"), true);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		timer[entry_id] += 1.0;
        
		lua_bind::FighterManager::set_position_lock(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32), true);
		if ControlModule::get_stick_y(fighter.module_accessor) > 0.707 { // UP
			PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{x: 0.0, y: 0.8});
			PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 270.0, y: 0.0, z: 0.0 }, 0);
			macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 5.0, 90, 90, 0, 50, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		} else if ControlModule::get_stick_x(fighter.module_accessor) < -0.707 { // LEFT
			PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{x: -0.8, y: 0.0});
			PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 180.0, y: 0.0, z: 0.0 }, 0);
			macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 5.0, 135, 90, 0, 50, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		} else if ControlModule::get_stick_x(fighter.module_accessor) > 0.707 { // RIGHT
			PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{x: 0.8, y: 0.0});
			PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.0, z: 0.0 }, 0);
			macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 5.0, 45, 90, 0, 50, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		} else if ControlModule::get_stick_y(fighter.module_accessor) < -0.707 { // DOWN
			PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{x: 0.0, y: -0.8});
			PostureModule::set_rot(fighter.module_accessor, &Vector3f{ x: 90.0, y: 0.0, z: 0.0 }, 0);
			macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 5.0, 270, 90, 0, 50, 3.0, 0.0, 4.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		} else {
			AttackModule::clear_all(fighter.module_accessor);
		}
		PostureModule::set_lr(fighter.module_accessor, 1.0);
		if timer[entry_id] >= 40.0 {
			timer[entry_id] = 0.0;
			MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 5.0, 1.0, false, 0.0, false, false);
		} 
		if MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_dash") {
			MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_dash"), 5.0, 1.0, false, 0.0, false, false);
		}
		if DamageModule::damage(fighter.module_accessor, 0) >= 150.0 {
			fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
		}
	}
}

unsafe extern "C" fn pacman_game_attackdash(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn pacman_effect_attackdash(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn pacman_expression_entry(agent: &mut L2CAgentBase) {}

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	if smash::app::utility::get_kind(&mut *module_accessor) == FIGHTER_KIND_PACMAN {
		return *SITUATION_KIND_AIR;
	} 
	original!()(module_accessor)
}

pub fn install() {
	skyline::install_hooks!(
		situation_kind_replace,
    );
    Agent::new("pacman")
        .on_line(Main, pacman_frame)
        .game_acmd("game_attackdash", pacman_game_attackdash, Default)
        .effect_acmd("effect_attackdash", pacman_effect_attackdash, Default)
        .expression_acmd("expression_entryl", pacman_expression_entry, Default)
        .expression_acmd("expression_entryr", pacman_expression_entry, Default)
        .install();
}
