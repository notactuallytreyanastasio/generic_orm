const {
  imul: imul__421
} = globalThis.Math;
import {
  type as type__2, requireInstanceOf as requireInstanceOf__216, clampInt64 as clampInt64__603, cmpFloat as cmpFloat__615, mappedGetOr as mappedGetOr_34, listedGet as listedGet_36, mappedForEach as mappedForEach_48, int64ToInt32 as int64ToInt32_88, int64ToFloat64 as int64ToFloat64_91, float64ToString as float64ToString_98, float64ToInt32 as float64ToInt32_100, float64ToInt64 as float64ToInt64_102, stringToInt32 as stringToInt32_114, stringToFloat64 as stringToFloat64_115, stringToInt64 as stringToInt64_120, listBuilderAdd as listBuilderAdd_132, listedGetOr as listedGetOr_136, listBuilderSet as listBuilderSet_144, listBuilderRemoveLast as listBuilderRemoveLast_149, mapBuilderConstructor as mapBuilderConstructor_210, panic as panic_217, mappedGet as mappedGet_222, mapBuilderSet as mapBuilderSet_223, listBuilderToList as listBuilderToList_229, mappedToMap as mappedToMap_230, stringGet as stringGet_279, stringNext as stringNext_387, stringHasAtLeast as stringHasAtLeast_458, stringBuilderAppendCodePoint as stringBuilderAppendCodePoint_463, requireStringIndex as requireStringIndex_492, int64ToInt32Unsafe as int64ToInt32Unsafe_612
} from "@temperlang/core";
export class InterchangeContext extends type__2() {
  /**
   * @param {string} headerName_1
   * @returns {string | null}
   */
  getHeader(headerName_1) {
    null;
  }
};
export class NullInterchangeContext extends type__2(InterchangeContext) {
  /**
   * @param {string} headerName_4
   * @returns {string | null}
   */
  getHeader(headerName_4) {
    return null;
  }
  /** @type {NullInterchangeContext} */
  static #instance_5 = new NullInterchangeContext();
  /** @returns {NullInterchangeContext} */
  static get instance() {
    return this.#instance_5;
  }
  constructor() {
    super ();
    return;
  }
};
export class JsonProducer extends type__2() {
  startObject() {
    null;
  }
  endObject() {
    null;
  }
  /** @param {string} key_10 */
  objectKey(key_10) {
    null;
  }
  startArray() {
    null;
  }
  endArray() {
    null;
  }
  nullValue() {
    null;
  }
  /** @param {boolean} x_15 */
  booleanValue(x_15) {
    null;
  }
  /** @param {number} x_17 */
  int32Value(x_17) {
    null;
  }
  /** @param {bigint} x_19 */
  int64Value(x_19) {
    null;
  }
  /** @param {number} x_21 */
  float64Value(x_21) {
    null;
  }
  /** @param {string} x_23 */
  numericTokenValue(x_23) {
    null;
  }
  /** @param {string} x_25 */
  stringValue(x_25) {
    null;
  }
  /** @returns {JsonParseErrorReceiver | null} */
  get parseErrorReceiver() {
    return null;
  }
};
export class JsonSyntaxTree extends type__2() {
  /** @param {JsonProducer} p_28 */
  produce(p_28) {
    null;
  }
};
export class JsonObject extends type__2(JsonSyntaxTree) {
  /** @type {Map<string, Array<JsonSyntaxTree>>} */
  #properties_29;
  /**
   * @param {string} propertyKey_31
   * @returns {JsonSyntaxTree | null}
   */
  propertyValueOrNull(propertyKey_31) {
    let return_32;
    const treeList_33 = mappedGetOr_34(this.#properties_29, propertyKey_31, Object.freeze([]));
    const lastIndex_35 = treeList_33.length - 1 | 0;
    if (lastIndex_35 >= 0) {
      return_32 = listedGet_36(treeList_33, lastIndex_35);
    } else {
      return_32 = null;
    }
    return return_32;
  }
  /**
   * @param {string} propertyKey_38
   * @returns {JsonSyntaxTree}
   */
  propertyValueOrBubble(propertyKey_38) {
    let return_39;
    let t_40 = this.propertyValueOrNull(propertyKey_38);
    if (t_40 == null) {
      throw Error();
    } else {
      return_39 = t_40;
    }
    return return_39;
  }
  /** @param {JsonProducer} p_42 */
  produce(p_42) {
    p_42.startObject();
    function fn_43(k_44, vs_45) {
      function fn_46(v_47) {
        p_42.objectKey(k_44);
        v_47.produce(p_42);
        return;
      }
      vs_45.forEach(fn_46);
      return;
    }
    mappedForEach_48(this.#properties_29, fn_43);
    p_42.endObject();
    return;
  }
  /** @param {Map<string, Array<JsonSyntaxTree>>} properties_49 */
  constructor(properties_49) {
    super ();
    this.#properties_29 = properties_49;
    return;
  }
  /** @returns {Map<string, Array<JsonSyntaxTree>>} */
  get properties() {
    return this.#properties_29;
  }
};
export class JsonArray extends type__2(JsonSyntaxTree) {
  /** @type {Array<JsonSyntaxTree>} */
  #elements_51;
  /** @param {JsonProducer} p_53 */
  produce(p_53) {
    p_53.startArray();
    function fn_54(v_55) {
      v_55.produce(p_53);
      return;
    }
    this.#elements_51.forEach(fn_54);
    p_53.endArray();
    return;
  }
  /** @param {Array<JsonSyntaxTree>} elements_56 */
  constructor(elements_56) {
    super ();
    this.#elements_51 = elements_56;
    return;
  }
  /** @returns {Array<JsonSyntaxTree>} */
  get elements() {
    return this.#elements_51;
  }
};
export class JsonBoolean extends type__2(JsonSyntaxTree) {
  /** @type {boolean} */
  #content_58;
  /** @param {JsonProducer} p_60 */
  produce(p_60) {
    p_60.booleanValue(this.#content_58);
    return;
  }
  /** @param {boolean} content_61 */
  constructor(content_61) {
    super ();
    this.#content_58 = content_61;
    return;
  }
  /** @returns {boolean} */
  get content() {
    return this.#content_58;
  }
};
export class JsonNull extends type__2(JsonSyntaxTree) {
  /** @param {JsonProducer} p_64 */
  produce(p_64) {
    p_64.nullValue();
    return;
  }
  constructor() {
    super ();
    return;
  }
};
export class JsonString extends type__2(JsonSyntaxTree) {
  /** @type {string} */
  #content_65;
  /** @param {JsonProducer} p_67 */
  produce(p_67) {
    p_67.stringValue(this.#content_65);
    return;
  }
  /** @param {string} content_68 */
  constructor(content_68) {
    super ();
    this.#content_65 = content_68;
    return;
  }
  /** @returns {string} */
  get content() {
    return this.#content_65;
  }
};
export class JsonNumeric extends type__2(JsonSyntaxTree) {
  /** @returns {string} */
  asJsonNumericToken() {
    null;
  }
  /** @returns {number} */
  asInt32() {
    null;
  }
  /** @returns {bigint} */
  asInt64() {
    null;
  }
  /** @returns {number} */
  asFloat64() {
    null;
  }
};
export class JsonInt32 extends type__2(JsonNumeric) {
  /** @type {number} */
  #content_74;
  /** @param {JsonProducer} p_76 */
  produce(p_76) {
    p_76.int32Value(this.#content_74);
    return;
  }
  /** @returns {string} */
  asJsonNumericToken() {
    return this.#content_74.toString();
  }
  /** @returns {number} */
  asInt32() {
    return this.#content_74;
  }
  /** @returns {bigint} */
  asInt64() {
    return BigInt(this.#content_74);
  }
  /** @returns {number} */
  asFloat64() {
    return this.#content_74;
  }
  /** @param {number} content_81 */
  constructor(content_81) {
    super ();
    this.#content_74 = content_81;
    return;
  }
  /** @returns {number} */
  get content() {
    return this.#content_74;
  }
};
export class JsonInt64 extends type__2(JsonNumeric) {
  /** @type {bigint} */
  #content_83;
  /** @param {JsonProducer} p_85 */
  produce(p_85) {
    p_85.int64Value(this.#content_83);
    return;
  }
  /** @returns {string} */
  asJsonNumericToken() {
    return this.#content_83.toString();
  }
  /** @returns {number} */
  asInt32() {
    return int64ToInt32_88(this.#content_83);
  }
  /** @returns {bigint} */
  asInt64() {
    return this.#content_83;
  }
  /** @returns {number} */
  asFloat64() {
    return int64ToFloat64_91(this.#content_83);
  }
  /** @param {bigint} content_92 */
  constructor(content_92) {
    super ();
    this.#content_83 = content_92;
    return;
  }
  /** @returns {bigint} */
  get content() {
    return this.#content_83;
  }
};
export class JsonFloat64 extends type__2(JsonNumeric) {
  /** @type {number} */
  #content_94;
  /** @param {JsonProducer} p_96 */
  produce(p_96) {
    p_96.float64Value(this.#content_94);
    return;
  }
  /** @returns {string} */
  asJsonNumericToken() {
    return float64ToString_98(this.#content_94);
  }
  /** @returns {number} */
  asInt32() {
    return float64ToInt32_100(this.#content_94);
  }
  /** @returns {bigint} */
  asInt64() {
    return float64ToInt64_102(this.#content_94);
  }
  /** @returns {number} */
  asFloat64() {
    return this.#content_94;
  }
  /** @param {number} content_104 */
  constructor(content_104) {
    super ();
    this.#content_94 = content_104;
    return;
  }
  /** @returns {number} */
  get content() {
    return this.#content_94;
  }
};
export class JsonNumericToken extends type__2(JsonNumeric) {
  /** @type {string} */
  #content_106;
  /** @param {JsonProducer} p_108 */
  produce(p_108) {
    p_108.numericTokenValue(this.#content_106);
    return;
  }
  /** @returns {string} */
  asJsonNumericToken() {
    return this.#content_106;
  }
  /** @returns {number} */
  asInt32() {
    let return_111;
    let t_112;
    let t_113;
    try {
      t_112 = stringToInt32_114(this.#content_106);
      return_111 = t_112;
    } catch {
      t_113 = stringToFloat64_115(this.#content_106);
      return_111 = float64ToInt32_100(t_113);
    }
    return return_111;
  }
  /** @returns {bigint} */
  asInt64() {
    let return_117;
    let t_118;
    let t_119;
    try {
      t_118 = stringToInt64_120(this.#content_106);
      return_117 = t_118;
    } catch {
      t_119 = stringToFloat64_115(this.#content_106);
      return_117 = float64ToInt64_102(t_119);
    }
    return return_117;
  }
  /** @returns {number} */
  asFloat64() {
    return stringToFloat64_115(this.#content_106);
  }
  /** @param {string} content_122 */
  constructor(content_122) {
    super ();
    this.#content_106 = content_122;
    return;
  }
  /** @returns {string} */
  get content() {
    return this.#content_106;
  }
};
export class JsonTextProducer extends type__2(JsonProducer) {
  /** @type {InterchangeContext} */
  #interchangeContext_124;
  /** @type {globalThis.Array<string>} */
  #buffer_125;
  /** @type {Array<number>} */
  #stack_126;
  /** @type {boolean} */
  #wellFormed_127;
  /** @param {InterchangeContext | null} [interchangeContext_128] */
  constructor(interchangeContext_128) {
    super ();
    let interchangeContext_129;
    if (interchangeContext_128 == null) {
      interchangeContext_129 = NullInterchangeContext.instance;
    } else {
      interchangeContext_129 = interchangeContext_128;
    }
    this.#interchangeContext_124 = interchangeContext_129;
    let t_130 = [""];
    this.#buffer_125 = t_130;
    let t_131 = [];
    this.#stack_126 = t_131;
    listBuilderAdd_132(this.#stack_126, 5);
    this.#wellFormed_127 = true;
    return;
  }
  /** @returns {number} */
  #state_134() {
    let t_135 = this.#stack_126.length;
    return listedGetOr_136(this.#stack_126, t_135 - 1 | 0, -1);
  }
  #beforeValue_138() {
    let t_139;
    let t_140;
    let t_141;
    let t_142;
    const currentState_143 = this.#state_134();
    if (currentState_143 === 3) {
      t_139 = this.#stack_126.length;
      listBuilderSet_144(this.#stack_126, t_139 - 1 | 0, 4);
    } else if (currentState_143 === 4) {
      this.#buffer_125[0] += ",";
    } else if (currentState_143 === 1) {
      t_140 = this.#stack_126.length;
      listBuilderSet_144(this.#stack_126, t_140 - 1 | 0, 2);
    } else if (currentState_143 === 5) {
      t_141 = this.#stack_126.length;
      listBuilderSet_144(this.#stack_126, t_141 - 1 | 0, 6);
    } else {
      if (currentState_143 === 6) {
        t_142 = true;
      } else {
        t_142 = currentState_143 === 2;
      }
      if (t_142) {
        this.#wellFormed_127 = false;
      }
    }
    return;
  }
  startObject() {
    this.#beforeValue_138();
    this.#buffer_125[0] += "{";
    listBuilderAdd_132(this.#stack_126, 0);
    return;
  }
  endObject() {
    let t_147;
    this.#buffer_125[0] += "}";
    const currentState_148 = this.#state_134();
    if (0 === currentState_148) {
      t_147 = true;
    } else {
      t_147 = 2 === currentState_148;
    }
    if (t_147) {
      listBuilderRemoveLast_149(this.#stack_126);
    } else {
      this.#wellFormed_127 = false;
    }
    return;
  }
  /** @param {string} key_151 */
  objectKey(key_151) {
    let t_152;
    const currentState_153 = this.#state_134();
    if (!(currentState_153 === 0)) {
      if (currentState_153 === 2) {
        this.#buffer_125[0] += ",";
      } else {
        this.#wellFormed_127 = false;
      }
    }
    encodeJsonString_154(key_151, this.#buffer_125);
    this.#buffer_125[0] += ":";
    if (currentState_153 >= 0) {
      t_152 = this.#stack_126.length;
      listBuilderSet_144(this.#stack_126, t_152 - 1 | 0, 1);
    }
    return;
  }
  startArray() {
    this.#beforeValue_138();
    this.#buffer_125[0] += "[";
    listBuilderAdd_132(this.#stack_126, 3);
    return;
  }
  endArray() {
    let t_157;
    this.#buffer_125[0] += "]";
    const currentState_158 = this.#state_134();
    if (3 === currentState_158) {
      t_157 = true;
    } else {
      t_157 = 4 === currentState_158;
    }
    if (t_157) {
      listBuilderRemoveLast_149(this.#stack_126);
    } else {
      this.#wellFormed_127 = false;
    }
    return;
  }
  nullValue() {
    this.#beforeValue_138();
    this.#buffer_125[0] += "null";
    return;
  }
  /** @param {boolean} x_161 */
  booleanValue(x_161) {
    let t_162;
    this.#beforeValue_138();
    if (x_161) {
      t_162 = "true";
    } else {
      t_162 = "false";
    }
    this.#buffer_125[0] += t_162;
    return;
  }
  /** @param {number} x_164 */
  int32Value(x_164) {
    this.#beforeValue_138();
    let t_165 = x_164.toString();
    this.#buffer_125[0] += t_165;
    return;
  }
  /** @param {bigint} x_167 */
  int64Value(x_167) {
    this.#beforeValue_138();
    let t_168 = x_167.toString();
    this.#buffer_125[0] += t_168;
    return;
  }
  /** @param {number} x_170 */
  float64Value(x_170) {
    this.#beforeValue_138();
    let t_171 = float64ToString_98(x_170);
    this.#buffer_125[0] += t_171;
    return;
  }
  /** @param {string} x_173 */
  numericTokenValue(x_173) {
    this.#beforeValue_138();
    this.#buffer_125[0] += x_173;
    return;
  }
  /** @param {string} x_175 */
  stringValue(x_175) {
    this.#beforeValue_138();
    encodeJsonString_154(x_175, this.#buffer_125);
    return;
  }
  /** @returns {string} */
  toJsonString() {
    let return_177;
    let t_178;
    let t_179;
    let t_180;
    if (this.#wellFormed_127) {
      if (this.#stack_126.length === 1) {
        t_178 = this.#state_134();
        t_179 = t_178 === 6;
      } else {
        t_179 = false;
      }
      t_180 = t_179;
    } else {
      t_180 = false;
    }
    if (t_180) {
      return_177 = this.#buffer_125[0];
    } else {
      throw Error();
    }
    return return_177;
  }
  /** @returns {InterchangeContext} */
  get interchangeContext() {
    return this.#interchangeContext_124;
  }
};
export class JsonParseErrorReceiver extends type__2() {
  /** @param {string} explanation_183 */
  explainJsonError(explanation_183) {
    null;
  }
};
export class JsonSyntaxTreeProducer extends type__2(JsonProducer, JsonParseErrorReceiver) {
  /** @type {Array<Array<JsonSyntaxTree>>} */
  #stack_184;
  /** @type {string | null} */
  #error_185;
  /** @returns {InterchangeContext} */
  get interchangeContext() {
    return NullInterchangeContext.instance;
  }
  constructor() {
    super ();
    let t_187 = [];
    this.#stack_184 = t_187;
    let t_188 = [];
    listBuilderAdd_132(this.#stack_184, t_188);
    this.#error_185 = null;
    return;
  }
  /** @param {JsonSyntaxTree} v_191 */
  #storeValue_190(v_191) {
    let t_192;
    if (! ! this.#stack_184.length) {
      t_192 = this.#stack_184.length;
      listBuilderAdd_132(listedGet_36(this.#stack_184, t_192 - 1 | 0), v_191);
    }
    return;
  }
  startObject() {
    let t_194 = [];
    listBuilderAdd_132(this.#stack_184, t_194);
    return;
  }
  endObject() {
    let return_196;
    let t_197;
    let t_198;
    let t_199;
    let t_200;
    let t_201;
    let t_202;
    let t_203;
    let t_204;
    let t_205;
    let t_206;
    fn_207: {
      if (! this.#stack_184.length) {
        return_196 = void 0;
        break fn_207;
      }
      const ls_208 = listBuilderRemoveLast_149(this.#stack_184);
      const m_209 = mapBuilderConstructor_210();
      let multis_211 = null;
      let i_212 = 0;
      let n_213 = ls_208.length & -2;
      while (i_212 < n_213) {
        const postfixReturn_214 = i_212;
        i_212 = i_212 + 1 | 0;
        const keyTree_215 = listedGet_36(ls_208, postfixReturn_214);
        if (!(keyTree_215 instanceof JsonString)) {
          break;
        }
        try {
          t_199 = requireInstanceOf__216(keyTree_215, JsonString);
          t_200 = t_199;
        } catch {
          t_200 = panic_217();
        }
        const key_218 = t_200.content;
        const postfixReturn_219 = i_212;
        i_212 = i_212 + 1 | 0;
        const value_220 = listedGet_36(ls_208, postfixReturn_219);
        if (m_209.has(key_218)) {
          if (multis_211 == null) {
            t_197 = mapBuilderConstructor_210();
            multis_211 = t_197;
          }
          try {
            if (multis_211 == null) {
              throw Error();
            } else {
              t_201 = multis_211;
            }
            t_202 = t_201;
          } catch {
            t_202 = panic_217();
          }
          const mb_221 = t_202;
          if (! mb_221.has(key_218)) {
            try {
              t_203 = mappedGet_222(m_209, key_218);
              t_204 = t_203;
            } catch {
              t_204 = panic_217();
            }
            mapBuilderSet_223(mb_221, key_218, t_204.slice());
          }
          try {
            t_205 = mappedGet_222(mb_221, key_218);
            t_206 = t_205;
          } catch {
            t_206 = panic_217();
          }
          listBuilderAdd_132(t_206, value_220);
        } else {
          mapBuilderSet_223(m_209, key_218, Object.freeze([value_220]));
        }
      }
      const multis_224 = multis_211;
      if (!(multis_224 == null)) {
        function fn_225(k_226, vs_227) {
          let t_228 = listBuilderToList_229(vs_227);
          mapBuilderSet_223(m_209, k_226, t_228);
          return;
        }
        mappedForEach_48(multis_224, fn_225);
      }
      t_198 = new JsonObject(mappedToMap_230(m_209));
      this.#storeValue_190(t_198);
      return_196 = void 0;
    }
    return return_196;
  }
  /** @param {string} key_232 */
  objectKey(key_232) {
    let t_233 = new JsonString(key_232);
    this.#storeValue_190(t_233);
    return;
  }
  startArray() {
    let t_235 = [];
    listBuilderAdd_132(this.#stack_184, t_235);
    return;
  }
  endArray() {
    let return_237;
    let t_238;
    fn_239: {
      if (! this.#stack_184.length) {
        return_237 = void 0;
        break fn_239;
      }
      const ls_240 = listBuilderRemoveLast_149(this.#stack_184);
      t_238 = new JsonArray(listBuilderToList_229(ls_240));
      this.#storeValue_190(t_238);
      return_237 = void 0;
    }
    return return_237;
  }
  nullValue() {
    let t_242 = new JsonNull();
    this.#storeValue_190(t_242);
    return;
  }
  /** @param {boolean} x_244 */
  booleanValue(x_244) {
    let t_245 = new JsonBoolean(x_244);
    this.#storeValue_190(t_245);
    return;
  }
  /** @param {number} x_247 */
  int32Value(x_247) {
    let t_248 = new JsonInt32(x_247);
    this.#storeValue_190(t_248);
    return;
  }
  /** @param {bigint} x_250 */
  int64Value(x_250) {
    let t_251 = new JsonInt64(x_250);
    this.#storeValue_190(t_251);
    return;
  }
  /** @param {number} x_253 */
  float64Value(x_253) {
    let t_254 = new JsonFloat64(x_253);
    this.#storeValue_190(t_254);
    return;
  }
  /** @param {string} x_256 */
  numericTokenValue(x_256) {
    let t_257 = new JsonNumericToken(x_256);
    this.#storeValue_190(t_257);
    return;
  }
  /** @param {string} x_259 */
  stringValue(x_259) {
    let t_260 = new JsonString(x_259);
    this.#storeValue_190(t_260);
    return;
  }
  /** @returns {JsonSyntaxTree} */
  toJsonSyntaxTree() {
    let t_262;
    if (this.#stack_184.length !== 1) {
      t_262 = true;
    } else {
      t_262 = !(this.#error_185 == null);
    }
    if (t_262) {
      throw Error();
    }
    const ls_263 = listedGet_36(this.#stack_184, 0);
    if (ls_263.length !== 1) {
      throw Error();
    }
    return listedGet_36(ls_263, 0);
  }
  /** @returns {string | null} */
  get jsonError() {
    return this.#error_185;
  }
  /** @returns {JsonParseErrorReceiver} */
  get parseErrorReceiver() {
    return this;
  }
  /** @param {string} error_267 */
  explainJsonError(error_267) {
    this.#error_185 = error_267;
    return;
  }
};
/**
 * @param {string} sourceText_269
 * @param {globalThis.number} i_270
 * @param {JsonProducer} out_271
 * @returns {globalThis.number}
 */
function parseJsonValue_268(sourceText_269, i_270, out_271) {
  let return_272;
  let t_273;
  let t_274;
  let t_275;
  fn_276: {
    t_273 = skipJsonSpaces_277(sourceText_269, i_270);
    i_270 = t_273;
    if (!(sourceText_269.length > i_270)) {
      expectedTokenError_278(sourceText_269, i_270, out_271, "JSON value");
      return_272 = -1;
      break fn_276;
    }
    t_274 = stringGet_279(sourceText_269, i_270);
    if (t_274 === 123) {
      return_272 = parseJsonObject_280(sourceText_269, i_270, out_271);
    } else if (t_274 === 91) {
      return_272 = parseJsonArray_281(sourceText_269, i_270, out_271);
    } else if (t_274 === 34) {
      return_272 = parseJsonString_282(sourceText_269, i_270, out_271);
    } else {
      if (t_274 === 116) {
        t_275 = true;
      } else {
        t_275 = t_274 === 102;
      }
      if (t_275) {
        return_272 = parseJsonBoolean_283(sourceText_269, i_270, out_271);
      } else if (t_274 === 110) {
        return_272 = parseJsonNull_284(sourceText_269, i_270, out_271);
      } else {
        return_272 = parseJsonNumber_285(sourceText_269, i_270, out_271);
      }
    }
  }
  return return_272;
}
/** @template T_286 */
export class JsonAdapter extends type__2() {
  /**
   * @param {T_286} x_288
   * @param {JsonProducer} p_289
   */
  encodeToJson(x_288, p_289) {
    null;
  }
  /**
   * @param {JsonSyntaxTree} t_291
   * @param {InterchangeContext} ic_292
   * @returns {T_286}
   */
  decodeFromJson(t_291, ic_292) {
    null;
  }
};
class BooleanJsonAdapter_293 extends type__2(JsonAdapter) {
  /**
   * @param {boolean} x_295
   * @param {JsonProducer} p_296
   */
  encodeToJson(x_295, p_296) {
    p_296.booleanValue(x_295);
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_298
   * @param {InterchangeContext} ic_299
   * @returns {boolean}
   */
  decodeFromJson(t_298, ic_299) {
    let t_300;
    t_300 = requireInstanceOf__216(t_298, JsonBoolean);
    return t_300.content;
  }
  constructor() {
    super ();
    return;
  }
}
class Float64JsonAdapter_301 extends type__2(JsonAdapter) {
  /**
   * @param {number} x_303
   * @param {JsonProducer} p_304
   */
  encodeToJson(x_303, p_304) {
    p_304.float64Value(x_303);
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_306
   * @param {InterchangeContext} ic_307
   * @returns {number}
   */
  decodeFromJson(t_306, ic_307) {
    let t_308;
    t_308 = requireInstanceOf__216(t_306, JsonNumeric);
    return t_308.asFloat64();
  }
  constructor() {
    super ();
    return;
  }
}
class Int32JsonAdapter_309 extends type__2(JsonAdapter) {
  /**
   * @param {number} x_311
   * @param {JsonProducer} p_312
   */
  encodeToJson(x_311, p_312) {
    p_312.int32Value(x_311);
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_314
   * @param {InterchangeContext} ic_315
   * @returns {number}
   */
  decodeFromJson(t_314, ic_315) {
    let t_316;
    t_316 = requireInstanceOf__216(t_314, JsonNumeric);
    return t_316.asInt32();
  }
  constructor() {
    super ();
    return;
  }
}
class Int64JsonAdapter_317 extends type__2(JsonAdapter) {
  /**
   * @param {bigint} x_319
   * @param {JsonProducer} p_320
   */
  encodeToJson(x_319, p_320) {
    p_320.int64Value(x_319);
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_322
   * @param {InterchangeContext} ic_323
   * @returns {bigint}
   */
  decodeFromJson(t_322, ic_323) {
    let t_324;
    t_324 = requireInstanceOf__216(t_322, JsonNumeric);
    return t_324.asInt64();
  }
  constructor() {
    super ();
    return;
  }
}
class StringJsonAdapter_325 extends type__2(JsonAdapter) {
  /**
   * @param {string} x_327
   * @param {JsonProducer} p_328
   */
  encodeToJson(x_327, p_328) {
    p_328.stringValue(x_327);
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_330
   * @param {InterchangeContext} ic_331
   * @returns {string}
   */
  decodeFromJson(t_330, ic_331) {
    let t_332;
    t_332 = requireInstanceOf__216(t_330, JsonString);
    return t_332.content;
  }
  constructor() {
    super ();
    return;
  }
}
/** @template T_334 */
class ListJsonAdapter_333 extends type__2(JsonAdapter) {
  /** @type {JsonAdapter<T_334>} */
  #adapterForT_335;
  /**
   * @param {Array<T_334>} x_337
   * @param {JsonProducer} p_338
   */
  encodeToJson(x_337, p_338) {
    const this341 = this;
    p_338.startArray();
    function fn_339(el_340) {
      this341.#adapterForT_335.encodeToJson(el_340, p_338);
      return;
    }
    x_337.forEach(fn_339);
    p_338.endArray();
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_343
   * @param {InterchangeContext} ic_344
   * @returns {Array<T_334>}
   */
  decodeFromJson(t_343, ic_344) {
    let t_345;
    const b_346 = [];
    let t_347;
    t_347 = requireInstanceOf__216(t_343, JsonArray);
    const elements_348 = t_347.elements;
    const n_349 = elements_348.length;
    let i_350 = 0;
    while (i_350 < n_349) {
      const el_351 = listedGet_36(elements_348, i_350);
      i_350 = i_350 + 1 | 0;
      t_345 = this.#adapterForT_335.decodeFromJson(el_351, ic_344);
      listBuilderAdd_132(b_346, t_345);
    }
    return listBuilderToList_229(b_346);
  }
  /** @param {JsonAdapter<T_334>} adapterForT_352 */
  constructor(adapterForT_352) {
    super ();
    this.#adapterForT_335 = adapterForT_352;
    return;
  }
}
/** @template T_353 */
export class OrNullJsonAdapter extends type__2(JsonAdapter) {
  /** @type {JsonAdapter<T_353>} */
  #adapterForT_354;
  /**
   * @param {T_353 | null} x_356
   * @param {JsonProducer} p_357
   */
  encodeToJson(x_356, p_357) {
    if (x_356 == null) {
      p_357.nullValue();
    } else {
      const x_358 = x_356;
      this.#adapterForT_354.encodeToJson(x_358, p_357);
    }
    return;
  }
  /**
   * @param {JsonSyntaxTree} t_360
   * @param {InterchangeContext} ic_361
   * @returns {T_353 | null}
   */
  decodeFromJson(t_360, ic_361) {
    let return_362;
    if (t_360 instanceof JsonNull) {
      return_362 = null;
    } else {
      return_362 = this.#adapterForT_354.decodeFromJson(t_360, ic_361);
    }
    return return_362;
  }
  /** @param {JsonAdapter<T_353>} adapterForT_363 */
  constructor(adapterForT_363) {
    super ();
    this.#adapterForT_354 = adapterForT_363;
    return;
  }
};
/** @type {Array<string>} */
const hexDigits_364 = Object.freeze(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"]);
/**
 * @param {number} cp_366
 * @param {globalThis.Array<string>} buffer_367
 */
function encodeHex4_365(cp_366, buffer_367) {
  const b0_368 = (cp_366 / 4096 | 0) & 15;
  const b1_369 = (cp_366 / 256 | 0) & 15;
  const b2_370 = (cp_366 / 16 | 0) & 15;
  const b3_371 = cp_366 & 15;
  let t_372 = listedGet_36(hexDigits_364, b0_368);
  buffer_367[0] += t_372;
  let t_373 = listedGet_36(hexDigits_364, b1_369);
  buffer_367[0] += t_373;
  let t_374 = listedGet_36(hexDigits_364, b2_370);
  buffer_367[0] += t_374;
  let t_375 = listedGet_36(hexDigits_364, b3_371);
  buffer_367[0] += t_375;
  return;
}
/**
 * @param {string} x_376
 * @param {globalThis.Array<string>} buffer_377
 */
function encodeJsonString_154(x_376, buffer_377) {
  let t_378;
  let t_379;
  let t_380;
  let t_381;
  buffer_377[0] += '"';
  let i_382 = 0;
  let emitted_383 = i_382;
  while (true) {
    if (!(x_376.length > i_382)) {
      break;
    }
    const cp_384 = stringGet_279(x_376, i_382);
    if (cp_384 === 8) {
      t_381 = "\\b";
    } else if (cp_384 === 9) {
      t_381 = "\\t";
    } else if (cp_384 === 10) {
      t_381 = "\\n";
    } else if (cp_384 === 12) {
      t_381 = "\\f";
    } else if (cp_384 === 13) {
      t_381 = "\\r";
    } else if (cp_384 === 34) {
      t_381 = '\\"';
    } else if (cp_384 === 92) {
      t_381 = "\\\\";
    } else {
      if (cp_384 < 32) {
        t_379 = true;
      } else {
        if (55296 <= cp_384) {
          t_378 = cp_384 <= 57343;
        } else {
          t_378 = false;
        }
        t_379 = t_378;
      }
      if (t_379) {
        t_380 = "\\u";
      } else {
        t_380 = "";
      }
      t_381 = t_380;
    }
    const replacement_385 = t_381;
    const nextI_386 = stringNext_387(x_376, i_382);
    if (replacement_385 !== "") {
      buffer_377[0] += x_376.substring(emitted_383, i_382);
      buffer_377[0] += replacement_385;
      if (replacement_385 === "\\u") {
        encodeHex4_365(cp_384, buffer_377);
      }
      emitted_383 = nextI_386;
    }
    i_382 = nextI_386;
  }
  buffer_377[0] += x_376.substring(emitted_383, i_382);
  buffer_377[0] += '"';
  return;
}
/**
 * @param {JsonProducer} out_389
 * @param {string} explanation_390
 */
function storeJsonError_388(out_389, explanation_390) {
  let t_391 = out_389.parseErrorReceiver;
  if (!(t_391 == null)) {
    t_391.explainJsonError(explanation_390);
  }
  return;
}
/**
 * @param {string} sourceText_392
 * @param {globalThis.number} i_393
 * @param {JsonProducer} out_394
 * @param {string} shortExplanation_395
 */
function expectedTokenError_278(sourceText_392, i_393, out_394, shortExplanation_395) {
  let t_396;
  let t_397;
  let gotten_398;
  if (sourceText_392.length > i_393) {
    t_396 = sourceText_392.length;
    t_397 = sourceText_392.substring(i_393, t_396);
    gotten_398 = "`" + t_397 + "`";
  } else {
    gotten_398 = "end-of-file";
  }
  storeJsonError_388(out_394, "Expected " + shortExplanation_395 + ", but got " + gotten_398);
  return;
}
/**
 * @param {string} sourceText_399
 * @param {globalThis.number} i_400
 * @returns {globalThis.number}
 */
function skipJsonSpaces_277(sourceText_399, i_400) {
  let t_401;
  let t_402;
  let t_403;
  let t_404;
  let t_405;
  while (true) {
    if (!(sourceText_399.length > i_400)) {
      break;
    }
    t_401 = stringGet_279(sourceText_399, i_400);
    if (t_401 === 9) {
      t_405 = true;
    } else {
      if (t_401 === 10) {
        t_404 = true;
      } else {
        if (t_401 === 13) {
          t_403 = true;
        } else {
          t_403 = t_401 === 32;
        }
        t_404 = t_403;
      }
      t_405 = t_404;
    }
    if (! t_405) {
      break;
    }
    t_402 = stringNext_387(sourceText_399, i_400);
    i_400 = t_402;
  }
  return i_400;
}
/**
 * @param {string} sourceText_407
 * @param {globalThis.number} start_408
 * @param {globalThis.number} limit_409
 * @returns {number}
 */
function decodeHexUnsigned_406(sourceText_407, start_408, limit_409) {
  let return_410;
  let t_411;
  let t_412;
  let t_413;
  let t_414;
  let t_415;
  fn_416: {
    let n_417 = 0;
    let i_418 = start_408;
    while (true) {
      if (!(i_418 - limit_409 < 0)) {
        break;
      }
      const cp_419 = stringGet_279(sourceText_407, i_418);
      if (48 <= cp_419) {
        t_412 = cp_419 <= 48;
      } else {
        t_412 = false;
      }
      if (t_412) {
        t_415 = cp_419 - 48 | 0;
      } else {
        if (65 <= cp_419) {
          t_413 = cp_419 <= 70;
        } else {
          t_413 = false;
        }
        if (t_413) {
          t_415 = (cp_419 - 65 | 0) + 10 | 0;
        } else {
          if (97 <= cp_419) {
            t_414 = cp_419 <= 102;
          } else {
            t_414 = false;
          }
          if (t_414) {
            t_415 = (cp_419 - 97 | 0) + 10 | 0;
          } else {
            return_410 = -1;
            break fn_416;
          }
        }
      }
      const digit_420 = t_415;
      n_417 = imul__421(n_417, 16) + digit_420 | 0;
      t_411 = stringNext_387(sourceText_407, i_418);
      i_418 = t_411;
    }
    return_410 = n_417;
  }
  return return_410;
}
/**
 * @param {string} sourceText_423
 * @param {globalThis.number} i_424
 * @param {globalThis.Array<string>} sb_425
 * @param {JsonProducer} errOut_426
 * @returns {globalThis.number}
 */
function parseJsonStringTo_422(sourceText_423, i_424, sb_425, errOut_426) {
  let return_427;
  let t_428;
  let t_429;
  let t_430;
  let t_431;
  let t_432;
  let t_433;
  let t_434;
  let t_435;
  let t_436;
  let t_437;
  let t_438;
  let t_439;
  let t_440;
  let t_441;
  let t_442;
  let t_443;
  let t_444;
  let t_445;
  let t_446;
  let t_447;
  let t_448;
  let t_449;
  fn_450: {
    if (!(sourceText_423.length > i_424)) {
      t_439 = true;
    } else {
      t_428 = stringGet_279(sourceText_423, i_424);
      t_439 = t_428 !== 34;
    }
    if (t_439) {
      expectedTokenError_278(sourceText_423, i_424, errOut_426, '"');
      return_427 = -1;
      break fn_450;
    }
    t_429 = stringNext_387(sourceText_423, i_424);
    i_424 = t_429;
    let leadSurrogate_451 = -1;
    let consumed_452 = i_424;
    while (true) {
      if (!(sourceText_423.length > i_424)) {
        break;
      }
      const cp_453 = stringGet_279(sourceText_423, i_424);
      if (cp_453 === 34) {
        break;
      }
      t_430 = stringNext_387(sourceText_423, i_424);
      let iNext_454 = t_430;
      const end_455 = sourceText_423.length;
      let needToFlush_456 = false;
      if (cp_453 !== 92) {
        t_445 = cp_453;
      } else {
        needToFlush_456 = true;
        if (!(sourceText_423.length > iNext_454)) {
          expectedTokenError_278(sourceText_423, iNext_454, errOut_426, "escape sequence");
          return_427 = -1;
          break fn_450;
        }
        const esc0_457 = stringGet_279(sourceText_423, iNext_454);
        t_431 = stringNext_387(sourceText_423, iNext_454);
        iNext_454 = t_431;
        if (esc0_457 === 34) {
          t_441 = true;
        } else {
          if (esc0_457 === 92) {
            t_440 = true;
          } else {
            t_440 = esc0_457 === 47;
          }
          t_441 = t_440;
        }
        if (t_441) {
          t_444 = esc0_457;
        } else if (esc0_457 === 98) {
          t_444 = 8;
        } else if (esc0_457 === 102) {
          t_444 = 12;
        } else if (esc0_457 === 110) {
          t_444 = 10;
        } else if (esc0_457 === 114) {
          t_444 = 13;
        } else if (esc0_457 === 116) {
          t_444 = 9;
        } else if (esc0_457 === 117) {
          if (stringHasAtLeast_458(sourceText_423, iNext_454, end_455, 4)) {
            const startHex_459 = iNext_454;
            t_432 = stringNext_387(sourceText_423, iNext_454);
            iNext_454 = t_432;
            t_433 = stringNext_387(sourceText_423, iNext_454);
            iNext_454 = t_433;
            t_434 = stringNext_387(sourceText_423, iNext_454);
            iNext_454 = t_434;
            t_435 = stringNext_387(sourceText_423, iNext_454);
            iNext_454 = t_435;
            t_436 = decodeHexUnsigned_406(sourceText_423, startHex_459, iNext_454);
            t_442 = t_436;
          } else {
            t_442 = -1;
          }
          const hex_460 = t_442;
          if (hex_460 < 0) {
            expectedTokenError_278(sourceText_423, iNext_454, errOut_426, "four hex digits");
            return_427 = -1;
            break fn_450;
          }
          t_443 = hex_460;
          t_444 = t_443;
        } else {
          expectedTokenError_278(sourceText_423, iNext_454, errOut_426, "escape sequence");
          return_427 = -1;
          break fn_450;
        }
        t_445 = t_444;
      }
      let decodedCp_461 = t_445;
      if (leadSurrogate_451 >= 0) {
        needToFlush_456 = true;
        const lead_462 = leadSurrogate_451;
        if (56320 <= decodedCp_461) {
          t_446 = decodedCp_461 <= 57343;
        } else {
          t_446 = false;
        }
        if (t_446) {
          leadSurrogate_451 = -1;
          decodedCp_461 = 65536 +(imul__421(lead_462 - 55296 | 0, 1024) |(decodedCp_461 - 56320 | 0)) | 0;
        }
      } else {
        if (55296 <= decodedCp_461) {
          t_447 = decodedCp_461 <= 56319;
        } else {
          t_447 = false;
        }
        if (t_447) {
          needToFlush_456 = true;
        }
      }
      if (needToFlush_456) {
        sb_425[0] += sourceText_423.substring(consumed_452, i_424);
        if (leadSurrogate_451 >= 0) {
          try {
            stringBuilderAppendCodePoint_463(sb_425, leadSurrogate_451);
          } catch {
            throw Error();
          }
        }
        if (55296 <= decodedCp_461) {
          t_448 = decodedCp_461 <= 56319;
        } else {
          t_448 = false;
        }
        if (t_448) {
          leadSurrogate_451 = decodedCp_461;
        } else {
          leadSurrogate_451 = -1;
          try {
            stringBuilderAppendCodePoint_463(sb_425, decodedCp_461);
          } catch {
            throw Error();
          }
        }
        consumed_452 = iNext_454;
      }
      i_424 = iNext_454;
    }
    if (!(sourceText_423.length > i_424)) {
      t_449 = true;
    } else {
      t_437 = stringGet_279(sourceText_423, i_424);
      t_449 = t_437 !== 34;
    }
    if (t_449) {
      expectedTokenError_278(sourceText_423, i_424, errOut_426, '"');
      return_427 = -1;
    } else {
      if (leadSurrogate_451 >= 0) {
        try {
          stringBuilderAppendCodePoint_463(sb_425, leadSurrogate_451);
        } catch {
          throw Error();
        }
      } else {
        sb_425[0] += sourceText_423.substring(consumed_452, i_424);
      }
      t_438 = stringNext_387(sourceText_423, i_424);
      i_424 = t_438;
      return_427 = i_424;
    }
  }
  return return_427;
}
/**
 * @param {string} sourceText_464
 * @param {globalThis.number} i_465
 * @param {JsonProducer} out_466
 * @returns {globalThis.number}
 */
function parseJsonObject_280(sourceText_464, i_465, out_466) {
  let return_467;
  let t_468;
  let t_469;
  let t_470;
  let t_471;
  let t_472;
  let t_473;
  let t_474;
  let t_475;
  let t_476;
  let t_477;
  let t_478;
  let t_479;
  let t_480;
  let t_481;
  let t_482;
  let t_483;
  let t_484;
  let t_485;
  let t_486;
  let t_487;
  let t_488;
  fn_489: {
    try {
      if (!(sourceText_464.length > i_465)) {
        t_481 = true;
      } else {
        t_468 = stringGet_279(sourceText_464, i_465);
        t_481 = t_468 !== 123;
      }
      if (t_481) {
        expectedTokenError_278(sourceText_464, i_465, out_466, "'{'");
        return_467 = -1;
        break fn_489;
      }
      out_466.startObject();
      t_469 = stringNext_387(sourceText_464, i_465);
      t_470 = skipJsonSpaces_277(sourceText_464, t_469);
      i_465 = t_470;
      if (sourceText_464.length > i_465) {
        t_471 = stringGet_279(sourceText_464, i_465);
        t_482 = t_471 !== 125;
      } else {
        t_482 = false;
      }
      if (t_482) {
        while (true) {
          const keyBuffer_490 = [""];
          const afterKey_491 = parseJsonStringTo_422(sourceText_464, i_465, keyBuffer_490, out_466);
          if (!(afterKey_491 >= 0)) {
            return_467 = -1;
            break fn_489;
          }
          t_472 = keyBuffer_490[0];
          out_466.objectKey(t_472);
          try {
            t_483 = requireStringIndex_492(afterKey_491);
            t_484 = t_483;
          } catch {
            t_484 = panic_217();
          }
          t_473 = skipJsonSpaces_277(sourceText_464, t_484);
          i_465 = t_473;
          if (sourceText_464.length > i_465) {
            t_474 = stringGet_279(sourceText_464, i_465);
            t_485 = t_474 === 58;
          } else {
            t_485 = false;
          }
          if (t_485) {
            t_475 = stringNext_387(sourceText_464, i_465);
            i_465 = t_475;
            const afterPropertyValue_493 = parseJsonValue_268(sourceText_464, i_465, out_466);
            if (!(afterPropertyValue_493 >= 0)) {
              return_467 = -1;
              break fn_489;
            }
            t_486 = requireStringIndex_492(afterPropertyValue_493);
            i_465 = t_486;
          } else {
            expectedTokenError_278(sourceText_464, i_465, out_466, "':'");
            return_467 = -1;
            break fn_489;
          }
          t_476 = skipJsonSpaces_277(sourceText_464, i_465);
          i_465 = t_476;
          if (sourceText_464.length > i_465) {
            t_477 = stringGet_279(sourceText_464, i_465);
            t_487 = t_477 === 44;
          } else {
            t_487 = false;
          }
          if (t_487) {
            t_478 = stringNext_387(sourceText_464, i_465);
            t_479 = skipJsonSpaces_277(sourceText_464, t_478);
            i_465 = t_479;
          } else {
            break;
          }
        }
      }
      if (sourceText_464.length > i_465) {
        t_480 = stringGet_279(sourceText_464, i_465);
        t_488 = t_480 === 125;
      } else {
        t_488 = false;
      }
      if (t_488) {
        out_466.endObject();
        return_467 = stringNext_387(sourceText_464, i_465);
      } else {
        expectedTokenError_278(sourceText_464, i_465, out_466, "'}'");
        return_467 = -1;
      }
    } catch {
      return_467 = panic_217();
    }
  }
  return return_467;
}
/**
 * @param {string} sourceText_494
 * @param {globalThis.number} i_495
 * @param {JsonProducer} out_496
 * @returns {globalThis.number}
 */
function parseJsonArray_281(sourceText_494, i_495, out_496) {
  let return_497;
  let t_498;
  let t_499;
  let t_500;
  let t_501;
  let t_502;
  let t_503;
  let t_504;
  let t_505;
  let t_506;
  let t_507;
  let t_508;
  let t_509;
  let t_510;
  let t_511;
  fn_512: {
    try {
      if (!(sourceText_494.length > i_495)) {
        t_507 = true;
      } else {
        t_498 = stringGet_279(sourceText_494, i_495);
        t_507 = t_498 !== 91;
      }
      if (t_507) {
        expectedTokenError_278(sourceText_494, i_495, out_496, "'['");
        return_497 = -1;
        break fn_512;
      }
      out_496.startArray();
      t_499 = stringNext_387(sourceText_494, i_495);
      t_500 = skipJsonSpaces_277(sourceText_494, t_499);
      i_495 = t_500;
      if (sourceText_494.length > i_495) {
        t_501 = stringGet_279(sourceText_494, i_495);
        t_508 = t_501 !== 93;
      } else {
        t_508 = false;
      }
      if (t_508) {
        while (true) {
          const afterElementValue_513 = parseJsonValue_268(sourceText_494, i_495, out_496);
          if (!(afterElementValue_513 >= 0)) {
            return_497 = -1;
            break fn_512;
          }
          t_509 = requireStringIndex_492(afterElementValue_513);
          i_495 = t_509;
          t_502 = skipJsonSpaces_277(sourceText_494, i_495);
          i_495 = t_502;
          if (sourceText_494.length > i_495) {
            t_503 = stringGet_279(sourceText_494, i_495);
            t_510 = t_503 === 44;
          } else {
            t_510 = false;
          }
          if (t_510) {
            t_504 = stringNext_387(sourceText_494, i_495);
            t_505 = skipJsonSpaces_277(sourceText_494, t_504);
            i_495 = t_505;
          } else {
            break;
          }
        }
      }
      if (sourceText_494.length > i_495) {
        t_506 = stringGet_279(sourceText_494, i_495);
        t_511 = t_506 === 93;
      } else {
        t_511 = false;
      }
      if (t_511) {
        out_496.endArray();
        return_497 = stringNext_387(sourceText_494, i_495);
      } else {
        expectedTokenError_278(sourceText_494, i_495, out_496, "']'");
        return_497 = -1;
      }
    } catch {
      return_497 = panic_217();
    }
  }
  return return_497;
}
/**
 * @param {string} sourceText_514
 * @param {globalThis.number} i_515
 * @param {JsonProducer} out_516
 * @returns {globalThis.number}
 */
function parseJsonString_282(sourceText_514, i_515, out_516) {
  let t_517;
  const sb_518 = [""];
  const after_519 = parseJsonStringTo_422(sourceText_514, i_515, sb_518, out_516);
  if (after_519 >= 0) {
    t_517 = sb_518[0];
    out_516.stringValue(t_517);
  }
  return after_519;
}
/**
 * @param {string} string_521
 * @param {globalThis.number} inString_522
 * @param {string} substring_523
 * @returns {globalThis.number}
 */
function afterSubstring_520(string_521, inString_522, substring_523) {
  let return_524;
  let t_525;
  let t_526;
  fn_527: {
    let i_528 = inString_522;
    let j_529 = 0;
    while (true) {
      if (!(substring_523.length > j_529)) {
        break;
      }
      if (!(string_521.length > i_528)) {
        return_524 = -1;
        break fn_527;
      }
      if (stringGet_279(string_521, i_528) !== stringGet_279(substring_523, j_529)) {
        return_524 = -1;
        break fn_527;
      }
      t_525 = stringNext_387(string_521, i_528);
      i_528 = t_525;
      t_526 = stringNext_387(substring_523, j_529);
      j_529 = t_526;
    }
    return_524 = i_528;
  }
  return return_524;
}
/**
 * @param {string} sourceText_530
 * @param {globalThis.number} i_531
 * @param {JsonProducer} out_532
 * @returns {globalThis.number}
 */
function parseJsonBoolean_283(sourceText_530, i_531, out_532) {
  let return_533;
  let t_534;
  fn_535: {
    let ch0_536;
    if (sourceText_530.length > i_531) {
      t_534 = stringGet_279(sourceText_530, i_531);
      ch0_536 = t_534;
    } else {
      ch0_536 = 0;
    }
    const end_537 = sourceText_530.length;
    let keyword_538;
    let n_539;
    if (ch0_536 === 102) {
      keyword_538 = "false";
      n_539 = 5;
    } else if (ch0_536 === 116) {
      keyword_538 = "true";
      n_539 = 4;
    } else {
      keyword_538 = null;
      n_539 = 0;
    }
    if (!(keyword_538 == null)) {
      const keyword_540 = keyword_538;
      if (stringHasAtLeast_458(sourceText_530, i_531, end_537, n_539)) {
        const after_541 = afterSubstring_520(sourceText_530, i_531, keyword_540);
        if (after_541 >= 0) {
          return_533 = requireStringIndex_492(after_541);
          out_532.booleanValue(n_539 === 4);
          break fn_535;
        }
      }
    }
    expectedTokenError_278(sourceText_530, i_531, out_532, "`false` or `true`");
    return_533 = -1;
  }
  return return_533;
}
/**
 * @param {string} sourceText_542
 * @param {globalThis.number} i_543
 * @param {JsonProducer} out_544
 * @returns {globalThis.number}
 */
function parseJsonNull_284(sourceText_542, i_543, out_544) {
  let return_545;
  fn_546: {
    const after_547 = afterSubstring_520(sourceText_542, i_543, "null");
    if (after_547 >= 0) {
      return_545 = requireStringIndex_492(after_547);
      out_544.nullValue();
      break fn_546;
    }
    expectedTokenError_278(sourceText_542, i_543, out_544, "`null`");
    return_545 = -1;
  }
  return return_545;
}
/**
 * @param {string} sourceText_548
 * @param {globalThis.number} i_549
 * @param {JsonProducer} out_550
 * @returns {globalThis.number}
 */
function parseJsonNumber_285(sourceText_548, i_549, out_550) {
  let return_551;
  let t_552;
  let t_553;
  let t_554;
  let t_555;
  let t_556;
  let t_557;
  let t_558;
  let t_559;
  let t_560;
  let t_561;
  let t_562;
  let t_563;
  let t_564;
  let t_565;
  let t_566;
  let t_567;
  let t_568;
  let t_569;
  let t_570;
  let t_571;
  let t_572;
  let t_573;
  let t_574;
  let t_575;
  let t_576;
  let t_577;
  let t_578;
  let t_579;
  let t_580;
  let t_581;
  let t_582;
  let t_583;
  let t_584;
  let t_585;
  let t_586;
  let t_587;
  let t_588;
  let t_589;
  let t_590;
  fn_591: {
    let isNegative_592 = false;
    const startOfNumber_593 = i_549;
    if (sourceText_548.length > i_549) {
      t_552 = stringGet_279(sourceText_548, i_549);
      t_570 = t_552 === 45;
    } else {
      t_570 = false;
    }
    if (t_570) {
      isNegative_592 = true;
      t_553 = stringNext_387(sourceText_548, i_549);
      i_549 = t_553;
    }
    let digit0_594;
    if (sourceText_548.length > i_549) {
      t_554 = stringGet_279(sourceText_548, i_549);
      digit0_594 = t_554;
    } else {
      digit0_594 = -1;
    }
    if (digit0_594 < 48) {
      t_571 = true;
    } else {
      t_571 = 57 < digit0_594;
    }
    if (t_571) {
      let error_595;
      if (! isNegative_592) {
        t_572 = digit0_594 !== 46;
      } else {
        t_572 = false;
      }
      if (t_572) {
        error_595 = "JSON value";
      } else {
        error_595 = "digit";
      }
      expectedTokenError_278(sourceText_548, i_549, out_550, error_595);
      return_551 = -1;
      break fn_591;
    }
    t_555 = stringNext_387(sourceText_548, i_549);
    i_549 = t_555;
    let nDigits_596 = 1;
    t_556 = digit0_594 - 48 | 0;
    let tentativeFloat64_597 = t_556;
    t_557 = BigInt(digit0_594 - 48 | 0);
    let tentativeInt64_598 = t_557;
    let overflowInt64_599 = false;
    if (48 !== digit0_594) {
      while (true) {
        if (!(sourceText_548.length > i_549)) {
          break;
        }
        const possibleDigit_600 = stringGet_279(sourceText_548, i_549);
        if (48 <= possibleDigit_600) {
          t_573 = possibleDigit_600 <= 57;
        } else {
          t_573 = false;
        }
        if (t_573) {
          t_558 = stringNext_387(sourceText_548, i_549);
          i_549 = t_558;
          nDigits_596 = nDigits_596 + 1 | 0;
          const nextDigit_601 = possibleDigit_600 - 48 | 0;
          t_574 = tentativeFloat64_597 * 10.0;
          t_559 = nextDigit_601;
          tentativeFloat64_597 = t_574 + t_559;
          const oldInt64_602 = tentativeInt64_598;
          t_575 = clampInt64__603(tentativeInt64_598 * BigInt("10"));
          t_560 = BigInt(nextDigit_601);
          tentativeInt64_598 = clampInt64__603(t_575 + t_560);
          if (tentativeInt64_598 < oldInt64_602) {
            if (clampInt64__603(BigInt("-9223372036854775808") - oldInt64_602) === clampInt64__603(- BigInt(nextDigit_601))) {
              if (isNegative_592) {
                t_576 = oldInt64_602 > BigInt("0");
              } else {
                t_576 = false;
              }
              t_577 = t_576;
            } else {
              t_577 = false;
            }
            if (! t_577) {
              overflowInt64_599 = true;
            }
          }
        } else {
          break;
        }
      }
    }
    let nDigitsAfterPoint_604 = 0;
    if (sourceText_548.length > i_549) {
      t_561 = stringGet_279(sourceText_548, i_549);
      t_578 = 46 === t_561;
    } else {
      t_578 = false;
    }
    if (t_578) {
      t_562 = stringNext_387(sourceText_548, i_549);
      i_549 = t_562;
      const afterPoint_605 = i_549;
      while (true) {
        if (!(sourceText_548.length > i_549)) {
          break;
        }
        const possibleDigit_606 = stringGet_279(sourceText_548, i_549);
        if (48 <= possibleDigit_606) {
          t_579 = possibleDigit_606 <= 57;
        } else {
          t_579 = false;
        }
        if (t_579) {
          t_563 = stringNext_387(sourceText_548, i_549);
          i_549 = t_563;
          nDigits_596 = nDigits_596 + 1 | 0;
          nDigitsAfterPoint_604 = nDigitsAfterPoint_604 + 1 | 0;
          t_580 = tentativeFloat64_597 * 10.0;
          t_564 = possibleDigit_606 - 48 | 0;
          tentativeFloat64_597 = t_580 + t_564;
        } else {
          break;
        }
      }
      if (i_549 === afterPoint_605) {
        expectedTokenError_278(sourceText_548, i_549, out_550, "digit");
        return_551 = -1;
        break fn_591;
      }
    }
    let nExponentDigits_607 = 0;
    if (sourceText_548.length > i_549) {
      t_565 = stringGet_279(sourceText_548, i_549);
      t_581 = 101 ===(t_565 | 32);
    } else {
      t_581 = false;
    }
    if (t_581) {
      t_566 = stringNext_387(sourceText_548, i_549);
      i_549 = t_566;
      if (!(sourceText_548.length > i_549)) {
        expectedTokenError_278(sourceText_548, i_549, out_550, "sign or digit");
        return_551 = -1;
        break fn_591;
      }
      const afterE_608 = stringGet_279(sourceText_548, i_549);
      if (afterE_608 === 43) {
        t_582 = true;
      } else {
        t_582 = afterE_608 === 45;
      }
      if (t_582) {
        t_567 = stringNext_387(sourceText_548, i_549);
        i_549 = t_567;
      }
      while (true) {
        if (!(sourceText_548.length > i_549)) {
          break;
        }
        const possibleDigit_609 = stringGet_279(sourceText_548, i_549);
        if (48 <= possibleDigit_609) {
          t_583 = possibleDigit_609 <= 57;
        } else {
          t_583 = false;
        }
        if (t_583) {
          t_568 = stringNext_387(sourceText_548, i_549);
          i_549 = t_568;
          nExponentDigits_607 = nExponentDigits_607 + 1 | 0;
        } else {
          break;
        }
      }
      if (nExponentDigits_607 === 0) {
        expectedTokenError_278(sourceText_548, i_549, out_550, "exponent digit");
        return_551 = -1;
        break fn_591;
      }
    }
    const afterExponent_610 = i_549;
    if (nExponentDigits_607 === 0) {
      if (nDigitsAfterPoint_604 === 0) {
        t_584 = ! overflowInt64_599;
      } else {
        t_584 = false;
      }
      t_585 = t_584;
    } else {
      t_585 = false;
    }
    if (t_585) {
      let value_611;
      if (isNegative_592) {
        value_611 = clampInt64__603(- tentativeInt64_598);
      } else {
        value_611 = tentativeInt64_598;
      }
      if (BigInt("-2147483648") <= value_611) {
        t_586 = value_611 <= BigInt("2147483647");
      } else {
        t_586 = false;
      }
      if (t_586) {
        t_569 = int64ToInt32Unsafe_612(value_611);
        out_550.int32Value(t_569);
      } else {
        out_550.int64Value(value_611);
      }
      return_551 = i_549;
      break fn_591;
    }
    const numericTokenString_613 = sourceText_548.substring(startOfNumber_593, i_549);
    let doubleValue_614 = NaN;
    if (nExponentDigits_607 !== 0) {
      t_587 = true;
    } else {
      t_587 = nDigitsAfterPoint_604 !== 0;
    }
    if (t_587) {
      try {
        t_588 = stringToFloat64_115(numericTokenString_613);
        doubleValue_614 = t_588;
      } catch {
      }
    }
    if (cmpFloat__615(doubleValue_614, -Infinity) !== 0) {
      if (cmpFloat__615(doubleValue_614, Infinity) !== 0) {
        t_589 = cmpFloat__615(doubleValue_614, NaN) !== 0;
      } else {
        t_589 = false;
      }
      t_590 = t_589;
    } else {
      t_590 = false;
    }
    if (t_590) {
      out_550.float64Value(doubleValue_614);
    } else {
      out_550.numericTokenValue(numericTokenString_613);
    }
    return_551 = i_549;
  }
  return return_551;
}
/**
 * @param {string} sourceText_616
 * @param {JsonProducer} out_617
 */
export function parseJsonToProducer(sourceText_616, out_617) {
  let t_618;
  let t_619;
  let t_620;
  let t_621;
  let t_622;
  let t_623;
  let i_624 = 0;
  const afterValue_625 = parseJsonValue_268(sourceText_616, i_624, out_617);
  if (afterValue_625 >= 0) {
    t_623 = requireStringIndex_492(afterValue_625);
    t_618 = skipJsonSpaces_277(sourceText_616, t_623);
    i_624 = t_618;
    if (sourceText_616.length > i_624) {
      t_619 = out_617.parseErrorReceiver;
      t_622 = !(t_619 == null);
    } else {
      t_622 = false;
    }
    if (t_622) {
      t_620 = sourceText_616.length;
      t_621 = sourceText_616.substring(i_624, t_620);
      storeJsonError_388(out_617, "Extraneous JSON `" + t_621 + "`");
    }
  }
  return;
};
/**
 * @param {string} sourceText_626
 * @returns {JsonSyntaxTree}
 */
export function parseJson(sourceText_626) {
  const p_627 = new JsonSyntaxTreeProducer();
  parseJsonToProducer(sourceText_626, p_627);
  return p_627.toJsonSyntaxTree();
};
/** @returns {JsonAdapter<boolean>} */
export function booleanJsonAdapter() {
  return new BooleanJsonAdapter_293();
};
/** @returns {JsonAdapter<number>} */
export function float64JsonAdapter() {
  return new Float64JsonAdapter_301();
};
/** @returns {JsonAdapter<number>} */
export function int32JsonAdapter() {
  return new Int32JsonAdapter_309();
};
/** @returns {JsonAdapter<bigint>} */
export function int64JsonAdapter() {
  return new Int64JsonAdapter_317();
};
/** @returns {JsonAdapter<string>} */
export function stringJsonAdapter() {
  return new StringJsonAdapter_325();
};
/**
 * @template {unknown} T_629
 * @param {JsonAdapter<T_629>} adapterForT_628
 * @returns {JsonAdapter<Array<T_629>>}
 */
export function listJsonAdapter(adapterForT_628) {
  return new ListJsonAdapter_333(adapterForT_628);
};
