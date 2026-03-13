package temper.std.regex;
import static temper.std.regex.RegexGlobal.regexRefs__164;
import java.util.List;
import temper.core.Nullable;
import java.util.function.Function;
public final class Regex {
    public final RegexNode data;
    public Regex(RegexNode data__264) {
        RegexNode t_421 = data__264;
        this.data = t_421;
        String formatted__266 = Core.regexFormat(data__264);
        Object t_1171 = Core.regexCompiledFormatted(data__264, formatted__266);
        this.compiled = t_1171;
    }
    public boolean found(String text__268) {
        return Core.regexCompiledFound(this, this.compiled, text__268);
    }
    public Match find(String text__271, @Nullable Integer begin__556) {
        int begin__272;
        if (begin__556 == null) {
            begin__272 = 0;
        } else {
            begin__272 = begin__556;
        }
        return Core.regexCompiledFind(this, this.compiled, text__271, begin__272, regexRefs__164);
    }
    public Match find(String text__271) {
        return find(text__271, null);
    }
    public String replace(String text__275, Function<Match, String> format__276) {
        return Core.regexCompiledReplace(this, this.compiled, text__275, format__276, regexRefs__164);
    }
    public List<String> split(String text__279) {
        return Core.regexCompiledSplit(this, this.compiled, text__279, regexRefs__164);
    }
    final Object compiled;
    public RegexNode getData() {
        return this.data;
    }
}
