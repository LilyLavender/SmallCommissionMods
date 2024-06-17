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

static pos1x: f32 = -70.0;
static pos1y: f32 = 0.0;
static pos2x: f32 = -40.0;
static pos2y: f32 = 24.23;
static pos3x: f32 = 0.0;
static pos3y: f32 = 0.0;
static pos4x: f32 = 40.0;
static pos4y: f32 = 24.23;
static pos5x: f32 = 70.0;
static pos5y: f32 = 0.0;

static mut gamePos: [i32; 8] = [3; 8];
static mut gamePosx: [f32; 8] = [0.0; 8];
static mut gamePosy: [f32; 8] = [0.0; 8];
static mut cooldown: [f32; 8] = [40.0; 8];
static mut changed: [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_GAMEWATCH )]
fn gamewatch_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		lua_bind::FighterManager::set_position_lock(singletons::FighterManager(), smash::app::FighterEntryID(entry_id as i32), true);
		PostureModule::set_pos(fighter.module_accessor, &Vector3f{ x: gamePosx[entry_id], y: gamePosy[entry_id], z: 0.0 });
		
		if !changed[entry_id] {
			if MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_s_4") && 
			   MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_hi_4") && 
			   MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_lw_4") && 
			   MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_s_4_charge") && 
			   MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_hi_4_charge") && 
			   MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attack_lw_4_charge") {
				
				if ControlModule::get_stick_x(fighter.module_accessor) < -0.707 { // LEFT
					if gamePos[entry_id] != 1 {
						gamePos[entry_id] -= 1;
						changed[entry_id] = true;
					}
				} else if ControlModule::get_stick_x(fighter.module_accessor) > 0.707 { // RIGHT
					if gamePos[entry_id] != 5 {
						gamePos[entry_id] += 1;
						changed[entry_id] = true;
					}
				} 
			}
			
			if gamePos[entry_id] == 1 {
				gamePosx[entry_id] = pos1x;
				gamePosy[entry_id] = pos1y;
			} else if gamePos[entry_id] == 2 {
				gamePosx[entry_id] = pos2x;
				gamePosy[entry_id] = pos2y;
			} else if gamePos[entry_id] == 3 {
				gamePosx[entry_id] = pos3x;
				gamePosy[entry_id] = pos3y;
			} else if gamePos[entry_id] == 4 {
				gamePosx[entry_id] = pos4x;
				gamePosy[entry_id] = pos4y;
			} else if gamePos[entry_id] == 5 {
				gamePosx[entry_id] = pos5x;
				gamePosy[entry_id] = pos5y;
			}
		}
		
		if changed[entry_id] {
			cooldown[entry_id] -= 1.0; 
		}
		if cooldown[entry_id] <= 0.0 {
			cooldown[entry_id] = 40.0;
			changed[entry_id] = false;
		} 
		if DamageModule::damage(fighter.module_accessor, 0) >= 150.0 {
			fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
			// this takes all stocks away instantly, to just take one, you'll have to put in some extra checks
		}
	}
}

#[skyline::hook(replace = StatusModule::situation_kind)]
unsafe fn situation_kind_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> i32 {
	if smash::app::utility::get_kind(&mut *module_accessor) == FIGHTER_KIND_GAMEWATCH {
		return *SITUATION_KIND_GROUND;
	}
	original!()(module_accessor)
}

pub fn install() {
	smashline::install_agent_frames!(
		gamewatch_frame,
    );
	skyline::install_hooks!(
		situation_kind_replace,
    );
}