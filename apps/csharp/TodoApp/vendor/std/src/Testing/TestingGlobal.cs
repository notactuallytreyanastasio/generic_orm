using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using T = System.Text;
using C = TemperLang.Core;
namespace TemperLang.Std.Testing
{
    public static class TestingGlobal
    {
        public static G::IReadOnlyList<G::KeyValuePair<string, G::IReadOnlyList<string>>> ProcessTestCases(G::IReadOnlyList<G::KeyValuePair<string, S::Action<Test>>> testCases__69)
        {
            G::KeyValuePair<string, G::IReadOnlyList<string>> fn__395(G::KeyValuePair<string, S::Action<Test>> testCase__71)
            {
                bool t___390;
                G::IReadOnlyList<string> t___393;
                bool t___246;
                bool t___248;
                string key__73 = testCase__71.Key;
                S::Action<Test> fun__74 = testCase__71.Value;
                Test test__75 = new Test();
                bool hadBubble__76 = false;
                try
                {
                    fun__74(test__75);
                }
                catch
                {
                    hadBubble__76 = true;
                }
                G::IReadOnlyList<string> messages__77 = test__75.Messages();
                G::IReadOnlyList<string> failures__78;
                if (test__75.Passing)
                {
                    t___246 = !hadBubble__76;
                }
                else
                {
                    t___246 = false;
                }
                if (t___246)
                {
                    failures__78 = C::Listed.CreateReadOnlyList<string>();
                }
                else
                {
                    if (hadBubble__76)
                    {
                        t___390 = test__75.FailedOnAssert;
                        t___248 = !t___390;
                    }
                    else
                    {
                        t___248 = false;
                    }
                    if (t___248)
                    {
                        G::IList<string> allMessages__79 = L::Enumerable.ToList(messages__77);
                        C::Listed.Add(allMessages__79, "Bubble");
                        t___393 = C::Listed.ToReadOnlyList(allMessages__79);
                        failures__78 = t___393;
                    }
                    else
                    {
                        failures__78 = messages__77;
                    }
                }
                return new G::KeyValuePair<string, G::IReadOnlyList<string>>(key__73, failures__78);
            }
            return C::Listed.ToReadOnlyList(L::Enumerable.Select(testCases__69, (S::Func<G::KeyValuePair<string, S::Action<Test>>, G::KeyValuePair<string, G::IReadOnlyList<string>>>) fn__395));
        }
        internal static string escapeXml__41(string s__103)
        {
            string return__40;
            int t___381;
            int t___382;
            bool t___225;
            bool t___226;
            bool t___227;
            bool t___228;
            string t___230;
            string t___231;
            T::StringBuilder sb__105 = new T::StringBuilder();
            int end__106 = s__103.Length;
            int emitted__107 = 0;
            int i__108 = 0;
            while (i__108 < end__106)
            {
                {
                    {
                        int c__109 = C::StringUtil.Get(s__103, i__108);
                        if (c__109 == 38)
                        {
                            t___231 = "&amp;";
                        }
                        else if (c__109 == 60)
                        {
                            t___231 = "&lt;";
                        }
                        else if (c__109 == 62)
                        {
                            t___231 = "&gt;";
                        }
                        else if (c__109 == 39)
                        {
                            t___231 = "&#39;";
                        }
                        else if (c__109 == 34)
                        {
                            t___231 = "&#34;";
                        }
                        else
                        {
                            if (c__109 == 10)
                            {
                                t___226 = true;
                            }
                            else
                            {
                                if (c__109 == 13)
                                {
                                    t___225 = true;
                                }
                                else
                                {
                                    t___225 = c__109 == 9;
                                }
                                t___226 = t___225;
                            }
                            if (t___226) goto continue___408;
                            else
                            {
                                if (c__109 < 32)
                                {
                                    t___228 = true;
                                }
                                else
                                {
                                    if (c__109 == 65534)
                                    {
                                        t___227 = true;
                                    }
                                    else
                                    {
                                        t___227 = c__109 == 65535;
                                    }
                                    t___228 = t___227;
                                }
                                if (t___228)
                                {
                                    t___230 = "[0x" + S::Convert.ToString(c__109, 16) + "]";
                                }
                                else goto continue___408;
                                t___231 = t___230;
                            }
                        }
                        string esc__110 = t___231;
                        C::StringUtil.AppendBetween(sb__105, s__103, emitted__107, i__108);
                        sb__105.Append(esc__110);
                        t___381 = C::StringUtil.Next(s__103, i__108);
                        emitted__107 = t___381;
                    }
                    continue___408:
                    {
                    }
                }
                t___382 = C::StringUtil.Next(s__103, i__108);
                i__108 = t___382;
            }
            if (emitted__107 == 0)
            {
                return__40 = s__103;
            }
            else
            {
                C::StringUtil.AppendBetween(sb__105, s__103, emitted__107, end__106);
                return__40 = sb__105.ToString();
            }
            return return__40;
        }
        public static void ReportTestResults(G::IReadOnlyList<G::KeyValuePair<string, G::IReadOnlyList<string>>> testResults__80, S::Action<string> writeLine__81)
        {
            int t___360;
            string t___363;
            string t___369;
            writeLine__81("<testsuites>");
            string total__83 = S::Convert.ToString(testResults__80.Count);
            int fn__352(int fails__85, G::KeyValuePair<string, G::IReadOnlyList<string>> testResult__86)
            {
                int t___203;
                if (testResult__86.Value.Count == 0)
                {
                    t___203 = 0;
                }
                else
                {
                    t___203 = 1;
                }
                return fails__85 + t___203;
            }
            string fails__84 = S::Convert.ToString(L::Enumerable.Aggregate(testResults__80, 0, (S::Func<int, G::KeyValuePair<string, G::IReadOnlyList<string>>, int>) fn__352));
            string totals__88 = "tests='" + total__83 + "' failures='" + fails__84 + "'";
            writeLine__81("  <testsuite name='suite' " + totals__88 + " time='0.0'>");
            int i__89 = 0;
            while (true)
            {
                t___360 = testResults__80.Count;
                if (!(i__89 < t___360)) break;
                G::KeyValuePair<string, G::IReadOnlyList<string>> testResult__90 = testResults__80[i__89];
                G::IReadOnlyList<string> failureMessages__91 = testResult__90.Value;
                t___363 = testResult__90.Key;
                string name__92 = escapeXml__41(t___363);
                string basics__93 = "name='" + name__92 + "' classname='" + name__92 + "' time='0.0'";
                if (failureMessages__91.Count == 0) writeLine__81("    <testcase " + basics__93 + " />");
                else
                {
                    writeLine__81("    <testcase " + basics__93 + ">");
                    string fn__351(string it__95)
                    {
                        return it__95;
                    }
                    t___369 = C::Listed.Join(failureMessages__91, ", ", (S::Func<string, string>) fn__351);
                    string message__94 = escapeXml__41(t___369);
                    writeLine__81("      <failure message='" + message__94 + "' />");
                    writeLine__81("    </testcase>");
                }
                i__89 = i__89 + 1;
            }
            writeLine__81("  </testsuite>");
            writeLine__81("</testsuites>");
        }
        public static string RunTestCases(G::IReadOnlyList<G::KeyValuePair<string, S::Action<Test>>> testCases__96)
        {
            T::StringBuilder report__98 = new T::StringBuilder();
            G::IReadOnlyList<G::KeyValuePair<string, G::IReadOnlyList<string>>> t___345 = ProcessTestCases(testCases__96);
            void fn__343(string line__99)
            {
                report__98.Append(line__99);
                report__98.Append("\n");
            }
            ReportTestResults(t___345, (S::Action<string>) fn__343);
            return report__98.ToString();
        }
        public static void RunTest(S::Action<Test> testFun__100)
        {
            Test test__102 = new Test();
            try
            {
                testFun__100(test__102);
            }
            catch
            {
                string fn__337()
                {
                    return "bubble during test running";
                }
                test__102.Assert(false, (S::Func<string>) fn__337);
            }
            test__102.SoftFailToHard();
        }
    }
}
