using U = Microsoft.VisualStudio.TestTools.UnitTesting;
using S0 = Orm.Src;
using S1 = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
using T = TemperLang.Std.Testing;
namespace Orm.Src
{
    [U::TestClass]
    public class SrcTests
    {
        internal static ISafeIdentifier csid__293(string name__438)
        {
            ISafeIdentifier t___2624;
            t___2624 = S0::SrcGlobal.SafeIdentifier(name__438);
            return t___2624;
        }
        internal static TableDef userTable__294()
        {
            return new TableDef(csid__293("users"), C::Listed.CreateReadOnlyList<FieldDef>(new FieldDef(csid__293("name"), new StringField(), false), new FieldDef(csid__293("email"), new StringField(), false), new FieldDef(csid__293("age"), new IntField(), true), new FieldDef(csid__293("score"), new FloatField(), true), new FieldDef(csid__293("active"), new BoolField(), true)));
        }
        [U::TestMethod]
        public void castWhitelistsAllowedFields__888()
        {
            T::Test test___20 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__442 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("email", "alice@example.com"), new G::KeyValuePair<string, string>("admin", "true")));
                TableDef t___4547 = userTable__294();
                ISafeIdentifier t___4548 = csid__293("name");
                ISafeIdentifier t___4549 = csid__293("email");
                IChangeset cs__443 = S0::SrcGlobal.Changeset(t___4547, params__442).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4548, t___4549));
                bool t___4552 = C::Mapped.ContainsKey(cs__443.Changes, "name");
                string fn__4542()
                {
                    return "name should be in changes";
                }
                test___20.Assert(t___4552, (S1::Func<string>) fn__4542);
                bool t___4556 = C::Mapped.ContainsKey(cs__443.Changes, "email");
                string fn__4541()
                {
                    return "email should be in changes";
                }
                test___20.Assert(t___4556, (S1::Func<string>) fn__4541);
                bool t___4562 = !C::Mapped.ContainsKey(cs__443.Changes, "admin");
                string fn__4540()
                {
                    return "admin must be dropped (not in whitelist)";
                }
                test___20.Assert(t___4562, (S1::Func<string>) fn__4540);
                bool t___4564 = cs__443.IsValid;
                string fn__4539()
                {
                    return "should still be valid";
                }
                test___20.Assert(t___4564, (S1::Func<string>) fn__4539);
            }
            finally
            {
                test___20.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void castIsReplacingNotAdditiveSecondCallResetsWhitelist__889()
        {
            T::Test test___21 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__445 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("email", "alice@example.com")));
                TableDef t___4525 = userTable__294();
                ISafeIdentifier t___4526 = csid__293("name");
                IChangeset cs__446 = S0::SrcGlobal.Changeset(t___4525, params__445).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4526)).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("email")));
                bool t___4533 = !C::Mapped.ContainsKey(cs__446.Changes, "name");
                string fn__4521()
                {
                    return "name must be excluded by second cast";
                }
                test___21.Assert(t___4533, (S1::Func<string>) fn__4521);
                bool t___4536 = C::Mapped.ContainsKey(cs__446.Changes, "email");
                string fn__4520()
                {
                    return "email should be present";
                }
                test___21.Assert(t___4536, (S1::Func<string>) fn__4520);
            }
            finally
            {
                test___21.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void castIgnoresEmptyStringValues__890()
        {
            T::Test test___22 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__448 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", ""), new G::KeyValuePair<string, string>("email", "bob@example.com")));
                TableDef t___4507 = userTable__294();
                ISafeIdentifier t___4508 = csid__293("name");
                ISafeIdentifier t___4509 = csid__293("email");
                IChangeset cs__449 = S0::SrcGlobal.Changeset(t___4507, params__448).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4508, t___4509));
                bool t___4514 = !C::Mapped.ContainsKey(cs__449.Changes, "name");
                string fn__4503()
                {
                    return "empty name should not be in changes";
                }
                test___22.Assert(t___4514, (S1::Func<string>) fn__4503);
                bool t___4517 = C::Mapped.ContainsKey(cs__449.Changes, "email");
                string fn__4502()
                {
                    return "email should be in changes";
                }
                test___22.Assert(t___4517, (S1::Func<string>) fn__4502);
            }
            finally
            {
                test___22.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateRequiredPassesWhenFieldPresent__891()
        {
            T::Test test___23 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__451 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice")));
                TableDef t___4489 = userTable__294();
                ISafeIdentifier t___4490 = csid__293("name");
                IChangeset cs__452 = S0::SrcGlobal.Changeset(t___4489, params__451).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4490)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name")));
                bool t___4494 = cs__452.IsValid;
                string fn__4486()
                {
                    return "should be valid";
                }
                test___23.Assert(t___4494, (S1::Func<string>) fn__4486);
                bool t___4500 = cs__452.Errors.Count == 0;
                string fn__4485()
                {
                    return "no errors expected";
                }
                test___23.Assert(t___4500, (S1::Func<string>) fn__4485);
            }
            finally
            {
                test___23.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateRequiredFailsWhenFieldMissing__892()
        {
            T::Test test___24 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__454 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>());
                TableDef t___4465 = userTable__294();
                ISafeIdentifier t___4466 = csid__293("name");
                IChangeset cs__455 = S0::SrcGlobal.Changeset(t___4465, params__454).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4466)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name")));
                bool t___4472 = !cs__455.IsValid;
                string fn__4463()
                {
                    return "should be invalid";
                }
                test___24.Assert(t___4472, (S1::Func<string>) fn__4463);
                bool t___4477 = cs__455.Errors.Count == 1;
                string fn__4462()
                {
                    return "should have one error";
                }
                test___24.Assert(t___4477, (S1::Func<string>) fn__4462);
                bool t___4483 = cs__455.Errors[0].Field == "name";
                string fn__4461()
                {
                    return "error should name the field";
                }
                test___24.Assert(t___4483, (S1::Func<string>) fn__4461);
            }
            finally
            {
                test___24.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateLengthPassesWithinRange__893()
        {
            T::Test test___25 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__457 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice")));
                TableDef t___4453 = userTable__294();
                ISafeIdentifier t___4454 = csid__293("name");
                IChangeset cs__458 = S0::SrcGlobal.Changeset(t___4453, params__457).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4454)).ValidateLength(csid__293("name"), 2, 50);
                bool t___4458 = cs__458.IsValid;
                string fn__4450()
                {
                    return "should be valid";
                }
                test___25.Assert(t___4458, (S1::Func<string>) fn__4450);
            }
            finally
            {
                test___25.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateLengthFailsWhenTooShort__894()
        {
            T::Test test___26 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__460 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "A")));
                TableDef t___4441 = userTable__294();
                ISafeIdentifier t___4442 = csid__293("name");
                IChangeset cs__461 = S0::SrcGlobal.Changeset(t___4441, params__460).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4442)).ValidateLength(csid__293("name"), 2, 50);
                bool t___4448 = !cs__461.IsValid;
                string fn__4438()
                {
                    return "should be invalid";
                }
                test___26.Assert(t___4448, (S1::Func<string>) fn__4438);
            }
            finally
            {
                test___26.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateLengthFailsWhenTooLong__895()
        {
            T::Test test___27 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__463 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "ABCDEFGHIJKLMNOPQRSTUVWXYZ")));
                TableDef t___4429 = userTable__294();
                ISafeIdentifier t___4430 = csid__293("name");
                IChangeset cs__464 = S0::SrcGlobal.Changeset(t___4429, params__463).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4430)).ValidateLength(csid__293("name"), 2, 10);
                bool t___4436 = !cs__464.IsValid;
                string fn__4426()
                {
                    return "should be invalid";
                }
                test___27.Assert(t___4436, (S1::Func<string>) fn__4426);
            }
            finally
            {
                test___27.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateIntPassesForValidInteger__896()
        {
            T::Test test___28 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__466 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("age", "30")));
                TableDef t___4418 = userTable__294();
                ISafeIdentifier t___4419 = csid__293("age");
                IChangeset cs__467 = S0::SrcGlobal.Changeset(t___4418, params__466).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4419)).ValidateInt(csid__293("age"));
                bool t___4423 = cs__467.IsValid;
                string fn__4415()
                {
                    return "should be valid";
                }
                test___28.Assert(t___4423, (S1::Func<string>) fn__4415);
            }
            finally
            {
                test___28.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateIntFailsForNonInteger__897()
        {
            T::Test test___29 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__469 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("age", "not-a-number")));
                TableDef t___4406 = userTable__294();
                ISafeIdentifier t___4407 = csid__293("age");
                IChangeset cs__470 = S0::SrcGlobal.Changeset(t___4406, params__469).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4407)).ValidateInt(csid__293("age"));
                bool t___4413 = !cs__470.IsValid;
                string fn__4403()
                {
                    return "should be invalid";
                }
                test___29.Assert(t___4413, (S1::Func<string>) fn__4403);
            }
            finally
            {
                test___29.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateFloatPassesForValidFloat__898()
        {
            T::Test test___30 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__472 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("score", "9.5")));
                TableDef t___4395 = userTable__294();
                ISafeIdentifier t___4396 = csid__293("score");
                IChangeset cs__473 = S0::SrcGlobal.Changeset(t___4395, params__472).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4396)).ValidateFloat(csid__293("score"));
                bool t___4400 = cs__473.IsValid;
                string fn__4392()
                {
                    return "should be valid";
                }
                test___30.Assert(t___4400, (S1::Func<string>) fn__4392);
            }
            finally
            {
                test___30.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateInt64_passesForValid64_bitInteger__899()
        {
            T::Test test___31 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__475 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("age", "9999999999")));
                TableDef t___4384 = userTable__294();
                ISafeIdentifier t___4385 = csid__293("age");
                IChangeset cs__476 = S0::SrcGlobal.Changeset(t___4384, params__475).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4385)).ValidateInt64(csid__293("age"));
                bool t___4389 = cs__476.IsValid;
                string fn__4381()
                {
                    return "should be valid";
                }
                test___31.Assert(t___4389, (S1::Func<string>) fn__4381);
            }
            finally
            {
                test___31.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateInt64_failsForNonInteger__900()
        {
            T::Test test___32 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__478 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("age", "not-a-number")));
                TableDef t___4372 = userTable__294();
                ISafeIdentifier t___4373 = csid__293("age");
                IChangeset cs__479 = S0::SrcGlobal.Changeset(t___4372, params__478).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4373)).ValidateInt64(csid__293("age"));
                bool t___4379 = !cs__479.IsValid;
                string fn__4369()
                {
                    return "should be invalid";
                }
                test___32.Assert(t___4379, (S1::Func<string>) fn__4369);
            }
            finally
            {
                test___32.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateBoolAcceptsTrue1_yesOn__901()
        {
            T::Test test___33 = new T::Test();
            try
            {
                void fn__4366(string v__481)
                {
                    G::IReadOnlyDictionary<string, string> params__482 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("active", v__481)));
                    TableDef t___4358 = userTable__294();
                    ISafeIdentifier t___4359 = csid__293("active");
                    IChangeset cs__483 = S0::SrcGlobal.Changeset(t___4358, params__482).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4359)).ValidateBool(csid__293("active"));
                    bool t___4363 = cs__483.IsValid;
                    string fn__4355()
                    {
                        return "should accept: " + v__481;
                    }
                    test___33.Assert(t___4363, (S1::Func<string>) fn__4355);
                }
                C::Listed.ForEach(C::Listed.CreateReadOnlyList<string>("true", "1", "yes", "on"), (S1::Action<string>) fn__4366);
            }
            finally
            {
                test___33.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateBoolAcceptsFalse0_noOff__902()
        {
            T::Test test___34 = new T::Test();
            try
            {
                void fn__4352(string v__485)
                {
                    G::IReadOnlyDictionary<string, string> params__486 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("active", v__485)));
                    TableDef t___4344 = userTable__294();
                    ISafeIdentifier t___4345 = csid__293("active");
                    IChangeset cs__487 = S0::SrcGlobal.Changeset(t___4344, params__486).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4345)).ValidateBool(csid__293("active"));
                    bool t___4349 = cs__487.IsValid;
                    string fn__4341()
                    {
                        return "should accept: " + v__485;
                    }
                    test___34.Assert(t___4349, (S1::Func<string>) fn__4341);
                }
                C::Listed.ForEach(C::Listed.CreateReadOnlyList<string>("false", "0", "no", "off"), (S1::Action<string>) fn__4352);
            }
            finally
            {
                test___34.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void validateBoolRejectsAmbiguousValues__903()
        {
            T::Test test___35 = new T::Test();
            try
            {
                void fn__4338(string v__489)
                {
                    G::IReadOnlyDictionary<string, string> params__490 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("active", v__489)));
                    TableDef t___4329 = userTable__294();
                    ISafeIdentifier t___4330 = csid__293("active");
                    IChangeset cs__491 = S0::SrcGlobal.Changeset(t___4329, params__490).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4330)).ValidateBool(csid__293("active"));
                    bool t___4336 = !cs__491.IsValid;
                    string fn__4326()
                    {
                        return "should reject ambiguous: " + v__489;
                    }
                    test___35.Assert(t___4336, (S1::Func<string>) fn__4326);
                }
                C::Listed.ForEach(C::Listed.CreateReadOnlyList<string>("TRUE", "Yes", "maybe", "2", "enabled"), (S1::Action<string>) fn__4338);
            }
            finally
            {
                test___35.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlEscapesBobbyTables__904()
        {
            T::Test test___36 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__493 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Robert'); DROP TABLE users;--"), new G::KeyValuePair<string, string>("email", "bobby@evil.com")));
                TableDef t___4314 = userTable__294();
                ISafeIdentifier t___4315 = csid__293("name");
                ISafeIdentifier t___4316 = csid__293("email");
                IChangeset cs__494 = S0::SrcGlobal.Changeset(t___4314, params__493).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4315, t___4316)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name"), csid__293("email")));
                SqlFragment t___2425;
                t___2425 = cs__494.ToInsertSql();
                SqlFragment sqlFrag__495 = t___2425;
                string s__496 = sqlFrag__495.ToString();
                bool t___4323 = s__496.IndexOf("''") >= 0;
                string fn__4310()
                {
                    return "single quote must be doubled: " + s__496;
                }
                test___36.Assert(t___4323, (S1::Func<string>) fn__4310);
            }
            finally
            {
                test___36.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlProducesCorrectSqlForStringField__905()
        {
            T::Test test___37 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__498 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Alice"), new G::KeyValuePair<string, string>("email", "a@example.com")));
                TableDef t___4294 = userTable__294();
                ISafeIdentifier t___4295 = csid__293("name");
                ISafeIdentifier t___4296 = csid__293("email");
                IChangeset cs__499 = S0::SrcGlobal.Changeset(t___4294, params__498).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4295, t___4296)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name"), csid__293("email")));
                SqlFragment t___2404;
                t___2404 = cs__499.ToInsertSql();
                SqlFragment sqlFrag__500 = t___2404;
                string s__501 = sqlFrag__500.ToString();
                bool t___4303 = s__501.IndexOf("INSERT INTO users") >= 0;
                string fn__4290()
                {
                    return "has INSERT INTO: " + s__501;
                }
                test___37.Assert(t___4303, (S1::Func<string>) fn__4290);
                bool t___4307 = s__501.IndexOf("'Alice'") >= 0;
                string fn__4289()
                {
                    return "has quoted name: " + s__501;
                }
                test___37.Assert(t___4307, (S1::Func<string>) fn__4289);
            }
            finally
            {
                test___37.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlProducesCorrectSqlForIntField__906()
        {
            T::Test test___38 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__503 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Bob"), new G::KeyValuePair<string, string>("email", "b@example.com"), new G::KeyValuePair<string, string>("age", "25")));
                TableDef t___4276 = userTable__294();
                ISafeIdentifier t___4277 = csid__293("name");
                ISafeIdentifier t___4278 = csid__293("email");
                ISafeIdentifier t___4279 = csid__293("age");
                IChangeset cs__504 = S0::SrcGlobal.Changeset(t___4276, params__503).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4277, t___4278, t___4279)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name"), csid__293("email")));
                SqlFragment t___2387;
                t___2387 = cs__504.ToInsertSql();
                SqlFragment sqlFrag__505 = t___2387;
                string s__506 = sqlFrag__505.ToString();
                bool t___4286 = s__506.IndexOf("25") >= 0;
                string fn__4271()
                {
                    return "age rendered unquoted: " + s__506;
                }
                test___38.Assert(t___4286, (S1::Func<string>) fn__4271);
            }
            finally
            {
                test___38.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlBubblesOnInvalidChangeset__907()
        {
            T::Test test___39 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__508 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>());
                TableDef t___4264 = userTable__294();
                ISafeIdentifier t___4265 = csid__293("name");
                IChangeset cs__509 = S0::SrcGlobal.Changeset(t___4264, params__508).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4265)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name")));
                bool didBubble__510;
                try
                {
                    cs__509.ToInsertSql();
                    didBubble__510 = false;
                }
                catch
                {
                    didBubble__510 = true;
                }
                string fn__4262()
                {
                    return "invalid changeset should bubble";
                }
                test___39.Assert(didBubble__510, (S1::Func<string>) fn__4262);
            }
            finally
            {
                test___39.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__908()
        {
            T::Test test___40 = new T::Test();
            try
            {
                TableDef strictTable__512 = new TableDef(csid__293("posts"), C::Listed.CreateReadOnlyList<FieldDef>(new FieldDef(csid__293("title"), new StringField(), false), new FieldDef(csid__293("body"), new StringField(), true)));
                G::IReadOnlyDictionary<string, string> params__513 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("body", "hello")));
                ISafeIdentifier t___4255 = csid__293("body");
                IChangeset cs__514 = S0::SrcGlobal.Changeset(strictTable__512, params__513).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4255));
                bool t___4257 = cs__514.IsValid;
                string fn__4244()
                {
                    return "changeset should appear valid (no explicit validation run)";
                }
                test___40.Assert(t___4257, (S1::Func<string>) fn__4244);
                bool didBubble__515;
                try
                {
                    cs__514.ToInsertSql();
                    didBubble__515 = false;
                }
                catch
                {
                    didBubble__515 = true;
                }
                string fn__4243()
                {
                    return "toInsertSql should enforce nullable regardless of isValid";
                }
                test___40.Assert(didBubble__515, (S1::Func<string>) fn__4243);
            }
            finally
            {
                test___40.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toUpdateSqlProducesCorrectSql__909()
        {
            T::Test test___41 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__517 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>(new G::KeyValuePair<string, string>("name", "Bob")));
                TableDef t___4234 = userTable__294();
                ISafeIdentifier t___4235 = csid__293("name");
                IChangeset cs__518 = S0::SrcGlobal.Changeset(t___4234, params__517).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4235)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name")));
                SqlFragment t___2347;
                t___2347 = cs__518.ToUpdateSql(42);
                SqlFragment sqlFrag__519 = t___2347;
                string s__520 = sqlFrag__519.ToString();
                bool t___4241 = s__520 == "UPDATE users SET name = 'Bob' WHERE id = 42";
                string fn__4231()
                {
                    return "got: " + s__520;
                }
                test___41.Assert(t___4241, (S1::Func<string>) fn__4231);
            }
            finally
            {
                test___41.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void toUpdateSqlBubblesOnInvalidChangeset__910()
        {
            T::Test test___42 = new T::Test();
            try
            {
                G::IReadOnlyDictionary<string, string> params__522 = C::Mapped.ConstructMap(C::Listed.CreateReadOnlyList<G::KeyValuePair<string, string>>());
                TableDef t___4224 = userTable__294();
                ISafeIdentifier t___4225 = csid__293("name");
                IChangeset cs__523 = S0::SrcGlobal.Changeset(t___4224, params__522).Cast(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4225)).ValidateRequired(C::Listed.CreateReadOnlyList<ISafeIdentifier>(csid__293("name")));
                bool didBubble__524;
                try
                {
                    cs__523.ToUpdateSql(1);
                    didBubble__524 = false;
                }
                catch
                {
                    didBubble__524 = true;
                }
                string fn__4222()
                {
                    return "invalid changeset should bubble";
                }
                test___42.Assert(didBubble__524, (S1::Func<string>) fn__4222);
            }
            finally
            {
                test___42.SoftFailToHard();
            }
        }
        internal static ISafeIdentifier sid__295(string name__579)
        {
            ISafeIdentifier t___2261;
            t___2261 = S0::SrcGlobal.SafeIdentifier(name__579);
            return t___2261;
        }
        [U::TestMethod]
        public void bareFromProducesSelect__935()
        {
            T::Test test___43 = new T::Test();
            try
            {
                Query q__582 = S0::SrcGlobal.From(sid__295("users"));
                bool t___4157 = q__582.ToSql().ToString() == "SELECT * FROM users";
                string fn__4152()
                {
                    return "bare query";
                }
                test___43.Assert(t___4157, (S1::Func<string>) fn__4152);
            }
            finally
            {
                test___43.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void selectRestrictsColumns__936()
        {
            T::Test test___44 = new T::Test();
            try
            {
                ISafeIdentifier t___4143 = sid__295("users");
                ISafeIdentifier t___4144 = sid__295("id");
                ISafeIdentifier t___4145 = sid__295("name");
                Query q__584 = S0::SrcGlobal.From(t___4143).Select(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4144, t___4145));
                bool t___4150 = q__584.ToSql().ToString() == "SELECT id, name FROM users";
                string fn__4142()
                {
                    return "select columns";
                }
                test___44.Assert(t___4150, (S1::Func<string>) fn__4142);
            }
            finally
            {
                test___44.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void whereAddsConditionWithIntValue__937()
        {
            T::Test test___45 = new T::Test();
            try
            {
                ISafeIdentifier t___4131 = sid__295("users");
                SqlBuilder t___4132 = new SqlBuilder();
                t___4132.AppendSafe("age > ");
                t___4132.AppendInt32(18);
                SqlFragment t___4135 = t___4132.Accumulated;
                Query q__586 = S0::SrcGlobal.From(t___4131).Where(t___4135);
                bool t___4140 = q__586.ToSql().ToString() == "SELECT * FROM users WHERE age > 18";
                string fn__4130()
                {
                    return "where int";
                }
                test___45.Assert(t___4140, (S1::Func<string>) fn__4130);
            }
            finally
            {
                test___45.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void whereAddsConditionWithBoolValue__939()
        {
            T::Test test___46 = new T::Test();
            try
            {
                ISafeIdentifier t___4119 = sid__295("users");
                SqlBuilder t___4120 = new SqlBuilder();
                t___4120.AppendSafe("active = ");
                t___4120.AppendBoolean(true);
                SqlFragment t___4123 = t___4120.Accumulated;
                Query q__588 = S0::SrcGlobal.From(t___4119).Where(t___4123);
                bool t___4128 = q__588.ToSql().ToString() == "SELECT * FROM users WHERE active = TRUE";
                string fn__4118()
                {
                    return "where bool";
                }
                test___46.Assert(t___4128, (S1::Func<string>) fn__4118);
            }
            finally
            {
                test___46.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void chainedWhereUsesAnd__941()
        {
            T::Test test___47 = new T::Test();
            try
            {
                ISafeIdentifier t___4102 = sid__295("users");
                SqlBuilder t___4103 = new SqlBuilder();
                t___4103.AppendSafe("age > ");
                t___4103.AppendInt32(18);
                SqlFragment t___4106 = t___4103.Accumulated;
                Query t___4107 = S0::SrcGlobal.From(t___4102).Where(t___4106);
                SqlBuilder t___4108 = new SqlBuilder();
                t___4108.AppendSafe("active = ");
                t___4108.AppendBoolean(true);
                Query q__590 = t___4107.Where(t___4108.Accumulated);
                bool t___4116 = q__590.ToSql().ToString() == "SELECT * FROM users WHERE age > 18 AND active = TRUE";
                string fn__4101()
                {
                    return "chained where";
                }
                test___47.Assert(t___4116, (S1::Func<string>) fn__4101);
            }
            finally
            {
                test___47.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void orderByAsc__944()
        {
            T::Test test___48 = new T::Test();
            try
            {
                ISafeIdentifier t___4093 = sid__295("users");
                ISafeIdentifier t___4094 = sid__295("name");
                Query q__592 = S0::SrcGlobal.From(t___4093).OrderBy(t___4094, true);
                bool t___4099 = q__592.ToSql().ToString() == "SELECT * FROM users ORDER BY name ASC";
                string fn__4092()
                {
                    return "order asc";
                }
                test___48.Assert(t___4099, (S1::Func<string>) fn__4092);
            }
            finally
            {
                test___48.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void orderByDesc__945()
        {
            T::Test test___49 = new T::Test();
            try
            {
                ISafeIdentifier t___4084 = sid__295("users");
                ISafeIdentifier t___4085 = sid__295("created_at");
                Query q__594 = S0::SrcGlobal.From(t___4084).OrderBy(t___4085, false);
                bool t___4090 = q__594.ToSql().ToString() == "SELECT * FROM users ORDER BY created_at DESC";
                string fn__4083()
                {
                    return "order desc";
                }
                test___49.Assert(t___4090, (S1::Func<string>) fn__4083);
            }
            finally
            {
                test___49.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void limitAndOffset__946()
        {
            T::Test test___50 = new T::Test();
            try
            {
                Query t___2195;
                t___2195 = S0::SrcGlobal.From(sid__295("users")).Limit(10);
                Query t___2196;
                t___2196 = t___2195.Offset(20);
                Query q__596 = t___2196;
                bool t___4081 = q__596.ToSql().ToString() == "SELECT * FROM users LIMIT 10 OFFSET 20";
                string fn__4076()
                {
                    return "limit/offset";
                }
                test___50.Assert(t___4081, (S1::Func<string>) fn__4076);
            }
            finally
            {
                test___50.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void limitBubblesOnNegative__947()
        {
            T::Test test___51 = new T::Test();
            try
            {
                bool didBubble__598;
                try
                {
                    S0::SrcGlobal.From(sid__295("users")).Limit(-1);
                    didBubble__598 = false;
                }
                catch
                {
                    didBubble__598 = true;
                }
                string fn__4072()
                {
                    return "negative limit should bubble";
                }
                test___51.Assert(didBubble__598, (S1::Func<string>) fn__4072);
            }
            finally
            {
                test___51.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void offsetBubblesOnNegative__948()
        {
            T::Test test___52 = new T::Test();
            try
            {
                bool didBubble__600;
                try
                {
                    S0::SrcGlobal.From(sid__295("users")).Offset(-1);
                    didBubble__600 = false;
                }
                catch
                {
                    didBubble__600 = true;
                }
                string fn__4068()
                {
                    return "negative offset should bubble";
                }
                test___52.Assert(didBubble__600, (S1::Func<string>) fn__4068);
            }
            finally
            {
                test___52.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void complexComposedQuery__949()
        {
            T::Test test___53 = new T::Test();
            try
            {
                int minAge__602 = 21;
                ISafeIdentifier t___4046 = sid__295("users");
                ISafeIdentifier t___4047 = sid__295("id");
                ISafeIdentifier t___4048 = sid__295("name");
                ISafeIdentifier t___4049 = sid__295("email");
                Query t___4050 = S0::SrcGlobal.From(t___4046).Select(C::Listed.CreateReadOnlyList<ISafeIdentifier>(t___4047, t___4048, t___4049));
                SqlBuilder t___4051 = new SqlBuilder();
                t___4051.AppendSafe("age >= ");
                t___4051.AppendInt32(21);
                Query t___4055 = t___4050.Where(t___4051.Accumulated);
                SqlBuilder t___4056 = new SqlBuilder();
                t___4056.AppendSafe("active = ");
                t___4056.AppendBoolean(true);
                Query t___2181;
                t___2181 = t___4055.Where(t___4056.Accumulated).OrderBy(sid__295("name"), true).Limit(25);
                Query t___2182;
                t___2182 = t___2181.Offset(0);
                Query q__603 = t___2182;
                bool t___4066 = q__603.ToSql().ToString() == "SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0";
                string fn__4045()
                {
                    return "complex query";
                }
                test___53.Assert(t___4066, (S1::Func<string>) fn__4045);
            }
            finally
            {
                test___53.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeToSqlAppliesDefaultLimitWhenNoneSet__952()
        {
            T::Test test___54 = new T::Test();
            try
            {
                Query q__605 = S0::SrcGlobal.From(sid__295("users"));
                SqlFragment t___2158;
                t___2158 = q__605.SafeToSql(100);
                SqlFragment t___2159 = t___2158;
                string s__606 = t___2159.ToString();
                bool t___4043 = s__606 == "SELECT * FROM users LIMIT 100";
                string fn__4039()
                {
                    return "should have limit: " + s__606;
                }
                test___54.Assert(t___4043, (S1::Func<string>) fn__4039);
            }
            finally
            {
                test___54.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeToSqlRespectsExplicitLimit__953()
        {
            T::Test test___55 = new T::Test();
            try
            {
                Query t___2150;
                t___2150 = S0::SrcGlobal.From(sid__295("users")).Limit(5);
                Query q__608 = t___2150;
                SqlFragment t___2153;
                t___2153 = q__608.SafeToSql(100);
                SqlFragment t___2154 = t___2153;
                string s__609 = t___2154.ToString();
                bool t___4037 = s__609 == "SELECT * FROM users LIMIT 5";
                string fn__4033()
                {
                    return "explicit limit preserved: " + s__609;
                }
                test___55.Assert(t___4037, (S1::Func<string>) fn__4033);
            }
            finally
            {
                test___55.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeToSqlBubblesOnNegativeDefaultLimit__954()
        {
            T::Test test___56 = new T::Test();
            try
            {
                bool didBubble__611;
                try
                {
                    S0::SrcGlobal.From(sid__295("users")).SafeToSql(-1);
                    didBubble__611 = false;
                }
                catch
                {
                    didBubble__611 = true;
                }
                string fn__4029()
                {
                    return "negative defaultLimit should bubble";
                }
                test___56.Assert(didBubble__611, (S1::Func<string>) fn__4029);
            }
            finally
            {
                test___56.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void whereWithInjectionAttemptInStringValueIsEscaped__955()
        {
            T::Test test___57 = new T::Test();
            try
            {
                string evil__613 = "'; DROP TABLE users; --";
                ISafeIdentifier t___4013 = sid__295("users");
                SqlBuilder t___4014 = new SqlBuilder();
                t___4014.AppendSafe("name = ");
                t___4014.AppendString("'; DROP TABLE users; --");
                SqlFragment t___4017 = t___4014.Accumulated;
                Query q__614 = S0::SrcGlobal.From(t___4013).Where(t___4017);
                string s__615 = q__614.ToSql().ToString();
                bool t___4022 = s__615.IndexOf("''") >= 0;
                string fn__4012()
                {
                    return "quotes must be doubled: " + s__615;
                }
                test___57.Assert(t___4022, (S1::Func<string>) fn__4012);
                bool t___4026 = s__615.IndexOf("SELECT * FROM users WHERE name =") >= 0;
                string fn__4011()
                {
                    return "structure intact: " + s__615;
                }
                test___57.Assert(t___4026, (S1::Func<string>) fn__4011);
            }
            finally
            {
                test___57.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__957()
        {
            T::Test test___58 = new T::Test();
            try
            {
                string attack__617 = "users; DROP TABLE users; --";
                bool didBubble__618;
                try
                {
                    S0::SrcGlobal.SafeIdentifier("users; DROP TABLE users; --");
                    didBubble__618 = false;
                }
                catch
                {
                    didBubble__618 = true;
                }
                string fn__4008()
                {
                    return "metacharacter-containing name must be rejected at construction";
                }
                test___58.Assert(didBubble__618, (S1::Func<string>) fn__4008);
            }
            finally
            {
                test___58.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeIdentifierAcceptsValidNames__958()
        {
            T::Test test___65 = new T::Test();
            try
            {
                ISafeIdentifier t___2123;
                t___2123 = S0::SrcGlobal.SafeIdentifier("user_name");
                ISafeIdentifier id__656 = t___2123;
                bool t___4006 = id__656.SqlValue == "user_name";
                string fn__4003()
                {
                    return "value should round-trip";
                }
                test___65.Assert(t___4006, (S1::Func<string>) fn__4003);
            }
            finally
            {
                test___65.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeIdentifierRejectsEmptyString__959()
        {
            T::Test test___66 = new T::Test();
            try
            {
                bool didBubble__658;
                try
                {
                    S0::SrcGlobal.SafeIdentifier("");
                    didBubble__658 = false;
                }
                catch
                {
                    didBubble__658 = true;
                }
                string fn__4000()
                {
                    return "empty string should bubble";
                }
                test___66.Assert(didBubble__658, (S1::Func<string>) fn__4000);
            }
            finally
            {
                test___66.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeIdentifierRejectsLeadingDigit__960()
        {
            T::Test test___67 = new T::Test();
            try
            {
                bool didBubble__660;
                try
                {
                    S0::SrcGlobal.SafeIdentifier("1col");
                    didBubble__660 = false;
                }
                catch
                {
                    didBubble__660 = true;
                }
                string fn__3997()
                {
                    return "leading digit should bubble";
                }
                test___67.Assert(didBubble__660, (S1::Func<string>) fn__3997);
            }
            finally
            {
                test___67.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void safeIdentifierRejectsSqlMetacharacters__961()
        {
            T::Test test___68 = new T::Test();
            try
            {
                G::IReadOnlyList<string> cases__662 = C::Listed.CreateReadOnlyList<string>("name); DROP TABLE", "col'", "a b", "a-b", "a.b", "a;b");
                void fn__3994(string c__663)
                {
                    bool didBubble__664;
                    try
                    {
                        S0::SrcGlobal.SafeIdentifier(c__663);
                        didBubble__664 = false;
                    }
                    catch
                    {
                        didBubble__664 = true;
                    }
                    string fn__3991()
                    {
                        return "should reject: " + c__663;
                    }
                    test___68.Assert(didBubble__664, (S1::Func<string>) fn__3991);
                }
                C::Listed.ForEach(cases__662, (S1::Action<string>) fn__3994);
            }
            finally
            {
                test___68.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void tableDefFieldLookupFound__962()
        {
            T::Test test___69 = new T::Test();
            try
            {
                ISafeIdentifier t___2100;
                t___2100 = S0::SrcGlobal.SafeIdentifier("users");
                ISafeIdentifier t___2101 = t___2100;
                ISafeIdentifier t___2102;
                t___2102 = S0::SrcGlobal.SafeIdentifier("name");
                ISafeIdentifier t___2103 = t___2102;
                StringField t___3981 = new StringField();
                FieldDef t___3982 = new FieldDef(t___2103, t___3981, false);
                ISafeIdentifier t___2106;
                t___2106 = S0::SrcGlobal.SafeIdentifier("age");
                ISafeIdentifier t___2107 = t___2106;
                IntField t___3983 = new IntField();
                FieldDef t___3984 = new FieldDef(t___2107, t___3983, false);
                TableDef td__666 = new TableDef(t___2101, C::Listed.CreateReadOnlyList<FieldDef>(t___3982, t___3984));
                FieldDef t___2111;
                t___2111 = td__666.Field("age");
                FieldDef f__667 = t___2111;
                bool t___3989 = f__667.Name.SqlValue == "age";
                string fn__3980()
                {
                    return "should find age field";
                }
                test___69.Assert(t___3989, (S1::Func<string>) fn__3980);
            }
            finally
            {
                test___69.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void tableDefFieldLookupNotFoundBubbles__963()
        {
            T::Test test___70 = new T::Test();
            try
            {
                ISafeIdentifier t___2091;
                t___2091 = S0::SrcGlobal.SafeIdentifier("users");
                ISafeIdentifier t___2092 = t___2091;
                ISafeIdentifier t___2093;
                t___2093 = S0::SrcGlobal.SafeIdentifier("name");
                ISafeIdentifier t___2094 = t___2093;
                StringField t___3975 = new StringField();
                FieldDef t___3976 = new FieldDef(t___2094, t___3975, false);
                TableDef td__669 = new TableDef(t___2092, C::Listed.CreateReadOnlyList<FieldDef>(t___3976));
                bool didBubble__670;
                try
                {
                    td__669.Field("nonexistent");
                    didBubble__670 = false;
                }
                catch
                {
                    didBubble__670 = true;
                }
                string fn__3974()
                {
                    return "unknown field should bubble";
                }
                test___70.Assert(didBubble__670, (S1::Func<string>) fn__3974);
            }
            finally
            {
                test___70.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void fieldDefNullableFlag__964()
        {
            T::Test test___71 = new T::Test();
            try
            {
                ISafeIdentifier t___2079;
                t___2079 = S0::SrcGlobal.SafeIdentifier("email");
                ISafeIdentifier t___2080 = t___2079;
                StringField t___3963 = new StringField();
                FieldDef required__672 = new FieldDef(t___2080, t___3963, false);
                ISafeIdentifier t___2083;
                t___2083 = S0::SrcGlobal.SafeIdentifier("bio");
                ISafeIdentifier t___2084 = t___2083;
                StringField t___3965 = new StringField();
                FieldDef optional__673 = new FieldDef(t___2084, t___3965, true);
                bool t___3969 = !required__672.Nullable;
                string fn__3962()
                {
                    return "required field should not be nullable";
                }
                test___71.Assert(t___3969, (S1::Func<string>) fn__3962);
                bool t___3971 = optional__673.Nullable;
                string fn__3961()
                {
                    return "optional field should be nullable";
                }
                test___71.Assert(t___3971, (S1::Func<string>) fn__3961);
            }
            finally
            {
                test___71.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void stringEscaping__965()
        {
            T::Test test___73 = new T::Test();
            try
            {
                string build__797(string name__799)
                {
                    SqlBuilder t___3943 = new SqlBuilder();
                    t___3943.AppendSafe("select * from hi where name = ");
                    t___3943.AppendString(name__799);
                    return t___3943.Accumulated.ToString();
                }
                string buildWrong__798(string name__801)
                {
                    return "select * from hi where name = '" + name__801 + "'";
                }
                string actual___967 = build__797("world");
                bool t___3953 = actual___967 == "select * from hi where name = 'world'";
                string fn__3950()
                {
                    return "expected build(\u0022world\u0022) == (" + "select * from hi where name = 'world'" + ") not (" + actual___967 + ")";
                }
                test___73.Assert(t___3953, (S1::Func<string>) fn__3950);
                string bobbyTables__803 = "Robert'); drop table hi;--";
                string actual___969 = build__797("Robert'); drop table hi;--");
                bool t___3957 = actual___969 == "select * from hi where name = 'Robert''); drop table hi;--'";
                string fn__3949()
                {
                    return "expected build(bobbyTables) == (" + "select * from hi where name = 'Robert''); drop table hi;--'" + ") not (" + actual___969 + ")";
                }
                test___73.Assert(t___3957, (S1::Func<string>) fn__3949);
                string fn__3948()
                {
                    return "expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')";
                }
                test___73.Assert(true, (S1::Func<string>) fn__3948);
            }
            finally
            {
                test___73.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void stringEdgeCases__973()
        {
            T::Test test___74 = new T::Test();
            try
            {
                SqlBuilder t___3911 = new SqlBuilder();
                t___3911.AppendSafe("v = ");
                t___3911.AppendString("");
                string actual___974 = t___3911.Accumulated.ToString();
                bool t___3917 = actual___974 == "v = ''";
                string fn__3910()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022\u0022).toString() == (" + "v = ''" + ") not (" + actual___974 + ")";
                }
                test___74.Assert(t___3917, (S1::Func<string>) fn__3910);
                SqlBuilder t___3919 = new SqlBuilder();
                t___3919.AppendSafe("v = ");
                t___3919.AppendString("a''b");
                string actual___977 = t___3919.Accumulated.ToString();
                bool t___3925 = actual___977 == "v = 'a''''b'";
                string fn__3909()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022a''b\u0022).toString() == (" + "v = 'a''''b'" + ") not (" + actual___977 + ")";
                }
                test___74.Assert(t___3925, (S1::Func<string>) fn__3909);
                SqlBuilder t___3927 = new SqlBuilder();
                t___3927.AppendSafe("v = ");
                t___3927.AppendString("Hello 世界");
                string actual___980 = t___3927.Accumulated.ToString();
                bool t___3933 = actual___980 == "v = 'Hello 世界'";
                string fn__3908()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Hello 世界\u0022).toString() == (" + "v = 'Hello 世界'" + ") not (" + actual___980 + ")";
                }
                test___74.Assert(t___3933, (S1::Func<string>) fn__3908);
                SqlBuilder t___3935 = new SqlBuilder();
                t___3935.AppendSafe("v = ");
                t___3935.AppendString("Line1\nLine2");
                string actual___983 = t___3935.Accumulated.ToString();
                bool t___3941 = actual___983 == "v = 'Line1\nLine2'";
                string fn__3907()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v = \u0022, \\interpolate, \u0022Line1\\nLine2\u0022).toString() == (" + "v = 'Line1\nLine2'" + ") not (" + actual___983 + ")";
                }
                test___74.Assert(t___3941, (S1::Func<string>) fn__3907);
            }
            finally
            {
                test___74.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void numbersAndBooleans__986()
        {
            T::Test test___75 = new T::Test();
            try
            {
                SqlBuilder t___3882 = new SqlBuilder();
                t___3882.AppendSafe("select ");
                t___3882.AppendInt32(42);
                t___3882.AppendSafe(", ");
                t___3882.AppendInt64(43);
                t___3882.AppendSafe(", ");
                t___3882.AppendFloat64(19.99);
                t___3882.AppendSafe(", ");
                t___3882.AppendBoolean(true);
                t___3882.AppendSafe(", ");
                t___3882.AppendBoolean(false);
                string actual___987 = t___3882.Accumulated.ToString();
                bool t___3896 = actual___987 == "select 42, 43, 19.99, TRUE, FALSE";
                string fn__3881()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022select \u0022, \\interpolate, 42, \u0022, \u0022, \\interpolate, 43, \u0022, \u0022, \\interpolate, 19.99, \u0022, \u0022, \\interpolate, true, \u0022, \u0022, \\interpolate, false).toString() == (" + "select 42, 43, 19.99, TRUE, FALSE" + ") not (" + actual___987 + ")";
                }
                test___75.Assert(t___3896, (S1::Func<string>) fn__3881);
                S1::DateTime t___2024;
                t___2024 = new S1::DateTime(2024, 12, 25);
                S1::DateTime date__806 = t___2024;
                SqlBuilder t___3898 = new SqlBuilder();
                t___3898.AppendSafe("insert into t values (");
                t___3898.AppendDate(date__806);
                t___3898.AppendSafe(")");
                string actual___990 = t___3898.Accumulated.ToString();
                bool t___3905 = actual___990 == "insert into t values ('2024-12-25')";
                string fn__3880()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022insert into t values (\u0022, \\interpolate, date, \u0022)\u0022).toString() == (" + "insert into t values ('2024-12-25')" + ") not (" + actual___990 + ")";
                }
                test___75.Assert(t___3905, (S1::Func<string>) fn__3880);
            }
            finally
            {
                test___75.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void lists__993()
        {
            T::Test test___76 = new T::Test();
            try
            {
                SqlBuilder t___3826 = new SqlBuilder();
                t___3826.AppendSafe("v IN (");
                t___3826.AppendStringList(C::Listed.CreateReadOnlyList<string>("a", "b", "c'd"));
                t___3826.AppendSafe(")");
                string actual___994 = t___3826.Accumulated.ToString();
                bool t___3833 = actual___994 == "v IN ('a', 'b', 'c''d')";
                string fn__3825()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(\u0022a\u0022, \u0022b\u0022, \u0022c'd\u0022), \u0022)\u0022).toString() == (" + "v IN ('a', 'b', 'c''d')" + ") not (" + actual___994 + ")";
                }
                test___76.Assert(t___3833, (S1::Func<string>) fn__3825);
                SqlBuilder t___3835 = new SqlBuilder();
                t___3835.AppendSafe("v IN (");
                t___3835.AppendInt32_List(C::Listed.CreateReadOnlyList<int>(1, 2, 3));
                t___3835.AppendSafe(")");
                string actual___997 = t___3835.Accumulated.ToString();
                bool t___3842 = actual___997 == "v IN (1, 2, 3)";
                string fn__3824()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2, 3), \u0022)\u0022).toString() == (" + "v IN (1, 2, 3)" + ") not (" + actual___997 + ")";
                }
                test___76.Assert(t___3842, (S1::Func<string>) fn__3824);
                SqlBuilder t___3844 = new SqlBuilder();
                t___3844.AppendSafe("v IN (");
                t___3844.AppendInt64_List(C::Listed.CreateReadOnlyList<long>(1, 2));
                t___3844.AppendSafe(")");
                string actual___1000 = t___3844.Accumulated.ToString();
                bool t___3851 = actual___1000 == "v IN (1, 2)";
                string fn__3823()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1, 2), \u0022)\u0022).toString() == (" + "v IN (1, 2)" + ") not (" + actual___1000 + ")";
                }
                test___76.Assert(t___3851, (S1::Func<string>) fn__3823);
                SqlBuilder t___3853 = new SqlBuilder();
                t___3853.AppendSafe("v IN (");
                t___3853.AppendFloat64_List(C::Listed.CreateReadOnlyList<double>(1.0, 2.0));
                t___3853.AppendSafe(")");
                string actual___1003 = t___3853.Accumulated.ToString();
                bool t___3860 = actual___1003 == "v IN (1.0, 2.0)";
                string fn__3822()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(1.0, 2.0), \u0022)\u0022).toString() == (" + "v IN (1.0, 2.0)" + ") not (" + actual___1003 + ")";
                }
                test___76.Assert(t___3860, (S1::Func<string>) fn__3822);
                SqlBuilder t___3862 = new SqlBuilder();
                t___3862.AppendSafe("v IN (");
                t___3862.AppendBooleanList(C::Listed.CreateReadOnlyList<bool>(true, false));
                t___3862.AppendSafe(")");
                string actual___1006 = t___3862.Accumulated.ToString();
                bool t___3869 = actual___1006 == "v IN (TRUE, FALSE)";
                string fn__3821()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, list(true, false), \u0022)\u0022).toString() == (" + "v IN (TRUE, FALSE)" + ") not (" + actual___1006 + ")";
                }
                test___76.Assert(t___3869, (S1::Func<string>) fn__3821);
                S1::DateTime t___1996;
                t___1996 = new S1::DateTime(2024, 1, 1);
                S1::DateTime t___1997 = t___1996;
                S1::DateTime t___1998;
                t___1998 = new S1::DateTime(2024, 12, 25);
                S1::DateTime t___1999 = t___1998;
                G::IReadOnlyList<S1::DateTime> dates__808 = C::Listed.CreateReadOnlyList<S1::DateTime>(t___1997, t___1999);
                SqlBuilder t___3871 = new SqlBuilder();
                t___3871.AppendSafe("v IN (");
                t___3871.AppendDateList(dates__808);
                t___3871.AppendSafe(")");
                string actual___1009 = t___3871.Accumulated.ToString();
                bool t___3878 = actual___1009 == "v IN ('2024-01-01', '2024-12-25')";
                string fn__3820()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022v IN (\u0022, \\interpolate, dates, \u0022)\u0022).toString() == (" + "v IN ('2024-01-01', '2024-12-25')" + ") not (" + actual___1009 + ")";
                }
                test___76.Assert(t___3878, (S1::Func<string>) fn__3820);
            }
            finally
            {
                test___76.SoftFailToHard();
            }
        }
        [U::TestMethod]
        public void nesting__1012()
        {
            T::Test test___77 = new T::Test();
            try
            {
                string name__810 = "Someone";
                SqlBuilder t___3789 = new SqlBuilder();
                t___3789.AppendSafe("where p.last_name = ");
                t___3789.AppendString("Someone");
                SqlFragment condition__811 = t___3789.Accumulated;
                SqlBuilder t___3793 = new SqlBuilder();
                t___3793.AppendSafe("select p.id from person p ");
                t___3793.AppendFragment(condition__811);
                string actual___1014 = t___3793.Accumulated.ToString();
                bool t___3799 = actual___1014 == "select p.id from person p where p.last_name = 'Someone'";
                string fn__3788()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual___1014 + ")";
                }
                test___77.Assert(t___3799, (S1::Func<string>) fn__3788);
                SqlBuilder t___3801 = new SqlBuilder();
                t___3801.AppendSafe("select p.id from person p ");
                t___3801.AppendPart(condition__811.ToSource());
                string actual___1017 = t___3801.Accumulated.ToString();
                bool t___3808 = actual___1017 == "select p.id from person p where p.last_name = 'Someone'";
                string fn__3787()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022select p.id from person p \u0022, \\interpolate, condition.toSource()).toString() == (" + "select p.id from person p where p.last_name = 'Someone'" + ") not (" + actual___1017 + ")";
                }
                test___77.Assert(t___3808, (S1::Func<string>) fn__3787);
                G::IReadOnlyList<ISqlPart> parts__812 = C::Listed.CreateReadOnlyList<ISqlPart>(new SqlString("a'b"), new SqlInt32(3));
                SqlBuilder t___3812 = new SqlBuilder();
                t___3812.AppendSafe("select ");
                t___3812.AppendPartList(parts__812);
                string actual___1020 = t___3812.Accumulated.ToString();
                bool t___3818 = actual___1020 == "select 'a''b', 3";
                string fn__3786()
                {
                    return "expected stringExpr(`-work//src/`.sql, true, \u0022select \u0022, \\interpolate, parts).toString() == (" + "select 'a''b', 3" + ") not (" + actual___1020 + ")";
                }
                test___77.Assert(t___3818, (S1::Func<string>) fn__3786);
            }
            finally
            {
                test___77.SoftFailToHard();
            }
        }
    }
}
