using G = System.Collections.Generic;
using T = System.Text;
using C = TemperLang.Core;
namespace TemperLang.Std.Json
{
    public static class JsonGlobal
    {
        internal static int parseJsonValue__356(string sourceText__670, int i__671, IJsonProducer out__672)
        {
            int return__302;
            int t___2201;
            int t___2204;
            bool t___1412;
            {
                {
                    t___2201 = skipJsonSpaces__355(sourceText__670, i__671);
                    i__671 = t___2201;
                    if (!C::StringUtil.HasIndex(sourceText__670, i__671))
                    {
                        expectedTokenError__353(sourceText__670, i__671, out__672, "JSON value");
                        return__302 = -1;
                        goto fn__673;
                    }
                    t___2204 = C::StringUtil.Get(sourceText__670, i__671);
                    if (t___2204 == 123)
                    {
                        return__302 = parseJsonObject__357(sourceText__670, i__671, out__672);
                    }
                    else if (t___2204 == 91)
                    {
                        return__302 = parseJsonArray__358(sourceText__670, i__671, out__672);
                    }
                    else if (t___2204 == 34)
                    {
                        return__302 = parseJsonString__359(sourceText__670, i__671, out__672);
                    }
                    else
                    {
                        if (t___2204 == 116)
                        {
                            t___1412 = true;
                        }
                        else
                        {
                            t___1412 = t___2204 == 102;
                        }
                        if (t___1412)
                        {
                            return__302 = parseJsonBoolean__362(sourceText__670, i__671, out__672);
                        }
                        else if (t___2204 == 110)
                        {
                            return__302 = parseJsonNull__363(sourceText__670, i__671, out__672);
                        }
                        else
                        {
                            return__302 = parseJsonNumber__365(sourceText__670, i__671, out__672);
                        }
                    }
                }
                fn__673:
                {
                }
            }
            return return__302;
        }
        internal static G::IReadOnlyList<string> hexDigits__373;
        internal static void encodeHex4__352(int cp__580, T::StringBuilder buffer__581)
        {
            int b0__583 = C::Core.DivSafe(cp__580, 4096) & 15;
            int b1__584 = C::Core.DivSafe(cp__580, 256) & 15;
            int b2__585 = C::Core.DivSafe(cp__580, 16) & 15;
            int b3__586 = cp__580 & 15;
            string t___2482 = hexDigits__373[b0__583];
            buffer__581.Append(t___2482);
            string t___2484 = hexDigits__373[b1__584];
            buffer__581.Append(t___2484);
            string t___2486 = hexDigits__373[b2__585];
            buffer__581.Append(t___2486);
            string t___2488 = hexDigits__373[b3__586];
            buffer__581.Append(t___2488);
        }
        internal static void encodeJsonString__351(string x__572, T::StringBuilder buffer__573)
        {
            bool t___1745;
            bool t___1746;
            string t___1747;
            string t___1748;
            buffer__573.Append("\u0022");
            int i__575 = 0;
            int emitted__576 = i__575;
            while (true)
            {
                if (!C::StringUtil.HasIndex(x__572, i__575)) break;
                int cp__577 = C::StringUtil.Get(x__572, i__575);
                if (cp__577 == 8)
                {
                    t___1748 = "\\b";
                }
                else if (cp__577 == 9)
                {
                    t___1748 = "\\t";
                }
                else if (cp__577 == 10)
                {
                    t___1748 = "\\n";
                }
                else if (cp__577 == 12)
                {
                    t___1748 = "\\f";
                }
                else if (cp__577 == 13)
                {
                    t___1748 = "\\r";
                }
                else if (cp__577 == 34)
                {
                    t___1748 = "\\\u0022";
                }
                else if (cp__577 == 92)
                {
                    t___1748 = "\\\\";
                }
                else
                {
                    if (cp__577 < 32)
                    {
                        t___1746 = true;
                    }
                    else
                    {
                        if (55296 <= cp__577)
                        {
                            t___1745 = cp__577 <= 57343;
                        }
                        else
                        {
                            t___1745 = false;
                        }
                        t___1746 = t___1745;
                    }
                    if (t___1746)
                    {
                        t___1747 = "\\u";
                    }
                    else
                    {
                        t___1747 = "";
                    }
                    t___1748 = t___1747;
                }
                string replacement__578 = t___1748;
                int nextI__579 = C::StringUtil.Next(x__572, i__575);
                if (replacement__578 != "")
                {
                    C::StringUtil.AppendBetween(buffer__573, x__572, emitted__576, i__575);
                    buffer__573.Append(replacement__578);
                    if (replacement__578 == "\\u") encodeHex4__352(cp__577, buffer__573);
                    emitted__576 = nextI__579;
                }
                i__575 = nextI__579;
            }
            C::StringUtil.AppendBetween(buffer__573, x__572, emitted__576, i__575);
            buffer__573.Append("\u0022");
        }
        internal static void storeJsonError__354(IJsonProducer out__659, string explanation__660)
        {
            IJsonParseErrorReceiver ? t___2361 = out__659.ParseErrorReceiver;
            if (!(t___2361 == null))(t___2361!).ExplainJsonError(explanation__660);
        }
        internal static void expectedTokenError__353(string sourceText__653, int i__654, IJsonProducer out__655, string shortExplanation__656)
        {
            int t___2358;
            string t___2359;
            string gotten__658;
            if (C::StringUtil.HasIndex(sourceText__653, i__654))
            {
                t___2358 = sourceText__653.Length;
                t___2359 = C::StringUtil.Slice(sourceText__653, i__654, t___2358);
                gotten__658 = "`" + t___2359 + "`";
            }
            else
            {
                gotten__658 = "end-of-file";
            }
            storeJsonError__354(out__655, "Expected " + shortExplanation__656 + ", but got " + gotten__658);
        }
        internal static int skipJsonSpaces__355(string sourceText__667, int i__668)
        {
            int t___2355;
            int t___2356;
            bool t___1616;
            bool t___1617;
            bool t___1618;
            while (true)
            {
                if (!C::StringUtil.HasIndex(sourceText__667, i__668)) break;
                t___2355 = C::StringUtil.Get(sourceText__667, i__668);
                if (t___2355 == 9)
                {
                    t___1618 = true;
                }
                else
                {
                    if (t___2355 == 10)
                    {
                        t___1617 = true;
                    }
                    else
                    {
                        if (t___2355 == 13)
                        {
                            t___1616 = true;
                        }
                        else
                        {
                            t___1616 = t___2355 == 32;
                        }
                        t___1617 = t___1616;
                    }
                    t___1618 = t___1617;
                }
                if (!t___1618) break;
                t___2356 = C::StringUtil.Next(sourceText__667, i__668);
                i__668 = t___2356;
            }
            return i__668;
        }
        internal static int decodeHexUnsigned__361(string sourceText__708, int start__709, int limit__710)
        {
            int return__307;
            int t___2353;
            bool t___1609;
            bool t___1610;
            bool t___1611;
            int t___1612;
            {
                {
                    int n__712 = 0;
                    int i__713 = start__709;
                    while (true)
                    {
                        if (!(i__713.CompareTo(limit__710) < 0)) break;
                        int cp__714 = C::StringUtil.Get(sourceText__708, i__713);
                        if (48 <= cp__714)
                        {
                            t___1609 = cp__714 <= 48;
                        }
                        else
                        {
                            t___1609 = false;
                        }
                        if (t___1609)
                        {
                            t___1612 = cp__714 - 48;
                        }
                        else
                        {
                            if (65 <= cp__714)
                            {
                                t___1610 = cp__714 <= 70;
                            }
                            else
                            {
                                t___1610 = false;
                            }
                            if (t___1610)
                            {
                                t___1612 = cp__714 - 65 + 10;
                            }
                            else
                            {
                                if (97 <= cp__714)
                                {
                                    t___1611 = cp__714 <= 102;
                                }
                                else
                                {
                                    t___1611 = false;
                                }
                                if (t___1611)
                                {
                                    t___1612 = cp__714 - 97 + 10;
                                }
                                else
                                {
                                    return__307 = -1;
                                    goto fn__711;
                                }
                            }
                        }
                        int digit__715 = t___1612;
                        n__712 = n__712 * 16 + digit__715;
                        t___2353 = C::StringUtil.Next(sourceText__708, i__713);
                        i__713 = t___2353;
                    }
                    return__307 = n__712;
                }
                fn__711:
                {
                }
            }
            return return__307;
        }
        internal static int parseJsonStringTo__360(string sourceText__692, int i__693, T::StringBuilder sb__694, IJsonProducer errOut__695)
        {
            int return__306;
            int t___2326;
            int t___2328;
            int t___2331;
            int t___2336;
            int t___2338;
            int t___2339;
            int t___2340;
            int t___2341;
            int t___2342;
            int t___2347;
            int t___2350;
            bool t___1570;
            bool t___1579;
            bool t___1580;
            int t___1588;
            int t___1589;
            int t___1591;
            int t___1593;
            bool t___1594;
            bool t___1595;
            bool t___1597;
            bool t___1601;
            {
                {
                    if (!C::StringUtil.HasIndex(sourceText__692, i__693))
                    {
                        t___1570 = true;
                    }
                    else
                    {
                        t___2326 = C::StringUtil.Get(sourceText__692, i__693);
                        t___1570 = t___2326 != 34;
                    }
                    if (t___1570)
                    {
                        expectedTokenError__353(sourceText__692, i__693, errOut__695, "\u0022");
                        return__306 = -1;
                        goto fn__696;
                    }
                    t___2328 = C::StringUtil.Next(sourceText__692, i__693);
                    i__693 = t___2328;
                    int leadSurrogate__697 = -1;
                    int consumed__698 = i__693;
                    while (true)
                    {
                        if (!C::StringUtil.HasIndex(sourceText__692, i__693)) break;
                        int cp__699 = C::StringUtil.Get(sourceText__692, i__693);
                        if (cp__699 == 34) break;
                        t___2331 = C::StringUtil.Next(sourceText__692, i__693);
                        int iNext__700 = t___2331;
                        int end__701 = sourceText__692.Length;
                        bool needToFlush__702 = false;
                        if (cp__699 != 92)
                        {
                            t___1593 = cp__699;
                        }
                        else
                        {
                            needToFlush__702 = true;
                            if (!C::StringUtil.HasIndex(sourceText__692, iNext__700))
                            {
                                expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "escape sequence");
                                return__306 = -1;
                                goto fn__696;
                            }
                            int esc0__704 = C::StringUtil.Get(sourceText__692, iNext__700);
                            t___2336 = C::StringUtil.Next(sourceText__692, iNext__700);
                            iNext__700 = t___2336;
                            if (esc0__704 == 34)
                            {
                                t___1580 = true;
                            }
                            else
                            {
                                if (esc0__704 == 92)
                                {
                                    t___1579 = true;
                                }
                                else
                                {
                                    t___1579 = esc0__704 == 47;
                                }
                                t___1580 = t___1579;
                            }
                            if (t___1580)
                            {
                                t___1591 = esc0__704;
                            }
                            else if (esc0__704 == 98)
                            {
                                t___1591 = 8;
                            }
                            else if (esc0__704 == 102)
                            {
                                t___1591 = 12;
                            }
                            else if (esc0__704 == 110)
                            {
                                t___1591 = 10;
                            }
                            else if (esc0__704 == 114)
                            {
                                t___1591 = 13;
                            }
                            else if (esc0__704 == 116)
                            {
                                t___1591 = 9;
                            }
                            else if (esc0__704 == 117)
                            {
                                if (C::StringUtil.HasAtLeast(sourceText__692, iNext__700, end__701, 4))
                                {
                                    int startHex__706 = iNext__700;
                                    t___2338 = C::StringUtil.Next(sourceText__692, iNext__700);
                                    iNext__700 = t___2338;
                                    t___2339 = C::StringUtil.Next(sourceText__692, iNext__700);
                                    iNext__700 = t___2339;
                                    t___2340 = C::StringUtil.Next(sourceText__692, iNext__700);
                                    iNext__700 = t___2340;
                                    t___2341 = C::StringUtil.Next(sourceText__692, iNext__700);
                                    iNext__700 = t___2341;
                                    t___2342 = decodeHexUnsigned__361(sourceText__692, startHex__706, iNext__700);
                                    t___1588 = t___2342;
                                }
                                else
                                {
                                    t___1588 = -1;
                                }
                                int hex__705 = t___1588;
                                if (hex__705 < 0)
                                {
                                    expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "four hex digits");
                                    return__306 = -1;
                                    goto fn__696;
                                }
                                t___1589 = hex__705;
                                t___1591 = t___1589;
                            }
                            else
                            {
                                expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "escape sequence");
                                return__306 = -1;
                                goto fn__696;
                            }
                            t___1593 = t___1591;
                        }
                        int decodedCp__703 = t___1593;
                        if (leadSurrogate__697 >= 0)
                        {
                            needToFlush__702 = true;
                            int lead__707 = leadSurrogate__697;
                            if (56320 <= decodedCp__703)
                            {
                                t___1594 = decodedCp__703 <= 57343;
                            }
                            else
                            {
                                t___1594 = false;
                            }
                            if (t___1594)
                            {
                                leadSurrogate__697 = -1;
                                decodedCp__703 = 65536 + ((lead__707 - 55296) * 1024 | decodedCp__703 - 56320);
                            }
                        }
                        else
                        {
                            if (55296 <= decodedCp__703)
                            {
                                t___1595 = decodedCp__703 <= 56319;
                            }
                            else
                            {
                                t___1595 = false;
                            }
                            if (t___1595)
                            {
                                needToFlush__702 = true;
                            }
                        }
                        if (needToFlush__702)
                        {
                            C::StringUtil.AppendBetween(sb__694, sourceText__692, consumed__698, i__693);
                            if (leadSurrogate__697 >= 0) C::StringUtil.AppendCodePoint(sb__694, leadSurrogate__697);
                            if (55296 <= decodedCp__703)
                            {
                                t___1597 = decodedCp__703 <= 56319;
                            }
                            else
                            {
                                t___1597 = false;
                            }
                            if (t___1597)
                            {
                                leadSurrogate__697 = decodedCp__703;
                            }
                            else
                            {
                                leadSurrogate__697 = -1;
                                C::StringUtil.AppendCodePoint(sb__694, decodedCp__703);
                            }
                            consumed__698 = iNext__700;
                        }
                        i__693 = iNext__700;
                    }
                    if (!C::StringUtil.HasIndex(sourceText__692, i__693))
                    {
                        t___1601 = true;
                    }
                    else
                    {
                        t___2347 = C::StringUtil.Get(sourceText__692, i__693);
                        t___1601 = t___2347 != 34;
                    }
                    if (t___1601)
                    {
                        expectedTokenError__353(sourceText__692, i__693, errOut__695, "\u0022");
                        return__306 = -1;
                    }
                    else
                    {
                        if (leadSurrogate__697 >= 0) C::StringUtil.AppendCodePoint(sb__694, leadSurrogate__697);
                        else C::StringUtil.AppendBetween(sb__694, sourceText__692, consumed__698, i__693);
                        t___2350 = C::StringUtil.Next(sourceText__692, i__693);
                        i__693 = t___2350;
                        return__306 = i__693;
                    }
                }
                fn__696:
                {
                }
            }
            return return__306;
        }
        internal static int parseJsonObject__357(string sourceText__674, int i__675, IJsonProducer out__676)
        {
            int return__303;
            int t___2296;
            int t___2299;
            int t___2300;
            int t___2302;
            string t___2306;
            int t___2308;
            int t___2310;
            int t___2311;
            int t___2315;
            int t___2317;
            int t___2318;
            int t___2319;
            int t___2321;
            bool t___1533;
            bool t___1539;
            int t___1545;
            int t___1547;
            bool t___1551;
            int t___1555;
            bool t___1560;
            bool t___1565;
            {
                {
                    if (!C::StringUtil.HasIndex(sourceText__674, i__675))
                    {
                        t___1533 = true;
                    }
                    else
                    {
                        t___2296 = C::StringUtil.Get(sourceText__674, i__675);
                        t___1533 = t___2296 != 123;
                    }
                    if (t___1533)
                    {
                        expectedTokenError__353(sourceText__674, i__675, out__676, "'{'");
                        return__303 = -1;
                        goto fn__677;
                    }
                    out__676.StartObject();
                    t___2299 = C::StringUtil.Next(sourceText__674, i__675);
                    t___2300 = skipJsonSpaces__355(sourceText__674, t___2299);
                    i__675 = t___2300;
                    if (C::StringUtil.HasIndex(sourceText__674, i__675))
                    {
                        t___2302 = C::StringUtil.Get(sourceText__674, i__675);
                        t___1539 = t___2302 != 125;
                    }
                    else
                    {
                        t___1539 = false;
                    }
                    if (t___1539) while (true)
                    {
                        T::StringBuilder keyBuffer__678 = new T::StringBuilder();
                        int afterKey__679 = parseJsonStringTo__360(sourceText__674, i__675, keyBuffer__678, out__676);
                        if (!(afterKey__679 >= 0))
                        {
                            return__303 = -1;
                            goto fn__677;
                        }
                        t___2306 = keyBuffer__678.ToString();
                        out__676.ObjectKey(t___2306);
                        t___1545 = C::StringUtil.RequireStringIndex(afterKey__679);
                        t___1547 = t___1545;
                        t___2308 = skipJsonSpaces__355(sourceText__674, t___1547);
                        i__675 = t___2308;
                        if (C::StringUtil.HasIndex(sourceText__674, i__675))
                        {
                            t___2310 = C::StringUtil.Get(sourceText__674, i__675);
                            t___1551 = t___2310 == 58;
                        }
                        else
                        {
                            t___1551 = false;
                        }
                        if (t___1551)
                        {
                            t___2311 = C::StringUtil.Next(sourceText__674, i__675);
                            i__675 = t___2311;
                            int afterPropertyValue__680 = parseJsonValue__356(sourceText__674, i__675, out__676);
                            if (!(afterPropertyValue__680 >= 0))
                            {
                                return__303 = -1;
                                goto fn__677;
                            }
                            t___1555 = C::StringUtil.RequireStringIndex(afterPropertyValue__680);
                            i__675 = t___1555;
                        }
                        else
                        {
                            expectedTokenError__353(sourceText__674, i__675, out__676, "':'");
                            return__303 = -1;
                            goto fn__677;
                        }
                        t___2315 = skipJsonSpaces__355(sourceText__674, i__675);
                        i__675 = t___2315;
                        if (C::StringUtil.HasIndex(sourceText__674, i__675))
                        {
                            t___2317 = C::StringUtil.Get(sourceText__674, i__675);
                            t___1560 = t___2317 == 44;
                        }
                        else
                        {
                            t___1560 = false;
                        }
                        if (t___1560)
                        {
                            t___2318 = C::StringUtil.Next(sourceText__674, i__675);
                            t___2319 = skipJsonSpaces__355(sourceText__674, t___2318);
                            i__675 = t___2319;
                        }
                        else break;
                    }
                    if (C::StringUtil.HasIndex(sourceText__674, i__675))
                    {
                        t___2321 = C::StringUtil.Get(sourceText__674, i__675);
                        t___1565 = t___2321 == 125;
                    }
                    else
                    {
                        t___1565 = false;
                    }
                    if (t___1565)
                    {
                        out__676.EndObject();
                        return__303 = C::StringUtil.Next(sourceText__674, i__675);
                    }
                    else
                    {
                        expectedTokenError__353(sourceText__674, i__675, out__676, "'}'");
                        return__303 = -1;
                    }
                }
                fn__677:
                {
                }
            }
            return return__303;
        }
        internal static int parseJsonArray__358(string sourceText__681, int i__682, IJsonProducer out__683)
        {
            int return__304;
            int t___2276;
            int t___2279;
            int t___2280;
            int t___2282;
            int t___2285;
            int t___2287;
            int t___2288;
            int t___2289;
            int t___2291;
            bool t___1509;
            bool t___1515;
            int t___1518;
            bool t___1523;
            bool t___1528;
            {
                {
                    if (!C::StringUtil.HasIndex(sourceText__681, i__682))
                    {
                        t___1509 = true;
                    }
                    else
                    {
                        t___2276 = C::StringUtil.Get(sourceText__681, i__682);
                        t___1509 = t___2276 != 91;
                    }
                    if (t___1509)
                    {
                        expectedTokenError__353(sourceText__681, i__682, out__683, "'['");
                        return__304 = -1;
                        goto fn__684;
                    }
                    out__683.StartArray();
                    t___2279 = C::StringUtil.Next(sourceText__681, i__682);
                    t___2280 = skipJsonSpaces__355(sourceText__681, t___2279);
                    i__682 = t___2280;
                    if (C::StringUtil.HasIndex(sourceText__681, i__682))
                    {
                        t___2282 = C::StringUtil.Get(sourceText__681, i__682);
                        t___1515 = t___2282 != 93;
                    }
                    else
                    {
                        t___1515 = false;
                    }
                    if (t___1515) while (true)
                    {
                        int afterElementValue__685 = parseJsonValue__356(sourceText__681, i__682, out__683);
                        if (!(afterElementValue__685 >= 0))
                        {
                            return__304 = -1;
                            goto fn__684;
                        }
                        t___1518 = C::StringUtil.RequireStringIndex(afterElementValue__685);
                        i__682 = t___1518;
                        t___2285 = skipJsonSpaces__355(sourceText__681, i__682);
                        i__682 = t___2285;
                        if (C::StringUtil.HasIndex(sourceText__681, i__682))
                        {
                            t___2287 = C::StringUtil.Get(sourceText__681, i__682);
                            t___1523 = t___2287 == 44;
                        }
                        else
                        {
                            t___1523 = false;
                        }
                        if (t___1523)
                        {
                            t___2288 = C::StringUtil.Next(sourceText__681, i__682);
                            t___2289 = skipJsonSpaces__355(sourceText__681, t___2288);
                            i__682 = t___2289;
                        }
                        else break;
                    }
                    if (C::StringUtil.HasIndex(sourceText__681, i__682))
                    {
                        t___2291 = C::StringUtil.Get(sourceText__681, i__682);
                        t___1528 = t___2291 == 93;
                    }
                    else
                    {
                        t___1528 = false;
                    }
                    if (t___1528)
                    {
                        out__683.EndArray();
                        return__304 = C::StringUtil.Next(sourceText__681, i__682);
                    }
                    else
                    {
                        expectedTokenError__353(sourceText__681, i__682, out__683, "']'");
                        return__304 = -1;
                    }
                }
                fn__684:
                {
                }
            }
            return return__304;
        }
        internal static int parseJsonString__359(string sourceText__686, int i__687, IJsonProducer out__688)
        {
            string t___2273;
            T::StringBuilder sb__690 = new T::StringBuilder();
            int after__691 = parseJsonStringTo__360(sourceText__686, i__687, sb__690, out__688);
            if (after__691 >= 0)
            {
                t___2273 = sb__690.ToString();
                out__688.StringValue(t___2273);
            }
            return after__691;
        }
        internal static int afterSubstring__364(string string__730, int inString__731, string substring__732)
        {
            int return__310;
            int t___2268;
            int t___2269;
            {
                {
                    int i__734 = inString__731;
                    int j__735 = 0;
                    while (true)
                    {
                        if (!C::StringUtil.HasIndex(substring__732, j__735)) break;
                        if (!C::StringUtil.HasIndex(string__730, i__734))
                        {
                            return__310 = -1;
                            goto fn__733;
                        }
                        if (C::StringUtil.Get(string__730, i__734) != C::StringUtil.Get(substring__732, j__735))
                        {
                            return__310 = -1;
                            goto fn__733;
                        }
                        t___2268 = C::StringUtil.Next(string__730, i__734);
                        i__734 = t___2268;
                        t___2269 = C::StringUtil.Next(substring__732, j__735);
                        j__735 = t___2269;
                    }
                    return__310 = i__734;
                }
                fn__733:
                {
                }
            }
            return return__310;
        }
        internal static int parseJsonBoolean__362(string sourceText__716, int i__717, IJsonProducer out__718)
        {
            int return__308;
            int t___2257;
            {
                {
                    int ch0__720;
                    if (C::StringUtil.HasIndex(sourceText__716, i__717))
                    {
                        t___2257 = C::StringUtil.Get(sourceText__716, i__717);
                        ch0__720 = t___2257;
                    }
                    else
                    {
                        ch0__720 = 0;
                    }
                    int end__721 = sourceText__716.Length;
                    string ? keyword__722;
                    int n__723;
                    if (ch0__720 == 102)
                    {
                        keyword__722 = "false";
                        n__723 = 5;
                    }
                    else if (ch0__720 == 116)
                    {
                        keyword__722 = "true";
                        n__723 = 4;
                    }
                    else
                    {
                        keyword__722 = null;
                        n__723 = 0;
                    }
                    if (!(keyword__722 == null))
                    {
                        string keyword___958 = keyword__722!;
                        if (C::StringUtil.HasAtLeast(sourceText__716, i__717, end__721, n__723))
                        {
                            int after__724 = afterSubstring__364(sourceText__716, i__717, keyword___958);
                            if (after__724 >= 0)
                            {
                                return__308 = C::StringUtil.RequireStringIndex(after__724);
                                out__718.BooleanValue(n__723 == 4);
                                goto fn__719;
                            }
                        }
                    }
                    expectedTokenError__353(sourceText__716, i__717, out__718, "`false` or `true`");
                    return__308 = -1;
                }
                fn__719:
                {
                }
            }
            return return__308;
        }
        internal static int parseJsonNull__363(string sourceText__725, int i__726, IJsonProducer out__727)
        {
            int return__309;
            {
                {
                    int after__729 = afterSubstring__364(sourceText__725, i__726, "null");
                    if (after__729 >= 0)
                    {
                        return__309 = C::StringUtil.RequireStringIndex(after__729);
                        out__727.NullValue();
                        goto fn__728;
                    }
                    expectedTokenError__353(sourceText__725, i__726, out__727, "`null`");
                    return__309 = -1;
                }
                fn__728:
                {
                }
            }
            return return__309;
        }
        internal static int parseJsonNumber__365(string sourceText__736, int i__737, IJsonProducer out__738)
        {
            int return__311;
            int t___2212;
            int t___2213;
            int t___2215;
            int t___2217;
            double t___2218;
            long t___2219;
            int t___2222;
            double t___2223;
            long t___2224;
            int t___2228;
            int t___2229;
            int t___2232;
            double t___2233;
            int t___2236;
            int t___2237;
            int t___2241;
            int t___2244;
            int t___2246;
            bool t___1420;
            bool t___1425;
            bool t___1426;
            bool t___1434;
            double t___1437;
            long t___1439;
            bool t___1442;
            bool t___1443;
            bool t___1446;
            bool t___1450;
            double t___1453;
            bool t___1456;
            bool t___1460;
            bool t___1464;
            bool t___1466;
            bool t___1467;
            bool t___1469;
            bool t___1472;
            double t___1473;
            bool t___1474;
            bool t___1475;
            {
                {
                    bool isNegative__740 = false;
                    int startOfNumber__741 = i__737;
                    if (C::StringUtil.HasIndex(sourceText__736, i__737))
                    {
                        t___2212 = C::StringUtil.Get(sourceText__736, i__737);
                        t___1420 = t___2212 == 45;
                    }
                    else
                    {
                        t___1420 = false;
                    }
                    if (t___1420)
                    {
                        isNegative__740 = true;
                        t___2213 = C::StringUtil.Next(sourceText__736, i__737);
                        i__737 = t___2213;
                    }
                    int digit0__742;
                    if (C::StringUtil.HasIndex(sourceText__736, i__737))
                    {
                        t___2215 = C::StringUtil.Get(sourceText__736, i__737);
                        digit0__742 = t___2215;
                    }
                    else
                    {
                        digit0__742 = -1;
                    }
                    if (digit0__742 < 48)
                    {
                        t___1425 = true;
                    }
                    else
                    {
                        t___1425 = 57 < digit0__742;
                    }
                    if (t___1425)
                    {
                        string error__743;
                        if (!isNegative__740)
                        {
                            t___1426 = digit0__742 != 46;
                        }
                        else
                        {
                            t___1426 = false;
                        }
                        if (t___1426)
                        {
                            error__743 = "JSON value";
                        }
                        else
                        {
                            error__743 = "digit";
                        }
                        expectedTokenError__353(sourceText__736, i__737, out__738, error__743);
                        return__311 = -1;
                        goto fn__739;
                    }
                    t___2217 = C::StringUtil.Next(sourceText__736, i__737);
                    i__737 = t___2217;
                    int nDigits__744 = 1;
                    t___2218 = (double)(digit0__742 - 48);
                    double tentativeFloat64__745 = t___2218;
                    t___2219 = (long)(digit0__742 - 48);
                    long tentativeInt64__746 = t___2219;
                    bool overflowInt64__747 = false;
                    if (48 != digit0__742) while (true)
                    {
                        if (!C::StringUtil.HasIndex(sourceText__736, i__737)) break;
                        int possibleDigit__748 = C::StringUtil.Get(sourceText__736, i__737);
                        if (48 <= possibleDigit__748)
                        {
                            t___1434 = possibleDigit__748 <= 57;
                        }
                        else
                        {
                            t___1434 = false;
                        }
                        if (t___1434)
                        {
                            t___2222 = C::StringUtil.Next(sourceText__736, i__737);
                            i__737 = t___2222;
                            nDigits__744 = nDigits__744 + 1;
                            int nextDigit__749 = possibleDigit__748 - 48;
                            t___1437 = tentativeFloat64__745 * 10.0;
                            t___2223 = (double) nextDigit__749;
                            tentativeFloat64__745 = t___1437 + t___2223;
                            long oldInt64__750 = tentativeInt64__746;
                            t___1439 = tentativeInt64__746 * 10;
                            t___2224 = (long) nextDigit__749;
                            tentativeInt64__746 = t___1439 + t___2224;
                            if (tentativeInt64__746 < oldInt64__750)
                            {
                                if (-9223372036854775808 - oldInt64__750 == -((long) nextDigit__749))
                                {
                                    if (isNegative__740)
                                    {
                                        t___1442 = oldInt64__750 > 0;
                                    }
                                    else
                                    {
                                        t___1442 = false;
                                    }
                                    t___1443 = t___1442;
                                }
                                else
                                {
                                    t___1443 = false;
                                }
                                if (!t___1443)
                                {
                                    overflowInt64__747 = true;
                                }
                            }
                        }
                        else break;
                    }
                    int nDigitsAfterPoint__751 = 0;
                    if (C::StringUtil.HasIndex(sourceText__736, i__737))
                    {
                        t___2228 = C::StringUtil.Get(sourceText__736, i__737);
                        t___1446 = 46 == t___2228;
                    }
                    else
                    {
                        t___1446 = false;
                    }
                    if (t___1446)
                    {
                        t___2229 = C::StringUtil.Next(sourceText__736, i__737);
                        i__737 = t___2229;
                        int afterPoint__752 = i__737;
                        while (true)
                        {
                            if (!C::StringUtil.HasIndex(sourceText__736, i__737)) break;
                            int possibleDigit__753 = C::StringUtil.Get(sourceText__736, i__737);
                            if (48 <= possibleDigit__753)
                            {
                                t___1450 = possibleDigit__753 <= 57;
                            }
                            else
                            {
                                t___1450 = false;
                            }
                            if (t___1450)
                            {
                                t___2232 = C::StringUtil.Next(sourceText__736, i__737);
                                i__737 = t___2232;
                                nDigits__744 = nDigits__744 + 1;
                                nDigitsAfterPoint__751 = nDigitsAfterPoint__751 + 1;
                                t___1453 = tentativeFloat64__745 * 10.0;
                                t___2233 = (double)(possibleDigit__753 - 48);
                                tentativeFloat64__745 = t___1453 + t___2233;
                            }
                            else break;
                        }
                        if (i__737 == afterPoint__752)
                        {
                            expectedTokenError__353(sourceText__736, i__737, out__738, "digit");
                            return__311 = -1;
                            goto fn__739;
                        }
                    }
                    int nExponentDigits__754 = 0;
                    if (C::StringUtil.HasIndex(sourceText__736, i__737))
                    {
                        t___2236 = C::StringUtil.Get(sourceText__736, i__737);
                        t___1456 = 101 == (t___2236 | 32);
                    }
                    else
                    {
                        t___1456 = false;
                    }
                    if (t___1456)
                    {
                        t___2237 = C::StringUtil.Next(sourceText__736, i__737);
                        i__737 = t___2237;
                        if (!C::StringUtil.HasIndex(sourceText__736, i__737))
                        {
                            expectedTokenError__353(sourceText__736, i__737, out__738, "sign or digit");
                            return__311 = -1;
                            goto fn__739;
                        }
                        int afterE__755 = C::StringUtil.Get(sourceText__736, i__737);
                        if (afterE__755 == 43)
                        {
                            t___1460 = true;
                        }
                        else
                        {
                            t___1460 = afterE__755 == 45;
                        }
                        if (t___1460)
                        {
                            t___2241 = C::StringUtil.Next(sourceText__736, i__737);
                            i__737 = t___2241;
                        }
                        while (true)
                        {
                            if (!C::StringUtil.HasIndex(sourceText__736, i__737)) break;
                            int possibleDigit__756 = C::StringUtil.Get(sourceText__736, i__737);
                            if (48 <= possibleDigit__756)
                            {
                                t___1464 = possibleDigit__756 <= 57;
                            }
                            else
                            {
                                t___1464 = false;
                            }
                            if (t___1464)
                            {
                                t___2244 = C::StringUtil.Next(sourceText__736, i__737);
                                i__737 = t___2244;
                                nExponentDigits__754 = nExponentDigits__754 + 1;
                            }
                            else break;
                        }
                        if (nExponentDigits__754 == 0)
                        {
                            expectedTokenError__353(sourceText__736, i__737, out__738, "exponent digit");
                            return__311 = -1;
                            goto fn__739;
                        }
                    }
                    int afterExponent__757 = i__737;
                    if (nExponentDigits__754 == 0)
                    {
                        if (nDigitsAfterPoint__751 == 0)
                        {
                            t___1466 = !overflowInt64__747;
                        }
                        else
                        {
                            t___1466 = false;
                        }
                        t___1467 = t___1466;
                    }
                    else
                    {
                        t___1467 = false;
                    }
                    if (t___1467)
                    {
                        long value__758;
                        if (isNegative__740)
                        {
                            value__758 = -tentativeInt64__746;
                        }
                        else
                        {
                            value__758 = tentativeInt64__746;
                        }
                        if (-2147483648 <= value__758)
                        {
                            t___1469 = value__758 <= 2147483647;
                        }
                        else
                        {
                            t___1469 = false;
                        }
                        if (t___1469)
                        {
                            t___2246 = (int) value__758;
                            out__738.Int32_Value(t___2246);
                        }
                        else out__738.Int64_Value(value__758);
                        return__311 = i__737;
                        goto fn__739;
                    }
                    string numericTokenString__759 = C::StringUtil.Slice(sourceText__736, startOfNumber__741, i__737);
                    double doubleValue__760 = double.NaN;
                    if (nExponentDigits__754 != 0)
                    {
                        t___1472 = true;
                    }
                    else
                    {
                        t___1472 = nDigitsAfterPoint__751 != 0;
                    }
                    if (t___1472)
                    {
                        try
                        {
                            t___1473 = C::Float64.ToFloat64(numericTokenString__759);
                            doubleValue__760 = t___1473;
                        }
                        catch
                        {
                        }
                    }
                    if (C::Float64.Compare(doubleValue__760, double.NegativeInfinity) != 0.0)
                    {
                        if (C::Float64.Compare(doubleValue__760, double.PositiveInfinity) != 0.0)
                        {
                            t___1474 = C::Float64.Compare(doubleValue__760, double.NaN) != 0.0;
                        }
                        else
                        {
                            t___1474 = false;
                        }
                        t___1475 = t___1474;
                    }
                    else
                    {
                        t___1475 = false;
                    }
                    if (t___1475) out__738.Float64_Value(doubleValue__760);
                    else out__738.NumericTokenValue(numericTokenString__759);
                    return__311 = i__737;
                }
                fn__739:
                {
                }
            }
            return return__311;
        }
        public static void ParseJsonToProducer(string sourceText__662, IJsonProducer out__663)
        {
            int t___2195;
            IJsonParseErrorReceiver ? t___2197;
            int t___2198;
            string t___2199;
            bool t___1402;
            int t___1405;
            int i__665 = 0;
            int afterValue__666 = parseJsonValue__356(sourceText__662, i__665, out__663);
            if (afterValue__666 >= 0)
            {
                t___1405 = C::StringUtil.RequireStringIndex(afterValue__666);
                t___2195 = skipJsonSpaces__355(sourceText__662, t___1405);
                i__665 = t___2195;
                if (C::StringUtil.HasIndex(sourceText__662, i__665))
                {
                    t___2197 = out__663.ParseErrorReceiver;
                    t___1402 = !(t___2197 == null);
                }
                else
                {
                    t___1402 = false;
                }
                if (t___1402)
                {
                    t___2198 = sourceText__662.Length;
                    t___2199 = C::StringUtil.Slice(sourceText__662, i__665, t___2198);
                    storeJsonError__354(out__663, "Extraneous JSON `" + t___2199 + "`");
                }
            }
        }
        public static IJsonSyntaxTree ParseJson(string sourceText__761)
        {
            JsonSyntaxTreeProducer p__763 = new JsonSyntaxTreeProducer();
            ParseJsonToProducer(sourceText__761, p__763);
            return p__763.ToJsonSyntaxTree();
        }
        public static IJsonAdapter<bool> BooleanJsonAdapter()
        {
            return new BooleanJsonAdapter();
        }
        public static IJsonAdapter<double> Float64_JsonAdapter()
        {
            return new Float64_JsonAdapter();
        }
        public static IJsonAdapter<int> Int32_JsonAdapter()
        {
            return new Int32_JsonAdapter();
        }
        public static IJsonAdapter<long> Int64_JsonAdapter()
        {
            return new Int64_JsonAdapter();
        }
        public static IJsonAdapter<string> StringJsonAdapter()
        {
            return new StringJsonAdapter();
        }
        public static IJsonAdapter<G::IReadOnlyList<T__183>> ListJsonAdapter<T__183>(IJsonAdapter<T__183> adapterForT__839)
        {
            return new ListJsonAdapter<T__183>(adapterForT__839);
        }
        static JsonGlobal()
        {
            hexDigits__373 = C::Listed.CreateReadOnlyList<string>("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f");
        }
    }
}
