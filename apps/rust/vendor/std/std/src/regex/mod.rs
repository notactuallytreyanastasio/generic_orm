#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
mod support;
pub use support::*;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            CODES__AMPERSAND.set(38).unwrap_or_else(| _ | panic!());
            CODES__BACKSLASH.set(92).unwrap_or_else(| _ | panic!());
            CODES__CARET.set(94).unwrap_or_else(| _ | panic!());
            CODES__CARRIAGE_RETURN.set(13).unwrap_or_else(| _ | panic!());
            CODES__CURLY_LEFT.set(123).unwrap_or_else(| _ | panic!());
            CODES__CURLY_RIGHT.set(125).unwrap_or_else(| _ | panic!());
            CODES__DASH.set(45).unwrap_or_else(| _ | panic!());
            CODES__DOT.set(46).unwrap_or_else(| _ | panic!());
            CODES__HIGH_CONTROL_MIN.set(127).unwrap_or_else(| _ | panic!());
            CODES__HIGH_CONTROL_MAX.set(159).unwrap_or_else(| _ | panic!());
            CODES__DIGIT0.set(48).unwrap_or_else(| _ | panic!());
            CODES__DIGIT9.set(57).unwrap_or_else(| _ | panic!());
            CODES__LOWER_A.set(97).unwrap_or_else(| _ | panic!());
            CODES__LOWER_Z.set(122).unwrap_or_else(| _ | panic!());
            CODES__NEWLINE.set(10).unwrap_or_else(| _ | panic!());
            CODES__PESO.set(36).unwrap_or_else(| _ | panic!());
            CODES__PIPE.set(124).unwrap_or_else(| _ | panic!());
            CODES__PLUS.set(43).unwrap_or_else(| _ | panic!());
            CODES__QUESTION.set(63).unwrap_or_else(| _ | panic!());
            CODES__ROUND_LEFT.set(40).unwrap_or_else(| _ | panic!());
            CODES__ROUND_RIGHT.set(41).unwrap_or_else(| _ | panic!());
            CODES__SLASH.set(47).unwrap_or_else(| _ | panic!());
            CODES__SQUARE_LEFT.set(91).unwrap_or_else(| _ | panic!());
            CODES__SQUARE_RIGHT.set(93).unwrap_or_else(| _ | panic!());
            CODES__STAR.set(42).unwrap_or_else(| _ | panic!());
            CODES__TAB.set(9).unwrap_or_else(| _ | panic!());
            CODES__TILDE.set(42).unwrap_or_else(| _ | panic!());
            CODES__UPPER_A.set(65).unwrap_or_else(| _ | panic!());
            CODES__UPPER_Z.set(90).unwrap_or_else(| _ | panic!());
            CODES__SPACE.set(32).unwrap_or_else(| _ | panic!());
            CODES__SURROGATE_MIN.set(55296).unwrap_or_else(| _ | panic!());
            CODES__SURROGATE_MAX.set(57343).unwrap_or_else(| _ | panic!());
            CODES__SUPPLEMENTAL_MIN.set(65536).unwrap_or_else(| _ | panic!());
            CODES__UINT16_MAX.set(65535).unwrap_or_else(| _ | panic!());
            CODES__UNDERSCORE.set(95).unwrap_or_else(| _ | panic!());
            let return__192: Special = Special::new(Begin::new());
            BEGIN.set(return__192.clone()).unwrap_or_else(| _ | panic!());
            let return__194: Special = Special::new(Dot::new());
            DOT.set(return__194.clone()).unwrap_or_else(| _ | panic!());
            let return__196: Special = Special::new(End::new());
            END.set(return__196.clone()).unwrap_or_else(| _ | panic!());
            let return__198: Special = Special::new(WordBoundary::new());
            WORD_BOUNDARY.set(return__198.clone()).unwrap_or_else(| _ | panic!());
            let return__200: SpecialSet = SpecialSet::new(Digit::new());
            DIGIT.set(return__200.clone()).unwrap_or_else(| _ | panic!());
            let return__202: SpecialSet = SpecialSet::new(Space::new());
            SPACE.set(return__202.clone()).unwrap_or_else(| _ | panic!());
            let return__204: SpecialSet = SpecialSet::new(Word::new());
            WORD.set(return__204.clone()).unwrap_or_else(| _ | panic!());
            let needsNoEscape__166: i32 = 0;
            let needsSimpleEscape__168: i32 = 2;
            let needsNumericEscape__167: i32 = 1;
            ESCAPE_NEEDS.set(buildEscapeNeeds__163()).unwrap_or_else(| _ | panic!());
            REGEX_REFS.set(RegexRefs::new(None, None, None, None)).unwrap_or_else(| _ | panic!());
            Ok(())
    }).clone()
}
static BEGIN: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn begin() -> Special {
    ( * BEGIN.get().unwrap()).clone()
}
static DOT: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn dot() -> Special {
    ( * DOT.get().unwrap()).clone()
}
static END: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn end() -> Special {
    ( * END.get().unwrap()).clone()
}
static WORD_BOUNDARY: std::sync::OnceLock<Special> = std::sync::OnceLock::new();
pub fn word_boundary() -> Special {
    ( * WORD_BOUNDARY.get().unwrap()).clone()
}
static DIGIT: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn digit() -> SpecialSet {
    ( * DIGIT.get().unwrap()).clone()
}
static SPACE: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn space() -> SpecialSet {
    ( * SPACE.get().unwrap()).clone()
}
static WORD: std::sync::OnceLock<SpecialSet> = std::sync::OnceLock::new();
pub fn word() -> SpecialSet {
    ( * WORD.get().unwrap()).clone()
}
static ESCAPE_NEEDS: std::sync::OnceLock<temper_core::List<i32>> = std::sync::OnceLock::new();
fn escape_needs() -> temper_core::List<i32> {
    ( * ESCAPE_NEEDS.get().unwrap()).clone()
}
static REGEX_REFS: std::sync::OnceLock<RegexRefs> = std::sync::OnceLock::new();
fn regex_refs() -> RegexRefs {
    ( * REGEX_REFS.get().unwrap()).clone()
}
pub trait RegexNodeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> RegexNode;
    fn compiled(& self) -> Regex {
        return Regex::new(self.clone_boxed());
    }
    fn found(& self, text__172: std::sync::Arc<String>) -> bool {
        return self.compiled().found(text__172.clone());
    }
    fn find(& self, text__175: std::sync::Arc<String>) -> temper_core::Result<Match> {
        let return__86: Match;
        return__86 = self.compiled().find(text__175.clone(), None) ? ;
        return Ok(return__86.clone());
    }
    fn replace(& self, text__178: std::sync::Arc<String>, format__179: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        return self.compiled().replace(text__178.clone(), format__179.clone());
    }
    fn split(& self, text__182: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        return self.compiled().split(text__182.clone());
    }
}
#[derive(Clone)]
pub struct RegexNode(std::sync::Arc<dyn RegexNodeTrait>);
impl RegexNode {
    pub fn new(selfish: impl RegexNodeTrait + 'static) -> RegexNode {
        RegexNode(std::sync::Arc::new(selfish))
    }
}
impl RegexNodeTrait for RegexNode {
    fn clone_boxed(& self) -> RegexNode {
        RegexNodeTrait::clone_boxed( & ( * self.0))
    }
    fn compiled(& self) -> Regex {
        RegexNodeTrait::compiled( & ( * self.0))
    }
    fn found(& self, arg1: std::sync::Arc<String>) -> bool {
        RegexNodeTrait::found( & ( * self.0), arg1)
    }
    fn find(& self, arg1: std::sync::Arc<String>) -> temper_core::Result<Match> {
        RegexNodeTrait::find( & ( * self.0), arg1)
    }
    fn replace(& self, arg1: std::sync::Arc<String>, arg2: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        RegexNodeTrait::replace( & ( * self.0), arg1, arg2)
    }
    fn split(& self, arg1: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        RegexNodeTrait::split( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(RegexNode);
impl std::ops::Deref for RegexNode {
    type Target = dyn RegexNodeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CaptureStruct {
    name: std::sync::Arc<String>, item: RegexNode
}
#[derive(Clone)]
pub struct Capture(std::sync::Arc<CaptureStruct>);
#[derive(Clone)]
pub struct CaptureBuilder {
    pub name: std::sync::Arc<String>, pub item: RegexNode
}
impl CaptureBuilder {
    pub fn build(self) -> Capture {
        Capture::new(self.name, self.item)
    }
}
impl Capture {
    pub fn new(name__187: impl temper_core::ToArcString, item__188: RegexNode) -> Capture {
        let name__187 = name__187.to_arc_string();
        let name;
        let item;
        name = name__187.clone();
        item = item__188.clone();
        let selfish = Capture(std::sync::Arc::new(CaptureStruct {
                    name, item
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
    pub fn item(& self) -> RegexNode {
        return self.0.item.clone();
    }
}
impl RegexNodeTrait for Capture {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Capture, [RegexNode]);
pub trait CodePartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + RegexNodeTrait {
    fn clone_boxed(& self) -> CodePart;
}
#[derive(Clone)]
pub struct CodePart(std::sync::Arc<dyn CodePartTrait>);
impl CodePart {
    pub fn new(selfish: impl CodePartTrait + 'static) -> CodePart {
        CodePart(std::sync::Arc::new(selfish))
    }
}
impl CodePartTrait for CodePart {
    fn clone_boxed(& self) -> CodePart {
        CodePartTrait::clone_boxed( & ( * self.0))
    }
}
impl RegexNodeTrait for CodePart {
    fn clone_boxed(& self) -> RegexNode {
        RegexNodeTrait::clone_boxed( & ( * self.0))
    }
    fn compiled(& self) -> Regex {
        RegexNodeTrait::compiled( & ( * self.0))
    }
    fn found(& self, arg1: std::sync::Arc<String>) -> bool {
        RegexNodeTrait::found( & ( * self.0), arg1)
    }
    fn find(& self, arg1: std::sync::Arc<String>) -> temper_core::Result<Match> {
        RegexNodeTrait::find( & ( * self.0), arg1)
    }
    fn replace(& self, arg1: std::sync::Arc<String>, arg2: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        RegexNodeTrait::replace( & ( * self.0), arg1, arg2)
    }
    fn split(& self, arg1: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        RegexNodeTrait::split( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(CodePart);
impl std::ops::Deref for CodePart {
    type Target = dyn CodePartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CodePointsStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct CodePoints(std::sync::Arc<CodePointsStruct>);
impl CodePoints {
    pub fn new(value__191: impl temper_core::ToArcString) -> CodePoints {
        let value__191 = value__191.to_arc_string();
        let value;
        value = value__191.clone();
        let selfish = CodePoints(std::sync::Arc::new(CodePointsStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl CodePartTrait for CodePoints {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for CodePoints {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodePoints, [CodePart, RegexNode]);
pub trait SpecialTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + RegexNodeTrait {
    fn clone_boxed(& self) -> Special;
}
#[derive(Clone)]
pub struct Special(std::sync::Arc<dyn SpecialTrait>);
impl Special {
    pub fn new(selfish: impl SpecialTrait + 'static) -> Special {
        Special(std::sync::Arc::new(selfish))
    }
}
impl SpecialTrait for Special {
    fn clone_boxed(& self) -> Special {
        SpecialTrait::clone_boxed( & ( * self.0))
    }
}
impl RegexNodeTrait for Special {
    fn clone_boxed(& self) -> RegexNode {
        RegexNodeTrait::clone_boxed( & ( * self.0))
    }
    fn compiled(& self) -> Regex {
        RegexNodeTrait::compiled( & ( * self.0))
    }
    fn found(& self, arg1: std::sync::Arc<String>) -> bool {
        RegexNodeTrait::found( & ( * self.0), arg1)
    }
    fn find(& self, arg1: std::sync::Arc<String>) -> temper_core::Result<Match> {
        RegexNodeTrait::find( & ( * self.0), arg1)
    }
    fn replace(& self, arg1: std::sync::Arc<String>, arg2: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        RegexNodeTrait::replace( & ( * self.0), arg1, arg2)
    }
    fn split(& self, arg1: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        RegexNodeTrait::split( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(Special);
impl std::ops::Deref for Special {
    type Target = dyn SpecialTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub trait SpecialSetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + CodePartTrait + SpecialTrait {
    fn clone_boxed(& self) -> SpecialSet;
}
#[derive(Clone)]
pub struct SpecialSet(std::sync::Arc<dyn SpecialSetTrait>);
impl SpecialSet {
    pub fn new(selfish: impl SpecialSetTrait + 'static) -> SpecialSet {
        SpecialSet(std::sync::Arc::new(selfish))
    }
}
impl SpecialSetTrait for SpecialSet {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSetTrait::clone_boxed( & ( * self.0))
    }
}
impl CodePartTrait for SpecialSet {
    fn clone_boxed(& self) -> CodePart {
        CodePartTrait::clone_boxed( & ( * self.0))
    }
}
impl RegexNodeTrait for SpecialSet {
    fn clone_boxed(& self) -> RegexNode {
        RegexNodeTrait::clone_boxed( & ( * self.0))
    }
    fn compiled(& self) -> Regex {
        RegexNodeTrait::compiled( & ( * self.0))
    }
    fn found(& self, arg1: std::sync::Arc<String>) -> bool {
        RegexNodeTrait::found( & ( * self.0), arg1)
    }
    fn find(& self, arg1: std::sync::Arc<String>) -> temper_core::Result<Match> {
        RegexNodeTrait::find( & ( * self.0), arg1)
    }
    fn replace(& self, arg1: std::sync::Arc<String>, arg2: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        RegexNodeTrait::replace( & ( * self.0), arg1, arg2)
    }
    fn split(& self, arg1: std::sync::Arc<String>) -> temper_core::List<std::sync::Arc<String>> {
        RegexNodeTrait::split( & ( * self.0), arg1)
    }
}
impl SpecialTrait for SpecialSet {
    fn clone_boxed(& self) -> Special {
        SpecialTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(SpecialSet);
impl std::ops::Deref for SpecialSet {
    type Target = dyn SpecialSetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct CodeRangeStruct {
    min: i32, max: i32
}
#[derive(Clone)]
pub struct CodeRange(std::sync::Arc<CodeRangeStruct>);
#[derive(Clone)]
pub struct CodeRangeBuilder {
    pub min: i32, pub max: i32
}
impl CodeRangeBuilder {
    pub fn build(self) -> CodeRange {
        CodeRange::new(self.min, self.max)
    }
}
impl CodeRange {
    pub fn new(min__209: i32, max__210: i32) -> CodeRange {
        let min;
        let max;
        min = min__209;
        max = max__210;
        let selfish = CodeRange(std::sync::Arc::new(CodeRangeStruct {
                    min, max
        }));
        return selfish;
    }
    pub fn min(& self) -> i32 {
        return self.0.min;
    }
    pub fn max(& self) -> i32 {
        return self.0.max;
    }
}
impl CodePartTrait for CodeRange {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for CodeRange {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodeRange, [CodePart, RegexNode]);
struct CodeSetStruct {
    items: temper_core::List<CodePart>, negated: bool
}
#[derive(Clone)]
pub struct CodeSet(std::sync::Arc<CodeSetStruct>);
#[derive(Clone, Default)]
pub struct CodeSetOptions {
    pub negated: Option<bool>
}
#[derive(Clone)]
pub struct CodeSetBuilder {
    pub items: temper_core::List<CodePart>
}
impl CodeSetBuilder {
    pub fn build(self) -> CodeSet {
        self.build_with(std::default::Default::default())
    }
    pub fn build_with(self, options: CodeSetOptions) -> CodeSet {
        CodeSet::new(self.items, options.negated)
    }
}
impl CodeSet {
    pub fn new(items__214: impl temper_core::ToList<CodePart>, negated__544: Option<bool>) -> CodeSet {
        let items__214 = items__214.to_list();
        let items;
        let negated;
        let negated__215: bool;
        if negated__544.is_none() {
            negated__215 = false;
        } else {
            negated__215 = negated__544.unwrap();
        }
        items = items__214.clone();
        negated = negated__215;
        let selfish = CodeSet(std::sync::Arc::new(CodeSetStruct {
                    items, negated
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<CodePart> {
        return self.0.items.clone();
    }
    pub fn negated(& self) -> bool {
        return self.0.negated;
    }
}
impl RegexNodeTrait for CodeSet {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(CodeSet, [RegexNode]);
struct OrStruct {
    items: temper_core::List<RegexNode>
}
#[derive(Clone)]
pub struct Or(std::sync::Arc<OrStruct>);
impl Or {
    pub fn new(items__218: impl temper_core::ToList<RegexNode>) -> Or {
        let items__218 = items__218.to_list();
        let items;
        items = items__218.clone();
        let selfish = Or(std::sync::Arc::new(OrStruct {
                    items
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<RegexNode> {
        return self.0.items.clone();
    }
}
impl RegexNodeTrait for Or {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Or, [RegexNode]);
struct RepeatStruct {
    item: RegexNode, min: i32, max: Option<i32>, reluctant: bool
}
#[derive(Clone)]
pub struct Repeat(std::sync::Arc<RepeatStruct>);
#[derive(Clone, Default)]
pub struct RepeatOptions {
    pub reluctant: Option<bool>
}
#[derive(Clone)]
pub struct RepeatBuilder {
    pub item: RegexNode, pub min: i32, pub max: Option<i32>
}
impl RepeatBuilder {
    pub fn build(self) -> Repeat {
        self.build_with(std::default::Default::default())
    }
    pub fn build_with(self, options: RepeatOptions) -> Repeat {
        Repeat::new(self.item, self.min, self.max, options.reluctant)
    }
}
impl Repeat {
    pub fn new(item__224: RegexNode, min__225: i32, max__226: Option<i32>, reluctant__546: Option<bool>) -> Repeat {
        let item;
        let min;
        let max;
        let reluctant;
        let reluctant__227: bool;
        if reluctant__546.is_none() {
            reluctant__227 = false;
        } else {
            reluctant__227 = reluctant__546.unwrap();
        }
        item = item__224.clone();
        min = min__225;
        max = max__226;
        reluctant = reluctant__227;
        let selfish = Repeat(std::sync::Arc::new(RepeatStruct {
                    item, min, max, reluctant
        }));
        return selfish;
    }
    pub fn item(& self) -> RegexNode {
        return self.0.item.clone();
    }
    pub fn min(& self) -> i32 {
        return self.0.min;
    }
    pub fn max(& self) -> Option<i32> {
        return self.0.max;
    }
    pub fn reluctant(& self) -> bool {
        return self.0.reluctant;
    }
}
impl RegexNodeTrait for Repeat {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Repeat, [RegexNode]);
struct SequenceStruct {
    items: temper_core::List<RegexNode>
}
#[derive(Clone)]
pub struct Sequence(std::sync::Arc<SequenceStruct>);
impl Sequence {
    pub fn new(items__238: impl temper_core::ToList<RegexNode>) -> Sequence {
        let items__238 = items__238.to_list();
        let items;
        items = items__238.clone();
        let selfish = Sequence(std::sync::Arc::new(SequenceStruct {
                    items
        }));
        return selfish;
    }
    pub fn items(& self) -> temper_core::List<RegexNode> {
        return self.0.items.clone();
    }
}
impl RegexNodeTrait for Sequence {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Sequence, [RegexNode]);
struct MatchStruct {
    full: Group, groups: temper_core::Map<std::sync::Arc<String>, Group>
}
#[derive(Clone)]
pub struct Match(std::sync::Arc<MatchStruct>);
#[derive(Clone)]
pub struct MatchBuilder {
    pub full: Group, pub groups: temper_core::Map<std::sync::Arc<String>, Group>
}
impl MatchBuilder {
    pub fn build(self) -> Match {
        Match::new(self.full, self.groups)
    }
}
impl Match {
    pub fn new(full__242: Group, groups__243: temper_core::Map<std::sync::Arc<String>, Group>) -> Match {
        let full;
        let groups;
        full = full__242.clone();
        groups = groups__243.clone();
        let selfish = Match(std::sync::Arc::new(MatchStruct {
                    full, groups
        }));
        return selfish;
    }
    pub fn full(& self) -> Group {
        return self.0.full.clone();
    }
    pub fn groups(& self) -> temper_core::Map<std::sync::Arc<String>, Group> {
        return self.0.groups.clone();
    }
}
temper_core::impl_any_value_trait!(Match, []);
struct GroupStruct {
    name: std::sync::Arc<String>, value: std::sync::Arc<String>, begin: usize, end: usize
}
#[derive(Clone)]
pub struct Group(std::sync::Arc<GroupStruct>);
#[derive(Clone)]
pub struct GroupBuilder {
    pub name: std::sync::Arc<String>, pub value: std::sync::Arc<String>, pub begin: usize, pub end: usize
}
impl GroupBuilder {
    pub fn build(self) -> Group {
        Group::new(self.name, self.value, self.begin, self.end)
    }
}
impl Group {
    pub fn new(name__249: impl temper_core::ToArcString, value__250: impl temper_core::ToArcString, begin__251: usize, end__252: usize) -> Group {
        let name__249 = name__249.to_arc_string();
        let value__250 = value__250.to_arc_string();
        let name;
        let value;
        let begin;
        let end;
        name = name__249.clone();
        value = value__250.clone();
        begin = begin__251;
        end = end__252;
        let selfish = Group(std::sync::Arc::new(GroupStruct {
                    name, value, begin, end
        }));
        return selfish;
    }
    pub fn name(& self) -> std::sync::Arc<String> {
        return self.0.name.clone();
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn begin(& self) -> usize {
        return self.0.begin;
    }
    pub fn end(& self) -> usize {
        return self.0.end;
    }
}
temper_core::impl_any_value_trait!(Group, []);
struct RegexRefsStruct {
    code_points: CodePoints, group: Group, r#match: Match, or_object: Or
}
#[derive(Clone)]
pub (crate) struct RegexRefs(std::sync::Arc<RegexRefsStruct>);
impl RegexRefs {
    pub fn new(codePoints__548: Option<CodePoints>, group__550: Option<Group>, match__552: Option<Match>, orObject__554: Option<Or>) -> RegexRefs {
        let code_points;
        let group;
        let r#match;
        let or_object;
        let mut t___1292: CodePoints;
        let mut t___1293: Group;
        let mut t___1295: temper_core::Map<std::sync::Arc<String>, Group>;
        let mut t___1296: Match;
        let mut t___1297: Or;
        let codePoints__258: CodePoints;
        if codePoints__548.is_none() {
            t___1292 = CodePoints::new("");
            codePoints__258 = t___1292.clone();
        } else {
            codePoints__258 = codePoints__548.clone().unwrap();
        }
        let group__259: Group;
        if group__550.is_none() {
            t___1293 = Group::new("", "", 0usize, 0usize);
            group__259 = t___1293.clone();
        } else {
            group__259 = group__550.clone().unwrap();
        }
        let match__260: Match;
        if match__552.is_none() {
            t___1295 = temper_core::Map::new( & [(std::sync::Arc::new("".to_string()), group__259.clone())]);
            t___1296 = Match::new(group__259.clone(), t___1295.clone());
            match__260 = t___1296.clone();
        } else {
            match__260 = match__552.clone().unwrap();
        }
        let orObject__261: Or;
        if orObject__554.is_none() {
            t___1297 = Or::new([]);
            orObject__261 = t___1297.clone();
        } else {
            orObject__261 = orObject__554.clone().unwrap();
        }
        code_points = codePoints__258.clone();
        group = group__259.clone();
        r#match = match__260.clone();
        or_object = orObject__261.clone();
        let selfish = RegexRefs(std::sync::Arc::new(RegexRefsStruct {
                    code_points, group, r#match, or_object
        }));
        return selfish;
    }
    pub fn code_points(& self) -> CodePoints {
        return self.0.code_points.clone();
    }
    pub fn group(& self) -> Group {
        return self.0.group.clone();
    }
    pub fn r#match(& self) -> Match {
        return self.0.r#match.clone();
    }
    pub fn or_object(& self) -> Or {
        return self.0.or_object.clone();
    }
}
temper_core::impl_any_value_trait!(RegexRefs, []);
struct RegexStruct {
    data: RegexNode, compiled: temper_core::AnyValue
}
#[derive(Clone)]
pub struct Regex(std::sync::Arc<RegexStruct>);
impl Regex {
    pub fn new(data__264: RegexNode) -> Regex {
        let data;
        let compiled;
        let t___421: RegexNode = data__264.clone();
        data = t___421.clone();
        let formatted__266: std::sync::Arc<String> = RegexFormatter::regex_format(data__264.clone());
        let mut t___1171: temper_core::AnyValue = compile_formatted( & ( * data__264), formatted__266.clone());
        compiled = t___1171.clone();
        let selfish = Regex(std::sync::Arc::new(RegexStruct {
                    data, compiled
        }));
        return selfish;
    }
    pub fn found(& self, text__268: impl temper_core::ToArcString) -> bool {
        let text__268 = text__268.to_arc_string();
        return compiled_found( & self, self.0.compiled.clone(), text__268.clone());
    }
    pub fn find(& self, text__271: impl temper_core::ToArcString, begin__556: Option<usize>) -> temper_core::Result<Match> {
        let text__271 = text__271.to_arc_string();
        let return__133: Match;
        let begin__272: usize;
        if begin__556.is_none() {
            begin__272 = 0usize;
        } else {
            begin__272 = begin__556.unwrap();
        }
        return__133 = compiled_find( & self, self.0.compiled.clone(), text__271.clone(), begin__272, regex_refs().clone()) ? ;
        return Ok(return__133.clone());
    }
    pub fn replace(& self, text__275: impl temper_core::ToArcString, format__276: std::sync::Arc<dyn Fn (Match) -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> std::sync::Arc<String> {
        let text__275 = text__275.to_arc_string();
        return compiled_replace( & self, self.0.compiled.clone(), text__275.clone(), & ( * format__276.clone()), regex_refs().clone());
    }
    pub fn split(& self, text__279: impl temper_core::ToArcString) -> temper_core::List<std::sync::Arc<String>> {
        let text__279 = text__279.to_arc_string();
        return compiled_split( & self, self.0.compiled.clone(), text__279.clone(), regex_refs().clone());
    }
    pub fn data(& self) -> RegexNode {
        return self.0.data.clone();
    }
}
temper_core::impl_any_value_trait!(Regex, []);
struct RegexFormatterStruct {
    out: std::sync::Arc<std::sync::RwLock<String>>
}
#[derive(Clone)]
pub (crate) struct RegexFormatter(std::sync::Arc<RegexFormatterStruct>);
impl RegexFormatter {
    pub fn regex_format(data__309: RegexNode) -> std::sync::Arc<String> {
        return RegexFormatter::new().format(data__309.clone());
    }
    pub fn format(& self, regex__312: RegexNode) -> std::sync::Arc<String> {
        self.push_regex(regex__312.clone());
        return temper_core::string::builder::to_string( & self.0.out);
    }
    fn push_regex(& self, regex__315: RegexNode) {
        let mut t___894: Capture;
        let mut t___895: CodePoints;
        let mut t___896: CodeRange;
        let mut t___897: CodeSet;
        let mut t___898: Or;
        let mut t___899: Repeat;
        let mut t___900: Sequence;
        if temper_core::is::<Capture>(regex__315.clone()) {
            t___894 = temper_core::cast::<Capture>(regex__315.clone()).unwrap();
            self.push_capture(t___894.clone());
        } else {
            if temper_core::is::<CodePoints>(regex__315.clone()) {
                t___895 = temper_core::cast::<CodePoints>(regex__315.clone()).unwrap();
                self.push_code_points(t___895.clone(), false);
            } else {
                if temper_core::is::<CodeRange>(regex__315.clone()) {
                    t___896 = temper_core::cast::<CodeRange>(regex__315.clone()).unwrap();
                    self.push_code_range(t___896.clone());
                } else {
                    if temper_core::is::<CodeSet>(regex__315.clone()) {
                        t___897 = temper_core::cast::<CodeSet>(regex__315.clone()).unwrap();
                        self.push_code_set(t___897.clone());
                    } else {
                        if temper_core::is::<Or>(regex__315.clone()) {
                            t___898 = temper_core::cast::<Or>(regex__315.clone()).unwrap();
                            self.push_or(t___898.clone());
                        } else {
                            if temper_core::is::<Repeat>(regex__315.clone()) {
                                t___899 = temper_core::cast::<Repeat>(regex__315.clone()).unwrap();
                                self.push_repeat(t___899.clone());
                            } else {
                                if temper_core::is::<Sequence>(regex__315.clone()) {
                                    t___900 = temper_core::cast::<Sequence>(regex__315.clone()).unwrap();
                                    self.push_sequence(t___900.clone());
                                } else {
                                    if regex__315.ptr_id() == begin().ptr_id() {
                                        temper_core::string::builder::append( & self.0.out, "^");
                                    } else {
                                        if regex__315.ptr_id() == dot().ptr_id() {
                                            temper_core::string::builder::append( & self.0.out, ".");
                                        } else {
                                            if regex__315.ptr_id() == end().ptr_id() {
                                                temper_core::string::builder::append( & self.0.out, "$");
                                            } else {
                                                if regex__315.ptr_id() == word_boundary().ptr_id() {
                                                    temper_core::string::builder::append( & self.0.out, "\\b");
                                                } else {
                                                    if regex__315.ptr_id() == digit().ptr_id() {
                                                        temper_core::string::builder::append( & self.0.out, "\\d");
                                                    } else {
                                                        if regex__315.ptr_id() == space().ptr_id() {
                                                            temper_core::string::builder::append( & self.0.out, "\\s");
                                                        } else {
                                                            if regex__315.ptr_id() == word().ptr_id() {
                                                                temper_core::string::builder::append( & self.0.out, "\\w");
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn push_capture(& self, capture__318: Capture) {
        temper_core::string::builder::append( & self.0.out, "(");
        let mut t___868: std::sync::Arc<std::sync::RwLock<String>> = self.0.out.clone();
        let mut t___1262: std::sync::Arc<String> = capture__318.name();
        self.push_capture_name(t___868.clone(), t___1262.clone());
        let mut t___1264: RegexNode = capture__318.item();
        self.push_regex(t___1264.clone());
        temper_core::string::builder::append( & self.0.out, ")");
    }
    fn push_capture_name(& self, out__321: std::sync::Arc<std::sync::RwLock<String>>, name__322: impl temper_core::ToArcString) {
        let name__322 = name__322.to_arc_string();
        temper_core::string::builder::append( & out__321, std::sync::Arc::new(format!("?<{}>", name__322.clone())));
    }
    fn push_code(& self, code__325: i32, insideCodeSet__326: bool) {
        let return__148: ();
        let mut t___856: bool;
        let mut t___857: bool;
        let mut t___858: std::sync::Arc<String>;
        let mut t___860: std::sync::Arc<String>;
        let mut t___861: bool;
        let mut t___862: bool;
        let mut t___863: bool;
        let mut t___864: bool;
        let mut t___865: std::sync::Arc<String>;
        'fn__327: {
            'ok___2629: {
                'orelse___562: {
                    let specialEscape__328: std::sync::Arc<String>;
                    if Some(code__325) == Some(Codes::carriage_return()) {
                        specialEscape__328 = std::sync::Arc::new("r".to_string());
                    } else {
                        if Some(code__325) == Some(Codes::newline()) {
                            specialEscape__328 = std::sync::Arc::new("n".to_string());
                        } else {
                            if Some(code__325) == Some(Codes::tab()) {
                                specialEscape__328 = std::sync::Arc::new("t".to_string());
                            } else {
                                specialEscape__328 = std::sync::Arc::new("".to_string());
                            }
                        }
                    }
                    if Some(specialEscape__328.as_str()) != Some("") {
                        temper_core::string::builder::append( & self.0.out, "\\");
                        temper_core::string::builder::append( & self.0.out, specialEscape__328.clone());
                        return__148 = ();
                        break 'fn__327;
                    }
                    if Some(code__325) <= Some(127) {
                        let escapeNeed__329: i32 = temper_core::ListedTrait::get( & escape_needs(), code__325);
                        if escapeNeed__329 == 2{
                            t___857 = true;
                        } else {
                            if insideCodeSet__326 {
                                t___856 = Some(code__325) == Some(Codes::dash());
                            } else {
                                t___856 = false;
                            }
                            t___857 = t___856;
                        }
                        if t___857 {
                            temper_core::string::builder::append( & self.0.out, "\\");
                            t___858 = match temper_core::string::from_code_point(code__325) {
                                Ok(x) => x,
                                _ => break 'orelse___562
                            };
                            temper_core::string::builder::append( & self.0.out, t___858.clone());
                            return__148 = ();
                            break 'fn__327;
                        } else {
                            if escapeNeed__329 == 0{
                                t___860 = match temper_core::string::from_code_point(code__325) {
                                    Ok(x) => x,
                                    _ => break 'orelse___562
                                };
                                temper_core::string::builder::append( & self.0.out, t___860.clone());
                                return__148 = ();
                                break 'fn__327;
                            }
                        }
                    }
                    if Some(code__325) >= Some(Codes::supplemental_min()) {
                        t___864 = true;
                    } else {
                        if Some(code__325) > Some(Codes::high_control_max()) {
                            if Some(Codes::surrogate_min()) <= Some(code__325) {
                                t___861 = Some(code__325) <= Some(Codes::surrogate_max());
                            } else {
                                t___861 = false;
                            }
                            if t___861 {
                                t___862 = true;
                            } else {
                                t___862 = Some(code__325) == Some(Codes::uint16_max());
                            }
                            t___863 = ! t___862;
                        } else {
                            t___863 = false;
                        }
                        t___864 = t___863;
                    }
                    if t___864 {
                        t___865 = match temper_core::string::from_code_point(code__325) {
                            Ok(x) => x,
                            _ => break 'orelse___562
                        };
                        temper_core::string::builder::append( & self.0.out, t___865.clone());
                    } else {
                        push_code_to( & self, self.0.out.clone(), code__325, insideCodeSet__326);
                    }
                    break 'ok___2629;
                }
                return panic!();
            }
            return__148 = ();
        }
        return return__148;
    }
    fn push_code_points(& self, codePoints__336: CodePoints, insideCodeSet__337: bool) {
        let mut t___1249: i32;
        let mut t___1251: usize;
        let value__339: std::sync::Arc<String> = codePoints__336.value();
        let mut index__340: usize = 0usize;
        'loop___2673: loop {
            if ! temper_core::string::has_index( & value__339, index__340) {
                break;
            }
            t___1249 = temper_core::string::get( & value__339, index__340);
            self.push_code(t___1249, insideCodeSet__337);
            t___1251 = temper_core::string::next( & value__339, index__340);
            index__340 = t___1251;
        }
    }
    fn push_code_range(& self, codeRange__342: CodeRange) {
        temper_core::string::builder::append( & self.0.out, "[");
        self.push_code_range_unwrapped(codeRange__342.clone());
        temper_core::string::builder::append( & self.0.out, "]");
    }
    fn push_code_range_unwrapped(& self, codeRange__345: CodeRange) {
        let mut t___1239: i32 = codeRange__345.min();
        self.push_code(t___1239, true);
        temper_core::string::builder::append( & self.0.out, "-");
        let mut t___1242: i32 = codeRange__345.max();
        self.push_code(t___1242, true);
    }
    fn push_code_set(& self, codeSet__348: CodeSet) {
        let mut t___1233: i32;
        let mut t___1235: CodePart;
        let mut t___841: CodeSet;
        let adjusted__350: RegexNode = self.adjust_code_set(codeSet__348.clone(), regex_refs().clone());
        if temper_core::is::<CodeSet>(adjusted__350.clone()) {
            t___841 = temper_core::cast::<CodeSet>(adjusted__350.clone()).unwrap();
            if temper_core::ListedTrait::is_empty( & t___841.items()) {
                if t___841.negated() {
                    temper_core::string::builder::append( & self.0.out, "[\\s\\S]");
                } else {
                    temper_core::string::builder::append( & self.0.out, "(?:$.)");
                }
            } else {
                temper_core::string::builder::append( & self.0.out, "[");
                if t___841.negated() {
                    temper_core::string::builder::append( & self.0.out, "^");
                }
                let mut i__351: i32 = 0;
                'loop___2674: loop {
                    t___1233 = temper_core::ListedTrait::len( & t___841.items());
                    if ! (Some(i__351) < Some(t___1233)) {
                        break;
                    }
                    t___1235 = temper_core::ListedTrait::get( & t___841.items(), i__351);
                    self.push_code_set_item(t___1235.clone());
                    i__351 = i__351.wrapping_add(1);
                }
                temper_core::string::builder::append( & self.0.out, "]");
            }
        } else {
            self.push_regex(adjusted__350.clone());
        }
    }
    fn adjust_code_set(& self, codeSet__353: CodeSet, regexRefs__354: RegexRefs) -> RegexNode {
        return RegexNode::new(codeSet__353.clone());
    }
    fn push_code_set_item(& self, codePart__357: CodePart) {
        let mut t___826: CodePoints;
        let mut t___827: CodeRange;
        let mut t___828: SpecialSet;
        if temper_core::is::<CodePoints>(codePart__357.clone()) {
            t___826 = temper_core::cast::<CodePoints>(codePart__357.clone()).unwrap();
            self.push_code_points(t___826.clone(), true);
        } else {
            if temper_core::is::<CodeRange>(codePart__357.clone()) {
                t___827 = temper_core::cast::<CodeRange>(codePart__357.clone()).unwrap();
                self.push_code_range_unwrapped(t___827.clone());
            } else {
                if temper_core::is::<SpecialSet>(codePart__357.clone()) {
                    t___828 = temper_core::cast::<SpecialSet>(codePart__357.clone()).unwrap();
                    self.push_regex(temper_core::cast::<RegexNode>(t___828.clone()).unwrap());
                }
            }
        }
    }
    fn push_or(& self, or__360: Or) {
        let mut t___1207: RegexNode;
        let mut t___1210: i32;
        let mut t___1213: RegexNode;
        if ! temper_core::ListedTrait::is_empty( & or__360.items()) {
            temper_core::string::builder::append( & self.0.out, "(?:");
            t___1207 = temper_core::ListedTrait::get( & or__360.items(), 0);
            self.push_regex(t___1207.clone());
            let mut i__362: i32 = 1;
            'loop___2675: loop {
                t___1210 = temper_core::ListedTrait::len( & or__360.items());
                if ! (Some(i__362) < Some(t___1210)) {
                    break;
                }
                temper_core::string::builder::append( & self.0.out, "|");
                t___1213 = temper_core::ListedTrait::get( & or__360.items(), i__362);
                self.push_regex(t___1213.clone());
                i__362 = i__362.wrapping_add(1);
            }
            temper_core::string::builder::append( & self.0.out, ")");
        }
    }
    fn push_repeat(& self, repeat__364: Repeat) {
        let mut t___1195: std::sync::Arc<String>;
        let mut t___1198: std::sync::Arc<String>;
        let mut t___803: bool;
        let mut t___804: bool;
        let mut t___805: bool;
        temper_core::string::builder::append( & self.0.out, "(?:");
        let mut t___1187: RegexNode = repeat__364.item();
        self.push_regex(t___1187.clone());
        temper_core::string::builder::append( & self.0.out, ")");
        let min__366: i32 = repeat__364.min();
        let max__367: Option<i32> = repeat__364.max();
        if Some(min__366) == Some(0) {
            t___803 = max__367 == Some(1);
        } else {
            t___803 = false;
        }
        if t___803 {
            temper_core::string::builder::append( & self.0.out, "?");
        } else {
            if Some(min__366) == Some(0) {
                t___804 = max__367.is_none();
            } else {
                t___804 = false;
            }
            if t___804 {
                temper_core::string::builder::append( & self.0.out, "*");
            } else {
                if Some(min__366) == Some(1) {
                    t___805 = max__367.is_none();
                } else {
                    t___805 = false;
                }
                if t___805 {
                    temper_core::string::builder::append( & self.0.out, "+");
                } else {
                    t___1195 = temper_core::int_to_string(min__366, None);
                    temper_core::string::builder::append( & self.0.out, std::sync::Arc::new(format!("{{{}", t___1195.clone())));
                    if Some(min__366) != max__367 {
                        temper_core::string::builder::append( & self.0.out, ",");
                        if ! max__367.is_none() {
                            t___1198 = temper_core::int_to_string(max__367.unwrap(), None);
                            temper_core::string::builder::append( & self.0.out, t___1198.clone());
                        }
                    }
                    temper_core::string::builder::append( & self.0.out, "}");
                }
            }
        }
        if repeat__364.reluctant() {
            temper_core::string::builder::append( & self.0.out, "?");
        }
    }
    fn push_sequence(& self, sequence__369: Sequence) {
        let mut t___1182: i32;
        let mut t___1184: RegexNode;
        let mut i__371: i32 = 0;
        'loop___2679: loop {
            t___1182 = temper_core::ListedTrait::len( & sequence__369.items());
            if ! (Some(i__371) < Some(t___1182)) {
                break;
            }
            t___1184 = temper_core::ListedTrait::get( & sequence__369.items(), i__371);
            self.push_regex(t___1184.clone());
            i__371 = i__371.wrapping_add(1);
        }
    }
    pub fn max_code(& self, codePart__373: CodePart) -> Option<i32> {
        let return__159: Option<i32>;
        let mut t___1178: usize;
        let mut t___791: CodePoints;
        if temper_core::is::<CodePoints>(codePart__373.clone()) {
            t___791 = temper_core::cast::<CodePoints>(codePart__373.clone()).unwrap();
            let value__375: std::sync::Arc<String> = t___791.value();
            if value__375.is_empty() {
                return__159 = None;
            } else {
                let mut max__376: i32 = 0;
                let mut index__377: usize = 0usize;
                'loop___2680: loop {
                    if ! temper_core::string::has_index( & value__375, index__377) {
                        break;
                    }
                    let next__378: i32 = temper_core::string::get( & value__375, index__377);
                    if Some(next__378) > Some(max__376) {
                        max__376 = next__378;
                    }
                    t___1178 = temper_core::string::next( & value__375, index__377);
                    index__377 = t___1178;
                }
                return__159 = Some(max__376);
            }
        } else {
            if temper_core::is::<CodeRange>(codePart__373.clone()) {
                return__159 = Some(temper_core::cast::<CodeRange>(codePart__373.clone()).unwrap().max());
            } else {
                if codePart__373.ptr_id() == digit().ptr_id() {
                    return__159 = Some(Codes::digit9());
                } else {
                    if codePart__373.ptr_id() == space().ptr_id() {
                        return__159 = Some(Codes::space());
                    } else {
                        if codePart__373.ptr_id() == word().ptr_id() {
                            return__159 = Some(Codes::lower_z());
                        } else {
                            return__159 = None;
                        }
                    }
                }
            }
        }
        return return__159;
    }
    pub fn new() -> RegexFormatter {
        let out;
        let mut t___1172: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        out = t___1172.clone();
        let selfish = RegexFormatter(std::sync::Arc::new(RegexFormatterStruct {
                    out
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(RegexFormatter, []);
struct CodesStruct {}
#[derive(Clone)]
pub (crate) struct Codes(std::sync::Arc<CodesStruct>);
static CODES__AMPERSAND: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__BACKSLASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CARET: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CARRIAGE_RETURN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CURLY_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__CURLY_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DOT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__HIGH_CONTROL_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__HIGH_CONTROL_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DIGIT0: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__DIGIT9: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__LOWER_A: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__LOWER_Z: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__NEWLINE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PESO: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PIPE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__PLUS: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__QUESTION: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__ROUND_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__ROUND_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SLASH: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SQUARE_LEFT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SQUARE_RIGHT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__STAR: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__TAB: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__TILDE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UPPER_A: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UPPER_Z: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SPACE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SURROGATE_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SURROGATE_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__SUPPLEMENTAL_MIN: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UINT16_MAX: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
static CODES__UNDERSCORE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
impl Codes {
    pub fn ampersand() -> i32 {
        * CODES__AMPERSAND.get().unwrap()
    }
    pub fn backslash() -> i32 {
        * CODES__BACKSLASH.get().unwrap()
    }
    pub fn caret() -> i32 {
        * CODES__CARET.get().unwrap()
    }
    pub fn carriage_return() -> i32 {
        * CODES__CARRIAGE_RETURN.get().unwrap()
    }
    pub fn curly_left() -> i32 {
        * CODES__CURLY_LEFT.get().unwrap()
    }
    pub fn curly_right() -> i32 {
        * CODES__CURLY_RIGHT.get().unwrap()
    }
    pub fn dash() -> i32 {
        * CODES__DASH.get().unwrap()
    }
    pub fn dot() -> i32 {
        * CODES__DOT.get().unwrap()
    }
    pub fn high_control_min() -> i32 {
        * CODES__HIGH_CONTROL_MIN.get().unwrap()
    }
    pub fn high_control_max() -> i32 {
        * CODES__HIGH_CONTROL_MAX.get().unwrap()
    }
    pub fn digit0() -> i32 {
        * CODES__DIGIT0.get().unwrap()
    }
    pub fn digit9() -> i32 {
        * CODES__DIGIT9.get().unwrap()
    }
    pub fn lower_a() -> i32 {
        * CODES__LOWER_A.get().unwrap()
    }
    pub fn lower_z() -> i32 {
        * CODES__LOWER_Z.get().unwrap()
    }
    pub fn newline() -> i32 {
        * CODES__NEWLINE.get().unwrap()
    }
    pub fn peso() -> i32 {
        * CODES__PESO.get().unwrap()
    }
    pub fn pipe() -> i32 {
        * CODES__PIPE.get().unwrap()
    }
    pub fn plus() -> i32 {
        * CODES__PLUS.get().unwrap()
    }
    pub fn question() -> i32 {
        * CODES__QUESTION.get().unwrap()
    }
    pub fn round_left() -> i32 {
        * CODES__ROUND_LEFT.get().unwrap()
    }
    pub fn round_right() -> i32 {
        * CODES__ROUND_RIGHT.get().unwrap()
    }
    pub fn slash() -> i32 {
        * CODES__SLASH.get().unwrap()
    }
    pub fn square_left() -> i32 {
        * CODES__SQUARE_LEFT.get().unwrap()
    }
    pub fn square_right() -> i32 {
        * CODES__SQUARE_RIGHT.get().unwrap()
    }
    pub fn star() -> i32 {
        * CODES__STAR.get().unwrap()
    }
    pub fn tab() -> i32 {
        * CODES__TAB.get().unwrap()
    }
    pub fn tilde() -> i32 {
        * CODES__TILDE.get().unwrap()
    }
    pub fn upper_a() -> i32 {
        * CODES__UPPER_A.get().unwrap()
    }
    pub fn upper_z() -> i32 {
        * CODES__UPPER_Z.get().unwrap()
    }
    pub fn space() -> i32 {
        * CODES__SPACE.get().unwrap()
    }
    pub fn surrogate_min() -> i32 {
        * CODES__SURROGATE_MIN.get().unwrap()
    }
    pub fn surrogate_max() -> i32 {
        * CODES__SURROGATE_MAX.get().unwrap()
    }
    pub fn supplemental_min() -> i32 {
        * CODES__SUPPLEMENTAL_MIN.get().unwrap()
    }
    pub fn uint16_max() -> i32 {
        * CODES__UINT16_MAX.get().unwrap()
    }
    pub fn underscore() -> i32 {
        * CODES__UNDERSCORE.get().unwrap()
    }
    pub fn new() -> Codes {
        let selfish = Codes(std::sync::Arc::new(CodesStruct {}));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(Codes, []);
struct BeginStruct {}
#[derive(Clone)]
pub (crate) struct Begin(std::sync::Arc<BeginStruct>);
impl Begin {
    pub fn new() -> Begin {
        let selfish = Begin(std::sync::Arc::new(BeginStruct {}));
        return selfish;
    }
}
impl SpecialTrait for Begin {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for Begin {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Begin, [Special, RegexNode]);
struct DotStruct {}
#[derive(Clone)]
pub (crate) struct Dot(std::sync::Arc<DotStruct>);
impl Dot {
    pub fn new() -> Dot {
        let selfish = Dot(std::sync::Arc::new(DotStruct {}));
        return selfish;
    }
}
impl SpecialTrait for Dot {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for Dot {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Dot, [Special, RegexNode]);
struct EndStruct {}
#[derive(Clone)]
pub (crate) struct End(std::sync::Arc<EndStruct>);
impl End {
    pub fn new() -> End {
        let selfish = End(std::sync::Arc::new(EndStruct {}));
        return selfish;
    }
}
impl SpecialTrait for End {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for End {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(End, [Special, RegexNode]);
struct WordBoundaryStruct {}
#[derive(Clone)]
pub (crate) struct WordBoundary(std::sync::Arc<WordBoundaryStruct>);
impl WordBoundary {
    pub fn new() -> WordBoundary {
        let selfish = WordBoundary(std::sync::Arc::new(WordBoundaryStruct {}));
        return selfish;
    }
}
impl SpecialTrait for WordBoundary {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
impl RegexNodeTrait for WordBoundary {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(WordBoundary, [Special, RegexNode]);
struct DigitStruct {}
#[derive(Clone)]
pub (crate) struct Digit(std::sync::Arc<DigitStruct>);
impl Digit {
    pub fn new() -> Digit {
        let selfish = Digit(std::sync::Arc::new(DigitStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Digit {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Digit {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Digit {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Digit {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Digit, [SpecialSet, CodePart, RegexNode, Special]);
struct SpaceStruct {}
#[derive(Clone)]
pub (crate) struct Space(std::sync::Arc<SpaceStruct>);
impl Space {
    pub fn new() -> Space {
        let selfish = Space(std::sync::Arc::new(SpaceStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Space {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Space {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Space {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Space {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Space, [SpecialSet, CodePart, RegexNode, Special]);
struct WordStruct {}
#[derive(Clone)]
pub (crate) struct Word(std::sync::Arc<WordStruct>);
impl Word {
    pub fn new() -> Word {
        let selfish = Word(std::sync::Arc::new(WordStruct {}));
        return selfish;
    }
}
impl SpecialSetTrait for Word {
    fn clone_boxed(& self) -> SpecialSet {
        SpecialSet::new(self.clone())
    }
}
impl CodePartTrait for Word {
    fn clone_boxed(& self) -> CodePart {
        CodePart::new(self.clone())
    }
}
impl RegexNodeTrait for Word {
    fn clone_boxed(& self) -> RegexNode {
        RegexNode::new(self.clone())
    }
}
impl SpecialTrait for Word {
    fn clone_boxed(& self) -> Special {
        Special::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Word, [SpecialSet, CodePart, RegexNode, Special]);
fn buildEscapeNeeds__163() -> temper_core::List<i32> {
    let mut t___935: bool;
    let mut t___936: bool;
    let mut t___937: bool;
    let mut t___938: bool;
    let mut t___939: bool;
    let mut t___940: bool;
    let mut t___941: bool;
    let mut t___942: bool;
    let mut t___943: bool;
    let mut t___944: bool;
    let mut t___945: bool;
    let mut t___946: bool;
    let mut t___947: bool;
    let mut t___948: bool;
    let mut t___949: bool;
    let mut t___950: bool;
    let mut t___951: bool;
    let mut t___952: bool;
    let mut t___953: bool;
    let mut t___954: bool;
    let mut t___955: bool;
    let mut t___956: bool;
    let mut t___957: bool;
    let mut t___958: bool;
    let mut t___959: i32;
    let escapeNeeds__381: temper_core::ListBuilder<i32> = temper_core::listed::new_builder();
    let mut code__382: i32 = 0;
    'loop___2682: while Some(code__382) <= Some(127) {
        if Some(code__382) == Some(Codes::dash()) {
            t___942 = true;
        } else {
            if Some(code__382) == Some(Codes::space()) {
                t___941 = true;
            } else {
                if Some(code__382) == Some(Codes::underscore()) {
                    t___940 = true;
                } else {
                    if Some(Codes::digit0()) <= Some(code__382) {
                        t___935 = Some(code__382) <= Some(Codes::digit9());
                    } else {
                        t___935 = false;
                    }
                    if t___935 {
                        t___939 = true;
                    } else {
                        if Some(Codes::upper_a()) <= Some(code__382) {
                            t___936 = Some(code__382) <= Some(Codes::upper_z());
                        } else {
                            t___936 = false;
                        }
                        if t___936 {
                            t___938 = true;
                        } else {
                            if Some(Codes::lower_a()) <= Some(code__382) {
                                t___937 = Some(code__382) <= Some(Codes::lower_z());
                            } else {
                                t___937 = false;
                            }
                            t___938 = t___937;
                        }
                        t___939 = t___938;
                    }
                    t___940 = t___939;
                }
                t___941 = t___940;
            }
            t___942 = t___941;
        }
        if t___942 {
            t___959 = 0;
        } else {
            if Some(code__382) == Some(Codes::ampersand()) {
                t___958 = true;
            } else {
                if Some(code__382) == Some(Codes::backslash()) {
                    t___957 = true;
                } else {
                    if Some(code__382) == Some(Codes::caret()) {
                        t___956 = true;
                    } else {
                        if Some(code__382) == Some(Codes::curly_left()) {
                            t___955 = true;
                        } else {
                            if Some(code__382) == Some(Codes::curly_right()) {
                                t___954 = true;
                            } else {
                                if Some(code__382) == Some(Codes::dot()) {
                                    t___953 = true;
                                } else {
                                    if Some(code__382) == Some(Codes::peso()) {
                                        t___952 = true;
                                    } else {
                                        if Some(code__382) == Some(Codes::pipe()) {
                                            t___951 = true;
                                        } else {
                                            if Some(code__382) == Some(Codes::plus()) {
                                                t___950 = true;
                                            } else {
                                                if Some(code__382) == Some(Codes::question()) {
                                                    t___949 = true;
                                                } else {
                                                    if Some(code__382) == Some(Codes::round_left()) {
                                                        t___948 = true;
                                                    } else {
                                                        if Some(code__382) == Some(Codes::round_right()) {
                                                            t___947 = true;
                                                        } else {
                                                            if Some(code__382) == Some(Codes::slash()) {
                                                                t___946 = true;
                                                            } else {
                                                                if Some(code__382) == Some(Codes::square_left()) {
                                                                    t___945 = true;
                                                                } else {
                                                                    if Some(code__382) == Some(Codes::square_right()) {
                                                                        t___944 = true;
                                                                    } else {
                                                                        if Some(code__382) == Some(Codes::star()) {
                                                                            t___943 = true;
                                                                        } else {
                                                                            t___943 = Some(code__382) == Some(Codes::tilde());
                                                                        }
                                                                        t___944 = t___943;
                                                                    }
                                                                    t___945 = t___944;
                                                                }
                                                                t___946 = t___945;
                                                            }
                                                            t___947 = t___946;
                                                        }
                                                        t___948 = t___947;
                                                    }
                                                    t___949 = t___948;
                                                }
                                                t___950 = t___949;
                                            }
                                            t___951 = t___950;
                                        }
                                        t___952 = t___951;
                                    }
                                    t___953 = t___952;
                                }
                                t___954 = t___953;
                            }
                            t___955 = t___954;
                        }
                        t___956 = t___955;
                    }
                    t___957 = t___956;
                }
                t___958 = t___957;
            }
            if t___958 {
                t___959 = 2;
            } else {
                t___959 = 1;
            }
        }
        temper_core::listed::add( & escapeNeeds__381, t___959, None);
        code__382 = code__382.wrapping_add(1);
    }
    return temper_core::ListedTrait::to_list( & escapeNeeds__381);
}
pub fn entire(item__228: RegexNode) -> RegexNode {
    return RegexNode::new(Sequence::new([temper_core::cast::<RegexNode>(begin().clone()).unwrap(), item__228.clone(), temper_core::cast::<RegexNode>(end().clone()).unwrap()]));
}
pub fn one_or_more(item__230: RegexNode, reluctant__558: Option<bool>) -> Repeat {
    let reluctant__231: bool;
    if reluctant__558.is_none() {
        reluctant__231 = false;
    } else {
        reluctant__231 = reluctant__558.unwrap();
    }
    return Repeat::new(item__230.clone(), 1, None, Some(reluctant__231));
}
pub fn optional(item__233: RegexNode, reluctant__560: Option<bool>) -> Repeat {
    let reluctant__234: bool;
    if reluctant__560.is_none() {
        reluctant__234 = false;
    } else {
        reluctant__234 = reluctant__560.unwrap();
    }
    return Repeat::new(item__233.clone(), 0, Some(1), Some(reluctant__234));
}
