#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
    unused_parens,
    unused_macros,
    unused_variables,
    unused_assignments,
    non_upper_case_globals,
    non_snake_case,
    dead_code,
    clippy::borrow_interior_mutable_const
)]

// Add the mod you want to build here

// Only used in nerfPacman & nerfGameWatch for keeping them in place
extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}

#[skyline::main(name = "smashline_test")]
pub fn main() {
    // Add the mod you want to build here
}
