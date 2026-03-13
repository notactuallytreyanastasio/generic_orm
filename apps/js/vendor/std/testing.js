import {
  strict as strict__687
} from "assert";
import {
  type as type__701, listBuilderAdd as listBuilderAdd_681, listBuilderToList as listBuilderToList_691, listedJoin as listedJoin_699, pairConstructor as pairConstructor_716, listedMap as listedMap_717, stringGet as stringGet_736, stringNext as stringNext_738, listedReduceFrom as listedReduceFrom_750, listedGet as listedGet_754
} from "@temperlang/core";
export class Test extends type__701() {
  /**
   * @param {boolean} success_677
   * @param {() => string} message_678
   */
  assert(success_677, message_678) {
    let t_679;
    if (! success_677) {
      this.#_passing_680 = false;
      t_679 = message_678();
      listBuilderAdd_681(this.#_messages_682, t_679);
    }
    return;
  }
  /**
   * @param {boolean} success_684
   * @param {() => string} message_685
   * @returns {void}
   */
  assertHard(success_684, message_685) {
    this.assert(success_684, message_685);
    if (! success_684) {
      this.#_failedOnAssert_686 = true;
      strict__687.fail(this.messagesCombined());
    }
    return;
  }
  /** @returns {void} */
  softFailToHard() {
    if (this.hasUnhandledFail) {
      this.#_failedOnAssert_686 = true;
      strict__687.fail(this.messagesCombined());
    }
    return;
  }
  /** @returns {boolean} */
  get passing() {
    return this.#_passing_680;
  }
  /** @returns {Array<string>} */
  messages() {
    return listBuilderToList_691(this.#_messages_682);
  }
  /** @returns {boolean} */
  get failedOnAssert() {
    return this.#_failedOnAssert_686;
  }
  /** @returns {boolean} */
  get hasUnhandledFail() {
    let t_694;
    if (this.#_failedOnAssert_686) {
      t_694 = true;
    } else {
      t_694 = this.#_passing_680;
    }
    return ! t_694;
  }
  /** @returns {string | null} */
  messagesCombined() {
    let return_696;
    if (! this.#_messages_682.length) {
      return_696 = null;
    } else {
      function fn_697(it_698) {
        return it_698;
      }
      return_696 = listedJoin_699(this.#_messages_682, ", ", fn_697);
    }
    return return_696;
  }
  /** @type {boolean} */
  #_failedOnAssert_686;
  /** @type {boolean} */
  #_passing_680;
  /** @type {Array<string>} */
  #_messages_682;
  constructor() {
    super ();
    this.#_failedOnAssert_686 = false;
    this.#_passing_680 = true;
    let t_700 = [];
    this.#_messages_682 = t_700;
    return;
  }
};
/**
 * @param {Array<Pair_718<string, (arg0: Test) => void>>} testCases_702
 * @returns {Array<Pair_718<string, Array<string>>>}
 */
export function processTestCases(testCases_702) {
  function fn_703(testCase_704) {
    let t_705;
    let t_706;
    let t_707;
    let t_708;
    const key_709 = testCase_704.key;
    const fun_710 = testCase_704.value;
    const test_711 = new Test();
    let hadBubble_712 = false;
    try {
      fun_710(test_711);
    } catch {
      hadBubble_712 = true;
    }
    const messages_713 = test_711.messages();
    let failures_714;
    if (test_711.passing) {
      t_707 = ! hadBubble_712;
    } else {
      t_707 = false;
    }
    if (t_707) {
      failures_714 = Object.freeze([]);
    } else {
      if (hadBubble_712) {
        t_705 = test_711.failedOnAssert;
        t_708 = ! t_705;
      } else {
        t_708 = false;
      }
      if (t_708) {
        const allMessages_715 = messages_713.slice();
        listBuilderAdd_681(allMessages_715, "Bubble");
        t_706 = listBuilderToList_691(allMessages_715);
        failures_714 = t_706;
      } else {
        failures_714 = messages_713;
      }
    }
    return pairConstructor_716(key_709, failures_714);
  }
  return listedMap_717(testCases_702, fn_703);
};
/**
 * @param {string} s_720
 * @returns {string}
 */
function escapeXml_719(s_720) {
  let return_721;
  let t_722;
  let t_723;
  let t_724;
  let t_725;
  let t_726;
  let t_727;
  let t_728;
  let t_729;
  const sb_730 = [""];
  const end_731 = s_720.length;
  let emitted_732 = 0;
  let i_733 = 0;
  while (i_733 < end_731) {
    continue_734: {
      const c_735 = stringGet_736(s_720, i_733);
      if (c_735 === 38) {
        t_729 = "&amp;";
      } else if (c_735 === 60) {
        t_729 = "&lt;";
      } else if (c_735 === 62) {
        t_729 = "&gt;";
      } else if (c_735 === 39) {
        t_729 = "&#39;";
      } else if (c_735 === 34) {
        t_729 = "&#34;";
      } else {
        if (c_735 === 10) {
          t_725 = true;
        } else {
          if (c_735 === 13) {
            t_724 = true;
          } else {
            t_724 = c_735 === 9;
          }
          t_725 = t_724;
        }
        if (t_725) {
          break continue_734;
        } else {
          if (c_735 < 32) {
            t_727 = true;
          } else {
            if (c_735 === 65534) {
              t_726 = true;
            } else {
              t_726 = c_735 === 65535;
            }
            t_727 = t_726;
          }
          if (t_727) {
            t_728 = "[0x" + c_735.toString(16) + "]";
          } else {
            break continue_734;
          }
          t_729 = t_728;
        }
      }
      const esc_737 = t_729;
      sb_730[0] += s_720.substring(emitted_732, i_733);
      sb_730[0] += esc_737;
      t_722 = stringNext_738(s_720, i_733);
      emitted_732 = t_722;
    }
    t_723 = stringNext_738(s_720, i_733);
    i_733 = t_723;
  }
  if (emitted_732 === 0) {
    return_721 = s_720;
  } else {
    sb_730[0] += s_720.substring(emitted_732, end_731);
    return_721 = sb_730[0];
  }
  return return_721;
}
/**
 * @param {Array<Pair_718<string, Array<string>>>} testResults_739
 * @param {(arg0: string) => void} writeLine_740
 */
export function reportTestResults(testResults_739, writeLine_740) {
  let t_741;
  let t_742;
  let t_743;
  writeLine_740("<testsuites>");
  const total_744 = testResults_739.length.toString();
  function fn_745(fails_746, testResult_747) {
    let t_748;
    if (! testResult_747.value.length) {
      t_748 = 0;
    } else {
      t_748 = 1;
    }
    return fails_746 + t_748 | 0;
  }
  const fails_749 = listedReduceFrom_750(testResults_739, 0, fn_745).toString();
  const totals_751 = "tests='" + total_744 + "' failures='" + fails_749 + "'";
  writeLine_740("  <testsuite name='suite' " + totals_751 + " time='0.0'>");
  let i_752 = 0;
  while (true) {
    t_741 = testResults_739.length;
    if (!(i_752 < t_741)) {
      break;
    }
    const testResult_753 = listedGet_754(testResults_739, i_752);
    const failureMessages_755 = testResult_753.value;
    t_742 = testResult_753.key;
    const name_756 = escapeXml_719(t_742);
    const basics_757 = "name='" + name_756 + "' classname='" + name_756 + "' time='0.0'";
    if (! failureMessages_755.length) {
      writeLine_740("    <testcase " + basics_757 + " />");
    } else {
      writeLine_740("    <testcase " + basics_757 + ">");
      function fn_758(it_759) {
        return it_759;
      }
      t_743 = listedJoin_699(failureMessages_755, ", ", fn_758);
      const message_760 = escapeXml_719(t_743);
      writeLine_740("      <failure message='" + message_760 + "' />");
      writeLine_740("    <\/testcase>");
    }
    i_752 = i_752 + 1 | 0;
  }
  writeLine_740("  <\/testsuite>");
  writeLine_740("<\/testsuites>");
  return;
};
/**
 * @param {Array<Pair_718<string, (arg0: Test) => void>>} testCases_761
 * @returns {string}
 */
export function runTestCases(testCases_761) {
  const report_762 = [""];
  let t_763 = processTestCases(testCases_761);
  function fn_764(line_765) {
    report_762[0] += line_765;
    report_762[0] += "\n";
    return;
  }
  reportTestResults(t_763, fn_764);
  return report_762[0];
};
/**
 * @param {(arg0: Test) => void} testFun_766
 * @returns {void}
 */
export function runTest(testFun_766) {
  const test_767 = new Test();
  try {
    testFun_766(test_767);
  } catch {
    function fn_768() {
      return "bubble during test running";
    }
    test_767.assert(false, fn_768);
  }
  test_767.softFailToHard();
  return;
};
