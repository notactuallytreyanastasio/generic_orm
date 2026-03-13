from abc import ABCMeta as ABCMeta35, abstractmethod as abstractmethod40
from builtins import str as str36, RuntimeError as RuntimeError37, bool as bool41, int as int42, float as float43, Exception as Exception46, list as list14, isinstance as isinstance50, len as len1, tuple as tuple20
from typing import Union as Union38, ClassVar as ClassVar39, Sequence as Sequence45, MutableSequence as MutableSequence47, Dict as Dict48, Any as Any52, TypeVar as TypeVar53, Generic as Generic54
from types import MappingProxyType as MappingProxyType44
from temper_core import Label as Label49, cast_by_type as cast_by_type51, string_from_code_point as string_from_code_point55, int_sub as int_sub0, list_get as list_get2, list_for_each as list_for_each3, mapped_for_each as mapped_for_each4, int_to_string as int_to_string5, int64_to_int32 as int64_to_int326, int64_to_float64 as int64_to_float647, float64_to_string as float64_to_string8, float64_to_int as float64_to_int9, float64_to_int64 as float64_to_int6410, string_to_int32 as string_to_int3211, string_to_float64 as string_to_float6412, string_to_int64 as string_to_int6413, list_get_or as list_get_or15, list_builder_set as list_builder_set16, int_add as int_add17, mapped_has as mapped_has18, map_builder_set as map_builder_set19, mapped_to_map as mapped_to_map21, string_get as string_get22, int_div as int_div23, string_next as string_next24, str_cat as str_cat25, int_mul as int_mul26, string_has_at_least as string_has_at_least27, require_string_index as require_string_index28, int64_mul as int64_mul29, int64_add as int64_add30, int64_sub as int64_sub31, int64_negate as int64_negate32, int64_to_int32_unsafe as int64_to_int32_unsafe33, float_not_eq as float_not_eq34
from math import nan as nan56, inf as inf57
int_sub_2527 = int_sub0
len_2528 = len1
list_get_2529 = list_get2
list_for_each_2530 = list_for_each3
mapped_for_each_2531 = mapped_for_each4
int_to_string_2532 = int_to_string5
int64_to_int32_2535 = int64_to_int326
int64_to_float64_2536 = int64_to_float647
float64_to_string_2537 = float64_to_string8
float64_to_int_2538 = float64_to_int9
float64_to_int64_2539 = float64_to_int6410
string_to_int32_2540 = string_to_int3211
string_to_float64_2541 = string_to_float6412
string_to_int64_2542 = string_to_int6413
list_2544 = list14
list_get_or_2546 = list_get_or15
list_builder_set_2547 = list_builder_set16
int_add_2554 = int_add17
mapped_has_2555 = mapped_has18
map_builder_set_2557 = map_builder_set19
tuple_2558 = tuple20
mapped_to_map_2559 = mapped_to_map21
string_get_2561 = string_get22
int_div_2562 = int_div23
string_next_2563 = string_next24
str_cat_2566 = str_cat25
int_mul_2568 = int_mul26
string_has_at_least_2569 = string_has_at_least27
require_string_index_2572 = require_string_index28
int64_mul_2573 = int64_mul29
int64_add_2574 = int64_add30
int64_sub_2575 = int64_sub31
int64_negate_2576 = int64_negate32
int64_to_int32_unsafe_2577 = int64_to_int32_unsafe33
float_not_eq_2578 = float_not_eq34
class InterchangeContext(metaclass = ABCMeta35):
    def get_header(this_78, header_name_376: 'str36') -> 'Union38[str36, None]':
        raise RuntimeError37()
class NullInterchangeContext(InterchangeContext):
    instance: ClassVar39['NullInterchangeContext']
    __slots__ = ()
    def get_header(this_79, header_name_379: 'str36') -> 'Union38[str36, None]':
        return None
    def __init__(this_188) -> None:
        pass
NullInterchangeContext.instance = NullInterchangeContext()
class JsonProducer(metaclass = ABCMeta35):
    @property
    @abstractmethod40
    def interchange_context(self) -> 'InterchangeContext':
        pass
    def start_object(this_80) -> 'None':
        raise RuntimeError37()
    def end_object(this_81) -> 'None':
        raise RuntimeError37()
    def object_key(this_82, key_389: 'str36') -> 'None':
        raise RuntimeError37()
    def start_array(this_83) -> 'None':
        raise RuntimeError37()
    def end_array(this_84) -> 'None':
        raise RuntimeError37()
    def null_value(this_85) -> 'None':
        raise RuntimeError37()
    def boolean_value(this_86, x_398: 'bool41') -> 'None':
        raise RuntimeError37()
    def int32_value(this_87, x_401: 'int42') -> 'None':
        raise RuntimeError37()
    def int64_value(this_88, x_404: 'int64_23') -> 'None':
        raise RuntimeError37()
    def float64_value(this_89, x_407: 'float43') -> 'None':
        raise RuntimeError37()
    def numeric_token_value(this_90, x_410: 'str36') -> 'None':
        'A number that fits the JSON number grammar to allow\ninterchange of numbers that are not easily representible\nusing numeric types that Temper connects to.\n\nthis__90: JsonProducer\n\nx__410: String\n'
        raise RuntimeError37()
    def string_value(this_91, x_413: 'str36') -> 'None':
        raise RuntimeError37()
    @property
    def parse_error_receiver(this_92) -> 'Union38[JsonParseErrorReceiver, None]':
        return None
class JsonSyntaxTree(metaclass = ABCMeta35):
    def produce(this_93, p_418: 'JsonProducer') -> 'None':
        raise RuntimeError37()
class JsonObject(JsonSyntaxTree):
    properties_420: 'MappingProxyType44[str36, (Sequence45[JsonSyntaxTree])]'
    __slots__ = ('properties_420',)
    def property_value_or_null(this_94, property_key_422: 'str36') -> 'Union38[JsonSyntaxTree, None]':
        "The JSON value tree associated with the given property key or null\nif there is no such value.\n\nThe properties map contains a list of sub-trees because JSON\nallows duplicate properties.  ECMA-404 \xa76 notes (emphasis added):\n\n> The JSON syntax does not impose any restrictions on the strings\n> used as names, **does not require that name strings be unique**,\n> and does not assign any significance to the ordering of\n> name/value pairs.\n\nWhen widely used JSON parsers need to relate a property key\nto a single value, they tend to prefer the last key/value pair\nfrom a JSON object.  For example:\n\nJS:\n\n    JSON.parse('{\"x\":\"first\",\"x\":\"last\"}').x === 'last'\n\nPython:\n\n    import json\n    json.loads('{\"x\":\"first\",\"x\":\"last\"}')['x'] == 'last'\n\nC#:\n\n   using System.Text.Json;\n\t\tJsonDocument d = JsonDocument.Parse(\n\t\t\t\"\"\"\n\t\t\t{\"x\":\"first\",\"x\":\"last\"}\n\t\t\t\"\"\"\n\t\t);\n\t\td.RootElement.GetProperty(\"x\").GetString() == \"last\"\n\nthis__94: JsonObject\n\npropertyKey__422: String\n"
        return_209: 'Union38[JsonSyntaxTree, None]'
        tree_list_424: 'Sequence45[JsonSyntaxTree]' = this_94.properties_420.get(property_key_422, ())
        last_index_425: 'int42' = int_sub_2527(len_2528(tree_list_424), 1)
        if last_index_425 >= 0:
            return_209 = list_get_2529(tree_list_424, last_index_425)
        else:
            return_209 = None
        return return_209
    def property_value_or_bubble(this_95, property_key_427: 'str36') -> 'JsonSyntaxTree':
        return_210: 'JsonSyntaxTree'
        t_2518: 'Union38[JsonSyntaxTree, None]' = this_95.property_value_or_null(property_key_427)
        if t_2518 is None:
            raise RuntimeError37()
        else:
            return_210 = t_2518
        return return_210
    def produce(this_96, p_430: 'JsonProducer') -> 'None':
        p_430.start_object()
        def fn_2513(k_432: 'str36', vs_433: 'Sequence45[JsonSyntaxTree]') -> 'None':
            def fn_2510(v_434: 'JsonSyntaxTree') -> 'None':
                p_430.object_key(k_432)
                v_434.produce(p_430)
            list_for_each_2530(vs_433, fn_2510)
        mapped_for_each_2531(this_96.properties_420, fn_2513)
        p_430.end_object()
    def __init__(this_206, properties_436: 'MappingProxyType44[str36, (Sequence45[JsonSyntaxTree])]') -> None:
        this_206.properties_420 = properties_436
    @property
    def properties(this_875) -> 'MappingProxyType44[str36, (Sequence45[JsonSyntaxTree])]':
        return this_875.properties_420
class JsonArray(JsonSyntaxTree):
    elements_437: 'Sequence45[JsonSyntaxTree]'
    __slots__ = ('elements_437',)
    def produce(this_97, p_439: 'JsonProducer') -> 'None':
        p_439.start_array()
        def fn_2503(v_441: 'JsonSyntaxTree') -> 'None':
            v_441.produce(p_439)
        list_for_each_2530(this_97.elements_437, fn_2503)
        p_439.end_array()
    def __init__(this_212, elements_443: 'Sequence45[JsonSyntaxTree]') -> None:
        this_212.elements_437 = elements_443
    @property
    def elements(this_878) -> 'Sequence45[JsonSyntaxTree]':
        return this_878.elements_437
class JsonBoolean(JsonSyntaxTree):
    content_444: 'bool41'
    __slots__ = ('content_444',)
    def produce(this_98, p_446: 'JsonProducer') -> 'None':
        p_446.boolean_value(this_98.content_444)
    def __init__(this_216, content_449: 'bool41') -> None:
        this_216.content_444 = content_449
    @property
    def content(this_881) -> 'bool41':
        return this_881.content_444
class JsonNull(JsonSyntaxTree):
    __slots__ = ()
    def produce(this_99, p_451: 'JsonProducer') -> 'None':
        p_451.null_value()
    def __init__(this_219) -> None:
        pass
class JsonString(JsonSyntaxTree):
    content_454: 'str36'
    __slots__ = ('content_454',)
    def produce(this_100, p_456: 'JsonProducer') -> 'None':
        p_456.string_value(this_100.content_454)
    def __init__(this_222, content_459: 'str36') -> None:
        this_222.content_454 = content_459
    @property
    def content(this_884) -> 'str36':
        return this_884.content_454
class JsonNumeric(JsonSyntaxTree, metaclass = ABCMeta35):
    def as_json_numeric_token(this_101) -> 'str36':
        raise RuntimeError37()
    def as_int32(this_102) -> 'int42':
        raise RuntimeError37()
    def as_int64(this_103) -> 'int64_23':
        raise RuntimeError37()
    def as_float64(this_104) -> 'float43':
        raise RuntimeError37()
class JsonInt32(JsonNumeric):
    content_468: 'int42'
    __slots__ = ('content_468',)
    def produce(this_105, p_470: 'JsonProducer') -> 'None':
        p_470.int32_value(this_105.content_468)
    def as_json_numeric_token(this_106) -> 'str36':
        return int_to_string_2532(this_106.content_468)
    def as_int32(this_107) -> 'int42':
        return this_107.content_468
    def as_int64(this_108) -> 'int64_23':
        return this_108.content_468
    def as_float64(this_109) -> 'float43':
        return float(this_109.content_468)
    def __init__(this_229, content_481: 'int42') -> None:
        this_229.content_468 = content_481
    @property
    def content(this_887) -> 'int42':
        return this_887.content_468
class JsonInt64(JsonNumeric):
    content_482: 'int64_23'
    __slots__ = ('content_482',)
    def produce(this_110, p_484: 'JsonProducer') -> 'None':
        p_484.int64_value(this_110.content_482)
    def as_json_numeric_token(this_111) -> 'str36':
        return int_to_string_2532(this_111.content_482)
    def as_int32(this_112) -> 'int42':
        return int64_to_int32_2535(this_112.content_482)
    def as_int64(this_113) -> 'int64_23':
        return this_113.content_482
    def as_float64(this_114) -> 'float43':
        return int64_to_float64_2536(this_114.content_482)
    def __init__(this_236, content_495: 'int64_23') -> None:
        this_236.content_482 = content_495
    @property
    def content(this_890) -> 'int64_23':
        return this_890.content_482
class JsonFloat64(JsonNumeric):
    content_496: 'float43'
    __slots__ = ('content_496',)
    def produce(this_115, p_498: 'JsonProducer') -> 'None':
        p_498.float64_value(this_115.content_496)
    def as_json_numeric_token(this_116) -> 'str36':
        return float64_to_string_2537(this_116.content_496)
    def as_int32(this_117) -> 'int42':
        return float64_to_int_2538(this_117.content_496)
    def as_int64(this_118) -> 'int64_23':
        return float64_to_int64_2539(this_118.content_496)
    def as_float64(this_119) -> 'float43':
        return this_119.content_496
    def __init__(this_243, content_509: 'float43') -> None:
        this_243.content_496 = content_509
    @property
    def content(this_893) -> 'float43':
        return this_893.content_496
class JsonNumericToken(JsonNumeric):
    content_510: 'str36'
    __slots__ = ('content_510',)
    def produce(this_120, p_512: 'JsonProducer') -> 'None':
        p_512.numeric_token_value(this_120.content_510)
    def as_json_numeric_token(this_121) -> 'str36':
        return this_121.content_510
    def as_int32(this_122) -> 'int42':
        return_254: 'int42'
        t_1774: 'int42'
        t_1775: 'float43'
        try:
            t_1774 = string_to_int32_2540(this_122.content_510)
            return_254 = t_1774
        except Exception46:
            t_1775 = string_to_float64_2541(this_122.content_510)
            return_254 = float64_to_int_2538(t_1775)
        return return_254
    def as_int64(this_123) -> 'int64_23':
        return_255: 'int64_23'
        t_1770: 'int64_23'
        t_1771: 'float43'
        try:
            t_1770 = string_to_int64_2542(this_123.content_510)
            return_255 = t_1770
        except Exception46:
            t_1771 = string_to_float64_2541(this_123.content_510)
            return_255 = float64_to_int64_2539(t_1771)
        return return_255
    def as_float64(this_124) -> 'float43':
        return string_to_float64_2541(this_124.content_510)
    def __init__(this_250, content_523: 'str36') -> None:
        this_250.content_510 = content_523
    @property
    def content(this_896) -> 'str36':
        return this_896.content_510
class JsonTextProducer(JsonProducer):
    interchange_context_524: 'InterchangeContext'
    buffer_525: 'list14[str36]'
    stack_526: 'MutableSequence47[int42]'
    well_formed_527: 'bool41'
    __slots__ = ('interchange_context_524', 'buffer_525', 'stack_526', 'well_formed_527')
    def __init__(this_125, interchange_context_938: 'Union38[InterchangeContext, None]' = None) -> None:
        _interchange_context_938: 'Union38[InterchangeContext, None]' = interchange_context_938
        interchange_context_529: 'InterchangeContext'
        if _interchange_context_938 is None:
            interchange_context_529 = NullInterchangeContext.instance
        else:
            interchange_context_529 = _interchange_context_938
        this_125.interchange_context_524 = interchange_context_529
        t_2470: 'list14[str36]' = ['']
        this_125.buffer_525 = t_2470
        t_2471: 'MutableSequence47[int42]' = list_2544()
        this_125.stack_526 = t_2471
        this_125.stack_526.append(5)
        this_125.well_formed_527 = True
    def state_531(this_126) -> 'int42':
        t_2468: 'int42' = len_2528(this_126.stack_526)
        return list_get_or_2546(this_126.stack_526, int_sub_2527(t_2468, 1), -1)
    def before_value_533(this_127) -> 'None':
        t_2461: 'int42'
        t_2464: 'int42'
        t_2466: 'int42'
        t_1728: 'bool41'
        current_state_535: 'int42' = this_127.state_531()
        if current_state_535 == 3:
            t_2461 = len_2528(this_127.stack_526)
            list_builder_set_2547(this_127.stack_526, int_sub_2527(t_2461, 1), 4)
        elif current_state_535 == 4:
            this_127.buffer_525.append(',')
        elif current_state_535 == 1:
            t_2464 = len_2528(this_127.stack_526)
            list_builder_set_2547(this_127.stack_526, int_sub_2527(t_2464, 1), 2)
        elif current_state_535 == 5:
            t_2466 = len_2528(this_127.stack_526)
            list_builder_set_2547(this_127.stack_526, int_sub_2527(t_2466, 1), 6)
        else:
            if current_state_535 == 6:
                t_1728 = True
            else:
                t_1728 = current_state_535 == 2
            if t_1728:
                this_127.well_formed_527 = False
    def start_object(this_128) -> 'None':
        this_128.before_value_533()
        this_128.buffer_525.append('{')
        this_128.stack_526.append(0)
    def end_object(this_129) -> 'None':
        t_1716: 'bool41'
        this_129.buffer_525.append('}')
        current_state_540: 'int42' = this_129.state_531()
        if 0 == current_state_540:
            t_1716 = True
        else:
            t_1716 = 2 == current_state_540
        if t_1716:
            this_129.stack_526.pop()
        else:
            this_129.well_formed_527 = False
    def object_key(this_130, key_542: 'str36') -> 'None':
        t_2452: 'int42'
        current_state_544: 'int42' = this_130.state_531()
        if not current_state_544 == 0:
            if current_state_544 == 2:
                this_130.buffer_525.append(',')
            else:
                this_130.well_formed_527 = False
        encode_json_string_351(key_542, this_130.buffer_525)
        this_130.buffer_525.append(':')
        if current_state_544 >= 0:
            t_2452 = len_2528(this_130.stack_526)
            list_builder_set_2547(this_130.stack_526, int_sub_2527(t_2452, 1), 1)
    def start_array(this_131) -> 'None':
        this_131.before_value_533()
        this_131.buffer_525.append('[')
        this_131.stack_526.append(3)
    def end_array(this_132) -> 'None':
        t_1704: 'bool41'
        this_132.buffer_525.append(']')
        current_state_549: 'int42' = this_132.state_531()
        if 3 == current_state_549:
            t_1704 = True
        else:
            t_1704 = 4 == current_state_549
        if t_1704:
            this_132.stack_526.pop()
        else:
            this_132.well_formed_527 = False
    def null_value(this_133) -> 'None':
        this_133.before_value_533()
        this_133.buffer_525.append('null')
    def boolean_value(this_134, x_553: 'bool41') -> 'None':
        t_1700: 'str36'
        this_134.before_value_533()
        if x_553:
            t_1700 = 'true'
        else:
            t_1700 = 'false'
        this_134.buffer_525.append(t_1700)
    def int32_value(this_135, x_556: 'int42') -> 'None':
        this_135.before_value_533()
        t_2436: 'str36' = int_to_string_2532(x_556)
        this_135.buffer_525.append(t_2436)
    def int64_value(this_136, x_559: 'int64_23') -> 'None':
        this_136.before_value_533()
        t_2433: 'str36' = int_to_string_2532(x_559)
        this_136.buffer_525.append(t_2433)
    def float64_value(this_137, x_562: 'float43') -> 'None':
        this_137.before_value_533()
        t_2430: 'str36' = float64_to_string_2537(x_562)
        this_137.buffer_525.append(t_2430)
    def numeric_token_value(this_138, x_565: 'str36') -> 'None':
        this_138.before_value_533()
        this_138.buffer_525.append(x_565)
    def string_value(this_139, x_568: 'str36') -> 'None':
        this_139.before_value_533()
        encode_json_string_351(x_568, this_139.buffer_525)
    def to_json_string(this_140) -> 'str36':
        return_272: 'str36'
        t_2423: 'int42'
        t_1689: 'bool41'
        t_1690: 'bool41'
        if this_140.well_formed_527:
            if len_2528(this_140.stack_526) == 1:
                t_2423 = this_140.state_531()
                t_1689 = t_2423 == 6
            else:
                t_1689 = False
            t_1690 = t_1689
        else:
            t_1690 = False
        if t_1690:
            return_272 = ''.join(this_140.buffer_525)
        else:
            raise RuntimeError37()
        return return_272
    @property
    def interchange_context(this_906) -> 'InterchangeContext':
        return this_906.interchange_context_524
class JsonParseErrorReceiver(metaclass = ABCMeta35):
    def explain_json_error(this_141, explanation_588: 'str36') -> 'None':
        raise RuntimeError37()
class JsonSyntaxTreeProducer(JsonProducer, JsonParseErrorReceiver):
    stack_590: 'MutableSequence47[(MutableSequence47[JsonSyntaxTree])]'
    error_591: 'Union38[str36, None]'
    __slots__ = ('stack_590', 'error_591')
    @property
    def interchange_context(this_142) -> 'InterchangeContext':
        return NullInterchangeContext.instance
    def __init__(this_143) -> None:
        t_2419: 'MutableSequence47[(MutableSequence47[JsonSyntaxTree])]' = list_2544()
        this_143.stack_590 = t_2419
        t_2420: 'MutableSequence47[JsonSyntaxTree]' = list_2544()
        this_143.stack_590.append(t_2420)
        this_143.error_591 = None
    def store_value_596(this_144, v_597: 'JsonSyntaxTree') -> 'None':
        t_2416: 'int42'
        if not (not this_144.stack_590):
            t_2416 = len_2528(this_144.stack_590)
            list_get_2529(this_144.stack_590, int_sub_2527(t_2416, 1)).append(v_597)
    def start_object(this_145) -> 'None':
        t_2413: 'MutableSequence47[JsonSyntaxTree]' = list_2544()
        this_145.stack_590.append(t_2413)
    def end_object(this_146) -> 'None':
        t_2402: 'Union38[(Dict48[str36, (MutableSequence47[JsonSyntaxTree])]), None]'
        t_2411: 'JsonObject'
        t_1656: 'JsonString'
        t_1658: 'JsonString'
        t_1664: 'Dict48[str36, (MutableSequence47[JsonSyntaxTree])]'
        t_1666: 'Dict48[str36, (MutableSequence47[JsonSyntaxTree])]'
        t_1668: 'Sequence45[JsonSyntaxTree]'
        t_1669: 'Sequence45[JsonSyntaxTree]'
        t_1671: 'MutableSequence47[JsonSyntaxTree]'
        t_1672: 'MutableSequence47[JsonSyntaxTree]'
        with Label49() as fn_602:
            if not this_146.stack_590:
                fn_602.break_()
            ls_603: 'MutableSequence47[JsonSyntaxTree]' = this_146.stack_590.pop()
            m_604: 'Dict48[str36, (Sequence45[JsonSyntaxTree])]' = {}
            multis_605: 'Union38[(Dict48[str36, (MutableSequence47[JsonSyntaxTree])]), None]' = None
            i_606: 'int42' = 0
            n_607: 'int42' = len_2528(ls_603) & -2
            while i_606 < n_607:
                postfix_return_40: 'int42' = i_606
                i_606 = int_add_2554(i_606, 1)
                key_tree_608: 'JsonSyntaxTree' = list_get_2529(ls_603, postfix_return_40)
                if not isinstance50(key_tree_608, JsonString):
                    break
                t_1656 = cast_by_type51(key_tree_608, JsonString)
                t_1658 = t_1656
                key_609: 'str36' = t_1658.content
                postfix_return_41: 'int42' = i_606
                i_606 = int_add_2554(i_606, 1)
                value_610: 'JsonSyntaxTree' = list_get_2529(ls_603, postfix_return_41)
                if mapped_has_2555(m_604, key_609):
                    if multis_605 is None:
                        t_2402 = {}
                        multis_605 = t_2402
                    if multis_605 is None:
                        raise RuntimeError37()
                    else:
                        t_1664 = multis_605
                    t_1666 = t_1664
                    mb_611: 'Dict48[str36, (MutableSequence47[JsonSyntaxTree])]' = t_1666
                    if not mapped_has_2555(mb_611, key_609):
                        t_1668 = m_604[key_609]
                        t_1669 = t_1668
                        map_builder_set_2557(mb_611, key_609, list_2544(t_1669))
                    t_1671 = mb_611[key_609]
                    t_1672 = t_1671
                    t_1672.append(value_610)
                else:
                    map_builder_set_2557(m_604, key_609, (value_610,))
            multis_612: 'Union38[(Dict48[str36, (MutableSequence47[JsonSyntaxTree])]), None]' = multis_605
            if not multis_612 is None:
                def fn_2392(k_613: 'str36', vs_614: 'MutableSequence47[JsonSyntaxTree]') -> 'None':
                    t_2390: 'Sequence45[JsonSyntaxTree]' = tuple_2558(vs_614)
                    map_builder_set_2557(m_604, k_613, t_2390)
                mapped_for_each_2531(multis_612, fn_2392)
            t_2411 = JsonObject(mapped_to_map_2559(m_604))
            this_146.store_value_596(t_2411)
    def object_key(this_147, key_616: 'str36') -> 'None':
        t_2388: 'JsonString' = JsonString(key_616)
        this_147.store_value_596(t_2388)
    def start_array(this_148) -> 'None':
        t_2386: 'MutableSequence47[JsonSyntaxTree]' = list_2544()
        this_148.stack_590.append(t_2386)
    def end_array(this_149) -> 'None':
        t_2384: 'JsonArray'
        with Label49() as fn_621:
            if not this_149.stack_590:
                fn_621.break_()
            ls_622: 'MutableSequence47[JsonSyntaxTree]' = this_149.stack_590.pop()
            t_2384 = JsonArray(tuple_2558(ls_622))
            this_149.store_value_596(t_2384)
    def null_value(this_150) -> 'None':
        t_2379: 'JsonNull' = JsonNull()
        this_150.store_value_596(t_2379)
    def boolean_value(this_151, x_626: 'bool41') -> 'None':
        t_2377: 'JsonBoolean' = JsonBoolean(x_626)
        this_151.store_value_596(t_2377)
    def int32_value(this_152, x_629: 'int42') -> 'None':
        t_2375: 'JsonInt32' = JsonInt32(x_629)
        this_152.store_value_596(t_2375)
    def int64_value(this_153, x_632: 'int64_23') -> 'None':
        t_2373: 'JsonInt64' = JsonInt64(x_632)
        this_153.store_value_596(t_2373)
    def float64_value(this_154, x_635: 'float43') -> 'None':
        t_2371: 'JsonFloat64' = JsonFloat64(x_635)
        this_154.store_value_596(t_2371)
    def numeric_token_value(this_155, x_638: 'str36') -> 'None':
        t_2369: 'JsonNumericToken' = JsonNumericToken(x_638)
        this_155.store_value_596(t_2369)
    def string_value(this_156, x_641: 'str36') -> 'None':
        t_2367: 'JsonString' = JsonString(x_641)
        this_156.store_value_596(t_2367)
    def to_json_syntax_tree(this_157) -> 'JsonSyntaxTree':
        t_1629: 'bool41'
        if len_2528(this_157.stack_590) != 1:
            t_1629 = True
        else:
            t_1629 = not this_157.error_591 is None
        if t_1629:
            raise RuntimeError37()
        ls_645: 'MutableSequence47[JsonSyntaxTree]' = list_get_2529(this_157.stack_590, 0)
        if len_2528(ls_645) != 1:
            raise RuntimeError37()
        return list_get_2529(ls_645, 0)
    @property
    def json_error(this_158) -> 'Union38[str36, None]':
        return this_158.error_591
    @property
    def parse_error_receiver(this_159) -> 'JsonParseErrorReceiver':
        return this_159
    def explain_json_error(this_160, error_651: 'str36') -> 'None':
        this_160.error_591 = error_651
def parse_json_value_356(source_text_670: 'str36', i_671: 'int42', out_672: 'JsonProducer') -> 'int42':
    return_302: 'int42'
    t_2201: 'int42'
    t_2204: 'int42'
    t_1412: 'bool41'
    with Label49() as fn_673:
        t_2201 = skip_json_spaces_355(source_text_670, i_671)
        i_671 = t_2201
        if not len1(source_text_670) > i_671:
            expected_token_error_353(source_text_670, i_671, out_672, 'JSON value')
            return_302 = -1
            fn_673.break_()
        t_2204 = string_get_2561(source_text_670, i_671)
        if t_2204 == 123:
            return_302 = parse_json_object_357(source_text_670, i_671, out_672)
        elif t_2204 == 91:
            return_302 = parse_json_array_358(source_text_670, i_671, out_672)
        elif t_2204 == 34:
            return_302 = parse_json_string_359(source_text_670, i_671, out_672)
        else:
            if t_2204 == 116:
                t_1412 = True
            else:
                t_1412 = t_2204 == 102
            if t_1412:
                return_302 = parse_json_boolean_362(source_text_670, i_671, out_672)
            elif t_2204 == 110:
                return_302 = parse_json_null_363(source_text_670, i_671, out_672)
            else:
                return_302 = parse_json_number_365(source_text_670, i_671, out_672)
    return return_302
T_161 = TypeVar53('T_161', bound = Any52, covariant = True)
class JsonAdapter(Generic54[T_161], metaclass = ABCMeta35):
    def encode_to_json(this_162, x_765: 'T_161', p_766: 'JsonProducer') -> 'None':
        raise RuntimeError37()
    def decode_from_json(this_163, t_769: 'JsonSyntaxTree', ic_770: 'InterchangeContext') -> 'T_161':
        raise RuntimeError37()
class BooleanJsonAdapter_164(JsonAdapter['bool41']):
    __slots__ = ()
    def encode_to_json(this_165, x_773: 'bool41', p_774: 'JsonProducer') -> 'None':
        p_774.boolean_value(x_773)
    def decode_from_json(this_166, t_777: 'JsonSyntaxTree', ic_778: 'InterchangeContext') -> 'bool41':
        t_1391: 'JsonBoolean'
        t_1391 = cast_by_type51(t_777, JsonBoolean)
        return t_1391.content
    def __init__(this_315) -> None:
        pass
class Float64JsonAdapter_167(JsonAdapter['float43']):
    __slots__ = ()
    def encode_to_json(this_168, x_783: 'float43', p_784: 'JsonProducer') -> 'None':
        p_784.float64_value(x_783)
    def decode_from_json(this_169, t_787: 'JsonSyntaxTree', ic_788: 'InterchangeContext') -> 'float43':
        t_1387: 'JsonNumeric'
        t_1387 = cast_by_type51(t_787, JsonNumeric)
        return t_1387.as_float64()
    def __init__(this_320) -> None:
        pass
class Int32JsonAdapter_170(JsonAdapter['int42']):
    __slots__ = ()
    def encode_to_json(this_171, x_793: 'int42', p_794: 'JsonProducer') -> 'None':
        p_794.int32_value(x_793)
    def decode_from_json(this_172, t_797: 'JsonSyntaxTree', ic_798: 'InterchangeContext') -> 'int42':
        t_1383: 'JsonNumeric'
        t_1383 = cast_by_type51(t_797, JsonNumeric)
        return t_1383.as_int32()
    def __init__(this_325) -> None:
        pass
class Int64JsonAdapter_173(JsonAdapter['int64_23']):
    __slots__ = ()
    def encode_to_json(this_174, x_803: 'int64_23', p_804: 'JsonProducer') -> 'None':
        p_804.int64_value(x_803)
    def decode_from_json(this_175, t_807: 'JsonSyntaxTree', ic_808: 'InterchangeContext') -> 'int64_23':
        t_1379: 'JsonNumeric'
        t_1379 = cast_by_type51(t_807, JsonNumeric)
        return t_1379.as_int64()
    def __init__(this_330) -> None:
        pass
class StringJsonAdapter_176(JsonAdapter['str36']):
    __slots__ = ()
    def encode_to_json(this_177, x_813: 'str36', p_814: 'JsonProducer') -> 'None':
        p_814.string_value(x_813)
    def decode_from_json(this_178, t_817: 'JsonSyntaxTree', ic_818: 'InterchangeContext') -> 'str36':
        t_1375: 'JsonString'
        t_1375 = cast_by_type51(t_817, JsonString)
        return t_1375.content
    def __init__(this_335) -> None:
        pass
T_180 = TypeVar53('T_180', bound = Any52)
class ListJsonAdapter_179(JsonAdapter['Sequence45[T_180]']):
    adapter_for_t_822: 'JsonAdapter[T_180]'
    __slots__ = ('adapter_for_t_822',)
    def encode_to_json(this_181, x_824: 'Sequence45[T_180]', p_825: 'JsonProducer') -> 'None':
        p_825.start_array()
        def fn_2174(el_827: 'T_180') -> 'None':
            this_181.adapter_for_t_822.encode_to_json(el_827, p_825)
        list_for_each_2530(x_824, fn_2174)
        p_825.end_array()
    def decode_from_json(this_182, t_829: 'JsonSyntaxTree', ic_830: 'InterchangeContext') -> 'Sequence45[T_180]':
        t_1369: 'T_180'
        b_832: 'MutableSequence47[T_180]' = list_2544()
        t_1364: 'JsonArray'
        t_1364 = cast_by_type51(t_829, JsonArray)
        elements_833: 'Sequence45[JsonSyntaxTree]' = t_1364.elements
        n_834: 'int42' = len_2528(elements_833)
        i_835: 'int42' = 0
        while i_835 < n_834:
            el_836: 'JsonSyntaxTree' = list_get_2529(elements_833, i_835)
            i_835 = int_add_2554(i_835, 1)
            t_1369 = this_182.adapter_for_t_822.decode_from_json(el_836, ic_830)
            b_832.append(t_1369)
        return tuple_2558(b_832)
    def __init__(this_340, adapter_for_t_838: 'JsonAdapter[T_180]') -> None:
        this_340.adapter_for_t_822 = adapter_for_t_838
T_184 = TypeVar53('T_184', bound = Any52)
class OrNullJsonAdapter(JsonAdapter['Union38[T_184, None]']):
    adapter_for_t_841: 'JsonAdapter[T_184]'
    __slots__ = ('adapter_for_t_841',)
    def encode_to_json(this_185, x_843: 'Union38[T_184, None]', p_844: 'JsonProducer') -> 'None':
        if x_843 is None:
            p_844.null_value()
        else:
            x_962: 'T_184' = x_843
            this_185.adapter_for_t_841.encode_to_json(x_962, p_844)
    def decode_from_json(this_186, t_847: 'JsonSyntaxTree', ic_848: 'InterchangeContext') -> 'Union38[T_184, None]':
        return_350: 'Union38[T_184, None]'
        if isinstance50(t_847, JsonNull):
            return_350 = None
        else:
            return_350 = this_186.adapter_for_t_841.decode_from_json(t_847, ic_848)
        return return_350
    def __init__(this_346, adapter_for_t_851: 'JsonAdapter[T_184]') -> None:
        this_346.adapter_for_t_841 = adapter_for_t_851
hex_digits_373: 'Sequence45[str36]' = ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f')
def encode_hex4_352(cp_580: 'int42', buffer_581: 'list14[str36]') -> 'None':
    b0_583: 'int42' = int_div_2562(cp_580, 4096) & 15
    b1_584: 'int42' = int_div_2562(cp_580, 256) & 15
    b2_585: 'int42' = int_div_2562(cp_580, 16) & 15
    b3_586: 'int42' = cp_580 & 15
    t_2482: 'str36' = list_get_2529(hex_digits_373, b0_583)
    buffer_581.append(t_2482)
    t_2484: 'str36' = list_get_2529(hex_digits_373, b1_584)
    buffer_581.append(t_2484)
    t_2486: 'str36' = list_get_2529(hex_digits_373, b2_585)
    buffer_581.append(t_2486)
    t_2488: 'str36' = list_get_2529(hex_digits_373, b3_586)
    buffer_581.append(t_2488)
def encode_json_string_351(x_572: 'str36', buffer_573: 'list14[str36]') -> 'None':
    t_1745: 'bool41'
    t_1746: 'bool41'
    t_1747: 'str36'
    t_1748: 'str36'
    buffer_573.append('"')
    i_575: 'int42' = 0
    emitted_576: 'int42' = i_575
    while True:
        if not len1(x_572) > i_575:
            break
        cp_577: 'int42' = string_get_2561(x_572, i_575)
        if cp_577 == 8:
            t_1748 = '\\b'
        elif cp_577 == 9:
            t_1748 = '\\t'
        elif cp_577 == 10:
            t_1748 = '\\n'
        elif cp_577 == 12:
            t_1748 = '\\f'
        elif cp_577 == 13:
            t_1748 = '\\r'
        elif cp_577 == 34:
            t_1748 = '\\"'
        elif cp_577 == 92:
            t_1748 = '\\\\'
        else:
            if cp_577 < 32:
                t_1746 = True
            else:
                if 55296 <= cp_577:
                    t_1745 = cp_577 <= 57343
                else:
                    t_1745 = False
                t_1746 = t_1745
            if t_1746:
                t_1747 = '\\u'
            else:
                t_1747 = ''
            t_1748 = t_1747
        replacement_578: 'str36' = t_1748
        next_i_579: 'int42' = string_next_2563(x_572, i_575)
        if replacement_578 != '':
            buffer_573.append(x_572[emitted_576 : i_575])
            buffer_573.append(replacement_578)
            if replacement_578 == '\\u':
                encode_hex4_352(cp_577, buffer_573)
            emitted_576 = next_i_579
        i_575 = next_i_579
    buffer_573.append(x_572[emitted_576 : i_575])
    buffer_573.append('"')
def store_json_error_354(out_659: 'JsonProducer', explanation_660: 'str36') -> 'None':
    t_2361: 'Union38[JsonParseErrorReceiver, None]' = out_659.parse_error_receiver
    if not t_2361 is None:
        t_2361.explain_json_error(explanation_660)
def expected_token_error_353(source_text_653: 'str36', i_654: 'int42', out_655: 'JsonProducer', short_explanation_656: 'str36') -> 'None':
    t_2358: 'int42'
    t_2359: 'str36'
    gotten_658: 'str36'
    if len1(source_text_653) > i_654:
        t_2358 = len_2528(source_text_653)
        t_2359 = source_text_653[i_654 : t_2358]
        gotten_658 = str_cat_2566('`', t_2359, '`')
    else:
        gotten_658 = 'end-of-file'
    store_json_error_354(out_655, str_cat_2566('Expected ', short_explanation_656, ', but got ', gotten_658))
def skip_json_spaces_355(source_text_667: 'str36', i_668: 'int42') -> 'int42':
    t_2355: 'int42'
    t_2356: 'int42'
    t_1616: 'bool41'
    t_1617: 'bool41'
    t_1618: 'bool41'
    while True:
        if not len1(source_text_667) > i_668:
            break
        t_2355 = string_get_2561(source_text_667, i_668)
        if t_2355 == 9:
            t_1618 = True
        else:
            if t_2355 == 10:
                t_1617 = True
            else:
                if t_2355 == 13:
                    t_1616 = True
                else:
                    t_1616 = t_2355 == 32
                t_1617 = t_1616
            t_1618 = t_1617
        if not t_1618:
            break
        t_2356 = string_next_2563(source_text_667, i_668)
        i_668 = t_2356
    return i_668
def decode_hex_unsigned_361(source_text_708: 'str36', start_709: 'int42', limit_710: 'int42') -> 'int42':
    return_307: 'int42'
    t_2353: 'int42'
    t_1609: 'bool41'
    t_1610: 'bool41'
    t_1611: 'bool41'
    t_1612: 'int42'
    with Label49() as fn_711:
        n_712: 'int42' = 0
        i_713: 'int42' = start_709
        while True:
            if not i_713 - limit_710 < 0:
                break
            cp_714: 'int42' = string_get_2561(source_text_708, i_713)
            if 48 <= cp_714:
                t_1609 = cp_714 <= 48
            else:
                t_1609 = False
            if t_1609:
                t_1612 = int_sub_2527(cp_714, 48)
            else:
                if 65 <= cp_714:
                    t_1610 = cp_714 <= 70
                else:
                    t_1610 = False
                if t_1610:
                    t_1612 = int_add_2554(int_sub_2527(cp_714, 65), 10)
                else:
                    if 97 <= cp_714:
                        t_1611 = cp_714 <= 102
                    else:
                        t_1611 = False
                    if t_1611:
                        t_1612 = int_add_2554(int_sub_2527(cp_714, 97), 10)
                    else:
                        return_307 = -1
                        fn_711.break_()
            digit_715: 'int42' = t_1612
            n_712 = int_add_2554(int_mul_2568(n_712, 16), digit_715)
            t_2353 = string_next_2563(source_text_708, i_713)
            i_713 = t_2353
        return_307 = n_712
    return return_307
def parse_json_string_to_360(source_text_692: 'str36', i_693: 'int42', sb_694: 'list14[str36]', err_out_695: 'JsonProducer') -> 'int42':
    return_306: 'int42'
    t_2326: 'int42'
    t_2328: 'int42'
    t_2331: 'int42'
    t_2336: 'int42'
    t_2338: 'int42'
    t_2339: 'int42'
    t_2340: 'int42'
    t_2341: 'int42'
    t_2342: 'int42'
    t_2347: 'int42'
    t_2350: 'int42'
    t_1570: 'bool41'
    t_1579: 'bool41'
    t_1580: 'bool41'
    t_1588: 'int42'
    t_1589: 'int42'
    t_1591: 'int42'
    t_1593: 'int42'
    t_1594: 'bool41'
    t_1595: 'bool41'
    t_1597: 'bool41'
    t_1601: 'bool41'
    with Label49() as fn_696:
        if not len1(source_text_692) > i_693:
            t_1570 = True
        else:
            t_2326 = string_get_2561(source_text_692, i_693)
            t_1570 = t_2326 != 34
        if t_1570:
            expected_token_error_353(source_text_692, i_693, err_out_695, '"')
            return_306 = -1
            fn_696.break_()
        t_2328 = string_next_2563(source_text_692, i_693)
        i_693 = t_2328
        lead_surrogate_697: 'int42' = -1
        consumed_698: 'int42' = i_693
        while True:
            if not len1(source_text_692) > i_693:
                break
            cp_699: 'int42' = string_get_2561(source_text_692, i_693)
            if cp_699 == 34:
                break
            t_2331 = string_next_2563(source_text_692, i_693)
            i_next_700: 'int42' = t_2331
            end_701: 'int42' = len_2528(source_text_692)
            need_to_flush_702: 'bool41' = False
            if cp_699 != 92:
                t_1593 = cp_699
            else:
                need_to_flush_702 = True
                if not len1(source_text_692) > i_next_700:
                    expected_token_error_353(source_text_692, i_next_700, err_out_695, 'escape sequence')
                    return_306 = -1
                    fn_696.break_()
                esc0_704: 'int42' = string_get_2561(source_text_692, i_next_700)
                t_2336 = string_next_2563(source_text_692, i_next_700)
                i_next_700 = t_2336
                if esc0_704 == 34:
                    t_1580 = True
                else:
                    if esc0_704 == 92:
                        t_1579 = True
                    else:
                        t_1579 = esc0_704 == 47
                    t_1580 = t_1579
                if t_1580:
                    t_1591 = esc0_704
                elif esc0_704 == 98:
                    t_1591 = 8
                elif esc0_704 == 102:
                    t_1591 = 12
                elif esc0_704 == 110:
                    t_1591 = 10
                elif esc0_704 == 114:
                    t_1591 = 13
                elif esc0_704 == 116:
                    t_1591 = 9
                elif esc0_704 == 117:
                    if string_has_at_least_2569(source_text_692, i_next_700, end_701, 4):
                        start_hex_706: 'int42' = i_next_700
                        t_2338 = string_next_2563(source_text_692, i_next_700)
                        i_next_700 = t_2338
                        t_2339 = string_next_2563(source_text_692, i_next_700)
                        i_next_700 = t_2339
                        t_2340 = string_next_2563(source_text_692, i_next_700)
                        i_next_700 = t_2340
                        t_2341 = string_next_2563(source_text_692, i_next_700)
                        i_next_700 = t_2341
                        t_2342 = decode_hex_unsigned_361(source_text_692, start_hex_706, i_next_700)
                        t_1588 = t_2342
                    else:
                        t_1588 = -1
                    hex_705: 'int42' = t_1588
                    if hex_705 < 0:
                        expected_token_error_353(source_text_692, i_next_700, err_out_695, 'four hex digits')
                        return_306 = -1
                        fn_696.break_()
                    t_1589 = hex_705
                    t_1591 = t_1589
                else:
                    expected_token_error_353(source_text_692, i_next_700, err_out_695, 'escape sequence')
                    return_306 = -1
                    fn_696.break_()
                t_1593 = t_1591
            decoded_cp_703: 'int42' = t_1593
            if lead_surrogate_697 >= 0:
                need_to_flush_702 = True
                lead_707: 'int42' = lead_surrogate_697
                if 56320 <= decoded_cp_703:
                    t_1594 = decoded_cp_703 <= 57343
                else:
                    t_1594 = False
                if t_1594:
                    lead_surrogate_697 = -1
                    decoded_cp_703 = int_add_2554(65536, int_mul_2568(int_sub_2527(lead_707, 55296), 1024) | int_sub_2527(decoded_cp_703, 56320))
            else:
                if 55296 <= decoded_cp_703:
                    t_1595 = decoded_cp_703 <= 56319
                else:
                    t_1595 = False
                if t_1595:
                    need_to_flush_702 = True
            if need_to_flush_702:
                sb_694.append(source_text_692[consumed_698 : i_693])
                if lead_surrogate_697 >= 0:
                    sb_694.append(string_from_code_point55(lead_surrogate_697))
                if 55296 <= decoded_cp_703:
                    t_1597 = decoded_cp_703 <= 56319
                else:
                    t_1597 = False
                if t_1597:
                    lead_surrogate_697 = decoded_cp_703
                else:
                    lead_surrogate_697 = -1
                    sb_694.append(string_from_code_point55(decoded_cp_703))
                consumed_698 = i_next_700
            i_693 = i_next_700
        if not len1(source_text_692) > i_693:
            t_1601 = True
        else:
            t_2347 = string_get_2561(source_text_692, i_693)
            t_1601 = t_2347 != 34
        if t_1601:
            expected_token_error_353(source_text_692, i_693, err_out_695, '"')
            return_306 = -1
        else:
            if lead_surrogate_697 >= 0:
                sb_694.append(string_from_code_point55(lead_surrogate_697))
            else:
                sb_694.append(source_text_692[consumed_698 : i_693])
            t_2350 = string_next_2563(source_text_692, i_693)
            i_693 = t_2350
            return_306 = i_693
    return return_306
def parse_json_object_357(source_text_674: 'str36', i_675: 'int42', out_676: 'JsonProducer') -> 'int42':
    return_303: 'int42'
    t_2296: 'int42'
    t_2299: 'int42'
    t_2300: 'int42'
    t_2302: 'int42'
    t_2306: 'str36'
    t_2308: 'int42'
    t_2310: 'int42'
    t_2311: 'int42'
    t_2315: 'int42'
    t_2317: 'int42'
    t_2318: 'int42'
    t_2319: 'int42'
    t_2321: 'int42'
    t_1533: 'bool41'
    t_1539: 'bool41'
    t_1545: 'int42'
    t_1547: 'int42'
    t_1551: 'bool41'
    t_1555: 'int42'
    t_1560: 'bool41'
    t_1565: 'bool41'
    with Label49() as fn_677:
        if not len1(source_text_674) > i_675:
            t_1533 = True
        else:
            t_2296 = string_get_2561(source_text_674, i_675)
            t_1533 = t_2296 != 123
        if t_1533:
            expected_token_error_353(source_text_674, i_675, out_676, "'{'")
            return_303 = -1
            fn_677.break_()
        out_676.start_object()
        t_2299 = string_next_2563(source_text_674, i_675)
        t_2300 = skip_json_spaces_355(source_text_674, t_2299)
        i_675 = t_2300
        if len1(source_text_674) > i_675:
            t_2302 = string_get_2561(source_text_674, i_675)
            t_1539 = t_2302 != 125
        else:
            t_1539 = False
        if t_1539:
            while True:
                key_buffer_678: 'list14[str36]' = ['']
                after_key_679: 'int42' = parse_json_string_to_360(source_text_674, i_675, key_buffer_678, out_676)
                if not after_key_679 >= 0:
                    return_303 = -1
                    fn_677.break_()
                t_2306 = ''.join(key_buffer_678)
                out_676.object_key(t_2306)
                t_1545 = require_string_index_2572(after_key_679)
                t_1547 = t_1545
                t_2308 = skip_json_spaces_355(source_text_674, t_1547)
                i_675 = t_2308
                if len1(source_text_674) > i_675:
                    t_2310 = string_get_2561(source_text_674, i_675)
                    t_1551 = t_2310 == 58
                else:
                    t_1551 = False
                if t_1551:
                    t_2311 = string_next_2563(source_text_674, i_675)
                    i_675 = t_2311
                    after_property_value_680: 'int42' = parse_json_value_356(source_text_674, i_675, out_676)
                    if not after_property_value_680 >= 0:
                        return_303 = -1
                        fn_677.break_()
                    t_1555 = require_string_index_2572(after_property_value_680)
                    i_675 = t_1555
                else:
                    expected_token_error_353(source_text_674, i_675, out_676, "':'")
                    return_303 = -1
                    fn_677.break_()
                t_2315 = skip_json_spaces_355(source_text_674, i_675)
                i_675 = t_2315
                if len1(source_text_674) > i_675:
                    t_2317 = string_get_2561(source_text_674, i_675)
                    t_1560 = t_2317 == 44
                else:
                    t_1560 = False
                if t_1560:
                    t_2318 = string_next_2563(source_text_674, i_675)
                    t_2319 = skip_json_spaces_355(source_text_674, t_2318)
                    i_675 = t_2319
                else:
                    break
        if len1(source_text_674) > i_675:
            t_2321 = string_get_2561(source_text_674, i_675)
            t_1565 = t_2321 == 125
        else:
            t_1565 = False
        if t_1565:
            out_676.end_object()
            return_303 = string_next_2563(source_text_674, i_675)
        else:
            expected_token_error_353(source_text_674, i_675, out_676, "'}'")
            return_303 = -1
    return return_303
def parse_json_array_358(source_text_681: 'str36', i_682: 'int42', out_683: 'JsonProducer') -> 'int42':
    return_304: 'int42'
    t_2276: 'int42'
    t_2279: 'int42'
    t_2280: 'int42'
    t_2282: 'int42'
    t_2285: 'int42'
    t_2287: 'int42'
    t_2288: 'int42'
    t_2289: 'int42'
    t_2291: 'int42'
    t_1509: 'bool41'
    t_1515: 'bool41'
    t_1518: 'int42'
    t_1523: 'bool41'
    t_1528: 'bool41'
    with Label49() as fn_684:
        if not len1(source_text_681) > i_682:
            t_1509 = True
        else:
            t_2276 = string_get_2561(source_text_681, i_682)
            t_1509 = t_2276 != 91
        if t_1509:
            expected_token_error_353(source_text_681, i_682, out_683, "'['")
            return_304 = -1
            fn_684.break_()
        out_683.start_array()
        t_2279 = string_next_2563(source_text_681, i_682)
        t_2280 = skip_json_spaces_355(source_text_681, t_2279)
        i_682 = t_2280
        if len1(source_text_681) > i_682:
            t_2282 = string_get_2561(source_text_681, i_682)
            t_1515 = t_2282 != 93
        else:
            t_1515 = False
        if t_1515:
            while True:
                after_element_value_685: 'int42' = parse_json_value_356(source_text_681, i_682, out_683)
                if not after_element_value_685 >= 0:
                    return_304 = -1
                    fn_684.break_()
                t_1518 = require_string_index_2572(after_element_value_685)
                i_682 = t_1518
                t_2285 = skip_json_spaces_355(source_text_681, i_682)
                i_682 = t_2285
                if len1(source_text_681) > i_682:
                    t_2287 = string_get_2561(source_text_681, i_682)
                    t_1523 = t_2287 == 44
                else:
                    t_1523 = False
                if t_1523:
                    t_2288 = string_next_2563(source_text_681, i_682)
                    t_2289 = skip_json_spaces_355(source_text_681, t_2288)
                    i_682 = t_2289
                else:
                    break
        if len1(source_text_681) > i_682:
            t_2291 = string_get_2561(source_text_681, i_682)
            t_1528 = t_2291 == 93
        else:
            t_1528 = False
        if t_1528:
            out_683.end_array()
            return_304 = string_next_2563(source_text_681, i_682)
        else:
            expected_token_error_353(source_text_681, i_682, out_683, "']'")
            return_304 = -1
    return return_304
def parse_json_string_359(source_text_686: 'str36', i_687: 'int42', out_688: 'JsonProducer') -> 'int42':
    t_2273: 'str36'
    sb_690: 'list14[str36]' = ['']
    after_691: 'int42' = parse_json_string_to_360(source_text_686, i_687, sb_690, out_688)
    if after_691 >= 0:
        t_2273 = ''.join(sb_690)
        out_688.string_value(t_2273)
    return after_691
def after_substring_364(string_730: 'str36', in_string_731: 'int42', substring_732: 'str36') -> 'int42':
    return_310: 'int42'
    t_2268: 'int42'
    t_2269: 'int42'
    with Label49() as fn_733:
        i_734: 'int42' = in_string_731
        j_735: 'int42' = 0
        while True:
            if not len1(substring_732) > j_735:
                break
            if not len1(string_730) > i_734:
                return_310 = -1
                fn_733.break_()
            if string_get_2561(string_730, i_734) != string_get_2561(substring_732, j_735):
                return_310 = -1
                fn_733.break_()
            t_2268 = string_next_2563(string_730, i_734)
            i_734 = t_2268
            t_2269 = string_next_2563(substring_732, j_735)
            j_735 = t_2269
        return_310 = i_734
    return return_310
def parse_json_boolean_362(source_text_716: 'str36', i_717: 'int42', out_718: 'JsonProducer') -> 'int42':
    return_308: 'int42'
    t_2257: 'int42'
    with Label49() as fn_719:
        ch0_720: 'int42'
        if len1(source_text_716) > i_717:
            t_2257 = string_get_2561(source_text_716, i_717)
            ch0_720 = t_2257
        else:
            ch0_720 = 0
        end_721: 'int42' = len_2528(source_text_716)
        keyword_722: 'Union38[str36, None]'
        n_723: 'int42'
        if ch0_720 == 102:
            keyword_722 = 'false'
            n_723 = 5
        elif ch0_720 == 116:
            keyword_722 = 'true'
            n_723 = 4
        else:
            keyword_722 = None
            n_723 = 0
        if not keyword_722 is None:
            keyword_958: 'str36' = keyword_722
            if string_has_at_least_2569(source_text_716, i_717, end_721, n_723):
                after_724: 'int42' = after_substring_364(source_text_716, i_717, keyword_958)
                if after_724 >= 0:
                    return_308 = require_string_index_2572(after_724)
                    out_718.boolean_value(n_723 == 4)
                    fn_719.break_()
        expected_token_error_353(source_text_716, i_717, out_718, '`false` or `true`')
        return_308 = -1
    return return_308
def parse_json_null_363(source_text_725: 'str36', i_726: 'int42', out_727: 'JsonProducer') -> 'int42':
    return_309: 'int42'
    with Label49() as fn_728:
        after_729: 'int42' = after_substring_364(source_text_725, i_726, 'null')
        if after_729 >= 0:
            return_309 = require_string_index_2572(after_729)
            out_727.null_value()
            fn_728.break_()
        expected_token_error_353(source_text_725, i_726, out_727, '`null`')
        return_309 = -1
    return return_309
def parse_json_number_365(source_text_736: 'str36', i_737: 'int42', out_738: 'JsonProducer') -> 'int42':
    return_311: 'int42'
    t_2212: 'int42'
    t_2213: 'int42'
    t_2215: 'int42'
    t_2217: 'int42'
    t_2218: 'float43'
    t_2219: 'int64_23'
    t_2222: 'int42'
    t_2223: 'float43'
    t_2224: 'int64_23'
    t_2228: 'int42'
    t_2229: 'int42'
    t_2232: 'int42'
    t_2233: 'float43'
    t_2236: 'int42'
    t_2237: 'int42'
    t_2241: 'int42'
    t_2244: 'int42'
    t_2246: 'int42'
    t_1420: 'bool41'
    t_1425: 'bool41'
    t_1426: 'bool41'
    t_1434: 'bool41'
    t_1437: 'float43'
    t_1439: 'int64_23'
    t_1442: 'bool41'
    t_1443: 'bool41'
    t_1446: 'bool41'
    t_1450: 'bool41'
    t_1453: 'float43'
    t_1456: 'bool41'
    t_1460: 'bool41'
    t_1464: 'bool41'
    t_1466: 'bool41'
    t_1467: 'bool41'
    t_1469: 'bool41'
    t_1472: 'bool41'
    t_1473: 'float43'
    t_1474: 'bool41'
    t_1475: 'bool41'
    with Label49() as fn_739:
        is_negative_740: 'bool41' = False
        start_of_number_741: 'int42' = i_737
        if len1(source_text_736) > i_737:
            t_2212 = string_get_2561(source_text_736, i_737)
            t_1420 = t_2212 == 45
        else:
            t_1420 = False
        if t_1420:
            is_negative_740 = True
            t_2213 = string_next_2563(source_text_736, i_737)
            i_737 = t_2213
        digit0_742: 'int42'
        if len1(source_text_736) > i_737:
            t_2215 = string_get_2561(source_text_736, i_737)
            digit0_742 = t_2215
        else:
            digit0_742 = -1
        if digit0_742 < 48:
            t_1425 = True
        else:
            t_1425 = 57 < digit0_742
        if t_1425:
            error_743: 'str36'
            if not is_negative_740:
                t_1426 = digit0_742 != 46
            else:
                t_1426 = False
            if t_1426:
                error_743 = 'JSON value'
            else:
                error_743 = 'digit'
            expected_token_error_353(source_text_736, i_737, out_738, error_743)
            return_311 = -1
            fn_739.break_()
        t_2217 = string_next_2563(source_text_736, i_737)
        i_737 = t_2217
        n_digits_744: 'int42' = 1
        t_2218 = float(int_sub_2527(digit0_742, 48))
        tentative_float64_745: 'float43' = t_2218
        t_2219 = int_sub_2527(digit0_742, 48)
        tentative_int64_746: 'int64_23' = t_2219
        overflow_int64_747: 'bool41' = False
        if 48 != digit0_742:
            while True:
                if not len1(source_text_736) > i_737:
                    break
                possible_digit_748: 'int42' = string_get_2561(source_text_736, i_737)
                if 48 <= possible_digit_748:
                    t_1434 = possible_digit_748 <= 57
                else:
                    t_1434 = False
                if t_1434:
                    t_2222 = string_next_2563(source_text_736, i_737)
                    i_737 = t_2222
                    n_digits_744 = int_add_2554(n_digits_744, 1)
                    next_digit_749: 'int42' = int_sub_2527(possible_digit_748, 48)
                    t_1437 = tentative_float64_745 * 10.0
                    t_2223 = float(next_digit_749)
                    tentative_float64_745 = t_1437 + t_2223
                    old_int64_750: 'int64_23' = tentative_int64_746
                    t_1439 = int64_mul_2573(tentative_int64_746, 10)
                    t_2224 = next_digit_749
                    tentative_int64_746 = int64_add_2574(t_1439, t_2224)
                    if tentative_int64_746 < old_int64_750:
                        if int64_sub_2575(-9223372036854775808, old_int64_750) == int64_negate_2576(next_digit_749):
                            if is_negative_740:
                                t_1442 = old_int64_750 > 0
                            else:
                                t_1442 = False
                            t_1443 = t_1442
                        else:
                            t_1443 = False
                        if not t_1443:
                            overflow_int64_747 = True
                else:
                    break
        n_digits_after_point_751: 'int42' = 0
        if len1(source_text_736) > i_737:
            t_2228 = string_get_2561(source_text_736, i_737)
            t_1446 = 46 == t_2228
        else:
            t_1446 = False
        if t_1446:
            t_2229 = string_next_2563(source_text_736, i_737)
            i_737 = t_2229
            after_point_752: 'int42' = i_737
            while True:
                if not len1(source_text_736) > i_737:
                    break
                possible_digit_753: 'int42' = string_get_2561(source_text_736, i_737)
                if 48 <= possible_digit_753:
                    t_1450 = possible_digit_753 <= 57
                else:
                    t_1450 = False
                if t_1450:
                    t_2232 = string_next_2563(source_text_736, i_737)
                    i_737 = t_2232
                    n_digits_744 = int_add_2554(n_digits_744, 1)
                    n_digits_after_point_751 = int_add_2554(n_digits_after_point_751, 1)
                    t_1453 = tentative_float64_745 * 10.0
                    t_2233 = float(int_sub_2527(possible_digit_753, 48))
                    tentative_float64_745 = t_1453 + t_2233
                else:
                    break
            if i_737 == after_point_752:
                expected_token_error_353(source_text_736, i_737, out_738, 'digit')
                return_311 = -1
                fn_739.break_()
        n_exponent_digits_754: 'int42' = 0
        if len1(source_text_736) > i_737:
            t_2236 = string_get_2561(source_text_736, i_737)
            t_1456 = 101 == t_2236 | 32
        else:
            t_1456 = False
        if t_1456:
            t_2237 = string_next_2563(source_text_736, i_737)
            i_737 = t_2237
            if not len1(source_text_736) > i_737:
                expected_token_error_353(source_text_736, i_737, out_738, 'sign or digit')
                return_311 = -1
                fn_739.break_()
            after_e_755: 'int42' = string_get_2561(source_text_736, i_737)
            if after_e_755 == 43:
                t_1460 = True
            else:
                t_1460 = after_e_755 == 45
            if t_1460:
                t_2241 = string_next_2563(source_text_736, i_737)
                i_737 = t_2241
            while True:
                if not len1(source_text_736) > i_737:
                    break
                possible_digit_756: 'int42' = string_get_2561(source_text_736, i_737)
                if 48 <= possible_digit_756:
                    t_1464 = possible_digit_756 <= 57
                else:
                    t_1464 = False
                if t_1464:
                    t_2244 = string_next_2563(source_text_736, i_737)
                    i_737 = t_2244
                    n_exponent_digits_754 = int_add_2554(n_exponent_digits_754, 1)
                else:
                    break
            if n_exponent_digits_754 == 0:
                expected_token_error_353(source_text_736, i_737, out_738, 'exponent digit')
                return_311 = -1
                fn_739.break_()
        after_exponent_757: 'int42' = i_737
        if n_exponent_digits_754 == 0:
            if n_digits_after_point_751 == 0:
                t_1466 = not overflow_int64_747
            else:
                t_1466 = False
            t_1467 = t_1466
        else:
            t_1467 = False
        if t_1467:
            value_758: 'int64_23'
            if is_negative_740:
                value_758 = int64_negate_2576(tentative_int64_746)
            else:
                value_758 = tentative_int64_746
            if -2147483648 <= value_758:
                t_1469 = value_758 <= 2147483647
            else:
                t_1469 = False
            if t_1469:
                t_2246 = int64_to_int32_unsafe_2577(value_758)
                out_738.int32_value(t_2246)
            else:
                out_738.int64_value(value_758)
            return_311 = i_737
            fn_739.break_()
        numeric_token_string_759: 'str36' = source_text_736[start_of_number_741 : i_737]
        double_value_760: 'float43' = nan56
        if n_exponent_digits_754 != 0:
            t_1472 = True
        else:
            t_1472 = n_digits_after_point_751 != 0
        if t_1472:
            try:
                t_1473 = string_to_float64_2541(numeric_token_string_759)
                double_value_760 = t_1473
            except Exception46:
                pass
        if float_not_eq_2578(double_value_760, -inf57):
            if float_not_eq_2578(double_value_760, inf57):
                t_1474 = float_not_eq_2578(double_value_760, nan56)
            else:
                t_1474 = False
            t_1475 = t_1474
        else:
            t_1475 = False
        if t_1475:
            out_738.float64_value(double_value_760)
        else:
            out_738.numeric_token_value(numeric_token_string_759)
        return_311 = i_737
    return return_311
def parse_json_to_producer(source_text_662: 'str36', out_663: 'JsonProducer') -> 'None':
    t_2195: 'int42'
    t_2197: 'Union38[JsonParseErrorReceiver, None]'
    t_2198: 'int42'
    t_2199: 'str36'
    t_1402: 'bool41'
    t_1405: 'int42'
    i_665: 'int42' = 0
    after_value_666: 'int42' = parse_json_value_356(source_text_662, i_665, out_663)
    if after_value_666 >= 0:
        t_1405 = require_string_index_2572(after_value_666)
        t_2195 = skip_json_spaces_355(source_text_662, t_1405)
        i_665 = t_2195
        if len1(source_text_662) > i_665:
            t_2197 = out_663.parse_error_receiver
            t_1402 = not t_2197 is None
        else:
            t_1402 = False
        if t_1402:
            t_2198 = len_2528(source_text_662)
            t_2199 = source_text_662[i_665 : t_2198]
            store_json_error_354(out_663, str_cat_2566('Extraneous JSON `', t_2199, '`'))
def parse_json(source_text_761: 'str36') -> 'JsonSyntaxTree':
    p_763: 'JsonSyntaxTreeProducer' = JsonSyntaxTreeProducer()
    parse_json_to_producer(source_text_761, p_763)
    return p_763.to_json_syntax_tree()
def boolean_json_adapter() -> 'JsonAdapter[bool41]':
    return BooleanJsonAdapter_164()
def float64_json_adapter() -> 'JsonAdapter[float43]':
    return Float64JsonAdapter_167()
def int32_json_adapter() -> 'JsonAdapter[int42]':
    return Int32JsonAdapter_170()
def int64_json_adapter() -> 'JsonAdapter[int64_23]':
    return Int64JsonAdapter_173()
def string_json_adapter() -> 'JsonAdapter[str36]':
    return StringJsonAdapter_176()
T_183 = TypeVar53('T_183', bound = Any52)
def list_json_adapter(adapter_for_t_839: 'JsonAdapter[T_183]') -> 'JsonAdapter[(Sequence45[T_183])]':
    return ListJsonAdapter_179(adapter_for_t_839)
