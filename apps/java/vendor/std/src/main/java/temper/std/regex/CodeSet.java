package temper.std.regex;
import java.util.List;
import temper.core.Nullable;
public final class CodeSet implements RegexNode {
    public final List<CodePart> items;
    public final boolean negated;
    public static final class Builder {
        List<CodePart> items;
        public Builder items(List<CodePart> items) {
            this.items = items;
            return this;
        }
        @Nullable Boolean negated;
        public Builder negated(@Nullable Boolean negated) {
            this.negated = negated;
            return this;
        }
        public CodeSet build() {
            return new CodeSet(items, negated);
        }
    }
    public CodeSet(List<CodePart> items__214, @Nullable Boolean negated__544) {
        boolean negated__215;
        if (negated__544 == null) {
            negated__215 = false;
        } else {
            negated__215 = negated__544;
        }
        this.items = items__214;
        this.negated = negated__215;
    }
    public CodeSet(List<CodePart> items__214) {
        this(items__214, null);
    }
    public List<CodePart> getItems() {
        return this.items;
    }
    public boolean isNegated() {
        return this.negated;
    }
}
