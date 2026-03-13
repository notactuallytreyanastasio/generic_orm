local temper = require('temper-core');
local safeIdentifier, TableDef, FieldDef, StringField, IntField, FloatField, BoolField, changeset, from, SqlBuilder, SqlString, SqlInt32, local_518, local_519, csid__293, userTable__294, sid__295, exports;
safeIdentifier = temper.import('orm/src', 'safeIdentifier');
TableDef = temper.import('orm/src', 'TableDef');
FieldDef = temper.import('orm/src', 'FieldDef');
StringField = temper.import('orm/src', 'StringField');
IntField = temper.import('orm/src', 'IntField');
FloatField = temper.import('orm/src', 'FloatField');
BoolField = temper.import('orm/src', 'BoolField');
changeset = temper.import('orm/src', 'changeset');
from = temper.import('orm/src', 'from');
SqlBuilder = temper.import('orm/src', 'SqlBuilder');
SqlString = temper.import('orm/src', 'SqlString');
SqlInt32 = temper.import('orm/src', 'SqlInt32');
local_518 = (unpack or table.unpack);
local_519 = require('luaunit');
local_519.FAILURE_PREFIX = temper.test_failure_prefix;
Test_ = {};
csid__293 = function(name__438)
  local return__194, t_109, local_110, local_111, local_112;
  local_110, local_111, local_112 = temper.pcall(function()
    t_109 = safeIdentifier(name__438);
    return__194 = t_109;
  end);
  if local_110 then
  else
    return__194 = temper.bubble();
  end
  return return__194;
end;
userTable__294 = function()
  return TableDef(csid__293('users'), temper.listof(FieldDef(csid__293('name'), StringField(), false), FieldDef(csid__293('email'), StringField(), false), FieldDef(csid__293('age'), IntField(), true), FieldDef(csid__293('score'), FloatField(), true), FieldDef(csid__293('active'), BoolField(), true)));
end;
Test_.test_castWhitelistsAllowedFields__888 = function()
  temper.test('cast whitelists allowed fields', function(test_114)
    local params__442, t_115, t_116, t_117, cs__443, t_118, fn__4542, t_119, fn__4541, t_120, fn__4540, t_121, fn__4539;
    params__442 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('email', 'alice@example.com'), temper.pair_constructor('admin', 'true')));
    t_115 = userTable__294();
    t_116 = csid__293('name');
    t_117 = csid__293('email');
    cs__443 = changeset(t_115, params__442):cast(temper.listof(t_116, t_117));
    t_118 = temper.mapped_has(cs__443.changes, 'name');
    fn__4542 = function()
      return 'name should be in changes';
    end;
    temper.test_assert(test_114, t_118, fn__4542);
    t_119 = temper.mapped_has(cs__443.changes, 'email');
    fn__4541 = function()
      return 'email should be in changes';
    end;
    temper.test_assert(test_114, t_119, fn__4541);
    t_120 = not temper.mapped_has(cs__443.changes, 'admin');
    fn__4540 = function()
      return 'admin must be dropped (not in whitelist)';
    end;
    temper.test_assert(test_114, t_120, fn__4540);
    t_121 = cs__443.isValid;
    fn__4539 = function()
      return 'should still be valid';
    end;
    temper.test_assert(test_114, t_121, fn__4539);
    return nil;
  end);
end;
Test_.test_castIsReplacingNotAdditiveSecondCallResetsWhitelist__889 = function()
  temper.test('cast is replacing not additive \xe2\x80\x94 second call resets whitelist', function(test_122)
    local params__445, t_123, t_124, cs__446, t_125, fn__4521, t_126, fn__4520;
    params__445 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('email', 'alice@example.com')));
    t_123 = userTable__294();
    t_124 = csid__293('name');
    cs__446 = changeset(t_123, params__445):cast(temper.listof(t_124)):cast(temper.listof(csid__293('email')));
    t_125 = not temper.mapped_has(cs__446.changes, 'name');
    fn__4521 = function()
      return 'name must be excluded by second cast';
    end;
    temper.test_assert(test_122, t_125, fn__4521);
    t_126 = temper.mapped_has(cs__446.changes, 'email');
    fn__4520 = function()
      return 'email should be present';
    end;
    temper.test_assert(test_122, t_126, fn__4520);
    return nil;
  end);
end;
Test_.test_castIgnoresEmptyStringValues__890 = function()
  temper.test('cast ignores empty string values', function(test_127)
    local params__448, t_128, t_129, t_130, cs__449, t_131, fn__4503, t_132, fn__4502;
    params__448 = temper.map_constructor(temper.listof(temper.pair_constructor('name', ''), temper.pair_constructor('email', 'bob@example.com')));
    t_128 = userTable__294();
    t_129 = csid__293('name');
    t_130 = csid__293('email');
    cs__449 = changeset(t_128, params__448):cast(temper.listof(t_129, t_130));
    t_131 = not temper.mapped_has(cs__449.changes, 'name');
    fn__4503 = function()
      return 'empty name should not be in changes';
    end;
    temper.test_assert(test_127, t_131, fn__4503);
    t_132 = temper.mapped_has(cs__449.changes, 'email');
    fn__4502 = function()
      return 'email should be in changes';
    end;
    temper.test_assert(test_127, t_132, fn__4502);
    return nil;
  end);
end;
Test_.test_validateRequiredPassesWhenFieldPresent__891 = function()
  temper.test('validateRequired passes when field present', function(test_133)
    local params__451, t_134, t_135, cs__452, t_136, fn__4486, t_137, fn__4485;
    params__451 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice')));
    t_134 = userTable__294();
    t_135 = csid__293('name');
    cs__452 = changeset(t_134, params__451):cast(temper.listof(t_135)):validateRequired(temper.listof(csid__293('name')));
    t_136 = cs__452.isValid;
    fn__4486 = function()
      return 'should be valid';
    end;
    temper.test_assert(test_133, t_136, fn__4486);
    t_137 = (temper.list_length(cs__452.errors) == 0);
    fn__4485 = function()
      return 'no errors expected';
    end;
    temper.test_assert(test_133, t_137, fn__4485);
    return nil;
  end);
end;
Test_.test_validateRequiredFailsWhenFieldMissing__892 = function()
  temper.test('validateRequired fails when field missing', function(test_138)
    local params__454, t_139, t_140, cs__455, t_141, fn__4463, t_142, fn__4462, t_143, fn__4461;
    params__454 = temper.map_constructor(temper.listof());
    t_139 = userTable__294();
    t_140 = csid__293('name');
    cs__455 = changeset(t_139, params__454):cast(temper.listof(t_140)):validateRequired(temper.listof(csid__293('name')));
    t_141 = not cs__455.isValid;
    fn__4463 = function()
      return 'should be invalid';
    end;
    temper.test_assert(test_138, t_141, fn__4463);
    t_142 = (temper.list_length(cs__455.errors) == 1);
    fn__4462 = function()
      return 'should have one error';
    end;
    temper.test_assert(test_138, t_142, fn__4462);
    t_143 = temper.str_eq((temper.list_get(cs__455.errors, 0)).field, 'name');
    fn__4461 = function()
      return 'error should name the field';
    end;
    temper.test_assert(test_138, t_143, fn__4461);
    return nil;
  end);
end;
Test_.test_validateLengthPassesWithinRange__893 = function()
  temper.test('validateLength passes within range', function(test_144)
    local params__457, t_145, t_146, cs__458, t_147, fn__4450;
    params__457 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice')));
    t_145 = userTable__294();
    t_146 = csid__293('name');
    cs__458 = changeset(t_145, params__457):cast(temper.listof(t_146)):validateLength(csid__293('name'), 2, 50);
    t_147 = cs__458.isValid;
    fn__4450 = function()
      return 'should be valid';
    end;
    temper.test_assert(test_144, t_147, fn__4450);
    return nil;
  end);
end;
Test_.test_validateLengthFailsWhenTooShort__894 = function()
  temper.test('validateLength fails when too short', function(test_148)
    local params__460, t_149, t_150, cs__461, t_151, fn__4438;
    params__460 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'A')));
    t_149 = userTable__294();
    t_150 = csid__293('name');
    cs__461 = changeset(t_149, params__460):cast(temper.listof(t_150)):validateLength(csid__293('name'), 2, 50);
    t_151 = not cs__461.isValid;
    fn__4438 = function()
      return 'should be invalid';
    end;
    temper.test_assert(test_148, t_151, fn__4438);
    return nil;
  end);
end;
Test_.test_validateLengthFailsWhenTooLong__895 = function()
  temper.test('validateLength fails when too long', function(test_152)
    local params__463, t_153, t_154, cs__464, t_155, fn__4426;
    params__463 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ')));
    t_153 = userTable__294();
    t_154 = csid__293('name');
    cs__464 = changeset(t_153, params__463):cast(temper.listof(t_154)):validateLength(csid__293('name'), 2, 10);
    t_155 = not cs__464.isValid;
    fn__4426 = function()
      return 'should be invalid';
    end;
    temper.test_assert(test_152, t_155, fn__4426);
    return nil;
  end);
end;
Test_.test_validateIntPassesForValidInteger__896 = function()
  temper.test('validateInt passes for valid integer', function(test_156)
    local params__466, t_157, t_158, cs__467, t_159, fn__4415;
    params__466 = temper.map_constructor(temper.listof(temper.pair_constructor('age', '30')));
    t_157 = userTable__294();
    t_158 = csid__293('age');
    cs__467 = changeset(t_157, params__466):cast(temper.listof(t_158)):validateInt(csid__293('age'));
    t_159 = cs__467.isValid;
    fn__4415 = function()
      return 'should be valid';
    end;
    temper.test_assert(test_156, t_159, fn__4415);
    return nil;
  end);
end;
Test_.test_validateIntFailsForNonInteger__897 = function()
  temper.test('validateInt fails for non-integer', function(test_160)
    local params__469, t_161, t_162, cs__470, t_163, fn__4403;
    params__469 = temper.map_constructor(temper.listof(temper.pair_constructor('age', 'not-a-number')));
    t_161 = userTable__294();
    t_162 = csid__293('age');
    cs__470 = changeset(t_161, params__469):cast(temper.listof(t_162)):validateInt(csid__293('age'));
    t_163 = not cs__470.isValid;
    fn__4403 = function()
      return 'should be invalid';
    end;
    temper.test_assert(test_160, t_163, fn__4403);
    return nil;
  end);
end;
Test_.test_validateFloatPassesForValidFloat__898 = function()
  temper.test('validateFloat passes for valid float', function(test_164)
    local params__472, t_165, t_166, cs__473, t_167, fn__4392;
    params__472 = temper.map_constructor(temper.listof(temper.pair_constructor('score', '9.5')));
    t_165 = userTable__294();
    t_166 = csid__293('score');
    cs__473 = changeset(t_165, params__472):cast(temper.listof(t_166)):validateFloat(csid__293('score'));
    t_167 = cs__473.isValid;
    fn__4392 = function()
      return 'should be valid';
    end;
    temper.test_assert(test_164, t_167, fn__4392);
    return nil;
  end);
end;
Test_.test_validateInt64_passesForValid64_bitInteger__899 = function()
  temper.test('validateInt64 passes for valid 64-bit integer', function(test_168)
    local params__475, t_169, t_170, cs__476, t_171, fn__4381;
    params__475 = temper.map_constructor(temper.listof(temper.pair_constructor('age', '9999999999')));
    t_169 = userTable__294();
    t_170 = csid__293('age');
    cs__476 = changeset(t_169, params__475):cast(temper.listof(t_170)):validateInt64(csid__293('age'));
    t_171 = cs__476.isValid;
    fn__4381 = function()
      return 'should be valid';
    end;
    temper.test_assert(test_168, t_171, fn__4381);
    return nil;
  end);
end;
Test_.test_validateInt64_failsForNonInteger__900 = function()
  temper.test('validateInt64 fails for non-integer', function(test_172)
    local params__478, t_173, t_174, cs__479, t_175, fn__4369;
    params__478 = temper.map_constructor(temper.listof(temper.pair_constructor('age', 'not-a-number')));
    t_173 = userTable__294();
    t_174 = csid__293('age');
    cs__479 = changeset(t_173, params__478):cast(temper.listof(t_174)):validateInt64(csid__293('age'));
    t_175 = not cs__479.isValid;
    fn__4369 = function()
      return 'should be invalid';
    end;
    temper.test_assert(test_172, t_175, fn__4369);
    return nil;
  end);
end;
Test_.test_validateBoolAcceptsTrue1_yesOn__901 = function()
  temper.test('validateBool accepts true/1/yes/on', function(test_176)
    local fn__4366;
    fn__4366 = function(v__481)
      local params__482, t_177, t_178, cs__483, t_179, fn__4355;
      params__482 = temper.map_constructor(temper.listof(temper.pair_constructor('active', v__481)));
      t_177 = userTable__294();
      t_178 = csid__293('active');
      cs__483 = changeset(t_177, params__482):cast(temper.listof(t_178)):validateBool(csid__293('active'));
      t_179 = cs__483.isValid;
      fn__4355 = function()
        return temper.concat('should accept: ', v__481);
      end;
      temper.test_assert(test_176, t_179, fn__4355);
      return nil;
    end;
    temper.list_foreach(temper.listof('true', '1', 'yes', 'on'), fn__4366);
    return nil;
  end);
end;
Test_.test_validateBoolAcceptsFalse0_noOff__902 = function()
  temper.test('validateBool accepts false/0/no/off', function(test_180)
    local fn__4352;
    fn__4352 = function(v__485)
      local params__486, t_181, t_182, cs__487, t_183, fn__4341;
      params__486 = temper.map_constructor(temper.listof(temper.pair_constructor('active', v__485)));
      t_181 = userTable__294();
      t_182 = csid__293('active');
      cs__487 = changeset(t_181, params__486):cast(temper.listof(t_182)):validateBool(csid__293('active'));
      t_183 = cs__487.isValid;
      fn__4341 = function()
        return temper.concat('should accept: ', v__485);
      end;
      temper.test_assert(test_180, t_183, fn__4341);
      return nil;
    end;
    temper.list_foreach(temper.listof('false', '0', 'no', 'off'), fn__4352);
    return nil;
  end);
end;
Test_.test_validateBoolRejectsAmbiguousValues__903 = function()
  temper.test('validateBool rejects ambiguous values', function(test_184)
    local fn__4338;
    fn__4338 = function(v__489)
      local params__490, t_185, t_186, cs__491, t_187, fn__4326;
      params__490 = temper.map_constructor(temper.listof(temper.pair_constructor('active', v__489)));
      t_185 = userTable__294();
      t_186 = csid__293('active');
      cs__491 = changeset(t_185, params__490):cast(temper.listof(t_186)):validateBool(csid__293('active'));
      t_187 = not cs__491.isValid;
      fn__4326 = function()
        return temper.concat('should reject ambiguous: ', v__489);
      end;
      temper.test_assert(test_184, t_187, fn__4326);
      return nil;
    end;
    temper.list_foreach(temper.listof('TRUE', 'Yes', 'maybe', '2', 'enabled'), fn__4338);
    return nil;
  end);
end;
Test_.test_toInsertSqlEscapesBobbyTables__904 = function()
  temper.test('toInsertSql escapes Bobby Tables', function(test_188)
    local t_189, params__493, t_190, t_191, t_192, cs__494, sqlFrag__495, local_193, local_194, local_195, s__496, t_197, fn__4310;
    params__493 = temper.map_constructor(temper.listof(temper.pair_constructor('name', "Robert'); DROP TABLE users;--"), temper.pair_constructor('email', 'bobby@evil.com')));
    t_190 = userTable__294();
    t_191 = csid__293('name');
    t_192 = csid__293('email');
    cs__494 = changeset(t_190, params__493):cast(temper.listof(t_191, t_192)):validateRequired(temper.listof(csid__293('name'), csid__293('email')));
    local_193, local_194, local_195 = temper.pcall(function()
      t_189 = cs__494:toInsertSql();
      sqlFrag__495 = t_189;
    end);
    if local_193 then
    else
      sqlFrag__495 = temper.bubble();
    end
    s__496 = sqlFrag__495:toString();
    t_197 = temper.is_string_index(temper.string_indexof(s__496, "''"));
    fn__4310 = function()
      return temper.concat('single quote must be doubled: ', s__496);
    end;
    temper.test_assert(test_188, t_197, fn__4310);
    return nil;
  end);
end;
Test_.test_toInsertSqlProducesCorrectSqlForStringField__905 = function()
  temper.test('toInsertSql produces correct SQL for string field', function(test_198)
    local t_199, params__498, t_200, t_201, t_202, cs__499, sqlFrag__500, local_203, local_204, local_205, s__501, t_207, fn__4290, t_208, fn__4289;
    params__498 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Alice'), temper.pair_constructor('email', 'a@example.com')));
    t_200 = userTable__294();
    t_201 = csid__293('name');
    t_202 = csid__293('email');
    cs__499 = changeset(t_200, params__498):cast(temper.listof(t_201, t_202)):validateRequired(temper.listof(csid__293('name'), csid__293('email')));
    local_203, local_204, local_205 = temper.pcall(function()
      t_199 = cs__499:toInsertSql();
      sqlFrag__500 = t_199;
    end);
    if local_203 then
    else
      sqlFrag__500 = temper.bubble();
    end
    s__501 = sqlFrag__500:toString();
    t_207 = temper.is_string_index(temper.string_indexof(s__501, 'INSERT INTO users'));
    fn__4290 = function()
      return temper.concat('has INSERT INTO: ', s__501);
    end;
    temper.test_assert(test_198, t_207, fn__4290);
    t_208 = temper.is_string_index(temper.string_indexof(s__501, "'Alice'"));
    fn__4289 = function()
      return temper.concat('has quoted name: ', s__501);
    end;
    temper.test_assert(test_198, t_208, fn__4289);
    return nil;
  end);
end;
Test_.test_toInsertSqlProducesCorrectSqlForIntField__906 = function()
  temper.test('toInsertSql produces correct SQL for int field', function(test_209)
    local t_210, params__503, t_211, t_212, t_213, t_214, cs__504, sqlFrag__505, local_215, local_216, local_217, s__506, t_219, fn__4271;
    params__503 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Bob'), temper.pair_constructor('email', 'b@example.com'), temper.pair_constructor('age', '25')));
    t_211 = userTable__294();
    t_212 = csid__293('name');
    t_213 = csid__293('email');
    t_214 = csid__293('age');
    cs__504 = changeset(t_211, params__503):cast(temper.listof(t_212, t_213, t_214)):validateRequired(temper.listof(csid__293('name'), csid__293('email')));
    local_215, local_216, local_217 = temper.pcall(function()
      t_210 = cs__504:toInsertSql();
      sqlFrag__505 = t_210;
    end);
    if local_215 then
    else
      sqlFrag__505 = temper.bubble();
    end
    s__506 = sqlFrag__505:toString();
    t_219 = temper.is_string_index(temper.string_indexof(s__506, '25'));
    fn__4271 = function()
      return temper.concat('age rendered unquoted: ', s__506);
    end;
    temper.test_assert(test_209, t_219, fn__4271);
    return nil;
  end);
end;
Test_.test_toInsertSqlBubblesOnInvalidChangeset__907 = function()
  temper.test('toInsertSql bubbles on invalid changeset', function(test_220)
    local params__508, t_221, t_222, cs__509, didBubble__510, local_223, local_224, local_225, fn__4262;
    params__508 = temper.map_constructor(temper.listof());
    t_221 = userTable__294();
    t_222 = csid__293('name');
    cs__509 = changeset(t_221, params__508):cast(temper.listof(t_222)):validateRequired(temper.listof(csid__293('name')));
    local_223, local_224, local_225 = temper.pcall(function()
      cs__509:toInsertSql();
      didBubble__510 = false;
    end);
    if local_223 then
    else
      didBubble__510 = true;
    end
    fn__4262 = function()
      return 'invalid changeset should bubble';
    end;
    temper.test_assert(test_220, didBubble__510, fn__4262);
    return nil;
  end);
end;
Test_.test_toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__908 = function()
  temper.test('toInsertSql enforces non-nullable fields independently of isValid', function(test_227)
    local strictTable__512, params__513, t_228, cs__514, t_229, fn__4244, didBubble__515, local_230, local_231, local_232, fn__4243;
    strictTable__512 = TableDef(csid__293('posts'), temper.listof(FieldDef(csid__293('title'), StringField(), false), FieldDef(csid__293('body'), StringField(), true)));
    params__513 = temper.map_constructor(temper.listof(temper.pair_constructor('body', 'hello')));
    t_228 = csid__293('body');
    cs__514 = changeset(strictTable__512, params__513):cast(temper.listof(t_228));
    t_229 = cs__514.isValid;
    fn__4244 = function()
      return 'changeset should appear valid (no explicit validation run)';
    end;
    temper.test_assert(test_227, t_229, fn__4244);
    local_230, local_231, local_232 = temper.pcall(function()
      cs__514:toInsertSql();
      didBubble__515 = false;
    end);
    if local_230 then
    else
      didBubble__515 = true;
    end
    fn__4243 = function()
      return 'toInsertSql should enforce nullable regardless of isValid';
    end;
    temper.test_assert(test_227, didBubble__515, fn__4243);
    return nil;
  end);
end;
Test_.test_toUpdateSqlProducesCorrectSql__909 = function()
  temper.test('toUpdateSql produces correct SQL', function(test_234)
    local t_235, params__517, t_236, t_237, cs__518, sqlFrag__519, local_238, local_239, local_240, s__520, t_242, fn__4231;
    params__517 = temper.map_constructor(temper.listof(temper.pair_constructor('name', 'Bob')));
    t_236 = userTable__294();
    t_237 = csid__293('name');
    cs__518 = changeset(t_236, params__517):cast(temper.listof(t_237)):validateRequired(temper.listof(csid__293('name')));
    local_238, local_239, local_240 = temper.pcall(function()
      t_235 = cs__518:toUpdateSql(42);
      sqlFrag__519 = t_235;
    end);
    if local_238 then
    else
      sqlFrag__519 = temper.bubble();
    end
    s__520 = sqlFrag__519:toString();
    t_242 = temper.str_eq(s__520, "UPDATE users SET name = 'Bob' WHERE id = 42");
    fn__4231 = function()
      return temper.concat('got: ', s__520);
    end;
    temper.test_assert(test_234, t_242, fn__4231);
    return nil;
  end);
end;
Test_.test_toUpdateSqlBubblesOnInvalidChangeset__910 = function()
  temper.test('toUpdateSql bubbles on invalid changeset', function(test_243)
    local params__522, t_244, t_245, cs__523, didBubble__524, local_246, local_247, local_248, fn__4222;
    params__522 = temper.map_constructor(temper.listof());
    t_244 = userTable__294();
    t_245 = csid__293('name');
    cs__523 = changeset(t_244, params__522):cast(temper.listof(t_245)):validateRequired(temper.listof(csid__293('name')));
    local_246, local_247, local_248 = temper.pcall(function()
      cs__523:toUpdateSql(1);
      didBubble__524 = false;
    end);
    if local_246 then
    else
      didBubble__524 = true;
    end
    fn__4222 = function()
      return 'invalid changeset should bubble';
    end;
    temper.test_assert(test_243, didBubble__524, fn__4222);
    return nil;
  end);
end;
sid__295 = function(name__579)
  local return__214, t_250, local_251, local_252, local_253;
  local_251, local_252, local_253 = temper.pcall(function()
    t_250 = safeIdentifier(name__579);
    return__214 = t_250;
  end);
  if local_251 then
  else
    return__214 = temper.bubble();
  end
  return return__214;
end;
Test_.test_bareFromProducesSelect__935 = function()
  temper.test('bare from produces SELECT *', function(test_255)
    local q__582, t_256, fn__4152;
    q__582 = from(sid__295('users'));
    t_256 = temper.str_eq(q__582:toSql():toString(), 'SELECT * FROM users');
    fn__4152 = function()
      return 'bare query';
    end;
    temper.test_assert(test_255, t_256, fn__4152);
    return nil;
  end);
end;
Test_.test_selectRestrictsColumns__936 = function()
  temper.test('select restricts columns', function(test_257)
    local t_258, t_259, t_260, q__584, t_261, fn__4142;
    t_258 = sid__295('users');
    t_259 = sid__295('id');
    t_260 = sid__295('name');
    q__584 = from(t_258):select(temper.listof(t_259, t_260));
    t_261 = temper.str_eq(q__584:toSql():toString(), 'SELECT id, name FROM users');
    fn__4142 = function()
      return 'select columns';
    end;
    temper.test_assert(test_257, t_261, fn__4142);
    return nil;
  end);
end;
Test_.test_whereAddsConditionWithIntValue__937 = function()
  temper.test('where adds condition with int value', function(test_262)
    local t_263, t_264, t_265, q__586, t_266, fn__4130;
    t_263 = sid__295('users');
    t_264 = SqlBuilder();
    t_264:appendSafe('age > ');
    t_264:appendInt32(18);
    t_265 = t_264.accumulated;
    q__586 = from(t_263):where(t_265);
    t_266 = temper.str_eq(q__586:toSql():toString(), 'SELECT * FROM users WHERE age > 18');
    fn__4130 = function()
      return 'where int';
    end;
    temper.test_assert(test_262, t_266, fn__4130);
    return nil;
  end);
end;
Test_.test_whereAddsConditionWithBoolValue__939 = function()
  temper.test('where adds condition with bool value', function(test_267)
    local t_268, t_269, t_270, q__588, t_271, fn__4118;
    t_268 = sid__295('users');
    t_269 = SqlBuilder();
    t_269:appendSafe('active = ');
    t_269:appendBoolean(true);
    t_270 = t_269.accumulated;
    q__588 = from(t_268):where(t_270);
    t_271 = temper.str_eq(q__588:toSql():toString(), 'SELECT * FROM users WHERE active = TRUE');
    fn__4118 = function()
      return 'where bool';
    end;
    temper.test_assert(test_267, t_271, fn__4118);
    return nil;
  end);
end;
Test_.test_chainedWhereUsesAnd__941 = function()
  temper.test('chained where uses AND', function(test_272)
    local t_273, t_274, t_275, t_276, t_277, q__590, t_278, fn__4101;
    t_273 = sid__295('users');
    t_274 = SqlBuilder();
    t_274:appendSafe('age > ');
    t_274:appendInt32(18);
    t_275 = t_274.accumulated;
    t_276 = from(t_273):where(t_275);
    t_277 = SqlBuilder();
    t_277:appendSafe('active = ');
    t_277:appendBoolean(true);
    q__590 = t_276:where(t_277.accumulated);
    t_278 = temper.str_eq(q__590:toSql():toString(), 'SELECT * FROM users WHERE age > 18 AND active = TRUE');
    fn__4101 = function()
      return 'chained where';
    end;
    temper.test_assert(test_272, t_278, fn__4101);
    return nil;
  end);
end;
Test_.test_orderByAsc__944 = function()
  temper.test('orderBy ASC', function(test_279)
    local t_280, t_281, q__592, t_282, fn__4092;
    t_280 = sid__295('users');
    t_281 = sid__295('name');
    q__592 = from(t_280):orderBy(t_281, true);
    t_282 = temper.str_eq(q__592:toSql():toString(), 'SELECT * FROM users ORDER BY name ASC');
    fn__4092 = function()
      return 'order asc';
    end;
    temper.test_assert(test_279, t_282, fn__4092);
    return nil;
  end);
end;
Test_.test_orderByDesc__945 = function()
  temper.test('orderBy DESC', function(test_283)
    local t_284, t_285, q__594, t_286, fn__4083;
    t_284 = sid__295('users');
    t_285 = sid__295('created_at');
    q__594 = from(t_284):orderBy(t_285, false);
    t_286 = temper.str_eq(q__594:toSql():toString(), 'SELECT * FROM users ORDER BY created_at DESC');
    fn__4083 = function()
      return 'order desc';
    end;
    temper.test_assert(test_283, t_286, fn__4083);
    return nil;
  end);
end;
Test_.test_limitAndOffset__946 = function()
  temper.test('limit and offset', function(test_287)
    local t_288, t_289, q__596, local_290, local_291, local_292, t_294, fn__4076;
    local_290, local_291, local_292 = temper.pcall(function()
      t_288 = from(sid__295('users')):limit(10);
      t_289 = t_288:offset(20);
      q__596 = t_289;
    end);
    if local_290 then
    else
      q__596 = temper.bubble();
    end
    t_294 = temper.str_eq(q__596:toSql():toString(), 'SELECT * FROM users LIMIT 10 OFFSET 20');
    fn__4076 = function()
      return 'limit/offset';
    end;
    temper.test_assert(test_287, t_294, fn__4076);
    return nil;
  end);
end;
Test_.test_limitBubblesOnNegative__947 = function()
  temper.test('limit bubbles on negative', function(test_295)
    local didBubble__598, local_296, local_297, local_298, fn__4072;
    local_296, local_297, local_298 = temper.pcall(function()
      from(sid__295('users')):limit(-1);
      didBubble__598 = false;
    end);
    if local_296 then
    else
      didBubble__598 = true;
    end
    fn__4072 = function()
      return 'negative limit should bubble';
    end;
    temper.test_assert(test_295, didBubble__598, fn__4072);
    return nil;
  end);
end;
Test_.test_offsetBubblesOnNegative__948 = function()
  temper.test('offset bubbles on negative', function(test_300)
    local didBubble__600, local_301, local_302, local_303, fn__4068;
    local_301, local_302, local_303 = temper.pcall(function()
      from(sid__295('users')):offset(-1);
      didBubble__600 = false;
    end);
    if local_301 then
    else
      didBubble__600 = true;
    end
    fn__4068 = function()
      return 'negative offset should bubble';
    end;
    temper.test_assert(test_300, didBubble__600, fn__4068);
    return nil;
  end);
end;
Test_.test_complexComposedQuery__949 = function()
  temper.test('complex composed query', function(test_305)
    local t_306, t_307, t_308, t_309, t_310, t_311, t_312, t_313, t_314, t_315, minAge__602, q__603, local_316, local_317, local_318, t_320, fn__4045;
    minAge__602 = 21;
    local_316, local_317, local_318 = temper.pcall(function()
      t_306 = sid__295('users');
      t_307 = sid__295('id');
      t_308 = sid__295('name');
      t_309 = sid__295('email');
      t_310 = from(t_306):select(temper.listof(t_307, t_308, t_309));
      t_311 = SqlBuilder();
      t_311:appendSafe('age >= ');
      t_311:appendInt32(21);
      t_312 = t_310:where(t_311.accumulated);
      t_313 = SqlBuilder();
      t_313:appendSafe('active = ');
      t_313:appendBoolean(true);
      t_314 = t_312:where(t_313.accumulated):orderBy(sid__295('name'), true):limit(25);
      t_315 = t_314:offset(0);
      q__603 = t_315;
    end);
    if local_316 then
    else
      q__603 = temper.bubble();
    end
    t_320 = temper.str_eq(q__603:toSql():toString(), 'SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0');
    fn__4045 = function()
      return 'complex query';
    end;
    temper.test_assert(test_305, t_320, fn__4045);
    return nil;
  end);
end;
Test_.test_safeToSqlAppliesDefaultLimitWhenNoneSet__952 = function()
  temper.test('safeToSql applies default limit when none set', function(test_321)
    local t_322, t_323, q__605, local_324, local_325, local_326, s__606, t_328, fn__4039;
    q__605 = from(sid__295('users'));
    local_324, local_325, local_326 = temper.pcall(function()
      t_322 = q__605:safeToSql(100);
      t_323 = t_322;
    end);
    if local_324 then
    else
      t_323 = temper.bubble();
    end
    s__606 = t_323:toString();
    t_328 = temper.str_eq(s__606, 'SELECT * FROM users LIMIT 100');
    fn__4039 = function()
      return temper.concat('should have limit: ', s__606);
    end;
    temper.test_assert(test_321, t_328, fn__4039);
    return nil;
  end);
end;
Test_.test_safeToSqlRespectsExplicitLimit__953 = function()
  temper.test('safeToSql respects explicit limit', function(test_329)
    local t_330, t_331, t_332, q__608, local_333, local_334, local_335, local_337, local_338, local_339, s__609, t_341, fn__4033;
    local_333, local_334, local_335 = temper.pcall(function()
      t_330 = from(sid__295('users')):limit(5);
      q__608 = t_330;
    end);
    if local_333 then
    else
      q__608 = temper.bubble();
    end
    local_337, local_338, local_339 = temper.pcall(function()
      t_331 = q__608:safeToSql(100);
      t_332 = t_331;
    end);
    if local_337 then
    else
      t_332 = temper.bubble();
    end
    s__609 = t_332:toString();
    t_341 = temper.str_eq(s__609, 'SELECT * FROM users LIMIT 5');
    fn__4033 = function()
      return temper.concat('explicit limit preserved: ', s__609);
    end;
    temper.test_assert(test_329, t_341, fn__4033);
    return nil;
  end);
end;
Test_.test_safeToSqlBubblesOnNegativeDefaultLimit__954 = function()
  temper.test('safeToSql bubbles on negative defaultLimit', function(test_342)
    local didBubble__611, local_343, local_344, local_345, fn__4029;
    local_343, local_344, local_345 = temper.pcall(function()
      from(sid__295('users')):safeToSql(-1);
      didBubble__611 = false;
    end);
    if local_343 then
    else
      didBubble__611 = true;
    end
    fn__4029 = function()
      return 'negative defaultLimit should bubble';
    end;
    temper.test_assert(test_342, didBubble__611, fn__4029);
    return nil;
  end);
end;
Test_.test_whereWithInjectionAttemptInStringValueIsEscaped__955 = function()
  temper.test('where with injection attempt in string value is escaped', function(test_347)
    local evil__613, t_348, t_349, t_350, q__614, s__615, t_351, fn__4012, t_352, fn__4011;
    evil__613 = "'; DROP TABLE users; --";
    t_348 = sid__295('users');
    t_349 = SqlBuilder();
    t_349:appendSafe('name = ');
    t_349:appendString("'; DROP TABLE users; --");
    t_350 = t_349.accumulated;
    q__614 = from(t_348):where(t_350);
    s__615 = q__614:toSql():toString();
    t_351 = temper.is_string_index(temper.string_indexof(s__615, "''"));
    fn__4012 = function()
      return temper.concat('quotes must be doubled: ', s__615);
    end;
    temper.test_assert(test_347, t_351, fn__4012);
    t_352 = temper.is_string_index(temper.string_indexof(s__615, 'SELECT * FROM users WHERE name ='));
    fn__4011 = function()
      return temper.concat('structure intact: ', s__615);
    end;
    temper.test_assert(test_347, t_352, fn__4011);
    return nil;
  end);
end;
Test_.test_safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__957 = function()
  temper.test('safeIdentifier rejects user-supplied table name with metacharacters', function(test_353)
    local attack__617, didBubble__618, local_354, local_355, local_356, fn__4008;
    attack__617 = 'users; DROP TABLE users; --';
    local_354, local_355, local_356 = temper.pcall(function()
      safeIdentifier('users; DROP TABLE users; --');
      didBubble__618 = false;
    end);
    if local_354 then
    else
      didBubble__618 = true;
    end
    fn__4008 = function()
      return 'metacharacter-containing name must be rejected at construction';
    end;
    temper.test_assert(test_353, didBubble__618, fn__4008);
    return nil;
  end);
end;
Test_.test_safeIdentifierAcceptsValidNames__958 = function()
  temper.test('safeIdentifier accepts valid names', function(test_358)
    local t_359, id__656, local_360, local_361, local_362, t_364, fn__4003;
    local_360, local_361, local_362 = temper.pcall(function()
      t_359 = safeIdentifier('user_name');
      id__656 = t_359;
    end);
    if local_360 then
    else
      id__656 = temper.bubble();
    end
    t_364 = temper.str_eq(id__656.sqlValue, 'user_name');
    fn__4003 = function()
      return 'value should round-trip';
    end;
    temper.test_assert(test_358, t_364, fn__4003);
    return nil;
  end);
end;
Test_.test_safeIdentifierRejectsEmptyString__959 = function()
  temper.test('safeIdentifier rejects empty string', function(test_365)
    local didBubble__658, local_366, local_367, local_368, fn__4000;
    local_366, local_367, local_368 = temper.pcall(function()
      safeIdentifier('');
      didBubble__658 = false;
    end);
    if local_366 then
    else
      didBubble__658 = true;
    end
    fn__4000 = function()
      return 'empty string should bubble';
    end;
    temper.test_assert(test_365, didBubble__658, fn__4000);
    return nil;
  end);
end;
Test_.test_safeIdentifierRejectsLeadingDigit__960 = function()
  temper.test('safeIdentifier rejects leading digit', function(test_370)
    local didBubble__660, local_371, local_372, local_373, fn__3997;
    local_371, local_372, local_373 = temper.pcall(function()
      safeIdentifier('1col');
      didBubble__660 = false;
    end);
    if local_371 then
    else
      didBubble__660 = true;
    end
    fn__3997 = function()
      return 'leading digit should bubble';
    end;
    temper.test_assert(test_370, didBubble__660, fn__3997);
    return nil;
  end);
end;
Test_.test_safeIdentifierRejectsSqlMetacharacters__961 = function()
  temper.test('safeIdentifier rejects SQL metacharacters', function(test_375)
    local cases__662, fn__3994;
    cases__662 = temper.listof('name); DROP TABLE', "col'", 'a b', 'a-b', 'a.b', 'a;b');
    fn__3994 = function(c__663)
      local didBubble__664, local_376, local_377, local_378, fn__3991;
      local_376, local_377, local_378 = temper.pcall(function()
        safeIdentifier(c__663);
        didBubble__664 = false;
      end);
      if local_376 then
      else
        didBubble__664 = true;
      end
      fn__3991 = function()
        return temper.concat('should reject: ', c__663);
      end;
      temper.test_assert(test_375, didBubble__664, fn__3991);
      return nil;
    end;
    temper.list_foreach(cases__662, fn__3994);
    return nil;
  end);
end;
Test_.test_tableDefFieldLookupFound__962 = function()
  temper.test('TableDef field lookup - found', function(test_380)
    local t_381, t_382, t_383, t_384, t_385, t_386, t_387, local_388, local_389, local_390, local_392, local_393, local_394, t_396, t_397, local_398, local_399, local_400, t_402, t_403, td__666, f__667, local_404, local_405, local_406, t_408, fn__3980;
    local_388, local_389, local_390 = temper.pcall(function()
      t_381 = safeIdentifier('users');
      t_382 = t_381;
    end);
    if local_388 then
    else
      t_382 = temper.bubble();
    end
    local_392, local_393, local_394 = temper.pcall(function()
      t_383 = safeIdentifier('name');
      t_384 = t_383;
    end);
    if local_392 then
    else
      t_384 = temper.bubble();
    end
    t_396 = StringField();
    t_397 = FieldDef(t_384, t_396, false);
    local_398, local_399, local_400 = temper.pcall(function()
      t_385 = safeIdentifier('age');
      t_386 = t_385;
    end);
    if local_398 then
    else
      t_386 = temper.bubble();
    end
    t_402 = IntField();
    t_403 = FieldDef(t_386, t_402, false);
    td__666 = TableDef(t_382, temper.listof(t_397, t_403));
    local_404, local_405, local_406 = temper.pcall(function()
      t_387 = td__666:field('age');
      f__667 = t_387;
    end);
    if local_404 then
    else
      f__667 = temper.bubble();
    end
    t_408 = temper.str_eq(f__667.name.sqlValue, 'age');
    fn__3980 = function()
      return 'should find age field';
    end;
    temper.test_assert(test_380, t_408, fn__3980);
    return nil;
  end);
end;
Test_.test_tableDefFieldLookupNotFoundBubbles__963 = function()
  temper.test('TableDef field lookup - not found bubbles', function(test_409)
    local t_410, t_411, t_412, t_413, local_414, local_415, local_416, local_418, local_419, local_420, t_422, t_423, td__669, didBubble__670, local_424, local_425, local_426, fn__3974;
    local_414, local_415, local_416 = temper.pcall(function()
      t_410 = safeIdentifier('users');
      t_411 = t_410;
    end);
    if local_414 then
    else
      t_411 = temper.bubble();
    end
    local_418, local_419, local_420 = temper.pcall(function()
      t_412 = safeIdentifier('name');
      t_413 = t_412;
    end);
    if local_418 then
    else
      t_413 = temper.bubble();
    end
    t_422 = StringField();
    t_423 = FieldDef(t_413, t_422, false);
    td__669 = TableDef(t_411, temper.listof(t_423));
    local_424, local_425, local_426 = temper.pcall(function()
      td__669:field('nonexistent');
      didBubble__670 = false;
    end);
    if local_424 then
    else
      didBubble__670 = true;
    end
    fn__3974 = function()
      return 'unknown field should bubble';
    end;
    temper.test_assert(test_409, didBubble__670, fn__3974);
    return nil;
  end);
end;
Test_.test_fieldDefNullableFlag__964 = function()
  temper.test('FieldDef nullable flag', function(test_428)
    local t_429, t_430, t_431, t_432, local_433, local_434, local_435, t_437, required__672, local_438, local_439, local_440, t_442, optional__673, t_443, fn__3962, t_444, fn__3961;
    local_433, local_434, local_435 = temper.pcall(function()
      t_429 = safeIdentifier('email');
      t_430 = t_429;
    end);
    if local_433 then
    else
      t_430 = temper.bubble();
    end
    t_437 = StringField();
    required__672 = FieldDef(t_430, t_437, false);
    local_438, local_439, local_440 = temper.pcall(function()
      t_431 = safeIdentifier('bio');
      t_432 = t_431;
    end);
    if local_438 then
    else
      t_432 = temper.bubble();
    end
    t_442 = StringField();
    optional__673 = FieldDef(t_432, t_442, true);
    t_443 = not required__672.nullable;
    fn__3962 = function()
      return 'required field should not be nullable';
    end;
    temper.test_assert(test_428, t_443, fn__3962);
    t_444 = optional__673.nullable;
    fn__3961 = function()
      return 'optional field should be nullable';
    end;
    temper.test_assert(test_428, t_444, fn__3961);
    return nil;
  end);
end;
Test_.test_stringEscaping__965 = function()
  temper.test('string escaping', function(test_445)
    local build__797, buildWrong__798, actual_447, t_448, fn__3950, bobbyTables__803, actual_449, t_450, fn__3949, fn__3948;
    build__797 = function(name__799)
      local t_446;
      t_446 = SqlBuilder();
      t_446:appendSafe('select * from hi where name = ');
      t_446:appendString(name__799);
      return t_446.accumulated:toString();
    end;
    buildWrong__798 = function(name__801)
      return temper.concat("select * from hi where name = '", name__801, "'");
    end;
    actual_447 = build__797('world');
    t_448 = temper.str_eq(actual_447, "select * from hi where name = 'world'");
    fn__3950 = function()
      return temper.concat('expected build("world") == (', "select * from hi where name = 'world'", ') not (', actual_447, ')');
    end;
    temper.test_assert(test_445, t_448, fn__3950);
    bobbyTables__803 = "Robert'); drop table hi;--";
    actual_449 = build__797("Robert'); drop table hi;--");
    t_450 = temper.str_eq(actual_449, "select * from hi where name = 'Robert''); drop table hi;--'");
    fn__3949 = function()
      return temper.concat('expected build(bobbyTables) == (', "select * from hi where name = 'Robert''); drop table hi;--'", ') not (', actual_449, ')');
    end;
    temper.test_assert(test_445, t_450, fn__3949);
    fn__3948 = function()
      return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
    end;
    temper.test_assert(test_445, true, fn__3948);
    return nil;
  end);
end;
Test_.test_stringEdgeCases__973 = function()
  temper.test('string edge cases', function(test_451)
    local t_452, actual_453, t_454, fn__3910, t_455, actual_456, t_457, fn__3909, t_458, actual_459, t_460, fn__3908, t_461, actual_462, t_463, fn__3907;
    t_452 = SqlBuilder();
    t_452:appendSafe('v = ');
    t_452:appendString('');
    actual_453 = t_452.accumulated:toString();
    t_454 = temper.str_eq(actual_453, "v = ''");
    fn__3910 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "").toString() == (', "v = ''", ') not (', actual_453, ')');
    end;
    temper.test_assert(test_451, t_454, fn__3910);
    t_455 = SqlBuilder();
    t_455:appendSafe('v = ');
    t_455:appendString("a''b");
    actual_456 = t_455.accumulated:toString();
    t_457 = temper.str_eq(actual_456, "v = 'a''''b'");
    fn__3909 = function()
      return temper.concat("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (", "v = 'a''''b'", ') not (', actual_456, ')');
    end;
    temper.test_assert(test_451, t_457, fn__3909);
    t_458 = SqlBuilder();
    t_458:appendSafe('v = ');
    t_458:appendString('Hello \xe4\xb8\x96\xe7\x95\x8c');
    actual_459 = t_458.accumulated:toString();
    t_460 = temper.str_eq(actual_459, "v = 'Hello \xe4\xb8\x96\xe7\x95\x8c'");
    fn__3908 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Hello \xe4\xb8\x96\xe7\x95\x8c").toString() == (', "v = 'Hello \xe4\xb8\x96\xe7\x95\x8c'", ') not (', actual_459, ')');
    end;
    temper.test_assert(test_451, t_460, fn__3908);
    t_461 = SqlBuilder();
    t_461:appendSafe('v = ');
    t_461:appendString('Line1\nLine2');
    actual_462 = t_461.accumulated:toString();
    t_463 = temper.str_eq(actual_462, "v = 'Line1\nLine2'");
    fn__3907 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Line1\\nLine2").toString() == (', "v = 'Line1\nLine2'", ') not (', actual_462, ')');
    end;
    temper.test_assert(test_451, t_463, fn__3907);
    return nil;
  end);
end;
Test_.test_numbersAndBooleans__986 = function()
  temper.test('numbers and booleans', function(test_464)
    local t_465, t_466, actual_467, t_468, fn__3881, date__806, local_469, local_470, local_471, t_473, actual_474, t_475, fn__3880;
    t_466 = SqlBuilder();
    t_466:appendSafe('select ');
    t_466:appendInt32(42);
    t_466:appendSafe(', ');
    t_466:appendInt64(temper.int64_constructor(43));
    t_466:appendSafe(', ');
    t_466:appendFloat64(19.99);
    t_466:appendSafe(', ');
    t_466:appendBoolean(true);
    t_466:appendSafe(', ');
    t_466:appendBoolean(false);
    actual_467 = t_466.accumulated:toString();
    t_468 = temper.str_eq(actual_467, 'select 42, 43, 19.99, TRUE, FALSE');
    fn__3881 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, 42, ", ", \\interpolate, 43, ", ", \\interpolate, 19.99, ", ", \\interpolate, true, ", ", \\interpolate, false).toString() == (', 'select 42, 43, 19.99, TRUE, FALSE', ') not (', actual_467, ')');
    end;
    temper.test_assert(test_464, t_468, fn__3881);
    local_469, local_470, local_471 = temper.pcall(function()
      t_465 = temper.date_constructor(2024, 12, 25);
      date__806 = t_465;
    end);
    if local_469 then
    else
      date__806 = temper.bubble();
    end
    t_473 = SqlBuilder();
    t_473:appendSafe('insert into t values (');
    t_473:appendDate(date__806);
    t_473:appendSafe(')');
    actual_474 = t_473.accumulated:toString();
    t_475 = temper.str_eq(actual_474, "insert into t values ('2024-12-25')");
    fn__3880 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "insert into t values (", \\interpolate, date, ")").toString() == (', "insert into t values ('2024-12-25')", ') not (', actual_474, ')');
    end;
    temper.test_assert(test_464, t_475, fn__3880);
    return nil;
  end);
end;
Test_.test_lists__993 = function()
  temper.test('lists', function(test_476)
    local t_477, t_478, t_479, t_480, t_481, actual_482, t_483, fn__3825, t_484, actual_485, t_486, fn__3824, t_487, actual_488, t_489, fn__3823, t_490, actual_491, t_492, fn__3822, t_493, actual_494, t_495, fn__3821, local_496, local_497, local_498, local_500, local_501, local_502, dates__808, t_504, actual_505, t_506, fn__3820;
    t_481 = SqlBuilder();
    t_481:appendSafe('v IN (');
    t_481:appendStringList(temper.listof('a', 'b', "c'd"));
    t_481:appendSafe(')');
    actual_482 = t_481.accumulated:toString();
    t_483 = temper.str_eq(actual_482, "v IN ('a', 'b', 'c''d')");
    fn__3825 = function()
      return temper.concat("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (", "v IN ('a', 'b', 'c''d')", ') not (', actual_482, ')');
    end;
    temper.test_assert(test_476, t_483, fn__3825);
    t_484 = SqlBuilder();
    t_484:appendSafe('v IN (');
    t_484:appendInt32List(temper.listof(1, 2, 3));
    t_484:appendSafe(')');
    actual_485 = t_484.accumulated:toString();
    t_486 = temper.str_eq(actual_485, 'v IN (1, 2, 3)');
    fn__3824 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2, 3), ")").toString() == (', 'v IN (1, 2, 3)', ') not (', actual_485, ')');
    end;
    temper.test_assert(test_476, t_486, fn__3824);
    t_487 = SqlBuilder();
    t_487:appendSafe('v IN (');
    t_487:appendInt64List(temper.listof(temper.int64_constructor(1), temper.int64_constructor(2)));
    t_487:appendSafe(')');
    actual_488 = t_487.accumulated:toString();
    t_489 = temper.str_eq(actual_488, 'v IN (1, 2)');
    fn__3823 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2), ")").toString() == (', 'v IN (1, 2)', ') not (', actual_488, ')');
    end;
    temper.test_assert(test_476, t_489, fn__3823);
    t_490 = SqlBuilder();
    t_490:appendSafe('v IN (');
    t_490:appendFloat64List(temper.listof(1.0, 2.0));
    t_490:appendSafe(')');
    actual_491 = t_490.accumulated:toString();
    t_492 = temper.str_eq(actual_491, 'v IN (1.0, 2.0)');
    fn__3822 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1.0, 2.0), ")").toString() == (', 'v IN (1.0, 2.0)', ') not (', actual_491, ')');
    end;
    temper.test_assert(test_476, t_492, fn__3822);
    t_493 = SqlBuilder();
    t_493:appendSafe('v IN (');
    t_493:appendBooleanList(temper.listof(true, false));
    t_493:appendSafe(')');
    actual_494 = t_493.accumulated:toString();
    t_495 = temper.str_eq(actual_494, 'v IN (TRUE, FALSE)');
    fn__3821 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(true, false), ")").toString() == (', 'v IN (TRUE, FALSE)', ') not (', actual_494, ')');
    end;
    temper.test_assert(test_476, t_495, fn__3821);
    local_496, local_497, local_498 = temper.pcall(function()
      t_477 = temper.date_constructor(2024, 1, 1);
      t_478 = t_477;
    end);
    if local_496 then
    else
      t_478 = temper.bubble();
    end
    local_500, local_501, local_502 = temper.pcall(function()
      t_479 = temper.date_constructor(2024, 12, 25);
      t_480 = t_479;
    end);
    if local_500 then
    else
      t_480 = temper.bubble();
    end
    dates__808 = temper.listof(t_478, t_480);
    t_504 = SqlBuilder();
    t_504:appendSafe('v IN (');
    t_504:appendDateList(dates__808);
    t_504:appendSafe(')');
    actual_505 = t_504.accumulated:toString();
    t_506 = temper.str_eq(actual_505, "v IN ('2024-01-01', '2024-12-25')");
    fn__3820 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, dates, ")").toString() == (', "v IN ('2024-01-01', '2024-12-25')", ') not (', actual_505, ')');
    end;
    temper.test_assert(test_476, t_506, fn__3820);
    return nil;
  end);
end;
Test_.test_nesting__1012 = function()
  temper.test('nesting', function(test_507)
    local name__810, t_508, condition__811, t_509, actual_510, t_511, fn__3788, t_512, actual_513, t_514, fn__3787, parts__812, t_515, actual_516, t_517, fn__3786;
    name__810 = 'Someone';
    t_508 = SqlBuilder();
    t_508:appendSafe('where p.last_name = ');
    t_508:appendString('Someone');
    condition__811 = t_508.accumulated;
    t_509 = SqlBuilder();
    t_509:appendSafe('select p.id from person p ');
    t_509:appendFragment(condition__811);
    actual_510 = t_509.accumulated:toString();
    t_511 = temper.str_eq(actual_510, "select p.id from person p where p.last_name = 'Someone'");
    fn__3788 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_510, ')');
    end;
    temper.test_assert(test_507, t_511, fn__3788);
    t_512 = SqlBuilder();
    t_512:appendSafe('select p.id from person p ');
    t_512:appendPart(condition__811:toSource());
    actual_513 = t_512.accumulated:toString();
    t_514 = temper.str_eq(actual_513, "select p.id from person p where p.last_name = 'Someone'");
    fn__3787 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition.toSource()).toString() == (', "select p.id from person p where p.last_name = 'Someone'", ') not (', actual_513, ')');
    end;
    temper.test_assert(test_507, t_514, fn__3787);
    parts__812 = temper.listof(SqlString("a'b"), SqlInt32(3));
    t_515 = SqlBuilder();
    t_515:appendSafe('select ');
    t_515:appendPartList(parts__812);
    actual_516 = t_515.accumulated:toString();
    t_517 = temper.str_eq(actual_516, "select 'a''b', 3");
    fn__3786 = function()
      return temper.concat('expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, parts).toString() == (', "select 'a''b', 3", ') not (', actual_516, ')');
    end;
    temper.test_assert(test_507, t_517, fn__3786);
    return nil;
  end);
end;
exports = {};
local_519.LuaUnit.run(local_518({'--pattern', '^Test_%.', local_518(arg)}));
return exports;
