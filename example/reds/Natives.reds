@if(ModuleExists("Codeware"))
import Codeware.*

native func SumInts(ints: array<Int32>) -> Int32;
native func UseTypes(name: CName, tweak: TweakDBID, item: ItemID, entity: EntityID, res: ResRef) -> Void;
native func CallDemo(player: ref<PlayerPuppet>) -> Void;
native func CallNativeDemo(time: ref<TimeSystem>) -> Void;
native func CompareEngineTime(time: EngineTime, float: Float) -> Void;
native func DebugBoardDefinition(definition: BlackboardID_Uint) -> Void;


@addField(PlayerStateMachineDef)
public let BoardDefinition: BlackboardID_Uint;

// call like: Game.GetPlayer():Test()
@addMethod(PlayerPuppet)
public func Test() -> Void {
    let system: ref<TimeSystem> = GameInstance.GetTimeSystem(this.GetGame());
    let sim: EngineTime = system.GetSimTime();
    let float: Float = EngineTime.ToFloat(sim);
    CompareEngineTime(sim, float); // [ 245, 197, 155, 12, 53, 0, 0, 0 ] / 22784.48
    let str: String = EngineTime.ToString(sim);
    LogChannel(n"DEBUG", str); // [DEBUG] 22784.480204
}

@if(ModuleExists("Codeware"))
public func TestFields() -> Void {
    let definitions = Reflection.GetClass(n"gamebbAllScriptDefinitions");
    LogChannel(n"DEBUG", "reflection");
    let props = definitions.GetProperties();
    LogChannel(n"DEBUG", ToString(ArraySize(props)));
    for prop in props {
        LogChannel(n"DEBUG", s"\(prop.GetName()): \(prop.GetType().GetName())");
    }
}

@if(ModuleExists("Codeware"))
public func TestStatic() -> Void {
    let cls = Reflection.GetClass(n"EngineTime");
    let fromfloat = cls.GetStaticFunction(n"FromFloat");
    let tofloat = cls.GetStaticFunction(n"ToFloat");
    let engine = fromfloat.Call([837997.8]);
    let raw = tofloat.Call([engine]);
    LogChannel(n"DEBUG", ToString(raw));
}

@addMethod(PlayerPuppet)
public func TestRoundTrip() -> Void {
    let system: ref<TimeSystem> = GameInstance.GetTimeSystem(this.GetGame());
    let sim: EngineTime = system.GetSimTime();
    let float: Float = EngineTime.ToFloat(sim);
    LogChannel(n"DEBUG", ToString(float));
    let cls = Reflection.GetClass(n"EngineTime");
    let fromfloat = cls.GetStaticFunction(n"FromFloat");
    let outcome: Bool;
    let back = fromfloat.Call([float], outcome);
    LogChannel(n"DEBUG", ToString(outcome));
    let casted: EngineTime = FromVariant<EngineTime>(back);
    let roundtrip: Float = EngineTime.ToFloat(casted);
    LogChannel(n"DEBUG", ToString(roundtrip));
}
