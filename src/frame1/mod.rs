use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40
    },
    smash_script::*,
    smashline::*
};
use smash::lib::L2CValue;
use smash::lib::L2CAgent;

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    MotionModule::set_rate(boma, 1.0);
    original!()(lua_state);
}

#[fighter_frame_callback]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO || 
        status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_FAILURE || 
        status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_HANG || 
        status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_LANDING || 
        status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_REACH || 
        status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_100 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_START || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4 || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD || 
        status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_START || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_CUT || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH_PULL || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_JUMP || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_TURN || 
        status_kind == *FIGHTER_STATUS_KIND_CATCH_WAIT || 
        status_kind == *FIGHTER_STATUS_KIND_CLIFF_ATTACK || 
        status_kind == *FIGHTER_STATUS_KIND_FINAL || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_FALL || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_JUMP || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_JUMP_SQUAT || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_LANDING || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_TURN || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_WAIT || 
        status_kind == *FIGHTER_STATUS_KIND_HAMMER_WALK || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_GRASS_PULL || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_LIFT_TURN || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_LIFT_WAIT || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_LIFT_WALK || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_ROCKETBELT_HOVER_KEEP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SCREW_FALL || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_AIR || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_FLY || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP_AERIAL || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_JUMP_SQUAT || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_LANDING || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_TURN || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_WAIT || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_B || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_B || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_BRAKE_F || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SHOOT_WALK_F || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SPECIALFLAG_HOIST || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_STARRING || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_STARRING_SHOOT || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING_DASH || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING_S3 || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING_S4 || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_DASH || 
        status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY || 
        status_kind == *FIGHTER_STATUS_KIND_LANDING || 
        status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR || 
        status_kind == *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT || 
        status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL || 
        status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT || 
        status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || 
        status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW || 
        status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N || 
        status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S || 
        status_kind == *FIGHTER_STATUS_KIND_THROW || 
        status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE || 
        status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR || 
        status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV || 
        status_kind == *FIGHTER_STATUS_KIND_TREAD_FALL || 
        status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP {
            MotionModule::set_rate(fighter.module_accessor, 10.0);
        }
        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace
    );
    smashline::install_agent_frame_callbacks!(
        global_fighter_frame
    );
}