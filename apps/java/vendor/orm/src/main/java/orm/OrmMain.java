package orm;
import temper.core.Core;
public class OrmMain {
    private OrmMain() {}
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("orm.OrmGlobal");
    }
}