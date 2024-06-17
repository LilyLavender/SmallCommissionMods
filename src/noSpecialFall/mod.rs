use smash::hash40;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon};
use smashline::*;
use smash_script::*;
use std::f32::consts::E;

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe { 
		if(MotionModule::motion_kind(fighter.module_accessor) == hash40("fall_special")) {
			fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
		}
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
    );
}