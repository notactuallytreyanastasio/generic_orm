package temper.std.regex;
import temper.core.Nullable;
public final class Repeat implements RegexNode {
    public final RegexNode item;
    public final int min;
    public final @Nullable Integer max;
    public final boolean reluctant;
    public static final class Builder {
        RegexNode item;
        public Builder item(RegexNode item) {
            this.item = item;
            return this;
        }
        int min;
        boolean min__set;
        public Builder min(int min) {
            min__set = true;
            this.min = min;
            return this;
        }
        @Nullable Integer max;
        boolean max__set;
        public Builder max(@Nullable Integer max) {
            max__set = true;
            this.max = max;
            return this;
        }
        @Nullable Boolean reluctant;
        public Builder reluctant(@Nullable Boolean reluctant) {
            this.reluctant = reluctant;
            return this;
        }
        public Repeat build() {
            if (!min__set || !max__set || item == null) {
                StringBuilder _message = new StringBuilder("Missing required fields:");
                if (!min__set) {
                    _message.append(" min");
                }
                if (!max__set) {
                    _message.append(" max");
                }
                if (item == null) {
                    _message.append(" item");
                }
                throw new IllegalStateException(_message.toString());
            }
            return new Repeat(item, min, max, reluctant);
        }
    }
    public Repeat(RegexNode item__224, int min__225, @Nullable Integer max__226, @Nullable Boolean reluctant__546) {
        boolean reluctant__227;
        if (reluctant__546 == null) {
            reluctant__227 = false;
        } else {
            reluctant__227 = reluctant__546;
        }
        this.item = item__224;
        this.min = min__225;
        this.max = max__226;
        this.reluctant = reluctant__227;
    }
    public Repeat(RegexNode item__224, int min__225, @Nullable Integer max__226) {
        this(item__224, min__225, max__226, null);
    }
    public RegexNode getItem() {
        return this.item;
    }
    public int getMin() {
        return this.min;
    }
    public @Nullable Integer getMax() {
        return this.max;
    }
    public boolean isReluctant() {
        return this.reluctant;
    }
}
