using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
using T = TemperLang.Std.Temporal;
namespace Orm.Src
{
    class ChangesetImpl: IChangeset
    {
        readonly TableDef _tableDef__341;
        readonly G::IReadOnlyDictionary<string, string> _params__342;
        readonly G::IReadOnlyDictionary<string, string> _changes__343;
        readonly G::IReadOnlyList<ChangesetError> _errors__344;
        readonly bool _isValid__345;
        public TableDef TableDef
        {
            get
            {
                return this._tableDef__341;
            }
        }
        public G::IReadOnlyDictionary<string, string> Changes
        {
            get
            {
                return this._changes__343;
            }
        }
        public G::IReadOnlyList<ChangesetError> Errors
        {
            get
            {
                return this._errors__344;
            }
        }
        public bool IsValid
        {
            get
            {
                return this._isValid__345;
            }
        }
        public IChangeset Cast(G::IReadOnlyList<ISafeIdentifier> allowedFields__355)
        {
            G::IDictionary<string, string> mb__357 = new C::OrderedDictionary<string, string>();
            void fn__4731(ISafeIdentifier f__358)
            {
                string t___4729;
                string t___4726 = f__358.SqlValue;
                string val__359 = C::Mapped.GetOrDefault(this._params__342, t___4726, "");
                if (!string.IsNullOrEmpty(val__359))
                {
                    t___4729 = f__358.SqlValue;
                    mb__357[t___4729] = val__359;
                }
            }
            C::Listed.ForEach(allowedFields__355, (S::Action<ISafeIdentifier>) fn__4731);
            return new ChangesetImpl(this._tableDef__341, this._params__342, C::Mapped.ToMap(mb__357), this._errors__344, this._isValid__345);
        }
        public IChangeset ValidateRequired(G::IReadOnlyList<ISafeIdentifier> fields__361)
        {
            IChangeset return__183;
            G::IReadOnlyList<ChangesetError> t___4724;
            TableDef t___2816;
            G::IReadOnlyDictionary<string, string> t___2817;
            G::IReadOnlyDictionary<string, string> t___2818;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__183 = this;
                        goto fn__362;
                    }
                    G::IList<ChangesetError> eb__363 = L::Enumerable.ToList(this._errors__344);
                    bool valid__364 = true;
                    void fn__4720(ISafeIdentifier f__365)
                    {
                        ChangesetError t___4718;
                        string t___4715 = f__365.SqlValue;
                        if (!C::Mapped.ContainsKey(this._changes__343, t___4715))
                        {
                            t___4718 = new ChangesetError(f__365.SqlValue, "is required");
                            C::Listed.Add(eb__363, t___4718);
                            valid__364 = false;
                        }
                    }
                    C::Listed.ForEach(fields__361, (S::Action<ISafeIdentifier>) fn__4720);
                    t___2816 = this._tableDef__341;
                    t___2817 = this._params__342;
                    t___2818 = this._changes__343;
                    t___4724 = C::Listed.ToReadOnlyList(eb__363);
                    return__183 = new ChangesetImpl(t___2816, t___2817, t___2818, t___4724, valid__364);
                }
                fn__362:
                {
                }
            }
            return return__183;
        }
        public IChangeset ValidateLength(ISafeIdentifier field__367, int min__368, int max__369)
        {
            IChangeset return__184;
            string t___4702;
            G::IReadOnlyList<ChangesetError> t___4713;
            bool t___2799;
            TableDef t___2805;
            G::IReadOnlyDictionary<string, string> t___2806;
            G::IReadOnlyDictionary<string, string> t___2807;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__184 = this;
                        goto fn__370;
                    }
                    t___4702 = field__367.SqlValue;
                    string val__371 = C::Mapped.GetOrDefault(this._changes__343, t___4702, "");
                    int len__372 = C::StringUtil.CountBetween(val__371, 0, val__371.Length);
                    if (len__372 < min__368)
                    {
                        t___2799 = true;
                    }
                    else
                    {
                        t___2799 = len__372 > max__369;
                    }
                    if (t___2799)
                    {
                        string msg__373 = "must be between " + S::Convert.ToString(min__368) + " and " + S::Convert.ToString(max__369) + " characters";
                        G::IList<ChangesetError> eb__374 = L::Enumerable.ToList(this._errors__344);
                        C::Listed.Add(eb__374, new ChangesetError(field__367.SqlValue, msg__373));
                        t___2805 = this._tableDef__341;
                        t___2806 = this._params__342;
                        t___2807 = this._changes__343;
                        t___4713 = C::Listed.ToReadOnlyList(eb__374);
                        return__184 = new ChangesetImpl(t___2805, t___2806, t___2807, t___4713, false);
                        goto fn__370;
                    }
                    return__184 = this;
                }
                fn__370:
                {
                }
            }
            return return__184;
        }
        public IChangeset ValidateInt(ISafeIdentifier field__376)
        {
            IChangeset return__185;
            string t___4693;
            G::IReadOnlyList<ChangesetError> t___4700;
            TableDef t___2790;
            G::IReadOnlyDictionary<string, string> t___2791;
            G::IReadOnlyDictionary<string, string> t___2792;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__185 = this;
                        goto fn__377;
                    }
                    t___4693 = field__376.SqlValue;
                    string val__378 = C::Mapped.GetOrDefault(this._changes__343, t___4693, "");
                    if (string.IsNullOrEmpty(val__378))
                    {
                        return__185 = this;
                        goto fn__377;
                    }
                    bool parseOk__379;
                    try
                    {
                        C::Core.ToInt(val__378);
                        parseOk__379 = true;
                    }
                    catch
                    {
                        parseOk__379 = false;
                    }
                    if (!parseOk__379)
                    {
                        G::IList<ChangesetError> eb__380 = L::Enumerable.ToList(this._errors__344);
                        C::Listed.Add(eb__380, new ChangesetError(field__376.SqlValue, "must be an integer"));
                        t___2790 = this._tableDef__341;
                        t___2791 = this._params__342;
                        t___2792 = this._changes__343;
                        t___4700 = C::Listed.ToReadOnlyList(eb__380);
                        return__185 = new ChangesetImpl(t___2790, t___2791, t___2792, t___4700, false);
                        goto fn__377;
                    }
                    return__185 = this;
                }
                fn__377:
                {
                }
            }
            return return__185;
        }
        public IChangeset ValidateInt64(ISafeIdentifier field__382)
        {
            IChangeset return__186;
            string t___4684;
            G::IReadOnlyList<ChangesetError> t___4691;
            TableDef t___2777;
            G::IReadOnlyDictionary<string, string> t___2778;
            G::IReadOnlyDictionary<string, string> t___2779;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__186 = this;
                        goto fn__383;
                    }
                    t___4684 = field__382.SqlValue;
                    string val__384 = C::Mapped.GetOrDefault(this._changes__343, t___4684, "");
                    if (string.IsNullOrEmpty(val__384))
                    {
                        return__186 = this;
                        goto fn__383;
                    }
                    bool parseOk__385;
                    try
                    {
                        C::Core.ToInt64(val__384);
                        parseOk__385 = true;
                    }
                    catch
                    {
                        parseOk__385 = false;
                    }
                    if (!parseOk__385)
                    {
                        G::IList<ChangesetError> eb__386 = L::Enumerable.ToList(this._errors__344);
                        C::Listed.Add(eb__386, new ChangesetError(field__382.SqlValue, "must be a 64-bit integer"));
                        t___2777 = this._tableDef__341;
                        t___2778 = this._params__342;
                        t___2779 = this._changes__343;
                        t___4691 = C::Listed.ToReadOnlyList(eb__386);
                        return__186 = new ChangesetImpl(t___2777, t___2778, t___2779, t___4691, false);
                        goto fn__383;
                    }
                    return__186 = this;
                }
                fn__383:
                {
                }
            }
            return return__186;
        }
        public IChangeset ValidateFloat(ISafeIdentifier field__388)
        {
            IChangeset return__187;
            string t___4675;
            G::IReadOnlyList<ChangesetError> t___4682;
            TableDef t___2764;
            G::IReadOnlyDictionary<string, string> t___2765;
            G::IReadOnlyDictionary<string, string> t___2766;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__187 = this;
                        goto fn__389;
                    }
                    t___4675 = field__388.SqlValue;
                    string val__390 = C::Mapped.GetOrDefault(this._changes__343, t___4675, "");
                    if (string.IsNullOrEmpty(val__390))
                    {
                        return__187 = this;
                        goto fn__389;
                    }
                    bool parseOk__391;
                    try
                    {
                        C::Float64.ToFloat64(val__390);
                        parseOk__391 = true;
                    }
                    catch
                    {
                        parseOk__391 = false;
                    }
                    if (!parseOk__391)
                    {
                        G::IList<ChangesetError> eb__392 = L::Enumerable.ToList(this._errors__344);
                        C::Listed.Add(eb__392, new ChangesetError(field__388.SqlValue, "must be a number"));
                        t___2764 = this._tableDef__341;
                        t___2765 = this._params__342;
                        t___2766 = this._changes__343;
                        t___4682 = C::Listed.ToReadOnlyList(eb__392);
                        return__187 = new ChangesetImpl(t___2764, t___2765, t___2766, t___4682, false);
                        goto fn__389;
                    }
                    return__187 = this;
                }
                fn__389:
                {
                }
            }
            return return__187;
        }
        public IChangeset ValidateBool(ISafeIdentifier field__394)
        {
            IChangeset return__188;
            string t___4666;
            G::IReadOnlyList<ChangesetError> t___4673;
            bool t___2739;
            bool t___2740;
            bool t___2742;
            bool t___2743;
            bool t___2745;
            TableDef t___2751;
            G::IReadOnlyDictionary<string, string> t___2752;
            G::IReadOnlyDictionary<string, string> t___2753;
            {
                {
                    if (!this._isValid__345)
                    {
                        return__188 = this;
                        goto fn__395;
                    }
                    t___4666 = field__394.SqlValue;
                    string val__396 = C::Mapped.GetOrDefault(this._changes__343, t___4666, "");
                    if (string.IsNullOrEmpty(val__396))
                    {
                        return__188 = this;
                        goto fn__395;
                    }
                    bool isTrue__397;
                    if (val__396 == "true")
                    {
                        isTrue__397 = true;
                    }
                    else
                    {
                        if (val__396 == "1")
                        {
                            t___2740 = true;
                        }
                        else
                        {
                            if (val__396 == "yes")
                            {
                                t___2739 = true;
                            }
                            else
                            {
                                t___2739 = val__396 == "on";
                            }
                            t___2740 = t___2739;
                        }
                        isTrue__397 = t___2740;
                    }
                    bool isFalse__398;
                    if (val__396 == "false")
                    {
                        isFalse__398 = true;
                    }
                    else
                    {
                        if (val__396 == "0")
                        {
                            t___2743 = true;
                        }
                        else
                        {
                            if (val__396 == "no")
                            {
                                t___2742 = true;
                            }
                            else
                            {
                                t___2742 = val__396 == "off";
                            }
                            t___2743 = t___2742;
                        }
                        isFalse__398 = t___2743;
                    }
                    if (!isTrue__397)
                    {
                        t___2745 = !isFalse__398;
                    }
                    else
                    {
                        t___2745 = false;
                    }
                    if (t___2745)
                    {
                        G::IList<ChangesetError> eb__399 = L::Enumerable.ToList(this._errors__344);
                        C::Listed.Add(eb__399, new ChangesetError(field__394.SqlValue, "must be a boolean (true/false/1/0/yes/no/on/off)"));
                        t___2751 = this._tableDef__341;
                        t___2752 = this._params__342;
                        t___2753 = this._changes__343;
                        t___4673 = C::Listed.ToReadOnlyList(eb__399);
                        return__188 = new ChangesetImpl(t___2751, t___2752, t___2753, t___4673, false);
                        goto fn__395;
                    }
                    return__188 = this;
                }
                fn__395:
                {
                }
            }
            return return__188;
        }
        SqlBoolean ParseBoolSqlPart(string val__401)
        {
            SqlBoolean return__189;
            bool t___2728;
            bool t___2729;
            bool t___2730;
            bool t___2732;
            bool t___2733;
            bool t___2734;
            {
                {
                    if (val__401 == "true")
                    {
                        t___2730 = true;
                    }
                    else
                    {
                        if (val__401 == "1")
                        {
                            t___2729 = true;
                        }
                        else
                        {
                            if (val__401 == "yes")
                            {
                                t___2728 = true;
                            }
                            else
                            {
                                t___2728 = val__401 == "on";
                            }
                            t___2729 = t___2728;
                        }
                        t___2730 = t___2729;
                    }
                    if (t___2730)
                    {
                        return__189 = new SqlBoolean(true);
                        goto fn__402;
                    }
                    if (val__401 == "false")
                    {
                        t___2734 = true;
                    }
                    else
                    {
                        if (val__401 == "0")
                        {
                            t___2733 = true;
                        }
                        else
                        {
                            if (val__401 == "no")
                            {
                                t___2732 = true;
                            }
                            else
                            {
                                t___2732 = val__401 == "off";
                            }
                            t___2733 = t___2732;
                        }
                        t___2734 = t___2733;
                    }
                    if (t___2734)
                    {
                        return__189 = new SqlBoolean(false);
                        goto fn__402;
                    }
                    throw new S::Exception();
                }
                fn__402:
                {
                }
            }
            return return__189;
        }
        ISqlPart ValueToSqlPart(FieldDef fieldDef__404, string val__405)
        {
            ISqlPart return__190;
            int t___2715;
            long t___2718;
            double t___2721;
            S::DateTime t___2726;
            {
                {
                    IFieldType ft__407 = fieldDef__404.FieldType;
                    if (ft__407 is StringField)
                    {
                        return__190 = new SqlString(val__405);
                        goto fn__406;
                    }
                    if (ft__407 is IntField)
                    {
                        t___2715 = C::Core.ToInt(val__405);
                        return__190 = new SqlInt32(t___2715);
                        goto fn__406;
                    }
                    if (ft__407 is Int64_Field)
                    {
                        t___2718 = C::Core.ToInt64(val__405);
                        return__190 = new SqlInt64(t___2718);
                        goto fn__406;
                    }
                    if (ft__407 is FloatField)
                    {
                        t___2721 = C::Float64.ToFloat64(val__405);
                        return__190 = new SqlFloat64(t___2721);
                        goto fn__406;
                    }
                    if (ft__407 is BoolField)
                    {
                        return__190 = this.ParseBoolSqlPart(val__405);
                        goto fn__406;
                    }
                    if (ft__407 is DateField)
                    {
                        t___2726 = T::TemporalSupport.FromIsoString(val__405);
                        return__190 = new SqlDate(t___2726);
                        goto fn__406;
                    }
                    throw new S::Exception();
                }
                fn__406:
                {
                }
            }
            return return__190;
        }
        public SqlFragment ToInsertSql()
        {
            int t___4615;
            string t___4620;
            bool t___4621;
            int t___4626;
            string t___4628;
            string t___4631;
            int t___4646;
            bool t___2680;
            FieldDef t___2688;
            ISqlPart t___2692;
            if (!this._isValid__345) throw new S::Exception();
            int i__410 = 0;
            while (true)
            {
                t___4615 = this._tableDef__341.Fields.Count;
                if (!(i__410 < t___4615)) break;
                FieldDef f__411 = this._tableDef__341.Fields[i__410];
                if (!f__411.Nullable)
                {
                    t___4620 = f__411.Name.SqlValue;
                    t___4621 = C::Mapped.ContainsKey(this._changes__343, t___4620);
                    t___2680 = !t___4621;
                }
                else
                {
                    t___2680 = false;
                }
                if (t___2680) throw new S::Exception();
                i__410 = i__410 + 1;
            }
            G::IReadOnlyList<G::KeyValuePair<string, string>> pairs__412 = C::Mapped.ToList(this._changes__343);
            if (pairs__412.Count == 0) throw new S::Exception();
            G::IList<string> colNames__413 = new G::List<string>();
            G::IList<ISqlPart> valParts__414 = new G::List<ISqlPart>();
            int i__415 = 0;
            while (true)
            {
                t___4626 = pairs__412.Count;
                if (!(i__415 < t___4626)) break;
                G::KeyValuePair<string, string> pair__416 = pairs__412[i__415];
                t___4628 = pair__416.Key;
                t___2688 = this._tableDef__341.Field(t___4628);
                FieldDef fd__417 = t___2688;
                C::Listed.Add(colNames__413, pair__416.Key);
                t___4631 = pair__416.Value;
                t___2692 = this.ValueToSqlPart(fd__417, t___4631);
                C::Listed.Add(valParts__414, t___2692);
                i__415 = i__415 + 1;
            }
            SqlBuilder b__418 = new SqlBuilder();
            b__418.AppendSafe("INSERT INTO ");
            b__418.AppendSafe(this._tableDef__341.TableName.SqlValue);
            b__418.AppendSafe(" (");
            G::IReadOnlyList<string> t___4639 = C::Listed.ToReadOnlyList(colNames__413);
            string fn__4613(string c__419)
            {
                return c__419;
            }
            b__418.AppendSafe(C::Listed.Join(t___4639, ", ", (S::Func<string, string>) fn__4613));
            b__418.AppendSafe(") VALUES (");
            b__418.AppendPart(valParts__414[0]);
            int j__420 = 1;
            while (true)
            {
                t___4646 = valParts__414.Count;
                if (!(j__420 < t___4646)) break;
                b__418.AppendSafe(", ");
                b__418.AppendPart(valParts__414[j__420]);
                j__420 = j__420 + 1;
            }
            b__418.AppendSafe(")");
            return b__418.Accumulated;
        }
        public SqlFragment ToUpdateSql(int id__422)
        {
            int t___4601;
            string t___4604;
            string t___4608;
            FieldDef t___2662;
            ISqlPart t___2667;
            if (!this._isValid__345) throw new S::Exception();
            G::IReadOnlyList<G::KeyValuePair<string, string>> pairs__424 = C::Mapped.ToList(this._changes__343);
            if (pairs__424.Count == 0) throw new S::Exception();
            SqlBuilder b__425 = new SqlBuilder();
            b__425.AppendSafe("UPDATE ");
            b__425.AppendSafe(this._tableDef__341.TableName.SqlValue);
            b__425.AppendSafe(" SET ");
            int i__426 = 0;
            while (true)
            {
                t___4601 = pairs__424.Count;
                if (!(i__426 < t___4601)) break;
                if (i__426 > 0) b__425.AppendSafe(", ");
                G::KeyValuePair<string, string> pair__427 = pairs__424[i__426];
                t___4604 = pair__427.Key;
                t___2662 = this._tableDef__341.Field(t___4604);
                FieldDef fd__428 = t___2662;
                b__425.AppendSafe(pair__427.Key);
                b__425.AppendSafe(" = ");
                t___4608 = pair__427.Value;
                t___2667 = this.ValueToSqlPart(fd__428, t___4608);
                b__425.AppendPart(t___2667);
                i__426 = i__426 + 1;
            }
            b__425.AppendSafe(" WHERE id = ");
            b__425.AppendInt32(id__422);
            return b__425.Accumulated;
        }
        public ChangesetImpl(TableDef _tableDef__430, G::IReadOnlyDictionary<string, string> _params__431, G::IReadOnlyDictionary<string, string> _changes__432, G::IReadOnlyList<ChangesetError> _errors__433, bool _isValid__434)
        {
            this._tableDef__341 = _tableDef__430;
            this._params__342 = _params__431;
            this._changes__343 = _changes__432;
            this._errors__344 = _errors__433;
            this._isValid__345 = _isValid__434;
        }
    }
}
