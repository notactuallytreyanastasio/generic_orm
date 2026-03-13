from builtins import bool as bool41, str as str36, Exception as Exception46, int as int42, list as list14, tuple as tuple20, len as len1
from typing import MutableSequence as MutableSequence47, Callable as Callable67, Sequence as Sequence45, Union as Union38
from temper_core import Pair as Pair64, Label as Label49, list_join as list_join63, list_map as list_map65, string_get as string_get22, str_cat as str_cat25, int_to_string as int_to_string5, string_next as string_next24, int_add as int_add17, listed_reduce_from as listed_reduce_from66, list_get as list_get2
tuple_2592 = tuple20
list_join_2594 = list_join63
list_2595 = list14
pair_2596 = Pair64
list_map_2597 = list_map65
len_2599 = len1
string_get_2600 = string_get22
str_cat_2601 = str_cat25
int_to_string_2602 = int_to_string5
string_next_2605 = string_next24
int_add_2607 = int_add17
listed_reduce_from_2608 = listed_reduce_from66
list_get_2609 = list_get2
class Test:
    failed_on_assert_65: 'bool41'
    passing_66: 'bool41'
    messages_67: 'MutableSequence47[str36]'
    __slots__ = ('failed_on_assert_65', 'passing_66', 'messages_67')
    def assert_(this_13, success_43: 'bool41', message_44: 'Callable67[[], str36]') -> 'None':
        t_406: 'str36'
        if not success_43:
            this_13.passing_66 = False
            t_406 = message_44()
            this_13.messages_67.append(t_406)
    def assert_hard(this_14, success_47: 'bool41', message_48: 'Callable67[[], str36]') -> 'None':
        this_14.assert_(success_47, message_48)
        if not success_47:
            this_14.failed_on_assert_65 = True
            assert False, str36(this_14.messages_combined())
    def soft_fail_to_hard(this_15) -> 'None':
        if this_15.has_unhandled_fail:
            this_15.failed_on_assert_65 = True
            assert False, str36(this_15.messages_combined())
    @property
    def passing(this_17) -> 'bool41':
        return this_17.passing_66
    def messages(this_18) -> 'Sequence45[str36]':
        "Messages access is presented as a function because it likely allocates. Also,\nmessages might be automatically constructed in some cases, so it's possibly\nunwise to depend on their exact formatting.\n\nthis__18: Test\n"
        return tuple_2592(this_18.messages_67)
    @property
    def failed_on_assert(this_19) -> 'bool41':
        return this_19.failed_on_assert_65
    @property
    def has_unhandled_fail(this_20) -> 'bool41':
        t_264: 'bool41'
        if this_20.failed_on_assert_65:
            t_264 = True
        else:
            t_264 = this_20.passing_66
        return not t_264
    def messages_combined(this_21) -> 'Union38[str36, None]':
        return_35: 'Union38[str36, None]'
        if not this_21.messages_67:
            return_35 = None
        else:
            def fn_399(it_64: 'str36') -> 'str36':
                return it_64
            return_35 = list_join_2594(this_21.messages_67, ', ', fn_399)
        return return_35
    def __init__(this_25) -> None:
        this_25.failed_on_assert_65 = False
        this_25.passing_66 = True
        t_398: 'MutableSequence47[str36]' = list_2595()
        this_25.messages_67 = t_398
def process_test_cases(test_cases_69: 'Sequence45[(Pair64[str36, (Callable67[[Test], None])])]') -> 'Sequence45[(Pair64[str36, (Sequence45[str36])])]':
    def fn_395(test_case_71: 'Pair64[str36, (Callable67[[Test], None])]') -> 'Pair64[str36, (Sequence45[str36])]':
        t_390: 'bool41'
        t_393: 'Sequence45[str36]'
        t_246: 'bool41'
        t_248: 'bool41'
        key_73: 'str36' = test_case_71.key
        fun_74: 'Callable67[[Test], None]' = test_case_71.value
        test_75: 'Test' = Test()
        had_bubble_76: 'bool41' = False
        try:
            fun_74(test_75)
        except Exception46:
            had_bubble_76 = True
        messages_77: 'Sequence45[str36]' = test_75.messages()
        failures_78: 'Sequence45[str36]'
        if test_75.passing:
            t_246 = not had_bubble_76
        else:
            t_246 = False
        if t_246:
            failures_78 = ()
        else:
            if had_bubble_76:
                t_390 = test_75.failed_on_assert
                t_248 = not t_390
            else:
                t_248 = False
            if t_248:
                all_messages_79: 'MutableSequence47[str36]' = list_2595(messages_77)
                all_messages_79.append('Bubble')
                t_393 = tuple_2592(all_messages_79)
                failures_78 = t_393
            else:
                failures_78 = messages_77
        return pair_2596(key_73, failures_78)
    return list_map_2597(test_cases_69, fn_395)
def escape_xml_41(s_103: 'str36') -> 'str36':
    'escapeXml takes a string and escapes it so that it has the same meaning as an\nXML text node or attribute value.\n\ns__103: String\n'
    return_40: 'str36'
    t_381: 'int42'
    t_382: 'int42'
    t_225: 'bool41'
    t_226: 'bool41'
    t_227: 'bool41'
    t_228: 'bool41'
    t_230: 'str36'
    t_231: 'str36'
    sb_105: 'list14[str36]' = ['']
    end_106: 'int42' = len_2599(s_103)
    emitted_107: 'int42' = 0
    i_108: 'int42' = 0
    while i_108 < end_106:
        with Label49() as continue_408:
            c_109: 'int42' = string_get_2600(s_103, i_108)
            if c_109 == 38:
                t_231 = '&amp;'
            elif c_109 == 60:
                t_231 = '&lt;'
            elif c_109 == 62:
                t_231 = '&gt;'
            elif c_109 == 39:
                t_231 = '&#39;'
            elif c_109 == 34:
                t_231 = '&#34;'
            else:
                if c_109 == 10:
                    t_226 = True
                else:
                    if c_109 == 13:
                        t_225 = True
                    else:
                        t_225 = c_109 == 9
                    t_226 = t_225
                if t_226:
                    continue_408.break_()
                else:
                    if c_109 < 32:
                        t_228 = True
                    else:
                        if c_109 == 65534:
                            t_227 = True
                        else:
                            t_227 = c_109 == 65535
                        t_228 = t_227
                    if t_228:
                        t_230 = str_cat_2601('[0x', int_to_string_2602(c_109, 16), ']')
                    else:
                        continue_408.break_()
                    t_231 = t_230
            esc_110: 'str36' = t_231
            sb_105.append(s_103[emitted_107 : i_108])
            sb_105.append(esc_110)
            t_381 = string_next_2605(s_103, i_108)
            emitted_107 = t_381
        t_382 = string_next_2605(s_103, i_108)
        i_108 = t_382
    if emitted_107 == 0:
        return_40 = s_103
    else:
        sb_105.append(s_103[emitted_107 : end_106])
        return_40 = ''.join(sb_105)
    return return_40
def report_test_results(test_results_80: 'Sequence45[(Pair64[str36, (Sequence45[str36])])]', write_line_81: 'Callable67[[str36], None]') -> 'None':
    t_360: 'int42'
    t_363: 'str36'
    t_369: 'str36'
    write_line_81('<testsuites>')
    total_83: 'str36' = int_to_string_2602(len_2599(test_results_80))
    def fn_352(fails_85: 'int42', test_result_86: 'Pair64[str36, (Sequence45[str36])]') -> 'int42':
        t_203: 'int42'
        if not test_result_86.value:
            t_203 = 0
        else:
            t_203 = 1
        return int_add_2607(fails_85, t_203)
    fails_84: 'str36' = int_to_string_2602(listed_reduce_from_2608(test_results_80, 0, fn_352))
    totals_88: 'str36' = str_cat_2601("tests='", total_83, "' failures='", fails_84, "'")
    write_line_81(str_cat_2601("  <testsuite name='suite' ", totals_88, " time='0.0'>"))
    i_89: 'int42' = 0
    while True:
        t_360 = len_2599(test_results_80)
        if not i_89 < t_360:
            break
        test_result_90: 'Pair64[str36, (Sequence45[str36])]' = list_get_2609(test_results_80, i_89)
        failure_messages_91: 'Sequence45[str36]' = test_result_90.value
        t_363 = test_result_90.key
        name_92: 'str36' = escape_xml_41(t_363)
        basics_93: 'str36' = str_cat_2601("name='", name_92, "' classname='", name_92, "' time='0.0'")
        if not failure_messages_91:
            write_line_81(str_cat_2601('    <testcase ', basics_93, ' />'))
        else:
            write_line_81(str_cat_2601('    <testcase ', basics_93, '>'))
            def fn_351(it_95: 'str36') -> 'str36':
                return it_95
            t_369 = list_join_2594(failure_messages_91, ', ', fn_351)
            message_94: 'str36' = escape_xml_41(t_369)
            write_line_81(str_cat_2601("      <failure message='", message_94, "' />"))
            write_line_81('    </testcase>')
        i_89 = int_add_2607(i_89, 1)
    write_line_81('  </testsuite>')
    write_line_81('</testsuites>')
def run_test_cases(test_cases_96: 'Sequence45[(Pair64[str36, (Callable67[[Test], None])])]') -> 'str36':
    report_98: 'list14[str36]' = ['']
    t_345: 'Sequence45[(Pair64[str36, (Sequence45[str36])])]' = process_test_cases(test_cases_96)
    def fn_343(line_99: 'str36') -> 'None':
        report_98.append(line_99)
        report_98.append('\n')
    report_test_results(t_345, fn_343)
    return ''.join(report_98)
def run_test(test_fun_100: 'Callable67[[Test], None]') -> 'None':
    test_102: 'Test' = Test()
    try:
        test_fun_100(test_102)
    except Exception46:
        def fn_337() -> 'str36':
            return 'bubble during test running'
        test_102.assert_(False, fn_337)
    test_102.soft_fail_to_hard()
