from temper_std.json import JsonAdapter, JsonProducer, JsonSyntaxTree, InterchangeContext, JsonString
from datetime import date as date62
from builtins import str as str36, int as int42, bool as bool41, list as list14, len as len1
from temper_core import cast_by_type as cast_by_type51, date_to_string as date_to_string58, date_from_iso_string as date_from_iso_string59, arith_int_mod as arith_int_mod60, int_to_string as int_to_string5, string_get as string_get22, string_next as string_next24, string_count_between as string_count_between61, int_sub as int_sub0
from typing import Sequence as Sequence45
from temper_std.json import JsonAdapter, JsonString
date_to_string_2579 = date_to_string58
date_from_iso_string_2580 = date_from_iso_string59
arith_int_mod_2581 = arith_int_mod60
int_to_string_2582 = int_to_string5
len_2583 = len1
string_get_2584 = string_get22
string_next_2586 = string_next24
string_count_between_2587 = string_count_between61
int_sub_2588 = int_sub0
class DateJsonAdapter_109(JsonAdapter['date62']):
    __slots__ = ()
    def encode_to_json(this_120, x_116: 'date62', p_117: 'JsonProducer') -> 'None':
        encode_to_json_90(x_116, p_117)
    def decode_from_json(this_121, t_118: 'JsonSyntaxTree', ic_119: 'InterchangeContext') -> 'date62':
        return decode_from_json_93(t_118, ic_119)
    def __init__(this_122) -> None:
        pass
# Type `std/temporal/`.Date connected to datetime.date
def encode_to_json_90(this_20: 'date62', p_91: 'JsonProducer') -> 'None':
    t_313: 'str36' = date_to_string_2579(this_20)
    p_91.string_value(t_313)
def decode_from_json_93(t_94: 'JsonSyntaxTree', ic_95: 'InterchangeContext') -> 'date62':
    t_190: 'JsonString'
    t_190 = cast_by_type51(t_94, JsonString)
    return date_from_iso_string_2580(t_190.content)
def json_adapter_124() -> 'JsonAdapter[date62]':
    return DateJsonAdapter_109()
days_in_month_34: 'Sequence45[int42]' = (0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)
def is_leap_year_32(year_41: 'int42') -> 'bool41':
    return_21: 'bool41'
    t_263: 'int42'
    if arith_int_mod_2581(year_41, 4) == 0:
        if arith_int_mod_2581(year_41, 100) != 0:
            return_21 = True
        else:
            t_263 = arith_int_mod_2581(year_41, 400)
            return_21 = t_263 == 0
    else:
        return_21 = False
    return return_21
def pad_to_33(min_width_43: 'int42', num_44: 'int42', sb_45: 'list14[str36]') -> 'None':
    "If the decimal representation of \\|num\\| is longer than [minWidth],\nthen appends that representation.\nOtherwise any sign for [num] followed by enough zeroes to bring the\nwhole length up to [minWidth].\n\n```temper\n// When the width is greater than the decimal's length,\n// we pad to that width.\n\"0123\" == do {\n  let sb = new StringBuilder();\n  padTo(4, 123, sb);\n  sb.toString()\n}\n\n// When the width is the same or lesser, we just use the string form.\n\"123\" == do {\n  let sb = new StringBuilder();\n  padTo(2, 123, sb);\n  sb.toString()\n}\n\n// The sign is always on the left.\n\"-01\" == do {\n  let sb = new StringBuilder();\n  padTo(3, -1, sb);\n  sb.toString()\n}\n```\n\nminWidth__43: Int32\n\nnum__44: Int32\n\nsb__45: builtins.list<String>\n"
    t_346: 'int42'
    t_348: 'int42'
    t_257: 'bool41'
    decimal_47: 'str36' = int_to_string_2582(num_44, 10)
    decimal_index_48: 'int42' = 0
    decimal_end_49: 'int42' = len_2583(decimal_47)
    if decimal_index_48 < decimal_end_49:
        t_346 = string_get_2584(decimal_47, decimal_index_48)
        t_257 = t_346 == 45
    else:
        t_257 = False
    if t_257:
        sb_45.append('-')
        t_348 = string_next_2586(decimal_47, decimal_index_48)
        decimal_index_48 = t_348
    t_349: 'int42' = string_count_between_2587(decimal_47, decimal_index_48, decimal_end_49)
    n_needed_50: 'int42' = int_sub_2588(min_width_43, t_349)
    while n_needed_50 > 0:
        sb_45.append('0')
        n_needed_50 = int_sub_2588(n_needed_50, 1)
    sb_45.append(decimal_47[decimal_index_48 : decimal_end_49])
day_of_week_lookup_table_leapy_35: 'Sequence45[int42]' = (0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6)
day_of_week_lookup_table_not_leapy_36: 'Sequence45[int42]' = (0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5)
