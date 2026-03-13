package temper.std.regex;
import temper.core.Nullable;
import java.util.List;
import java.util.Map;
import temper.core.Core;
import java.util.AbstractMap.SimpleImmutableEntry;
final class RegexRefs {
    public final CodePoints codePoints;
    public final Group group;
    public final Match match;
    public final Or orObject;
    public static final class Builder {
        @Nullable CodePoints codePoints;
        public Builder codePoints(@Nullable CodePoints codePoints) {
            this.codePoints = codePoints;
            return this;
        }
        @Nullable Group group;
        public Builder group(@Nullable Group group) {
            this.group = group;
            return this;
        }
        @Nullable Match match;
        public Builder match(@Nullable Match match) {
            this.match = match;
            return this;
        }
        @Nullable Or orObject;
        public Builder orObject(@Nullable Or orObject) {
            this.orObject = orObject;
            return this;
        }
        public RegexRefs build() {
            return new RegexRefs(codePoints, group, match, orObject);
        }
    }
    public RegexRefs(@Nullable CodePoints codePoints__548, @Nullable Group group__550, @Nullable Match match__552, @Nullable Or orObject__554) {
        CodePoints t_1292;
        Group t_1293;
        Map<String, Group> t_1295;
        Match t_1296;
        Or t_1297;
        CodePoints codePoints__258;
        if (codePoints__548 == null) {
            t_1292 = new CodePoints("");
            codePoints__258 = t_1292;
        } else {
            codePoints__258 = codePoints__548;
        }
        Group group__259;
        if (group__550 == null) {
            t_1293 = new Group("", "", 0, 0);
            group__259 = t_1293;
        } else {
            group__259 = group__550;
        }
        Match match__260;
        if (match__552 == null) {
            t_1295 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("", group__259)));
            t_1296 = new Match(group__259, t_1295);
            match__260 = t_1296;
        } else {
            match__260 = match__552;
        }
        Or orObject__261;
        if (orObject__554 == null) {
            t_1297 = new Or(List.of());
            orObject__261 = t_1297;
        } else {
            orObject__261 = orObject__554;
        }
        this.codePoints = codePoints__258;
        this.group = group__259;
        this.match = match__260;
        this.orObject = orObject__261;
    }
    public RegexRefs(@Nullable CodePoints codePoints__548, @Nullable Group group__550, @Nullable Match match__552) {
        this(codePoints__548, group__550, match__552, null);
    }
    public RegexRefs(@Nullable CodePoints codePoints__548, @Nullable Group group__550) {
        this(codePoints__548, group__550, null, null);
    }
    public RegexRefs(@Nullable CodePoints codePoints__548) {
        this(codePoints__548, null, null, null);
    }
    public RegexRefs() {
        this(null, null, null, null);
    }
    public CodePoints getCodePoints() {
        return this.codePoints;
    }
    public Group getGroup() {
        return this.group;
    }
    public Match getMatch() {
        return this.match;
    }
    public Or getOrObject() {
        return this.orObject;
    }
}
