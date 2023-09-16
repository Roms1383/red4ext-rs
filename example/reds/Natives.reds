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
