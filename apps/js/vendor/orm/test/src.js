import {
  BoolField, FieldDef, FloatField, IntField, SafeIdentifier, SqlBuilder, SqlInt32, SqlString, StringField, TableDef, changeset, from, safeIdentifier
} from "../src.js";
import {
  Test as Test_448
} from "@temperlang/std/testing";
import {
  panic as panic_445, mapConstructor as mapConstructor_426, pairConstructor as pairConstructor_450, listedGet as listedGet_179
} from "@temperlang/core";
/**
 * @param {string} name_442
 * @returns {SafeIdentifier}
 */
function csid_441(name_442) {
  let return_443;
  let t_444;
  try {
    t_444 = safeIdentifier(name_442);
    return_443 = t_444;
  } catch {
    return_443 = panic_445();
  }
  return return_443;
}
/** @returns {TableDef} */
function userTable_446() {
  return new TableDef(csid_441("users"), Object.freeze([new FieldDef(csid_441("name"), new StringField(), false), new FieldDef(csid_441("email"), new StringField(), false), new FieldDef(csid_441("age"), new IntField(), true), new FieldDef(csid_441("score"), new FloatField(), true), new FieldDef(csid_441("active"), new BoolField(), true)]));
}
it("cast whitelists allowed fields", function () {
    const test_447 = new Test_448();
    try {
      const params_449 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Alice"), pairConstructor_450("email", "alice@example.com"), pairConstructor_450("admin", "true")]));
      let t_451 = userTable_446();
      let t_452 = csid_441("name");
      let t_453 = csid_441("email");
      const cs_454 = changeset(t_451, params_449).cast(Object.freeze([t_452, t_453]));
      let t_455 = cs_454.changes.has("name");
      function fn_456() {
        return "name should be in changes";
      }
      test_447.assert(t_455, fn_456);
      let t_457 = cs_454.changes.has("email");
      function fn_458() {
        return "email should be in changes";
      }
      test_447.assert(t_457, fn_458);
      let t_459 = ! cs_454.changes.has("admin");
      function fn_460() {
        return "admin must be dropped (not in whitelist)";
      }
      test_447.assert(t_459, fn_460);
      let t_461 = cs_454.isValid;
      function fn_462() {
        return "should still be valid";
      }
      test_447.assert(t_461, fn_462);
      return;
    } finally {
      test_447.softFailToHard();
    }
});
it("cast is replacing not additive — second call resets whitelist", function () {
    const test_463 = new Test_448();
    try {
      const params_464 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Alice"), pairConstructor_450("email", "alice@example.com")]));
      let t_465 = userTable_446();
      let t_466 = csid_441("name");
      const cs_467 = changeset(t_465, params_464).cast(Object.freeze([t_466])).cast(Object.freeze([csid_441("email")]));
      let t_468 = ! cs_467.changes.has("name");
      function fn_469() {
        return "name must be excluded by second cast";
      }
      test_463.assert(t_468, fn_469);
      let t_470 = cs_467.changes.has("email");
      function fn_471() {
        return "email should be present";
      }
      test_463.assert(t_470, fn_471);
      return;
    } finally {
      test_463.softFailToHard();
    }
});
it("cast ignores empty string values", function () {
    const test_472 = new Test_448();
    try {
      const params_473 = mapConstructor_426(Object.freeze([pairConstructor_450("name", ""), pairConstructor_450("email", "bob@example.com")]));
      let t_474 = userTable_446();
      let t_475 = csid_441("name");
      let t_476 = csid_441("email");
      const cs_477 = changeset(t_474, params_473).cast(Object.freeze([t_475, t_476]));
      let t_478 = ! cs_477.changes.has("name");
      function fn_479() {
        return "empty name should not be in changes";
      }
      test_472.assert(t_478, fn_479);
      let t_480 = cs_477.changes.has("email");
      function fn_481() {
        return "email should be in changes";
      }
      test_472.assert(t_480, fn_481);
      return;
    } finally {
      test_472.softFailToHard();
    }
});
it("validateRequired passes when field present", function () {
    const test_482 = new Test_448();
    try {
      const params_483 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Alice")]));
      let t_484 = userTable_446();
      let t_485 = csid_441("name");
      const cs_486 = changeset(t_484, params_483).cast(Object.freeze([t_485])).validateRequired(Object.freeze([csid_441("name")]));
      let t_487 = cs_486.isValid;
      function fn_488() {
        return "should be valid";
      }
      test_482.assert(t_487, fn_488);
      let t_489 = cs_486.errors.length === 0;
      function fn_490() {
        return "no errors expected";
      }
      test_482.assert(t_489, fn_490);
      return;
    } finally {
      test_482.softFailToHard();
    }
});
it("validateRequired fails when field missing", function () {
    const test_491 = new Test_448();
    try {
      const params_492 = mapConstructor_426(Object.freeze([]));
      let t_493 = userTable_446();
      let t_494 = csid_441("name");
      const cs_495 = changeset(t_493, params_492).cast(Object.freeze([t_494])).validateRequired(Object.freeze([csid_441("name")]));
      let t_496 = ! cs_495.isValid;
      function fn_497() {
        return "should be invalid";
      }
      test_491.assert(t_496, fn_497);
      let t_498 = cs_495.errors.length === 1;
      function fn_499() {
        return "should have one error";
      }
      test_491.assert(t_498, fn_499);
      let t_500 = listedGet_179(cs_495.errors, 0).field === "name";
      function fn_501() {
        return "error should name the field";
      }
      test_491.assert(t_500, fn_501);
      return;
    } finally {
      test_491.softFailToHard();
    }
});
it("validateLength passes within range", function () {
    const test_502 = new Test_448();
    try {
      const params_503 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Alice")]));
      let t_504 = userTable_446();
      let t_505 = csid_441("name");
      const cs_506 = changeset(t_504, params_503).cast(Object.freeze([t_505])).validateLength(csid_441("name"), 2, 50);
      let t_507 = cs_506.isValid;
      function fn_508() {
        return "should be valid";
      }
      test_502.assert(t_507, fn_508);
      return;
    } finally {
      test_502.softFailToHard();
    }
});
it("validateLength fails when too short", function () {
    const test_509 = new Test_448();
    try {
      const params_510 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "A")]));
      let t_511 = userTable_446();
      let t_512 = csid_441("name");
      const cs_513 = changeset(t_511, params_510).cast(Object.freeze([t_512])).validateLength(csid_441("name"), 2, 50);
      let t_514 = ! cs_513.isValid;
      function fn_515() {
        return "should be invalid";
      }
      test_509.assert(t_514, fn_515);
      return;
    } finally {
      test_509.softFailToHard();
    }
});
it("validateLength fails when too long", function () {
    const test_516 = new Test_448();
    try {
      const params_517 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "ABCDEFGHIJKLMNOPQRSTUVWXYZ")]));
      let t_518 = userTable_446();
      let t_519 = csid_441("name");
      const cs_520 = changeset(t_518, params_517).cast(Object.freeze([t_519])).validateLength(csid_441("name"), 2, 10);
      let t_521 = ! cs_520.isValid;
      function fn_522() {
        return "should be invalid";
      }
      test_516.assert(t_521, fn_522);
      return;
    } finally {
      test_516.softFailToHard();
    }
});
it("validateInt passes for valid integer", function () {
    const test_523 = new Test_448();
    try {
      const params_524 = mapConstructor_426(Object.freeze([pairConstructor_450("age", "30")]));
      let t_525 = userTable_446();
      let t_526 = csid_441("age");
      const cs_527 = changeset(t_525, params_524).cast(Object.freeze([t_526])).validateInt(csid_441("age"));
      let t_528 = cs_527.isValid;
      function fn_529() {
        return "should be valid";
      }
      test_523.assert(t_528, fn_529);
      return;
    } finally {
      test_523.softFailToHard();
    }
});
it("validateInt fails for non-integer", function () {
    const test_530 = new Test_448();
    try {
      const params_531 = mapConstructor_426(Object.freeze([pairConstructor_450("age", "not-a-number")]));
      let t_532 = userTable_446();
      let t_533 = csid_441("age");
      const cs_534 = changeset(t_532, params_531).cast(Object.freeze([t_533])).validateInt(csid_441("age"));
      let t_535 = ! cs_534.isValid;
      function fn_536() {
        return "should be invalid";
      }
      test_530.assert(t_535, fn_536);
      return;
    } finally {
      test_530.softFailToHard();
    }
});
it("validateFloat passes for valid float", function () {
    const test_537 = new Test_448();
    try {
      const params_538 = mapConstructor_426(Object.freeze([pairConstructor_450("score", "9.5")]));
      let t_539 = userTable_446();
      let t_540 = csid_441("score");
      const cs_541 = changeset(t_539, params_538).cast(Object.freeze([t_540])).validateFloat(csid_441("score"));
      let t_542 = cs_541.isValid;
      function fn_543() {
        return "should be valid";
      }
      test_537.assert(t_542, fn_543);
      return;
    } finally {
      test_537.softFailToHard();
    }
});
it("validateInt64 passes for valid 64-bit integer", function () {
    const test_544 = new Test_448();
    try {
      const params_545 = mapConstructor_426(Object.freeze([pairConstructor_450("age", "9999999999")]));
      let t_546 = userTable_446();
      let t_547 = csid_441("age");
      const cs_548 = changeset(t_546, params_545).cast(Object.freeze([t_547])).validateInt64(csid_441("age"));
      let t_549 = cs_548.isValid;
      function fn_550() {
        return "should be valid";
      }
      test_544.assert(t_549, fn_550);
      return;
    } finally {
      test_544.softFailToHard();
    }
});
it("validateInt64 fails for non-integer", function () {
    const test_551 = new Test_448();
    try {
      const params_552 = mapConstructor_426(Object.freeze([pairConstructor_450("age", "not-a-number")]));
      let t_553 = userTable_446();
      let t_554 = csid_441("age");
      const cs_555 = changeset(t_553, params_552).cast(Object.freeze([t_554])).validateInt64(csid_441("age"));
      let t_556 = ! cs_555.isValid;
      function fn_557() {
        return "should be invalid";
      }
      test_551.assert(t_556, fn_557);
      return;
    } finally {
      test_551.softFailToHard();
    }
});
it("validateBool accepts true/1/yes/on", function () {
    const test_558 = new Test_448();
    try {
      function fn_559(v_560) {
        const params_561 = mapConstructor_426(Object.freeze([pairConstructor_450("active", v_560)]));
        let t_562 = userTable_446();
        let t_563 = csid_441("active");
        const cs_564 = changeset(t_562, params_561).cast(Object.freeze([t_563])).validateBool(csid_441("active"));
        let t_565 = cs_564.isValid;
        function fn_566() {
          return "should accept: " + v_560;
        }
        test_558.assert(t_565, fn_566);
        return;
      }
      Object.freeze(["true", "1", "yes", "on"]).forEach(fn_559);
      return;
    } finally {
      test_558.softFailToHard();
    }
});
it("validateBool accepts false/0/no/off", function () {
    const test_567 = new Test_448();
    try {
      function fn_568(v_569) {
        const params_570 = mapConstructor_426(Object.freeze([pairConstructor_450("active", v_569)]));
        let t_571 = userTable_446();
        let t_572 = csid_441("active");
        const cs_573 = changeset(t_571, params_570).cast(Object.freeze([t_572])).validateBool(csid_441("active"));
        let t_574 = cs_573.isValid;
        function fn_575() {
          return "should accept: " + v_569;
        }
        test_567.assert(t_574, fn_575);
        return;
      }
      Object.freeze(["false", "0", "no", "off"]).forEach(fn_568);
      return;
    } finally {
      test_567.softFailToHard();
    }
});
it("validateBool rejects ambiguous values", function () {
    const test_576 = new Test_448();
    try {
      function fn_577(v_578) {
        const params_579 = mapConstructor_426(Object.freeze([pairConstructor_450("active", v_578)]));
        let t_580 = userTable_446();
        let t_581 = csid_441("active");
        const cs_582 = changeset(t_580, params_579).cast(Object.freeze([t_581])).validateBool(csid_441("active"));
        let t_583 = ! cs_582.isValid;
        function fn_584() {
          return "should reject ambiguous: " + v_578;
        }
        test_576.assert(t_583, fn_584);
        return;
      }
      Object.freeze(["TRUE", "Yes", "maybe", "2", "enabled"]).forEach(fn_577);
      return;
    } finally {
      test_576.softFailToHard();
    }
});
it("toInsertSql escapes Bobby Tables", function () {
    const test_585 = new Test_448();
    try {
      let t_586;
      const params_587 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Robert'); DROP TABLE users;--"), pairConstructor_450("email", "bobby@evil.com")]));
      let t_588 = userTable_446();
      let t_589 = csid_441("name");
      let t_590 = csid_441("email");
      const cs_591 = changeset(t_588, params_587).cast(Object.freeze([t_589, t_590])).validateRequired(Object.freeze([csid_441("name"), csid_441("email")]));
      let sqlFrag_592;
      try {
        t_586 = cs_591.toInsertSql();
        sqlFrag_592 = t_586;
      } catch {
        sqlFrag_592 = panic_445();
      }
      const s_593 = sqlFrag_592.toString();
      let t_594 = s_593.indexOf("''") >= 0;
      function fn_595() {
        return "single quote must be doubled: " + s_593;
      }
      test_585.assert(t_594, fn_595);
      return;
    } finally {
      test_585.softFailToHard();
    }
});
it("toInsertSql produces correct SQL for string field", function () {
    const test_596 = new Test_448();
    try {
      let t_597;
      const params_598 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Alice"), pairConstructor_450("email", "a@example.com")]));
      let t_599 = userTable_446();
      let t_600 = csid_441("name");
      let t_601 = csid_441("email");
      const cs_602 = changeset(t_599, params_598).cast(Object.freeze([t_600, t_601])).validateRequired(Object.freeze([csid_441("name"), csid_441("email")]));
      let sqlFrag_603;
      try {
        t_597 = cs_602.toInsertSql();
        sqlFrag_603 = t_597;
      } catch {
        sqlFrag_603 = panic_445();
      }
      const s_604 = sqlFrag_603.toString();
      let t_605 = s_604.indexOf("INSERT INTO users") >= 0;
      function fn_606() {
        return "has INSERT INTO: " + s_604;
      }
      test_596.assert(t_605, fn_606);
      let t_607 = s_604.indexOf("'Alice'") >= 0;
      function fn_608() {
        return "has quoted name: " + s_604;
      }
      test_596.assert(t_607, fn_608);
      return;
    } finally {
      test_596.softFailToHard();
    }
});
it("toInsertSql produces correct SQL for int field", function () {
    const test_609 = new Test_448();
    try {
      let t_610;
      const params_611 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Bob"), pairConstructor_450("email", "b@example.com"), pairConstructor_450("age", "25")]));
      let t_612 = userTable_446();
      let t_613 = csid_441("name");
      let t_614 = csid_441("email");
      let t_615 = csid_441("age");
      const cs_616 = changeset(t_612, params_611).cast(Object.freeze([t_613, t_614, t_615])).validateRequired(Object.freeze([csid_441("name"), csid_441("email")]));
      let sqlFrag_617;
      try {
        t_610 = cs_616.toInsertSql();
        sqlFrag_617 = t_610;
      } catch {
        sqlFrag_617 = panic_445();
      }
      const s_618 = sqlFrag_617.toString();
      let t_619 = s_618.indexOf("25") >= 0;
      function fn_620() {
        return "age rendered unquoted: " + s_618;
      }
      test_609.assert(t_619, fn_620);
      return;
    } finally {
      test_609.softFailToHard();
    }
});
it("toInsertSql bubbles on invalid changeset", function () {
    const test_621 = new Test_448();
    try {
      const params_622 = mapConstructor_426(Object.freeze([]));
      let t_623 = userTable_446();
      let t_624 = csid_441("name");
      const cs_625 = changeset(t_623, params_622).cast(Object.freeze([t_624])).validateRequired(Object.freeze([csid_441("name")]));
      let didBubble_626;
      try {
        cs_625.toInsertSql();
        didBubble_626 = false;
      } catch {
        didBubble_626 = true;
      }
      function fn_627() {
        return "invalid changeset should bubble";
      }
      test_621.assert(didBubble_626, fn_627);
      return;
    } finally {
      test_621.softFailToHard();
    }
});
it("toInsertSql enforces non-nullable fields independently of isValid", function () {
    const test_628 = new Test_448();
    try {
      const strictTable_629 = new TableDef(csid_441("posts"), Object.freeze([new FieldDef(csid_441("title"), new StringField(), false), new FieldDef(csid_441("body"), new StringField(), true)]));
      const params_630 = mapConstructor_426(Object.freeze([pairConstructor_450("body", "hello")]));
      let t_631 = csid_441("body");
      const cs_632 = changeset(strictTable_629, params_630).cast(Object.freeze([t_631]));
      let t_633 = cs_632.isValid;
      function fn_634() {
        return "changeset should appear valid (no explicit validation run)";
      }
      test_628.assert(t_633, fn_634);
      let didBubble_635;
      try {
        cs_632.toInsertSql();
        didBubble_635 = false;
      } catch {
        didBubble_635 = true;
      }
      function fn_636() {
        return "toInsertSql should enforce nullable regardless of isValid";
      }
      test_628.assert(didBubble_635, fn_636);
      return;
    } finally {
      test_628.softFailToHard();
    }
});
it("toUpdateSql produces correct SQL", function () {
    const test_637 = new Test_448();
    try {
      let t_638;
      const params_639 = mapConstructor_426(Object.freeze([pairConstructor_450("name", "Bob")]));
      let t_640 = userTable_446();
      let t_641 = csid_441("name");
      const cs_642 = changeset(t_640, params_639).cast(Object.freeze([t_641])).validateRequired(Object.freeze([csid_441("name")]));
      let sqlFrag_643;
      try {
        t_638 = cs_642.toUpdateSql(42);
        sqlFrag_643 = t_638;
      } catch {
        sqlFrag_643 = panic_445();
      }
      const s_644 = sqlFrag_643.toString();
      let t_645 = s_644 === "UPDATE users SET name = 'Bob' WHERE id = 42";
      function fn_646() {
        return "got: " + s_644;
      }
      test_637.assert(t_645, fn_646);
      return;
    } finally {
      test_637.softFailToHard();
    }
});
it("toUpdateSql bubbles on invalid changeset", function () {
    const test_647 = new Test_448();
    try {
      const params_648 = mapConstructor_426(Object.freeze([]));
      let t_649 = userTable_446();
      let t_650 = csid_441("name");
      const cs_651 = changeset(t_649, params_648).cast(Object.freeze([t_650])).validateRequired(Object.freeze([csid_441("name")]));
      let didBubble_652;
      try {
        cs_651.toUpdateSql(1);
        didBubble_652 = false;
      } catch {
        didBubble_652 = true;
      }
      function fn_653() {
        return "invalid changeset should bubble";
      }
      test_647.assert(didBubble_652, fn_653);
      return;
    } finally {
      test_647.softFailToHard();
    }
});
/**
 * @param {string} name_659
 * @returns {SafeIdentifier}
 */
function sid_658(name_659) {
  let return_660;
  let t_661;
  try {
    t_661 = safeIdentifier(name_659);
    return_660 = t_661;
  } catch {
    return_660 = panic_445();
  }
  return return_660;
}
it("bare from produces SELECT *", function () {
    const test_662 = new Test_448();
    try {
      const q_663 = from(sid_658("users"));
      let t_664 = q_663.toSql().toString() === "SELECT * FROM users";
      function fn_665() {
        return "bare query";
      }
      test_662.assert(t_664, fn_665);
      return;
    } finally {
      test_662.softFailToHard();
    }
});
it("select restricts columns", function () {
    const test_666 = new Test_448();
    try {
      let t_667 = sid_658("users");
      let t_668 = sid_658("id");
      let t_669 = sid_658("name");
      const q_670 = from(t_667).select(Object.freeze([t_668, t_669]));
      let t_671 = q_670.toSql().toString() === "SELECT id, name FROM users";
      function fn_672() {
        return "select columns";
      }
      test_666.assert(t_671, fn_672);
      return;
    } finally {
      test_666.softFailToHard();
    }
});
it("where adds condition with int value", function () {
    const test_673 = new Test_448();
    try {
      let t_674 = sid_658("users");
      let t_675 = new SqlBuilder();
      t_675.appendSafe("age > ");
      t_675.appendInt32(18);
      let t_676 = t_675.accumulated;
      const q_677 = from(t_674).where(t_676);
      let t_678 = q_677.toSql().toString() === "SELECT * FROM users WHERE age > 18";
      function fn_679() {
        return "where int";
      }
      test_673.assert(t_678, fn_679);
      return;
    } finally {
      test_673.softFailToHard();
    }
});
it("where adds condition with bool value", function () {
    const test_680 = new Test_448();
    try {
      let t_681 = sid_658("users");
      let t_682 = new SqlBuilder();
      t_682.appendSafe("active = ");
      t_682.appendBoolean(true);
      let t_683 = t_682.accumulated;
      const q_684 = from(t_681).where(t_683);
      let t_685 = q_684.toSql().toString() === "SELECT * FROM users WHERE active = TRUE";
      function fn_686() {
        return "where bool";
      }
      test_680.assert(t_685, fn_686);
      return;
    } finally {
      test_680.softFailToHard();
    }
});
it("chained where uses AND", function () {
    const test_687 = new Test_448();
    try {
      let t_688 = sid_658("users");
      let t_689 = new SqlBuilder();
      t_689.appendSafe("age > ");
      t_689.appendInt32(18);
      let t_690 = t_689.accumulated;
      let t_691 = from(t_688).where(t_690);
      let t_692 = new SqlBuilder();
      t_692.appendSafe("active = ");
      t_692.appendBoolean(true);
      const q_693 = t_691.where(t_692.accumulated);
      let t_694 = q_693.toSql().toString() === "SELECT * FROM users WHERE age > 18 AND active = TRUE";
      function fn_695() {
        return "chained where";
      }
      test_687.assert(t_694, fn_695);
      return;
    } finally {
      test_687.softFailToHard();
    }
});
it("orderBy ASC", function () {
    const test_696 = new Test_448();
    try {
      let t_697 = sid_658("users");
      let t_698 = sid_658("name");
      const q_699 = from(t_697).orderBy(t_698, true);
      let t_700 = q_699.toSql().toString() === "SELECT * FROM users ORDER BY name ASC";
      function fn_701() {
        return "order asc";
      }
      test_696.assert(t_700, fn_701);
      return;
    } finally {
      test_696.softFailToHard();
    }
});
it("orderBy DESC", function () {
    const test_702 = new Test_448();
    try {
      let t_703 = sid_658("users");
      let t_704 = sid_658("created_at");
      const q_705 = from(t_703).orderBy(t_704, false);
      let t_706 = q_705.toSql().toString() === "SELECT * FROM users ORDER BY created_at DESC";
      function fn_707() {
        return "order desc";
      }
      test_702.assert(t_706, fn_707);
      return;
    } finally {
      test_702.softFailToHard();
    }
});
it("limit and offset", function () {
    const test_708 = new Test_448();
    try {
      let t_709;
      let t_710;
      let q_711;
      try {
        t_709 = from(sid_658("users")).limit(10);
        t_710 = t_709.offset(20);
        q_711 = t_710;
      } catch {
        q_711 = panic_445();
      }
      let t_712 = q_711.toSql().toString() === "SELECT * FROM users LIMIT 10 OFFSET 20";
      function fn_713() {
        return "limit/offset";
      }
      test_708.assert(t_712, fn_713);
      return;
    } finally {
      test_708.softFailToHard();
    }
});
it("limit bubbles on negative", function () {
    const test_714 = new Test_448();
    try {
      let didBubble_715;
      try {
        from(sid_658("users")).limit(-1);
        didBubble_715 = false;
      } catch {
        didBubble_715 = true;
      }
      function fn_716() {
        return "negative limit should bubble";
      }
      test_714.assert(didBubble_715, fn_716);
      return;
    } finally {
      test_714.softFailToHard();
    }
});
it("offset bubbles on negative", function () {
    const test_717 = new Test_448();
    try {
      let didBubble_718;
      try {
        from(sid_658("users")).offset(-1);
        didBubble_718 = false;
      } catch {
        didBubble_718 = true;
      }
      function fn_719() {
        return "negative offset should bubble";
      }
      test_717.assert(didBubble_718, fn_719);
      return;
    } finally {
      test_717.softFailToHard();
    }
});
it("complex composed query", function () {
    const test_720 = new Test_448();
    try {
      let t_721;
      let t_722;
      let t_723;
      let t_724;
      let t_725;
      let t_726;
      let t_727;
      let t_728;
      let t_729;
      let t_730;
      const minAge_731 = 21;
      let q_732;
      try {
        t_721 = sid_658("users");
        t_722 = sid_658("id");
        t_723 = sid_658("name");
        t_724 = sid_658("email");
        t_725 = from(t_721).select(Object.freeze([t_722, t_723, t_724]));
        t_726 = new SqlBuilder();
        t_726.appendSafe("age >= ");
        t_726.appendInt32(21);
        t_727 = t_725.where(t_726.accumulated);
        t_728 = new SqlBuilder();
        t_728.appendSafe("active = ");
        t_728.appendBoolean(true);
        t_729 = t_727.where(t_728.accumulated).orderBy(sid_658("name"), true).limit(25);
        t_730 = t_729.offset(0);
        q_732 = t_730;
      } catch {
        q_732 = panic_445();
      }
      let t_733 = q_732.toSql().toString() === "SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0";
      function fn_734() {
        return "complex query";
      }
      test_720.assert(t_733, fn_734);
      return;
    } finally {
      test_720.softFailToHard();
    }
});
it("safeToSql applies default limit when none set", function () {
    const test_735 = new Test_448();
    try {
      let t_736;
      let t_737;
      const q_738 = from(sid_658("users"));
      try {
        t_736 = q_738.safeToSql(100);
        t_737 = t_736;
      } catch {
        t_737 = panic_445();
      }
      const s_739 = t_737.toString();
      let t_740 = s_739 === "SELECT * FROM users LIMIT 100";
      function fn_741() {
        return "should have limit: " + s_739;
      }
      test_735.assert(t_740, fn_741);
      return;
    } finally {
      test_735.softFailToHard();
    }
});
it("safeToSql respects explicit limit", function () {
    const test_742 = new Test_448();
    try {
      let t_743;
      let t_744;
      let t_745;
      let q_746;
      try {
        t_743 = from(sid_658("users")).limit(5);
        q_746 = t_743;
      } catch {
        q_746 = panic_445();
      }
      try {
        t_744 = q_746.safeToSql(100);
        t_745 = t_744;
      } catch {
        t_745 = panic_445();
      }
      const s_747 = t_745.toString();
      let t_748 = s_747 === "SELECT * FROM users LIMIT 5";
      function fn_749() {
        return "explicit limit preserved: " + s_747;
      }
      test_742.assert(t_748, fn_749);
      return;
    } finally {
      test_742.softFailToHard();
    }
});
it("safeToSql bubbles on negative defaultLimit", function () {
    const test_750 = new Test_448();
    try {
      let didBubble_751;
      try {
        from(sid_658("users")).safeToSql(-1);
        didBubble_751 = false;
      } catch {
        didBubble_751 = true;
      }
      function fn_752() {
        return "negative defaultLimit should bubble";
      }
      test_750.assert(didBubble_751, fn_752);
      return;
    } finally {
      test_750.softFailToHard();
    }
});
it("where with injection attempt in string value is escaped", function () {
    const test_753 = new Test_448();
    try {
      const evil_754 = "'; DROP TABLE users; --";
      let t_755 = sid_658("users");
      let t_756 = new SqlBuilder();
      t_756.appendSafe("name = ");
      t_756.appendString("'; DROP TABLE users; --");
      let t_757 = t_756.accumulated;
      const q_758 = from(t_755).where(t_757);
      const s_759 = q_758.toSql().toString();
      let t_760 = s_759.indexOf("''") >= 0;
      function fn_761() {
        return "quotes must be doubled: " + s_759;
      }
      test_753.assert(t_760, fn_761);
      let t_762 = s_759.indexOf("SELECT * FROM users WHERE name =") >= 0;
      function fn_763() {
        return "structure intact: " + s_759;
      }
      test_753.assert(t_762, fn_763);
      return;
    } finally {
      test_753.softFailToHard();
    }
});
it("safeIdentifier rejects user-supplied table name with metacharacters", function () {
    const test_764 = new Test_448();
    try {
      const attack_765 = "users; DROP TABLE users; --";
      let didBubble_766;
      try {
        safeIdentifier("users; DROP TABLE users; --");
        didBubble_766 = false;
      } catch {
        didBubble_766 = true;
      }
      function fn_767() {
        return "metacharacter-containing name must be rejected at construction";
      }
      test_764.assert(didBubble_766, fn_767);
      return;
    } finally {
      test_764.softFailToHard();
    }
});
it("safeIdentifier accepts valid names", function () {
    const test_768 = new Test_448();
    try {
      let t_769;
      let id_770;
      try {
        t_769 = safeIdentifier("user_name");
        id_770 = t_769;
      } catch {
        id_770 = panic_445();
      }
      let t_771 = id_770.sqlValue === "user_name";
      function fn_772() {
        return "value should round-trip";
      }
      test_768.assert(t_771, fn_772);
      return;
    } finally {
      test_768.softFailToHard();
    }
});
it("safeIdentifier rejects empty string", function () {
    const test_773 = new Test_448();
    try {
      let didBubble_774;
      try {
        safeIdentifier("");
        didBubble_774 = false;
      } catch {
        didBubble_774 = true;
      }
      function fn_775() {
        return "empty string should bubble";
      }
      test_773.assert(didBubble_774, fn_775);
      return;
    } finally {
      test_773.softFailToHard();
    }
});
it("safeIdentifier rejects leading digit", function () {
    const test_776 = new Test_448();
    try {
      let didBubble_777;
      try {
        safeIdentifier("1col");
        didBubble_777 = false;
      } catch {
        didBubble_777 = true;
      }
      function fn_778() {
        return "leading digit should bubble";
      }
      test_776.assert(didBubble_777, fn_778);
      return;
    } finally {
      test_776.softFailToHard();
    }
});
it("safeIdentifier rejects SQL metacharacters", function () {
    const test_779 = new Test_448();
    try {
      const cases_780 = Object.freeze(["name); DROP TABLE", "col'", "a b", "a-b", "a.b", "a;b"]);
      function fn_781(c_782) {
        let didBubble_783;
        try {
          safeIdentifier(c_782);
          didBubble_783 = false;
        } catch {
          didBubble_783 = true;
        }
        function fn_784() {
          return "should reject: " + c_782;
        }
        test_779.assert(didBubble_783, fn_784);
        return;
      }
      cases_780.forEach(fn_781);
      return;
    } finally {
      test_779.softFailToHard();
    }
});
it("TableDef field lookup - found", function () {
    const test_785 = new Test_448();
    try {
      let t_786;
      let t_787;
      let t_788;
      let t_789;
      let t_790;
      let t_791;
      let t_792;
      try {
        t_786 = safeIdentifier("users");
        t_787 = t_786;
      } catch {
        t_787 = panic_445();
      }
      try {
        t_788 = safeIdentifier("name");
        t_789 = t_788;
      } catch {
        t_789 = panic_445();
      }
      let t_793 = new StringField();
      let t_794 = new FieldDef(t_789, t_793, false);
      try {
        t_790 = safeIdentifier("age");
        t_791 = t_790;
      } catch {
        t_791 = panic_445();
      }
      let t_795 = new IntField();
      let t_796 = new FieldDef(t_791, t_795, false);
      const td_797 = new TableDef(t_787, Object.freeze([t_794, t_796]));
      let f_798;
      try {
        t_792 = td_797.field("age");
        f_798 = t_792;
      } catch {
        f_798 = panic_445();
      }
      let t_799 = f_798.name.sqlValue === "age";
      function fn_800() {
        return "should find age field";
      }
      test_785.assert(t_799, fn_800);
      return;
    } finally {
      test_785.softFailToHard();
    }
});
it("TableDef field lookup - not found bubbles", function () {
    const test_801 = new Test_448();
    try {
      let t_802;
      let t_803;
      let t_804;
      let t_805;
      try {
        t_802 = safeIdentifier("users");
        t_803 = t_802;
      } catch {
        t_803 = panic_445();
      }
      try {
        t_804 = safeIdentifier("name");
        t_805 = t_804;
      } catch {
        t_805 = panic_445();
      }
      let t_806 = new StringField();
      let t_807 = new FieldDef(t_805, t_806, false);
      const td_808 = new TableDef(t_803, Object.freeze([t_807]));
      let didBubble_809;
      try {
        td_808.field("nonexistent");
        didBubble_809 = false;
      } catch {
        didBubble_809 = true;
      }
      function fn_810() {
        return "unknown field should bubble";
      }
      test_801.assert(didBubble_809, fn_810);
      return;
    } finally {
      test_801.softFailToHard();
    }
});
it("FieldDef nullable flag", function () {
    const test_811 = new Test_448();
    try {
      let t_812;
      let t_813;
      let t_814;
      let t_815;
      try {
        t_812 = safeIdentifier("email");
        t_813 = t_812;
      } catch {
        t_813 = panic_445();
      }
      let t_816 = new StringField();
      const required_817 = new FieldDef(t_813, t_816, false);
      try {
        t_814 = safeIdentifier("bio");
        t_815 = t_814;
      } catch {
        t_815 = panic_445();
      }
      let t_818 = new StringField();
      const optional_819 = new FieldDef(t_815, t_818, true);
      let t_820 = ! required_817.nullable;
      function fn_821() {
        return "required field should not be nullable";
      }
      test_811.assert(t_820, fn_821);
      let t_822 = optional_819.nullable;
      function fn_823() {
        return "optional field should be nullable";
      }
      test_811.assert(t_822, fn_823);
      return;
    } finally {
      test_811.softFailToHard();
    }
});
it("string escaping", function () {
    const test_824 = new Test_448();
    try {
      function build_825(name_826) {
        let t_827 = new SqlBuilder();
        t_827.appendSafe("select * from hi where name = ");
        t_827.appendString(name_826);
        return t_827.accumulated.toString();
      }
      function buildWrong_828(name_829) {
        return "select * from hi where name = '" + name_829 + "'";
      }
      const actual_830 = build_825("world");
      let t_831 = actual_830 === "select * from hi where name = 'world'";
      function fn_832() {
        return 'expected build("world") == (' + "select * from hi where name = 'world'" + ") not (" + actual_830 + ")";
      }
      test_824.assert(t_831, fn_832);
      const bobbyTables_833 = "Robert'); drop table hi;--";
      const actual_834 = build_825("Robert'); drop table hi;--");
      let t_835 = actual_834 === "select * from hi where name = 'Robert''); drop table hi;--'";
      function fn_836() {
        return "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual_834 + ")";
      }
      test_824.assert(t_835, fn_836);
      function fn_837() {
        return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
      }
      test_824.assert(true, fn_837);
      return;
    } finally {
      test_824.softFailToHard();
    }
});
it("string edge cases", function () {
    const test_838 = new Test_448();
    try {
      let t_839 = new SqlBuilder();
      t_839.appendSafe("v = ");
      t_839.appendString("");
      const actual_840 = t_839.accumulated.toString();
      let t_841 = actual_840 === "v = ''";
      function fn_842() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "").toString() == (' + "v = ''" + ") not (" + actual_840 + ")";
      }
      test_838.assert(t_841, fn_842);
      let t_843 = new SqlBuilder();
      t_843.appendSafe("v = ");
      t_843.appendString("a''b");
      const actual_844 = t_843.accumulated.toString();
      let t_845 = actual_844 === "v = 'a''''b'";
      function fn_846() {
        return "expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (" + "v = 'a''''b'" + ") not (" + actual_844 + ")";
      }
      test_838.assert(t_845, fn_846);
      let t_847 = new SqlBuilder();
      t_847.appendSafe("v = ");
      t_847.appendString("Hello 世界");
      const actual_848 = t_847.accumulated.toString();
      let t_849 = actual_848 === "v = 'Hello 世界'";
      function fn_850() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Hello 世界").toString() == (' + "v = 'Hello 世界'" + ") not (" + actual_848 + ")";
      }
      test_838.assert(t_849, fn_850);
      let t_851 = new SqlBuilder();
      t_851.appendSafe("v = ");
      t_851.appendString("Line1\nLine2");
      const actual_852 = t_851.accumulated.toString();
      let t_853 = actual_852 === "v = 'Line1\nLine2'";
      function fn_854() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v = ", \\interpolate, "Line1\\nLine2").toString() == (' + "v = 'Line1\nLine2'" + ") not (" + actual_852 + ")";
      }
      test_838.assert(t_853, fn_854);
      return;
    } finally {
      test_838.softFailToHard();
    }
});
it("numbers and booleans", function () {
    const test_855 = new Test_448();
    try {
      let t_856;
      let t_857 = new SqlBuilder();
      t_857.appendSafe("select ");
      t_857.appendInt32(42);
      t_857.appendSafe(", ");
      t_857.appendInt64(BigInt("43"));
      t_857.appendSafe(", ");
      t_857.appendFloat64(19.99);
      t_857.appendSafe(", ");
      t_857.appendBoolean(true);
      t_857.appendSafe(", ");
      t_857.appendBoolean(false);
      const actual_858 = t_857.accumulated.toString();
      let t_859 = actual_858 === "select 42, 43, 19.99, TRUE, FALSE";
      function fn_860() {
        return 'expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, 42, ", ", \\interpolate, 43, ", ", \\interpolate, 19.99, ", ", \\interpolate, true, ", ", \\interpolate, false).toString() == (' + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual_858 + ")";
      }
      test_855.assert(t_859, fn_860);
      let date_861;
      try {
        t_856 = new (globalThis.Date)(globalThis.Date.UTC(2024, 12 - 1, 25));
        date_861 = t_856;
      } catch {
        date_861 = panic_445();
      }
      let t_862 = new SqlBuilder();
      t_862.appendSafe("insert into t values (");
      t_862.appendDate(date_861);
      t_862.appendSafe(")");
      const actual_863 = t_862.accumulated.toString();
      let t_864 = actual_863 === "insert into t values ('2024-12-25')";
      function fn_865() {
        return 'expected stringExpr(`-work//src/`.sql, true, "insert into t values (", \\interpolate, date, ")").toString() == (' + "insert into t values ('2024-12-25')" + ") not (" + actual_863 + ")";
      }
      test_855.assert(t_864, fn_865);
      return;
    } finally {
      test_855.softFailToHard();
    }
});
it("lists", function () {
    const test_866 = new Test_448();
    try {
      let t_867;
      let t_868;
      let t_869;
      let t_870;
      let t_871 = new SqlBuilder();
      t_871.appendSafe("v IN (");
      t_871.appendStringList(Object.freeze(["a", "b", "c'd"]));
      t_871.appendSafe(")");
      const actual_872 = t_871.accumulated.toString();
      let t_873 = actual_872 === "v IN ('a', 'b', 'c''d')";
      function fn_874() {
        return "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual_872 + ")";
      }
      test_866.assert(t_873, fn_874);
      let t_875 = new SqlBuilder();
      t_875.appendSafe("v IN (");
      t_875.appendInt32List(Object.freeze([1, 2, 3]));
      t_875.appendSafe(")");
      const actual_876 = t_875.accumulated.toString();
      let t_877 = actual_876 === "v IN (1, 2, 3)";
      function fn_878() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2, 3), ")").toString() == (' + "v IN (1, 2, 3)" + ") not (" + actual_876 + ")";
      }
      test_866.assert(t_877, fn_878);
      let t_879 = new SqlBuilder();
      t_879.appendSafe("v IN (");
      t_879.appendInt64List(Object.freeze([BigInt("1"), BigInt("2")]));
      t_879.appendSafe(")");
      const actual_880 = t_879.accumulated.toString();
      let t_881 = actual_880 === "v IN (1, 2)";
      function fn_882() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1, 2), ")").toString() == (' + "v IN (1, 2)" + ") not (" + actual_880 + ")";
      }
      test_866.assert(t_881, fn_882);
      let t_883 = new SqlBuilder();
      t_883.appendSafe("v IN (");
      t_883.appendFloat64List(Object.freeze([1.0, 2.0]));
      t_883.appendSafe(")");
      const actual_884 = t_883.accumulated.toString();
      let t_885 = actual_884 === "v IN (1.0, 2.0)";
      function fn_886() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(1.0, 2.0), ")").toString() == (' + "v IN (1.0, 2.0)" + ") not (" + actual_884 + ")";
      }
      test_866.assert(t_885, fn_886);
      let t_887 = new SqlBuilder();
      t_887.appendSafe("v IN (");
      t_887.appendBooleanList(Object.freeze([true, false]));
      t_887.appendSafe(")");
      const actual_888 = t_887.accumulated.toString();
      let t_889 = actual_888 === "v IN (TRUE, FALSE)";
      function fn_890() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, list(true, false), ")").toString() == (' + "v IN (TRUE, FALSE)" + ") not (" + actual_888 + ")";
      }
      test_866.assert(t_889, fn_890);
      try {
        t_867 = new (globalThis.Date)(globalThis.Date.UTC(2024, 1 - 1, 1));
        t_868 = t_867;
      } catch {
        t_868 = panic_445();
      }
      try {
        t_869 = new (globalThis.Date)(globalThis.Date.UTC(2024, 12 - 1, 25));
        t_870 = t_869;
      } catch {
        t_870 = panic_445();
      }
      const dates_891 = Object.freeze([t_868, t_870]);
      let t_892 = new SqlBuilder();
      t_892.appendSafe("v IN (");
      t_892.appendDateList(dates_891);
      t_892.appendSafe(")");
      const actual_893 = t_892.accumulated.toString();
      let t_894 = actual_893 === "v IN ('2024-01-01', '2024-12-25')";
      function fn_895() {
        return 'expected stringExpr(`-work//src/`.sql, true, "v IN (", \\interpolate, dates, ")").toString() == (' + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual_893 + ")";
      }
      test_866.assert(t_894, fn_895);
      return;
    } finally {
      test_866.softFailToHard();
    }
});
it("nesting", function () {
    const test_896 = new Test_448();
    try {
      const name_897 = "Someone";
      let t_898 = new SqlBuilder();
      t_898.appendSafe("where p.last_name = ");
      t_898.appendString("Someone");
      const condition_899 = t_898.accumulated;
      let t_900 = new SqlBuilder();
      t_900.appendSafe("select p.id from person p ");
      t_900.appendFragment(condition_899);
      const actual_901 = t_900.accumulated.toString();
      let t_902 = actual_901 === "select p.id from person p where p.last_name = 'Someone'";
      function fn_903() {
        return 'expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition).toString() == (' + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_901 + ")";
      }
      test_896.assert(t_902, fn_903);
      let t_904 = new SqlBuilder();
      t_904.appendSafe("select p.id from person p ");
      t_904.appendPart(condition_899.toSource());
      const actual_905 = t_904.accumulated.toString();
      let t_906 = actual_905 === "select p.id from person p where p.last_name = 'Someone'";
      function fn_907() {
        return 'expected stringExpr(`-work//src/`.sql, true, "select p.id from person p ", \\interpolate, condition.toSource()).toString() == (' + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_905 + ")";
      }
      test_896.assert(t_906, fn_907);
      const parts_908 = Object.freeze([new SqlString("a'b"), new SqlInt32(3)]);
      let t_909 = new SqlBuilder();
      t_909.appendSafe("select ");
      t_909.appendPartList(parts_908);
      const actual_910 = t_909.accumulated.toString();
      let t_911 = actual_910 === "select 'a''b', 3";
      function fn_912() {
        return 'expected stringExpr(`-work//src/`.sql, true, "select ", \\interpolate, parts).toString() == (' + "select 'a''b', 3" + ") not (" + actual_910 + ")";
      }
      test_896.assert(t_911, fn_912);
      return;
    } finally {
      test_896.softFailToHard();
    }
});
