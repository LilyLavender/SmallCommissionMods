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

static mut changed: bool = false;

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		
		let p1d = DamageModule::damage(boma1, 0); 
		let p2d = DamageModule::damage(boma2, 0); 
				
		if (StatusModule::status_kind(boma1) == *FIGHTER_STATUS_KIND_CATCH_ATTACK && changed == false) {
			DamageModule::add_damage(boma1, (p2d - p1d), 0);
			DamageModule::add_damage(boma2, (p1d - p2d), 0);
			changed = true;
		}
		
		if((StatusModule::status_kind(boma1) == *FIGHTER_STATUS_KIND_WALK || StatusModule::status_kind(boma1) == *FIGHTER_STATUS_KIND_RUN || StatusModule::status_kind(boma1) == *FIGHTER_STATUS_KIND_WAIT || sv_information::is_ready_go() == false)) {
			changed = false;
		}
		
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame,
    );
}