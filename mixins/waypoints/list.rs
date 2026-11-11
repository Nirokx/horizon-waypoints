// The mod's row in Options > Mods, with its two keys.
fn waypoints_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::waypoints_mod as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: m::title(),
        description: m::DESCRIPTION.into(),
        keys: m::keys(),
    });
}
