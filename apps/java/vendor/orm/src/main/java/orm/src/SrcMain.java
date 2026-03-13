package orm.src;
import temper.core.Core;
public final class SrcMain {
    private SrcMain() {
    }
    public static void main(String[] args) throws ClassNotFoundException {
        Core.initSimpleLogging();
        Class.forName("orm.src.SrcGlobal");
        Core.waitUntilTasksComplete();
    }
}
