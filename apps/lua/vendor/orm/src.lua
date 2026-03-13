local temper = require('temper-core');
local ChangesetError, Changeset, ChangesetImpl__91, OrderClause, Query, SafeIdentifier, ValidatedIdentifier__115, FieldType, StringField, IntField, Int64Field, FloatField, BoolField, DateField, FieldDef, TableDef, SqlBuilder, SqlFragment, SqlPart, SqlSource, SqlBoolean, SqlDate, SqlFloat64, SqlInt32, SqlInt64, SqlString, changeset, isIdentStart__296, isIdentPart__297, safeIdentifier, deleteSql, from, exports;
ChangesetError = temper.type('ChangesetError');
ChangesetError.constructor = function(this__150, field__303, message__304)
  this__150.field__300 = field__303;
  this__150.message__301 = message__304;
  return nil;
end;
ChangesetError.get.field = function(this__844)
  return this__844.field__300;
end;
ChangesetError.get.message = function(this__847)
  return this__847.message__301;
end;
Changeset = temper.type('Changeset');
Changeset.get.tableDef = function(this__78)
  temper.virtual();
end;
Changeset.get.changes = function(this__79)
  temper.virtual();
end;
Changeset.get.errors = function(this__80)
  temper.virtual();
end;
Changeset.get.isValid = function(this__81)
  temper.virtual();
end;
Changeset.methods.cast = function(this__82, allowedFields__314)
  temper.virtual();
end;
Changeset.methods.validateRequired = function(this__83, fields__317)
  temper.virtual();
end;
Changeset.methods.validateLength = function(this__84, field__320, min__321, max__322)
  temper.virtual();
end;
Changeset.methods.validateInt = function(this__85, field__325)
  temper.virtual();
end;
Changeset.methods.validateInt64 = function(this__86, field__328)
  temper.virtual();
end;
Changeset.methods.validateFloat = function(this__87, field__331)
  temper.virtual();
end;
Changeset.methods.validateBool = function(this__88, field__334)
  temper.virtual();
end;
Changeset.methods.toInsertSql = function(this__89)
  temper.virtual();
end;
Changeset.methods.toUpdateSql = function(this__90, id__339)
  temper.virtual();
end;
ChangesetImpl__91 = temper.type('ChangesetImpl__91', Changeset);
ChangesetImpl__91.get.tableDef = function(this__92)
  return this__92._tableDef__341;
end;
ChangesetImpl__91.get.changes = function(this__93)
  return this__93._changes__343;
end;
ChangesetImpl__91.get.errors = function(this__94)
  return this__94._errors__344;
end;
ChangesetImpl__91.get.isValid = function(this__95)
  return this__95._isValid__345;
end;
ChangesetImpl__91.methods.cast = function(this__96, allowedFields__355)
  local mb__357, fn__4731;
  mb__357 = temper.mapbuilder_constructor();
  fn__4731 = function(f__358)
    local t_0, t_1, val__359;
    t_1 = f__358.sqlValue;
    val__359 = temper.mapped_getor(this__96._params__342, t_1, '');
    if not temper.string_isempty(val__359) then
      t_0 = f__358.sqlValue;
      temper.mapbuilder_set(mb__357, t_0, val__359);
    end
    return nil;
  end;
  temper.list_foreach(allowedFields__355, fn__4731);
  return ChangesetImpl__91(this__96._tableDef__341, this__96._params__342, temper.mapped_tomap(mb__357), this__96._errors__344, this__96._isValid__345);
end;
ChangesetImpl__91.methods.validateRequired = function(this__97, fields__361)
  local return__183, t_2, t_3, t_4, t_5, eb__363, valid__364, fn__4720;
  ::continue_1::
  if not this__97._isValid__345 then
    return__183 = this__97;
    goto break_0;
  end
  eb__363 = temper.list_tolistbuilder(this__97._errors__344);
  valid__364 = true;
  fn__4720 = function(f__365)
    local t_6, t_7;
    t_7 = f__365.sqlValue;
    if not temper.mapped_has(this__97._changes__343, t_7) then
      t_6 = ChangesetError(f__365.sqlValue, 'is required');
      temper.listbuilder_add(eb__363, t_6);
      valid__364 = false;
    end
    return nil;
  end;
  temper.list_foreach(fields__361, fn__4720);
  t_3 = this__97._tableDef__341;
  t_4 = this__97._params__342;
  t_5 = this__97._changes__343;
  t_2 = temper.listbuilder_tolist(eb__363);
  return__183 = ChangesetImpl__91(t_3, t_4, t_5, t_2, valid__364);
  ::break_0::return return__183;
end;
ChangesetImpl__91.methods.validateLength = function(this__98, field__367, min__368, max__369)
  local return__184, t_8, t_9, t_10, t_11, t_12, t_13, val__371, len__372;
  ::continue_3::
  if not this__98._isValid__345 then
    return__184 = this__98;
    goto break_2;
  end
  t_8 = field__367.sqlValue;
  val__371 = temper.mapped_getor(this__98._changes__343, t_8, '');
  len__372 = temper.string_countbetween(val__371, 1.0, temper.string_end(val__371));
  if (len__372 < min__368) then
    t_10 = true;
  else
    t_10 = (len__372 > max__369);
  end
  if t_10 then
    local msg__373, eb__374;
    msg__373 = temper.concat('must be between ', temper.int32_tostring(min__368), ' and ', temper.int32_tostring(max__369), ' characters');
    eb__374 = temper.list_tolistbuilder(this__98._errors__344);
    temper.listbuilder_add(eb__374, ChangesetError(field__367.sqlValue, msg__373));
    t_11 = this__98._tableDef__341;
    t_12 = this__98._params__342;
    t_13 = this__98._changes__343;
    t_9 = temper.listbuilder_tolist(eb__374);
    return__184 = ChangesetImpl__91(t_11, t_12, t_13, t_9, false);
    goto break_2;
  end
  return__184 = this__98;
  ::break_2::return return__184;
end;
ChangesetImpl__91.methods.validateInt = function(this__99, field__376)
  local return__185, t_14, t_15, t_16, t_17, t_18, val__378, parseOk__379, local_19, local_20, local_21;
  ::continue_5::
  if not this__99._isValid__345 then
    return__185 = this__99;
    goto break_4;
  end
  t_14 = field__376.sqlValue;
  val__378 = temper.mapped_getor(this__99._changes__343, t_14, '');
  if temper.string_isempty(val__378) then
    return__185 = this__99;
    goto break_4;
  end
  local_19, local_20, local_21 = temper.pcall(function()
    temper.string_toint32(val__378);
    parseOk__379 = true;
  end);
  if local_19 then
  else
    parseOk__379 = false;
  end
  if not parseOk__379 then
    local eb__380;
    eb__380 = temper.list_tolistbuilder(this__99._errors__344);
    temper.listbuilder_add(eb__380, ChangesetError(field__376.sqlValue, 'must be an integer'));
    t_16 = this__99._tableDef__341;
    t_17 = this__99._params__342;
    t_18 = this__99._changes__343;
    t_15 = temper.listbuilder_tolist(eb__380);
    return__185 = ChangesetImpl__91(t_16, t_17, t_18, t_15, false);
    goto break_4;
  end
  return__185 = this__99;
  ::break_4::return return__185;
end;
ChangesetImpl__91.methods.validateInt64 = function(this__100, field__382)
  local return__186, t_23, t_24, t_25, t_26, t_27, val__384, parseOk__385, local_28, local_29, local_30;
  ::continue_7::
  if not this__100._isValid__345 then
    return__186 = this__100;
    goto break_6;
  end
  t_23 = field__382.sqlValue;
  val__384 = temper.mapped_getor(this__100._changes__343, t_23, '');
  if temper.string_isempty(val__384) then
    return__186 = this__100;
    goto break_6;
  end
  local_28, local_29, local_30 = temper.pcall(function()
    temper.string_toint64(val__384);
    parseOk__385 = true;
  end);
  if local_28 then
  else
    parseOk__385 = false;
  end
  if not parseOk__385 then
    local eb__386;
    eb__386 = temper.list_tolistbuilder(this__100._errors__344);
    temper.listbuilder_add(eb__386, ChangesetError(field__382.sqlValue, 'must be a 64-bit integer'));
    t_25 = this__100._tableDef__341;
    t_26 = this__100._params__342;
    t_27 = this__100._changes__343;
    t_24 = temper.listbuilder_tolist(eb__386);
    return__186 = ChangesetImpl__91(t_25, t_26, t_27, t_24, false);
    goto break_6;
  end
  return__186 = this__100;
  ::break_6::return return__186;
end;
ChangesetImpl__91.methods.validateFloat = function(this__101, field__388)
  local return__187, t_32, t_33, t_34, t_35, t_36, val__390, parseOk__391, local_37, local_38, local_39;
  ::continue_9::
  if not this__101._isValid__345 then
    return__187 = this__101;
    goto break_8;
  end
  t_32 = field__388.sqlValue;
  val__390 = temper.mapped_getor(this__101._changes__343, t_32, '');
  if temper.string_isempty(val__390) then
    return__187 = this__101;
    goto break_8;
  end
  local_37, local_38, local_39 = temper.pcall(function()
    temper.string_tofloat64(val__390);
    parseOk__391 = true;
  end);
  if local_37 then
  else
    parseOk__391 = false;
  end
  if not parseOk__391 then
    local eb__392;
    eb__392 = temper.list_tolistbuilder(this__101._errors__344);
    temper.listbuilder_add(eb__392, ChangesetError(field__388.sqlValue, 'must be a number'));
    t_34 = this__101._tableDef__341;
    t_35 = this__101._params__342;
    t_36 = this__101._changes__343;
    t_33 = temper.listbuilder_tolist(eb__392);
    return__187 = ChangesetImpl__91(t_34, t_35, t_36, t_33, false);
    goto break_8;
  end
  return__187 = this__101;
  ::break_8::return return__187;
end;
ChangesetImpl__91.methods.validateBool = function(this__102, field__394)
  local return__188, t_41, t_42, t_43, t_44, t_45, t_46, t_47, t_48, t_49, t_50, val__396, isTrue__397, isFalse__398;
  ::continue_11::
  if not this__102._isValid__345 then
    return__188 = this__102;
    goto break_10;
  end
  t_41 = field__394.sqlValue;
  val__396 = temper.mapped_getor(this__102._changes__343, t_41, '');
  if temper.string_isempty(val__396) then
    return__188 = this__102;
    goto break_10;
  end
  if temper.str_eq(val__396, 'true') then
    isTrue__397 = true;
  else
    if temper.str_eq(val__396, '1') then
      t_44 = true;
    else
      if temper.str_eq(val__396, 'yes') then
        t_43 = true;
      else
        t_43 = temper.str_eq(val__396, 'on');
      end
      t_44 = t_43;
    end
    isTrue__397 = t_44;
  end
  if temper.str_eq(val__396, 'false') then
    isFalse__398 = true;
  else
    if temper.str_eq(val__396, '0') then
      t_46 = true;
    else
      if temper.str_eq(val__396, 'no') then
        t_45 = true;
      else
        t_45 = temper.str_eq(val__396, 'off');
      end
      t_46 = t_45;
    end
    isFalse__398 = t_46;
  end
  if not isTrue__397 then
    t_47 = not isFalse__398;
  else
    t_47 = false;
  end
  if t_47 then
    local eb__399;
    eb__399 = temper.list_tolistbuilder(this__102._errors__344);
    temper.listbuilder_add(eb__399, ChangesetError(field__394.sqlValue, 'must be a boolean (true/false/1/0/yes/no/on/off)'));
    t_48 = this__102._tableDef__341;
    t_49 = this__102._params__342;
    t_50 = this__102._changes__343;
    t_42 = temper.listbuilder_tolist(eb__399);
    return__188 = ChangesetImpl__91(t_48, t_49, t_50, t_42, false);
    goto break_10;
  end
  return__188 = this__102;
  ::break_10::return return__188;
end;
ChangesetImpl__91.methods.parseBoolSqlPart = function(this__103, val__401)
  local return__189, t_51, t_52, t_53, t_54, t_55, t_56;
  ::continue_13::
  if temper.str_eq(val__401, 'true') then
    t_53 = true;
  else
    if temper.str_eq(val__401, '1') then
      t_52 = true;
    else
      if temper.str_eq(val__401, 'yes') then
        t_51 = true;
      else
        t_51 = temper.str_eq(val__401, 'on');
      end
      t_52 = t_51;
    end
    t_53 = t_52;
  end
  if t_53 then
    return__189 = SqlBoolean(true);
    goto break_12;
  end
  if temper.str_eq(val__401, 'false') then
    t_56 = true;
  else
    if temper.str_eq(val__401, '0') then
      t_55 = true;
    else
      if temper.str_eq(val__401, 'no') then
        t_54 = true;
      else
        t_54 = temper.str_eq(val__401, 'off');
      end
      t_55 = t_54;
    end
    t_56 = t_55;
  end
  if t_56 then
    return__189 = SqlBoolean(false);
    goto break_12;
  end
  temper.bubble();
  ::break_12::return return__189;
end;
ChangesetImpl__91.methods.valueToSqlPart = function(this__104, fieldDef__404, val__405)
  local return__190, t_57, t_58, t_59, t_60, ft__407;
  ::continue_15::ft__407 = fieldDef__404.fieldType;
  if temper.instance_of(ft__407, StringField) then
    return__190 = SqlString(val__405);
    goto break_14;
  end
  if temper.instance_of(ft__407, IntField) then
    t_57 = temper.string_toint32(val__405);
    return__190 = SqlInt32(t_57);
    goto break_14;
  end
  if temper.instance_of(ft__407, Int64Field) then
    t_58 = temper.string_toint64(val__405);
    return__190 = SqlInt64(t_58);
    goto break_14;
  end
  if temper.instance_of(ft__407, FloatField) then
    t_59 = temper.string_tofloat64(val__405);
    return__190 = SqlFloat64(t_59);
    goto break_14;
  end
  if temper.instance_of(ft__407, BoolField) then
    return__190 = this__104:parseBoolSqlPart(val__405);
    goto break_14;
  end
  if temper.instance_of(ft__407, DateField) then
    t_60 = temper.date_fromisostring(val__405);
    return__190 = SqlDate(t_60);
    goto break_14;
  end
  temper.bubble();
  ::break_14::return return__190;
end;
ChangesetImpl__91.methods.toInsertSql = function(this__105)
  local t_61, t_62, t_63, t_64, t_65, t_66, t_67, t_68, t_69, t_70, i__410, pairs__412, colNames__413, valParts__414, i__415, b__418, t_71, fn__4613, j__420;
  if not this__105._isValid__345 then
    temper.bubble();
  end
  i__410 = 0;
  while true do
    local f__411;
    t_61 = temper.list_length(this__105._tableDef__341.fields);
    if not (i__410 < t_61) then
      break;
    end
    f__411 = temper.list_get(this__105._tableDef__341.fields, i__410);
    if not f__411.nullable then
      t_62 = f__411.name.sqlValue;
      t_63 = temper.mapped_has(this__105._changes__343, t_62);
      t_68 = not t_63;
    else
      t_68 = false;
    end
    if t_68 then
      temper.bubble();
    end
    i__410 = temper.int32_add(i__410, 1);
  end
  pairs__412 = temper.mapped_tolist(this__105._changes__343);
  if (temper.list_length(pairs__412) == 0) then
    temper.bubble();
  end
  colNames__413 = temper.listbuilder_constructor();
  valParts__414 = temper.listbuilder_constructor();
  i__415 = 0;
  while true do
    local pair__416, fd__417;
    t_64 = temper.list_length(pairs__412);
    if not (i__415 < t_64) then
      break;
    end
    pair__416 = temper.list_get(pairs__412, i__415);
    t_65 = pair__416.key;
    t_69 = this__105._tableDef__341:field(t_65);
    fd__417 = t_69;
    temper.listbuilder_add(colNames__413, pair__416.key);
    t_66 = pair__416.value;
    t_70 = this__105:valueToSqlPart(fd__417, t_66);
    temper.listbuilder_add(valParts__414, t_70);
    i__415 = temper.int32_add(i__415, 1);
  end
  b__418 = SqlBuilder();
  b__418:appendSafe('INSERT INTO ');
  b__418:appendSafe(this__105._tableDef__341.tableName.sqlValue);
  b__418:appendSafe(' (');
  t_71 = temper.listbuilder_tolist(colNames__413);
  fn__4613 = function(c__419)
    return c__419;
  end;
  b__418:appendSafe(temper.listed_join(t_71, ', ', fn__4613));
  b__418:appendSafe(') VALUES (');
  b__418:appendPart(temper.listed_get(valParts__414, 0));
  j__420 = 1;
  while true do
    t_67 = temper.listbuilder_length(valParts__414);
    if not (j__420 < t_67) then
      break;
    end
    b__418:appendSafe(', ');
    b__418:appendPart(temper.listed_get(valParts__414, j__420));
    j__420 = temper.int32_add(j__420, 1);
  end
  b__418:appendSafe(')');
  return b__418.accumulated;
end;
ChangesetImpl__91.methods.toUpdateSql = function(this__106, id__422)
  local t_72, t_73, t_74, t_75, t_76, pairs__424, b__425, i__426;
  if not this__106._isValid__345 then
    temper.bubble();
  end
  pairs__424 = temper.mapped_tolist(this__106._changes__343);
  if (temper.list_length(pairs__424) == 0) then
    temper.bubble();
  end
  b__425 = SqlBuilder();
  b__425:appendSafe('UPDATE ');
  b__425:appendSafe(this__106._tableDef__341.tableName.sqlValue);
  b__425:appendSafe(' SET ');
  i__426 = 0;
  while true do
    local pair__427, fd__428;
    t_72 = temper.list_length(pairs__424);
    if not (i__426 < t_72) then
      break;
    end
    if (i__426 > 0) then
      b__425:appendSafe(', ');
    end
    pair__427 = temper.list_get(pairs__424, i__426);
    t_73 = pair__427.key;
    t_75 = this__106._tableDef__341:field(t_73);
    fd__428 = t_75;
    b__425:appendSafe(pair__427.key);
    b__425:appendSafe(' = ');
    t_74 = pair__427.value;
    t_76 = this__106:valueToSqlPart(fd__428, t_74);
    b__425:appendPart(t_76);
    i__426 = temper.int32_add(i__426, 1);
  end
  b__425:appendSafe(' WHERE id = ');
  b__425:appendInt32(id__422);
  return b__425.accumulated;
end;
ChangesetImpl__91.constructor = function(this__173, _tableDef__430, _params__431, _changes__432, _errors__433, _isValid__434)
  this__173._tableDef__341 = _tableDef__430;
  this__173._params__342 = _params__431;
  this__173._changes__343 = _changes__432;
  this__173._errors__344 = _errors__433;
  this__173._isValid__345 = _isValid__434;
  return nil;
end;
OrderClause = temper.type('OrderClause');
OrderClause.constructor = function(this__197, field__532, ascending__533)
  this__197.field__529 = field__532;
  this__197.ascending__530 = ascending__533;
  return nil;
end;
OrderClause.get.field = function(this__912)
  return this__912.field__529;
end;
OrderClause.get.ascending = function(this__915)
  return this__915.ascending__530;
end;
Query = temper.type('Query');
Query.methods.where = function(this__107, condition__541)
  local nb__543;
  nb__543 = temper.list_tolistbuilder(this__107.conditions__535);
  temper.listbuilder_add(nb__543, condition__541);
  return Query(this__107.tableName__534, temper.listbuilder_tolist(nb__543), this__107.selectedFields__536, this__107.orderClauses__537, this__107.limitVal__538, this__107.offsetVal__539);
end;
Query.methods.select = function(this__108, fields__545)
  return Query(this__108.tableName__534, this__108.conditions__535, fields__545, this__108.orderClauses__537, this__108.limitVal__538, this__108.offsetVal__539);
end;
Query.methods.orderBy = function(this__109, field__548, ascending__549)
  local nb__551;
  nb__551 = temper.list_tolistbuilder(this__109.orderClauses__537);
  temper.listbuilder_add(nb__551, OrderClause(field__548, ascending__549));
  return Query(this__109.tableName__534, this__109.conditions__535, this__109.selectedFields__536, temper.listbuilder_tolist(nb__551), this__109.limitVal__538, this__109.offsetVal__539);
end;
Query.methods.limit = function(this__110, n__553)
  if (n__553 < 0) then
    temper.bubble();
  end
  return Query(this__110.tableName__534, this__110.conditions__535, this__110.selectedFields__536, this__110.orderClauses__537, n__553, this__110.offsetVal__539);
end;
Query.methods.offset = function(this__111, n__556)
  if (n__556 < 0) then
    temper.bubble();
  end
  return Query(this__111.tableName__534, this__111.conditions__535, this__111.selectedFields__536, this__111.orderClauses__537, this__111.limitVal__538, n__556);
end;
Query.methods.toSql = function(this__112)
  local t_77, b__560, lv__565, ov__566;
  b__560 = SqlBuilder();
  b__560:appendSafe('SELECT ');
  if temper.listed_isempty(this__112.selectedFields__536) then
    b__560:appendSafe('*');
  else
    local fn__4170;
    fn__4170 = function(f__561)
      return f__561.sqlValue;
    end;
    b__560:appendSafe(temper.listed_join(this__112.selectedFields__536, ', ', fn__4170));
  end
  b__560:appendSafe(' FROM ');
  b__560:appendSafe(this__112.tableName__534.sqlValue);
  if not temper.listed_isempty(this__112.conditions__535) then
    local i__562;
    b__560:appendSafe(' WHERE ');
    b__560:appendFragment(temper.list_get(this__112.conditions__535, 0));
    i__562 = 1;
    while true do
      t_77 = temper.list_length(this__112.conditions__535);
      if not (i__562 < t_77) then
        break;
      end
      b__560:appendSafe(' AND ');
      b__560:appendFragment(temper.list_get(this__112.conditions__535, i__562));
      i__562 = temper.int32_add(i__562, 1);
    end
  end
  if not temper.listed_isempty(this__112.orderClauses__537) then
    local first__563, fn__4169;
    b__560:appendSafe(' ORDER BY ');
    first__563 = true;
    fn__4169 = function(oc__564)
      local t_78, t_79;
      if not first__563 then
        b__560:appendSafe(', ');
      end
      first__563 = false;
      t_79 = oc__564.field.sqlValue;
      b__560:appendSafe(t_79);
      if oc__564.ascending then
        t_78 = ' ASC';
      else
        t_78 = ' DESC';
      end
      b__560:appendSafe(t_78);
      return nil;
    end;
    temper.list_foreach(this__112.orderClauses__537, fn__4169);
  end
  lv__565 = this__112.limitVal__538;
  if not temper.is_null(lv__565) then
    local lv_80;
    lv_80 = lv__565;
    b__560:appendSafe(' LIMIT ');
    b__560:appendInt32(lv_80);
  end
  ov__566 = this__112.offsetVal__539;
  if not temper.is_null(ov__566) then
    local ov_81;
    ov_81 = ov__566;
    b__560:appendSafe(' OFFSET ');
    b__560:appendInt32(ov_81);
  end
  return b__560.accumulated;
end;
Query.methods.safeToSql = function(this__113, defaultLimit__568)
  local return__212, t_82;
  if (defaultLimit__568 < 0) then
    temper.bubble();
  end
  if not temper.is_null(this__113.limitVal__538) then
    return__212 = this__113:toSql();
  else
    t_82 = this__113:limit(defaultLimit__568);
    return__212 = t_82:toSql();
  end
  return return__212;
end;
Query.constructor = function(this__199, tableName__571, conditions__572, selectedFields__573, orderClauses__574, limitVal__575, offsetVal__576)
  if (limitVal__575 == nil) then
    limitVal__575 = temper.null;
  end
  if (offsetVal__576 == nil) then
    offsetVal__576 = temper.null;
  end
  this__199.tableName__534 = tableName__571;
  this__199.conditions__535 = conditions__572;
  this__199.selectedFields__536 = selectedFields__573;
  this__199.orderClauses__537 = orderClauses__574;
  this__199.limitVal__538 = limitVal__575;
  this__199.offsetVal__539 = offsetVal__576;
  return nil;
end;
Query.get.tableName = function(this__918)
  return this__918.tableName__534;
end;
Query.get.conditions = function(this__921)
  return this__921.conditions__535;
end;
Query.get.selectedFields = function(this__924)
  return this__924.selectedFields__536;
end;
Query.get.orderClauses = function(this__927)
  return this__927.orderClauses__537;
end;
Query.get.limitVal = function(this__930)
  return this__930.limitVal__538;
end;
Query.get.offsetVal = function(this__933)
  return this__933.offsetVal__539;
end;
SafeIdentifier = temper.type('SafeIdentifier');
SafeIdentifier.get.sqlValue = function(this__114)
  temper.virtual();
end;
ValidatedIdentifier__115 = temper.type('ValidatedIdentifier__115', SafeIdentifier);
ValidatedIdentifier__115.get.sqlValue = function(this__116)
  return this__116._value__621;
end;
ValidatedIdentifier__115.constructor = function(this__218, _value__625)
  this__218._value__621 = _value__625;
  return nil;
end;
FieldType = temper.type('FieldType');
StringField = temper.type('StringField', FieldType);
StringField.constructor = function(this__224)
  return nil;
end;
IntField = temper.type('IntField', FieldType);
IntField.constructor = function(this__226)
  return nil;
end;
Int64Field = temper.type('Int64Field', FieldType);
Int64Field.constructor = function(this__228)
  return nil;
end;
FloatField = temper.type('FloatField', FieldType);
FloatField.constructor = function(this__230)
  return nil;
end;
BoolField = temper.type('BoolField', FieldType);
BoolField.constructor = function(this__232)
  return nil;
end;
DateField = temper.type('DateField', FieldType);
DateField.constructor = function(this__234)
  return nil;
end;
FieldDef = temper.type('FieldDef');
FieldDef.constructor = function(this__236, name__643, fieldType__644, nullable__645)
  this__236.name__639 = name__643;
  this__236.fieldType__640 = fieldType__644;
  this__236.nullable__641 = nullable__645;
  return nil;
end;
FieldDef.get.name = function(this__850)
  return this__850.name__639;
end;
FieldDef.get.fieldType = function(this__853)
  return this__853.fieldType__640;
end;
FieldDef.get.nullable = function(this__856)
  return this__856.nullable__641;
end;
TableDef = temper.type('TableDef');
TableDef.methods.field = function(this__117, name__649)
  local return__241, this__2984, n__2985, i__2986;
  ::continue_17::this__2984 = this__117.fields__647;
  n__2985 = temper.list_length(this__2984);
  i__2986 = 0;
  while (i__2986 < n__2985) do
    local el__2987, f__651;
    el__2987 = temper.list_get(this__2984, i__2986);
    i__2986 = temper.int32_add(i__2986, 1);
    f__651 = el__2987;
    if temper.str_eq(f__651.name.sqlValue, name__649) then
      return__241 = f__651;
      goto break_16;
    end
  end
  temper.bubble();
  ::break_16::return return__241;
end;
TableDef.constructor = function(this__238, tableName__653, fields__654)
  this__238.tableName__646 = tableName__653;
  this__238.fields__647 = fields__654;
  return nil;
end;
TableDef.get.tableName = function(this__859)
  return this__859.tableName__646;
end;
TableDef.get.fields = function(this__862)
  return this__862.fields__647;
end;
SqlBuilder = temper.type('SqlBuilder');
SqlBuilder.methods.appendSafe = function(this__118, sqlSource__676)
  local t_83;
  t_83 = SqlSource(sqlSource__676);
  temper.listbuilder_add(this__118.buffer__674, t_83);
  return nil;
end;
SqlBuilder.methods.appendFragment = function(this__119, fragment__679)
  local t_84;
  t_84 = fragment__679.parts;
  temper.listbuilder_addall(this__119.buffer__674, t_84);
  return nil;
end;
SqlBuilder.methods.appendPart = function(this__120, part__682)
  temper.listbuilder_add(this__120.buffer__674, part__682);
  return nil;
end;
SqlBuilder.methods.appendPartList = function(this__121, values__685)
  local fn__4783;
  fn__4783 = function(x__687)
    this__121:appendPart(x__687);
    return nil;
  end;
  this__121:appendList(values__685, fn__4783);
  return nil;
end;
SqlBuilder.methods.appendBoolean = function(this__122, value__689)
  local t_85;
  t_85 = SqlBoolean(value__689);
  temper.listbuilder_add(this__122.buffer__674, t_85);
  return nil;
end;
SqlBuilder.methods.appendBooleanList = function(this__123, values__692)
  local fn__4777;
  fn__4777 = function(x__694)
    this__123:appendBoolean(x__694);
    return nil;
  end;
  this__123:appendList(values__692, fn__4777);
  return nil;
end;
SqlBuilder.methods.appendDate = function(this__124, value__696)
  local t_86;
  t_86 = SqlDate(value__696);
  temper.listbuilder_add(this__124.buffer__674, t_86);
  return nil;
end;
SqlBuilder.methods.appendDateList = function(this__125, values__699)
  local fn__4771;
  fn__4771 = function(x__701)
    this__125:appendDate(x__701);
    return nil;
  end;
  this__125:appendList(values__699, fn__4771);
  return nil;
end;
SqlBuilder.methods.appendFloat64 = function(this__126, value__703)
  local t_87;
  t_87 = SqlFloat64(value__703);
  temper.listbuilder_add(this__126.buffer__674, t_87);
  return nil;
end;
SqlBuilder.methods.appendFloat64List = function(this__127, values__706)
  local fn__4765;
  fn__4765 = function(x__708)
    this__127:appendFloat64(x__708);
    return nil;
  end;
  this__127:appendList(values__706, fn__4765);
  return nil;
end;
SqlBuilder.methods.appendInt32 = function(this__128, value__710)
  local t_88;
  t_88 = SqlInt32(value__710);
  temper.listbuilder_add(this__128.buffer__674, t_88);
  return nil;
end;
SqlBuilder.methods.appendInt32List = function(this__129, values__713)
  local fn__4759;
  fn__4759 = function(x__715)
    this__129:appendInt32(x__715);
    return nil;
  end;
  this__129:appendList(values__713, fn__4759);
  return nil;
end;
SqlBuilder.methods.appendInt64 = function(this__130, value__717)
  local t_89;
  t_89 = SqlInt64(value__717);
  temper.listbuilder_add(this__130.buffer__674, t_89);
  return nil;
end;
SqlBuilder.methods.appendInt64List = function(this__131, values__720)
  local fn__4753;
  fn__4753 = function(x__722)
    this__131:appendInt64(x__722);
    return nil;
  end;
  this__131:appendList(values__720, fn__4753);
  return nil;
end;
SqlBuilder.methods.appendString = function(this__132, value__724)
  local t_90;
  t_90 = SqlString(value__724);
  temper.listbuilder_add(this__132.buffer__674, t_90);
  return nil;
end;
SqlBuilder.methods.appendStringList = function(this__133, values__727)
  local fn__4747;
  fn__4747 = function(x__729)
    this__133:appendString(x__729);
    return nil;
  end;
  this__133:appendList(values__727, fn__4747);
  return nil;
end;
SqlBuilder.methods.appendList = function(this__134, values__731, appendValue__732)
  local t_91, t_92, i__734;
  i__734 = 0;
  while true do
    t_91 = temper.listed_length(values__731);
    if not (i__734 < t_91) then
      break;
    end
    if (i__734 > 0) then
      this__134:appendSafe(', ');
    end
    t_92 = temper.listed_get(values__731, i__734);
    appendValue__732(t_92);
    i__734 = temper.int32_add(i__734, 1);
  end
  return nil;
end;
SqlBuilder.get.accumulated = function(this__135)
  return SqlFragment(temper.listbuilder_tolist(this__135.buffer__674));
end;
SqlBuilder.constructor = function(this__243)
  local t_93;
  t_93 = temper.listbuilder_constructor();
  this__243.buffer__674 = t_93;
  return nil;
end;
SqlFragment = temper.type('SqlFragment');
SqlFragment.methods.toSource = function(this__140)
  return SqlSource(this__140:toString());
end;
SqlFragment.methods.toString = function(this__141)
  local t_94, builder__746, i__747;
  builder__746 = temper.stringbuilder_constructor();
  i__747 = 0;
  while true do
    t_94 = temper.list_length(this__141.parts__741);
    if not (i__747 < t_94) then
      break;
    end
    temper.list_get(this__141.parts__741, i__747):formatTo(builder__746);
    i__747 = temper.int32_add(i__747, 1);
  end
  return temper.stringbuilder_tostring(builder__746);
end;
SqlFragment.constructor = function(this__264, parts__749)
  this__264.parts__741 = parts__749;
  return nil;
end;
SqlFragment.get.parts = function(this__868)
  return this__868.parts__741;
end;
SqlPart = temper.type('SqlPart');
SqlPart.methods.formatTo = function(this__142, builder__751)
  temper.virtual();
end;
SqlSource = temper.type('SqlSource', SqlPart);
SqlSource.methods.formatTo = function(this__143, builder__755)
  temper.stringbuilder_append(builder__755, this__143.source__753);
  return nil;
end;
SqlSource.constructor = function(this__270, source__758)
  this__270.source__753 = source__758;
  return nil;
end;
SqlSource.get.source = function(this__865)
  return this__865.source__753;
end;
SqlBoolean = temper.type('SqlBoolean', SqlPart);
SqlBoolean.methods.formatTo = function(this__144, builder__761)
  local t_95;
  if this__144.value__759 then
    t_95 = 'TRUE';
  else
    t_95 = 'FALSE';
  end
  temper.stringbuilder_append(builder__761, t_95);
  return nil;
end;
SqlBoolean.constructor = function(this__273, value__764)
  this__273.value__759 = value__764;
  return nil;
end;
SqlBoolean.get.value = function(this__871)
  return this__871.value__759;
end;
SqlDate = temper.type('SqlDate', SqlPart);
SqlDate.methods.formatTo = function(this__145, builder__767)
  local t_96;
  temper.stringbuilder_append(builder__767, "'");
  t_96 = temper.date_tostring(this__145.value__765);
  temper.stringbuilder_append(builder__767, t_96);
  temper.stringbuilder_append(builder__767, "'");
  return nil;
end;
SqlDate.constructor = function(this__276, value__770)
  this__276.value__765 = value__770;
  return nil;
end;
SqlDate.get.value = function(this__886)
  return this__886.value__765;
end;
SqlFloat64 = temper.type('SqlFloat64', SqlPart);
SqlFloat64.methods.formatTo = function(this__146, builder__773)
  local t_97;
  t_97 = temper.float64_tostring(this__146.value__771);
  temper.stringbuilder_append(builder__773, t_97);
  return nil;
end;
SqlFloat64.constructor = function(this__279, value__776)
  this__279.value__771 = value__776;
  return nil;
end;
SqlFloat64.get.value = function(this__883)
  return this__883.value__771;
end;
SqlInt32 = temper.type('SqlInt32', SqlPart);
SqlInt32.methods.formatTo = function(this__147, builder__779)
  local t_98;
  t_98 = temper.int32_tostring(this__147.value__777);
  temper.stringbuilder_append(builder__779, t_98);
  return nil;
end;
SqlInt32.constructor = function(this__282, value__782)
  this__282.value__777 = value__782;
  return nil;
end;
SqlInt32.get.value = function(this__877)
  return this__877.value__777;
end;
SqlInt64 = temper.type('SqlInt64', SqlPart);
SqlInt64.methods.formatTo = function(this__148, builder__785)
  local t_99;
  t_99 = temper.int64_tostring(this__148.value__783);
  temper.stringbuilder_append(builder__785, t_99);
  return nil;
end;
SqlInt64.constructor = function(this__285, value__788)
  this__285.value__783 = value__788;
  return nil;
end;
SqlInt64.get.value = function(this__880)
  return this__880.value__783;
end;
SqlString = temper.type('SqlString', SqlPart);
SqlString.methods.formatTo = function(this__149, builder__791)
  local fn__4802;
  temper.stringbuilder_append(builder__791, "'");
  fn__4802 = function(c__793)
    if (c__793 == 39) then
      temper.stringbuilder_append(builder__791, "''");
    else
      local local_100, local_101, local_102;
      local_100, local_101, local_102 = temper.pcall(function()
        temper.stringbuilder_appendcodepoint(builder__791, c__793);
      end);
      if local_100 then
      else
        temper.bubble();
      end
    end
    return nil;
  end;
  temper.string_foreach(this__149.value__789, fn__4802);
  temper.stringbuilder_append(builder__791, "'");
  return nil;
end;
SqlString.constructor = function(this__288, value__795)
  this__288.value__789 = value__795;
  return nil;
end;
SqlString.get.value = function(this__874)
  return this__874.value__789;
end;
changeset = function(tableDef__435, params__436)
  local t_104;
  t_104 = temper.map_constructor(temper.listof());
  return ChangesetImpl__91(tableDef__435, params__436, t_104, temper.listof(), true);
end;
isIdentStart__296 = function(c__626)
  local return__221, t_105, t_106;
  if (c__626 >= 97) then
    t_105 = (c__626 <= 122);
  else
    t_105 = false;
  end
  if t_105 then
    return__221 = true;
  else
    if (c__626 >= 65) then
      t_106 = (c__626 <= 90);
    else
      t_106 = false;
    end
    if t_106 then
      return__221 = true;
    else
      return__221 = (c__626 == 95);
    end
  end
  return return__221;
end;
isIdentPart__297 = function(c__628)
  local return__222;
  if isIdentStart__296(c__628) then
    return__222 = true;
  elseif (c__628 >= 48) then
    return__222 = (c__628 <= 57);
  else
    return__222 = false;
  end
  return return__222;
end;
safeIdentifier = function(name__630)
  local t_107, idx__632, t_108;
  if temper.string_isempty(name__630) then
    temper.bubble();
  end
  idx__632 = 1.0;
  if not isIdentStart__296(temper.string_get(name__630, idx__632)) then
    temper.bubble();
  end
  t_108 = temper.string_next(name__630, idx__632);
  idx__632 = t_108;
  while true do
    if not temper.string_hasindex(name__630, idx__632) then
      break;
    end
    if not isIdentPart__297(temper.string_get(name__630, idx__632)) then
      temper.bubble();
    end
    t_107 = temper.string_next(name__630, idx__632);
    idx__632 = t_107;
  end
  return ValidatedIdentifier__115(name__630);
end;
deleteSql = function(tableDef__525, id__526)
  local b__528;
  b__528 = SqlBuilder();
  b__528:appendSafe('DELETE FROM ');
  b__528:appendSafe(tableDef__525.tableName.sqlValue);
  b__528:appendSafe(' WHERE id = ');
  b__528:appendInt32(id__526);
  return b__528.accumulated;
end;
from = function(tableName__577)
  return Query(tableName__577, temper.listof(), temper.listof(), temper.listof(), temper.null, temper.null);
end;
exports = {};
exports.ChangesetError = ChangesetError;
exports.Changeset = Changeset;
exports.OrderClause = OrderClause;
exports.Query = Query;
exports.SafeIdentifier = SafeIdentifier;
exports.FieldType = FieldType;
exports.StringField = StringField;
exports.IntField = IntField;
exports.Int64Field = Int64Field;
exports.FloatField = FloatField;
exports.BoolField = BoolField;
exports.DateField = DateField;
exports.FieldDef = FieldDef;
exports.TableDef = TableDef;
exports.SqlBuilder = SqlBuilder;
exports.SqlFragment = SqlFragment;
exports.SqlPart = SqlPart;
exports.SqlSource = SqlSource;
exports.SqlBoolean = SqlBoolean;
exports.SqlDate = SqlDate;
exports.SqlFloat64 = SqlFloat64;
exports.SqlInt32 = SqlInt32;
exports.SqlInt64 = SqlInt64;
exports.SqlString = SqlString;
exports.changeset = changeset;
exports.safeIdentifier = safeIdentifier;
exports.deleteSql = deleteSql;
exports.from = from;
return exports;
