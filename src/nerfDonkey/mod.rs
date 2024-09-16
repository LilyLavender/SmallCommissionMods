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

static mut timer: [f32; 8] = [0.0; 8];
const FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_TIMING_GOOD : i32 = 0x200000eb;

unsafe extern "C" fn donkey_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		timer[entry_id] += 1.0;
		
		if timer[entry_id] >= 44.0 {
			WorkModule::on_flag(fighter.module_accessor, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_TIMING_GOOD);
		} else {
			WorkModule::off_flag(fighter.module_accessor, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_TIMING_GOOD);
		}
		if timer[entry_id] == 52.0 {
			macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 4.0, true);
		}
		if timer[entry_id] >= 60.0 {
			timer[entry_id] = 0.0;
		}
	}
}

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(&mut *boma);
	
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
	l2c_agent.clear_lua_stack();
    for i in 0..36 {
		
        if fighter_kind == FIGHTER_KIND_DONKEY {
			
			if WorkModule::is_flag(boma, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_TIMING_GOOD) {
				
				if i == 3 {
					l2c_agent.push_lua_stack(&mut L2CValue::new_num(hitbox_params[3].get_f32() * 1.8));
				} else if i == 32 { // should probably replace those hashes with strings eventually
					if hitbox_params[32].get_int() != L2CValue::new_int(0x1330ee124e).get_int() {
						l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x13beb18342));
					} else {
						l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
					}
				} else {
					l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
				}
				
			} else {
				
				if i == 3 {
					l2c_agent.push_lua_stack(&mut L2CValue::new_num(hitbox_params[3].get_f32() / 4.0));
				} else {
					l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
				}
				
			}
			
		} else {
			l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
		}
    }
    original!()(lua_state);
}

pub fn install() {
    Agent::new("donkey")
        .on_line(Main, donkey_frame)
        .install();
    skyline::install_hooks!(
        attack_replace,
    );
}
