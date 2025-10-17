use super::*;

#[skyline::hook(replace = GroundModule::can_entry_cliff)]
unsafe extern "C" fn can_entry_cliff_replace(fighter: &mut BattleObjectModuleAccessor) -> bool {
    false
}

pub fn install() {
    skyline::install_hooks!(
        can_entry_cliff_replace
    );
}
