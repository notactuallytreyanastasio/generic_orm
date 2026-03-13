import {
  JsonProducer as JsonProducer_635, JsonSyntaxTree as JsonSyntaxTree_640, InterchangeContext as InterchangeContext_641, JsonAdapter as JsonAdapter_642, JsonString as JsonString_650
} from "./json.js";
import {
  type as type__643, requireInstanceOf as requireInstanceOf__651, modIntInt as modIntInt_658, stringGet as stringGet_669, stringNext as stringNext_670, stringCountBetween as stringCountBetween_672
} from "@temperlang/core";
class DateJsonAdapter_630 extends type__643(JsonAdapter_642) {
  /**
   * @param {globalThis.Date} x_632
   * @param {JsonProducer_635} p_633
   */
  encodeToJson(x_632, p_633) {
    encodeToJson_634(x_632, p_633);
    return;
  }
  /**
   * @param {JsonSyntaxTree_640} t_637
   * @param {InterchangeContext_641} ic_638
   * @returns {globalThis.Date}
   */
  decodeFromJson(t_637, ic_638) {
    return decodeFromJson_639(t_637, ic_638);
  }
  constructor() {
    super ();
    return;
  }
}
// Type `std/temporal/`.Date connected to globalThis.Date
/**
 * @param {globalThis.Date} this_644
 * @param {JsonProducer_635} p_645
 */
function encodeToJson_634(this_644, p_645) {
  let t_646 = this_644.toISOString().split("T")[0];
  p_645.stringValue(t_646);
  return;
}
/**
 * @param {JsonSyntaxTree_640} t_647
 * @param {InterchangeContext_641} ic_648
 * @returns {globalThis.Date}
 */
function decodeFromJson_639(t_647, ic_648) {
  let t_649;
  t_649 = requireInstanceOf__651(t_647, JsonString_650);
  return new (globalThis.Date)(globalThis.Date.parse(t_649.content));
}
/** @returns {JsonAdapter_642<globalThis.Date>} */
function jsonAdapter_652() {
  return new DateJsonAdapter_630();
}
/** @type {Array<number>} */
const daysInMonth_653 = Object.freeze([0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]);
/**
 * @param {number} year_655
 * @returns {boolean}
 */
function isLeapYear_654(year_655) {
  let return_656;
  let t_657;
  if (modIntInt_658(year_655, 4) === 0) {
    if (modIntInt_658(year_655, 100) !== 0) {
      return_656 = true;
    } else {
      t_657 = modIntInt_658(year_655, 400);
      return_656 = t_657 === 0;
    }
  } else {
    return_656 = false;
  }
  return return_656;
}
/**
 * @param {number} minWidth_660
 * @param {number} num_661
 * @param {globalThis.Array<string>} sb_662
 */
function padTo_659(minWidth_660, num_661, sb_662) {
  let t_663;
  let t_664;
  let t_665;
  const decimal_666 = num_661.toString(10);
  let decimalIndex_667 = 0;
  const decimalEnd_668 = decimal_666.length;
  if (decimalIndex_667 < decimalEnd_668) {
    t_663 = stringGet_669(decimal_666, decimalIndex_667);
    t_665 = t_663 === 45;
  } else {
    t_665 = false;
  }
  if (t_665) {
    sb_662[0] += "-";
    t_664 = stringNext_670(decimal_666, decimalIndex_667);
    decimalIndex_667 = t_664;
  }
  let t_671 = stringCountBetween_672(decimal_666, decimalIndex_667, decimalEnd_668);
  let nNeeded_673 = minWidth_660 - t_671 | 0;
  while (nNeeded_673 > 0) {
    sb_662[0] += "0";
    nNeeded_673 = nNeeded_673 - 1 | 0;
  }
  sb_662[0] += decimal_666.substring(decimalIndex_667, decimalEnd_668);
  return;
}
/** @type {Array<number>} */
const dayOfWeekLookupTableLeapy_674 = Object.freeze([0, 0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6]);
/** @type {Array<number>} */
const dayOfWeekLookupTableNotLeapy_675 = Object.freeze([0, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5]);
