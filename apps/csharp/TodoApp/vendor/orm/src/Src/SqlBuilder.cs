using S = System;
using G = System.Collections.Generic;
using C = TemperLang.Core;
namespace Orm.Src
{
    public class SqlBuilder
    {
        readonly G::IList<ISqlPart> buffer__674;
        public void AppendSafe(string sqlSource__676)
        {
            SqlSource t___4789 = new SqlSource(sqlSource__676);
            C::Listed.Add(this.buffer__674, t___4789);
        }
        public void AppendFragment(SqlFragment fragment__679)
        {
            G::IReadOnlyList<ISqlPart> t___4787 = fragment__679.Parts;
            C::Listed.AddAll(this.buffer__674, t___4787);
        }
        public void AppendPart(ISqlPart part__682)
        {
            C::Listed.Add(this.buffer__674, part__682);
        }
        public void AppendPartList(G::IReadOnlyList<ISqlPart> values__685)
        {
            void fn__4783(ISqlPart x__687)
            {
                this.AppendPart(x__687);
            }
            this.AppendList(values__685, (S::Action<ISqlPart>) fn__4783);
        }
        public void AppendBoolean(bool value__689)
        {
            SqlBoolean t___4780 = new SqlBoolean(value__689);
            C::Listed.Add(this.buffer__674, t___4780);
        }
        public void AppendBooleanList(G::IReadOnlyList<bool> values__692)
        {
            void fn__4777(bool x__694)
            {
                this.AppendBoolean(x__694);
            }
            this.AppendList(values__692, (S::Action<bool>) fn__4777);
        }
        public void AppendDate(S::DateTime value__696)
        {
            SqlDate t___4774 = new SqlDate(value__696);
            C::Listed.Add(this.buffer__674, t___4774);
        }
        public void AppendDateList(G::IReadOnlyList<S::DateTime> values__699)
        {
            void fn__4771(S::DateTime x__701)
            {
                this.AppendDate(x__701);
            }
            this.AppendList(values__699, (S::Action<S::DateTime>) fn__4771);
        }
        public void AppendFloat64(double value__703)
        {
            SqlFloat64 t___4768 = new SqlFloat64(value__703);
            C::Listed.Add(this.buffer__674, t___4768);
        }
        public void AppendFloat64_List(G::IReadOnlyList<double> values__706)
        {
            void fn__4765(double x__708)
            {
                this.AppendFloat64(x__708);
            }
            this.AppendList(values__706, (S::Action<double>) fn__4765);
        }
        public void AppendInt32(int value__710)
        {
            SqlInt32 t___4762 = new SqlInt32(value__710);
            C::Listed.Add(this.buffer__674, t___4762);
        }
        public void AppendInt32_List(G::IReadOnlyList<int> values__713)
        {
            void fn__4759(int x__715)
            {
                this.AppendInt32(x__715);
            }
            this.AppendList(values__713, (S::Action<int>) fn__4759);
        }
        public void AppendInt64(long value__717)
        {
            SqlInt64 t___4756 = new SqlInt64(value__717);
            C::Listed.Add(this.buffer__674, t___4756);
        }
        public void AppendInt64_List(G::IReadOnlyList<long> values__720)
        {
            void fn__4753(long x__722)
            {
                this.AppendInt64(x__722);
            }
            this.AppendList(values__720, (S::Action<long>) fn__4753);
        }
        public void AppendString(string value__724)
        {
            SqlString t___4750 = new SqlString(value__724);
            C::Listed.Add(this.buffer__674, t___4750);
        }
        public void AppendStringList(G::IReadOnlyList<string> values__727)
        {
            void fn__4747(string x__729)
            {
                this.AppendString(x__729);
            }
            this.AppendList(values__727, (S::Action<string>) fn__4747);
        }
        void AppendList<T__136>(G::IReadOnlyList<T__136> values__731, S::Action<T__136> appendValue__732)
        {
            int t___4742;
            T__136 t___4744;
            int i__734 = 0;
            while (true)
            {
                t___4742 = values__731.Count;
                if (!(i__734 < t___4742)) break;
                if (i__734 > 0) this.AppendSafe(", ");
                t___4744 = values__731[i__734];
                appendValue__732(t___4744);
                i__734 = i__734 + 1;
            }
        }
        public SqlFragment Accumulated
        {
            get
            {
                return new SqlFragment(C::Listed.ToReadOnlyList(this.buffer__674));
            }
        }
        public SqlBuilder()
        {
            G::IList<ISqlPart> t___4739 = new G::List<ISqlPart>();
            this.buffer__674 = t___4739;
        }
    }
}
