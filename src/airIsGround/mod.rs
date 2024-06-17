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

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(boma: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    let status_kind = StatusModule::status_kind(boma);
    // This if statement makes it so the code doesn't take effect if the player is in hitstun
    if ![*FIGHTER_STATUS_KIND_DAMAGE, 
    *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
    *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
    *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
    *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
    *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
    ].contains(&status_kind) {
        return *SITUATION_KIND_GROUND;
    }
    original!()(boma)
}

// Causes the player to go down when holding down
unsafe extern "C" fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if ControlModule::get_stick_y(boma) <= -0.707 {
            // Change the y value to adjust how fast the player goes down
            PostureModule::add_pos_2d(boma, &Vector2f{x: 0.0, y: -0.8});
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        situation_kind_replace
    );
    Agent::new("fighter")
        .on_line(Main, global_fighter_frame)
        .install();
}