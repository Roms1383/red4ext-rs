native func SumInts(ints: array<Int32>) -> Int32;
native func UseTypes(name: CName, tweak: TweakDBID, item: ItemID, entity: EntityID, res: ResRef) -> Void;
native func CallDemo(player: ref<PlayerPuppet>) -> Void;

native func InitializeManager(manager: ref<SomeManager>) -> Void;
public class SomeManager extends IScriptable {
    public func SomeRedscriptMethod() -> Void {
        LogChannel(n"DEBUG", s"called SomeRedscriptMethod");
    }
    public static func SomeStaticRedscriptMethod() -> Void {
        LogChannel(n"DEBUG", s"called SomeStaticRedscriptMethod");
    }
}

public func TestManager() -> Void {
    let manager: ref<SomeManager> = new SomeManager();
    InitializeManager(manager);
    manager = null;
}