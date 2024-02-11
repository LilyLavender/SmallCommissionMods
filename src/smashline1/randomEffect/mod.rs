use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
		hash40
    },
    smash_script::*,
	smashline::*
};

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for i in 0..36 {
        if i == 32 {
			let randomEffectType = sv_math::rand(hash40("fighter"), 1000) as usize;
			let colAttrVisual = [
				hash40("collision_attr_aura"),
				hash40("collision_attr_coin"),
				hash40("collision_attr_cutup"),
				hash40("collision_attr_elec"),
				hash40("collision_attr_fire"),
				hash40("collision_attr_magic"),
				hash40("collision_attr_normal"),
				hash40("collision_attr_pierce"),
				hash40("collision_attr_punch"),
				hash40("collision_attr_purple"),
				hash40("collision_attr_rush"),
				hash40("collision_attr_stab"),
				hash40("collision_attr_sting"),
				hash40("collision_attr_water"),
				hash40("collision_attr_whip")
			];
			let colAttrSpecial = [
				hash40("collision_attr_bind"),
				hash40("collision_attr_bury"),
				hash40("collision_attr_curse_poison"),
				hash40("collision_attr_flower"),
				hash40("collision_attr_ice"),
				hash40("collision_attr_lay"),
				hash40("collision_attr_paralyze"),
				hash40("collision_attr_sleep"),
				hash40("collision_attr_slip"),
				hash40("collision_attr_turn")
			];
			
			let chosenEffect = if randomEffectType < 400 {
				colAttrVisual[sv_math::rand(hash40("fighter"), colAttrVisual.len() as i32) as usize]
			} else if randomEffectType > 400 {
				colAttrSpecial[sv_math::rand(hash40("fighter"), colAttrSpecial.len() as i32) as usize]
			} else {
				hash40("collision_attr_death")
			};
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(chosenEffect));
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
