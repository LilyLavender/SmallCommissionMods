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
		//get bomas
        let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		
		//get positions
		let b1x = PostureModule::pos_x(boma1);
		let b1y = PostureModule::pos_y(boma1);
		let b2x = PostureModule::pos_x(boma2);
		let b2y = PostureModule::pos_y(boma2);
		
		//distance formula
		let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
		let d = dSquared.sqrt();
		
		//get player percents
		let p1d = DamageModule::damage(boma1, 0);
		let p2d = DamageModule::damage(boma2, 0);
		
		//kill player 1
        if (d <= 25.0) {
			if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_DEAD && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_DEAD) { 
				if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_REBIRTH && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_REBIRTH) { 
					if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_STANDBY && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_STANDBY) { 
						if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_DEMO && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_DEMO) {
							if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_ENTRY && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_ENTRY) {
								if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_NONE && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_NONE) {
									StatusModule::change_status_request_from_script(boma1, *FIGHTER_STATUS_KIND_DEAD, false);
								}
							}
						}
					}
				}
			}
		}
		//kill player 2
        if (d <= 25.0) {
			if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_DEAD && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_DEAD) { 
				if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_REBIRTH && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_REBIRTH) { 
					if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_STANDBY && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_STANDBY) { 
						if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_DEMO && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_DEMO) {
							if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_ENTRY && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_ENTRY) {
								if (StatusModule::status_kind(boma1) != *FIGHTER_STATUS_KIND_NONE && StatusModule::status_kind(boma2) != *FIGHTER_STATUS_KIND_NONE) {
									StatusModule::change_status_request_from_script(boma2, *FIGHTER_STATUS_KIND_DEAD, false);
								}
							}
						}
					}
				}
			}
		}
		
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
    );
}