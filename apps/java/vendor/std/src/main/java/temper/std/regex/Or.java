package temper.std.regex;
import java.util.List;
/**
 * `Or` matches any one of multiple options, such as `/ab|cd|e&#42;/`.
 */
public final class Or implements RegexNode {
    public final List<RegexNode> items;
    public Or(List<RegexNode> items__218) {
        this.items = items__218;
    }
    public List<RegexNode> getItems() {
        return this.items;
    }
}
