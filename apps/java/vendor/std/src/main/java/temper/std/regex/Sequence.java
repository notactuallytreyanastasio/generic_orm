package temper.std.regex;
import java.util.List;
/**
 * `Sequence` strings along multiple other regexes in order.
 */
public final class Sequence implements RegexNode {
    public final List<RegexNode> items;
    public Sequence(List<RegexNode> items__238) {
        this.items = items__238;
    }
    public List<RegexNode> getItems() {
        return this.items;
    }
}
