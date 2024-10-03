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

unsafe extern "C" fn master_game_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    execute(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn master_game_attacklw4charge(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    execute(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
}

unsafe extern "C" fn master_game_attacks4hi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    execute(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
}

unsafe extern "C" fn master_game_attacks4lw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    execute(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 25.0);
}

unsafe extern "C" fn master_game_specialairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 1.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 96.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(agent.lua_state_agent, 117.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
}

unsafe extern "C" fn master_game_specialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
}

unsafe extern "C" fn master_game_specialairnmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, -0.6, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_REVERT_FALL_SPEED);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
}

unsafe extern "C" fn master_game_specialairnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN);
    }
    frame(agent.lua_state_agent, 45.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT);
    }
}

unsafe extern "C" fn master_game_specialairsf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 10.0, 5.0, 5.0, 9.0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 5.0, 5.0, 5.0, 5.0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_specialairsfdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::SET_SPEED_EX(agent, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 10.0, 5.0, 5.0, 9.0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 5.0, 5.0, 5.0, 5.0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn master_game_specialairsstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
}

unsafe extern "C" fn master_game_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 3.0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_START_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_TURN_CHECK);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_1);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_END_SUPER_ARMOR);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_FORBID_LANDING);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
    frame(agent.lua_state_agent, 96.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
    frame(agent.lua_state_agent, 117.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_INHERIT_LANDING_2);
    }
}

unsafe extern "C" fn master_game_speciallwhit(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_PULL_AXE);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_DEGREE_X);
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_CONTROL_ENERGY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_LW_FLAG_REVERT_FALL_SPEED);
    }
}

unsafe extern "C" fn master_game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn master_game_specialnmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn master_game_specialnstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_ENABLE_TURN);
    }
    frame(agent.lua_state_agent, 45.0);
    macros::FT_MOTION_RATE(agent, 0.9);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_N_FLAG_CAN_SHOOT);
    }
}

unsafe extern "C" fn master_game_specialsf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_F_REQUEST);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
}

unsafe extern "C" fn master_game_specialsfdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_UP_REQUEST);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_DOWN_REQUEST);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CHANGE_ATK_END_REQUEST);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MASTER_STATUS_SPECIAL_S_FLAG_SPEED_CONTROL_REQUEST);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_SPEAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn master_game_specialsstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.8);
}

unsafe extern "C" fn master_effect_attackairb(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_attackairf(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 5, 8, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }else{
			if macros::is_excute(agent) {
				macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 12, 8, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
		}
	}
}

unsafe extern "C" fn master_effect_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -5, 5, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn master_effect_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 7, 20, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 13, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
    }
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
	macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -13, 0, 0, 0, 0, 0, 0.95, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn master_effect_attacks4(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }else{
			if macros::is_excute(agent) {
				macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
		}
	}
}

unsafe extern "C" fn master_effect_attacks4hi(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }else{
			if macros::is_excute(agent) {
				macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
		}
	}
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
		macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn master_effect_attacks4lw(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -20, 18, 13.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }else{
			if macros::is_excute(agent) {
				macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -8, 17, 13.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
			}
		}
	}
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
		macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
	}
}

unsafe extern "C" fn master_effect_specialairlw(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_specialairlwcancel(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_specialairlwhit(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_specialairsf(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_specialairsfdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, 0.6);
    }
}

unsafe extern "C" fn master_effect_specialairsstart(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn master_effect_speciallwcancel(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_speciallwhit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 17, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.9);
    }
}

unsafe extern "C" fn master_effect_speciallwlanding2(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn master_effect_specialnmax(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn master_effect_specialsf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn master_effect_specialsfdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn master_effect_specialsstart(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("master")
        .game_acmd("game_attackairb", master_game_attackairb, Default)
        .game_acmd("game_attackairf", master_game_attackairf, Default)
        .game_acmd("game_attackairlw", master_game_attackairlw, Default)
        .game_acmd("game_attackairn", master_game_attackairn, Default)
        .game_acmd("game_attacklw4", master_game_attacklw4, Default)
        .game_acmd("game_attacklw4charge", master_game_attacklw4charge, Default)
        .game_acmd("game_attacks4", master_game_attacks4, Default)
        .game_acmd("game_attacks4hi", master_game_attacks4hi, Default)
        .game_acmd("game_attacks4lw", master_game_attacks4lw, Default)
        .game_acmd("game_specialairlw", master_game_specialairlw, Default)
        .game_acmd("game_specialairn", master_game_specialairn, Default)
        .game_acmd("game_specialairnmax", master_game_specialairnmax, Default)
        .game_acmd("game_specialairnstart", master_game_specialairnstart, Default)
        .game_acmd("game_specialairsf", master_game_specialairsf, Default)
        .game_acmd("game_specialairsfdash", master_game_specialairsfdash, Default)
        .game_acmd("game_specialairsstart", master_game_specialairsstart, Default)
        .game_acmd("game_speciallw", master_game_speciallw, Default)
        .game_acmd("game_speciallwhit", master_game_speciallwhit, Default)
        .game_acmd("game_specialn", master_game_specialn, Default)
        .game_acmd("game_specialnmax", master_game_specialnmax, Default)
        .game_acmd("game_specialnstart", master_game_specialnstart, Default)
        .game_acmd("game_specialsf", master_game_specialsf, Default)
        .game_acmd("game_specialsfdash", master_game_specialsfdash, Default)
        .game_acmd("game_specialsstart", master_game_specialsstart, Default)
        .effect_acmd("effect_attackairb", master_effect_attackairb, Default)
        .effect_acmd("effect_attackairf", master_effect_attackairf, Default)
        .effect_acmd("effect_attackairlw", master_effect_attackairlw, Default)
        .effect_acmd("effect_attackairn", master_effect_attackairn, Default)
        .effect_acmd("effect_attacklw4", master_effect_attacklw4, Default)
        .effect_acmd("effect_attacks4", master_effect_attacks4, Default)
        .effect_acmd("effect_attacks4hi", master_effect_attacks4hi, Default)
        .effect_acmd("effect_attacks4lw", master_effect_attacks4lw, Default)
        .effect_acmd("effect_specialairlw", master_effect_specialairlw, Default)
        .effect_acmd("effect_specialairlwcancel", master_effect_specialairlwcancel, Default)
        .effect_acmd("effect_specialairlwhit", master_effect_specialairlwhit, Default)
        .effect_acmd("effect_specialairsf", master_effect_specialairsf, Default)
        .effect_acmd("effect_specialairsfdash", master_effect_specialairsfdash, Default)
        .effect_acmd("effect_specialairsstart", master_effect_specialairsstart, Default)
        .effect_acmd("effect_speciallw", master_effect_speciallw, Default)
        .effect_acmd("effect_speciallwcancel", master_effect_speciallwcancel, Default)
        .effect_acmd("effect_speciallwhit", master_effect_speciallwhit, Default)
        .effect_acmd("effect_speciallwlanding2", master_effect_speciallwlanding2, Default)
        .effect_acmd("effect_specialnmax", master_effect_specialnmax, Default)
        .effect_acmd("effect_specialsf", master_effect_specialsf, Default)
        .effect_acmd("effect_specialsfdash", master_effect_specialsfdash, Default)
        .effect_acmd("effect_specialsstart", master_effect_specialsstart, Default)
        .install();
}
