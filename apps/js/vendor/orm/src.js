import {
  type as type__6, mapBuilderConstructor as mapBuilderConstructor_43, mappedGetOr as mappedGetOr_49, mapBuilderSet as mapBuilderSet_51, mappedToMap as mappedToMap_52, listBuilderAdd as listBuilderAdd_68, listBuilderToList as listBuilderToList_69, stringCountBetween as stringCountBetween_84, stringToInt32 as stringToInt32_98, stringToInt64 as stringToInt64_111, stringToFloat64 as stringToFloat64_124, listedGet as listedGet_179, mappedToList as mappedToList_181, listedJoin as listedJoin_191, listBuilderAddAll as listBuilderAddAll_302, float64ToString as float64ToString_399, stringBuilderAppendCodePoint as stringBuilderAppendCodePoint_419, stringForEach as stringForEach_420, mapConstructor as mapConstructor_426, stringGet as stringGet_438, stringNext as stringNext_440, panic as panic_445, pairConstructor as pairConstructor_450
} from "@temperlang/core";
export class ChangesetError extends type__6() {
  /** @type {string} */
  #field_0;
  /** @type {string} */
  #message_1;
  /**
   * @param {{
   *   field: string, message: string
   * }}
   * props
   * @returns {ChangesetError}
   */
  static["new"](props) {
    return new ChangesetError(props.field, props.message);
  }
  /**
   * @param {string} field_2
   * @param {string} message_3
   */
  constructor(field_2, message_3) {
    super ();
    this.#field_0 = field_2;
    this.#message_1 = message_3;
    return;
  }
  /** @returns {string} */
  get field() {
    return this.#field_0;
  }
  /** @returns {string} */
  get message() {
    return this.#message_1;
  }
};
export class Changeset extends type__6() {
  /** @returns {TableDef} */
  get tableDef() {
    null;
  }
  /** @returns {Map<string, string>} */
  get changes() {
    null;
  }
  /** @returns {Array<ChangesetError>} */
  get errors() {
    null;
  }
  /** @returns {boolean} */
  get isValid() {
    null;
  }
  /**
   * @param {Array<SafeIdentifier>} allowedFields_12
   * @returns {Changeset}
   */
  cast(allowedFields_12) {
    null;
  }
  /**
   * @param {Array<SafeIdentifier>} fields_14
   * @returns {Changeset}
   */
  validateRequired(fields_14) {
    null;
  }
  /**
   * @param {SafeIdentifier} field_16
   * @param {number} min_17
   * @param {number} max_18
   * @returns {Changeset}
   */
  validateLength(field_16, min_17, max_18) {
    null;
  }
  /**
   * @param {SafeIdentifier} field_20
   * @returns {Changeset}
   */
  validateInt(field_20) {
    null;
  }
  /**
   * @param {SafeIdentifier} field_22
   * @returns {Changeset}
   */
  validateInt64(field_22) {
    null;
  }
  /**
   * @param {SafeIdentifier} field_24
   * @returns {Changeset}
   */
  validateFloat(field_24) {
    null;
  }
  /**
   * @param {SafeIdentifier} field_26
   * @returns {Changeset}
   */
  validateBool(field_26) {
    null;
  }
  /** @returns {SqlFragment} */
  toInsertSql() {
    null;
  }
  /**
   * @param {number} id_29
   * @returns {SqlFragment}
   */
  toUpdateSql(id_29) {
    null;
  }
};
class ChangesetImpl_30 extends type__6(Changeset) {
  /** @type {TableDef} */
  #_tableDef_31;
  /** @type {Map<string, string>} */
  #_params_32;
  /** @type {Map<string, string>} */
  #_changes_33;
  /** @type {Array<ChangesetError>} */
  #_errors_34;
  /** @type {boolean} */
  #_isValid_35;
  /** @returns {TableDef} */
  get tableDef() {
    return this.#_tableDef_31;
  }
  /** @returns {Map<string, string>} */
  get changes() {
    return this.#_changes_33;
  }
  /** @returns {Array<ChangesetError>} */
  get errors() {
    return this.#_errors_34;
  }
  /** @returns {boolean} */
  get isValid() {
    return this.#_isValid_35;
  }
  /**
   * @param {Array<SafeIdentifier>} allowedFields_41
   * @returns {Changeset}
   */
  cast(allowedFields_41) {
    const this50 = this;
    const mb_42 = mapBuilderConstructor_43();
    function fn_44(f_45) {
      let t_46;
      let t_47 = f_45.sqlValue;
      const val_48 = mappedGetOr_49(this50.#_params_32, t_47, "");
      if (! ! val_48) {
        t_46 = f_45.sqlValue;
        mapBuilderSet_51(mb_42, t_46, val_48);
      }
      return;
    }
    allowedFields_41.forEach(fn_44);
    return new ChangesetImpl_30(this.#_tableDef_31, this.#_params_32, mappedToMap_52(mb_42), this.#_errors_34, this.#_isValid_35);
  }
  /**
   * @param {Array<SafeIdentifier>} fields_54
   * @returns {Changeset}
   */
  validateRequired(fields_54) {
    const this67 = this;
    let return_55;
    let t_56;
    let t_57;
    let t_58;
    let t_59;
    fn_60: {
      if (! this.#_isValid_35) {
        return_55 = this;
        break fn_60;
      }
      const eb_61 = this.#_errors_34.slice();
      let valid_62 = true;
      function fn_63(f_64) {
        let t_65;
        let t_66 = f_64.sqlValue;
        if (! this67.#_changes_33.has(t_66)) {
          t_65 = new ChangesetError(f_64.sqlValue, "is required");
          listBuilderAdd_68(eb_61, t_65);
          valid_62 = false;
        }
        return;
      }
      fields_54.forEach(fn_63);
      t_57 = this.#_tableDef_31;
      t_58 = this.#_params_32;
      t_59 = this.#_changes_33;
      t_56 = listBuilderToList_69(eb_61);
      return_55 = new ChangesetImpl_30(t_57, t_58, t_59, t_56, valid_62);
    }
    return return_55;
  }
  /**
   * @param {SafeIdentifier} field_71
   * @param {number} min_72
   * @param {number} max_73
   * @returns {Changeset}
   */
  validateLength(field_71, min_72, max_73) {
    let return_74;
    let t_75;
    let t_76;
    let t_77;
    let t_78;
    let t_79;
    let t_80;
    fn_81: {
      if (! this.#_isValid_35) {
        return_74 = this;
        break fn_81;
      }
      t_75 = field_71.sqlValue;
      const val_82 = mappedGetOr_49(this.#_changes_33, t_75, "");
      const len_83 = stringCountBetween_84(val_82, 0, val_82.length);
      if (len_83 < min_72) {
        t_77 = true;
      } else {
        t_77 = len_83 > max_73;
      }
      if (t_77) {
        const msg_85 = "must be between " + min_72.toString() + " and " + max_73.toString() + " characters";
        const eb_86 = this.#_errors_34.slice();
        listBuilderAdd_68(eb_86, new ChangesetError(field_71.sqlValue, msg_85));
        t_78 = this.#_tableDef_31;
        t_79 = this.#_params_32;
        t_80 = this.#_changes_33;
        t_76 = listBuilderToList_69(eb_86);
        return_74 = new ChangesetImpl_30(t_78, t_79, t_80, t_76, false);
        break fn_81;
      }
      return_74 = this;
    }
    return return_74;
  }
  /**
   * @param {SafeIdentifier} field_88
   * @returns {Changeset}
   */
  validateInt(field_88) {
    let return_89;
    let t_90;
    let t_91;
    let t_92;
    let t_93;
    let t_94;
    fn_95: {
      if (! this.#_isValid_35) {
        return_89 = this;
        break fn_95;
      }
      t_90 = field_88.sqlValue;
      const val_96 = mappedGetOr_49(this.#_changes_33, t_90, "");
      if (! val_96) {
        return_89 = this;
        break fn_95;
      }
      let parseOk_97;
      try {
        stringToInt32_98(val_96);
        parseOk_97 = true;
      } catch {
        parseOk_97 = false;
      }
      if (! parseOk_97) {
        const eb_99 = this.#_errors_34.slice();
        listBuilderAdd_68(eb_99, new ChangesetError(field_88.sqlValue, "must be an integer"));
        t_92 = this.#_tableDef_31;
        t_93 = this.#_params_32;
        t_94 = this.#_changes_33;
        t_91 = listBuilderToList_69(eb_99);
        return_89 = new ChangesetImpl_30(t_92, t_93, t_94, t_91, false);
        break fn_95;
      }
      return_89 = this;
    }
    return return_89;
  }
  /**
   * @param {SafeIdentifier} field_101
   * @returns {Changeset}
   */
  validateInt64(field_101) {
    let return_102;
    let t_103;
    let t_104;
    let t_105;
    let t_106;
    let t_107;
    fn_108: {
      if (! this.#_isValid_35) {
        return_102 = this;
        break fn_108;
      }
      t_103 = field_101.sqlValue;
      const val_109 = mappedGetOr_49(this.#_changes_33, t_103, "");
      if (! val_109) {
        return_102 = this;
        break fn_108;
      }
      let parseOk_110;
      try {
        stringToInt64_111(val_109);
        parseOk_110 = true;
      } catch {
        parseOk_110 = false;
      }
      if (! parseOk_110) {
        const eb_112 = this.#_errors_34.slice();
        listBuilderAdd_68(eb_112, new ChangesetError(field_101.sqlValue, "must be a 64-bit integer"));
        t_105 = this.#_tableDef_31;
        t_106 = this.#_params_32;
        t_107 = this.#_changes_33;
        t_104 = listBuilderToList_69(eb_112);
        return_102 = new ChangesetImpl_30(t_105, t_106, t_107, t_104, false);
        break fn_108;
      }
      return_102 = this;
    }
    return return_102;
  }
  /**
   * @param {SafeIdentifier} field_114
   * @returns {Changeset}
   */
  validateFloat(field_114) {
    let return_115;
    let t_116;
    let t_117;
    let t_118;
    let t_119;
    let t_120;
    fn_121: {
      if (! this.#_isValid_35) {
        return_115 = this;
        break fn_121;
      }
      t_116 = field_114.sqlValue;
      const val_122 = mappedGetOr_49(this.#_changes_33, t_116, "");
      if (! val_122) {
        return_115 = this;
        break fn_121;
      }
      let parseOk_123;
      try {
        stringToFloat64_124(val_122);
        parseOk_123 = true;
      } catch {
        parseOk_123 = false;
      }
      if (! parseOk_123) {
        const eb_125 = this.#_errors_34.slice();
        listBuilderAdd_68(eb_125, new ChangesetError(field_114.sqlValue, "must be a number"));
        t_118 = this.#_tableDef_31;
        t_119 = this.#_params_32;
        t_120 = this.#_changes_33;
        t_117 = listBuilderToList_69(eb_125);
        return_115 = new ChangesetImpl_30(t_118, t_119, t_120, t_117, false);
        break fn_121;
      }
      return_115 = this;
    }
    return return_115;
  }
  /**
   * @param {SafeIdentifier} field_127
   * @returns {Changeset}
   */
  validateBool(field_127) {
    let return_128;
    let t_129;
    let t_130;
    let t_131;
    let t_132;
    let t_133;
    let t_134;
    let t_135;
    let t_136;
    let t_137;
    let t_138;
    fn_139: {
      if (! this.#_isValid_35) {
        return_128 = this;
        break fn_139;
      }
      t_129 = field_127.sqlValue;
      const val_140 = mappedGetOr_49(this.#_changes_33, t_129, "");
      if (! val_140) {
        return_128 = this;
        break fn_139;
      }
      let isTrue_141;
      if (val_140 === "true") {
        isTrue_141 = true;
      } else {
        if (val_140 === "1") {
          t_132 = true;
        } else {
          if (val_140 === "yes") {
            t_131 = true;
          } else {
            t_131 = val_140 === "on";
          }
          t_132 = t_131;
        }
        isTrue_141 = t_132;
      }
      let isFalse_142;
      if (val_140 === "false") {
        isFalse_142 = true;
      } else {
        if (val_140 === "0") {
          t_134 = true;
        } else {
          if (val_140 === "no") {
            t_133 = true;
          } else {
            t_133 = val_140 === "off";
          }
          t_134 = t_133;
        }
        isFalse_142 = t_134;
      }
      if (! isTrue_141) {
        t_135 = ! isFalse_142;
      } else {
        t_135 = false;
      }
      if (t_135) {
        const eb_143 = this.#_errors_34.slice();
        listBuilderAdd_68(eb_143, new ChangesetError(field_127.sqlValue, "must be a boolean (true/false/1/0/yes/no/on/off)"));
        t_136 = this.#_tableDef_31;
        t_137 = this.#_params_32;
        t_138 = this.#_changes_33;
        t_130 = listBuilderToList_69(eb_143);
        return_128 = new ChangesetImpl_30(t_136, t_137, t_138, t_130, false);
        break fn_139;
      }
      return_128 = this;
    }
    return return_128;
  }
  /**
   * @param {string} val_146
   * @returns {SqlBoolean}
   */
  #parseBoolSqlPart_145(val_146) {
    let return_147;
    let t_148;
    let t_149;
    let t_150;
    let t_151;
    let t_152;
    let t_153;
    fn_154: {
      if (val_146 === "true") {
        t_150 = true;
      } else {
        if (val_146 === "1") {
          t_149 = true;
        } else {
          if (val_146 === "yes") {
            t_148 = true;
          } else {
            t_148 = val_146 === "on";
          }
          t_149 = t_148;
        }
        t_150 = t_149;
      }
      if (t_150) {
        return_147 = new SqlBoolean(true);
        break fn_154;
      }
      if (val_146 === "false") {
        t_153 = true;
      } else {
        if (val_146 === "0") {
          t_152 = true;
        } else {
          if (val_146 === "no") {
            t_151 = true;
          } else {
            t_151 = val_146 === "off";
          }
          t_152 = t_151;
        }
        t_153 = t_152;
      }
      if (t_153) {
        return_147 = new SqlBoolean(false);
        break fn_154;
      }
      throw Error();
    }
    return return_147;
  }
  /**
   * @param {FieldDef} fieldDef_157
   * @param {string} val_158
   * @returns {SqlPart}
   */
  #valueToSqlPart_156(fieldDef_157, val_158) {
    let return_159;
    let t_160;
    let t_161;
    let t_162;
    let t_163;
    fn_164: {
      const ft_165 = fieldDef_157.fieldType;
      if (ft_165 instanceof StringField) {
        return_159 = new SqlString(val_158);
        break fn_164;
      }
      if (ft_165 instanceof IntField) {
        t_160 = stringToInt32_98(val_158);
        return_159 = new SqlInt32(t_160);
        break fn_164;
      }
      if (ft_165 instanceof Int64Field) {
        t_161 = stringToInt64_111(val_158);
        return_159 = new SqlInt64(t_161);
        break fn_164;
      }
      if (ft_165 instanceof FloatField) {
        t_162 = stringToFloat64_124(val_158);
        return_159 = new SqlFloat64(t_162);
        break fn_164;
      }
      if (ft_165 instanceof BoolField) {
        return_159 = this.#parseBoolSqlPart_145(val_158);
        break fn_164;
      }
      if (ft_165 instanceof DateField) {
        t_163 = new (globalThis.Date)(globalThis.Date.parse(val_158));
        return_159 = new SqlDate(t_163);
        break fn_164;
      }
      throw Error();
    }
    return return_159;
  }
  /** @returns {SqlFragment} */
  toInsertSql() {
    let t_167;
    let t_168;
    let t_169;
    let t_170;
    let t_171;
    let t_172;
    let t_173;
    let t_174;
    let t_175;
    let t_176;
    if (! this.#_isValid_35) {
      throw Error();
    }
    let i_177 = 0;
    while (true) {
      t_167 = this.#_tableDef_31.fields.length;
      if (!(i_177 < t_167)) {
        break;
      }
      const f_178 = listedGet_179(this.#_tableDef_31.fields, i_177);
      if (! f_178.nullable) {
        t_168 = f_178.name.sqlValue;
        t_169 = this.#_changes_33.has(t_168);
        t_174 = ! t_169;
      } else {
        t_174 = false;
      }
      if (t_174) {
        throw Error();
      }
      i_177 = i_177 + 1 | 0;
    }
    const pairs_180 = mappedToList_181(this.#_changes_33);
    if (pairs_180.length === 0) {
      throw Error();
    }
    const colNames_182 = [];
    const valParts_183 = [];
    let i_184 = 0;
    while (true) {
      t_170 = pairs_180.length;
      if (!(i_184 < t_170)) {
        break;
      }
      const pair_185 = listedGet_179(pairs_180, i_184);
      t_171 = pair_185.key;
      t_175 = this.#_tableDef_31.field(t_171);
      const fd_186 = t_175;
      listBuilderAdd_68(colNames_182, pair_185.key);
      t_172 = pair_185.value;
      t_176 = this.#valueToSqlPart_156(fd_186, t_172);
      listBuilderAdd_68(valParts_183, t_176);
      i_184 = i_184 + 1 | 0;
    }
    const b_187 = new SqlBuilder();
    b_187.appendSafe("INSERT INTO ");
    b_187.appendSafe(this.#_tableDef_31.tableName.sqlValue);
    b_187.appendSafe(" (");
    let t_188 = listBuilderToList_69(colNames_182);
    function fn_189(c_190) {
      return c_190;
    }
    b_187.appendSafe(listedJoin_191(t_188, ", ", fn_189));
    b_187.appendSafe(") VALUES (");
    b_187.appendPart(listedGet_179(valParts_183, 0));
    let j_192 = 1;
    while (true) {
      t_173 = valParts_183.length;
      if (!(j_192 < t_173)) {
        break;
      }
      b_187.appendSafe(", ");
      b_187.appendPart(listedGet_179(valParts_183, j_192));
      j_192 = j_192 + 1 | 0;
    }
    b_187.appendSafe(")");
    return b_187.accumulated;
  }
  /**
   * @param {number} id_194
   * @returns {SqlFragment}
   */
  toUpdateSql(id_194) {
    let t_195;
    let t_196;
    let t_197;
    let t_198;
    let t_199;
    if (! this.#_isValid_35) {
      throw Error();
    }
    const pairs_200 = mappedToList_181(this.#_changes_33);
    if (pairs_200.length === 0) {
      throw Error();
    }
    const b_201 = new SqlBuilder();
    b_201.appendSafe("UPDATE ");
    b_201.appendSafe(this.#_tableDef_31.tableName.sqlValue);
    b_201.appendSafe(" SET ");
    let i_202 = 0;
    while (true) {
      t_195 = pairs_200.length;
      if (!(i_202 < t_195)) {
        break;
      }
      if (i_202 > 0) {
        b_201.appendSafe(", ");
      }
      const pair_203 = listedGet_179(pairs_200, i_202);
      t_196 = pair_203.key;
      t_198 = this.#_tableDef_31.field(t_196);
      const fd_204 = t_198;
      b_201.appendSafe(pair_203.key);
      b_201.appendSafe(" = ");
      t_197 = pair_203.value;
      t_199 = this.#valueToSqlPart_156(fd_204, t_197);
      b_201.appendPart(t_199);
      i_202 = i_202 + 1 | 0;
    }
    b_201.appendSafe(" WHERE id = ");
    b_201.appendInt32(id_194);
    return b_201.accumulated;
  }
  /**
   * @param {{
   *   _tableDef: TableDef, _params: Map<string, string>, _changes: Map<string, string>, _errors: Array<ChangesetError>, _isValid: boolean
   * }}
   * props
   * @returns {ChangesetImpl_30}
   */
  static["new"](props) {
    return new ChangesetImpl_30(props._tableDef, props._params, props._changes, props._errors, props._isValid);
  }
  /**
   * @param {TableDef} _tableDef_205
   * @param {Map<string, string>} _params_206
   * @param {Map<string, string>} _changes_207
   * @param {Array<ChangesetError>} _errors_208
   * @param {boolean} _isValid_209
   */
  constructor(_tableDef_205, _params_206, _changes_207, _errors_208, _isValid_209) {
    super ();
    this.#_tableDef_31 = _tableDef_205;
    this.#_params_32 = _params_206;
    this.#_changes_33 = _changes_207;
    this.#_errors_34 = _errors_208;
    this.#_isValid_35 = _isValid_209;
    return;
  }
}
export class OrderClause extends type__6() {
  /** @type {SafeIdentifier} */
  #field_210;
  /** @type {boolean} */
  #ascending_211;
  /**
   * @param {{
   *   field: SafeIdentifier, ascending: boolean
   * }}
   * props
   * @returns {OrderClause}
   */
  static["new"](props) {
    return new OrderClause(props.field, props.ascending);
  }
  /**
   * @param {SafeIdentifier} field_212
   * @param {boolean} ascending_213
   */
  constructor(field_212, ascending_213) {
    super ();
    this.#field_210 = field_212;
    this.#ascending_211 = ascending_213;
    return;
  }
  /** @returns {SafeIdentifier} */
  get field() {
    return this.#field_210;
  }
  /** @returns {boolean} */
  get ascending() {
    return this.#ascending_211;
  }
};
export class Query extends type__6() {
  /** @type {SafeIdentifier} */
  #tableName_216;
  /** @type {Array<SqlFragment>} */
  #conditions_217;
  /** @type {Array<SafeIdentifier>} */
  #selectedFields_218;
  /** @type {Array<OrderClause>} */
  #orderClauses_219;
  /** @type {number | null} */
  #limitVal_220;
  /** @type {number | null} */
  #offsetVal_221;
  /**
   * @param {SqlFragment} condition_223
   * @returns {Query}
   */
  where(condition_223) {
    const nb_224 = this.#conditions_217.slice();
    listBuilderAdd_68(nb_224, condition_223);
    return new Query(this.#tableName_216, listBuilderToList_69(nb_224), this.#selectedFields_218, this.#orderClauses_219, this.#limitVal_220, this.#offsetVal_221);
  }
  /**
   * @param {Array<SafeIdentifier>} fields_226
   * @returns {Query}
   */
  select(fields_226) {
    return new Query(this.#tableName_216, this.#conditions_217, fields_226, this.#orderClauses_219, this.#limitVal_220, this.#offsetVal_221);
  }
  /**
   * @param {SafeIdentifier} field_228
   * @param {boolean} ascending_229
   * @returns {Query}
   */
  orderBy(field_228, ascending_229) {
    const nb_230 = this.#orderClauses_219.slice();
    listBuilderAdd_68(nb_230, new OrderClause(field_228, ascending_229));
    return new Query(this.#tableName_216, this.#conditions_217, this.#selectedFields_218, listBuilderToList_69(nb_230), this.#limitVal_220, this.#offsetVal_221);
  }
  /**
   * @param {number} n_232
   * @returns {Query}
   */
  limit(n_232) {
    if (n_232 < 0) {
      throw Error();
    }
    return new Query(this.#tableName_216, this.#conditions_217, this.#selectedFields_218, this.#orderClauses_219, n_232, this.#offsetVal_221);
  }
  /**
   * @param {number} n_234
   * @returns {Query}
   */
  offset(n_234) {
    if (n_234 < 0) {
      throw Error();
    }
    return new Query(this.#tableName_216, this.#conditions_217, this.#selectedFields_218, this.#orderClauses_219, this.#limitVal_220, n_234);
  }
  /** @returns {SqlFragment} */
  toSql() {
    let t_236;
    const b_237 = new SqlBuilder();
    b_237.appendSafe("SELECT ");
    if (! this.#selectedFields_218.length) {
      b_237.appendSafe("*");
    } else {
      function fn_238(f_239) {
        return f_239.sqlValue;
      }
      b_237.appendSafe(listedJoin_191(this.#selectedFields_218, ", ", fn_238));
    }
    b_237.appendSafe(" FROM ");
    b_237.appendSafe(this.#tableName_216.sqlValue);
    if (! ! this.#conditions_217.length) {
      b_237.appendSafe(" WHERE ");
      b_237.appendFragment(listedGet_179(this.#conditions_217, 0));
      let i_240 = 1;
      while (true) {
        t_236 = this.#conditions_217.length;
        if (!(i_240 < t_236)) {
          break;
        }
        b_237.appendSafe(" AND ");
        b_237.appendFragment(listedGet_179(this.#conditions_217, i_240));
        i_240 = i_240 + 1 | 0;
      }
    }
    if (! ! this.#orderClauses_219.length) {
      b_237.appendSafe(" ORDER BY ");
      let first_241 = true;
      function fn_242(oc_243) {
        let t_244;
        if (! first_241) {
          b_237.appendSafe(", ");
        }
        first_241 = false;
        let t_245 = oc_243.field.sqlValue;
        b_237.appendSafe(t_245);
        if (oc_243.ascending) {
          t_244 = " ASC";
        } else {
          t_244 = " DESC";
        }
        b_237.appendSafe(t_244);
        return;
      }
      this.#orderClauses_219.forEach(fn_242);
    }
    const lv_246 = this.#limitVal_220;
    if (!(lv_246 == null)) {
      const lv_247 = lv_246;
      b_237.appendSafe(" LIMIT ");
      b_237.appendInt32(lv_247);
    }
    const ov_248 = this.#offsetVal_221;
    if (!(ov_248 == null)) {
      const ov_249 = ov_248;
      b_237.appendSafe(" OFFSET ");
      b_237.appendInt32(ov_249);
    }
    return b_237.accumulated;
  }
  /**
   * @param {number} defaultLimit_251
   * @returns {SqlFragment}
   */
  safeToSql(defaultLimit_251) {
    let return_252;
    let t_253;
    if (defaultLimit_251 < 0) {
      throw Error();
    }
    if (!(this.#limitVal_220 == null)) {
      return_252 = this.toSql();
    } else {
      t_253 = this.limit(defaultLimit_251);
      return_252 = t_253.toSql();
    }
    return return_252;
  }
  /**
   * @param {{
   *   tableName: SafeIdentifier, conditions: Array<SqlFragment>, selectedFields: Array<SafeIdentifier>, orderClauses: Array<OrderClause>, limitVal: number | null, offsetVal: number | null
   * }}
   * props
   * @returns {Query}
   */
  static["new"](props) {
    return new Query(props.tableName, props.conditions, props.selectedFields, props.orderClauses, props.limitVal, props.offsetVal);
  }
  /**
   * @param {SafeIdentifier} tableName_254
   * @param {Array<SqlFragment>} conditions_255
   * @param {Array<SafeIdentifier>} selectedFields_256
   * @param {Array<OrderClause>} orderClauses_257
   * @param {number | null} limitVal_258
   * @param {number | null} offsetVal_259
   */
  constructor(tableName_254, conditions_255, selectedFields_256, orderClauses_257, limitVal_258, offsetVal_259) {
    super ();
    this.#tableName_216 = tableName_254;
    this.#conditions_217 = conditions_255;
    this.#selectedFields_218 = selectedFields_256;
    this.#orderClauses_219 = orderClauses_257;
    this.#limitVal_220 = limitVal_258;
    this.#offsetVal_221 = offsetVal_259;
    return;
  }
  /** @returns {SafeIdentifier} */
  get tableName() {
    return this.#tableName_216;
  }
  /** @returns {Array<SqlFragment>} */
  get conditions() {
    return this.#conditions_217;
  }
  /** @returns {Array<SafeIdentifier>} */
  get selectedFields() {
    return this.#selectedFields_218;
  }
  /** @returns {Array<OrderClause>} */
  get orderClauses() {
    return this.#orderClauses_219;
  }
  /** @returns {number | null} */
  get limitVal() {
    return this.#limitVal_220;
  }
  /** @returns {number | null} */
  get offsetVal() {
    return this.#offsetVal_221;
  }
};
export class SafeIdentifier extends type__6() {
  /** @returns {string} */
  get sqlValue() {
    null;
  }
};
class ValidatedIdentifier_267 extends type__6(SafeIdentifier) {
  /** @type {string} */
  #_value_268;
  /** @returns {string} */
  get sqlValue() {
    return this.#_value_268;
  }
  /** @param {string} _value_270 */
  constructor(_value_270) {
    super ();
    this.#_value_268 = _value_270;
    return;
  }
}
export class FieldType extends type__6() {
};
export class StringField extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class IntField extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class Int64Field extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class FloatField extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class BoolField extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class DateField extends type__6(FieldType) {
  constructor() {
    super ();
    return;
  }
};
export class FieldDef extends type__6() {
  /** @type {SafeIdentifier} */
  #name_271;
  /** @type {FieldType} */
  #fieldType_272;
  /** @type {boolean} */
  #nullable_273;
  /**
   * @param {{
   *   name: SafeIdentifier, fieldType: FieldType, nullable: boolean
   * }}
   * props
   * @returns {FieldDef}
   */
  static["new"](props) {
    return new FieldDef(props.name, props.fieldType, props.nullable);
  }
  /**
   * @param {SafeIdentifier} name_274
   * @param {FieldType} fieldType_275
   * @param {boolean} nullable_276
   */
  constructor(name_274, fieldType_275, nullable_276) {
    super ();
    this.#name_271 = name_274;
    this.#fieldType_272 = fieldType_275;
    this.#nullable_273 = nullable_276;
    return;
  }
  /** @returns {SafeIdentifier} */
  get name() {
    return this.#name_271;
  }
  /** @returns {FieldType} */
  get fieldType() {
    return this.#fieldType_272;
  }
  /** @returns {boolean} */
  get nullable() {
    return this.#nullable_273;
  }
};
export class TableDef extends type__6() {
  /** @type {SafeIdentifier} */
  #tableName_280;
  /** @type {Array<FieldDef>} */
  #fields_281;
  /**
   * @param {string} name_283
   * @returns {FieldDef}
   */
  field(name_283) {
    let return_284;
    fn_285: {
      const this_286 = this.#fields_281;
      const n_287 = this_286.length;
      let i_288 = 0;
      while (i_288 < n_287) {
        const el_289 = listedGet_179(this_286, i_288);
        i_288 = i_288 + 1 | 0;
        const f_290 = el_289;
        if (f_290.name.sqlValue === name_283) {
          return_284 = f_290;
          break fn_285;
        }
      }
      throw Error();
    }
    return return_284;
  }
  /**
   * @param {{
   *   tableName: SafeIdentifier, fields: Array<FieldDef>
   * }}
   * props
   * @returns {TableDef}
   */
  static["new"](props) {
    return new TableDef(props.tableName, props.fields);
  }
  /**
   * @param {SafeIdentifier} tableName_291
   * @param {Array<FieldDef>} fields_292
   */
  constructor(tableName_291, fields_292) {
    super ();
    this.#tableName_280 = tableName_291;
    this.#fields_281 = fields_292;
    return;
  }
  /** @returns {SafeIdentifier} */
  get tableName() {
    return this.#tableName_280;
  }
  /** @returns {Array<FieldDef>} */
  get fields() {
    return this.#fields_281;
  }
};
export class SqlBuilder extends type__6() {
  /** @type {Array<SqlPart>} */
  #buffer_295;
  /** @param {string} sqlSource_297 */
  appendSafe(sqlSource_297) {
    let t_298 = new SqlSource(sqlSource_297);
    listBuilderAdd_68(this.#buffer_295, t_298);
    return;
  }
  /** @param {SqlFragment} fragment_300 */
  appendFragment(fragment_300) {
    let t_301 = fragment_300.parts;
    listBuilderAddAll_302(this.#buffer_295, t_301);
    return;
  }
  /** @param {SqlPart} part_304 */
  appendPart(part_304) {
    listBuilderAdd_68(this.#buffer_295, part_304);
    return;
  }
  /** @param {Array<SqlPart>} values_306 */
  appendPartList(values_306) {
    const this309 = this;
    function fn_307(x_308) {
      this309.appendPart(x_308);
      return;
    }
    this.#appendList_310(values_306, fn_307);
    return;
  }
  /** @param {boolean} value_312 */
  appendBoolean(value_312) {
    let t_313 = new SqlBoolean(value_312);
    listBuilderAdd_68(this.#buffer_295, t_313);
    return;
  }
  /** @param {Array<boolean>} values_315 */
  appendBooleanList(values_315) {
    const this318 = this;
    function fn_316(x_317) {
      this318.appendBoolean(x_317);
      return;
    }
    this.#appendList_310(values_315, fn_316);
    return;
  }
  /** @param {globalThis.Date} value_320 */
  appendDate(value_320) {
    let t_321 = new SqlDate(value_320);
    listBuilderAdd_68(this.#buffer_295, t_321);
    return;
  }
  /** @param {Array<globalThis.Date>} values_323 */
  appendDateList(values_323) {
    const this326 = this;
    function fn_324(x_325) {
      this326.appendDate(x_325);
      return;
    }
    this.#appendList_310(values_323, fn_324);
    return;
  }
  /** @param {number} value_328 */
  appendFloat64(value_328) {
    let t_329 = new SqlFloat64(value_328);
    listBuilderAdd_68(this.#buffer_295, t_329);
    return;
  }
  /** @param {Array<number>} values_331 */
  appendFloat64List(values_331) {
    const this334 = this;
    function fn_332(x_333) {
      this334.appendFloat64(x_333);
      return;
    }
    this.#appendList_310(values_331, fn_332);
    return;
  }
  /** @param {number} value_336 */
  appendInt32(value_336) {
    let t_337 = new SqlInt32(value_336);
    listBuilderAdd_68(this.#buffer_295, t_337);
    return;
  }
  /** @param {Array<number>} values_339 */
  appendInt32List(values_339) {
    const this342 = this;
    function fn_340(x_341) {
      this342.appendInt32(x_341);
      return;
    }
    this.#appendList_310(values_339, fn_340);
    return;
  }
  /** @param {bigint} value_344 */
  appendInt64(value_344) {
    let t_345 = new SqlInt64(value_344);
    listBuilderAdd_68(this.#buffer_295, t_345);
    return;
  }
  /** @param {Array<bigint>} values_347 */
  appendInt64List(values_347) {
    const this350 = this;
    function fn_348(x_349) {
      this350.appendInt64(x_349);
      return;
    }
    this.#appendList_310(values_347, fn_348);
    return;
  }
  /** @param {string} value_352 */
  appendString(value_352) {
    let t_353 = new SqlString(value_352);
    listBuilderAdd_68(this.#buffer_295, t_353);
    return;
  }
  /** @param {Array<string>} values_355 */
  appendStringList(values_355) {
    const this358 = this;
    function fn_356(x_357) {
      this358.appendString(x_357);
      return;
    }
    this.#appendList_310(values_355, fn_356);
    return;
  }
  /**
   * @template {unknown} T_365
   * @param {Array<T_365>} values_360
   * @param {(arg0: T_365) => void} appendValue_361
   */
  #appendList_310(values_360, appendValue_361) {
    let t_362;
    let t_363;
    let i_364 = 0;
    while (true) {
      t_362 = values_360.length;
      if (!(i_364 < t_362)) {
        break;
      }
      if (i_364 > 0) {
        this.appendSafe(", ");
      }
      t_363 = listedGet_179(values_360, i_364);
      appendValue_361(t_363);
      i_364 = i_364 + 1 | 0;
    }
    return;
  }
  /** @returns {SqlFragment} */
  get accumulated() {
    return new SqlFragment(listBuilderToList_69(this.#buffer_295));
  }
  constructor() {
    super ();
    let t_367 = [];
    this.#buffer_295 = t_367;
    return;
  }
};
export class SqlFragment extends type__6() {
  /** @type {Array<SqlPart>} */
  #parts_368;
  /** @returns {SqlSource} */
  toSource() {
    return new SqlSource(this.toString());
  }
  /** @returns {string} */
  toString() {
    let t_371;
    const builder_372 = [""];
    let i_373 = 0;
    while (true) {
      t_371 = this.#parts_368.length;
      if (!(i_373 < t_371)) {
        break;
      }
      listedGet_179(this.#parts_368, i_373).formatTo(builder_372);
      i_373 = i_373 + 1 | 0;
    }
    return builder_372[0];
  }
  /** @param {Array<SqlPart>} parts_374 */
  constructor(parts_374) {
    super ();
    this.#parts_368 = parts_374;
    return;
  }
  /** @returns {Array<SqlPart>} */
  get parts() {
    return this.#parts_368;
  }
};
export class SqlPart extends type__6() {
  /** @param {globalThis.Array<string>} builder_377 */
  formatTo(builder_377) {
    null;
  }
};
export class SqlSource extends type__6(SqlPart) {
  /** @type {string} */
  #source_378;
  /** @param {globalThis.Array<string>} builder_380 */
  formatTo(builder_380) {
    builder_380[0] += this.#source_378;
    return;
  }
  /** @param {string} source_381 */
  constructor(source_381) {
    super ();
    this.#source_378 = source_381;
    return;
  }
  /** @returns {string} */
  get source() {
    return this.#source_378;
  }
};
export class SqlBoolean extends type__6(SqlPart) {
  /** @type {boolean} */
  #value_383;
  /** @param {globalThis.Array<string>} builder_385 */
  formatTo(builder_385) {
    let t_386;
    if (this.#value_383) {
      t_386 = "TRUE";
    } else {
      t_386 = "FALSE";
    }
    builder_385[0] += t_386;
    return;
  }
  /** @param {boolean} value_387 */
  constructor(value_387) {
    super ();
    this.#value_383 = value_387;
    return;
  }
  /** @returns {boolean} */
  get value() {
    return this.#value_383;
  }
};
export class SqlDate extends type__6(SqlPart) {
  /** @type {globalThis.Date} */
  #value_389;
  /** @param {globalThis.Array<string>} builder_391 */
  formatTo(builder_391) {
    builder_391[0] += "'";
    let t_392 = this.#value_389.toISOString().split("T")[0];
    builder_391[0] += t_392;
    builder_391[0] += "'";
    return;
  }
  /** @param {globalThis.Date} value_393 */
  constructor(value_393) {
    super ();
    this.#value_389 = value_393;
    return;
  }
  /** @returns {globalThis.Date} */
  get value() {
    return this.#value_389;
  }
};
export class SqlFloat64 extends type__6(SqlPart) {
  /** @type {number} */
  #value_395;
  /** @param {globalThis.Array<string>} builder_397 */
  formatTo(builder_397) {
    let t_398 = float64ToString_399(this.#value_395);
    builder_397[0] += t_398;
    return;
  }
  /** @param {number} value_400 */
  constructor(value_400) {
    super ();
    this.#value_395 = value_400;
    return;
  }
  /** @returns {number} */
  get value() {
    return this.#value_395;
  }
};
export class SqlInt32 extends type__6(SqlPart) {
  /** @type {number} */
  #value_402;
  /** @param {globalThis.Array<string>} builder_404 */
  formatTo(builder_404) {
    let t_405 = this.#value_402.toString();
    builder_404[0] += t_405;
    return;
  }
  /** @param {number} value_406 */
  constructor(value_406) {
    super ();
    this.#value_402 = value_406;
    return;
  }
  /** @returns {number} */
  get value() {
    return this.#value_402;
  }
};
export class SqlInt64 extends type__6(SqlPart) {
  /** @type {bigint} */
  #value_408;
  /** @param {globalThis.Array<string>} builder_410 */
  formatTo(builder_410) {
    let t_411 = this.#value_408.toString();
    builder_410[0] += t_411;
    return;
  }
  /** @param {bigint} value_412 */
  constructor(value_412) {
    super ();
    this.#value_408 = value_412;
    return;
  }
  /** @returns {bigint} */
  get value() {
    return this.#value_408;
  }
};
export class SqlString extends type__6(SqlPart) {
  /** @type {string} */
  #value_414;
  /** @param {globalThis.Array<string>} builder_416 */
  formatTo(builder_416) {
    builder_416[0] += "'";
    function fn_417(c_418) {
      if (c_418 === 39) {
        builder_416[0] += "''";
      } else {
        try {
          stringBuilderAppendCodePoint_419(builder_416, c_418);
        } catch {
          throw Error();
        }
      }
      return;
    }
    stringForEach_420(this.#value_414, fn_417);
    builder_416[0] += "'";
    return;
  }
  /** @param {string} value_421 */
  constructor(value_421) {
    super ();
    this.#value_414 = value_421;
    return;
  }
  /** @returns {string} */
  get value() {
    return this.#value_414;
  }
};
/**
 * @param {TableDef} tableDef_423
 * @param {Map<string, string>} params_424
 * @returns {Changeset}
 */
export function changeset(tableDef_423, params_424) {
  let t_425 = mapConstructor_426(Object.freeze([]));
  return new ChangesetImpl_30(tableDef_423, params_424, t_425, Object.freeze([]), true);
};
/**
 * @param {number} c_428
 * @returns {boolean}
 */
function isIdentStart_427(c_428) {
  let return_429;
  let t_430;
  let t_431;
  if (c_428 >= 97) {
    t_430 = c_428 <= 122;
  } else {
    t_430 = false;
  }
  if (t_430) {
    return_429 = true;
  } else {
    if (c_428 >= 65) {
      t_431 = c_428 <= 90;
    } else {
      t_431 = false;
    }
    if (t_431) {
      return_429 = true;
    } else {
      return_429 = c_428 === 95;
    }
  }
  return return_429;
}
/**
 * @param {number} c_433
 * @returns {boolean}
 */
function isIdentPart_432(c_433) {
  let return_434;
  if (isIdentStart_427(c_433)) {
    return_434 = true;
  } else if (c_433 >= 48) {
    return_434 = c_433 <= 57;
  } else {
    return_434 = false;
  }
  return return_434;
}
/**
 * @param {string} name_435
 * @returns {SafeIdentifier}
 */
export function safeIdentifier(name_435) {
  let t_436;
  if (! name_435) {
    throw Error();
  }
  let idx_437 = 0;
  if (! isIdentStart_427(stringGet_438(name_435, idx_437))) {
    throw Error();
  }
  let t_439 = stringNext_440(name_435, idx_437);
  idx_437 = t_439;
  while (true) {
    if (!(name_435.length > idx_437)) {
      break;
    }
    if (! isIdentPart_432(stringGet_438(name_435, idx_437))) {
      throw Error();
    }
    t_436 = stringNext_440(name_435, idx_437);
    idx_437 = t_436;
  }
  return new ValidatedIdentifier_267(name_435);
};
/**
 * @param {TableDef} tableDef_654
 * @param {number} id_655
 * @returns {SqlFragment}
 */
export function deleteSql(tableDef_654, id_655) {
  const b_656 = new SqlBuilder();
  b_656.appendSafe("DELETE FROM ");
  b_656.appendSafe(tableDef_654.tableName.sqlValue);
  b_656.appendSafe(" WHERE id = ");
  b_656.appendInt32(id_655);
  return b_656.accumulated;
};
/**
 * @param {SafeIdentifier} tableName_657
 * @returns {Query}
 */
export function from(tableName_657) {
  return new Query(tableName_657, Object.freeze([]), Object.freeze([]), Object.freeze([]), null, null);
};
