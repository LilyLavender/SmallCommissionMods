use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn purin_game_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("bust"), 20.0, 88, 110 /* kbg */, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
		checkForHit(agent);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {
		checkForHit(agent);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {
		checkForHit(agent);
	}
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        checkForHit(agent);
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn checkForHit(agent: &mut L2CAgentBase) {
	let ama = agent.module_accessor;
	if AttackModule::is_infliction_status(ama, *COLLISION_KIND_MASK_HIT) {
		MotionModule::set_frame(ama, MotionModule::end_frame(ama) - 10.0, true);
	}
}

pub fn install() {
    Agent::new("purin")
        .game_acmd("game_speciallw", purin_game_speciallw, Default)
        .install();
}
