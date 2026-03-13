package temper.std.regex;
import java.util.List;
import temper.core.Nullable;
import temper.core.Core;
import java.util.ArrayList;
import static temper.std.regex.Codes.dot;
import static temper.std.regex.Codes.dash;
import static temper.std.regex.Codes.peso;
import static temper.std.regex.Codes.pipe;
import static temper.std.regex.Codes.plus;
import static temper.std.regex.Codes.star;
import static temper.std.regex.Codes.space;
import static temper.std.regex.Codes.caret;
import static temper.std.regex.Codes.slash;
import static temper.std.regex.Codes.tilde;
import static temper.std.regex.Codes.digit0;
import static temper.std.regex.Codes.digit9;
import static temper.std.regex.Codes.upperA;
import static temper.std.regex.Codes.upperZ;
import static temper.std.regex.Codes.lowerA;
import static temper.std.regex.Codes.lowerZ;
import static temper.std.regex.Codes.question;
import static temper.std.regex.Codes.ampersand;
import static temper.std.regex.Codes.backslash;
import static temper.std.regex.Codes.curlyLeft;
import static temper.std.regex.Codes.roundLeft;
import static temper.std.regex.Codes.underscore;
import static temper.std.regex.Codes.curlyRight;
import static temper.std.regex.Codes.roundRight;
import static temper.std.regex.Codes.squareLeft;
import static temper.std.regex.Codes.squareRight;
public final class RegexGlobal {
    private RegexGlobal() {
    }
    static final Special return__192;
    public static final Special Begin;
    static final Special return__194;
    public static final Special Dot;
    static final Special return__196;
    public static final Special End;
    static final Special return__198;
    public static final Special WordBoundary;
    static final SpecialSet return__200;
    public static final SpecialSet Digit;
    static final SpecialSet return__202;
    public static final SpecialSet Space;
    static final SpecialSet return__204;
    public static final SpecialSet Word;
    static final List<Integer> escapeNeeds__165;
    static final RegexRefs regexRefs__164;
    static List<Integer> buildEscapeNeeds__163() {
        boolean t_935;
        boolean t_936;
        boolean t_937;
        boolean t_938;
        boolean t_939;
        boolean t_940;
        boolean t_941;
        boolean t_942;
        boolean t_943;
        boolean t_944;
        boolean t_945;
        boolean t_946;
        boolean t_947;
        boolean t_948;
        boolean t_949;
        boolean t_950;
        boolean t_951;
        boolean t_952;
        boolean t_953;
        boolean t_954;
        boolean t_955;
        boolean t_956;
        boolean t_957;
        boolean t_958;
        int t_959;
        List<Integer> escapeNeeds__381 = new ArrayList<>();
        int code__382 = 0;
        while (code__382 <= 127) {
            if (code__382 == dash) {
                t_942 = true;
            } else {
                if (code__382 == space) {
                    t_941 = true;
                } else {
                    if (code__382 == underscore) {
                        t_940 = true;
                    } else {
                        if (digit0 <= code__382) {
                            t_935 = code__382 <= digit9;
                        } else {
                            t_935 = false;
                        }
                        if (t_935) {
                            t_939 = true;
                        } else {
                            if (upperA <= code__382) {
                                t_936 = code__382 <= upperZ;
                            } else {
                                t_936 = false;
                            }
                            if (t_936) {
                                t_938 = true;
                            } else {
                                if (lowerA <= code__382) {
                                    t_937 = code__382 <= lowerZ;
                                } else {
                                    t_937 = false;
                                }
                                t_938 = t_937;
                            }
                            t_939 = t_938;
                        }
                        t_940 = t_939;
                    }
                    t_941 = t_940;
                }
                t_942 = t_941;
            }
            if (t_942) {
                t_959 = 0;
            } else {
                if (code__382 == ampersand) {
                    t_958 = true;
                } else {
                    if (code__382 == backslash) {
                        t_957 = true;
                    } else {
                        if (code__382 == caret) {
                            t_956 = true;
                        } else {
                            if (code__382 == curlyLeft) {
                                t_955 = true;
                            } else {
                                if (code__382 == curlyRight) {
                                    t_954 = true;
                                } else {
                                    if (code__382 == dot) {
                                        t_953 = true;
                                    } else {
                                        if (code__382 == peso) {
                                            t_952 = true;
                                        } else {
                                            if (code__382 == pipe) {
                                                t_951 = true;
                                            } else {
                                                if (code__382 == plus) {
                                                    t_950 = true;
                                                } else {
                                                    if (code__382 == question) {
                                                        t_949 = true;
                                                    } else {
                                                        if (code__382 == roundLeft) {
                                                            t_948 = true;
                                                        } else {
                                                            if (code__382 == roundRight) {
                                                                t_947 = true;
                                                            } else {
                                                                if (code__382 == slash) {
                                                                    t_946 = true;
                                                                } else {
                                                                    if (code__382 == squareLeft) {
                                                                        t_945 = true;
                                                                    } else {
                                                                        if (code__382 == squareRight) {
                                                                            t_944 = true;
                                                                        } else {
                                                                            if (code__382 == star) {
                                                                                t_943 = true;
                                                                            } else {
                                                                                t_943 = code__382 == tilde;
                                                                            }
                                                                            t_944 = t_943;
                                                                        }
                                                                        t_945 = t_944;
                                                                    }
                                                                    t_946 = t_945;
                                                                }
                                                                t_947 = t_946;
                                                            }
                                                            t_948 = t_947;
                                                        }
                                                        t_949 = t_948;
                                                    }
                                                    t_950 = t_949;
                                                }
                                                t_951 = t_950;
                                            }
                                            t_952 = t_951;
                                        }
                                        t_953 = t_952;
                                    }
                                    t_954 = t_953;
                                }
                                t_955 = t_954;
                            }
                            t_956 = t_955;
                        }
                        t_957 = t_956;
                    }
                    t_958 = t_957;
                }
                if (t_958) {
                    t_959 = 2;
                } else {
                    t_959 = 1;
                }
            }
            Core.listAdd(escapeNeeds__381, t_959);
            code__382 = code__382 + 1;
        }
        return List.copyOf(escapeNeeds__381);
    }
    public static RegexNode entire(RegexNode item__228) {
        return new Sequence(List.of(Begin, item__228, End));
    }
    public static Repeat oneOrMore(RegexNode item__230, @Nullable Boolean reluctant__558) {
        boolean reluctant__231;
        if (reluctant__558 == null) {
            reluctant__231 = false;
        } else {
            reluctant__231 = reluctant__558;
        }
        return new Repeat(item__230, 1, null, reluctant__231);
    }
    public static Repeat oneOrMore(RegexNode item__230) {
        return oneOrMore(item__230, null);
    }
    public static Repeat optional(RegexNode item__233, @Nullable Boolean reluctant__560) {
        boolean reluctant__234;
        if (reluctant__560 == null) {
            reluctant__234 = false;
        } else {
            reluctant__234 = reluctant__560;
        }
        return new Repeat(item__233, 0, 1, reluctant__234);
    }
    public static Repeat optional(RegexNode item__233) {
        return optional(item__233, null);
    }
    static {
        return__192 = new Begin();
        Begin = return__192;
        return__194 = new Dot();
        Dot = return__194;
        return__196 = new End();
        End = return__196;
        return__198 = new WordBoundary();
        WordBoundary = return__198;
        return__200 = new Digit();
        Digit = return__200;
        return__202 = new Space();
        Space = return__202;
        return__204 = new Word();
        Word = return__204;
        escapeNeeds__165 = RegexGlobal.buildEscapeNeeds__163();
        regexRefs__164 = new RegexRefs();
    }
}
