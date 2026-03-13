package temper.std.json;
import temper.core.Core;
import java.util.List;
import temper.core.Nullable;
import static java.lang.Double.NaN;
import static java.lang.Double.NEGATIVE_INFINITY;
import static java.lang.Double.POSITIVE_INFINITY;
public final class JsonGlobal {
    private JsonGlobal() {
    }
    static final List<String> hexDigits__373;
    static int parseJsonValue__356(String sourceText__670, int i__671, JsonProducer out__672) {
        int return__302;
        int t_2201;
        int t_2204;
        boolean t_1412;
        fn__673: {
            t_2201 = JsonGlobal.skipJsonSpaces__355(sourceText__670, i__671);
            i__671 = t_2201;
            if (!Core.stringHasIndex(sourceText__670, i__671)) {
                JsonGlobal.expectedTokenError__353(sourceText__670, i__671, out__672, "JSON value");
                return__302 = -1;
                break fn__673;
            }
            t_2204 = sourceText__670.codePointAt(i__671);
            if (t_2204 == 123) {
                return__302 = JsonGlobal.parseJsonObject__357(sourceText__670, i__671, out__672);
            } else if (t_2204 == 91) {
                return__302 = JsonGlobal.parseJsonArray__358(sourceText__670, i__671, out__672);
            } else if (t_2204 == 34) {
                return__302 = JsonGlobal.parseJsonString__359(sourceText__670, i__671, out__672);
            } else {
                if (t_2204 == 116) {
                    t_1412 = true;
                } else {
                    t_1412 = t_2204 == 102;
                }
                if (t_1412) {
                    return__302 = JsonGlobal.parseJsonBoolean__362(sourceText__670, i__671, out__672);
                } else if (t_2204 == 110) {
                    return__302 = JsonGlobal.parseJsonNull__363(sourceText__670, i__671, out__672);
                } else {
                    return__302 = JsonGlobal.parseJsonNumber__365(sourceText__670, i__671, out__672);
                }
            }
        }
        return return__302;
    }
    static void encodeHex4__352(int cp__580, StringBuilder buffer__581) {
        int b0__583 = cp__580 / 4096 & 15;
        int b1__584 = cp__580 / 256 & 15;
        int b2__585 = cp__580 / 16 & 15;
        int b3__586 = cp__580 & 15;
        String t_2482 = Core.listGet(hexDigits__373, b0__583);
        buffer__581.append(t_2482);
        String t_2484 = Core.listGet(hexDigits__373, b1__584);
        buffer__581.append(t_2484);
        String t_2486 = Core.listGet(hexDigits__373, b2__585);
        buffer__581.append(t_2486);
        String t_2488 = Core.listGet(hexDigits__373, b3__586);
        buffer__581.append(t_2488);
    }
    static void encodeJsonString__351(String x__572, StringBuilder buffer__573) {
        boolean t_1745;
        boolean t_1746;
        String t_1747;
        String t_1748;
        buffer__573.append("\"");
        int i__575 = 0;
        int emitted__576 = i__575;
        while (true) {
            if (!Core.stringHasIndex(x__572, i__575)) {
                break;
            }
            int cp__577 = x__572.codePointAt(i__575);
            if (cp__577 == 8) {
                t_1748 = "\\b";
            } else if (cp__577 == 9) {
                t_1748 = "\\t";
            } else if (cp__577 == 10) {
                t_1748 = "\\n";
            } else if (cp__577 == 12) {
                t_1748 = "\\f";
            } else if (cp__577 == 13) {
                t_1748 = "\\r";
            } else if (cp__577 == 34) {
                t_1748 = "\\\"";
            } else if (cp__577 == 92) {
                t_1748 = "\\\\";
            } else {
                if (cp__577 < 32) {
                    t_1746 = true;
                } else {
                    if (55296 <= cp__577) {
                        t_1745 = cp__577 <= 57343;
                    } else {
                        t_1745 = false;
                    }
                    t_1746 = t_1745;
                }
                if (t_1746) {
                    t_1747 = "\\u";
                } else {
                    t_1747 = "";
                }
                t_1748 = t_1747;
            }
            String replacement__578 = t_1748;
            int nextI__579 = Core.stringNext(x__572, i__575);
            if (!replacement__578.equals("")) {
                Core.stringBuilderAppendBetween(buffer__573, x__572, emitted__576, i__575);
                buffer__573.append(replacement__578);
                if (replacement__578.equals("\\u")) {
                    JsonGlobal.encodeHex4__352(cp__577, buffer__573);
                }
                emitted__576 = nextI__579;
            }
            i__575 = nextI__579;
        }
        Core.stringBuilderAppendBetween(buffer__573, x__572, emitted__576, i__575);
        buffer__573.append("\"");
    }
    static void storeJsonError__354(JsonProducer out__659, String explanation__660) {
        @Nullable JsonParseErrorReceiver t_2361 = out__659.getParseErrorReceiver();
        if (t_2361 != null) {
            t_2361.explainJsonError(explanation__660);
        }
    }
    static void expectedTokenError__353(String sourceText__653, int i__654, JsonProducer out__655, String shortExplanation__656) {
        int t_2358;
        String t_2359;
        String gotten__658;
        if (Core.stringHasIndex(sourceText__653, i__654)) {
            t_2358 = sourceText__653.length();
            t_2359 = Core.stringSlice(sourceText__653, i__654, t_2358);
            gotten__658 = "`" + t_2359 + "`";
        } else {
            gotten__658 = "end-of-file";
        }
        JsonGlobal.storeJsonError__354(out__655, "Expected " + shortExplanation__656 + ", but got " + gotten__658);
    }
    static int skipJsonSpaces__355(String sourceText__667, int i__668) {
        int t_2355;
        int t_2356;
        boolean t_1616;
        boolean t_1617;
        boolean t_1618;
        while (true) {
            if (!Core.stringHasIndex(sourceText__667, i__668)) {
                break;
            }
            t_2355 = sourceText__667.codePointAt(i__668);
            if (t_2355 == 9) {
                t_1618 = true;
            } else {
                if (t_2355 == 10) {
                    t_1617 = true;
                } else {
                    if (t_2355 == 13) {
                        t_1616 = true;
                    } else {
                        t_1616 = t_2355 == 32;
                    }
                    t_1617 = t_1616;
                }
                t_1618 = t_1617;
            }
            if (!t_1618) {
                break;
            }
            t_2356 = Core.stringNext(sourceText__667, i__668);
            i__668 = t_2356;
        }
        return i__668;
    }
    static int decodeHexUnsigned__361(String sourceText__708, int start__709, int limit__710) {
        int return__307;
        int t_2353;
        boolean t_1609;
        boolean t_1610;
        boolean t_1611;
        int t_1612;
        fn__711: {
            int n__712 = 0;
            int i__713 = start__709;
            while (true) {
                if (Integer.compare(i__713, limit__710) >= 0) {
                    break;
                }
                int cp__714 = sourceText__708.codePointAt(i__713);
                if (48 <= cp__714) {
                    t_1609 = cp__714 <= 48;
                } else {
                    t_1609 = false;
                }
                if (t_1609) {
                    t_1612 = cp__714 - 48;
                } else {
                    if (65 <= cp__714) {
                        t_1610 = cp__714 <= 70;
                    } else {
                        t_1610 = false;
                    }
                    if (t_1610) {
                        t_1612 = cp__714 - 65 + 10;
                    } else {
                        if (97 <= cp__714) {
                            t_1611 = cp__714 <= 102;
                        } else {
                            t_1611 = false;
                        }
                        if (t_1611) {
                            t_1612 = cp__714 - 97 + 10;
                        } else {
                            return__307 = -1;
                            break fn__711;
                        }
                    }
                }
                int digit__715 = t_1612;
                n__712 = n__712 * 16 + digit__715;
                t_2353 = Core.stringNext(sourceText__708, i__713);
                i__713 = t_2353;
            }
            return__307 = n__712;
        }
        return return__307;
    }
    static int parseJsonStringTo__360(String sourceText__692, int i__693, StringBuilder sb__694, JsonProducer errOut__695) {
        int return__306;
        int t_2326;
        int t_2328;
        int t_2331;
        int t_2336;
        int t_2338;
        int t_2339;
        int t_2340;
        int t_2341;
        int t_2342;
        int t_2347;
        int t_2350;
        boolean t_1570;
        boolean t_1579;
        boolean t_1580;
        int t_1588;
        int t_1589;
        int t_1591;
        int t_1593;
        boolean t_1594;
        boolean t_1595;
        boolean t_1597;
        boolean t_1601;
        fn__696: {
            if (!Core.stringHasIndex(sourceText__692, i__693)) {
                t_1570 = true;
            } else {
                t_2326 = sourceText__692.codePointAt(i__693);
                t_1570 = t_2326 != 34;
            }
            if (t_1570) {
                JsonGlobal.expectedTokenError__353(sourceText__692, i__693, errOut__695, "\"");
                return__306 = -1;
                break fn__696;
            }
            t_2328 = Core.stringNext(sourceText__692, i__693);
            i__693 = t_2328;
            int leadSurrogate__697 = -1;
            int consumed__698 = i__693;
            while (true) {
                if (!Core.stringHasIndex(sourceText__692, i__693)) {
                    break;
                }
                int cp__699 = sourceText__692.codePointAt(i__693);
                if (cp__699 == 34) {
                    break;
                }
                t_2331 = Core.stringNext(sourceText__692, i__693);
                int iNext__700 = t_2331;
                int end__701 = sourceText__692.length();
                boolean needToFlush__702 = false;
                if (cp__699 != 92) {
                    t_1593 = cp__699;
                } else {
                    needToFlush__702 = true;
                    if (!Core.stringHasIndex(sourceText__692, iNext__700)) {
                        JsonGlobal.expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "escape sequence");
                        return__306 = -1;
                        break fn__696;
                    }
                    int esc0__704 = sourceText__692.codePointAt(iNext__700);
                    t_2336 = Core.stringNext(sourceText__692, iNext__700);
                    iNext__700 = t_2336;
                    if (esc0__704 == 34) {
                        t_1580 = true;
                    } else {
                        if (esc0__704 == 92) {
                            t_1579 = true;
                        } else {
                            t_1579 = esc0__704 == 47;
                        }
                        t_1580 = t_1579;
                    }
                    if (t_1580) {
                        t_1591 = esc0__704;
                    } else if (esc0__704 == 98) {
                        t_1591 = 8;
                    } else if (esc0__704 == 102) {
                        t_1591 = 12;
                    } else if (esc0__704 == 110) {
                        t_1591 = 10;
                    } else if (esc0__704 == 114) {
                        t_1591 = 13;
                    } else if (esc0__704 == 116) {
                        t_1591 = 9;
                    } else if (esc0__704 == 117) {
                        if (Core.stringHasAtLeast(sourceText__692, iNext__700, end__701, 4)) {
                            int startHex__706 = iNext__700;
                            t_2338 = Core.stringNext(sourceText__692, iNext__700);
                            iNext__700 = t_2338;
                            t_2339 = Core.stringNext(sourceText__692, iNext__700);
                            iNext__700 = t_2339;
                            t_2340 = Core.stringNext(sourceText__692, iNext__700);
                            iNext__700 = t_2340;
                            t_2341 = Core.stringNext(sourceText__692, iNext__700);
                            iNext__700 = t_2341;
                            t_2342 = JsonGlobal.decodeHexUnsigned__361(sourceText__692, startHex__706, iNext__700);
                            t_1588 = t_2342;
                        } else {
                            t_1588 = -1;
                        }
                        int hex__705 = t_1588;
                        if (hex__705 < 0) {
                            JsonGlobal.expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "four hex digits");
                            return__306 = -1;
                            break fn__696;
                        }
                        t_1589 = hex__705;
                        t_1591 = t_1589;
                    } else {
                        JsonGlobal.expectedTokenError__353(sourceText__692, iNext__700, errOut__695, "escape sequence");
                        return__306 = -1;
                        break fn__696;
                    }
                    t_1593 = t_1591;
                }
                int decodedCp__703 = t_1593;
                if (leadSurrogate__697 >= 0) {
                    needToFlush__702 = true;
                    int lead__707 = leadSurrogate__697;
                    if (56320 <= decodedCp__703) {
                        t_1594 = decodedCp__703 <= 57343;
                    } else {
                        t_1594 = false;
                    }
                    if (t_1594) {
                        leadSurrogate__697 = -1;
                        decodedCp__703 = 65536 + ((lead__707 - 55296) * 1024 | decodedCp__703 - 56320);
                    }
                } else {
                    if (55296 <= decodedCp__703) {
                        t_1595 = decodedCp__703 <= 56319;
                    } else {
                        t_1595 = false;
                    }
                    if (t_1595) {
                        needToFlush__702 = true;
                    }
                }
                if (needToFlush__702) {
                    Core.stringBuilderAppendBetween(sb__694, sourceText__692, consumed__698, i__693);
                    if (leadSurrogate__697 >= 0) {
                        Core.stringBuilderAppendCodePoint(sb__694, leadSurrogate__697);
                    }
                    if (55296 <= decodedCp__703) {
                        t_1597 = decodedCp__703 <= 56319;
                    } else {
                        t_1597 = false;
                    }
                    if (t_1597) {
                        leadSurrogate__697 = decodedCp__703;
                    } else {
                        leadSurrogate__697 = -1;
                        Core.stringBuilderAppendCodePoint(sb__694, decodedCp__703);
                    }
                    consumed__698 = iNext__700;
                }
                i__693 = iNext__700;
            }
            if (!Core.stringHasIndex(sourceText__692, i__693)) {
                t_1601 = true;
            } else {
                t_2347 = sourceText__692.codePointAt(i__693);
                t_1601 = t_2347 != 34;
            }
            if (t_1601) {
                JsonGlobal.expectedTokenError__353(sourceText__692, i__693, errOut__695, "\"");
                return__306 = -1;
            } else {
                if (leadSurrogate__697 >= 0) {
                    Core.stringBuilderAppendCodePoint(sb__694, leadSurrogate__697);
                } else {
                    Core.stringBuilderAppendBetween(sb__694, sourceText__692, consumed__698, i__693);
                }
                t_2350 = Core.stringNext(sourceText__692, i__693);
                i__693 = t_2350;
                return__306 = i__693;
            }
        }
        return return__306;
    }
    static int parseJsonObject__357(String sourceText__674, int i__675, JsonProducer out__676) {
        int return__303;
        int t_2296;
        int t_2299;
        int t_2300;
        int t_2302;
        String t_2306;
        int t_2308;
        int t_2310;
        int t_2311;
        int t_2315;
        int t_2317;
        int t_2318;
        int t_2319;
        int t_2321;
        boolean t_1533;
        boolean t_1539;
        int t_1545;
        int t_1547;
        boolean t_1551;
        int t_1555;
        boolean t_1560;
        boolean t_1565;
        fn__677: {
            if (!Core.stringHasIndex(sourceText__674, i__675)) {
                t_1533 = true;
            } else {
                t_2296 = sourceText__674.codePointAt(i__675);
                t_1533 = t_2296 != 123;
            }
            if (t_1533) {
                JsonGlobal.expectedTokenError__353(sourceText__674, i__675, out__676, "'{'");
                return__303 = -1;
                break fn__677;
            }
            out__676.startObject();
            t_2299 = Core.stringNext(sourceText__674, i__675);
            t_2300 = JsonGlobal.skipJsonSpaces__355(sourceText__674, t_2299);
            i__675 = t_2300;
            if (Core.stringHasIndex(sourceText__674, i__675)) {
                t_2302 = sourceText__674.codePointAt(i__675);
                t_1539 = t_2302 != 125;
            } else {
                t_1539 = false;
            }
            if (t_1539) {
                while (true) {
                    StringBuilder keyBuffer__678 = new StringBuilder();
                    int afterKey__679 = JsonGlobal.parseJsonStringTo__360(sourceText__674, i__675, keyBuffer__678, out__676);
                    if (afterKey__679 < 0) {
                        return__303 = -1;
                        break fn__677;
                    }
                    t_2306 = keyBuffer__678.toString();
                    out__676.objectKey(t_2306);
                    t_1545 = Core.requireStringIndex(afterKey__679);
                    t_1547 = t_1545;
                    t_2308 = JsonGlobal.skipJsonSpaces__355(sourceText__674, t_1547);
                    i__675 = t_2308;
                    if (Core.stringHasIndex(sourceText__674, i__675)) {
                        t_2310 = sourceText__674.codePointAt(i__675);
                        t_1551 = t_2310 == 58;
                    } else {
                        t_1551 = false;
                    }
                    if (t_1551) {
                        t_2311 = Core.stringNext(sourceText__674, i__675);
                        i__675 = t_2311;
                        int afterPropertyValue__680 = JsonGlobal.parseJsonValue__356(sourceText__674, i__675, out__676);
                        if (afterPropertyValue__680 < 0) {
                            return__303 = -1;
                            break fn__677;
                        }
                        t_1555 = Core.requireStringIndex(afterPropertyValue__680);
                        i__675 = t_1555;
                    } else {
                        JsonGlobal.expectedTokenError__353(sourceText__674, i__675, out__676, "':'");
                        return__303 = -1;
                        break fn__677;
                    }
                    t_2315 = JsonGlobal.skipJsonSpaces__355(sourceText__674, i__675);
                    i__675 = t_2315;
                    if (Core.stringHasIndex(sourceText__674, i__675)) {
                        t_2317 = sourceText__674.codePointAt(i__675);
                        t_1560 = t_2317 == 44;
                    } else {
                        t_1560 = false;
                    }
                    if (t_1560) {
                        t_2318 = Core.stringNext(sourceText__674, i__675);
                        t_2319 = JsonGlobal.skipJsonSpaces__355(sourceText__674, t_2318);
                        i__675 = t_2319;
                    } else {
                        break;
                    }
                }
            }
            if (Core.stringHasIndex(sourceText__674, i__675)) {
                t_2321 = sourceText__674.codePointAt(i__675);
                t_1565 = t_2321 == 125;
            } else {
                t_1565 = false;
            }
            if (t_1565) {
                out__676.endObject();
                return__303 = Core.stringNext(sourceText__674, i__675);
            } else {
                JsonGlobal.expectedTokenError__353(sourceText__674, i__675, out__676, "'}'");
                return__303 = -1;
            }
        }
        return return__303;
    }
    static int parseJsonArray__358(String sourceText__681, int i__682, JsonProducer out__683) {
        int return__304;
        int t_2276;
        int t_2279;
        int t_2280;
        int t_2282;
        int t_2285;
        int t_2287;
        int t_2288;
        int t_2289;
        int t_2291;
        boolean t_1509;
        boolean t_1515;
        int t_1518;
        boolean t_1523;
        boolean t_1528;
        fn__684: {
            if (!Core.stringHasIndex(sourceText__681, i__682)) {
                t_1509 = true;
            } else {
                t_2276 = sourceText__681.codePointAt(i__682);
                t_1509 = t_2276 != 91;
            }
            if (t_1509) {
                JsonGlobal.expectedTokenError__353(sourceText__681, i__682, out__683, "'['");
                return__304 = -1;
                break fn__684;
            }
            out__683.startArray();
            t_2279 = Core.stringNext(sourceText__681, i__682);
            t_2280 = JsonGlobal.skipJsonSpaces__355(sourceText__681, t_2279);
            i__682 = t_2280;
            if (Core.stringHasIndex(sourceText__681, i__682)) {
                t_2282 = sourceText__681.codePointAt(i__682);
                t_1515 = t_2282 != 93;
            } else {
                t_1515 = false;
            }
            if (t_1515) {
                while (true) {
                    int afterElementValue__685 = JsonGlobal.parseJsonValue__356(sourceText__681, i__682, out__683);
                    if (afterElementValue__685 < 0) {
                        return__304 = -1;
                        break fn__684;
                    }
                    t_1518 = Core.requireStringIndex(afterElementValue__685);
                    i__682 = t_1518;
                    t_2285 = JsonGlobal.skipJsonSpaces__355(sourceText__681, i__682);
                    i__682 = t_2285;
                    if (Core.stringHasIndex(sourceText__681, i__682)) {
                        t_2287 = sourceText__681.codePointAt(i__682);
                        t_1523 = t_2287 == 44;
                    } else {
                        t_1523 = false;
                    }
                    if (t_1523) {
                        t_2288 = Core.stringNext(sourceText__681, i__682);
                        t_2289 = JsonGlobal.skipJsonSpaces__355(sourceText__681, t_2288);
                        i__682 = t_2289;
                    } else {
                        break;
                    }
                }
            }
            if (Core.stringHasIndex(sourceText__681, i__682)) {
                t_2291 = sourceText__681.codePointAt(i__682);
                t_1528 = t_2291 == 93;
            } else {
                t_1528 = false;
            }
            if (t_1528) {
                out__683.endArray();
                return__304 = Core.stringNext(sourceText__681, i__682);
            } else {
                JsonGlobal.expectedTokenError__353(sourceText__681, i__682, out__683, "']'");
                return__304 = -1;
            }
        }
        return return__304;
    }
    static int parseJsonString__359(String sourceText__686, int i__687, JsonProducer out__688) {
        String t_2273;
        StringBuilder sb__690 = new StringBuilder();
        int after__691 = JsonGlobal.parseJsonStringTo__360(sourceText__686, i__687, sb__690, out__688);
        if (after__691 >= 0) {
            t_2273 = sb__690.toString();
            out__688.stringValue(t_2273);
        }
        return after__691;
    }
    static int afterSubstring__364(String string__730, int inString__731, String substring__732) {
        int return__310;
        int t_2268;
        int t_2269;
        fn__733: {
            int i__734 = inString__731;
            int j__735 = 0;
            while (true) {
                if (!Core.stringHasIndex(substring__732, j__735)) {
                    break;
                }
                if (!Core.stringHasIndex(string__730, i__734)) {
                    return__310 = -1;
                    break fn__733;
                }
                if (string__730.codePointAt(i__734) != substring__732.codePointAt(j__735)) {
                    return__310 = -1;
                    break fn__733;
                }
                t_2268 = Core.stringNext(string__730, i__734);
                i__734 = t_2268;
                t_2269 = Core.stringNext(substring__732, j__735);
                j__735 = t_2269;
            }
            return__310 = i__734;
        }
        return return__310;
    }
    static int parseJsonBoolean__362(String sourceText__716, int i__717, JsonProducer out__718) {
        int return__308;
        int t_2257;
        fn__719: {
            int ch0__720;
            if (Core.stringHasIndex(sourceText__716, i__717)) {
                t_2257 = sourceText__716.codePointAt(i__717);
                ch0__720 = t_2257;
            } else {
                ch0__720 = 0;
            }
            int end__721 = sourceText__716.length();
            @Nullable String keyword__722;
            int n__723;
            if (ch0__720 == 102) {
                keyword__722 = "false";
                n__723 = 5;
            } else if (ch0__720 == 116) {
                keyword__722 = "true";
                n__723 = 4;
            } else {
                keyword__722 = null;
                n__723 = 0;
            }
            if (keyword__722 != null) {
                String keyword_958 = keyword__722;
                if (Core.stringHasAtLeast(sourceText__716, i__717, end__721, n__723)) {
                    int after__724 = JsonGlobal.afterSubstring__364(sourceText__716, i__717, keyword_958);
                    if (after__724 >= 0) {
                        return__308 = Core.requireStringIndex(after__724);
                        out__718.booleanValue(n__723 == 4);
                        break fn__719;
                    }
                }
            }
            JsonGlobal.expectedTokenError__353(sourceText__716, i__717, out__718, "`false` or `true`");
            return__308 = -1;
        }
        return return__308;
    }
    static int parseJsonNull__363(String sourceText__725, int i__726, JsonProducer out__727) {
        int return__309;
        fn__728: {
            int after__729 = JsonGlobal.afterSubstring__364(sourceText__725, i__726, "null");
            if (after__729 >= 0) {
                return__309 = Core.requireStringIndex(after__729);
                out__727.nullValue();
                break fn__728;
            }
            JsonGlobal.expectedTokenError__353(sourceText__725, i__726, out__727, "`null`");
            return__309 = -1;
        }
        return return__309;
    }
    static int parseJsonNumber__365(String sourceText__736, int i__737, JsonProducer out__738) {
        int return__311;
        int t_2212;
        int t_2213;
        int t_2215;
        int t_2217;
        double t_2218;
        long t_2219;
        int t_2222;
        double t_2223;
        long t_2224;
        int t_2228;
        int t_2229;
        int t_2232;
        double t_2233;
        int t_2236;
        int t_2237;
        int t_2241;
        int t_2244;
        int t_2246;
        boolean t_1420;
        boolean t_1425;
        boolean t_1426;
        boolean t_1434;
        double t_1437;
        long t_1439;
        boolean t_1442;
        boolean t_1443;
        boolean t_1446;
        boolean t_1450;
        double t_1453;
        boolean t_1456;
        boolean t_1460;
        boolean t_1464;
        boolean t_1466;
        boolean t_1467;
        boolean t_1469;
        boolean t_1472;
        double t_1473;
        boolean t_1474;
        boolean t_1475;
        fn__739: {
            boolean isNegative__740 = false;
            int startOfNumber__741 = i__737;
            if (Core.stringHasIndex(sourceText__736, i__737)) {
                t_2212 = sourceText__736.codePointAt(i__737);
                t_1420 = t_2212 == 45;
            } else {
                t_1420 = false;
            }
            if (t_1420) {
                isNegative__740 = true;
                t_2213 = Core.stringNext(sourceText__736, i__737);
                i__737 = t_2213;
            }
            int digit0__742;
            if (Core.stringHasIndex(sourceText__736, i__737)) {
                t_2215 = sourceText__736.codePointAt(i__737);
                digit0__742 = t_2215;
            } else {
                digit0__742 = -1;
            }
            if (digit0__742 < 48) {
                t_1425 = true;
            } else {
                t_1425 = 57 < digit0__742;
            }
            if (t_1425) {
                String error__743;
                if (!isNegative__740) {
                    t_1426 = digit0__742 != 46;
                } else {
                    t_1426 = false;
                }
                if (t_1426) {
                    error__743 = "JSON value";
                } else {
                    error__743 = "digit";
                }
                JsonGlobal.expectedTokenError__353(sourceText__736, i__737, out__738, error__743);
                return__311 = -1;
                break fn__739;
            }
            t_2217 = Core.stringNext(sourceText__736, i__737);
            i__737 = t_2217;
            int nDigits__744 = 1;
            t_2218 = (double)(digit0__742 - 48);
            double tentativeFloat64__745 = t_2218;
            t_2219 = (long)(digit0__742 - 48);
            long tentativeInt64__746 = t_2219;
            boolean overflowInt64__747 = false;
            if (48 != digit0__742) {
                while (true) {
                    if (!Core.stringHasIndex(sourceText__736, i__737)) {
                        break;
                    }
                    int possibleDigit__748 = sourceText__736.codePointAt(i__737);
                    if (48 <= possibleDigit__748) {
                        t_1434 = possibleDigit__748 <= 57;
                    } else {
                        t_1434 = false;
                    }
                    if (t_1434) {
                        t_2222 = Core.stringNext(sourceText__736, i__737);
                        i__737 = t_2222;
                        nDigits__744 = nDigits__744 + 1;
                        int nextDigit__749 = possibleDigit__748 - 48;
                        t_1437 = tentativeFloat64__745 * 10.0D;
                        t_2223 = (double) nextDigit__749;
                        tentativeFloat64__745 = t_1437 + t_2223;
                        long oldInt64__750 = tentativeInt64__746;
                        t_1439 = tentativeInt64__746 * 10;
                        t_2224 = (long) nextDigit__749;
                        tentativeInt64__746 = t_1439 + t_2224;
                        if (tentativeInt64__746 < oldInt64__750) {
                            if (-9223372036854775808L - oldInt64__750 == -((long) nextDigit__749)) {
                                if (isNegative__740) {
                                    t_1442 = oldInt64__750 > 0;
                                } else {
                                    t_1442 = false;
                                }
                                t_1443 = t_1442;
                            } else {
                                t_1443 = false;
                            }
                            if (!t_1443) {
                                overflowInt64__747 = true;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            int nDigitsAfterPoint__751 = 0;
            if (Core.stringHasIndex(sourceText__736, i__737)) {
                t_2228 = sourceText__736.codePointAt(i__737);
                t_1446 = 46 == t_2228;
            } else {
                t_1446 = false;
            }
            if (t_1446) {
                t_2229 = Core.stringNext(sourceText__736, i__737);
                i__737 = t_2229;
                int afterPoint__752 = i__737;
                while (true) {
                    if (!Core.stringHasIndex(sourceText__736, i__737)) {
                        break;
                    }
                    int possibleDigit__753 = sourceText__736.codePointAt(i__737);
                    if (48 <= possibleDigit__753) {
                        t_1450 = possibleDigit__753 <= 57;
                    } else {
                        t_1450 = false;
                    }
                    if (t_1450) {
                        t_2232 = Core.stringNext(sourceText__736, i__737);
                        i__737 = t_2232;
                        nDigits__744 = nDigits__744 + 1;
                        nDigitsAfterPoint__751 = nDigitsAfterPoint__751 + 1;
                        t_1453 = tentativeFloat64__745 * 10.0D;
                        t_2233 = (double)(possibleDigit__753 - 48);
                        tentativeFloat64__745 = t_1453 + t_2233;
                    } else {
                        break;
                    }
                }
                if (i__737 == afterPoint__752) {
                    JsonGlobal.expectedTokenError__353(sourceText__736, i__737, out__738, "digit");
                    return__311 = -1;
                    break fn__739;
                }
            }
            int nExponentDigits__754 = 0;
            if (Core.stringHasIndex(sourceText__736, i__737)) {
                t_2236 = sourceText__736.codePointAt(i__737);
                t_1456 = 101 == (t_2236 | 32);
            } else {
                t_1456 = false;
            }
            if (t_1456) {
                t_2237 = Core.stringNext(sourceText__736, i__737);
                i__737 = t_2237;
                if (!Core.stringHasIndex(sourceText__736, i__737)) {
                    JsonGlobal.expectedTokenError__353(sourceText__736, i__737, out__738, "sign or digit");
                    return__311 = -1;
                    break fn__739;
                }
                int afterE__755 = sourceText__736.codePointAt(i__737);
                if (afterE__755 == 43) {
                    t_1460 = true;
                } else {
                    t_1460 = afterE__755 == 45;
                }
                if (t_1460) {
                    t_2241 = Core.stringNext(sourceText__736, i__737);
                    i__737 = t_2241;
                }
                while (true) {
                    if (!Core.stringHasIndex(sourceText__736, i__737)) {
                        break;
                    }
                    int possibleDigit__756 = sourceText__736.codePointAt(i__737);
                    if (48 <= possibleDigit__756) {
                        t_1464 = possibleDigit__756 <= 57;
                    } else {
                        t_1464 = false;
                    }
                    if (t_1464) {
                        t_2244 = Core.stringNext(sourceText__736, i__737);
                        i__737 = t_2244;
                        nExponentDigits__754 = nExponentDigits__754 + 1;
                    } else {
                        break;
                    }
                }
                if (nExponentDigits__754 == 0) {
                    JsonGlobal.expectedTokenError__353(sourceText__736, i__737, out__738, "exponent digit");
                    return__311 = -1;
                    break fn__739;
                }
            }
            int afterExponent__757 = i__737;
            if (nExponentDigits__754 == 0) {
                if (nDigitsAfterPoint__751 == 0) {
                    t_1466 = !overflowInt64__747;
                } else {
                    t_1466 = false;
                }
                t_1467 = t_1466;
            } else {
                t_1467 = false;
            }
            if (t_1467) {
                long value__758;
                if (isNegative__740) {
                    value__758 = -tentativeInt64__746;
                } else {
                    value__758 = tentativeInt64__746;
                }
                if (-2147483648 <= value__758) {
                    t_1469 = value__758 <= 2147483647;
                } else {
                    t_1469 = false;
                }
                if (t_1469) {
                    t_2246 = (int) value__758;
                    out__738.int32Value(t_2246);
                } else {
                    out__738.int64Value(value__758);
                }
                return__311 = i__737;
                break fn__739;
            }
            String numericTokenString__759 = Core.stringSlice(sourceText__736, startOfNumber__741, i__737);
            double doubleValue__760 = NaN;
            if (nExponentDigits__754 != 0) {
                t_1472 = true;
            } else {
                t_1472 = nDigitsAfterPoint__751 != 0;
            }
            if (t_1472) {
                try {
                    t_1473 = Core.stringToFloat64(numericTokenString__759);
                    doubleValue__760 = t_1473;
                } catch (RuntimeException ignored$3) {
                }
            }
            if (Double.doubleToLongBits(doubleValue__760) != Double.doubleToLongBits(NEGATIVE_INFINITY)) {
                if (Double.doubleToLongBits(doubleValue__760) != Double.doubleToLongBits(POSITIVE_INFINITY)) {
                    t_1474 = Double.doubleToLongBits(doubleValue__760) != Double.doubleToLongBits(NaN);
                } else {
                    t_1474 = false;
                }
                t_1475 = t_1474;
            } else {
                t_1475 = false;
            }
            if (t_1475) {
                out__738.float64Value(doubleValue__760);
            } else {
                out__738.numericTokenValue(numericTokenString__759);
            }
            return__311 = i__737;
        }
        return return__311;
    }
    public static void parseJsonToProducer(String sourceText__662, JsonProducer out__663) {
        int t_2195;
        @Nullable JsonParseErrorReceiver t_2197;
        int t_2198;
        String t_2199;
        boolean t_1402;
        int t_1405;
        int i__665 = 0;
        int afterValue__666 = JsonGlobal.parseJsonValue__356(sourceText__662, i__665, out__663);
        if (afterValue__666 >= 0) {
            t_1405 = Core.requireStringIndex(afterValue__666);
            t_2195 = JsonGlobal.skipJsonSpaces__355(sourceText__662, t_1405);
            i__665 = t_2195;
            if (Core.stringHasIndex(sourceText__662, i__665)) {
                t_2197 = out__663.getParseErrorReceiver();
                t_1402 = t_2197 != null;
            } else {
                t_1402 = false;
            }
            if (t_1402) {
                t_2198 = sourceText__662.length();
                t_2199 = Core.stringSlice(sourceText__662, i__665, t_2198);
                JsonGlobal.storeJsonError__354(out__663, "Extraneous JSON `" + t_2199 + "`");
            }
        }
    }
    public static JsonSyntaxTree parseJson(String sourceText__761) {
        JsonSyntaxTreeProducer p__763 = new JsonSyntaxTreeProducer();
        JsonGlobal.parseJsonToProducer(sourceText__761, p__763);
        return p__763.toJsonSyntaxTree();
    }
    public static JsonAdapter<Boolean> booleanJsonAdapter() {
        return new BooleanJsonAdapter();
    }
    public static JsonAdapter<Double> float64JsonAdapter() {
        return new Float64JsonAdapter();
    }
    public static JsonAdapter<Integer> int32JsonAdapter() {
        return new Int32JsonAdapter();
    }
    public static JsonAdapter<Long> int64JsonAdapter() {
        return new Int64JsonAdapter();
    }
    public static JsonAdapter<String> stringJsonAdapter() {
        return new StringJsonAdapter();
    }
    public static<T__183> JsonAdapter<List<T__183>> listJsonAdapter(JsonAdapter<T__183> adapterForT__839) {
        return new ListJsonAdapter<>(adapterForT__839);
    }
    static {
        hexDigits__373 = List.of("0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f");
    }
}
