package orm.src;
import temper.std.testing.Test;
import java.util.function.Supplier;
import java.util.List;
import java.util.AbstractMap.SimpleImmutableEntry;
import temper.core.Core;
import java.util.Map;
import java.time.LocalDate;
import java.util.function.Consumer;
import java.util.function.Function;
public final class SrcTest {
    private SrcTest() {
    }
    static SafeIdentifier csid__293(String name__438) {
        SafeIdentifier t_2624;
        t_2624 = SrcGlobal.safeIdentifier(name__438);
        return t_2624;
    }
    static TableDef userTable__294() {
        return new TableDef(SrcTest.csid__293("users"), List.of(new FieldDef(SrcTest.csid__293("name"), new StringField(), false), new FieldDef(SrcTest.csid__293("email"), new StringField(), false), new FieldDef(SrcTest.csid__293("age"), new IntField(), true), new FieldDef(SrcTest.csid__293("score"), new FloatField(), true), new FieldDef(SrcTest.csid__293("active"), new BoolField(), true)));
    }
    @org.junit.jupiter.api.Test public void castWhitelistsAllowedFields__888() {
        Test test_20 = new Test();
        try {
            Map<String, String> params__442 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("email", "alice@example.com"), new SimpleImmutableEntry<>("admin", "true")));
            TableDef t_4547 = SrcTest.userTable__294();
            SafeIdentifier t_4548 = SrcTest.csid__293("name");
            SafeIdentifier t_4549 = SrcTest.csid__293("email");
            Changeset cs__443 = SrcGlobal.changeset(t_4547, params__442).cast(List.of(t_4548, t_4549));
            boolean t_4552 = cs__443.getChanges().containsKey("name");
            Supplier<String> fn__4542 = () -> "name should be in changes";
            test_20.assert_(t_4552, fn__4542);
            boolean t_4556 = cs__443.getChanges().containsKey("email");
            Supplier<String> fn__4541 = () -> "email should be in changes";
            test_20.assert_(t_4556, fn__4541);
            boolean t_4562 = !cs__443.getChanges().containsKey("admin");
            Supplier<String> fn__4540 = () -> "admin must be dropped (not in whitelist)";
            test_20.assert_(t_4562, fn__4540);
            boolean t_4564 = cs__443.isValid();
            Supplier<String> fn__4539 = () -> "should still be valid";
            test_20.assert_(t_4564, fn__4539);
        } finally {
            test_20.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void castIsReplacingNotAdditiveSecondCallResetsWhitelist__889() {
        Test test_21 = new Test();
        try {
            Map<String, String> params__445 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("email", "alice@example.com")));
            TableDef t_4525 = SrcTest.userTable__294();
            SafeIdentifier t_4526 = SrcTest.csid__293("name");
            Changeset cs__446 = SrcGlobal.changeset(t_4525, params__445).cast(List.of(t_4526)).cast(List.of(SrcTest.csid__293("email")));
            boolean t_4533 = !cs__446.getChanges().containsKey("name");
            Supplier<String> fn__4521 = () -> "name must be excluded by second cast";
            test_21.assert_(t_4533, fn__4521);
            boolean t_4536 = cs__446.getChanges().containsKey("email");
            Supplier<String> fn__4520 = () -> "email should be present";
            test_21.assert_(t_4536, fn__4520);
        } finally {
            test_21.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void castIgnoresEmptyStringValues__890() {
        Test test_22 = new Test();
        try {
            Map<String, String> params__448 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", ""), new SimpleImmutableEntry<>("email", "bob@example.com")));
            TableDef t_4507 = SrcTest.userTable__294();
            SafeIdentifier t_4508 = SrcTest.csid__293("name");
            SafeIdentifier t_4509 = SrcTest.csid__293("email");
            Changeset cs__449 = SrcGlobal.changeset(t_4507, params__448).cast(List.of(t_4508, t_4509));
            boolean t_4514 = !cs__449.getChanges().containsKey("name");
            Supplier<String> fn__4503 = () -> "empty name should not be in changes";
            test_22.assert_(t_4514, fn__4503);
            boolean t_4517 = cs__449.getChanges().containsKey("email");
            Supplier<String> fn__4502 = () -> "email should be in changes";
            test_22.assert_(t_4517, fn__4502);
        } finally {
            test_22.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateRequiredPassesWhenFieldPresent__891() {
        Test test_23 = new Test();
        try {
            Map<String, String> params__451 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice")));
            TableDef t_4489 = SrcTest.userTable__294();
            SafeIdentifier t_4490 = SrcTest.csid__293("name");
            Changeset cs__452 = SrcGlobal.changeset(t_4489, params__451).cast(List.of(t_4490)).validateRequired(List.of(SrcTest.csid__293("name")));
            boolean t_4494 = cs__452.isValid();
            Supplier<String> fn__4486 = () -> "should be valid";
            test_23.assert_(t_4494, fn__4486);
            boolean t_4500 = cs__452.getErrors().size() == 0;
            Supplier<String> fn__4485 = () -> "no errors expected";
            test_23.assert_(t_4500, fn__4485);
        } finally {
            test_23.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateRequiredFailsWhenFieldMissing__892() {
        Test test_24 = new Test();
        try {
            Map<String, String> params__454 = Core.mapConstructor(List.of());
            TableDef t_4465 = SrcTest.userTable__294();
            SafeIdentifier t_4466 = SrcTest.csid__293("name");
            Changeset cs__455 = SrcGlobal.changeset(t_4465, params__454).cast(List.of(t_4466)).validateRequired(List.of(SrcTest.csid__293("name")));
            boolean t_4472 = !cs__455.isValid();
            Supplier<String> fn__4463 = () -> "should be invalid";
            test_24.assert_(t_4472, fn__4463);
            boolean t_4477 = cs__455.getErrors().size() == 1;
            Supplier<String> fn__4462 = () -> "should have one error";
            test_24.assert_(t_4477, fn__4462);
            boolean t_4483 = Core.listGet(cs__455.getErrors(), 0).getField().equals("name");
            Supplier<String> fn__4461 = () -> "error should name the field";
            test_24.assert_(t_4483, fn__4461);
        } finally {
            test_24.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateLengthPassesWithinRange__893() {
        Test test_25 = new Test();
        try {
            Map<String, String> params__457 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice")));
            TableDef t_4453 = SrcTest.userTable__294();
            SafeIdentifier t_4454 = SrcTest.csid__293("name");
            Changeset cs__458 = SrcGlobal.changeset(t_4453, params__457).cast(List.of(t_4454)).validateLength(SrcTest.csid__293("name"), 2, 50);
            boolean t_4458 = cs__458.isValid();
            Supplier<String> fn__4450 = () -> "should be valid";
            test_25.assert_(t_4458, fn__4450);
        } finally {
            test_25.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateLengthFailsWhenTooShort__894() {
        Test test_26 = new Test();
        try {
            Map<String, String> params__460 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "A")));
            TableDef t_4441 = SrcTest.userTable__294();
            SafeIdentifier t_4442 = SrcTest.csid__293("name");
            Changeset cs__461 = SrcGlobal.changeset(t_4441, params__460).cast(List.of(t_4442)).validateLength(SrcTest.csid__293("name"), 2, 50);
            boolean t_4448 = !cs__461.isValid();
            Supplier<String> fn__4438 = () -> "should be invalid";
            test_26.assert_(t_4448, fn__4438);
        } finally {
            test_26.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateLengthFailsWhenTooLong__895() {
        Test test_27 = new Test();
        try {
            Map<String, String> params__463 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "ABCDEFGHIJKLMNOPQRSTUVWXYZ")));
            TableDef t_4429 = SrcTest.userTable__294();
            SafeIdentifier t_4430 = SrcTest.csid__293("name");
            Changeset cs__464 = SrcGlobal.changeset(t_4429, params__463).cast(List.of(t_4430)).validateLength(SrcTest.csid__293("name"), 2, 10);
            boolean t_4436 = !cs__464.isValid();
            Supplier<String> fn__4426 = () -> "should be invalid";
            test_27.assert_(t_4436, fn__4426);
        } finally {
            test_27.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateIntPassesForValidInteger__896() {
        Test test_28 = new Test();
        try {
            Map<String, String> params__466 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("age", "30")));
            TableDef t_4418 = SrcTest.userTable__294();
            SafeIdentifier t_4419 = SrcTest.csid__293("age");
            Changeset cs__467 = SrcGlobal.changeset(t_4418, params__466).cast(List.of(t_4419)).validateInt(SrcTest.csid__293("age"));
            boolean t_4423 = cs__467.isValid();
            Supplier<String> fn__4415 = () -> "should be valid";
            test_28.assert_(t_4423, fn__4415);
        } finally {
            test_28.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateIntFailsForNonInteger__897() {
        Test test_29 = new Test();
        try {
            Map<String, String> params__469 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("age", "not-a-number")));
            TableDef t_4406 = SrcTest.userTable__294();
            SafeIdentifier t_4407 = SrcTest.csid__293("age");
            Changeset cs__470 = SrcGlobal.changeset(t_4406, params__469).cast(List.of(t_4407)).validateInt(SrcTest.csid__293("age"));
            boolean t_4413 = !cs__470.isValid();
            Supplier<String> fn__4403 = () -> "should be invalid";
            test_29.assert_(t_4413, fn__4403);
        } finally {
            test_29.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateFloatPassesForValidFloat__898() {
        Test test_30 = new Test();
        try {
            Map<String, String> params__472 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("score", "9.5")));
            TableDef t_4395 = SrcTest.userTable__294();
            SafeIdentifier t_4396 = SrcTest.csid__293("score");
            Changeset cs__473 = SrcGlobal.changeset(t_4395, params__472).cast(List.of(t_4396)).validateFloat(SrcTest.csid__293("score"));
            boolean t_4400 = cs__473.isValid();
            Supplier<String> fn__4392 = () -> "should be valid";
            test_30.assert_(t_4400, fn__4392);
        } finally {
            test_30.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateInt64_passesForValid64_bitInteger__899() {
        Test test_31 = new Test();
        try {
            Map<String, String> params__475 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("age", "9999999999")));
            TableDef t_4384 = SrcTest.userTable__294();
            SafeIdentifier t_4385 = SrcTest.csid__293("age");
            Changeset cs__476 = SrcGlobal.changeset(t_4384, params__475).cast(List.of(t_4385)).validateInt64(SrcTest.csid__293("age"));
            boolean t_4389 = cs__476.isValid();
            Supplier<String> fn__4381 = () -> "should be valid";
            test_31.assert_(t_4389, fn__4381);
        } finally {
            test_31.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateInt64_failsForNonInteger__900() {
        Test test_32 = new Test();
        try {
            Map<String, String> params__478 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("age", "not-a-number")));
            TableDef t_4372 = SrcTest.userTable__294();
            SafeIdentifier t_4373 = SrcTest.csid__293("age");
            Changeset cs__479 = SrcGlobal.changeset(t_4372, params__478).cast(List.of(t_4373)).validateInt64(SrcTest.csid__293("age"));
            boolean t_4379 = !cs__479.isValid();
            Supplier<String> fn__4369 = () -> "should be invalid";
            test_32.assert_(t_4379, fn__4369);
        } finally {
            test_32.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateBoolAcceptsTrue1_yesOn__901() {
        Test test_33 = new Test();
        try {
            Consumer<String> fn__4366 = v__481 -> {
                Map<String, String> params__482 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("active", v__481)));
                TableDef t_4358 = SrcTest.userTable__294();
                SafeIdentifier t_4359 = SrcTest.csid__293("active");
                Changeset cs__483 = SrcGlobal.changeset(t_4358, params__482).cast(List.of(t_4359)).validateBool(SrcTest.csid__293("active"));
                boolean t_4363 = cs__483.isValid();
                Supplier<String> fn__4355 = () -> "should accept: " + v__481;
                test_33.assert_(t_4363, fn__4355);
            };
            List.of("true", "1", "yes", "on").forEach(fn__4366);
        } finally {
            test_33.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateBoolAcceptsFalse0_noOff__902() {
        Test test_34 = new Test();
        try {
            Consumer<String> fn__4352 = v__485 -> {
                Map<String, String> params__486 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("active", v__485)));
                TableDef t_4344 = SrcTest.userTable__294();
                SafeIdentifier t_4345 = SrcTest.csid__293("active");
                Changeset cs__487 = SrcGlobal.changeset(t_4344, params__486).cast(List.of(t_4345)).validateBool(SrcTest.csid__293("active"));
                boolean t_4349 = cs__487.isValid();
                Supplier<String> fn__4341 = () -> "should accept: " + v__485;
                test_34.assert_(t_4349, fn__4341);
            };
            List.of("false", "0", "no", "off").forEach(fn__4352);
        } finally {
            test_34.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void validateBoolRejectsAmbiguousValues__903() {
        Test test_35 = new Test();
        try {
            Consumer<String> fn__4338 = v__489 -> {
                Map<String, String> params__490 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("active", v__489)));
                TableDef t_4329 = SrcTest.userTable__294();
                SafeIdentifier t_4330 = SrcTest.csid__293("active");
                Changeset cs__491 = SrcGlobal.changeset(t_4329, params__490).cast(List.of(t_4330)).validateBool(SrcTest.csid__293("active"));
                boolean t_4336 = !cs__491.isValid();
                Supplier<String> fn__4326 = () -> "should reject ambiguous: " + v__489;
                test_35.assert_(t_4336, fn__4326);
            };
            List.of("TRUE", "Yes", "maybe", "2", "enabled").forEach(fn__4338);
        } finally {
            test_35.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlEscapesBobbyTables__904() {
        Test test_36 = new Test();
        try {
            Map<String, String> params__493 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Robert'); DROP TABLE users;--"), new SimpleImmutableEntry<>("email", "bobby@evil.com")));
            TableDef t_4314 = SrcTest.userTable__294();
            SafeIdentifier t_4315 = SrcTest.csid__293("name");
            SafeIdentifier t_4316 = SrcTest.csid__293("email");
            Changeset cs__494 = SrcGlobal.changeset(t_4314, params__493).cast(List.of(t_4315, t_4316)).validateRequired(List.of(SrcTest.csid__293("name"), SrcTest.csid__293("email")));
            SqlFragment t_2425;
            t_2425 = cs__494.toInsertSql();
            SqlFragment sqlFrag__495 = t_2425;
            String s__496 = sqlFrag__495.toString();
            boolean t_4323 = s__496.indexOf("''") >= 0;
            Supplier<String> fn__4310 = () -> "single quote must be doubled: " + s__496;
            test_36.assert_(t_4323, fn__4310);
        } finally {
            test_36.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlProducesCorrectSqlForStringField__905() {
        Test test_37 = new Test();
        try {
            Map<String, String> params__498 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Alice"), new SimpleImmutableEntry<>("email", "a@example.com")));
            TableDef t_4294 = SrcTest.userTable__294();
            SafeIdentifier t_4295 = SrcTest.csid__293("name");
            SafeIdentifier t_4296 = SrcTest.csid__293("email");
            Changeset cs__499 = SrcGlobal.changeset(t_4294, params__498).cast(List.of(t_4295, t_4296)).validateRequired(List.of(SrcTest.csid__293("name"), SrcTest.csid__293("email")));
            SqlFragment t_2404;
            t_2404 = cs__499.toInsertSql();
            SqlFragment sqlFrag__500 = t_2404;
            String s__501 = sqlFrag__500.toString();
            boolean t_4303 = s__501.indexOf("INSERT INTO users") >= 0;
            Supplier<String> fn__4290 = () -> "has INSERT INTO: " + s__501;
            test_37.assert_(t_4303, fn__4290);
            boolean t_4307 = s__501.indexOf("'Alice'") >= 0;
            Supplier<String> fn__4289 = () -> "has quoted name: " + s__501;
            test_37.assert_(t_4307, fn__4289);
        } finally {
            test_37.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlProducesCorrectSqlForIntField__906() {
        Test test_38 = new Test();
        try {
            Map<String, String> params__503 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Bob"), new SimpleImmutableEntry<>("email", "b@example.com"), new SimpleImmutableEntry<>("age", "25")));
            TableDef t_4276 = SrcTest.userTable__294();
            SafeIdentifier t_4277 = SrcTest.csid__293("name");
            SafeIdentifier t_4278 = SrcTest.csid__293("email");
            SafeIdentifier t_4279 = SrcTest.csid__293("age");
            Changeset cs__504 = SrcGlobal.changeset(t_4276, params__503).cast(List.of(t_4277, t_4278, t_4279)).validateRequired(List.of(SrcTest.csid__293("name"), SrcTest.csid__293("email")));
            SqlFragment t_2387;
            t_2387 = cs__504.toInsertSql();
            SqlFragment sqlFrag__505 = t_2387;
            String s__506 = sqlFrag__505.toString();
            boolean t_4286 = s__506.indexOf("25") >= 0;
            Supplier<String> fn__4271 = () -> "age rendered unquoted: " + s__506;
            test_38.assert_(t_4286, fn__4271);
        } finally {
            test_38.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlBubblesOnInvalidChangeset__907() {
        Test test_39 = new Test();
        try {
            Map<String, String> params__508 = Core.mapConstructor(List.of());
            TableDef t_4264 = SrcTest.userTable__294();
            SafeIdentifier t_4265 = SrcTest.csid__293("name");
            Changeset cs__509 = SrcGlobal.changeset(t_4264, params__508).cast(List.of(t_4265)).validateRequired(List.of(SrcTest.csid__293("name")));
            boolean didBubble__510;
            boolean didBubble_4859;
            try {
                cs__509.toInsertSql();
                didBubble_4859 = false;
            } catch (RuntimeException ignored$4) {
                didBubble_4859 = true;
            }
            didBubble__510 = didBubble_4859;
            Supplier<String> fn__4262 = () -> "invalid changeset should bubble";
            test_39.assert_(didBubble__510, fn__4262);
        } finally {
            test_39.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__908() {
        Test test_40 = new Test();
        try {
            TableDef strictTable__512 = new TableDef(SrcTest.csid__293("posts"), List.of(new FieldDef(SrcTest.csid__293("title"), new StringField(), false), new FieldDef(SrcTest.csid__293("body"), new StringField(), true)));
            Map<String, String> params__513 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("body", "hello")));
            SafeIdentifier t_4255 = SrcTest.csid__293("body");
            Changeset cs__514 = SrcGlobal.changeset(strictTable__512, params__513).cast(List.of(t_4255));
            boolean t_4257 = cs__514.isValid();
            Supplier<String> fn__4244 = () -> "changeset should appear valid (no explicit validation run)";
            test_40.assert_(t_4257, fn__4244);
            boolean didBubble__515;
            boolean didBubble_4860;
            try {
                cs__514.toInsertSql();
                didBubble_4860 = false;
            } catch (RuntimeException ignored$5) {
                didBubble_4860 = true;
            }
            didBubble__515 = didBubble_4860;
            Supplier<String> fn__4243 = () -> "toInsertSql should enforce nullable regardless of isValid";
            test_40.assert_(didBubble__515, fn__4243);
        } finally {
            test_40.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toUpdateSqlProducesCorrectSql__909() {
        Test test_41 = new Test();
        try {
            Map<String, String> params__517 = Core.mapConstructor(List.of(new SimpleImmutableEntry<>("name", "Bob")));
            TableDef t_4234 = SrcTest.userTable__294();
            SafeIdentifier t_4235 = SrcTest.csid__293("name");
            Changeset cs__518 = SrcGlobal.changeset(t_4234, params__517).cast(List.of(t_4235)).validateRequired(List.of(SrcTest.csid__293("name")));
            SqlFragment t_2347;
            t_2347 = cs__518.toUpdateSql(42);
            SqlFragment sqlFrag__519 = t_2347;
            String s__520 = sqlFrag__519.toString();
            boolean t_4241 = s__520.equals("UPDATE users SET name = 'Bob' WHERE id = 42");
            Supplier<String> fn__4231 = () -> "got: " + s__520;
            test_41.assert_(t_4241, fn__4231);
        } finally {
            test_41.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void toUpdateSqlBubblesOnInvalidChangeset__910() {
        Test test_42 = new Test();
        try {
            Map<String, String> params__522 = Core.mapConstructor(List.of());
            TableDef t_4224 = SrcTest.userTable__294();
            SafeIdentifier t_4225 = SrcTest.csid__293("name");
            Changeset cs__523 = SrcGlobal.changeset(t_4224, params__522).cast(List.of(t_4225)).validateRequired(List.of(SrcTest.csid__293("name")));
            boolean didBubble__524;
            boolean didBubble_4861;
            try {
                cs__523.toUpdateSql(1);
                didBubble_4861 = false;
            } catch (RuntimeException ignored$6) {
                didBubble_4861 = true;
            }
            didBubble__524 = didBubble_4861;
            Supplier<String> fn__4222 = () -> "invalid changeset should bubble";
            test_42.assert_(didBubble__524, fn__4222);
        } finally {
            test_42.softFailToHard();
        }
    }
    static SafeIdentifier sid__295(String name__579) {
        SafeIdentifier t_2261;
        t_2261 = SrcGlobal.safeIdentifier(name__579);
        return t_2261;
    }
    @org.junit.jupiter.api.Test public void bareFromProducesSelect__935() {
        Test test_43 = new Test();
        try {
            Query q__582 = SrcGlobal.from(SrcTest.sid__295("users"));
            boolean t_4157 = q__582.toSql().toString().equals("SELECT * FROM users");
            Supplier<String> fn__4152 = () -> "bare query";
            test_43.assert_(t_4157, fn__4152);
        } finally {
            test_43.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void selectRestrictsColumns__936() {
        Test test_44 = new Test();
        try {
            SafeIdentifier t_4143 = SrcTest.sid__295("users");
            SafeIdentifier t_4144 = SrcTest.sid__295("id");
            SafeIdentifier t_4145 = SrcTest.sid__295("name");
            Query q__584 = SrcGlobal.from(t_4143).select(List.of(t_4144, t_4145));
            boolean t_4150 = q__584.toSql().toString().equals("SELECT id, name FROM users");
            Supplier<String> fn__4142 = () -> "select columns";
            test_44.assert_(t_4150, fn__4142);
        } finally {
            test_44.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void whereAddsConditionWithIntValue__937() {
        Test test_45 = new Test();
        try {
            SafeIdentifier t_4131 = SrcTest.sid__295("users");
            SqlBuilder t_4132 = new SqlBuilder();
            t_4132.appendSafe("age > ");
            t_4132.appendInt32(18);
            SqlFragment t_4135 = t_4132.getAccumulated();
            Query q__586 = SrcGlobal.from(t_4131).where(t_4135);
            boolean t_4140 = q__586.toSql().toString().equals("SELECT * FROM users WHERE age > 18");
            Supplier<String> fn__4130 = () -> "where int";
            test_45.assert_(t_4140, fn__4130);
        } finally {
            test_45.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void whereAddsConditionWithBoolValue__939() {
        Test test_46 = new Test();
        try {
            SafeIdentifier t_4119 = SrcTest.sid__295("users");
            SqlBuilder t_4120 = new SqlBuilder();
            t_4120.appendSafe("active = ");
            t_4120.appendBoolean(true);
            SqlFragment t_4123 = t_4120.getAccumulated();
            Query q__588 = SrcGlobal.from(t_4119).where(t_4123);
            boolean t_4128 = q__588.toSql().toString().equals("SELECT * FROM users WHERE active = TRUE");
            Supplier<String> fn__4118 = () -> "where bool";
            test_46.assert_(t_4128, fn__4118);
        } finally {
            test_46.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void chainedWhereUsesAnd__941() {
        Test test_47 = new Test();
        try {
            SafeIdentifier t_4102 = SrcTest.sid__295("users");
            SqlBuilder t_4103 = new SqlBuilder();
            t_4103.appendSafe("age > ");
            t_4103.appendInt32(18);
            SqlFragment t_4106 = t_4103.getAccumulated();
            Query t_4107 = SrcGlobal.from(t_4102).where(t_4106);
            SqlBuilder t_4108 = new SqlBuilder();
            t_4108.appendSafe("active = ");
            t_4108.appendBoolean(true);
            Query q__590 = t_4107.where(t_4108.getAccumulated());
            boolean t_4116 = q__590.toSql().toString().equals("SELECT * FROM users WHERE age > 18 AND active = TRUE");
            Supplier<String> fn__4101 = () -> "chained where";
            test_47.assert_(t_4116, fn__4101);
        } finally {
            test_47.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void orderByAsc__944() {
        Test test_48 = new Test();
        try {
            SafeIdentifier t_4093 = SrcTest.sid__295("users");
            SafeIdentifier t_4094 = SrcTest.sid__295("name");
            Query q__592 = SrcGlobal.from(t_4093).orderBy(t_4094, true);
            boolean t_4099 = q__592.toSql().toString().equals("SELECT * FROM users ORDER BY name ASC");
            Supplier<String> fn__4092 = () -> "order asc";
            test_48.assert_(t_4099, fn__4092);
        } finally {
            test_48.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void orderByDesc__945() {
        Test test_49 = new Test();
        try {
            SafeIdentifier t_4084 = SrcTest.sid__295("users");
            SafeIdentifier t_4085 = SrcTest.sid__295("created_at");
            Query q__594 = SrcGlobal.from(t_4084).orderBy(t_4085, false);
            boolean t_4090 = q__594.toSql().toString().equals("SELECT * FROM users ORDER BY created_at DESC");
            Supplier<String> fn__4083 = () -> "order desc";
            test_49.assert_(t_4090, fn__4083);
        } finally {
            test_49.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void limitAndOffset__946() {
        Test test_50 = new Test();
        try {
            Query t_2195;
            t_2195 = SrcGlobal.from(SrcTest.sid__295("users")).limit(10);
            Query t_2196;
            t_2196 = t_2195.offset(20);
            Query q__596 = t_2196;
            boolean t_4081 = q__596.toSql().toString().equals("SELECT * FROM users LIMIT 10 OFFSET 20");
            Supplier<String> fn__4076 = () -> "limit/offset";
            test_50.assert_(t_4081, fn__4076);
        } finally {
            test_50.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void limitBubblesOnNegative__947() {
        Test test_51 = new Test();
        try {
            boolean didBubble__598;
            boolean didBubble_4862;
            try {
                SrcGlobal.from(SrcTest.sid__295("users")).limit(-1);
                didBubble_4862 = false;
            } catch (RuntimeException ignored$7) {
                didBubble_4862 = true;
            }
            didBubble__598 = didBubble_4862;
            Supplier<String> fn__4072 = () -> "negative limit should bubble";
            test_51.assert_(didBubble__598, fn__4072);
        } finally {
            test_51.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void offsetBubblesOnNegative__948() {
        Test test_52 = new Test();
        try {
            boolean didBubble__600;
            boolean didBubble_4863;
            try {
                SrcGlobal.from(SrcTest.sid__295("users")).offset(-1);
                didBubble_4863 = false;
            } catch (RuntimeException ignored$8) {
                didBubble_4863 = true;
            }
            didBubble__600 = didBubble_4863;
            Supplier<String> fn__4068 = () -> "negative offset should bubble";
            test_52.assert_(didBubble__600, fn__4068);
        } finally {
            test_52.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void complexComposedQuery__949() {
        Test test_53 = new Test();
        try {
            int minAge__602 = 21;
            SafeIdentifier t_4046 = SrcTest.sid__295("users");
            SafeIdentifier t_4047 = SrcTest.sid__295("id");
            SafeIdentifier t_4048 = SrcTest.sid__295("name");
            SafeIdentifier t_4049 = SrcTest.sid__295("email");
            Query t_4050 = SrcGlobal.from(t_4046).select(List.of(t_4047, t_4048, t_4049));
            SqlBuilder t_4051 = new SqlBuilder();
            t_4051.appendSafe("age >= ");
            t_4051.appendInt32(21);
            Query t_4055 = t_4050.where(t_4051.getAccumulated());
            SqlBuilder t_4056 = new SqlBuilder();
            t_4056.appendSafe("active = ");
            t_4056.appendBoolean(true);
            Query t_2181;
            t_2181 = t_4055.where(t_4056.getAccumulated()).orderBy(SrcTest.sid__295("name"), true).limit(25);
            Query t_2182;
            t_2182 = t_2181.offset(0);
            Query q__603 = t_2182;
            boolean t_4066 = q__603.toSql().toString().equals("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
            Supplier<String> fn__4045 = () -> "complex query";
            test_53.assert_(t_4066, fn__4045);
        } finally {
            test_53.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeToSqlAppliesDefaultLimitWhenNoneSet__952() {
        Test test_54 = new Test();
        try {
            Query q__605 = SrcGlobal.from(SrcTest.sid__295("users"));
            SqlFragment t_2158;
            t_2158 = q__605.safeToSql(100);
            SqlFragment t_2159 = t_2158;
            String s__606 = t_2159.toString();
            boolean t_4043 = s__606.equals("SELECT * FROM users LIMIT 100");
            Supplier<String> fn__4039 = () -> "should have limit: " + s__606;
            test_54.assert_(t_4043, fn__4039);
        } finally {
            test_54.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeToSqlRespectsExplicitLimit__953() {
        Test test_55 = new Test();
        try {
            Query t_2150;
            t_2150 = SrcGlobal.from(SrcTest.sid__295("users")).limit(5);
            Query q__608 = t_2150;
            SqlFragment t_2153;
            t_2153 = q__608.safeToSql(100);
            SqlFragment t_2154 = t_2153;
            String s__609 = t_2154.toString();
            boolean t_4037 = s__609.equals("SELECT * FROM users LIMIT 5");
            Supplier<String> fn__4033 = () -> "explicit limit preserved: " + s__609;
            test_55.assert_(t_4037, fn__4033);
        } finally {
            test_55.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeToSqlBubblesOnNegativeDefaultLimit__954() {
        Test test_56 = new Test();
        try {
            boolean didBubble__611;
            boolean didBubble_4864;
            try {
                SrcGlobal.from(SrcTest.sid__295("users")).safeToSql(-1);
                didBubble_4864 = false;
            } catch (RuntimeException ignored$9) {
                didBubble_4864 = true;
            }
            didBubble__611 = didBubble_4864;
            Supplier<String> fn__4029 = () -> "negative defaultLimit should bubble";
            test_56.assert_(didBubble__611, fn__4029);
        } finally {
            test_56.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void whereWithInjectionAttemptInStringValueIsEscaped__955() {
        Test test_57 = new Test();
        try {
            String evil__613 = "'; DROP TABLE users; --";
            SafeIdentifier t_4013 = SrcTest.sid__295("users");
            SqlBuilder t_4014 = new SqlBuilder();
            t_4014.appendSafe("name = ");
            t_4014.appendString("'; DROP TABLE users; --");
            SqlFragment t_4017 = t_4014.getAccumulated();
            Query q__614 = SrcGlobal.from(t_4013).where(t_4017);
            String s__615 = q__614.toSql().toString();
            boolean t_4022 = s__615.indexOf("''") >= 0;
            Supplier<String> fn__4012 = () -> "quotes must be doubled: " + s__615;
            test_57.assert_(t_4022, fn__4012);
            boolean t_4026 = s__615.indexOf("SELECT * FROM users WHERE name =") >= 0;
            Supplier<String> fn__4011 = () -> "structure intact: " + s__615;
            test_57.assert_(t_4026, fn__4011);
        } finally {
            test_57.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__957() {
        Test test_58 = new Test();
        try {
            String attack__617 = "users; DROP TABLE users; --";
            boolean didBubble__618;
            boolean didBubble_4865;
            try {
                SrcGlobal.safeIdentifier("users; DROP TABLE users; --");
                didBubble_4865 = false;
            } catch (RuntimeException ignored$10) {
                didBubble_4865 = true;
            }
            didBubble__618 = didBubble_4865;
            Supplier<String> fn__4008 = () -> "metacharacter-containing name must be rejected at construction";
            test_58.assert_(didBubble__618, fn__4008);
        } finally {
            test_58.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeIdentifierAcceptsValidNames__958() {
        Test test_65 = new Test();
        try {
            SafeIdentifier t_2123;
            t_2123 = SrcGlobal.safeIdentifier("user_name");
            SafeIdentifier id__656 = t_2123;
            boolean t_4006 = id__656.getSqlValue().equals("user_name");
            Supplier<String> fn__4003 = () -> "value should round-trip";
            test_65.assert_(t_4006, fn__4003);
        } finally {
            test_65.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeIdentifierRejectsEmptyString__959() {
        Test test_66 = new Test();
        try {
            boolean didBubble__658;
            boolean didBubble_4866;
            try {
                SrcGlobal.safeIdentifier("");
                didBubble_4866 = false;
            } catch (RuntimeException ignored$11) {
                didBubble_4866 = true;
            }
            didBubble__658 = didBubble_4866;
            Supplier<String> fn__4000 = () -> "empty string should bubble";
            test_66.assert_(didBubble__658, fn__4000);
        } finally {
            test_66.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeIdentifierRejectsLeadingDigit__960() {
        Test test_67 = new Test();
        try {
            boolean didBubble__660;
            boolean didBubble_4867;
            try {
                SrcGlobal.safeIdentifier("1col");
                didBubble_4867 = false;
            } catch (RuntimeException ignored$12) {
                didBubble_4867 = true;
            }
            didBubble__660 = didBubble_4867;
            Supplier<String> fn__3997 = () -> "leading digit should bubble";
            test_67.assert_(didBubble__660, fn__3997);
        } finally {
            test_67.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void safeIdentifierRejectsSqlMetacharacters__961() {
        Test test_68 = new Test();
        try {
            List<String> cases__662 = List.of("name); DROP TABLE", "col'", "a b", "a-b", "a.b", "a;b");
            Consumer<String> fn__3994 = c__663 -> {
                boolean didBubble__664;
                boolean didBubble_4868;
                try {
                    SrcGlobal.safeIdentifier(c__663);
                    didBubble_4868 = false;
                } catch (RuntimeException ignored$13) {
                    didBubble_4868 = true;
                }
                didBubble__664 = didBubble_4868;
                Supplier<String> fn__3991 = () -> "should reject: " + c__663;
                test_68.assert_(didBubble__664, fn__3991);
            };
            cases__662.forEach(fn__3994);
        } finally {
            test_68.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void tableDefFieldLookupFound__962() {
        Test test_69 = new Test();
        try {
            SafeIdentifier t_2100;
            t_2100 = SrcGlobal.safeIdentifier("users");
            SafeIdentifier t_2101 = t_2100;
            SafeIdentifier t_2102;
            t_2102 = SrcGlobal.safeIdentifier("name");
            SafeIdentifier t_2103 = t_2102;
            StringField t_3981 = new StringField();
            FieldDef t_3982 = new FieldDef(t_2103, t_3981, false);
            SafeIdentifier t_2106;
            t_2106 = SrcGlobal.safeIdentifier("age");
            SafeIdentifier t_2107 = t_2106;
            IntField t_3983 = new IntField();
            FieldDef t_3984 = new FieldDef(t_2107, t_3983, false);
            TableDef td__666 = new TableDef(t_2101, List.of(t_3982, t_3984));
            FieldDef t_2111;
            t_2111 = td__666.field("age");
            FieldDef f__667 = t_2111;
            boolean t_3989 = f__667.getName().getSqlValue().equals("age");
            Supplier<String> fn__3980 = () -> "should find age field";
            test_69.assert_(t_3989, fn__3980);
        } finally {
            test_69.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void tableDefFieldLookupNotFoundBubbles__963() {
        Test test_70 = new Test();
        try {
            SafeIdentifier t_2091;
            t_2091 = SrcGlobal.safeIdentifier("users");
            SafeIdentifier t_2092 = t_2091;
            SafeIdentifier t_2093;
            t_2093 = SrcGlobal.safeIdentifier("name");
            SafeIdentifier t_2094 = t_2093;
            StringField t_3975 = new StringField();
            FieldDef t_3976 = new FieldDef(t_2094, t_3975, false);
            TableDef td__669 = new TableDef(t_2092, List.of(t_3976));
            boolean didBubble__670;
            boolean didBubble_4869;
            try {
                td__669.field("nonexistent");
                didBubble_4869 = false;
            } catch (RuntimeException ignored$14) {
                didBubble_4869 = true;
            }
            didBubble__670 = didBubble_4869;
            Supplier<String> fn__3974 = () -> "unknown field should bubble";
            test_70.assert_(didBubble__670, fn__3974);
        } finally {
            test_70.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void fieldDefNullableFlag__964() {
        Test test_71 = new Test();
        try {
            SafeIdentifier t_2079;
            t_2079 = SrcGlobal.safeIdentifier("email");
            SafeIdentifier t_2080 = t_2079;
            StringField t_3963 = new StringField();
            FieldDef required__672 = new FieldDef(t_2080, t_3963, false);
            SafeIdentifier t_2083;
            t_2083 = SrcGlobal.safeIdentifier("bio");
            SafeIdentifier t_2084 = t_2083;
            StringField t_3965 = new StringField();
            FieldDef optional__673 = new FieldDef(t_2084, t_3965, true);
            boolean t_3969 = !required__672.isNullable();
            Supplier<String> fn__3962 = () -> "required field should not be nullable";
            test_71.assert_(t_3969, fn__3962);
            boolean t_3971 = optional__673.isNullable();
            Supplier<String> fn__3961 = () -> "optional field should be nullable";
            test_71.assert_(t_3971, fn__3961);
        } finally {
            test_71.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void stringEscaping__965() {
        Test test_73 = new Test();
        try {
            Function<String, String> build__797 = name__799 -> {
                SqlBuilder t_3943 = new SqlBuilder();
                t_3943.appendSafe("select * from hi where name = ");
                t_3943.appendString(name__799);
                return t_3943.getAccumulated().toString();
            };
            Function<String, String> buildWrong__798 = name__801 -> "select * from hi where name = '" + name__801 + "'";
            String actual_967 = build__797.apply("world");
            boolean t_3953 = actual_967.equals("select * from hi where name = 'world'");
            Supplier<String> fn__3950 = () -> "expected build(\"world\") == (" + "select * from hi where name = 'world'" + ") not (" + actual_967 + ")";
            test_73.assert_(t_3953, fn__3950);
            String bobbyTables__803 = "Robert'); drop table hi;--";
            String actual_969 = build__797.apply("Robert'); drop table hi;--");
            boolean t_3957 = actual_969.equals("select * from hi where name = 'Robert''); drop table hi;--'");
            Supplier<String> fn__3949 = () -> "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual_969 + ")";
            test_73.assert_(t_3957, fn__3949);
            Supplier<String> fn__3948 = () -> "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
            test_73.assert_(true, fn__3948);
        } finally {
            test_73.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void stringEdgeCases__973() {
        Test test_74 = new Test();
        try {
            SqlBuilder t_3911 = new SqlBuilder();
            t_3911.appendSafe("v = ");
            t_3911.appendString("");
            String actual_974 = t_3911.getAccumulated().toString();
            boolean t_3917 = actual_974.equals("v = ''");
            Supplier<String> fn__3910 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (" + "v = ''" + ") not (" + actual_974 + ")";
            test_74.assert_(t_3917, fn__3910);
            SqlBuilder t_3919 = new SqlBuilder();
            t_3919.appendSafe("v = ");
            t_3919.appendString("a''b");
            String actual_977 = t_3919.getAccumulated().toString();
            boolean t_3925 = actual_977.equals("v = 'a''''b'");
            Supplier<String> fn__3909 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (" + "v = 'a''''b'" + ") not (" + actual_977 + ")";
            test_74.assert_(t_3925, fn__3909);
            SqlBuilder t_3927 = new SqlBuilder();
            t_3927.appendSafe("v = ");
            t_3927.appendString("Hello \u4e16\u754c");
            String actual_980 = t_3927.getAccumulated().toString();
            boolean t_3933 = actual_980.equals("v = 'Hello \u4e16\u754c'");
            Supplier<String> fn__3908 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello \u4e16\u754c\").toString() == (" + "v = 'Hello \u4e16\u754c'" + ") not (" + actual_980 + ")";
            test_74.assert_(t_3933, fn__3908);
            SqlBuilder t_3935 = new SqlBuilder();
            t_3935.appendSafe("v = ");
            t_3935.appendString("Line1\nLine2");
            String actual_983 = t_3935.getAccumulated().toString();
            boolean t_3941 = actual_983.equals("v = 'Line1\nLine2'");
            Supplier<String> fn__3907 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (" + "v = 'Line1\nLine2'" + ") not (" + actual_983 + ")";
            test_74.assert_(t_3941, fn__3907);
        } finally {
            test_74.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void numbersAndBooleans__986() {
        Test test_75 = new Test();
        try {
            SqlBuilder t_3882 = new SqlBuilder();
            t_3882.appendSafe("select ");
            t_3882.appendInt32(42);
            t_3882.appendSafe(", ");
            t_3882.appendInt64(43);
            t_3882.appendSafe(", ");
            t_3882.appendFloat64(19.99D);
            t_3882.appendSafe(", ");
            t_3882.appendBoolean(true);
            t_3882.appendSafe(", ");
            t_3882.appendBoolean(false);
            String actual_987 = t_3882.getAccumulated().toString();
            boolean t_3896 = actual_987.equals("select 42, 43, 19.99, TRUE, FALSE");
            Supplier<String> fn__3881 = () -> "expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (" + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual_987 + ")";
            test_75.assert_(t_3896, fn__3881);
            LocalDate t_2024;
            t_2024 = LocalDate.of(2024, 12, 25);
            LocalDate date__806 = t_2024;
            SqlBuilder t_3898 = new SqlBuilder();
            t_3898.appendSafe("insert into t values (");
            t_3898.appendDate(date__806);
            t_3898.appendSafe(")");
            String actual_990 = t_3898.getAccumulated().toString();
            boolean t_3905 = actual_990.equals("insert into t values ('2024-12-25')");
            Supplier<String> fn__3880 = () -> "expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (" + "insert into t values ('2024-12-25')" + ") not (" + actual_990 + ")";
            test_75.assert_(t_3905, fn__3880);
        } finally {
            test_75.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void lists__993() {
        Test test_76 = new Test();
        try {
            SqlBuilder t_3826 = new SqlBuilder();
            t_3826.appendSafe("v IN (");
            t_3826.appendStringList(List.of("a", "b", "c'd"));
            t_3826.appendSafe(")");
            String actual_994 = t_3826.getAccumulated().toString();
            boolean t_3833 = actual_994.equals("v IN ('a', 'b', 'c''d')");
            Supplier<String> fn__3825 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual_994 + ")";
            test_76.assert_(t_3833, fn__3825);
            SqlBuilder t_3835 = new SqlBuilder();
            t_3835.appendSafe("v IN (");
            t_3835.appendInt32List(List.of(1, 2, 3));
            t_3835.appendSafe(")");
            String actual_997 = t_3835.getAccumulated().toString();
            boolean t_3842 = actual_997.equals("v IN (1, 2, 3)");
            Supplier<String> fn__3824 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (" + "v IN (1, 2, 3)" + ") not (" + actual_997 + ")";
            test_76.assert_(t_3842, fn__3824);
            SqlBuilder t_3844 = new SqlBuilder();
            t_3844.appendSafe("v IN (");
            t_3844.appendInt64List(List.of(1, 2));
            t_3844.appendSafe(")");
            String actual_1000 = t_3844.getAccumulated().toString();
            boolean t_3851 = actual_1000.equals("v IN (1, 2)");
            Supplier<String> fn__3823 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (" + "v IN (1, 2)" + ") not (" + actual_1000 + ")";
            test_76.assert_(t_3851, fn__3823);
            SqlBuilder t_3853 = new SqlBuilder();
            t_3853.appendSafe("v IN (");
            t_3853.appendFloat64List(List.of(1.0D, 2.0D));
            t_3853.appendSafe(")");
            String actual_1003 = t_3853.getAccumulated().toString();
            boolean t_3860 = actual_1003.equals("v IN (1.0, 2.0)");
            Supplier<String> fn__3822 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (" + "v IN (1.0, 2.0)" + ") not (" + actual_1003 + ")";
            test_76.assert_(t_3860, fn__3822);
            SqlBuilder t_3862 = new SqlBuilder();
            t_3862.appendSafe("v IN (");
            t_3862.appendBooleanList(List.of(true, false));
            t_3862.appendSafe(")");
            String actual_1006 = t_3862.getAccumulated().toString();
            boolean t_3869 = actual_1006.equals("v IN (TRUE, FALSE)");
            Supplier<String> fn__3821 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (" + "v IN (TRUE, FALSE)" + ") not (" + actual_1006 + ")";
            test_76.assert_(t_3869, fn__3821);
            LocalDate t_1996;
            t_1996 = LocalDate.of(2024, 1, 1);
            LocalDate t_1997 = t_1996;
            LocalDate t_1998;
            t_1998 = LocalDate.of(2024, 12, 25);
            LocalDate t_1999 = t_1998;
            List<LocalDate> dates__808 = List.of(t_1997, t_1999);
            SqlBuilder t_3871 = new SqlBuilder();
            t_3871.appendSafe("v IN (");
            t_3871.appendDateList(dates__808);
            t_3871.appendSafe(")");
            String actual_1009 = t_3871.getAccumulated().toString();
            boolean t_3878 = actual_1009.equals("v IN ('2024-01-01', '2024-12-25')");
            Supplier<String> fn__3820 = () -> "expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (" + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual_1009 + ")";
            test_76.assert_(t_3878, fn__3820);
        } finally {
            test_76.softFailToHard();
        }
    }
    @org.junit.jupiter.api.Test public void nesting__1012() {
        Test test_77 = new Test();
        try {
            String name__810 = "Someone";
            SqlBuilder t_3789 = new SqlBuilder();
            t_3789.appendSafe("where p.last_name = ");
            t_3789.appendString("Someone");
            SqlFragment condition__811 = t_3789.getAccumulated();
            SqlBuilder t_3793 = new SqlBuilder();
            t_3793.appendSafe("select p.id from person p ");
            t_3793.appendFragment(condition__811);
            String actual_1014 = t_3793.getAccumulated().toString();
            boolean t_3799 = actual_1014.equals("select p.id from person p where p.last_name = 'Someone'");
            Supplier<String> fn__3788 = () -> "expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_1014 + ")";
            test_77.assert_(t_3799, fn__3788);
            SqlBuilder t_3801 = new SqlBuilder();
            t_3801.appendSafe("select p.id from person p ");
            t_3801.appendPart(condition__811.toSource());
            String actual_1017 = t_3801.getAccumulated().toString();
            boolean t_3808 = actual_1017.equals("select p.id from person p where p.last_name = 'Someone'");
            Supplier<String> fn__3787 = () -> "expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual_1017 + ")";
            test_77.assert_(t_3808, fn__3787);
            List<SqlPart> parts__812 = List.of(new SqlString("a'b"), new SqlInt32(3));
            SqlBuilder t_3812 = new SqlBuilder();
            t_3812.appendSafe("select ");
            t_3812.appendPartList(parts__812);
            String actual_1020 = t_3812.getAccumulated().toString();
            boolean t_3818 = actual_1020.equals("select 'a''b', 3");
            Supplier<String> fn__3786 = () -> "expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (" + "select 'a''b', 3" + ") not (" + actual_1020 + ")";
            test_77.assert_(t_3818, fn__3786);
        } finally {
            test_77.softFailToHard();
        }
    }
}
