package temper.std.regex;
import java.util.Map;
public final class Match {
    public final Group full;
    public final Map<String, Group> groups;
    public static final class Builder {
        Group full;
        public Builder full(Group full) {
            this.full = full;
            return this;
        }
        Map<String, Group> groups;
        public Builder groups(Map<String, Group> groups) {
            this.groups = groups;
            return this;
        }
        public Match build() {
            return new Match(full, groups);
        }
    }
    public Match(Group full__242, Map<String, Group> groups__243) {
        this.full = full__242;
        this.groups = groups__243;
    }
    public Group getFull() {
        return this.full;
    }
    public Map<String, Group> getGroups() {
        return this.groups;
    }
}
