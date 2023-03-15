use smash::hash40;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::lib::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon};
use smashline::*;
use smash_script::*;

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    // let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    // let fighter_kind = smash::app::utility::get_kind(boma);
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for i in 0..36 {
        if i == 32 {
            let randomEffect = sv_math::rand(hash40("fighter"), 23) as usize;
			let randomTxt = [
				82430076406,
				84299446750,
				82425287246,
				84607901136,
				116764182975,
				88148215354,
				87856530745,
				82781917156,
				84803683138,
				90808232845,
				80658106433,
				81191629927,
				88498338105,
				92925133491,
				101406614725,
				89104769103,
				91968837809,
				88531030715,
				82063051125,
				82443485437,
				87514974371,
				84931485642,
				87658628301
			];
			let chosen = randomTxt[randomEffect];
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(chosen));
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        attack_replace
    );
}
