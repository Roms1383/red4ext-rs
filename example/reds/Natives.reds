native func SumInts(ints: array<Int32>) -> Int32;
native func UseTypes(name: CName, tweak: TweakDBID, item: ItemID, entity: EntityID, res: ResRef, sim: EngineTime) -> Void;
native func CallDemo(player: ref<PlayerPuppet>) -> Void;
native func TestNativeStruct(female: String, male: String, locale: Locale) -> Translation;

enum Locale {
    English = 0,
    Polish = 1,
    French = 2,
}

public struct Translation {
    let Female: String;
    let Male: String;
    let Locale: Locale;
    public static func Create(female: String, male: String, locale: Locale) -> Translation {
        return new Translation(female, male, locale);
    }
}

@addMethod(PlayerPuppet)
public func TestRoundTrip() -> Void {
    let translation = TestNativeStruct("beautiful", "handsome", Locale.English);
    LogChannel(n"DEBUG", s"female: \(translation.Female)");
    LogChannel(n"DEBUG", s"male: \(translation.Male)");
    LogChannel(n"DEBUG", s"locale: \(ToString(translation.Locale))");
}