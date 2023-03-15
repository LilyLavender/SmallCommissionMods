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

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
    return *SITUATION_KIND_GROUND;
}

pub fn install() {
    skyline::install_hooks!(
        situation_kind_replace
    );
}