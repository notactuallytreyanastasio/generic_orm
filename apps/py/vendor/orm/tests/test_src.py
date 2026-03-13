from temper_std.testing import Test
from builtins import str as str27, bool as bool33, Exception as Exception37, int as int31
from unittest import TestCase as TestCase46
from types import MappingProxyType as MappingProxyType32
from typing import Sequence as Sequence29
from datetime import date as date26
from orm.src import SafeIdentifier, safe_identifier, TableDef, FieldDef, StringField, IntField, FloatField, BoolField, map_constructor_4850, pair_4854, changeset, Changeset, mapped_has_4827, len_4830, list_get_4838, str_cat_4832, list_for_each_4824, SqlFragment, from_, Query, SqlBuilder, date_4857, SqlString, SqlInt32, SqlPart
def csid_293(name_438: 'str27') -> 'SafeIdentifier':
    t_2624: 'SafeIdentifier'
    t_2624 = safe_identifier(name_438)
    return t_2624
def user_table_294() -> 'TableDef':
    return TableDef(csid_293('users'), (FieldDef(csid_293('name'), StringField(), False), FieldDef(csid_293('email'), StringField(), False), FieldDef(csid_293('age'), IntField(), True), FieldDef(csid_293('score'), FloatField(), True), FieldDef(csid_293('active'), BoolField(), True)))
class TestCase45(TestCase46):
    def test___castWhitelistsAllowedFields__888(self) -> None:
        'cast whitelists allowed fields'
        test_20: Test = Test()
        try:
            params_442: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Alice'), pair_4854('email', 'alice@example.com'), pair_4854('admin', 'true')))
            t_4547: 'TableDef' = user_table_294()
            t_4548: 'SafeIdentifier' = csid_293('name')
            t_4549: 'SafeIdentifier' = csid_293('email')
            cs_443: 'Changeset' = changeset(t_4547, params_442).cast((t_4548, t_4549))
            t_4552: 'bool33' = mapped_has_4827(cs_443.changes, 'name')
            def fn_4542() -> 'str27':
                return 'name should be in changes'
            test_20.assert_(t_4552, fn_4542)
            t_4556: 'bool33' = mapped_has_4827(cs_443.changes, 'email')
            def fn_4541() -> 'str27':
                return 'email should be in changes'
            test_20.assert_(t_4556, fn_4541)
            t_4562: 'bool33' = not mapped_has_4827(cs_443.changes, 'admin')
            def fn_4540() -> 'str27':
                return 'admin must be dropped (not in whitelist)'
            test_20.assert_(t_4562, fn_4540)
            t_4564: 'bool33' = cs_443.is_valid
            def fn_4539() -> 'str27':
                return 'should still be valid'
            test_20.assert_(t_4564, fn_4539)
        finally:
            test_20.soft_fail_to_hard()
class TestCase47(TestCase46):
    def test___castIsReplacingNotAdditiveSecondCallResetsWhitelist__889(self) -> None:
        'cast is replacing not additive \u2014 second call resets whitelist'
        test_21: Test = Test()
        try:
            params_445: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Alice'), pair_4854('email', 'alice@example.com')))
            t_4525: 'TableDef' = user_table_294()
            t_4526: 'SafeIdentifier' = csid_293('name')
            cs_446: 'Changeset' = changeset(t_4525, params_445).cast((t_4526,)).cast((csid_293('email'),))
            t_4533: 'bool33' = not mapped_has_4827(cs_446.changes, 'name')
            def fn_4521() -> 'str27':
                return 'name must be excluded by second cast'
            test_21.assert_(t_4533, fn_4521)
            t_4536: 'bool33' = mapped_has_4827(cs_446.changes, 'email')
            def fn_4520() -> 'str27':
                return 'email should be present'
            test_21.assert_(t_4536, fn_4520)
        finally:
            test_21.soft_fail_to_hard()
class TestCase48(TestCase46):
    def test___castIgnoresEmptyStringValues__890(self) -> None:
        'cast ignores empty string values'
        test_22: Test = Test()
        try:
            params_448: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', ''), pair_4854('email', 'bob@example.com')))
            t_4507: 'TableDef' = user_table_294()
            t_4508: 'SafeIdentifier' = csid_293('name')
            t_4509: 'SafeIdentifier' = csid_293('email')
            cs_449: 'Changeset' = changeset(t_4507, params_448).cast((t_4508, t_4509))
            t_4514: 'bool33' = not mapped_has_4827(cs_449.changes, 'name')
            def fn_4503() -> 'str27':
                return 'empty name should not be in changes'
            test_22.assert_(t_4514, fn_4503)
            t_4517: 'bool33' = mapped_has_4827(cs_449.changes, 'email')
            def fn_4502() -> 'str27':
                return 'email should be in changes'
            test_22.assert_(t_4517, fn_4502)
        finally:
            test_22.soft_fail_to_hard()
class TestCase49(TestCase46):
    def test___validateRequiredPassesWhenFieldPresent__891(self) -> None:
        'validateRequired passes when field present'
        test_23: Test = Test()
        try:
            params_451: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Alice'),))
            t_4489: 'TableDef' = user_table_294()
            t_4490: 'SafeIdentifier' = csid_293('name')
            cs_452: 'Changeset' = changeset(t_4489, params_451).cast((t_4490,)).validate_required((csid_293('name'),))
            t_4494: 'bool33' = cs_452.is_valid
            def fn_4486() -> 'str27':
                return 'should be valid'
            test_23.assert_(t_4494, fn_4486)
            t_4500: 'bool33' = len_4830(cs_452.errors) == 0
            def fn_4485() -> 'str27':
                return 'no errors expected'
            test_23.assert_(t_4500, fn_4485)
        finally:
            test_23.soft_fail_to_hard()
class TestCase50(TestCase46):
    def test___validateRequiredFailsWhenFieldMissing__892(self) -> None:
        'validateRequired fails when field missing'
        test_24: Test = Test()
        try:
            params_454: 'MappingProxyType32[str27, str27]' = map_constructor_4850(())
            t_4465: 'TableDef' = user_table_294()
            t_4466: 'SafeIdentifier' = csid_293('name')
            cs_455: 'Changeset' = changeset(t_4465, params_454).cast((t_4466,)).validate_required((csid_293('name'),))
            t_4472: 'bool33' = not cs_455.is_valid
            def fn_4463() -> 'str27':
                return 'should be invalid'
            test_24.assert_(t_4472, fn_4463)
            t_4477: 'bool33' = len_4830(cs_455.errors) == 1
            def fn_4462() -> 'str27':
                return 'should have one error'
            test_24.assert_(t_4477, fn_4462)
            t_4483: 'bool33' = list_get_4838(cs_455.errors, 0).field == 'name'
            def fn_4461() -> 'str27':
                return 'error should name the field'
            test_24.assert_(t_4483, fn_4461)
        finally:
            test_24.soft_fail_to_hard()
class TestCase51(TestCase46):
    def test___validateLengthPassesWithinRange__893(self) -> None:
        'validateLength passes within range'
        test_25: Test = Test()
        try:
            params_457: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Alice'),))
            t_4453: 'TableDef' = user_table_294()
            t_4454: 'SafeIdentifier' = csid_293('name')
            cs_458: 'Changeset' = changeset(t_4453, params_457).cast((t_4454,)).validate_length(csid_293('name'), 2, 50)
            t_4458: 'bool33' = cs_458.is_valid
            def fn_4450() -> 'str27':
                return 'should be valid'
            test_25.assert_(t_4458, fn_4450)
        finally:
            test_25.soft_fail_to_hard()
class TestCase52(TestCase46):
    def test___validateLengthFailsWhenTooShort__894(self) -> None:
        'validateLength fails when too short'
        test_26: Test = Test()
        try:
            params_460: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'A'),))
            t_4441: 'TableDef' = user_table_294()
            t_4442: 'SafeIdentifier' = csid_293('name')
            cs_461: 'Changeset' = changeset(t_4441, params_460).cast((t_4442,)).validate_length(csid_293('name'), 2, 50)
            t_4448: 'bool33' = not cs_461.is_valid
            def fn_4438() -> 'str27':
                return 'should be invalid'
            test_26.assert_(t_4448, fn_4438)
        finally:
            test_26.soft_fail_to_hard()
class TestCase53(TestCase46):
    def test___validateLengthFailsWhenTooLong__895(self) -> None:
        'validateLength fails when too long'
        test_27: Test = Test()
        try:
            params_463: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'),))
            t_4429: 'TableDef' = user_table_294()
            t_4430: 'SafeIdentifier' = csid_293('name')
            cs_464: 'Changeset' = changeset(t_4429, params_463).cast((t_4430,)).validate_length(csid_293('name'), 2, 10)
            t_4436: 'bool33' = not cs_464.is_valid
            def fn_4426() -> 'str27':
                return 'should be invalid'
            test_27.assert_(t_4436, fn_4426)
        finally:
            test_27.soft_fail_to_hard()
class TestCase54(TestCase46):
    def test___validateIntPassesForValidInteger__896(self) -> None:
        'validateInt passes for valid integer'
        test_28: Test = Test()
        try:
            params_466: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('age', '30'),))
            t_4418: 'TableDef' = user_table_294()
            t_4419: 'SafeIdentifier' = csid_293('age')
            cs_467: 'Changeset' = changeset(t_4418, params_466).cast((t_4419,)).validate_int(csid_293('age'))
            t_4423: 'bool33' = cs_467.is_valid
            def fn_4415() -> 'str27':
                return 'should be valid'
            test_28.assert_(t_4423, fn_4415)
        finally:
            test_28.soft_fail_to_hard()
class TestCase55(TestCase46):
    def test___validateIntFailsForNonInteger__897(self) -> None:
        'validateInt fails for non-integer'
        test_29: Test = Test()
        try:
            params_469: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('age', 'not-a-number'),))
            t_4406: 'TableDef' = user_table_294()
            t_4407: 'SafeIdentifier' = csid_293('age')
            cs_470: 'Changeset' = changeset(t_4406, params_469).cast((t_4407,)).validate_int(csid_293('age'))
            t_4413: 'bool33' = not cs_470.is_valid
            def fn_4403() -> 'str27':
                return 'should be invalid'
            test_29.assert_(t_4413, fn_4403)
        finally:
            test_29.soft_fail_to_hard()
class TestCase56(TestCase46):
    def test___validateFloatPassesForValidFloat__898(self) -> None:
        'validateFloat passes for valid float'
        test_30: Test = Test()
        try:
            params_472: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('score', '9.5'),))
            t_4395: 'TableDef' = user_table_294()
            t_4396: 'SafeIdentifier' = csid_293('score')
            cs_473: 'Changeset' = changeset(t_4395, params_472).cast((t_4396,)).validate_float(csid_293('score'))
            t_4400: 'bool33' = cs_473.is_valid
            def fn_4392() -> 'str27':
                return 'should be valid'
            test_30.assert_(t_4400, fn_4392)
        finally:
            test_30.soft_fail_to_hard()
class TestCase57(TestCase46):
    def test___validateInt64_passesForValid64_bitInteger__899(self) -> None:
        'validateInt64 passes for valid 64-bit integer'
        test_31: Test = Test()
        try:
            params_475: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('age', '9999999999'),))
            t_4384: 'TableDef' = user_table_294()
            t_4385: 'SafeIdentifier' = csid_293('age')
            cs_476: 'Changeset' = changeset(t_4384, params_475).cast((t_4385,)).validate_int64(csid_293('age'))
            t_4389: 'bool33' = cs_476.is_valid
            def fn_4381() -> 'str27':
                return 'should be valid'
            test_31.assert_(t_4389, fn_4381)
        finally:
            test_31.soft_fail_to_hard()
class TestCase58(TestCase46):
    def test___validateInt64_failsForNonInteger__900(self) -> None:
        'validateInt64 fails for non-integer'
        test_32: Test = Test()
        try:
            params_478: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('age', 'not-a-number'),))
            t_4372: 'TableDef' = user_table_294()
            t_4373: 'SafeIdentifier' = csid_293('age')
            cs_479: 'Changeset' = changeset(t_4372, params_478).cast((t_4373,)).validate_int64(csid_293('age'))
            t_4379: 'bool33' = not cs_479.is_valid
            def fn_4369() -> 'str27':
                return 'should be invalid'
            test_32.assert_(t_4379, fn_4369)
        finally:
            test_32.soft_fail_to_hard()
class TestCase59(TestCase46):
    def test___validateBoolAcceptsTrue1_yesOn__901(self) -> None:
        'validateBool accepts true/1/yes/on'
        test_33: Test = Test()
        try:
            def fn_4366(v_481: 'str27') -> 'None':
                params_482: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('active', v_481),))
                t_4358: 'TableDef' = user_table_294()
                t_4359: 'SafeIdentifier' = csid_293('active')
                cs_483: 'Changeset' = changeset(t_4358, params_482).cast((t_4359,)).validate_bool(csid_293('active'))
                t_4363: 'bool33' = cs_483.is_valid
                def fn_4355() -> 'str27':
                    return str_cat_4832('should accept: ', v_481)
                test_33.assert_(t_4363, fn_4355)
            list_for_each_4824(('true', '1', 'yes', 'on'), fn_4366)
        finally:
            test_33.soft_fail_to_hard()
class TestCase60(TestCase46):
    def test___validateBoolAcceptsFalse0_noOff__902(self) -> None:
        'validateBool accepts false/0/no/off'
        test_34: Test = Test()
        try:
            def fn_4352(v_485: 'str27') -> 'None':
                params_486: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('active', v_485),))
                t_4344: 'TableDef' = user_table_294()
                t_4345: 'SafeIdentifier' = csid_293('active')
                cs_487: 'Changeset' = changeset(t_4344, params_486).cast((t_4345,)).validate_bool(csid_293('active'))
                t_4349: 'bool33' = cs_487.is_valid
                def fn_4341() -> 'str27':
                    return str_cat_4832('should accept: ', v_485)
                test_34.assert_(t_4349, fn_4341)
            list_for_each_4824(('false', '0', 'no', 'off'), fn_4352)
        finally:
            test_34.soft_fail_to_hard()
class TestCase61(TestCase46):
    def test___validateBoolRejectsAmbiguousValues__903(self) -> None:
        'validateBool rejects ambiguous values'
        test_35: Test = Test()
        try:
            def fn_4338(v_489: 'str27') -> 'None':
                params_490: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('active', v_489),))
                t_4329: 'TableDef' = user_table_294()
                t_4330: 'SafeIdentifier' = csid_293('active')
                cs_491: 'Changeset' = changeset(t_4329, params_490).cast((t_4330,)).validate_bool(csid_293('active'))
                t_4336: 'bool33' = not cs_491.is_valid
                def fn_4326() -> 'str27':
                    return str_cat_4832('should reject ambiguous: ', v_489)
                test_35.assert_(t_4336, fn_4326)
            list_for_each_4824(('TRUE', 'Yes', 'maybe', '2', 'enabled'), fn_4338)
        finally:
            test_35.soft_fail_to_hard()
class TestCase62(TestCase46):
    def test___toInsertSqlEscapesBobbyTables__904(self) -> None:
        'toInsertSql escapes Bobby Tables'
        test_36: Test = Test()
        try:
            params_493: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', "Robert'); DROP TABLE users;--"), pair_4854('email', 'bobby@evil.com')))
            t_4314: 'TableDef' = user_table_294()
            t_4315: 'SafeIdentifier' = csid_293('name')
            t_4316: 'SafeIdentifier' = csid_293('email')
            cs_494: 'Changeset' = changeset(t_4314, params_493).cast((t_4315, t_4316)).validate_required((csid_293('name'), csid_293('email')))
            t_2425: 'SqlFragment'
            t_2425 = cs_494.to_insert_sql()
            sql_frag_495: 'SqlFragment' = t_2425
            s_496: 'str27' = sql_frag_495.to_string()
            t_4323: 'bool33' = s_496.find("''") >= 0
            def fn_4310() -> 'str27':
                return str_cat_4832('single quote must be doubled: ', s_496)
            test_36.assert_(t_4323, fn_4310)
        finally:
            test_36.soft_fail_to_hard()
class TestCase63(TestCase46):
    def test___toInsertSqlProducesCorrectSqlForStringField__905(self) -> None:
        'toInsertSql produces correct SQL for string field'
        test_37: Test = Test()
        try:
            params_498: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Alice'), pair_4854('email', 'a@example.com')))
            t_4294: 'TableDef' = user_table_294()
            t_4295: 'SafeIdentifier' = csid_293('name')
            t_4296: 'SafeIdentifier' = csid_293('email')
            cs_499: 'Changeset' = changeset(t_4294, params_498).cast((t_4295, t_4296)).validate_required((csid_293('name'), csid_293('email')))
            t_2404: 'SqlFragment'
            t_2404 = cs_499.to_insert_sql()
            sql_frag_500: 'SqlFragment' = t_2404
            s_501: 'str27' = sql_frag_500.to_string()
            t_4303: 'bool33' = s_501.find('INSERT INTO users') >= 0
            def fn_4290() -> 'str27':
                return str_cat_4832('has INSERT INTO: ', s_501)
            test_37.assert_(t_4303, fn_4290)
            t_4307: 'bool33' = s_501.find("'Alice'") >= 0
            def fn_4289() -> 'str27':
                return str_cat_4832('has quoted name: ', s_501)
            test_37.assert_(t_4307, fn_4289)
        finally:
            test_37.soft_fail_to_hard()
class TestCase64(TestCase46):
    def test___toInsertSqlProducesCorrectSqlForIntField__906(self) -> None:
        'toInsertSql produces correct SQL for int field'
        test_38: Test = Test()
        try:
            params_503: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Bob'), pair_4854('email', 'b@example.com'), pair_4854('age', '25')))
            t_4276: 'TableDef' = user_table_294()
            t_4277: 'SafeIdentifier' = csid_293('name')
            t_4278: 'SafeIdentifier' = csid_293('email')
            t_4279: 'SafeIdentifier' = csid_293('age')
            cs_504: 'Changeset' = changeset(t_4276, params_503).cast((t_4277, t_4278, t_4279)).validate_required((csid_293('name'), csid_293('email')))
            t_2387: 'SqlFragment'
            t_2387 = cs_504.to_insert_sql()
            sql_frag_505: 'SqlFragment' = t_2387
            s_506: 'str27' = sql_frag_505.to_string()
            t_4286: 'bool33' = s_506.find('25') >= 0
            def fn_4271() -> 'str27':
                return str_cat_4832('age rendered unquoted: ', s_506)
            test_38.assert_(t_4286, fn_4271)
        finally:
            test_38.soft_fail_to_hard()
class TestCase65(TestCase46):
    def test___toInsertSqlBubblesOnInvalidChangeset__907(self) -> None:
        'toInsertSql bubbles on invalid changeset'
        test_39: Test = Test()
        try:
            params_508: 'MappingProxyType32[str27, str27]' = map_constructor_4850(())
            t_4264: 'TableDef' = user_table_294()
            t_4265: 'SafeIdentifier' = csid_293('name')
            cs_509: 'Changeset' = changeset(t_4264, params_508).cast((t_4265,)).validate_required((csid_293('name'),))
            did_bubble_510: 'bool33'
            try:
                cs_509.to_insert_sql()
                did_bubble_510 = False
            except Exception37:
                did_bubble_510 = True
            def fn_4262() -> 'str27':
                return 'invalid changeset should bubble'
            test_39.assert_(did_bubble_510, fn_4262)
        finally:
            test_39.soft_fail_to_hard()
class TestCase66(TestCase46):
    def test___toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__908(self) -> None:
        'toInsertSql enforces non-nullable fields independently of isValid'
        test_40: Test = Test()
        try:
            strict_table_512: 'TableDef' = TableDef(csid_293('posts'), (FieldDef(csid_293('title'), StringField(), False), FieldDef(csid_293('body'), StringField(), True)))
            params_513: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('body', 'hello'),))
            t_4255: 'SafeIdentifier' = csid_293('body')
            cs_514: 'Changeset' = changeset(strict_table_512, params_513).cast((t_4255,))
            t_4257: 'bool33' = cs_514.is_valid
            def fn_4244() -> 'str27':
                return 'changeset should appear valid (no explicit validation run)'
            test_40.assert_(t_4257, fn_4244)
            did_bubble_515: 'bool33'
            try:
                cs_514.to_insert_sql()
                did_bubble_515 = False
            except Exception37:
                did_bubble_515 = True
            def fn_4243() -> 'str27':
                return 'toInsertSql should enforce nullable regardless of isValid'
            test_40.assert_(did_bubble_515, fn_4243)
        finally:
            test_40.soft_fail_to_hard()
class TestCase67(TestCase46):
    def test___toUpdateSqlProducesCorrectSql__909(self) -> None:
        'toUpdateSql produces correct SQL'
        test_41: Test = Test()
        try:
            params_517: 'MappingProxyType32[str27, str27]' = map_constructor_4850((pair_4854('name', 'Bob'),))
            t_4234: 'TableDef' = user_table_294()
            t_4235: 'SafeIdentifier' = csid_293('name')
            cs_518: 'Changeset' = changeset(t_4234, params_517).cast((t_4235,)).validate_required((csid_293('name'),))
            t_2347: 'SqlFragment'
            t_2347 = cs_518.to_update_sql(42)
            sql_frag_519: 'SqlFragment' = t_2347
            s_520: 'str27' = sql_frag_519.to_string()
            t_4241: 'bool33' = s_520 == "UPDATE users SET name = 'Bob' WHERE id = 42"
            def fn_4231() -> 'str27':
                return str_cat_4832('got: ', s_520)
            test_41.assert_(t_4241, fn_4231)
        finally:
            test_41.soft_fail_to_hard()
class TestCase68(TestCase46):
    def test___toUpdateSqlBubblesOnInvalidChangeset__910(self) -> None:
        'toUpdateSql bubbles on invalid changeset'
        test_42: Test = Test()
        try:
            params_522: 'MappingProxyType32[str27, str27]' = map_constructor_4850(())
            t_4224: 'TableDef' = user_table_294()
            t_4225: 'SafeIdentifier' = csid_293('name')
            cs_523: 'Changeset' = changeset(t_4224, params_522).cast((t_4225,)).validate_required((csid_293('name'),))
            did_bubble_524: 'bool33'
            try:
                cs_523.to_update_sql(1)
                did_bubble_524 = False
            except Exception37:
                did_bubble_524 = True
            def fn_4222() -> 'str27':
                return 'invalid changeset should bubble'
            test_42.assert_(did_bubble_524, fn_4222)
        finally:
            test_42.soft_fail_to_hard()
def sid_295(name_579: 'str27') -> 'SafeIdentifier':
    t_2261: 'SafeIdentifier'
    t_2261 = safe_identifier(name_579)
    return t_2261
class TestCase69(TestCase46):
    def test___bareFromProducesSelect__935(self) -> None:
        'bare from produces SELECT *'
        test_43: Test = Test()
        try:
            q_582: 'Query' = from_(sid_295('users'))
            t_4157: 'bool33' = q_582.to_sql().to_string() == 'SELECT * FROM users'
            def fn_4152() -> 'str27':
                return 'bare query'
            test_43.assert_(t_4157, fn_4152)
        finally:
            test_43.soft_fail_to_hard()
class TestCase70(TestCase46):
    def test___selectRestrictsColumns__936(self) -> None:
        'select restricts columns'
        test_44: Test = Test()
        try:
            t_4143: 'SafeIdentifier' = sid_295('users')
            t_4144: 'SafeIdentifier' = sid_295('id')
            t_4145: 'SafeIdentifier' = sid_295('name')
            q_584: 'Query' = from_(t_4143).select((t_4144, t_4145))
            t_4150: 'bool33' = q_584.to_sql().to_string() == 'SELECT id, name FROM users'
            def fn_4142() -> 'str27':
                return 'select columns'
            test_44.assert_(t_4150, fn_4142)
        finally:
            test_44.soft_fail_to_hard()
class TestCase71(TestCase46):
    def test___whereAddsConditionWithIntValue__937(self) -> None:
        'where adds condition with int value'
        test_45: Test = Test()
        try:
            t_4131: 'SafeIdentifier' = sid_295('users')
            t_4132: 'SqlBuilder' = SqlBuilder()
            t_4132.append_safe('age > ')
            t_4132.append_int32(18)
            t_4135: 'SqlFragment' = t_4132.accumulated
            q_586: 'Query' = from_(t_4131).where(t_4135)
            t_4140: 'bool33' = q_586.to_sql().to_string() == 'SELECT * FROM users WHERE age > 18'
            def fn_4130() -> 'str27':
                return 'where int'
            test_45.assert_(t_4140, fn_4130)
        finally:
            test_45.soft_fail_to_hard()
class TestCase72(TestCase46):
    def test___whereAddsConditionWithBoolValue__939(self) -> None:
        'where adds condition with bool value'
        test_46: Test = Test()
        try:
            t_4119: 'SafeIdentifier' = sid_295('users')
            t_4120: 'SqlBuilder' = SqlBuilder()
            t_4120.append_safe('active = ')
            t_4120.append_boolean(True)
            t_4123: 'SqlFragment' = t_4120.accumulated
            q_588: 'Query' = from_(t_4119).where(t_4123)
            t_4128: 'bool33' = q_588.to_sql().to_string() == 'SELECT * FROM users WHERE active = TRUE'
            def fn_4118() -> 'str27':
                return 'where bool'
            test_46.assert_(t_4128, fn_4118)
        finally:
            test_46.soft_fail_to_hard()
class TestCase73(TestCase46):
    def test___chainedWhereUsesAnd__941(self) -> None:
        'chained where uses AND'
        test_47: Test = Test()
        try:
            t_4102: 'SafeIdentifier' = sid_295('users')
            t_4103: 'SqlBuilder' = SqlBuilder()
            t_4103.append_safe('age > ')
            t_4103.append_int32(18)
            t_4106: 'SqlFragment' = t_4103.accumulated
            t_4107: 'Query' = from_(t_4102).where(t_4106)
            t_4108: 'SqlBuilder' = SqlBuilder()
            t_4108.append_safe('active = ')
            t_4108.append_boolean(True)
            q_590: 'Query' = t_4107.where(t_4108.accumulated)
            t_4116: 'bool33' = q_590.to_sql().to_string() == 'SELECT * FROM users WHERE age > 18 AND active = TRUE'
            def fn_4101() -> 'str27':
                return 'chained where'
            test_47.assert_(t_4116, fn_4101)
        finally:
            test_47.soft_fail_to_hard()
class TestCase74(TestCase46):
    def test___orderByAsc__944(self) -> None:
        'orderBy ASC'
        test_48: Test = Test()
        try:
            t_4093: 'SafeIdentifier' = sid_295('users')
            t_4094: 'SafeIdentifier' = sid_295('name')
            q_592: 'Query' = from_(t_4093).order_by(t_4094, True)
            t_4099: 'bool33' = q_592.to_sql().to_string() == 'SELECT * FROM users ORDER BY name ASC'
            def fn_4092() -> 'str27':
                return 'order asc'
            test_48.assert_(t_4099, fn_4092)
        finally:
            test_48.soft_fail_to_hard()
class TestCase75(TestCase46):
    def test___orderByDesc__945(self) -> None:
        'orderBy DESC'
        test_49: Test = Test()
        try:
            t_4084: 'SafeIdentifier' = sid_295('users')
            t_4085: 'SafeIdentifier' = sid_295('created_at')
            q_594: 'Query' = from_(t_4084).order_by(t_4085, False)
            t_4090: 'bool33' = q_594.to_sql().to_string() == 'SELECT * FROM users ORDER BY created_at DESC'
            def fn_4083() -> 'str27':
                return 'order desc'
            test_49.assert_(t_4090, fn_4083)
        finally:
            test_49.soft_fail_to_hard()
class TestCase76(TestCase46):
    def test___limitAndOffset__946(self) -> None:
        'limit and offset'
        test_50: Test = Test()
        try:
            t_2195: 'Query'
            t_2195 = from_(sid_295('users')).limit(10)
            t_2196: 'Query'
            t_2196 = t_2195.offset(20)
            q_596: 'Query' = t_2196
            t_4081: 'bool33' = q_596.to_sql().to_string() == 'SELECT * FROM users LIMIT 10 OFFSET 20'
            def fn_4076() -> 'str27':
                return 'limit/offset'
            test_50.assert_(t_4081, fn_4076)
        finally:
            test_50.soft_fail_to_hard()
class TestCase77(TestCase46):
    def test___limitBubblesOnNegative__947(self) -> None:
        'limit bubbles on negative'
        test_51: Test = Test()
        try:
            did_bubble_598: 'bool33'
            try:
                from_(sid_295('users')).limit(-1)
                did_bubble_598 = False
            except Exception37:
                did_bubble_598 = True
            def fn_4072() -> 'str27':
                return 'negative limit should bubble'
            test_51.assert_(did_bubble_598, fn_4072)
        finally:
            test_51.soft_fail_to_hard()
class TestCase78(TestCase46):
    def test___offsetBubblesOnNegative__948(self) -> None:
        'offset bubbles on negative'
        test_52: Test = Test()
        try:
            did_bubble_600: 'bool33'
            try:
                from_(sid_295('users')).offset(-1)
                did_bubble_600 = False
            except Exception37:
                did_bubble_600 = True
            def fn_4068() -> 'str27':
                return 'negative offset should bubble'
            test_52.assert_(did_bubble_600, fn_4068)
        finally:
            test_52.soft_fail_to_hard()
class TestCase79(TestCase46):
    def test___complexComposedQuery__949(self) -> None:
        'complex composed query'
        test_53: Test = Test()
        try:
            min_age_602: 'int31' = 21
            t_4046: 'SafeIdentifier' = sid_295('users')
            t_4047: 'SafeIdentifier' = sid_295('id')
            t_4048: 'SafeIdentifier' = sid_295('name')
            t_4049: 'SafeIdentifier' = sid_295('email')
            t_4050: 'Query' = from_(t_4046).select((t_4047, t_4048, t_4049))
            t_4051: 'SqlBuilder' = SqlBuilder()
            t_4051.append_safe('age >= ')
            t_4051.append_int32(21)
            t_4055: 'Query' = t_4050.where(t_4051.accumulated)
            t_4056: 'SqlBuilder' = SqlBuilder()
            t_4056.append_safe('active = ')
            t_4056.append_boolean(True)
            t_2181: 'Query'
            t_2181 = t_4055.where(t_4056.accumulated).order_by(sid_295('name'), True).limit(25)
            t_2182: 'Query'
            t_2182 = t_2181.offset(0)
            q_603: 'Query' = t_2182
            t_4066: 'bool33' = q_603.to_sql().to_string() == 'SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0'
            def fn_4045() -> 'str27':
                return 'complex query'
            test_53.assert_(t_4066, fn_4045)
        finally:
            test_53.soft_fail_to_hard()
class TestCase80(TestCase46):
    def test___safeToSqlAppliesDefaultLimitWhenNoneSet__952(self) -> None:
        'safeToSql applies default limit when none set'
        test_54: Test = Test()
        try:
            q_605: 'Query' = from_(sid_295('users'))
            t_2158: 'SqlFragment'
            t_2158 = q_605.safe_to_sql(100)
            t_2159: 'SqlFragment' = t_2158
            s_606: 'str27' = t_2159.to_string()
            t_4043: 'bool33' = s_606 == 'SELECT * FROM users LIMIT 100'
            def fn_4039() -> 'str27':
                return str_cat_4832('should have limit: ', s_606)
            test_54.assert_(t_4043, fn_4039)
        finally:
            test_54.soft_fail_to_hard()
class TestCase81(TestCase46):
    def test___safeToSqlRespectsExplicitLimit__953(self) -> None:
        'safeToSql respects explicit limit'
        test_55: Test = Test()
        try:
            t_2150: 'Query'
            t_2150 = from_(sid_295('users')).limit(5)
            q_608: 'Query' = t_2150
            t_2153: 'SqlFragment'
            t_2153 = q_608.safe_to_sql(100)
            t_2154: 'SqlFragment' = t_2153
            s_609: 'str27' = t_2154.to_string()
            t_4037: 'bool33' = s_609 == 'SELECT * FROM users LIMIT 5'
            def fn_4033() -> 'str27':
                return str_cat_4832('explicit limit preserved: ', s_609)
            test_55.assert_(t_4037, fn_4033)
        finally:
            test_55.soft_fail_to_hard()
class TestCase82(TestCase46):
    def test___safeToSqlBubblesOnNegativeDefaultLimit__954(self) -> None:
        'safeToSql bubbles on negative defaultLimit'
        test_56: Test = Test()
        try:
            did_bubble_611: 'bool33'
            try:
                from_(sid_295('users')).safe_to_sql(-1)
                did_bubble_611 = False
            except Exception37:
                did_bubble_611 = True
            def fn_4029() -> 'str27':
                return 'negative defaultLimit should bubble'
            test_56.assert_(did_bubble_611, fn_4029)
        finally:
            test_56.soft_fail_to_hard()
class TestCase83(TestCase46):
    def test___whereWithInjectionAttemptInStringValueIsEscaped__955(self) -> None:
        'where with injection attempt in string value is escaped'
        test_57: Test = Test()
        try:
            evil_613: 'str27' = "'; DROP TABLE users; --"
            t_4013: 'SafeIdentifier' = sid_295('users')
            t_4014: 'SqlBuilder' = SqlBuilder()
            t_4014.append_safe('name = ')
            t_4014.append_string("'; DROP TABLE users; --")
            t_4017: 'SqlFragment' = t_4014.accumulated
            q_614: 'Query' = from_(t_4013).where(t_4017)
            s_615: 'str27' = q_614.to_sql().to_string()
            t_4022: 'bool33' = s_615.find("''") >= 0
            def fn_4012() -> 'str27':
                return str_cat_4832('quotes must be doubled: ', s_615)
            test_57.assert_(t_4022, fn_4012)
            t_4026: 'bool33' = s_615.find('SELECT * FROM users WHERE name =') >= 0
            def fn_4011() -> 'str27':
                return str_cat_4832('structure intact: ', s_615)
            test_57.assert_(t_4026, fn_4011)
        finally:
            test_57.soft_fail_to_hard()
class TestCase84(TestCase46):
    def test___safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__957(self) -> None:
        'safeIdentifier rejects user-supplied table name with metacharacters'
        test_58: Test = Test()
        try:
            attack_617: 'str27' = 'users; DROP TABLE users; --'
            did_bubble_618: 'bool33'
            try:
                safe_identifier('users; DROP TABLE users; --')
                did_bubble_618 = False
            except Exception37:
                did_bubble_618 = True
            def fn_4008() -> 'str27':
                return 'metacharacter-containing name must be rejected at construction'
            test_58.assert_(did_bubble_618, fn_4008)
        finally:
            test_58.soft_fail_to_hard()
class TestCase85(TestCase46):
    def test___safeIdentifierAcceptsValidNames__958(self) -> None:
        'safeIdentifier accepts valid names'
        test_65: Test = Test()
        try:
            t_2123: 'SafeIdentifier'
            t_2123 = safe_identifier('user_name')
            id_656: 'SafeIdentifier' = t_2123
            t_4006: 'bool33' = id_656.sql_value == 'user_name'
            def fn_4003() -> 'str27':
                return 'value should round-trip'
            test_65.assert_(t_4006, fn_4003)
        finally:
            test_65.soft_fail_to_hard()
class TestCase86(TestCase46):
    def test___safeIdentifierRejectsEmptyString__959(self) -> None:
        'safeIdentifier rejects empty string'
        test_66: Test = Test()
        try:
            did_bubble_658: 'bool33'
            try:
                safe_identifier('')
                did_bubble_658 = False
            except Exception37:
                did_bubble_658 = True
            def fn_4000() -> 'str27':
                return 'empty string should bubble'
            test_66.assert_(did_bubble_658, fn_4000)
        finally:
            test_66.soft_fail_to_hard()
class TestCase87(TestCase46):
    def test___safeIdentifierRejectsLeadingDigit__960(self) -> None:
        'safeIdentifier rejects leading digit'
        test_67: Test = Test()
        try:
            did_bubble_660: 'bool33'
            try:
                safe_identifier('1col')
                did_bubble_660 = False
            except Exception37:
                did_bubble_660 = True
            def fn_3997() -> 'str27':
                return 'leading digit should bubble'
            test_67.assert_(did_bubble_660, fn_3997)
        finally:
            test_67.soft_fail_to_hard()
class TestCase88(TestCase46):
    def test___safeIdentifierRejectsSqlMetacharacters__961(self) -> None:
        'safeIdentifier rejects SQL metacharacters'
        test_68: Test = Test()
        try:
            cases_662: 'Sequence29[str27]' = ('name); DROP TABLE', "col'", 'a b', 'a-b', 'a.b', 'a;b')
            def fn_3994(c_663: 'str27') -> 'None':
                did_bubble_664: 'bool33'
                try:
                    safe_identifier(c_663)
                    did_bubble_664 = False
                except Exception37:
                    did_bubble_664 = True
                def fn_3991() -> 'str27':
                    return str_cat_4832('should reject: ', c_663)
                test_68.assert_(did_bubble_664, fn_3991)
            list_for_each_4824(cases_662, fn_3994)
        finally:
            test_68.soft_fail_to_hard()
class TestCase89(TestCase46):
    def test___tableDefFieldLookupFound__962(self) -> None:
        'TableDef field lookup - found'
        test_69: Test = Test()
        try:
            t_2100: 'SafeIdentifier'
            t_2100 = safe_identifier('users')
            t_2101: 'SafeIdentifier' = t_2100
            t_2102: 'SafeIdentifier'
            t_2102 = safe_identifier('name')
            t_2103: 'SafeIdentifier' = t_2102
            t_3981: 'StringField' = StringField()
            t_3982: 'FieldDef' = FieldDef(t_2103, t_3981, False)
            t_2106: 'SafeIdentifier'
            t_2106 = safe_identifier('age')
            t_2107: 'SafeIdentifier' = t_2106
            t_3983: 'IntField' = IntField()
            t_3984: 'FieldDef' = FieldDef(t_2107, t_3983, False)
            td_666: 'TableDef' = TableDef(t_2101, (t_3982, t_3984))
            t_2111: 'FieldDef'
            t_2111 = td_666.field('age')
            f_667: 'FieldDef' = t_2111
            t_3989: 'bool33' = f_667.name.sql_value == 'age'
            def fn_3980() -> 'str27':
                return 'should find age field'
            test_69.assert_(t_3989, fn_3980)
        finally:
            test_69.soft_fail_to_hard()
class TestCase90(TestCase46):
    def test___tableDefFieldLookupNotFoundBubbles__963(self) -> None:
        'TableDef field lookup - not found bubbles'
        test_70: Test = Test()
        try:
            t_2091: 'SafeIdentifier'
            t_2091 = safe_identifier('users')
            t_2092: 'SafeIdentifier' = t_2091
            t_2093: 'SafeIdentifier'
            t_2093 = safe_identifier('name')
            t_2094: 'SafeIdentifier' = t_2093
            t_3975: 'StringField' = StringField()
            t_3976: 'FieldDef' = FieldDef(t_2094, t_3975, False)
            td_669: 'TableDef' = TableDef(t_2092, (t_3976,))
            did_bubble_670: 'bool33'
            try:
                td_669.field('nonexistent')
                did_bubble_670 = False
            except Exception37:
                did_bubble_670 = True
            def fn_3974() -> 'str27':
                return 'unknown field should bubble'
            test_70.assert_(did_bubble_670, fn_3974)
        finally:
            test_70.soft_fail_to_hard()
class TestCase91(TestCase46):
    def test___fieldDefNullableFlag__964(self) -> None:
        'FieldDef nullable flag'
        test_71: Test = Test()
        try:
            t_2079: 'SafeIdentifier'
            t_2079 = safe_identifier('email')
            t_2080: 'SafeIdentifier' = t_2079
            t_3963: 'StringField' = StringField()
            required_672: 'FieldDef' = FieldDef(t_2080, t_3963, False)
            t_2083: 'SafeIdentifier'
            t_2083 = safe_identifier('bio')
            t_2084: 'SafeIdentifier' = t_2083
            t_3965: 'StringField' = StringField()
            optional_673: 'FieldDef' = FieldDef(t_2084, t_3965, True)
            t_3969: 'bool33' = not required_672.nullable
            def fn_3962() -> 'str27':
                return 'required field should not be nullable'
            test_71.assert_(t_3969, fn_3962)
            t_3971: 'bool33' = optional_673.nullable
            def fn_3961() -> 'str27':
                return 'optional field should be nullable'
            test_71.assert_(t_3971, fn_3961)
        finally:
            test_71.soft_fail_to_hard()
class TestCase92(TestCase46):
    def test___stringEscaping__965(self) -> None:
        'string escaping'
        test_73: Test = Test()
        try:
            def build_797(name_799: 'str27') -> 'str27':
                t_3943: 'SqlBuilder' = SqlBuilder()
                t_3943.append_safe('select * from hi where name = ')
                t_3943.append_string(name_799)
                return t_3943.accumulated.to_string()
            def build_wrong_798(name_801: 'str27') -> 'str27':
                return str_cat_4832("select * from hi where name = '", name_801, "'")
            actual_967: 'str27' = build_797('world')
            t_3953: 'bool33' = actual_967 == "select * from hi where name = 'world'"
            def fn_3950() -> 'str27':
                return str_cat_4832('expected build("world") == (', "select * from hi where name = 'world'", ') not (', actual_967, ')')
            test_73.assert_(t_3953, fn_3950)
            bobby_tables_803: 'str27' = "Robert'); drop table hi;--"
            actual_969: 'str27' = build_797("Robert'); drop table hi;--")
            t_3957: 'bool33' = actual_969 == "select * from hi where name = 'Robert''); drop table hi;--'"
            def fn_3949() -> 'str27':
                return str_cat_4832('expected build(bobbyTables) == (', "select * from hi where name = 'Robert''); drop table hi;--'", ') not (', actual_969, ')')
            test_73.assert_(t_3957, fn_3949)
            def fn_3948() -> 'str27':
                return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')"
            test_73.assert_(True, fn_3948)
        finally:
            test_73.soft_fail_to_hard()
class TestCase93(TestCase46):
    def test___stringEdgeCases__973(self) -> None:
        'string edge cases'
        test_74: Test = Test()
        try:
            t_3911: 'SqlBuilder' = SqlBuilder()
            t_3911.append_safe('v = ')
            t_3911.append_string('')
            actual_974: 'str27' = t_3911.accumulated.to_string()
            t_3917: 'bool33' = actual_974 == "v = ''"
            def fn_3910() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "").toString() == (', "v = ''", ') not (', actual_974, ')')
            test_74.assert_(t_3917, fn_3910)
            t_3919: 'SqlBuilder' = SqlBuilder()
            t_3919.append_safe('v = ')
            t_3919.append_string("a''b")
            actual_977: 'str27' = t_3919.accumulated.to_string()
            t_3925: 'bool33' = actual_977 == "v = 'a''''b'"
            def fn_3909() -> 'str27':
                return str_cat_4832("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (", "v = 'a''''b'", ') not (', actual_977, ')')
            test_74.assert_(t_3925, fn_3909)
            t_3927: 'SqlBuilder' = SqlBuilder()
            t_3927.append_safe('v = ')
            t_3927.append_string('Hello \u4e16\u754c')
            actual_980: 'str27' = t_3927.accumulated.to_string()
            t_3933: 'bool33' = actual_980 == "v = 'Hello \u4e16\u754c'"
            def fn_3908() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Hello \u4e16\u754c").toString() == (', "v = 'Hello \u4e16\u754c'", ') not (', actual_980, ')')
            test_74.assert_(t_3933, fn_3908)
            t_3935: 'SqlBuilder' = SqlBuilder()
            t_3935.append_safe('v = ')
            t_3935.append_string('Line1\nLine2')
            actual_983: 'str27' = t_3935.accumulated.to_string()
            t_3941: 'bool33' = actual_983 == "v = 'Line1\nLine2'"
            def fn_3907() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Line1\\nLine2").toString() == (', "v = 'Line1\nLine2'", ') not (', actual_983, ')')
            test_74.assert_(t_3941, fn_3907)
        finally:
            test_74.soft_fail_to_hard()
class TestCase94(TestCase46):
    def test___numbersAndBooleans__986(self) -> None:
        'numbers and booleans'
        test_75: Test = Test()
        try:
            t_3882: 'SqlBuilder' = SqlBuilder()
            t_3882.append_safe('select ')
            t_3882.append_int32(42)
            t_3882.append_safe(', ')
            t_3882.append_int64(43)
            t_3882.append_safe(', ')
            t_3882.append_float64(19.99)
            t_3882.append_safe(', ')
            t_3882.append_boolean(True)
            t_3882.append_safe(', ')
            t_3882.append_boolean(False)
            actual_987: 'str27' = t_3882.accumulated.to_string()
            t_3896: 'bool33' = actual_987 == 'select 42, 43, 19.99, TRUE, FALSE'
            def fn_3881() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, 42, ", ", \\interpolate, 43, ", ", \\interpolate, 19.99, ", ", \\interpolate, true, ", ", \\interpolate, false).toString() == (', 'select 42, 43, 19.99, TRUE, FALSE', ') not (', actual_987, ')')
            test_75.assert_(t_3896, fn_3881)
            t_2024: 'date26'
            t_2024 = date_4857(2024, 12, 25)
            date_806: 'date26' = t_2024
            t_3898: 'SqlBuilder' = SqlBuilder()
            t_3898.append_safe('insert into t values (')
            t_3898.append_date(date_806)
            t_3898.append_safe(')')
            actual_990: 'str27' = t_3898.accumulated.to_string()
            t_3905: 'bool33' = actual_990 == "insert into t values ('2024-12-25')"
            def fn_3880() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "insert into t values (", \\interpolate, date, ")").toString() == (', "insert into t values ('2024-12-25')", ') not (', actual_990, ')')
            test_75.assert_(t_3905, fn_3880)
        finally:
            test_75.soft_fail_to_hard()
class TestCase95(TestCase46):
    def test___lists__993(self) -> None:
        'lists'
        test_76: Test = Test()
        try:
            t_3826: 'SqlBuilder' = SqlBuilder()
            t_3826.append_safe('v IN (')
            t_3826.append_string_list(('a', 'b', "c'd"))
            t_3826.append_safe(')')
            actual_994: 'str27' = t_3826.accumulated.to_string()
            t_3833: 'bool33' = actual_994 == "v IN ('a', 'b', 'c''d')"
            def fn_3825() -> 'str27':
                return str_cat_4832("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (", "v IN ('a', 'b', 'c''d')", ') not (', actual_994, ')')
            test_76.assert_(t_3833, fn_3825)
            t_3835: 'SqlBuilder' = SqlBuilder()
            t_3835.append_safe('v IN (')
            t_3835.append_int32_list((1, 2, 3))
            t_3835.append_safe(')')
            actual_997: 'str27' = t_3835.accumulated.to_string()
            t_3842: 'bool33' = actual_997 == 'v IN (1, 2, 3)'
            def fn_3824() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2, 3), ")").toString() == (', 'v IN (1, 2, 3)', ') not (', actual_997, ')')
            test_76.assert_(t_3842, fn_3824)
            t_3844: 'SqlBuilder' = SqlBuilder()
            t_3844.append_safe('v IN (')
            t_3844.append_int64_list((1, 2))
            t_3844.append_safe(')')
            actual_1000: 'str27' = t_3844.accumulated.to_string()
            t_3851: 'bool33' = actual_1000 == 'v IN (1, 2)'
            def fn_3823() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2), ")").toString() == (', 'v IN (1, 2)', ') not (', actual_1000, ')')
            test_76.assert_(t_3851, fn_3823)
            t_3853: 'SqlBuilder' = SqlBuilder()
            t_3853.append_safe('v IN (')
            t_3853.append_float64_list((1.0, 2.0))
            t_3853.append_safe(')')
            actual_1003: 'str27' = t_3853.accumulated.to_string()
            t_3860: 'bool33' = actual_1003 == 'v IN (1.0, 2.0)'
            def fn_3822() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1.0, 2.0), ")").toString() == (', 'v IN (1.0, 2.0)', ') not (', actual_1003, ')')
            test_76.assert_(t_3860, fn_3822)
            t_3862: 'SqlBuilder' = SqlBuilder()
            t_3862.append_safe('v IN (')
            t_3862.append_boolean_list((True, False))
            t_3862.append_safe(')')
            actual_1006: 'str27' = t_3862.accumulated.to_string()
            t_3869: 'bool33' = actual_1006 == 'v IN (TRUE, FALSE)'
            def fn_3821() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(true, false), ")").toString() == (', 'v IN (TRUE, FALSE)', ') not (', actual_1006, ')')
            test_76.assert_(t_3869, fn_3821)
            t_1996: 'date26'
            t_1996 = date_4857(2024, 1, 1)
            t_1997: 'date26' = t_1996
            t_1998: 'date26'
            t_1998 = date_4857(2024, 12, 25)
            t_1999: 'date26' = t_1998
            dates_808: 'Sequence29[date26]' = (t_1997, t_1999)
            t_3871: 'SqlBuilder' = SqlBuilder()
            t_3871.append_safe('v IN (')
            t_3871.append_date_list(dates_808)
            t_3871.append_safe(')')
            actual_1009: 'str27' = t_3871.accumulated.to_string()
            t_3878: 'bool33' = actual_1009 == "v IN ('2024-01-01', '2024-12-25')"
            def fn_3820() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, dates, ")").toString() == (', "v IN ('2024-01-01', '2024-12-25')", ') not (', actual_1009, ')')
            test_76.assert_(t_3878, fn_3820)
        finally:
            test_76.soft_fail_to_hard()
class TestCase96(TestCase46):
    def test___nesting__1012(self) -> None:
        'nesting'
        test_77: Test = Test()
        try:
            name_810: 'str27' = 'Someone'
            t_3789: 'SqlBuilder' = SqlBuilder()
            t_3789.append_safe('where p.last_name = ')
            t_3789.append_string('Someone')
            condition_811: 'SqlFragment' = t_3789.accumulated
            t_3793: 'SqlBuilder' = SqlBuilder()
            t_3793.append_safe('select p.id from person p ')
            t_3793.append_fragment(condition_811)
            actual_1014: 'str27' = t_3793.accumulated.to_string()
            t_3799: 'bool33' = actual_1014 == "select p.id from person p where p.last_name = 'Someone'"
            def fn_3788() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_1014, ')')
            test_77.assert_(t_3799, fn_3788)
            t_3801: 'SqlBuilder' = SqlBuilder()
            t_3801.append_safe('select p.id from person p ')
            t_3801.append_part(condition_811.to_source())
            actual_1017: 'str27' = t_3801.accumulated.to_string()
            t_3808: 'bool33' = actual_1017 == "select p.id from person p where p.last_name = 'Someone'"
            def fn_3787() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition.toSource()).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_1017, ')')
            test_77.assert_(t_3808, fn_3787)
            parts_812: 'Sequence29[SqlPart]' = (SqlString("a'b"), SqlInt32(3))
            t_3812: 'SqlBuilder' = SqlBuilder()
            t_3812.append_safe('select ')
            t_3812.append_part_list(parts_812)
            actual_1020: 'str27' = t_3812.accumulated.to_string()
            t_3818: 'bool33' = actual_1020 == "select 'a''b', 3"
            def fn_3786() -> 'str27':
                return str_cat_4832('expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, parts).toString() == (', "select 'a''b', 3", ') not (', actual_1020, ')')
            test_77.assert_(t_3818, fn_3786)
        finally:
            test_77.soft_fail_to_hard()
