use red4ext_rs::prelude::*;

define_plugin! {
    name: "example",
    author: "author",
    version: 0:1:0,
    on_register: {
        register_function!("SumInts", sum_ints);
        register_function!("UseTypes", use_types);
        register_function!("CallDemo", call_demo);
        register_function!("CallProxy", call_proxy);
    }
}

/// call function with primitives
///
/// try in-game in CET console:
///
/// ```lua
/// LogChannel(CName.new("DEBUG"), SumInts({ 2000, 77 }))
/// ```
fn sum_ints(ints: Vec<i32>) -> i32 {
    ints.iter().sum()
}

/// call function with game special types
///
/// try in-game in CET console:
///
/// ```lua
/// UseTypes(CName.new("Test"), TDBID.Create("Items.BlackLaceV0"), ItemID.FromTDBID(TDBID.Create("Items.BlackLaceV0")), Game.GetPlayer():GetEntityID(), "base\\characters\\entities\\player\\player_ma_fpp.ent")
/// ```
/// > ⚠️ output can be found in mod's logs
fn use_types(name: CName, tweak: TweakDbId, item: ItemId, entity: EntityId, res: ResRef) {
    info!("got CName {name:#?}, TweakDBID {tweak:#?}, ItemID {item:#?}, EntityID {entity:#?}, ResRef {res:#?}");
    let res = res_ref!("base" / "mod" / "custom.ent").unwrap();
    info!("created res ref: {res:#?}");
}

/// call function with handle
///
/// try in-game in CET console:
///
/// ```lua
/// CallDemo(Game.GetPlayer())
/// ```
/// > ⚠️ output can be found in mod's logs
fn call_demo(player: PlayerPuppet) {
    let res = add_u32(2, 2);
    info!("2 + 2 = {}", res);

    info!("player display name: {}", player.get_display_name());
    info!("player vehicles: {}", player.get_unlocked_vehicles_size());
    player.disable_camera_bobbing(true);
    info!(
        "can apply breating effect: {}",
        PlayerPuppet::can_apply_breathing_effect(player.clone())
    );
}

/// import a global operator
///
/// function names gets automatically mangled,
/// this one becomes `OperatorAdd;Uint32Uint32;Uint32`
///
/// try in-game in CET console:
///
/// ```lua
/// LogChannel(CName.new("DEBUG"), OperatorAdd(2000, 77))
/// ```
/// > ⚠️ output can be found in mod's logs
#[redscript_global(name = "OperatorAdd", operator)]
fn add_u32(l: u32, r: u32) -> u32;

/// define a binding for a class type
#[derive(Clone, Default)]
#[repr(transparent)]
struct PlayerPuppet(WRef<IScriptable>);

#[redscript_import]
impl PlayerPuppet {
    /// imports `public native func GetDisplayName() -> String`
    ///
    /// the method name is interpreted as PascalCase
    ///
    /// you can also specify it explicitly with a `name` attribute
    #[redscript(native)]
    fn get_display_name(&self) -> String;

    /// imports `private func GetUnlockedVehiclesSize() -> Int32`
    fn get_unlocked_vehicles_size(&self) -> i32;

    /// imports 'private func DisableCameraBobbing(b: Bool) -> Void'
    fn disable_camera_bobbing(&self, toggle: bool);

    /// imports 'public final static func CanApplyBreathingEffect(player: wref<PlayerPuppet>) -> Bool'
    fn can_apply_breathing_effect(player: PlayerPuppet) -> bool;
}

unsafe impl RefRepr for PlayerPuppet {
    type Type = Weak;

    const CLASS_NAME: &'static str = "PlayerPuppet";
}

#[allow(dead_code)]
#[repr(transparent)]
#[derive(Clone, Default)]
struct RustTdbId(WRef<IScriptable>);

unsafe impl RefRepr for RustTdbId {
    type Type = Weak;

    const CLASS_NAME: &'static str = "RustTDBID";
}

#[redscript_import]
impl RustTdbId {
    #[redscript(name = "ToStringDEBUG")]
    fn to_string_debug(&self, id: TweakDbId) -> String;
}

fn call_proxy(proxy: RustTdbId, id: TweakDbId) -> String {
    proxy.to_string_debug(id)
}
