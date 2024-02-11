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
    let fighter_kind = smash::app::utility::get_kind(&mut *boma);
	
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
    for i in 0..36 {
		
        if fighter_kind == FIGHTER_KIND_POPO || fighter_kind == FIGHTER_KIND_NANA {
			
			if i == 31 {
				l2c_agent.push_lua_stack(&mut L2CValue::new_int(1));
			} else {
				l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
			}
			
		} else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
		}
    }
    original!()(lua_state);
}

pub fn install() {
	skyline::install_hooks!(
        attack_replace,
    );
}