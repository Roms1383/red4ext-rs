native func SumInts(ints: array<Int32>) -> Int32;
native func UseTypes(name: CName, tweak: TweakDBID, item: ItemID, entity: EntityID, res: ResRef) -> Void;
native func CallDemo(player: ref<PlayerPuppet>) -> Void;
native func CallProxy(id: TweakDBID) -> String;

public class RustTDBID extends IScriptable {
    public static func ToStringDEBUG(id: TweakDBID) -> String {
        return TDBID.ToStringDEBUG(id);
    }
}

public func TestProxy() -> Void {
    let id: TweakDBID = t"Items.BlackLaceV0";
    let stringified = CallProxy(id);
    LogChannel(n"DEBUG", s"CallProxy returned with \(stringified)");
}