package temper.std.regex;
/**
 * `Capture` is a [group](#groups) that remembers the matched text for later
 * access. Temper supports only named matches, with current intended syntax
 * `/(?name = ...)/`.
 */
public final class Capture implements RegexNode {
    public final String name;
    public final RegexNode item;
    public static final class Builder {
        String name;
        public Builder name(String name) {
            this.name = name;
            return this;
        }
        RegexNode item;
        public Builder item(RegexNode item) {
            this.item = item;
            return this;
        }
        public Capture build() {
            return new Capture(name, item);
        }
    }
    public Capture(String name__187, RegexNode item__188) {
        this.name = name__187;
        this.item = item__188;
    }
    public String getName() {
        return this.name;
    }
    public RegexNode getItem() {
        return this.item;
    }
}
