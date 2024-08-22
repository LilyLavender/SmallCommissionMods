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

unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		// Get bomas
        let boma = fighter.module_accessor;
        let boma1 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(0));
        let boma2 = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
		
		// Get positions
		let b1x = PostureModule::pos_x(boma1);
		let b1y = PostureModule::pos_y(boma1);
		let b2x = PostureModule::pos_x(boma2);
		let b2y = PostureModule::pos_y(boma2);
		
		// Distance formula
		let dSquared: f32 = (b1x - b2x) * (b1x - b2x) + (b1y - b2y) * (b1y - b2y);
		let d = dSquared.sqrt();
		
		// Get player percents
		let p1d = DamageModule::damage(boma1, 0);
		let p2d = DamageModule::damage(boma2, 0);
		
		// Kill players
        if d <= 25.0
        && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DEAD 
		&& StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_REBIRTH 
		&& StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_STANDBY 
		&& StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_DEMO 
		&& StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_ENTRY 
		&& StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_NONE {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DEAD, false);
		}
    }
}

pub fn install() {
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame)
        .install();
}
