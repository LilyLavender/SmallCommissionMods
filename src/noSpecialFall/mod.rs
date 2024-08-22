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

#[skyline::hook(replace = smash::lua2cpp::L2CFighterBase_change_status)]
pub unsafe fn change_status_replace(fighter: &mut L2CFighterBase, status_kind: L2CValue, unk: L2CValue) -> L2CValue {
    let mut status_kind = status_kind;
    
    if status_kind == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        status_kind = FIGHTER_STATUS_KIND_FALL.into();
    }
    
    original!()(fighter, status_kind, unk)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            change_status_replace
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}
