package temper.std.temporal;
import java.util.List;
import temper.core.Core;
import java.time.LocalDate;
import temper.std.json.JsonString;
import temper.std.json.JsonAdapter;
import temper.std.json.JsonProducer;
import temper.std.json.JsonSyntaxTree;
import temper.std.json.InterchangeContext;
public final class TemporalGlobal {
    private TemporalGlobal() {
    }
    static final List<Integer> daysInMonth__34;
    static final List<Integer> dayOfWeekLookupTableLeapy__35;
    static final List<Integer> dayOfWeekLookupTableNotLeapy__36;
    static void encodeToJson__90(LocalDate this__20, JsonProducer p__91) {
        String t_313 = this__20.toString();
        p__91.stringValue(t_313);
    }
    static LocalDate decodeFromJson__93(JsonSyntaxTree t__94, InterchangeContext ic__95) {
        JsonString t_190;
        t_190 = Core.cast(JsonString.class, t__94);
        return LocalDate.parse(t_190.getContent());
    }
    static JsonAdapter<LocalDate> jsonAdapter__124() {
        return new DateJsonAdapter();
    }
    static boolean isLeapYear__32(int year__41) {
        boolean return__21;
        int t_263;
        if (year__41 % 4 == 0) {
            if (year__41 % 100 != 0) {
                return__21 = true;
            } else {
                t_263 = year__41 % 400;
                return__21 = t_263 == 0;
            }
        } else {
            return__21 = false;
        }
        return return__21;
    }
    /**
     * If the decimal representation of &#92;|num&#92;| is longer than [minWidth],
     * then appends that representation.
     * Otherwise any sign for [num] followed by enough zeroes to bring the
     * whole length up to [minWidth].
     *
     * ```temper
     * // When the width is greater than the decimal's length,
     * // we pad to that width.
     * "0123" == do &#123;
     *   let sb = new StringBuilder();
     *   padTo(4, 123, sb);
     *   sb.toString()
     * &#125;
     *
     * // When the width is the same or lesser, we just use the string form.
     * "123" == do &#123;
     *   let sb = new StringBuilder();
     *   padTo(2, 123, sb);
     *   sb.toString()
     * &#125;
     *
     * // The sign is always on the left.
     * "-01" == do &#123;
     *   let sb = new StringBuilder();
     *   padTo(3, -1, sb);
     *   sb.toString()
     * &#125;
     * ```
     */
    static void padTo__33(int minWidth__43, int num__44, StringBuilder sb__45) {
        int t_346;
        int t_348;
        boolean t_257;
        String decimal__47 = Integer.toString(num__44, 10);
        int decimalIndex__48 = 0;
        int decimalEnd__49 = decimal__47.length();
        if (decimalIndex__48 < decimalEnd__49) {
            t_346 = decimal__47.codePointAt(decimalIndex__48);
            t_257 = t_346 == 45;
        } else {
            t_257 = false;
        }
        if (t_257) {
            sb__45.append("-");
            t_348 = Core.stringNext(decimal__47, decimalIndex__48);
            decimalIndex__48 = t_348;
        }
        int t_349 = Core.stringCountBetween(decimal__47, decimalIndex__48, decimalEnd__49);
        int nNeeded__50 = minWidth__43 - t_349;
        while (nNeeded__50 > 0) {
            sb__45.append("0");
            nNeeded__50 = nNeeded__50 - 1;
        }
        Core.stringBuilderAppendBetween(sb__45, decimal__47, decimalIndex__48, decimalEnd__49);
    }
    static {
        daysInMonth__34 = List.of(0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31);
        dayOfWeekLookupTableLeapy__35 = List.of(0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6);
        dayOfWeekLookupTableNotLeapy__36 = List.of(0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5);
    }
}
