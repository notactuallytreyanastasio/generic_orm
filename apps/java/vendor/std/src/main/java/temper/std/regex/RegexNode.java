package temper.std.regex;
import java.util.List;
import java.util.function.Function;
public interface RegexNode {
    default Regex compiled() {
        return new Regex(this);
    }
    default boolean found(String text__172) {
        return this.compiled().found(text__172);
    }
    default Match find(String text__175) {
        return this.compiled().find(text__175);
    }
    /**
     * Replace and split functions are also available. Both apply to all matches in
     * the string, replacing all or splitting at all.
     */
    default String replace(String text__178, Function<Match, String> format__179) {
        return this.compiled().replace(text__178, format__179);
    }
    default List<String> split(String text__182) {
        return this.compiled().split(text__182);
    }
}
