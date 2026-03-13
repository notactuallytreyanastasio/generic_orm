package temper.std;
public class StdGlobal {
    private StdGlobal() {}
    static {
        try {
            Class.forName("temper.std.json.JsonGlobal");
            Class.forName("temper.std.temporal.TemporalGlobal");
            Class.forName("temper.std.testing.TestingGlobal");
            Class.forName("temper.std.net.NetGlobal");
            Class.forName("temper.std.regex.RegexGlobal");
        } catch (ClassNotFoundException e) {
            throw new NoClassDefFoundError(e.getMessage());
        }
    }
}