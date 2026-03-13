package temper.std.testing;
import java.util.List;
import java.util.function.Consumer;
import java.util.Map.Entry;
import temper.core.Core;
import java.util.function.Function;
import java.util.function.Supplier;
import java.util.ArrayList;
import temper.std.testing.function.Int32PairInt32;
import java.util.AbstractMap.SimpleImmutableEntry;
public final class TestingGlobal {
    private TestingGlobal() {
    }
    public static List<Entry<String, List<String>>> processTestCases(List<Entry<String, Consumer<Test>>> testCases__69) {
        Function<Entry<String, Consumer<Test>>, Entry<String, List<String>>> fn__395 = testCase__71 -> {
            boolean t_390;
            List<String> t_393;
            boolean t_246;
            boolean t_248;
            String key__73 = testCase__71.getKey();
            Consumer<Test> fun__74 = testCase__71.getValue();
            Test test__75 = new Test();
            boolean hadBubble__76 = false;
            try {
                fun__74.accept(test__75);
            } catch (RuntimeException ignored$1) {
                hadBubble__76 = true;
            }
            List<String> messages__77 = test__75.messages();
            List<String> failures__78;
            if (test__75.isPassing()) {
                t_246 = !hadBubble__76;
            } else {
                t_246 = false;
            }
            if (t_246) {
                failures__78 = List.of();
            } else {
                if (hadBubble__76) {
                    t_390 = test__75.isFailedOnAssert();
                    t_248 = !t_390;
                } else {
                    t_248 = false;
                }
                if (t_248) {
                    List<String> allMessages__79 = new ArrayList<>(messages__77);
                    Core.listAdd(allMessages__79, "Bubble");
                    t_393 = List.copyOf(allMessages__79);
                    failures__78 = t_393;
                } else {
                    failures__78 = messages__77;
                }
            }
            return new SimpleImmutableEntry<>(key__73, failures__78);
        };
        return Core.listMapObjToObj(testCases__69, fn__395);
    }
    /**
     * escapeXml takes a string and escapes it so that it has the same meaning as an
     * XML text node or attribute value.
     */
    static String escapeXml__41(String s__103) {
        String return__40;
        int t_381;
        int t_382;
        boolean t_225;
        boolean t_226;
        boolean t_227;
        boolean t_228;
        String t_230;
        String t_231;
        StringBuilder sb__105 = new StringBuilder();
        int end__106 = s__103.length();
        int emitted__107 = 0;
        int i__108 = 0;
        while (i__108 < end__106) {
            continue_408: {
                int c__109 = s__103.codePointAt(i__108);
                if (c__109 == 38) {
                    t_231 = "&amp;";
                } else if (c__109 == 60) {
                    t_231 = "&lt;";
                } else if (c__109 == 62) {
                    t_231 = "&gt;";
                } else if (c__109 == 39) {
                    t_231 = "&#39;";
                } else if (c__109 == 34) {
                    t_231 = "&#34;";
                } else {
                    if (c__109 == 10) {
                        t_226 = true;
                    } else {
                        if (c__109 == 13) {
                            t_225 = true;
                        } else {
                            t_225 = c__109 == 9;
                        }
                        t_226 = t_225;
                    }
                    if (t_226) {
                        break continue_408;
                    } else {
                        if (c__109 < 32) {
                            t_228 = true;
                        } else {
                            if (c__109 == 65534) {
                                t_227 = true;
                            } else {
                                t_227 = c__109 == 65535;
                            }
                            t_228 = t_227;
                        }
                        if (t_228) {
                            t_230 = "[0x" + Integer.toString(c__109, 16) + "]";
                        } else {
                            break continue_408;
                        }
                        t_231 = t_230;
                    }
                }
                String esc__110 = t_231;
                Core.stringBuilderAppendBetween(sb__105, s__103, emitted__107, i__108);
                sb__105.append(esc__110);
                t_381 = Core.stringNext(s__103, i__108);
                emitted__107 = t_381;
            }
            t_382 = Core.stringNext(s__103, i__108);
            i__108 = t_382;
        }
        if (emitted__107 == 0) {
            return__40 = s__103;
        } else {
            Core.stringBuilderAppendBetween(sb__105, s__103, emitted__107, end__106);
            return__40 = sb__105.toString();
        }
        return return__40;
    }
    public static void reportTestResults(List<Entry<String, List<String>>> testResults__80, Consumer<String> writeLine__81) {
        int t_360;
        String t_363;
        String t_369;
        writeLine__81.accept("<testsuites>");
        String total__83 = Integer.toString(testResults__80.size());
        Int32PairInt32 fn__352 = (fails__85, testResult__86) -> {
            int t_203;
            if (testResult__86.getValue().isEmpty()) {
                t_203 = 0;
            } else {
                t_203 = 1;
            }
            return fails__85 + t_203;
        };
        String fails__84 = Integer.toString(Core.listedReduceObjToInt(testResults__80, 0, fn__352 :: applyAsInt));
        String totals__88 = "tests='" + total__83 + "' failures='" + fails__84 + "'";
        writeLine__81.accept("  <testsuite name='suite' " + totals__88 + " time='0.0'>");
        int i__89 = 0;
        while (true) {
            t_360 = testResults__80.size();
            if (i__89 >= t_360) {
                break;
            }
            Entry<String, List<String>> testResult__90 = Core.listGet(testResults__80, i__89);
            List<String> failureMessages__91 = testResult__90.getValue();
            t_363 = testResult__90.getKey();
            String name__92 = TestingGlobal.escapeXml__41(t_363);
            String basics__93 = "name='" + name__92 + "' classname='" + name__92 + "' time='0.0'";
            if (failureMessages__91.isEmpty()) {
                writeLine__81.accept("    <testcase " + basics__93 + " />");
            } else {
                writeLine__81.accept("    <testcase " + basics__93 + ">");
                Function<String, String> fn__351 = it__95 -> it__95;
                t_369 = Core.listJoinObj(failureMessages__91, ", ", fn__351);
                String message__94 = TestingGlobal.escapeXml__41(t_369);
                writeLine__81.accept("      <failure message='" + message__94 + "' />");
                writeLine__81.accept("    </testcase>");
            }
            i__89 = i__89 + 1;
        }
        writeLine__81.accept("  </testsuite>");
        writeLine__81.accept("</testsuites>");
    }
    public static String runTestCases(List<Entry<String, Consumer<Test>>> testCases__96) {
        StringBuilder report__98 = new StringBuilder();
        List<Entry<String, List<String>>> t_345 = TestingGlobal.processTestCases(testCases__96);
        Consumer<String> fn__343 = line__99 -> {
            report__98.append(line__99);
            report__98.append("\n");
        };
        TestingGlobal.reportTestResults(t_345, fn__343);
        return report__98.toString();
    }
    public static void runTest(Consumer<Test> testFun__100) {
        Test test__102 = new Test();
        try {
            testFun__100.accept(test__102);
        } catch (RuntimeException ignored$2) {
            Supplier<String> fn__337 = () -> "bubble during test running";
            test__102.assert_(false, fn__337);
        }
        test__102.softFailToHard();
    }
}
