from builtins import str as str27, RuntimeError as RuntimeError30, int as int31, bool as bool33, Exception as Exception37, float as float38, isinstance as isinstance39, list as list3, len as len6, tuple as tuple5
from abc import ABCMeta as ABCMeta28
from typing import Sequence as Sequence29, Dict as Dict34, MutableSequence as MutableSequence36, Union as Union40, Any as Any41, TypeVar as TypeVar42, Callable as Callable43
from types import MappingProxyType as MappingProxyType32
from temper_core import Label as Label35, Pair as Pair25, string_from_code_point as string_from_code_point44, map_builder_set as map_builder_set0, list_for_each as list_for_each1, mapped_to_map as mapped_to_map2, mapped_has as mapped_has4, string_count_between as string_count_between7, str_cat as str_cat8, int_to_string as int_to_string9, string_to_int32 as string_to_int3210, string_to_int64 as string_to_int6411, string_to_float64 as string_to_float6412, date_from_iso_string as date_from_iso_string13, list_get as list_get14, int_add as int_add15, mapped_to_list as mapped_to_list16, list_join as list_join17, list_builder_add_all as list_builder_add_all18, date_to_string as date_to_string19, float64_to_string as float64_to_string20, string_for_each as string_for_each21, map_constructor as map_constructor22, string_get as string_get23, string_next as string_next24
from datetime import date as date26
map_builder_set_4823 = map_builder_set0
list_for_each_4824 = list_for_each1
mapped_to_map_4825 = mapped_to_map2
list_4826 = list3
mapped_has_4827 = mapped_has4
tuple_4829 = tuple5
len_4830 = len6
string_count_between_4831 = string_count_between7
str_cat_4832 = str_cat8
int_to_string_4833 = int_to_string9
string_to_int32_4834 = string_to_int3210
string_to_int64_4835 = string_to_int6411
string_to_float64_4836 = string_to_float6412
date_from_iso_string_4837 = date_from_iso_string13
list_get_4838 = list_get14
int_add_4839 = int_add15
mapped_to_list_4840 = mapped_to_list16
list_join_4841 = list_join17
list_builder_add_all_4842 = list_builder_add_all18
date_to_string_4846 = date_to_string19
float64_to_string_4847 = float64_to_string20
string_for_each_4849 = string_for_each21
map_constructor_4850 = map_constructor22
string_get_4851 = string_get23
string_next_4852 = string_next24
pair_4854 = Pair25
date_4857 = date26
class ChangesetError:
    field_300: 'str27'
    message_301: 'str27'
    __slots__ = ('field_300', 'message_301')
    def __init__(this_150, field_303: 'str27', message_304: 'str27') -> None:
        this_150.field_300 = field_303
        this_150.message_301 = message_304
    @property
    def field(this_844) -> 'str27':
        return this_844.field_300
    @property
    def message(this_847) -> 'str27':
        return this_847.message_301
class Changeset(metaclass = ABCMeta28):
    def cast(this_82, allowed_fields_314: 'Sequence29[SafeIdentifier]') -> 'Changeset':
        raise RuntimeError30()
    def validate_required(this_83, fields_317: 'Sequence29[SafeIdentifier]') -> 'Changeset':
        raise RuntimeError30()
    def validate_length(this_84, field_320: 'SafeIdentifier', min_321: 'int31', max_322: 'int31') -> 'Changeset':
        raise RuntimeError30()
    def validate_int(this_85, field_325: 'SafeIdentifier') -> 'Changeset':
        raise RuntimeError30()
    def validate_int64(this_86, field_328: 'SafeIdentifier') -> 'Changeset':
        raise RuntimeError30()
    def validate_float(this_87, field_331: 'SafeIdentifier') -> 'Changeset':
        raise RuntimeError30()
    def validate_bool(this_88, field_334: 'SafeIdentifier') -> 'Changeset':
        raise RuntimeError30()
    def to_insert_sql(this_89) -> 'SqlFragment':
        raise RuntimeError30()
    def to_update_sql(this_90, id_339: 'int31') -> 'SqlFragment':
        raise RuntimeError30()
class ChangesetImpl_91(Changeset):
    table_def_341: 'TableDef'
    params_342: 'MappingProxyType32[str27, str27]'
    changes_343: 'MappingProxyType32[str27, str27]'
    errors_344: 'Sequence29[ChangesetError]'
    is_valid_345: 'bool33'
    __slots__ = ('table_def_341', 'params_342', 'changes_343', 'errors_344', 'is_valid_345')
    @property
    def table_def(this_92) -> 'TableDef':
        return this_92.table_def_341
    @property
    def changes(this_93) -> 'MappingProxyType32[str27, str27]':
        return this_93.changes_343
    @property
    def errors(this_94) -> 'Sequence29[ChangesetError]':
        return this_94.errors_344
    @property
    def is_valid(this_95) -> 'bool33':
        return this_95.is_valid_345
    def cast(this_96, allowed_fields_355: 'Sequence29[SafeIdentifier]') -> 'Changeset':
        mb_357: 'Dict34[str27, str27]' = {}
        def fn_4731(f_358: 'SafeIdentifier') -> 'None':
            t_4729: 'str27'
            t_4726: 'str27' = f_358.sql_value
            val_359: 'str27' = this_96.params_342.get(t_4726, '')
            if not (not val_359):
                t_4729 = f_358.sql_value
                map_builder_set_4823(mb_357, t_4729, val_359)
        list_for_each_4824(allowed_fields_355, fn_4731)
        return ChangesetImpl_91(this_96.table_def_341, this_96.params_342, mapped_to_map_4825(mb_357), this_96.errors_344, this_96.is_valid_345)
    def validate_required(this_97, fields_361: 'Sequence29[SafeIdentifier]') -> 'Changeset':
        return_183: 'Changeset'
        t_4724: 'Sequence29[ChangesetError]'
        t_2816: 'TableDef'
        t_2817: 'MappingProxyType32[str27, str27]'
        t_2818: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_362:
            if not this_97.is_valid_345:
                return_183 = this_97
                fn_362.break_()
            eb_363: 'MutableSequence36[ChangesetError]' = list_4826(this_97.errors_344)
            valid_364: 'bool33' = True
            def fn_4720(f_365: 'SafeIdentifier') -> 'None':
                nonlocal valid_364
                t_4718: 'ChangesetError'
                t_4715: 'str27' = f_365.sql_value
                if not mapped_has_4827(this_97.changes_343, t_4715):
                    t_4718 = ChangesetError(f_365.sql_value, 'is required')
                    eb_363.append(t_4718)
                    valid_364 = False
            list_for_each_4824(fields_361, fn_4720)
            t_2816 = this_97.table_def_341
            t_2817 = this_97.params_342
            t_2818 = this_97.changes_343
            t_4724 = tuple_4829(eb_363)
            return_183 = ChangesetImpl_91(t_2816, t_2817, t_2818, t_4724, valid_364)
        return return_183
    def validate_length(this_98, field_367: 'SafeIdentifier', min_368: 'int31', max_369: 'int31') -> 'Changeset':
        return_184: 'Changeset'
        t_4702: 'str27'
        t_4713: 'Sequence29[ChangesetError]'
        t_2799: 'bool33'
        t_2805: 'TableDef'
        t_2806: 'MappingProxyType32[str27, str27]'
        t_2807: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_370:
            if not this_98.is_valid_345:
                return_184 = this_98
                fn_370.break_()
            t_4702 = field_367.sql_value
            val_371: 'str27' = this_98.changes_343.get(t_4702, '')
            len_372: 'int31' = string_count_between_4831(val_371, 0, len_4830(val_371))
            if len_372 < min_368:
                t_2799 = True
            else:
                t_2799 = len_372 > max_369
            if t_2799:
                msg_373: 'str27' = str_cat_4832('must be between ', int_to_string_4833(min_368), ' and ', int_to_string_4833(max_369), ' characters')
                eb_374: 'MutableSequence36[ChangesetError]' = list_4826(this_98.errors_344)
                eb_374.append(ChangesetError(field_367.sql_value, msg_373))
                t_2805 = this_98.table_def_341
                t_2806 = this_98.params_342
                t_2807 = this_98.changes_343
                t_4713 = tuple_4829(eb_374)
                return_184 = ChangesetImpl_91(t_2805, t_2806, t_2807, t_4713, False)
                fn_370.break_()
            return_184 = this_98
        return return_184
    def validate_int(this_99, field_376: 'SafeIdentifier') -> 'Changeset':
        return_185: 'Changeset'
        t_4693: 'str27'
        t_4700: 'Sequence29[ChangesetError]'
        t_2790: 'TableDef'
        t_2791: 'MappingProxyType32[str27, str27]'
        t_2792: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_377:
            if not this_99.is_valid_345:
                return_185 = this_99
                fn_377.break_()
            t_4693 = field_376.sql_value
            val_378: 'str27' = this_99.changes_343.get(t_4693, '')
            if not val_378:
                return_185 = this_99
                fn_377.break_()
            parse_ok_379: 'bool33'
            try:
                string_to_int32_4834(val_378)
                parse_ok_379 = True
            except Exception37:
                parse_ok_379 = False
            if not parse_ok_379:
                eb_380: 'MutableSequence36[ChangesetError]' = list_4826(this_99.errors_344)
                eb_380.append(ChangesetError(field_376.sql_value, 'must be an integer'))
                t_2790 = this_99.table_def_341
                t_2791 = this_99.params_342
                t_2792 = this_99.changes_343
                t_4700 = tuple_4829(eb_380)
                return_185 = ChangesetImpl_91(t_2790, t_2791, t_2792, t_4700, False)
                fn_377.break_()
            return_185 = this_99
        return return_185
    def validate_int64(this_100, field_382: 'SafeIdentifier') -> 'Changeset':
        return_186: 'Changeset'
        t_4684: 'str27'
        t_4691: 'Sequence29[ChangesetError]'
        t_2777: 'TableDef'
        t_2778: 'MappingProxyType32[str27, str27]'
        t_2779: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_383:
            if not this_100.is_valid_345:
                return_186 = this_100
                fn_383.break_()
            t_4684 = field_382.sql_value
            val_384: 'str27' = this_100.changes_343.get(t_4684, '')
            if not val_384:
                return_186 = this_100
                fn_383.break_()
            parse_ok_385: 'bool33'
            try:
                string_to_int64_4835(val_384)
                parse_ok_385 = True
            except Exception37:
                parse_ok_385 = False
            if not parse_ok_385:
                eb_386: 'MutableSequence36[ChangesetError]' = list_4826(this_100.errors_344)
                eb_386.append(ChangesetError(field_382.sql_value, 'must be a 64-bit integer'))
                t_2777 = this_100.table_def_341
                t_2778 = this_100.params_342
                t_2779 = this_100.changes_343
                t_4691 = tuple_4829(eb_386)
                return_186 = ChangesetImpl_91(t_2777, t_2778, t_2779, t_4691, False)
                fn_383.break_()
            return_186 = this_100
        return return_186
    def validate_float(this_101, field_388: 'SafeIdentifier') -> 'Changeset':
        return_187: 'Changeset'
        t_4675: 'str27'
        t_4682: 'Sequence29[ChangesetError]'
        t_2764: 'TableDef'
        t_2765: 'MappingProxyType32[str27, str27]'
        t_2766: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_389:
            if not this_101.is_valid_345:
                return_187 = this_101
                fn_389.break_()
            t_4675 = field_388.sql_value
            val_390: 'str27' = this_101.changes_343.get(t_4675, '')
            if not val_390:
                return_187 = this_101
                fn_389.break_()
            parse_ok_391: 'bool33'
            try:
                string_to_float64_4836(val_390)
                parse_ok_391 = True
            except Exception37:
                parse_ok_391 = False
            if not parse_ok_391:
                eb_392: 'MutableSequence36[ChangesetError]' = list_4826(this_101.errors_344)
                eb_392.append(ChangesetError(field_388.sql_value, 'must be a number'))
                t_2764 = this_101.table_def_341
                t_2765 = this_101.params_342
                t_2766 = this_101.changes_343
                t_4682 = tuple_4829(eb_392)
                return_187 = ChangesetImpl_91(t_2764, t_2765, t_2766, t_4682, False)
                fn_389.break_()
            return_187 = this_101
        return return_187
    def validate_bool(this_102, field_394: 'SafeIdentifier') -> 'Changeset':
        return_188: 'Changeset'
        t_4666: 'str27'
        t_4673: 'Sequence29[ChangesetError]'
        t_2739: 'bool33'
        t_2740: 'bool33'
        t_2742: 'bool33'
        t_2743: 'bool33'
        t_2745: 'bool33'
        t_2751: 'TableDef'
        t_2752: 'MappingProxyType32[str27, str27]'
        t_2753: 'MappingProxyType32[str27, str27]'
        with Label35() as fn_395:
            if not this_102.is_valid_345:
                return_188 = this_102
                fn_395.break_()
            t_4666 = field_394.sql_value
            val_396: 'str27' = this_102.changes_343.get(t_4666, '')
            if not val_396:
                return_188 = this_102
                fn_395.break_()
            is_true_397: 'bool33'
            if val_396 == 'true':
                is_true_397 = True
            else:
                if val_396 == '1':
                    t_2740 = True
                else:
                    if val_396 == 'yes':
                        t_2739 = True
                    else:
                        t_2739 = val_396 == 'on'
                    t_2740 = t_2739
                is_true_397 = t_2740
            is_false_398: 'bool33'
            if val_396 == 'false':
                is_false_398 = True
            else:
                if val_396 == '0':
                    t_2743 = True
                else:
                    if val_396 == 'no':
                        t_2742 = True
                    else:
                        t_2742 = val_396 == 'off'
                    t_2743 = t_2742
                is_false_398 = t_2743
            if not is_true_397:
                t_2745 = not is_false_398
            else:
                t_2745 = False
            if t_2745:
                eb_399: 'MutableSequence36[ChangesetError]' = list_4826(this_102.errors_344)
                eb_399.append(ChangesetError(field_394.sql_value, 'must be a boolean (true/false/1/0/yes/no/on/off)'))
                t_2751 = this_102.table_def_341
                t_2752 = this_102.params_342
                t_2753 = this_102.changes_343
                t_4673 = tuple_4829(eb_399)
                return_188 = ChangesetImpl_91(t_2751, t_2752, t_2753, t_4673, False)
                fn_395.break_()
            return_188 = this_102
        return return_188
    def parse_bool_sql_part_400(this_103, val_401: 'str27') -> 'SqlBoolean':
        return_189: 'SqlBoolean'
        t_2728: 'bool33'
        t_2729: 'bool33'
        t_2730: 'bool33'
        t_2732: 'bool33'
        t_2733: 'bool33'
        t_2734: 'bool33'
        with Label35() as fn_402:
            if val_401 == 'true':
                t_2730 = True
            else:
                if val_401 == '1':
                    t_2729 = True
                else:
                    if val_401 == 'yes':
                        t_2728 = True
                    else:
                        t_2728 = val_401 == 'on'
                    t_2729 = t_2728
                t_2730 = t_2729
            if t_2730:
                return_189 = SqlBoolean(True)
                fn_402.break_()
            if val_401 == 'false':
                t_2734 = True
            else:
                if val_401 == '0':
                    t_2733 = True
                else:
                    if val_401 == 'no':
                        t_2732 = True
                    else:
                        t_2732 = val_401 == 'off'
                    t_2733 = t_2732
                t_2734 = t_2733
            if t_2734:
                return_189 = SqlBoolean(False)
                fn_402.break_()
            raise RuntimeError30()
        return return_189
    def value_to_sql_part_403(this_104, field_def_404: 'FieldDef', val_405: 'str27') -> 'SqlPart':
        return_190: 'SqlPart'
        t_2715: 'int31'
        t_2718: 'int64_23'
        t_2721: 'float38'
        t_2726: 'date26'
        with Label35() as fn_406:
            ft_407: 'FieldType' = field_def_404.field_type
            if isinstance39(ft_407, StringField):
                return_190 = SqlString(val_405)
                fn_406.break_()
            if isinstance39(ft_407, IntField):
                t_2715 = string_to_int32_4834(val_405)
                return_190 = SqlInt32(t_2715)
                fn_406.break_()
            if isinstance39(ft_407, Int64Field):
                t_2718 = string_to_int64_4835(val_405)
                return_190 = SqlInt64(t_2718)
                fn_406.break_()
            if isinstance39(ft_407, FloatField):
                t_2721 = string_to_float64_4836(val_405)
                return_190 = SqlFloat64(t_2721)
                fn_406.break_()
            if isinstance39(ft_407, BoolField):
                return_190 = this_104.parse_bool_sql_part_400(val_405)
                fn_406.break_()
            if isinstance39(ft_407, DateField):
                t_2726 = date_from_iso_string_4837(val_405)
                return_190 = SqlDate(t_2726)
                fn_406.break_()
            raise RuntimeError30()
        return return_190
    def to_insert_sql(this_105) -> 'SqlFragment':
        t_4615: 'int31'
        t_4620: 'str27'
        t_4621: 'bool33'
        t_4626: 'int31'
        t_4628: 'str27'
        t_4631: 'str27'
        t_4646: 'int31'
        t_2680: 'bool33'
        t_2688: 'FieldDef'
        t_2692: 'SqlPart'
        if not this_105.is_valid_345:
            raise RuntimeError30()
        i_410: 'int31' = 0
        while True:
            t_4615 = len_4830(this_105.table_def_341.fields)
            if not i_410 < t_4615:
                break
            f_411: 'FieldDef' = list_get_4838(this_105.table_def_341.fields, i_410)
            if not f_411.nullable:
                t_4620 = f_411.name.sql_value
                t_4621 = mapped_has_4827(this_105.changes_343, t_4620)
                t_2680 = not t_4621
            else:
                t_2680 = False
            if t_2680:
                raise RuntimeError30()
            i_410 = int_add_4839(i_410, 1)
        pairs_412: 'Sequence29[(Pair25[str27, str27])]' = mapped_to_list_4840(this_105.changes_343)
        if len_4830(pairs_412) == 0:
            raise RuntimeError30()
        col_names_413: 'MutableSequence36[str27]' = list_4826()
        val_parts_414: 'MutableSequence36[SqlPart]' = list_4826()
        i_415: 'int31' = 0
        while True:
            t_4626 = len_4830(pairs_412)
            if not i_415 < t_4626:
                break
            pair_416: 'Pair25[str27, str27]' = list_get_4838(pairs_412, i_415)
            t_4628 = pair_416.key
            t_2688 = this_105.table_def_341.field(t_4628)
            fd_417: 'FieldDef' = t_2688
            col_names_413.append(pair_416.key)
            t_4631 = pair_416.value
            t_2692 = this_105.value_to_sql_part_403(fd_417, t_4631)
            val_parts_414.append(t_2692)
            i_415 = int_add_4839(i_415, 1)
        b_418: 'SqlBuilder' = SqlBuilder()
        b_418.append_safe('INSERT INTO ')
        b_418.append_safe(this_105.table_def_341.table_name.sql_value)
        b_418.append_safe(' (')
        t_4639: 'Sequence29[str27]' = tuple_4829(col_names_413)
        def fn_4613(c_419: 'str27') -> 'str27':
            return c_419
        b_418.append_safe(list_join_4841(t_4639, ', ', fn_4613))
        b_418.append_safe(') VALUES (')
        b_418.append_part(list_get_4838(val_parts_414, 0))
        j_420: 'int31' = 1
        while True:
            t_4646 = len_4830(val_parts_414)
            if not j_420 < t_4646:
                break
            b_418.append_safe(', ')
            b_418.append_part(list_get_4838(val_parts_414, j_420))
            j_420 = int_add_4839(j_420, 1)
        b_418.append_safe(')')
        return b_418.accumulated
    def to_update_sql(this_106, id_422: 'int31') -> 'SqlFragment':
        t_4601: 'int31'
        t_4604: 'str27'
        t_4608: 'str27'
        t_2662: 'FieldDef'
        t_2667: 'SqlPart'
        if not this_106.is_valid_345:
            raise RuntimeError30()
        pairs_424: 'Sequence29[(Pair25[str27, str27])]' = mapped_to_list_4840(this_106.changes_343)
        if len_4830(pairs_424) == 0:
            raise RuntimeError30()
        b_425: 'SqlBuilder' = SqlBuilder()
        b_425.append_safe('UPDATE ')
        b_425.append_safe(this_106.table_def_341.table_name.sql_value)
        b_425.append_safe(' SET ')
        i_426: 'int31' = 0
        while True:
            t_4601 = len_4830(pairs_424)
            if not i_426 < t_4601:
                break
            if i_426 > 0:
                b_425.append_safe(', ')
            pair_427: 'Pair25[str27, str27]' = list_get_4838(pairs_424, i_426)
            t_4604 = pair_427.key
            t_2662 = this_106.table_def_341.field(t_4604)
            fd_428: 'FieldDef' = t_2662
            b_425.append_safe(pair_427.key)
            b_425.append_safe(' = ')
            t_4608 = pair_427.value
            t_2667 = this_106.value_to_sql_part_403(fd_428, t_4608)
            b_425.append_part(t_2667)
            i_426 = int_add_4839(i_426, 1)
        b_425.append_safe(' WHERE id = ')
        b_425.append_int32(id_422)
        return b_425.accumulated
    def __init__(this_173, table_def_430: 'TableDef', params_431: 'MappingProxyType32[str27, str27]', changes_432: 'MappingProxyType32[str27, str27]', errors_433: 'Sequence29[ChangesetError]', is_valid_434: 'bool33') -> None:
        this_173.table_def_341 = table_def_430
        this_173.params_342 = params_431
        this_173.changes_343 = changes_432
        this_173.errors_344 = errors_433
        this_173.is_valid_345 = is_valid_434
class OrderClause:
    field_529: 'SafeIdentifier'
    ascending_530: 'bool33'
    __slots__ = ('field_529', 'ascending_530')
    def __init__(this_197, field_532: 'SafeIdentifier', ascending_533: 'bool33') -> None:
        this_197.field_529 = field_532
        this_197.ascending_530 = ascending_533
    @property
    def field(this_912) -> 'SafeIdentifier':
        return this_912.field_529
    @property
    def ascending(this_915) -> 'bool33':
        return this_915.ascending_530
class Query:
    table_name_534: 'SafeIdentifier'
    conditions_535: 'Sequence29[SqlFragment]'
    selected_fields_536: 'Sequence29[SafeIdentifier]'
    order_clauses_537: 'Sequence29[OrderClause]'
    limit_val_538: 'Union40[int31, None]'
    offset_val_539: 'Union40[int31, None]'
    __slots__ = ('table_name_534', 'conditions_535', 'selected_fields_536', 'order_clauses_537', 'limit_val_538', 'offset_val_539')
    def where(this_107, condition_541: 'SqlFragment') -> 'Query':
        nb_543: 'MutableSequence36[SqlFragment]' = list_4826(this_107.conditions_535)
        nb_543.append(condition_541)
        return Query(this_107.table_name_534, tuple_4829(nb_543), this_107.selected_fields_536, this_107.order_clauses_537, this_107.limit_val_538, this_107.offset_val_539)
    def select(this_108, fields_545: 'Sequence29[SafeIdentifier]') -> 'Query':
        return Query(this_108.table_name_534, this_108.conditions_535, fields_545, this_108.order_clauses_537, this_108.limit_val_538, this_108.offset_val_539)
    def order_by(this_109, field_548: 'SafeIdentifier', ascending_549: 'bool33') -> 'Query':
        nb_551: 'MutableSequence36[OrderClause]' = list_4826(this_109.order_clauses_537)
        nb_551.append(OrderClause(field_548, ascending_549))
        return Query(this_109.table_name_534, this_109.conditions_535, this_109.selected_fields_536, tuple_4829(nb_551), this_109.limit_val_538, this_109.offset_val_539)
    def limit(this_110, n_553: 'int31') -> 'Query':
        if n_553 < 0:
            raise RuntimeError30()
        return Query(this_110.table_name_534, this_110.conditions_535, this_110.selected_fields_536, this_110.order_clauses_537, n_553, this_110.offset_val_539)
    def offset(this_111, n_556: 'int31') -> 'Query':
        if n_556 < 0:
            raise RuntimeError30()
        return Query(this_111.table_name_534, this_111.conditions_535, this_111.selected_fields_536, this_111.order_clauses_537, this_111.limit_val_538, n_556)
    def to_sql(this_112) -> 'SqlFragment':
        t_4185: 'int31'
        b_560: 'SqlBuilder' = SqlBuilder()
        b_560.append_safe('SELECT ')
        if not this_112.selected_fields_536:
            b_560.append_safe('*')
        else:
            def fn_4170(f_561: 'SafeIdentifier') -> 'str27':
                return f_561.sql_value
            b_560.append_safe(list_join_4841(this_112.selected_fields_536, ', ', fn_4170))
        b_560.append_safe(' FROM ')
        b_560.append_safe(this_112.table_name_534.sql_value)
        if not (not this_112.conditions_535):
            b_560.append_safe(' WHERE ')
            b_560.append_fragment(list_get_4838(this_112.conditions_535, 0))
            i_562: 'int31' = 1
            while True:
                t_4185 = len_4830(this_112.conditions_535)
                if not i_562 < t_4185:
                    break
                b_560.append_safe(' AND ')
                b_560.append_fragment(list_get_4838(this_112.conditions_535, i_562))
                i_562 = int_add_4839(i_562, 1)
        if not (not this_112.order_clauses_537):
            b_560.append_safe(' ORDER BY ')
            first_563: 'bool33' = True
            def fn_4169(oc_564: 'OrderClause') -> 'None':
                nonlocal first_563
                t_2283: 'str27'
                if not first_563:
                    b_560.append_safe(', ')
                first_563 = False
                t_4164: 'str27' = oc_564.field.sql_value
                b_560.append_safe(t_4164)
                if oc_564.ascending:
                    t_2283 = ' ASC'
                else:
                    t_2283 = ' DESC'
                b_560.append_safe(t_2283)
            list_for_each_4824(this_112.order_clauses_537, fn_4169)
        lv_565: 'Union40[int31, None]' = this_112.limit_val_538
        if not lv_565 is None:
            lv_1068: 'int31' = lv_565
            b_560.append_safe(' LIMIT ')
            b_560.append_int32(lv_1068)
        ov_566: 'Union40[int31, None]' = this_112.offset_val_539
        if not ov_566 is None:
            ov_1069: 'int31' = ov_566
            b_560.append_safe(' OFFSET ')
            b_560.append_int32(ov_1069)
        return b_560.accumulated
    def safe_to_sql(this_113, default_limit_568: 'int31') -> 'SqlFragment':
        return_212: 'SqlFragment'
        t_2276: 'Query'
        if default_limit_568 < 0:
            raise RuntimeError30()
        if not this_113.limit_val_538 is None:
            return_212 = this_113.to_sql()
        else:
            t_2276 = this_113.limit(default_limit_568)
            return_212 = t_2276.to_sql()
        return return_212
    def __init__(this_199, table_name_571: 'SafeIdentifier', conditions_572: 'Sequence29[SqlFragment]', selected_fields_573: 'Sequence29[SafeIdentifier]', order_clauses_574: 'Sequence29[OrderClause]', limit_val_575: 'Union40[int31, None]', offset_val_576: 'Union40[int31, None]') -> None:
        this_199.table_name_534 = table_name_571
        this_199.conditions_535 = conditions_572
        this_199.selected_fields_536 = selected_fields_573
        this_199.order_clauses_537 = order_clauses_574
        this_199.limit_val_538 = limit_val_575
        this_199.offset_val_539 = offset_val_576
    @property
    def table_name(this_918) -> 'SafeIdentifier':
        return this_918.table_name_534
    @property
    def conditions(this_921) -> 'Sequence29[SqlFragment]':
        return this_921.conditions_535
    @property
    def selected_fields(this_924) -> 'Sequence29[SafeIdentifier]':
        return this_924.selected_fields_536
    @property
    def order_clauses(this_927) -> 'Sequence29[OrderClause]':
        return this_927.order_clauses_537
    @property
    def limit_val(this_930) -> 'Union40[int31, None]':
        return this_930.limit_val_538
    @property
    def offset_val(this_933) -> 'Union40[int31, None]':
        return this_933.offset_val_539
class SafeIdentifier(metaclass = ABCMeta28):
    pass
class ValidatedIdentifier_115(SafeIdentifier):
    value_621: 'str27'
    __slots__ = ('value_621',)
    @property
    def sql_value(this_116) -> 'str27':
        return this_116.value_621
    def __init__(this_218, value_625: 'str27') -> None:
        this_218.value_621 = value_625
class FieldType(metaclass = ABCMeta28):
    pass
class StringField(FieldType):
    __slots__ = ()
    def __init__(this_224) -> None:
        pass
class IntField(FieldType):
    __slots__ = ()
    def __init__(this_226) -> None:
        pass
class Int64Field(FieldType):
    __slots__ = ()
    def __init__(this_228) -> None:
        pass
class FloatField(FieldType):
    __slots__ = ()
    def __init__(this_230) -> None:
        pass
class BoolField(FieldType):
    __slots__ = ()
    def __init__(this_232) -> None:
        pass
class DateField(FieldType):
    __slots__ = ()
    def __init__(this_234) -> None:
        pass
class FieldDef:
    name_639: 'SafeIdentifier'
    field_type_640: 'FieldType'
    nullable_641: 'bool33'
    __slots__ = ('name_639', 'field_type_640', 'nullable_641')
    def __init__(this_236, name_643: 'SafeIdentifier', field_type_644: 'FieldType', nullable_645: 'bool33') -> None:
        this_236.name_639 = name_643
        this_236.field_type_640 = field_type_644
        this_236.nullable_641 = nullable_645
    @property
    def name(this_850) -> 'SafeIdentifier':
        return this_850.name_639
    @property
    def field_type(this_853) -> 'FieldType':
        return this_853.field_type_640
    @property
    def nullable(this_856) -> 'bool33':
        return this_856.nullable_641
class TableDef:
    table_name_646: 'SafeIdentifier'
    fields_647: 'Sequence29[FieldDef]'
    __slots__ = ('table_name_646', 'fields_647')
    def field(this_117, name_649: 'str27') -> 'FieldDef':
        return_241: 'FieldDef'
        with Label35() as fn_650:
            this_2984: 'Sequence29[FieldDef]' = this_117.fields_647
            n_2985: 'int31' = len_4830(this_2984)
            i_2986: 'int31' = 0
            while i_2986 < n_2985:
                el_2987: 'FieldDef' = list_get_4838(this_2984, i_2986)
                i_2986 = int_add_4839(i_2986, 1)
                f_651: 'FieldDef' = el_2987
                if f_651.name.sql_value == name_649:
                    return_241 = f_651
                    fn_650.break_()
            raise RuntimeError30()
        return return_241
    def __init__(this_238, table_name_653: 'SafeIdentifier', fields_654: 'Sequence29[FieldDef]') -> None:
        this_238.table_name_646 = table_name_653
        this_238.fields_647 = fields_654
    @property
    def table_name(this_859) -> 'SafeIdentifier':
        return this_859.table_name_646
    @property
    def fields(this_862) -> 'Sequence29[FieldDef]':
        return this_862.fields_647
T_136 = TypeVar42('T_136', bound = Any41)
class SqlBuilder:
    buffer_674: 'MutableSequence36[SqlPart]'
    __slots__ = ('buffer_674',)
    def append_safe(this_118, sql_source_676: 'str27') -> 'None':
        t_4789: 'SqlSource' = SqlSource(sql_source_676)
        this_118.buffer_674.append(t_4789)
    def append_fragment(this_119, fragment_679: 'SqlFragment') -> 'None':
        t_4787: 'Sequence29[SqlPart]' = fragment_679.parts
        list_builder_add_all_4842(this_119.buffer_674, t_4787)
    def append_part(this_120, part_682: 'SqlPart') -> 'None':
        this_120.buffer_674.append(part_682)
    def append_part_list(this_121, values_685: 'Sequence29[SqlPart]') -> 'None':
        def fn_4783(x_687: 'SqlPart') -> 'None':
            this_121.append_part(x_687)
        this_121.append_list_730(values_685, fn_4783)
    def append_boolean(this_122, value_689: 'bool33') -> 'None':
        t_4780: 'SqlBoolean' = SqlBoolean(value_689)
        this_122.buffer_674.append(t_4780)
    def append_boolean_list(this_123, values_692: 'Sequence29[bool33]') -> 'None':
        def fn_4777(x_694: 'bool33') -> 'None':
            this_123.append_boolean(x_694)
        this_123.append_list_730(values_692, fn_4777)
    def append_date(this_124, value_696: 'date26') -> 'None':
        t_4774: 'SqlDate' = SqlDate(value_696)
        this_124.buffer_674.append(t_4774)
    def append_date_list(this_125, values_699: 'Sequence29[date26]') -> 'None':
        def fn_4771(x_701: 'date26') -> 'None':
            this_125.append_date(x_701)
        this_125.append_list_730(values_699, fn_4771)
    def append_float64(this_126, value_703: 'float38') -> 'None':
        t_4768: 'SqlFloat64' = SqlFloat64(value_703)
        this_126.buffer_674.append(t_4768)
    def append_float64_list(this_127, values_706: 'Sequence29[float38]') -> 'None':
        def fn_4765(x_708: 'float38') -> 'None':
            this_127.append_float64(x_708)
        this_127.append_list_730(values_706, fn_4765)
    def append_int32(this_128, value_710: 'int31') -> 'None':
        t_4762: 'SqlInt32' = SqlInt32(value_710)
        this_128.buffer_674.append(t_4762)
    def append_int32_list(this_129, values_713: 'Sequence29[int31]') -> 'None':
        def fn_4759(x_715: 'int31') -> 'None':
            this_129.append_int32(x_715)
        this_129.append_list_730(values_713, fn_4759)
    def append_int64(this_130, value_717: 'int64_23') -> 'None':
        t_4756: 'SqlInt64' = SqlInt64(value_717)
        this_130.buffer_674.append(t_4756)
    def append_int64_list(this_131, values_720: 'Sequence29[int64_23]') -> 'None':
        def fn_4753(x_722: 'int64_23') -> 'None':
            this_131.append_int64(x_722)
        this_131.append_list_730(values_720, fn_4753)
    def append_string(this_132, value_724: 'str27') -> 'None':
        t_4750: 'SqlString' = SqlString(value_724)
        this_132.buffer_674.append(t_4750)
    def append_string_list(this_133, values_727: 'Sequence29[str27]') -> 'None':
        def fn_4747(x_729: 'str27') -> 'None':
            this_133.append_string(x_729)
        this_133.append_list_730(values_727, fn_4747)
    def append_list_730(this_134, values_731: 'Sequence29[T_136]', append_value_732: 'Callable43[[T_136], None]') -> 'None':
        t_4742: 'int31'
        t_4744: 'T_136'
        i_734: 'int31' = 0
        while True:
            t_4742 = len_4830(values_731)
            if not i_734 < t_4742:
                break
            if i_734 > 0:
                this_134.append_safe(', ')
            t_4744 = list_get_4838(values_731, i_734)
            append_value_732(t_4744)
            i_734 = int_add_4839(i_734, 1)
    @property
    def accumulated(this_135) -> 'SqlFragment':
        return SqlFragment(tuple_4829(this_135.buffer_674))
    def __init__(this_243) -> None:
        t_4739: 'MutableSequence36[SqlPart]' = list_4826()
        this_243.buffer_674 = t_4739
class SqlFragment:
    parts_741: 'Sequence29[SqlPart]'
    __slots__ = ('parts_741',)
    def to_source(this_140) -> 'SqlSource':
        return SqlSource(this_140.to_string())
    def to_string(this_141) -> 'str27':
        t_4809: 'int31'
        builder_746: 'list3[str27]' = ['']
        i_747: 'int31' = 0
        while True:
            t_4809 = len_4830(this_141.parts_741)
            if not i_747 < t_4809:
                break
            list_get_4838(this_141.parts_741, i_747).format_to(builder_746)
            i_747 = int_add_4839(i_747, 1)
        return ''.join(builder_746)
    def __init__(this_264, parts_749: 'Sequence29[SqlPart]') -> None:
        this_264.parts_741 = parts_749
    @property
    def parts(this_868) -> 'Sequence29[SqlPart]':
        return this_868.parts_741
class SqlPart(metaclass = ABCMeta28):
    def format_to(this_142, builder_751: 'list3[str27]') -> 'None':
        raise RuntimeError30()
class SqlSource(SqlPart):
    "`SqlSource` represents known-safe SQL source code that doesn't need escaped."
    source_753: 'str27'
    __slots__ = ('source_753',)
    def format_to(this_143, builder_755: 'list3[str27]') -> 'None':
        builder_755.append(this_143.source_753)
    def __init__(this_270, source_758: 'str27') -> None:
        this_270.source_753 = source_758
    @property
    def source(this_865) -> 'str27':
        return this_865.source_753
class SqlBoolean(SqlPart):
    value_759: 'bool33'
    __slots__ = ('value_759',)
    def format_to(this_144, builder_761: 'list3[str27]') -> 'None':
        t_2868: 'str27'
        if this_144.value_759:
            t_2868 = 'TRUE'
        else:
            t_2868 = 'FALSE'
        builder_761.append(t_2868)
    def __init__(this_273, value_764: 'bool33') -> None:
        this_273.value_759 = value_764
    @property
    def value(this_871) -> 'bool33':
        return this_871.value_759
class SqlDate(SqlPart):
    value_765: 'date26'
    __slots__ = ('value_765',)
    def format_to(this_145, builder_767: 'list3[str27]') -> 'None':
        builder_767.append("'")
        t_4792: 'str27' = date_to_string_4846(this_145.value_765)
        builder_767.append(t_4792)
        builder_767.append("'")
    def __init__(this_276, value_770: 'date26') -> None:
        this_276.value_765 = value_770
    @property
    def value(this_886) -> 'date26':
        return this_886.value_765
class SqlFloat64(SqlPart):
    value_771: 'float38'
    __slots__ = ('value_771',)
    def format_to(this_146, builder_773: 'list3[str27]') -> 'None':
        t_4795: 'str27' = float64_to_string_4847(this_146.value_771)
        builder_773.append(t_4795)
    def __init__(this_279, value_776: 'float38') -> None:
        this_279.value_771 = value_776
    @property
    def value(this_883) -> 'float38':
        return this_883.value_771
class SqlInt32(SqlPart):
    value_777: 'int31'
    __slots__ = ('value_777',)
    def format_to(this_147, builder_779: 'list3[str27]') -> 'None':
        t_4799: 'str27' = int_to_string_4833(this_147.value_777)
        builder_779.append(t_4799)
    def __init__(this_282, value_782: 'int31') -> None:
        this_282.value_777 = value_782
    @property
    def value(this_877) -> 'int31':
        return this_877.value_777
class SqlInt64(SqlPart):
    value_783: 'int64_23'
    __slots__ = ('value_783',)
    def format_to(this_148, builder_785: 'list3[str27]') -> 'None':
        t_4797: 'str27' = int_to_string_4833(this_148.value_783)
        builder_785.append(t_4797)
    def __init__(this_285, value_788: 'int64_23') -> None:
        this_285.value_783 = value_788
    @property
    def value(this_880) -> 'int64_23':
        return this_880.value_783
class SqlString(SqlPart):
    '`SqlString` represents text data that needs escaped.'
    value_789: 'str27'
    __slots__ = ('value_789',)
    def format_to(this_149, builder_791: 'list3[str27]') -> 'None':
        builder_791.append("'")
        def fn_4802(c_793: 'int31') -> 'None':
            if c_793 == 39:
                builder_791.append("''")
            else:
                builder_791.append(string_from_code_point44(c_793))
        string_for_each_4849(this_149.value_789, fn_4802)
        builder_791.append("'")
    def __init__(this_288, value_795: 'str27') -> None:
        this_288.value_789 = value_795
    @property
    def value(this_874) -> 'str27':
        return this_874.value_789
def changeset(table_def_435: 'TableDef', params_436: 'MappingProxyType32[str27, str27]') -> 'Changeset':
    t_4591: 'MappingProxyType32[str27, str27]' = map_constructor_4850(())
    return ChangesetImpl_91(table_def_435, params_436, t_4591, (), True)
def is_ident_start_296(c_626: 'int31') -> 'bool33':
    return_221: 'bool33'
    t_2636: 'bool33'
    t_2637: 'bool33'
    if c_626 >= 97:
        t_2636 = c_626 <= 122
    else:
        t_2636 = False
    if t_2636:
        return_221 = True
    else:
        if c_626 >= 65:
            t_2637 = c_626 <= 90
        else:
            t_2637 = False
        if t_2637:
            return_221 = True
        else:
            return_221 = c_626 == 95
    return return_221
def is_ident_part_297(c_628: 'int31') -> 'bool33':
    return_222: 'bool33'
    if is_ident_start_296(c_628):
        return_222 = True
    elif c_628 >= 48:
        return_222 = c_628 <= 57
    else:
        return_222 = False
    return return_222
def safe_identifier(name_630: 'str27') -> 'SafeIdentifier':
    t_4589: 'int31'
    if not name_630:
        raise RuntimeError30()
    idx_632: 'int31' = 0
    if not is_ident_start_296(string_get_4851(name_630, idx_632)):
        raise RuntimeError30()
    t_4586: 'int31' = string_next_4852(name_630, idx_632)
    idx_632 = t_4586
    while True:
        if not len6(name_630) > idx_632:
            break
        if not is_ident_part_297(string_get_4851(name_630, idx_632)):
            raise RuntimeError30()
        t_4589 = string_next_4852(name_630, idx_632)
        idx_632 = t_4589
    return ValidatedIdentifier_115(name_630)
def delete_sql(table_def_525: 'TableDef', id_526: 'int31') -> 'SqlFragment':
    b_528: 'SqlBuilder' = SqlBuilder()
    b_528.append_safe('DELETE FROM ')
    b_528.append_safe(table_def_525.table_name.sql_value)
    b_528.append_safe(' WHERE id = ')
    b_528.append_int32(id_526)
    return b_528.accumulated
def from_(table_name_577: 'SafeIdentifier') -> 'Query':
    return Query(table_name_577, (), (), (), None, None)
