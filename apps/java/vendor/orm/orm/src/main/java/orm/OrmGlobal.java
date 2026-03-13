package orm;
public class OrmGlobal {
    private OrmGlobal() {}
    static {
        try {
            Class.forName("orm.src.SrcGlobal");
        } catch (ClassNotFoundException e) {
            throw new NoClassDefFoundError(e.getMessage());
        }
    }
}