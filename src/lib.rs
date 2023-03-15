//When building the mod, I just uncomment whichever one I'm working on

#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_macros)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]


//mod randomEffect;
//mod quarter;
//mod closeKill;
//mod noSpecialFall;
//mod delayedKnockback;
//mod airIsGround;
//mod grabSwap;

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}

#[skyline::main(name = "randomEffect")]
pub fn main() {
    //randomEffect::install();
	//quarter::install();
	//closeKill::install();
	//noSpecialFall::install();
	//delayedKnockback::install();
	//airIsGround::install();
	//grabSwap::install();
}