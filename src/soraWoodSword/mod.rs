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

unsafe extern "C" fn trail_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        // Damage & knockback multipliers
        AttackModule::set_power_up(boma, 0.3);
		AttackModule::set_reaction_mul(boma, 0.5);
		
        let status_kind = StatusModule::status_kind(boma);
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK 
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F
        || status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N {
			if FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), false) - MotionModule::frame(boma) <= 7.0 {
				CancelModule::enable_cancel(boma);
			} 
		}
		
		
    }
}

unsafe extern "C" fn trail_game_specialn1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}

unsafe extern "C" fn trail_game_specialn2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(5.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.25);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::set_float(agent.module_accessor, 4.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 16.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -8.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 24.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -2.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 12.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -14.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 42, 64, 0, 52, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(6.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 57.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
}

unsafe extern "C" fn trail_game_specialn3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}

unsafe extern "C" fn trail_game_specialairn1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
}

unsafe extern "C" fn trail_game_specialairn2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 100, 40, 0, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(5.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.25);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::set_float(agent.module_accessor, 4.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 16.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -8.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 24.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -2.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 12.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, -14.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLOAT_ANGLE);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    wait(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 42, 64, 0, 52, 3.2, 0.0, 6.4, 2.2, Some(0.0), Some(6.4), Some(6.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 57.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
}

unsafe extern "C" fn trail_game_specialairn3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_N3_FLAG_CHANGE_MAGIC);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

unsafe extern "C" fn trail_effect_blank(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn trail_effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade1"), Hash40::new("tex_trail_keyblade2"), 7, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn trail_effect_attackhi4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn trail_effect_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_trail_keyblade3"), Hash40::new("tex_trail_keyblade2"), 20, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0 as u64, *EFFECT_AXIS_X, 0.0 as u64, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

pub fn install() {
    Agent::new("trail")
        .on_line(Main, trail_frame)
        .game_acmd("game_specialn1", trail_game_specialn1, Default)
        .game_acmd("game_specialn2", trail_game_specialn2, Default)
        .game_acmd("game_specialn3", trail_game_specialn3, Default)
        .game_acmd("game_specialairn1", trail_game_specialairn1, Default)
        .game_acmd("game_specialairn2", trail_game_specialairn2, Default)
        .game_acmd("game_specialairn3", trail_game_specialairn3, Default)
        .effect_acmd("effect_attackhi3", trail_effect_attackhi3, Default)
        .effect_acmd("effect_attackhi4", trail_effect_attackhi4, Default)
        .effect_acmd("effect_attacks4", trail_effect_attacks4, Default)
        .effect_acmd("effect_specialn1", trail_effect_blank, Default)
        .effect_acmd("effect_specialn2", trail_effect_blank, Default)
        .effect_acmd("effect_specialn3", trail_effect_blank, Default)
        .effect_acmd("effect_specialairn1", trail_effect_blank, Default)
        .effect_acmd("effect_specialairn2", trail_effect_blank, Default)
        .effect_acmd("effect_specialairn3", trail_effect_blank, Default)
        .effect_acmd("effect_specialn1start", trail_effect_blank, Default)
        .effect_acmd("effect_specialn12", trail_effect_blank, Default)
        .effect_acmd("effect_specialn1end", trail_effect_blank, Default)
        .effect_acmd("effect_appealhir", trail_effect_blank, Default)
        .effect_acmd("effect_appealhil", trail_effect_blank, Default)
        .effect_acmd("effect_appealsl", trail_effect_blank, Default)
        .effect_acmd("effect_appealsr", trail_effect_blank, Default)
        .effect_acmd("effect_appealhi1", trail_effect_blank, Default)
        .effect_acmd("effect_appealhi2", trail_effect_blank, Default)
        .effect_acmd("effect_appealhi3", trail_effect_blank, Default)
        .install();
}
