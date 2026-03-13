#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            NULL_INTERCHANGE_CONTEXT__INSTANCE.set(NullInterchangeContext::new()).unwrap_or_else(| _ | panic!());
            let JSON_STATE_OPEN_OBJECT__366: i32 = 0;
            let JSON_STATE_AFTER_KEY__367: i32 = 1;
            let JSON_STATE_AFTER_PROPERTY__368: i32 = 2;
            let JSON_STATE_OPEN_ARRAY__369: i32 = 3;
            let JSON_STATE_AFTER_ELEMENT__370: i32 = 4;
            let JSON_STATE_NO_VALUE__371: i32 = 5;
            let JSON_STATE_ONE_VALUE__372: i32 = 6;
            HEX_DIGITS.set(std::sync::Arc::new(vec![std::sync::Arc::new("0".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("3".to_string()), std::sync::Arc::new("4".to_string()), std::sync::Arc::new("5".to_string()), std::sync::Arc::new("6".to_string()), std::sync::Arc::new("7".to_string()), std::sync::Arc::new("8".to_string()), std::sync::Arc::new("9".to_string()), std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c".to_string()), std::sync::Arc::new("d".to_string()), std::sync::Arc::new("e".to_string()), std::sync::Arc::new("f".to_string())])).unwrap_or_else(| _ | panic!());
            let minInt64__374: i64 = -9223372036854775808;
            Ok(())
    }).clone()
}
static HEX_DIGITS: std::sync::OnceLock<temper_core::List<std::sync::Arc<String>>> = std::sync::OnceLock::new();
fn hex_digits() -> temper_core::List<std::sync::Arc<String>> {
    ( * HEX_DIGITS.get().unwrap()).clone()
}
pub trait InterchangeContextTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> InterchangeContext;
    fn get_header(& self, headerName__376: std::sync::Arc<String>) -> Option<std::sync::Arc<String>>;
}
#[derive(Clone)]
pub struct InterchangeContext(std::sync::Arc<dyn InterchangeContextTrait>);
impl InterchangeContext {
    pub fn new(selfish: impl InterchangeContextTrait + 'static) -> InterchangeContext {
        InterchangeContext(std::sync::Arc::new(selfish))
    }
}
impl InterchangeContextTrait for InterchangeContext {
    fn clone_boxed(& self) -> InterchangeContext {
        InterchangeContextTrait::clone_boxed( & ( * self.0))
    }
    fn get_header(& self, arg1: std::sync::Arc<String>) -> Option<std::sync::Arc<String>> {
        InterchangeContextTrait::get_header( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(InterchangeContext);
impl std::ops::Deref for InterchangeContext {
    type Target = dyn InterchangeContextTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct NullInterchangeContextStruct {}
#[derive(Clone)]
pub struct NullInterchangeContext(std::sync::Arc<NullInterchangeContextStruct>);
static NULL_INTERCHANGE_CONTEXT__INSTANCE: std::sync::OnceLock<NullInterchangeContext> = std::sync::OnceLock::new();
impl NullInterchangeContext {
    pub fn get_header(& self, headerName__379: impl temper_core::ToArcString) -> Option<std::sync::Arc<String>> {
        let headerName__379 = headerName__379.to_arc_string();
        return None;
    }
    pub fn instance() -> NullInterchangeContext {
        ( * NULL_INTERCHANGE_CONTEXT__INSTANCE.get().unwrap()).clone()
    }
    pub fn new() -> NullInterchangeContext {
        let selfish = NullInterchangeContext(std::sync::Arc::new(NullInterchangeContextStruct {}));
        return selfish;
    }
}
impl InterchangeContextTrait for NullInterchangeContext {
    fn clone_boxed(& self) -> InterchangeContext {
        InterchangeContext::new(self.clone())
    }
    fn get_header(& self, headerName__379: std::sync::Arc<String>) -> Option<std::sync::Arc<String>> {
        self.get_header(headerName__379)
    }
}
temper_core::impl_any_value_trait!(NullInterchangeContext, [InterchangeContext]);
pub trait JsonProducerTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> JsonProducer;
    fn interchange_context(& self) -> InterchangeContext;
    fn start_object(& self);
    fn end_object(& self);
    fn object_key(& self, key__389: std::sync::Arc<String>);
    fn start_array(& self);
    fn end_array(& self);
    fn null_value(& self);
    fn boolean_value(& self, x__398: bool);
    fn int32_value(& self, x__401: i32);
    fn int64_value(& self, x__404: i64);
    fn float64_value(& self, x__407: f64);
    fn numeric_token_value(& self, x__410: std::sync::Arc<String>);
    fn string_value(& self, x__413: std::sync::Arc<String>);
    fn parse_error_receiver(& self) -> Option<JsonParseErrorReceiver> {
        return None;
    }
}
#[derive(Clone)]
pub struct JsonProducer(std::sync::Arc<dyn JsonProducerTrait>);
impl JsonProducer {
    pub fn new(selfish: impl JsonProducerTrait + 'static) -> JsonProducer {
        JsonProducer(std::sync::Arc::new(selfish))
    }
}
impl JsonProducerTrait for JsonProducer {
    fn clone_boxed(& self) -> JsonProducer {
        JsonProducerTrait::clone_boxed( & ( * self.0))
    }
    fn parse_error_receiver(& self) -> Option<JsonParseErrorReceiver> {
        JsonProducerTrait::parse_error_receiver( & ( * self.0))
    }
    fn start_object(& self) -> () {
        JsonProducerTrait::start_object( & ( * self.0))
    }
    fn end_object(& self) -> () {
        JsonProducerTrait::end_object( & ( * self.0))
    }
    fn object_key(& self, arg1: std::sync::Arc<String>) -> () {
        JsonProducerTrait::object_key( & ( * self.0), arg1)
    }
    fn start_array(& self) -> () {
        JsonProducerTrait::start_array( & ( * self.0))
    }
    fn end_array(& self) -> () {
        JsonProducerTrait::end_array( & ( * self.0))
    }
    fn null_value(& self) -> () {
        JsonProducerTrait::null_value( & ( * self.0))
    }
    fn boolean_value(& self, arg1: bool) -> () {
        JsonProducerTrait::boolean_value( & ( * self.0), arg1)
    }
    fn int32_value(& self, arg1: i32) -> () {
        JsonProducerTrait::int32_value( & ( * self.0), arg1)
    }
    fn int64_value(& self, arg1: i64) -> () {
        JsonProducerTrait::int64_value( & ( * self.0), arg1)
    }
    fn float64_value(& self, arg1: f64) -> () {
        JsonProducerTrait::float64_value( & ( * self.0), arg1)
    }
    fn numeric_token_value(& self, arg1: std::sync::Arc<String>) -> () {
        JsonProducerTrait::numeric_token_value( & ( * self.0), arg1)
    }
    fn string_value(& self, arg1: std::sync::Arc<String>) -> () {
        JsonProducerTrait::string_value( & ( * self.0), arg1)
    }
    fn interchange_context(& self) -> InterchangeContext {
        JsonProducerTrait::interchange_context( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(JsonProducer);
impl std::ops::Deref for JsonProducer {
    type Target = dyn JsonProducerTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
pub enum JsonSyntaxTreeEnum {
    JsonObject(JsonObject), JsonArray(JsonArray), JsonBoolean(JsonBoolean), JsonNull(JsonNull), JsonString(JsonString), JsonNumeric(JsonNumeric)
}
pub trait JsonSyntaxTreeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> JsonSyntaxTreeEnum;
    fn clone_boxed(& self) -> JsonSyntaxTree;
    fn produce(& self, p__418: JsonProducer);
}
#[derive(Clone)]
pub struct JsonSyntaxTree(std::sync::Arc<dyn JsonSyntaxTreeTrait>);
impl JsonSyntaxTree {
    pub fn new(selfish: impl JsonSyntaxTreeTrait + 'static) -> JsonSyntaxTree {
        JsonSyntaxTree(std::sync::Arc::new(selfish))
    }
}
impl JsonSyntaxTreeTrait for JsonSyntaxTree {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTreeTrait::clone_boxed( & ( * self.0))
    }
    fn produce(& self, arg1: JsonProducer) -> () {
        JsonSyntaxTreeTrait::produce( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(JsonSyntaxTree);
impl std::ops::Deref for JsonSyntaxTree {
    type Target = dyn JsonSyntaxTreeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct JsonObjectStruct {
    properties: temper_core::Map<std::sync::Arc<String>, temper_core::List<JsonSyntaxTree>>
}
#[derive(Clone)]
pub struct JsonObject(std::sync::Arc<JsonObjectStruct>);
impl JsonObject {
    pub fn property_value_or_null(& self, propertyKey__422: impl temper_core::ToArcString) -> Option<JsonSyntaxTree> {
        let propertyKey__422 = propertyKey__422.to_arc_string();
        let return__209: Option<JsonSyntaxTree>;
        let treeList__424: temper_core::List<JsonSyntaxTree> = temper_core::MappedTrait::get_or( & self.0.properties, propertyKey__422.clone(), std::sync::Arc::new(vec![]));
        let lastIndex__425: i32 = temper_core::ListedTrait::len( & treeList__424).wrapping_sub(1);
        if Some(lastIndex__425) >= Some(0) {
            return__209 = Some(temper_core::ListedTrait::get( & treeList__424, lastIndex__425));
        } else {
            return__209 = None;
        }
        return return__209.clone();
    }
    pub fn property_value_or_bubble(& self, propertyKey__427: impl temper_core::ToArcString) -> temper_core::Result<JsonSyntaxTree> {
        let propertyKey__427 = propertyKey__427.to_arc_string();
        let return__210: JsonSyntaxTree;
        let mut t___2518: Option<JsonSyntaxTree> = self.property_value_or_null(propertyKey__427.clone());
        if t___2518.is_none() {
            return Err(temper_core::Error::new());
        } else {
            return__210 = t___2518.clone().unwrap();
        }
        return Ok(return__210.clone());
    }
    pub fn produce(& self, p__430: JsonProducer) {
        p__430.start_object();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            p__430: JsonProducer
        }
        impl ClosureGroup___1 {
            fn fn__2513(& self, k__432: impl temper_core::ToArcString, vs__433: impl temper_core::ToList<JsonSyntaxTree>) {
                let k__432 = k__432.to_arc_string();
                let vs__433 = vs__433.to_list();
                #[derive(Clone)]
                struct ClosureGroup___2 {
                    p__430: JsonProducer, k__432: std::sync::Arc<String>
                }
                impl ClosureGroup___2 {
                    fn fn__2510(& self, v__434: JsonSyntaxTree) {
                        self.p__430.object_key(self.k__432.clone());
                        v__434.produce(self.p__430.clone());
                    }
                }
                let closure_group = ClosureGroup___2 {
                    p__430: self.p__430.clone(), k__432: k__432.clone()
                };
                let fn__2510 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | v__434: JsonSyntaxTree | closure_group.fn__2510(v__434))
                };
                temper_core::listed::list_for_each( & vs__433, & ( * fn__2510.clone()));
            }
        }
        let closure_group = ClosureGroup___1 {
            p__430: p__430.clone()
        };
        let fn__2513 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | k__432: std::sync::Arc<String>, vs__433: temper_core::List<JsonSyntaxTree> | closure_group.fn__2513(k__432, vs__433))
        };
        temper_core::MappedTrait::for_each( & self.0.properties, & ( * fn__2513.clone()));
        p__430.end_object();
    }
    pub fn new(properties__436: temper_core::Map<std::sync::Arc<String>, temper_core::List<JsonSyntaxTree>>) -> JsonObject {
        let properties;
        properties = properties__436.clone();
        let selfish = JsonObject(std::sync::Arc::new(JsonObjectStruct {
                    properties
        }));
        return selfish;
    }
    pub fn properties(& self) -> temper_core::Map<std::sync::Arc<String>, temper_core::List<JsonSyntaxTree>> {
        return self.0.properties.clone();
    }
}
impl JsonSyntaxTreeTrait for JsonObject {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonObject(self.clone())
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__430: JsonProducer) {
        self.produce(p__430)
    }
}
temper_core::impl_any_value_trait!(JsonObject, [JsonSyntaxTree]);
struct JsonArrayStruct {
    elements: temper_core::List<JsonSyntaxTree>
}
#[derive(Clone)]
pub struct JsonArray(std::sync::Arc<JsonArrayStruct>);
impl JsonArray {
    pub fn produce(& self, p__439: JsonProducer) {
        p__439.start_array();
        #[derive(Clone)]
        struct ClosureGroup___3 {
            p__439: JsonProducer
        }
        impl ClosureGroup___3 {
            fn fn__2503(& self, v__441: JsonSyntaxTree) {
                v__441.produce(self.p__439.clone());
            }
        }
        let closure_group = ClosureGroup___3 {
            p__439: p__439.clone()
        };
        let fn__2503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__441: JsonSyntaxTree | closure_group.fn__2503(v__441))
        };
        temper_core::listed::list_for_each( & self.0.elements, & ( * fn__2503.clone()));
        p__439.end_array();
    }
    pub fn new(elements__443: impl temper_core::ToList<JsonSyntaxTree>) -> JsonArray {
        let elements__443 = elements__443.to_list();
        let elements;
        elements = elements__443.clone();
        let selfish = JsonArray(std::sync::Arc::new(JsonArrayStruct {
                    elements
        }));
        return selfish;
    }
    pub fn elements(& self) -> temper_core::List<JsonSyntaxTree> {
        return self.0.elements.clone();
    }
}
impl JsonSyntaxTreeTrait for JsonArray {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonArray(self.clone())
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__439: JsonProducer) {
        self.produce(p__439)
    }
}
temper_core::impl_any_value_trait!(JsonArray, [JsonSyntaxTree]);
struct JsonBooleanStruct {
    content: bool
}
#[derive(Clone)]
pub struct JsonBoolean(std::sync::Arc<JsonBooleanStruct>);
impl JsonBoolean {
    pub fn produce(& self, p__446: JsonProducer) {
        p__446.boolean_value(self.0.content);
    }
    pub fn new(content__449: bool) -> JsonBoolean {
        let content;
        content = content__449;
        let selfish = JsonBoolean(std::sync::Arc::new(JsonBooleanStruct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> bool {
        return self.0.content;
    }
}
impl JsonSyntaxTreeTrait for JsonBoolean {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonBoolean(self.clone())
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__446: JsonProducer) {
        self.produce(p__446)
    }
}
temper_core::impl_any_value_trait!(JsonBoolean, [JsonSyntaxTree]);
struct JsonNullStruct {}
#[derive(Clone)]
pub struct JsonNull(std::sync::Arc<JsonNullStruct>);
impl JsonNull {
    pub fn produce(& self, p__451: JsonProducer) {
        p__451.null_value();
    }
    pub fn new() -> JsonNull {
        let selfish = JsonNull(std::sync::Arc::new(JsonNullStruct {}));
        return selfish;
    }
}
impl JsonSyntaxTreeTrait for JsonNull {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonNull(self.clone())
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__451: JsonProducer) {
        self.produce(p__451)
    }
}
temper_core::impl_any_value_trait!(JsonNull, [JsonSyntaxTree]);
struct JsonStringStruct {
    content: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct JsonString(std::sync::Arc<JsonStringStruct>);
impl JsonString {
    pub fn produce(& self, p__456: JsonProducer) {
        p__456.string_value(self.0.content.clone());
    }
    pub fn new(content__459: impl temper_core::ToArcString) -> JsonString {
        let content__459 = content__459.to_arc_string();
        let content;
        content = content__459.clone();
        let selfish = JsonString(std::sync::Arc::new(JsonStringStruct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> std::sync::Arc<String> {
        return self.0.content.clone();
    }
}
impl JsonSyntaxTreeTrait for JsonString {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonString(self.clone())
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__456: JsonProducer) {
        self.produce(p__456)
    }
}
temper_core::impl_any_value_trait!(JsonString, [JsonSyntaxTree]);
pub enum JsonNumericEnum {
    JsonInt32(JsonInt32), JsonInt64(JsonInt64), JsonFloat64(JsonFloat64), JsonNumericToken(JsonNumericToken)
}
pub trait JsonNumericTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync + JsonSyntaxTreeTrait {
    fn as_enum(& self) -> JsonNumericEnum;
    fn clone_boxed(& self) -> JsonNumeric;
    fn as_json_numeric_token(& self) -> std::sync::Arc<String>;
    fn as_int32(& self) -> temper_core::Result<i32>;
    fn as_int64(& self) -> temper_core::Result<i64>;
    fn as_float64(& self) -> temper_core::Result<f64>;
}
#[derive(Clone)]
pub struct JsonNumeric(std::sync::Arc<dyn JsonNumericTrait>);
impl JsonNumeric {
    pub fn new(selfish: impl JsonNumericTrait + 'static) -> JsonNumeric {
        JsonNumeric(std::sync::Arc::new(selfish))
    }
}
impl JsonNumericTrait for JsonNumeric {
    fn as_enum(& self) -> JsonNumericEnum {
        JsonNumericTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JsonNumeric {
        JsonNumericTrait::clone_boxed( & ( * self.0))
    }
    fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        JsonNumericTrait::as_json_numeric_token( & ( * self.0))
    }
    fn as_int32(& self) -> temper_core::Result<i32> {
        JsonNumericTrait::as_int32( & ( * self.0))
    }
    fn as_int64(& self) -> temper_core::Result<i64> {
        JsonNumericTrait::as_int64( & ( * self.0))
    }
    fn as_float64(& self) -> temper_core::Result<f64> {
        JsonNumericTrait::as_float64( & ( * self.0))
    }
}
impl JsonSyntaxTreeTrait for JsonNumeric {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTreeTrait::clone_boxed( & ( * self.0))
    }
    fn produce(& self, arg1: JsonProducer) -> () {
        JsonSyntaxTreeTrait::produce( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(JsonNumeric);
impl std::ops::Deref for JsonNumeric {
    type Target = dyn JsonNumericTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct JsonInt32Struct {
    content: i32
}
#[derive(Clone)]
pub struct JsonInt32(std::sync::Arc<JsonInt32Struct>);
impl JsonInt32 {
    pub fn produce(& self, p__470: JsonProducer) {
        p__470.int32_value(self.0.content);
    }
    pub fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        return temper_core::int_to_string(self.0.content, None);
    }
    pub fn as_int32(& self) -> i32 {
        return self.0.content;
    }
    pub fn as_int64(& self) -> i64 {
        return self.0.content as i64;
    }
    pub fn as_float64(& self) -> f64 {
        return self.0.content as f64;
    }
    pub fn new(content__481: i32) -> JsonInt32 {
        let content;
        content = content__481;
        let selfish = JsonInt32(std::sync::Arc::new(JsonInt32Struct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> i32 {
        return self.0.content;
    }
}
impl JsonNumericTrait for JsonInt32 {
    fn as_enum(& self) -> JsonNumericEnum {
        JsonNumericEnum::JsonInt32(self.clone())
    }
    fn clone_boxed(& self) -> JsonNumeric {
        JsonNumeric::new(self.clone())
    }
    fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        self.as_json_numeric_token()
    }
    fn as_int32(& self) -> temper_core::Result<i32> {
        Ok(self.as_int32())
    }
    fn as_int64(& self) -> temper_core::Result<i64> {
        Ok(self.as_int64())
    }
    fn as_float64(& self) -> temper_core::Result<f64> {
        Ok(self.as_float64())
    }
}
impl JsonSyntaxTreeTrait for JsonInt32 {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonNumeric(JsonNumeric::new(self.clone()))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__470: JsonProducer) {
        self.produce(p__470)
    }
}
temper_core::impl_any_value_trait!(JsonInt32, [JsonNumeric, JsonSyntaxTree]);
struct JsonInt64Struct {
    content: i64
}
#[derive(Clone)]
pub struct JsonInt64(std::sync::Arc<JsonInt64Struct>);
impl JsonInt64 {
    pub fn produce(& self, p__484: JsonProducer) {
        p__484.int64_value(self.0.content);
    }
    pub fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        return temper_core::int64_to_string(self.0.content, None);
    }
    pub fn as_int32(& self) -> temper_core::Result<i32> {
        let return__240: i32;
        return__240 = temper_core::int64_to_int32(self.0.content) ? ;
        return Ok(return__240);
    }
    pub fn as_int64(& self) -> i64 {
        return self.0.content;
    }
    pub fn as_float64(& self) -> temper_core::Result<f64> {
        let return__242: f64;
        return__242 = temper_core::int64_to_float64(self.0.content) ? ;
        return Ok(return__242);
    }
    pub fn new(content__495: i64) -> JsonInt64 {
        let content;
        content = content__495;
        let selfish = JsonInt64(std::sync::Arc::new(JsonInt64Struct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> i64 {
        return self.0.content;
    }
}
impl JsonNumericTrait for JsonInt64 {
    fn as_enum(& self) -> JsonNumericEnum {
        JsonNumericEnum::JsonInt64(self.clone())
    }
    fn clone_boxed(& self) -> JsonNumeric {
        JsonNumeric::new(self.clone())
    }
    fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        self.as_json_numeric_token()
    }
    fn as_int32(& self) -> temper_core::Result<i32> {
        self.as_int32()
    }
    fn as_int64(& self) -> temper_core::Result<i64> {
        Ok(self.as_int64())
    }
    fn as_float64(& self) -> temper_core::Result<f64> {
        self.as_float64()
    }
}
impl JsonSyntaxTreeTrait for JsonInt64 {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonNumeric(JsonNumeric::new(self.clone()))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__484: JsonProducer) {
        self.produce(p__484)
    }
}
temper_core::impl_any_value_trait!(JsonInt64, [JsonNumeric, JsonSyntaxTree]);
struct JsonFloat64Struct {
    content: f64
}
#[derive(Clone)]
pub struct JsonFloat64(std::sync::Arc<JsonFloat64Struct>);
impl JsonFloat64 {
    pub fn produce(& self, p__498: JsonProducer) {
        p__498.float64_value(self.0.content);
    }
    pub fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        return temper_core::float64::to_string(self.0.content);
    }
    pub fn as_int32(& self) -> temper_core::Result<i32> {
        let return__247: i32;
        return__247 = temper_core::float64::to_int(self.0.content) ? ;
        return Ok(return__247);
    }
    pub fn as_int64(& self) -> temper_core::Result<i64> {
        let return__248: i64;
        return__248 = temper_core::float64::to_int64(self.0.content) ? ;
        return Ok(return__248);
    }
    pub fn as_float64(& self) -> f64 {
        return self.0.content;
    }
    pub fn new(content__509: f64) -> JsonFloat64 {
        let content;
        content = content__509;
        let selfish = JsonFloat64(std::sync::Arc::new(JsonFloat64Struct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> f64 {
        return self.0.content;
    }
}
impl JsonNumericTrait for JsonFloat64 {
    fn as_enum(& self) -> JsonNumericEnum {
        JsonNumericEnum::JsonFloat64(self.clone())
    }
    fn clone_boxed(& self) -> JsonNumeric {
        JsonNumeric::new(self.clone())
    }
    fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        self.as_json_numeric_token()
    }
    fn as_int32(& self) -> temper_core::Result<i32> {
        self.as_int32()
    }
    fn as_int64(& self) -> temper_core::Result<i64> {
        self.as_int64()
    }
    fn as_float64(& self) -> temper_core::Result<f64> {
        Ok(self.as_float64())
    }
}
impl JsonSyntaxTreeTrait for JsonFloat64 {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonNumeric(JsonNumeric::new(self.clone()))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__498: JsonProducer) {
        self.produce(p__498)
    }
}
temper_core::impl_any_value_trait!(JsonFloat64, [JsonNumeric, JsonSyntaxTree]);
struct JsonNumericTokenStruct {
    content: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct JsonNumericToken(std::sync::Arc<JsonNumericTokenStruct>);
impl JsonNumericToken {
    pub fn produce(& self, p__512: JsonProducer) {
        p__512.numeric_token_value(self.0.content.clone());
    }
    pub fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        return self.0.content.clone();
    }
    pub fn as_int32(& self) -> temper_core::Result<i32> {
        let return__254: i32;
        let mut t___1774: i32;
        let mut t___1775: f64;
        'ok___2540: {
            'orelse___940: {
                t___1774 = match temper_core::string::to_int( & self.0.content, None) {
                    Ok(x) => x,
                    _ => break 'orelse___940
                };
                return__254 = t___1774;
                break 'ok___2540;
            }
            t___1775 = temper_core::string::to_float64( & self.0.content) ? ;
            return__254 = temper_core::float64::to_int(t___1775) ? ;
        }
        return Ok(return__254);
    }
    pub fn as_int64(& self) -> temper_core::Result<i64> {
        let return__255: i64;
        let mut t___1770: i64;
        let mut t___1771: f64;
        'ok___2543: {
            'orelse___941: {
                t___1770 = match temper_core::string::to_int64( & self.0.content, None) {
                    Ok(x) => x,
                    _ => break 'orelse___941
                };
                return__255 = t___1770;
                break 'ok___2543;
            }
            t___1771 = temper_core::string::to_float64( & self.0.content) ? ;
            return__255 = temper_core::float64::to_int64(t___1771) ? ;
        }
        return Ok(return__255);
    }
    pub fn as_float64(& self) -> temper_core::Result<f64> {
        let return__256: f64;
        return__256 = temper_core::string::to_float64( & self.0.content) ? ;
        return Ok(return__256);
    }
    pub fn new(content__523: impl temper_core::ToArcString) -> JsonNumericToken {
        let content__523 = content__523.to_arc_string();
        let content;
        content = content__523.clone();
        let selfish = JsonNumericToken(std::sync::Arc::new(JsonNumericTokenStruct {
                    content
        }));
        return selfish;
    }
    pub fn content(& self) -> std::sync::Arc<String> {
        return self.0.content.clone();
    }
}
impl JsonNumericTrait for JsonNumericToken {
    fn as_enum(& self) -> JsonNumericEnum {
        JsonNumericEnum::JsonNumericToken(self.clone())
    }
    fn clone_boxed(& self) -> JsonNumeric {
        JsonNumeric::new(self.clone())
    }
    fn as_json_numeric_token(& self) -> std::sync::Arc<String> {
        self.as_json_numeric_token()
    }
    fn as_int32(& self) -> temper_core::Result<i32> {
        self.as_int32()
    }
    fn as_int64(& self) -> temper_core::Result<i64> {
        self.as_int64()
    }
    fn as_float64(& self) -> temper_core::Result<f64> {
        self.as_float64()
    }
}
impl JsonSyntaxTreeTrait for JsonNumericToken {
    fn as_enum(& self) -> JsonSyntaxTreeEnum {
        JsonSyntaxTreeEnum::JsonNumeric(JsonNumeric::new(self.clone()))
    }
    fn clone_boxed(& self) -> JsonSyntaxTree {
        JsonSyntaxTree::new(self.clone())
    }
    fn produce(& self, p__512: JsonProducer) {
        self.produce(p__512)
    }
}
temper_core::impl_any_value_trait!(JsonNumericToken, [JsonNumeric, JsonSyntaxTree]);
struct JsonTextProducerStruct {
    interchange_context: InterchangeContext, buffer: std::sync::Arc<std::sync::RwLock<String>>, stack: temper_core::ListBuilder<i32>, well_formed: bool
}
#[derive(Clone)]
pub struct JsonTextProducer(std::sync::Arc<std::sync::RwLock<JsonTextProducerStruct>>);
impl JsonTextProducer {
    pub fn new(interchangeContext__938: Option<InterchangeContext>) -> JsonTextProducer {
        let interchange_context;
        let buffer;
        let stack;
        let well_formed;
        let interchangeContext__529: InterchangeContext;
        if interchangeContext__938.is_none() {
            interchangeContext__529 = InterchangeContext::new(NullInterchangeContext::instance());
        } else {
            interchangeContext__529 = interchangeContext__938.clone().unwrap();
        }
        interchange_context = interchangeContext__529.clone();
        let mut t___2470: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        buffer = t___2470.clone();
        let mut t___2471: temper_core::ListBuilder<i32> = temper_core::listed::new_builder();
        stack = t___2471.clone();
        temper_core::listed::add( & stack, 5, None);
        well_formed = true;
        let selfish = JsonTextProducer(std::sync::Arc::new(std::sync::RwLock::new(JsonTextProducerStruct {
                        interchange_context, buffer, stack, well_formed
        })));
        return selfish;
    }
    fn state(& self) -> i32 {
        let mut t___2468: i32 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
        return temper_core::ListedTrait::get_or( & self.0.read().unwrap().stack, t___2468.wrapping_sub(1), -1);
    }
    fn before_value(& self) {
        let mut t___2461: i32;
        let mut t___2464: i32;
        let mut t___2466: i32;
        let mut t___1728: bool;
        let currentState__535: i32 = self.state();
        if Some(currentState__535) == Some(3) {
            t___2461 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
            temper_core::listed::set( & self.0.read().unwrap().stack, t___2461.wrapping_sub(1), 4);
        } else {
            if Some(currentState__535) == Some(4) {
                temper_core::string::builder::append( & self.0.read().unwrap().buffer, ",");
            } else {
                if Some(currentState__535) == Some(1) {
                    t___2464 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
                    temper_core::listed::set( & self.0.read().unwrap().stack, t___2464.wrapping_sub(1), 2);
                } else {
                    if Some(currentState__535) == Some(5) {
                        t___2466 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
                        temper_core::listed::set( & self.0.read().unwrap().stack, t___2466.wrapping_sub(1), 6);
                    } else {
                        if Some(currentState__535) == Some(6) {
                            t___1728 = true;
                        } else {
                            t___1728 = Some(currentState__535) == Some(2);
                        }
                        if t___1728 {
                            self.0.write().unwrap().well_formed = false;
                        }
                    }
                }
            }
        }
    }
    pub fn start_object(& self) {
        self.before_value();
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, "{");
        temper_core::listed::add( & self.0.read().unwrap().stack, 0, None);
    }
    pub fn end_object(& self) {
        let mut t___1716: bool;
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, "}");
        let currentState__540: i32 = self.state();
        if Some(0) == Some(currentState__540) {
            t___1716 = true;
        } else {
            t___1716 = Some(2) == Some(currentState__540);
        }
        if t___1716 {
            temper_core::listed::remove_last( & self.0.read().unwrap().stack);
        } else {
            self.0.write().unwrap().well_formed = false;
        }
    }
    pub fn object_key(& self, key__542: impl temper_core::ToArcString) {
        let key__542 = key__542.to_arc_string();
        let mut t___2452: i32;
        let currentState__544: i32 = self.state();
        if ! (Some(currentState__544) == Some(0)) {
            if Some(currentState__544) == Some(2) {
                temper_core::string::builder::append( & self.0.read().unwrap().buffer, ",");
            } else {
                self.0.write().unwrap().well_formed = false;
            }
        }
        encodeJsonString__351(key__542.clone(), self.0.read().unwrap().buffer.clone());
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, ":");
        if Some(currentState__544) >= Some(0) {
            t___2452 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
            temper_core::listed::set( & self.0.read().unwrap().stack, t___2452.wrapping_sub(1), 1);
        }
    }
    pub fn start_array(& self) {
        self.before_value();
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, "[");
        temper_core::listed::add( & self.0.read().unwrap().stack, 3, None);
    }
    pub fn end_array(& self) {
        let mut t___1704: bool;
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, "]");
        let currentState__549: i32 = self.state();
        if Some(3) == Some(currentState__549) {
            t___1704 = true;
        } else {
            t___1704 = Some(4) == Some(currentState__549);
        }
        if t___1704 {
            temper_core::listed::remove_last( & self.0.read().unwrap().stack);
        } else {
            self.0.write().unwrap().well_formed = false;
        }
    }
    pub fn null_value(& self) {
        self.before_value();
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, "null");
    }
    pub fn boolean_value(& self, x__553: bool) {
        let mut t___1700: std::sync::Arc<String>;
        self.before_value();
        if x__553 {
            t___1700 = std::sync::Arc::new("true".to_string());
        } else {
            t___1700 = std::sync::Arc::new("false".to_string());
        }
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, t___1700.clone());
    }
    pub fn int32_value(& self, x__556: i32) {
        self.before_value();
        let mut t___2436: std::sync::Arc<String> = temper_core::int_to_string(x__556, None);
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, t___2436.clone());
    }
    pub fn int64_value(& self, x__559: i64) {
        self.before_value();
        let mut t___2433: std::sync::Arc<String> = temper_core::int64_to_string(x__559, None);
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, t___2433.clone());
    }
    pub fn float64_value(& self, x__562: f64) {
        self.before_value();
        let mut t___2430: std::sync::Arc<String> = temper_core::float64::to_string(x__562);
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, t___2430.clone());
    }
    pub fn numeric_token_value(& self, x__565: impl temper_core::ToArcString) {
        let x__565 = x__565.to_arc_string();
        self.before_value();
        temper_core::string::builder::append( & self.0.read().unwrap().buffer, x__565.clone());
    }
    pub fn string_value(& self, x__568: impl temper_core::ToArcString) {
        let x__568 = x__568.to_arc_string();
        self.before_value();
        encodeJsonString__351(x__568.clone(), self.0.read().unwrap().buffer.clone());
    }
    pub fn to_json_string(& self) -> temper_core::Result<std::sync::Arc<String>> {
        let return__272: std::sync::Arc<String>;
        let mut t___2423: i32;
        let mut t___1689: bool;
        let mut t___1690: bool;
        if self.0.read().unwrap().well_formed {
            if Some(temper_core::ListedTrait::len( & self.0.read().unwrap().stack)) == Some(1) {
                t___2423 = self.state();
                t___1689 = Some(t___2423) == Some(6);
            } else {
                t___1689 = false;
            }
            t___1690 = t___1689;
        } else {
            t___1690 = false;
        }
        if t___1690 {
            return__272 = temper_core::string::builder::to_string( & self.0.read().unwrap().buffer);
        } else {
            return Err(temper_core::Error::new());
        }
        return Ok(return__272.clone());
    }
    pub fn interchange_context(& self) -> InterchangeContext {
        return self.0.read().unwrap().interchange_context.clone();
    }
}
impl JsonProducerTrait for JsonTextProducer {
    fn clone_boxed(& self) -> JsonProducer {
        JsonProducer::new(self.clone())
    }
    fn start_object(& self) {
        self.start_object()
    }
    fn end_object(& self) {
        self.end_object()
    }
    fn object_key(& self, key__542: std::sync::Arc<String>) {
        self.object_key(key__542)
    }
    fn start_array(& self) {
        self.start_array()
    }
    fn end_array(& self) {
        self.end_array()
    }
    fn null_value(& self) {
        self.null_value()
    }
    fn boolean_value(& self, x__553: bool) {
        self.boolean_value(x__553)
    }
    fn int32_value(& self, x__556: i32) {
        self.int32_value(x__556)
    }
    fn int64_value(& self, x__559: i64) {
        self.int64_value(x__559)
    }
    fn float64_value(& self, x__562: f64) {
        self.float64_value(x__562)
    }
    fn numeric_token_value(& self, x__565: std::sync::Arc<String>) {
        self.numeric_token_value(x__565)
    }
    fn string_value(& self, x__568: std::sync::Arc<String>) {
        self.string_value(x__568)
    }
    fn interchange_context(& self) -> InterchangeContext {
        self.interchange_context()
    }
}
temper_core::impl_any_value_trait!(JsonTextProducer, [JsonProducer]);
pub trait JsonParseErrorReceiverTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> JsonParseErrorReceiver;
    fn explain_json_error(& self, explanation__588: std::sync::Arc<String>);
}
#[derive(Clone)]
pub struct JsonParseErrorReceiver(std::sync::Arc<dyn JsonParseErrorReceiverTrait>);
impl JsonParseErrorReceiver {
    pub fn new(selfish: impl JsonParseErrorReceiverTrait + 'static) -> JsonParseErrorReceiver {
        JsonParseErrorReceiver(std::sync::Arc::new(selfish))
    }
}
impl JsonParseErrorReceiverTrait for JsonParseErrorReceiver {
    fn clone_boxed(& self) -> JsonParseErrorReceiver {
        JsonParseErrorReceiverTrait::clone_boxed( & ( * self.0))
    }
    fn explain_json_error(& self, arg1: std::sync::Arc<String>) -> () {
        JsonParseErrorReceiverTrait::explain_json_error( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(JsonParseErrorReceiver);
impl std::ops::Deref for JsonParseErrorReceiver {
    type Target = dyn JsonParseErrorReceiverTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct JsonSyntaxTreeProducerStruct {
    stack: temper_core::ListBuilder<temper_core::ListBuilder<JsonSyntaxTree>>, error: Option<std::sync::Arc<String>>
}
#[derive(Clone)]
pub struct JsonSyntaxTreeProducer(std::sync::Arc<std::sync::RwLock<JsonSyntaxTreeProducerStruct>>);
impl JsonSyntaxTreeProducer {
    pub fn interchange_context(& self) -> InterchangeContext {
        return InterchangeContext::new(NullInterchangeContext::instance());
    }
    pub fn new() -> JsonSyntaxTreeProducer {
        let stack;
        let error;
        let mut t___2419: temper_core::ListBuilder<temper_core::ListBuilder<JsonSyntaxTree>> = temper_core::listed::new_builder();
        stack = t___2419.clone();
        let mut t___2420: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::listed::new_builder();
        temper_core::listed::add( & stack, t___2420.clone(), None);
        error = None;
        let selfish = JsonSyntaxTreeProducer(std::sync::Arc::new(std::sync::RwLock::new(JsonSyntaxTreeProducerStruct {
                        stack, error
        })));
        return selfish;
    }
    fn store_value(& self, v__597: JsonSyntaxTree) {
        let mut t___2416: i32;
        if ! temper_core::ListedTrait::is_empty( & self.0.read().unwrap().stack) {
            t___2416 = temper_core::ListedTrait::len( & self.0.read().unwrap().stack);
            temper_core::listed::add( & temper_core::ListedTrait::get( & self.0.read().unwrap().stack, t___2416.wrapping_sub(1)), v__597.clone(), None);
        }
    }
    pub fn start_object(& self) {
        let mut t___2413: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::listed::new_builder();
        temper_core::listed::add( & self.0.read().unwrap().stack, t___2413.clone(), None);
    }
    pub fn end_object(& self) {
        let return__283: ();
        let mut t___2402: Option<temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>>>;
        let mut t___2411: JsonObject;
        let mut t___1656: JsonString;
        let mut t___1658: JsonString;
        let mut t___1664: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>>;
        let mut t___1666: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>>;
        let mut t___1668: temper_core::List<JsonSyntaxTree>;
        let mut t___1669: temper_core::List<JsonSyntaxTree>;
        let mut t___1671: temper_core::ListBuilder<JsonSyntaxTree>;
        let mut t___1672: temper_core::ListBuilder<JsonSyntaxTree>;
        'fn__602: {
            if temper_core::ListedTrait::is_empty( & self.0.read().unwrap().stack) {
                return__283 = ();
                break 'fn__602;
            }
            let ls__603: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::listed::remove_last( & self.0.read().unwrap().stack);
            let m__604: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::List<JsonSyntaxTree>> = temper_core::MapBuilder::new();
            let mut multis__605: Option<temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>>> = None;
            let mut i__606: i32 = 0;
            let mut n__607: i32 = temper_core::ListedTrait::len( & ls__603) & -2;
            'loop___2645: while Some(i__606) < Some(n__607) {
                let postfixReturn___40: i32 = i__606;
                i__606 = i__606.wrapping_add(1);
                let keyTree__608: JsonSyntaxTree = temper_core::ListedTrait::get( & ls__603, postfixReturn___40);
                if ! temper_core::is::<JsonString>(keyTree__608.clone()) {
                    break;
                }
                'ok___2555: {
                    'orelse___942: {
                        t___1656 = match temper_core::cast::<JsonString>(keyTree__608.clone()).ok_or_else(| | temper_core::Error::new()) {
                            Ok(x) => x,
                            _ => break 'orelse___942
                        };
                        t___1658 = t___1656.clone();
                        break 'ok___2555;
                    }
                    t___1658 = panic!();
                }
                let key__609: std::sync::Arc<String> = t___1658.content();
                let postfixReturn___41: i32 = i__606;
                i__606 = i__606.wrapping_add(1);
                let value__610: JsonSyntaxTree = temper_core::ListedTrait::get( & ls__603, postfixReturn___41);
                if temper_core::MappedTrait::has( & m__604, key__609.clone()) {
                    if multis__605.is_none() {
                        t___2402 = Some(temper_core::MapBuilder::new());
                        multis__605 = t___2402.clone();
                    }
                    'ok___2556: {
                        'orelse___943: {
                            if multis__605.is_none() {
                                break 'orelse___943;
                            } else {
                                t___1664 = multis__605.clone().unwrap();
                            }
                            t___1666 = t___1664.clone();
                            break 'ok___2556;
                        }
                        t___1666 = panic!();
                    }
                    let mb__611: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>> = t___1666.clone();
                    if ! temper_core::MappedTrait::has( & mb__611, key__609.clone()) {
                        'ok___2557: {
                            'orelse___944: {
                                t___1668 = match temper_core::MappedTrait::get( & m__604, key__609.clone()) {
                                    Ok(x) => x,
                                    _ => break 'orelse___944
                                };
                                t___1669 = t___1668.clone();
                                break 'ok___2557;
                            }
                            t___1669 = panic!();
                        }
                        temper_core::MapBuilder::set( & mb__611, key__609.clone(), temper_core::ListedTrait::to_list_builder( & t___1669));
                    }
                    'ok___2558: {
                        'orelse___945: {
                            t___1671 = match temper_core::MappedTrait::get( & mb__611, key__609.clone()) {
                                Ok(x) => x,
                                _ => break 'orelse___945
                            };
                            t___1672 = t___1671.clone();
                            break 'ok___2558;
                        }
                        t___1672 = panic!();
                    }
                    temper_core::listed::add( & t___1672, value__610.clone(), None);
                } else {
                    temper_core::MapBuilder::set( & m__604, key__609.clone(), std::sync::Arc::new(vec![value__610.clone()]));
                }
            }
            let multis__612: Option<temper_core::MapBuilder<std::sync::Arc<String>, temper_core::ListBuilder<JsonSyntaxTree>>> = multis__605.clone();
            if ! multis__612.is_none() {
                #[derive(Clone)]
                struct ClosureGroup___4 {
                    m__604: temper_core::MapBuilder<std::sync::Arc<String>, temper_core::List<JsonSyntaxTree>>
                }
                impl ClosureGroup___4 {
                    fn fn__2392(& self, k__613: impl temper_core::ToArcString, vs__614: impl temper_core::ToListBuilder<JsonSyntaxTree>) {
                        let k__613 = k__613.to_arc_string();
                        let vs__614 = vs__614.to_list_builder();
                        let mut t___2390: temper_core::List<JsonSyntaxTree> = temper_core::ListedTrait::to_list( & vs__614);
                        temper_core::MapBuilder::set( & self.m__604, k__613.clone(), t___2390.clone());
                    }
                }
                let closure_group = ClosureGroup___4 {
                    m__604: m__604.clone()
                };
                let fn__2392 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | k__613: std::sync::Arc<String>, vs__614: temper_core::ListBuilder<JsonSyntaxTree> | closure_group.fn__2392(k__613, vs__614))
                };
                temper_core::MappedTrait::for_each( & multis__612.clone().unwrap(), & ( * fn__2392.clone()));
            }
            t___2411 = JsonObject::new(temper_core::MappedTrait::to_map( & m__604));
            self.store_value(JsonSyntaxTree::new(t___2411.clone()));
            return__283 = ();
        }
        return return__283;
    }
    pub fn object_key(& self, key__616: impl temper_core::ToArcString) {
        let key__616 = key__616.to_arc_string();
        let mut t___2388: JsonString = JsonString::new(key__616.clone());
        self.store_value(JsonSyntaxTree::new(t___2388.clone()));
    }
    pub fn start_array(& self) {
        let mut t___2386: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::listed::new_builder();
        temper_core::listed::add( & self.0.read().unwrap().stack, t___2386.clone(), None);
    }
    pub fn end_array(& self) {
        let return__286: ();
        let mut t___2384: JsonArray;
        'fn__621: {
            if temper_core::ListedTrait::is_empty( & self.0.read().unwrap().stack) {
                return__286 = ();
                break 'fn__621;
            }
            let ls__622: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::listed::remove_last( & self.0.read().unwrap().stack);
            t___2384 = JsonArray::new(temper_core::ListedTrait::to_list( & ls__622));
            self.store_value(JsonSyntaxTree::new(t___2384.clone()));
            return__286 = ();
        }
        return return__286;
    }
    pub fn null_value(& self) {
        let mut t___2379: JsonNull = JsonNull::new();
        self.store_value(JsonSyntaxTree::new(t___2379.clone()));
    }
    pub fn boolean_value(& self, x__626: bool) {
        let mut t___2377: JsonBoolean = JsonBoolean::new(x__626);
        self.store_value(JsonSyntaxTree::new(t___2377.clone()));
    }
    pub fn int32_value(& self, x__629: i32) {
        let mut t___2375: JsonInt32 = JsonInt32::new(x__629);
        self.store_value(JsonSyntaxTree::new(t___2375.clone()));
    }
    pub fn int64_value(& self, x__632: i64) {
        let mut t___2373: JsonInt64 = JsonInt64::new(x__632);
        self.store_value(JsonSyntaxTree::new(t___2373.clone()));
    }
    pub fn float64_value(& self, x__635: f64) {
        let mut t___2371: JsonFloat64 = JsonFloat64::new(x__635);
        self.store_value(JsonSyntaxTree::new(t___2371.clone()));
    }
    pub fn numeric_token_value(& self, x__638: impl temper_core::ToArcString) {
        let x__638 = x__638.to_arc_string();
        let mut t___2369: JsonNumericToken = JsonNumericToken::new(x__638.clone());
        self.store_value(JsonSyntaxTree::new(t___2369.clone()));
    }
    pub fn string_value(& self, x__641: impl temper_core::ToArcString) {
        let x__641 = x__641.to_arc_string();
        let mut t___2367: JsonString = JsonString::new(x__641.clone());
        self.store_value(JsonSyntaxTree::new(t___2367.clone()));
    }
    pub fn to_json_syntax_tree(& self) -> temper_core::Result<JsonSyntaxTree> {
        let mut t___1629: bool;
        if Some(temper_core::ListedTrait::len( & self.0.read().unwrap().stack)) != Some(1) {
            t___1629 = true;
        } else {
            t___1629 = ! self.0.read().unwrap().error.is_none();
        }
        if t___1629 {
            return Err(temper_core::Error::new());
        }
        let ls__645: temper_core::ListBuilder<JsonSyntaxTree> = temper_core::ListedTrait::get( & self.0.read().unwrap().stack, 0);
        if Some(temper_core::ListedTrait::len( & ls__645)) != Some(1) {
            return Err(temper_core::Error::new());
        }
        return Ok(temper_core::ListedTrait::get( & ls__645, 0));
    }
    pub fn json_error(& self) -> Option<std::sync::Arc<String>> {
        return self.0.read().unwrap().error.clone();
    }
    pub fn parse_error_receiver(& self) -> JsonParseErrorReceiver {
        return JsonParseErrorReceiver::new(self.clone());
    }
    pub fn explain_json_error(& self, error__651: impl temper_core::ToArcString) {
        let error__651 = error__651.to_arc_string();
        self.0.write().unwrap().error = Some(error__651.clone());
    }
}
impl JsonProducerTrait for JsonSyntaxTreeProducer {
    fn clone_boxed(& self) -> JsonProducer {
        JsonProducer::new(self.clone())
    }
    fn parse_error_receiver(& self) -> Option<JsonParseErrorReceiver> {
        Some(self.parse_error_receiver())
    }
    fn start_object(& self) {
        self.start_object()
    }
    fn end_object(& self) {
        self.end_object()
    }
    fn object_key(& self, key__616: std::sync::Arc<String>) {
        self.object_key(key__616)
    }
    fn start_array(& self) {
        self.start_array()
    }
    fn end_array(& self) {
        self.end_array()
    }
    fn null_value(& self) {
        self.null_value()
    }
    fn boolean_value(& self, x__626: bool) {
        self.boolean_value(x__626)
    }
    fn int32_value(& self, x__629: i32) {
        self.int32_value(x__629)
    }
    fn int64_value(& self, x__632: i64) {
        self.int64_value(x__632)
    }
    fn float64_value(& self, x__635: f64) {
        self.float64_value(x__635)
    }
    fn numeric_token_value(& self, x__638: std::sync::Arc<String>) {
        self.numeric_token_value(x__638)
    }
    fn string_value(& self, x__641: std::sync::Arc<String>) {
        self.string_value(x__641)
    }
    fn interchange_context(& self) -> InterchangeContext {
        self.interchange_context()
    }
}
impl JsonParseErrorReceiverTrait for JsonSyntaxTreeProducer {
    fn clone_boxed(& self) -> JsonParseErrorReceiver {
        JsonParseErrorReceiver::new(self.clone())
    }
    fn explain_json_error(& self, error__651: std::sync::Arc<String>) {
        self.explain_json_error(error__651)
    }
}
temper_core::impl_any_value_trait!(JsonSyntaxTreeProducer, [JsonProducer, JsonParseErrorReceiver]);
fn parseJsonValue__356(sourceText__670: impl temper_core::ToArcString, mut i__671: usize, out__672: JsonProducer) -> Option<usize> {
    let sourceText__670 = sourceText__670.to_arc_string();
    let return__302: Option<usize>;
    let mut t___2201: usize;
    let mut t___2204: i32;
    let mut t___1412: bool;
    'fn__673: {
        t___2201 = skipJsonSpaces__355(sourceText__670.clone(), i__671);
        i__671 = t___2201;
        if ! temper_core::string::has_index( & sourceText__670, i__671) {
            expectedTokenError__353(sourceText__670.clone(), i__671, out__672.clone(), "JSON value");
            return__302 = None;
            break 'fn__673;
        }
        t___2204 = temper_core::string::get( & sourceText__670, i__671);
        if Some(t___2204) == Some(123) {
            return__302 = parseJsonObject__357(sourceText__670.clone(), i__671, out__672.clone());
        } else {
            if Some(t___2204) == Some(91) {
                return__302 = parseJsonArray__358(sourceText__670.clone(), i__671, out__672.clone());
            } else {
                if Some(t___2204) == Some(34) {
                    return__302 = parseJsonString__359(sourceText__670.clone(), i__671, out__672.clone());
                } else {
                    if Some(t___2204) == Some(116) {
                        t___1412 = true;
                    } else {
                        t___1412 = Some(t___2204) == Some(102);
                    }
                    if t___1412 {
                        return__302 = parseJsonBoolean__362(sourceText__670.clone(), i__671, out__672.clone());
                    } else {
                        if Some(t___2204) == Some(110) {
                            return__302 = parseJsonNull__363(sourceText__670.clone(), i__671, out__672.clone());
                        } else {
                            return__302 = parseJsonNumber__365(sourceText__670.clone(), i__671, out__672.clone());
                        }
                    }
                }
            }
        }
    }
    return return__302;
}
pub trait JsonAdapterTrait<T: Clone + std::marker::Send + std::marker::Sync + 'static>: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> JsonAdapter<T>;
    fn encode_to_json(& self, x__765: T, p__766: JsonProducer);
    fn decode_from_json(& self, t__769: JsonSyntaxTree, ic__770: InterchangeContext) -> temper_core::Result<T>;
}
#[derive(Clone)]
pub struct JsonAdapter<T: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<dyn JsonAdapterTrait<T>>);
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> JsonAdapter<T> {
    pub fn new(selfish: impl JsonAdapterTrait<T> + 'static) -> JsonAdapter<T> {
        JsonAdapter(std::sync::Arc::new(selfish))
    }
}
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> JsonAdapterTrait<T> for JsonAdapter<T> {
    fn clone_boxed(& self) -> JsonAdapter<T> {
        JsonAdapterTrait::clone_boxed( & ( * self.0))
    }
    fn encode_to_json(& self, arg1: T, arg2: JsonProducer) -> () {
        JsonAdapterTrait::encode_to_json( & ( * self.0), arg1, arg2)
    }
    fn decode_from_json(& self, arg1: JsonSyntaxTree, arg2: InterchangeContext) -> temper_core::Result<T> {
        JsonAdapterTrait::decode_from_json( & ( * self.0), arg1, arg2)
    }
}
temper_core::impl_any_value_trait_for_interface!(JsonAdapter<T>);
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> std::ops::Deref for JsonAdapter<T> {
    type Target = dyn JsonAdapterTrait<T>;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct BooleanJsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct BooleanJsonAdapter(std::sync::Arc<BooleanJsonAdapterStruct>);
impl BooleanJsonAdapter {
    pub fn encode_to_json(& self, x__773: bool, p__774: JsonProducer) {
        p__774.boolean_value(x__773);
    }
    pub fn decode_from_json(& self, t__777: JsonSyntaxTree, ic__778: InterchangeContext) -> temper_core::Result<bool> {
        let mut t___1391: JsonBoolean;
        t___1391 = temper_core::cast::<JsonBoolean>(t__777.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return Ok(t___1391.content());
    }
    pub fn new() -> BooleanJsonAdapter {
        let selfish = BooleanJsonAdapter(std::sync::Arc::new(BooleanJsonAdapterStruct {}));
        return selfish;
    }
}
impl JsonAdapterTrait<bool> for BooleanJsonAdapter {
    fn clone_boxed(& self) -> JsonAdapter<bool> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__773: bool, p__774: JsonProducer) {
        self.encode_to_json(x__773, p__774)
    }
    fn decode_from_json(& self, t__777: JsonSyntaxTree, ic__778: InterchangeContext) -> temper_core::Result<bool> {
        self.decode_from_json(t__777, ic__778)
    }
}
temper_core::impl_any_value_trait!(BooleanJsonAdapter, [JsonAdapter<bool>]);
struct Float64JsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct Float64JsonAdapter(std::sync::Arc<Float64JsonAdapterStruct>);
impl Float64JsonAdapter {
    pub fn encode_to_json(& self, x__783: f64, p__784: JsonProducer) {
        p__784.float64_value(x__783);
    }
    pub fn decode_from_json(& self, t__787: JsonSyntaxTree, ic__788: InterchangeContext) -> temper_core::Result<f64> {
        let return__323: f64;
        let mut t___1387: JsonNumeric;
        t___1387 = temper_core::cast::<JsonNumeric>(t__787.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return__323 = t___1387.as_float64() ? ;
        return Ok(return__323);
    }
    pub fn new() -> Float64JsonAdapter {
        let selfish = Float64JsonAdapter(std::sync::Arc::new(Float64JsonAdapterStruct {}));
        return selfish;
    }
}
impl JsonAdapterTrait<f64> for Float64JsonAdapter {
    fn clone_boxed(& self) -> JsonAdapter<f64> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__783: f64, p__784: JsonProducer) {
        self.encode_to_json(x__783, p__784)
    }
    fn decode_from_json(& self, t__787: JsonSyntaxTree, ic__788: InterchangeContext) -> temper_core::Result<f64> {
        self.decode_from_json(t__787, ic__788)
    }
}
temper_core::impl_any_value_trait!(Float64JsonAdapter, [JsonAdapter<f64>]);
struct Int32JsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct Int32JsonAdapter(std::sync::Arc<Int32JsonAdapterStruct>);
impl Int32JsonAdapter {
    pub fn encode_to_json(& self, x__793: i32, p__794: JsonProducer) {
        p__794.int32_value(x__793);
    }
    pub fn decode_from_json(& self, t__797: JsonSyntaxTree, ic__798: InterchangeContext) -> temper_core::Result<i32> {
        let return__328: i32;
        let mut t___1383: JsonNumeric;
        t___1383 = temper_core::cast::<JsonNumeric>(t__797.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return__328 = t___1383.as_int32() ? ;
        return Ok(return__328);
    }
    pub fn new() -> Int32JsonAdapter {
        let selfish = Int32JsonAdapter(std::sync::Arc::new(Int32JsonAdapterStruct {}));
        return selfish;
    }
}
impl JsonAdapterTrait<i32> for Int32JsonAdapter {
    fn clone_boxed(& self) -> JsonAdapter<i32> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__793: i32, p__794: JsonProducer) {
        self.encode_to_json(x__793, p__794)
    }
    fn decode_from_json(& self, t__797: JsonSyntaxTree, ic__798: InterchangeContext) -> temper_core::Result<i32> {
        self.decode_from_json(t__797, ic__798)
    }
}
temper_core::impl_any_value_trait!(Int32JsonAdapter, [JsonAdapter<i32>]);
struct Int64JsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct Int64JsonAdapter(std::sync::Arc<Int64JsonAdapterStruct>);
impl Int64JsonAdapter {
    pub fn encode_to_json(& self, x__803: i64, p__804: JsonProducer) {
        p__804.int64_value(x__803);
    }
    pub fn decode_from_json(& self, t__807: JsonSyntaxTree, ic__808: InterchangeContext) -> temper_core::Result<i64> {
        let return__333: i64;
        let mut t___1379: JsonNumeric;
        t___1379 = temper_core::cast::<JsonNumeric>(t__807.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return__333 = t___1379.as_int64() ? ;
        return Ok(return__333);
    }
    pub fn new() -> Int64JsonAdapter {
        let selfish = Int64JsonAdapter(std::sync::Arc::new(Int64JsonAdapterStruct {}));
        return selfish;
    }
}
impl JsonAdapterTrait<i64> for Int64JsonAdapter {
    fn clone_boxed(& self) -> JsonAdapter<i64> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__803: i64, p__804: JsonProducer) {
        self.encode_to_json(x__803, p__804)
    }
    fn decode_from_json(& self, t__807: JsonSyntaxTree, ic__808: InterchangeContext) -> temper_core::Result<i64> {
        self.decode_from_json(t__807, ic__808)
    }
}
temper_core::impl_any_value_trait!(Int64JsonAdapter, [JsonAdapter<i64>]);
struct StringJsonAdapterStruct {}
#[derive(Clone)]
pub (crate) struct StringJsonAdapter(std::sync::Arc<StringJsonAdapterStruct>);
impl StringJsonAdapter {
    pub fn encode_to_json(& self, x__813: impl temper_core::ToArcString, p__814: JsonProducer) {
        let x__813 = x__813.to_arc_string();
        p__814.string_value(x__813.clone());
    }
    pub fn decode_from_json(& self, t__817: JsonSyntaxTree, ic__818: InterchangeContext) -> temper_core::Result<std::sync::Arc<String>> {
        let mut t___1375: JsonString;
        t___1375 = temper_core::cast::<JsonString>(t__817.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        return Ok(t___1375.content());
    }
    pub fn new() -> StringJsonAdapter {
        let selfish = StringJsonAdapter(std::sync::Arc::new(StringJsonAdapterStruct {}));
        return selfish;
    }
}
impl JsonAdapterTrait<std::sync::Arc<String>> for StringJsonAdapter {
    fn clone_boxed(& self) -> JsonAdapter<std::sync::Arc<String>> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__813: std::sync::Arc<String>, p__814: JsonProducer) {
        self.encode_to_json(x__813, p__814)
    }
    fn decode_from_json(& self, t__817: JsonSyntaxTree, ic__818: InterchangeContext) -> temper_core::Result<std::sync::Arc<String>> {
        self.decode_from_json(t__817, ic__818)
    }
}
temper_core::impl_any_value_trait!(StringJsonAdapter, [JsonAdapter<std::sync::Arc<String>>]);
struct ListJsonAdapterStruct<T: Clone + std::marker::Send + std::marker::Sync + 'static> {
    adapter_for_t: JsonAdapter<T>, phantom_T: std::marker::PhantomData<T>
}
#[derive(Clone)]
pub (crate) struct ListJsonAdapter<T: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<ListJsonAdapterStruct<T>>);
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> ListJsonAdapter<T> {
    pub fn encode_to_json(& self, x__824: impl temper_core::ToList<T>, p__825: JsonProducer) {
        let x__824 = x__824.to_list();
        p__825.start_array();
        #[derive(Clone)]
        struct ClosureGroup___5<T> where T: Clone + std::marker::Send + std::marker::Sync + 'static {
            this__181: ListJsonAdapter<T>, p__825: JsonProducer, phantom_T: std::marker::PhantomData<T>
        }
        impl<T> ClosureGroup___5<T> where T: Clone + std::marker::Send + std::marker::Sync + 'static {
            fn fn__2174(& self, el__827: T) {
                self.this__181.0.adapter_for_t.encode_to_json(el__827.clone(), self.p__825.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            this__181: self.clone(), p__825: p__825.clone(), phantom_T: std::marker::PhantomData
        };
        let fn__2174 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | el__827: T | closure_group.fn__2174(el__827))
        };
        temper_core::listed::list_for_each( & x__824, & ( * fn__2174.clone()));
        p__825.end_array();
    }
    pub fn decode_from_json(& self, t__829: JsonSyntaxTree, ic__830: InterchangeContext) -> temper_core::Result<temper_core::List<T>> {
        let mut t___1369: T;
        let b__832: temper_core::ListBuilder<T> = temper_core::listed::new_builder();
        let mut t___1364: JsonArray;
        t___1364 = temper_core::cast::<JsonArray>(t__829.clone()).ok_or_else(| | temper_core::Error::new()) ? ;
        let elements__833: temper_core::List<JsonSyntaxTree> = t___1364.elements();
        let n__834: i32 = temper_core::ListedTrait::len( & elements__833);
        let mut i__835: i32 = 0;
        'loop___2649: while Some(i__835) < Some(n__834) {
            let el__836: JsonSyntaxTree = temper_core::ListedTrait::get( & elements__833, i__835);
            i__835 = i__835.wrapping_add(1);
            t___1369 = self.0.adapter_for_t.decode_from_json(el__836.clone(), ic__830.clone()) ? ;
            temper_core::listed::add( & b__832, t___1369.clone(), None);
        }
        return Ok(temper_core::ListedTrait::to_list( & b__832));
    }
    pub fn new(adapterForT__838: JsonAdapter<T>) -> ListJsonAdapter<T> {
        let adapter_for_t;
        adapter_for_t = adapterForT__838.clone();
        let selfish = ListJsonAdapter(std::sync::Arc::new(ListJsonAdapterStruct {
                    adapter_for_t, phantom_T: std::marker::PhantomData
        }));
        return selfish;
    }
}
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> JsonAdapterTrait<temper_core::List<T>> for ListJsonAdapter<T> {
    fn clone_boxed(& self) -> JsonAdapter<temper_core::List<T>> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__824: temper_core::List<T>, p__825: JsonProducer) {
        self.encode_to_json(x__824, p__825)
    }
    fn decode_from_json(& self, t__829: JsonSyntaxTree, ic__830: InterchangeContext) -> temper_core::Result<temper_core::List<T>> {
        self.decode_from_json(t__829, ic__830)
    }
}
temper_core::impl_any_value_trait!(ListJsonAdapter<T>, [JsonAdapter<temper_core::List<T>>]);
struct OrNullJsonAdapterStruct<T: Clone + std::marker::Send + std::marker::Sync + 'static> {
    adapter_for_t: JsonAdapter<T>, phantom_T: std::marker::PhantomData<T>
}
#[derive(Clone)]
pub struct OrNullJsonAdapter<T: Clone + std::marker::Send + std::marker::Sync + 'static>(std::sync::Arc<OrNullJsonAdapterStruct<T>>);
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> OrNullJsonAdapter<T> {
    pub fn encode_to_json(& self, x__843: Option<T>, p__844: JsonProducer) {
        if x__843.is_none() {
            p__844.null_value();
        } else {
            let x___962: T = x__843.clone().unwrap();
            self.0.adapter_for_t.encode_to_json(x___962.clone(), p__844.clone());
        }
    }
    pub fn decode_from_json(& self, t__847: JsonSyntaxTree, ic__848: InterchangeContext) -> temper_core::Result<Option<T>> {
        let return__350: Option<T>;
        if temper_core::is::<JsonNull>(t__847.clone()) {
            return__350 = None;
        } else {
            return__350 = Some(self.0.adapter_for_t.decode_from_json(t__847.clone(), ic__848.clone()) ? );
        }
        return Ok(return__350.clone());
    }
    pub fn new(adapterForT__851: JsonAdapter<T>) -> OrNullJsonAdapter<T> {
        let adapter_for_t;
        adapter_for_t = adapterForT__851.clone();
        let selfish = OrNullJsonAdapter(std::sync::Arc::new(OrNullJsonAdapterStruct {
                    adapter_for_t, phantom_T: std::marker::PhantomData
        }));
        return selfish;
    }
}
impl<T: Clone + std::marker::Send + std::marker::Sync + 'static> JsonAdapterTrait<Option<T>> for OrNullJsonAdapter<T> {
    fn clone_boxed(& self) -> JsonAdapter<Option<T>> {
        JsonAdapter::new(self.clone())
    }
    fn encode_to_json(& self, x__843: Option<T>, p__844: JsonProducer) {
        self.encode_to_json(x__843, p__844)
    }
    fn decode_from_json(& self, t__847: JsonSyntaxTree, ic__848: InterchangeContext) -> temper_core::Result<Option<T>> {
        self.decode_from_json(t__847, ic__848)
    }
}
temper_core::impl_any_value_trait!(OrNullJsonAdapter<T>, [JsonAdapter<Option<T>>]);
fn encodeHex4__352(cp__580: i32, buffer__581: std::sync::Arc<std::sync::RwLock<String>>) {
    let b0__583: i32 = cp__580.wrapping_div(4096) & 15;
    let b1__584: i32 = cp__580.wrapping_div(256) & 15;
    let b2__585: i32 = cp__580.wrapping_div(16) & 15;
    let b3__586: i32 = cp__580 & 15;
    let mut t___2482: std::sync::Arc<String> = temper_core::ListedTrait::get( & hex_digits(), b0__583);
    temper_core::string::builder::append( & buffer__581, t___2482.clone());
    let mut t___2484: std::sync::Arc<String> = temper_core::ListedTrait::get( & hex_digits(), b1__584);
    temper_core::string::builder::append( & buffer__581, t___2484.clone());
    let mut t___2486: std::sync::Arc<String> = temper_core::ListedTrait::get( & hex_digits(), b2__585);
    temper_core::string::builder::append( & buffer__581, t___2486.clone());
    let mut t___2488: std::sync::Arc<String> = temper_core::ListedTrait::get( & hex_digits(), b3__586);
    temper_core::string::builder::append( & buffer__581, t___2488.clone());
}
fn encodeJsonString__351(x__572: impl temper_core::ToArcString, buffer__573: std::sync::Arc<std::sync::RwLock<String>>) {
    let x__572 = x__572.to_arc_string();
    let mut t___1745: bool;
    let mut t___1746: bool;
    let mut t___1747: std::sync::Arc<String>;
    let mut t___1748: std::sync::Arc<String>;
    temper_core::string::builder::append( & buffer__573, "\"");
    let mut i__575: usize = 0usize;
    let mut emitted__576: usize = i__575;
    'loop___2651: loop {
        if ! temper_core::string::has_index( & x__572, i__575) {
            break;
        }
        let cp__577: i32 = temper_core::string::get( & x__572, i__575);
        if Some(cp__577) == Some(8) {
            t___1748 = std::sync::Arc::new("\\b".to_string());
        } else {
            if Some(cp__577) == Some(9) {
                t___1748 = std::sync::Arc::new("\\t".to_string());
            } else {
                if Some(cp__577) == Some(10) {
                    t___1748 = std::sync::Arc::new("\\n".to_string());
                } else {
                    if Some(cp__577) == Some(12) {
                        t___1748 = std::sync::Arc::new("\\f".to_string());
                    } else {
                        if Some(cp__577) == Some(13) {
                            t___1748 = std::sync::Arc::new("\\r".to_string());
                        } else {
                            if Some(cp__577) == Some(34) {
                                t___1748 = std::sync::Arc::new("\\\"".to_string());
                            } else {
                                if Some(cp__577) == Some(92) {
                                    t___1748 = std::sync::Arc::new("\\\\".to_string());
                                } else {
                                    if Some(cp__577) < Some(32) {
                                        t___1746 = true;
                                    } else {
                                        if Some(55296) <= Some(cp__577) {
                                            t___1745 = Some(cp__577) <= Some(57343);
                                        } else {
                                            t___1745 = false;
                                        }
                                        t___1746 = t___1745;
                                    }
                                    if t___1746 {
                                        t___1747 = std::sync::Arc::new("\\u".to_string());
                                    } else {
                                        t___1747 = std::sync::Arc::new("".to_string());
                                    }
                                    t___1748 = t___1747.clone();
                                }
                            }
                        }
                    }
                }
            }
        }
        let replacement__578: std::sync::Arc<String> = t___1748.clone();
        let nextI__579: usize = temper_core::string::next( & x__572, i__575);
        if Some(replacement__578.as_str()) != Some("") {
            temper_core::string::builder::append_between( & buffer__573, x__572.clone(), emitted__576, i__575);
            temper_core::string::builder::append( & buffer__573, replacement__578.clone());
            if Some(replacement__578.as_str()) == Some("\\u") {
                encodeHex4__352(cp__577, buffer__573.clone());
            }
            emitted__576 = nextI__579;
        }
        i__575 = nextI__579;
    }
    temper_core::string::builder::append_between( & buffer__573, x__572.clone(), emitted__576, i__575);
    temper_core::string::builder::append( & buffer__573, "\"");
}
fn storeJsonError__354(out__659: JsonProducer, explanation__660: impl temper_core::ToArcString) {
    let explanation__660 = explanation__660.to_arc_string();
    let mut t___2361: Option<JsonParseErrorReceiver> = out__659.parse_error_receiver();
    if ! t___2361.is_none() {
        t___2361.clone().unwrap().explain_json_error(explanation__660.clone());
    }
}
fn expectedTokenError__353(sourceText__653: impl temper_core::ToArcString, i__654: usize, out__655: JsonProducer, shortExplanation__656: impl temper_core::ToArcString) {
    let sourceText__653 = sourceText__653.to_arc_string();
    let shortExplanation__656 = shortExplanation__656.to_arc_string();
    let mut t___2358: usize;
    let mut t___2359: std::sync::Arc<String>;
    let gotten__658: std::sync::Arc<String>;
    if temper_core::string::has_index( & sourceText__653, i__654) {
        t___2358 = sourceText__653.len();
        t___2359 = temper_core::string::slice( & sourceText__653, i__654, t___2358);
        gotten__658 = std::sync::Arc::new(format!("`{}`", t___2359.clone()));
    } else {
        gotten__658 = std::sync::Arc::new("end-of-file".to_string());
    }
    storeJsonError__354(out__655.clone(), std::sync::Arc::new(format!("Expected {}, but got {}", shortExplanation__656.clone(), gotten__658.clone())));
}
fn skipJsonSpaces__355(sourceText__667: impl temper_core::ToArcString, mut i__668: usize) -> usize {
    let sourceText__667 = sourceText__667.to_arc_string();
    let mut t___2355: i32;
    let mut t___2356: usize;
    let mut t___1616: bool;
    let mut t___1617: bool;
    let mut t___1618: bool;
    'loop___2653: loop {
        if ! temper_core::string::has_index( & sourceText__667, i__668) {
            break;
        }
        t___2355 = temper_core::string::get( & sourceText__667, i__668);
        if Some(t___2355) == Some(9) {
            t___1618 = true;
        } else {
            if Some(t___2355) == Some(10) {
                t___1617 = true;
            } else {
                if Some(t___2355) == Some(13) {
                    t___1616 = true;
                } else {
                    t___1616 = Some(t___2355) == Some(32);
                }
                t___1617 = t___1616;
            }
            t___1618 = t___1617;
        }
        if ! t___1618 {
            break;
        }
        t___2356 = temper_core::string::next( & sourceText__667, i__668);
        i__668 = t___2356;
    }
    return i__668;
}
fn decodeHexUnsigned__361(sourceText__708: impl temper_core::ToArcString, start__709: usize, limit__710: usize) -> i32 {
    let sourceText__708 = sourceText__708.to_arc_string();
    let return__307: i32;
    let mut t___2353: usize;
    let mut t___1609: bool;
    let mut t___1610: bool;
    let mut t___1611: bool;
    let mut t___1612: i32;
    'fn__711: {
        let mut n__712: i32 = 0;
        let mut i__713: usize = start__709;
        'loop___2654: loop {
            if ! (Some(Some(i__713).cmp( & Some(limit__710)) as i32) < Some(0)) {
                break;
            }
            let cp__714: i32 = temper_core::string::get( & sourceText__708, i__713);
            if Some(48) <= Some(cp__714) {
                t___1609 = Some(cp__714) <= Some(48);
            } else {
                t___1609 = false;
            }
            if t___1609 {
                t___1612 = cp__714.wrapping_sub(48);
            } else {
                if Some(65) <= Some(cp__714) {
                    t___1610 = Some(cp__714) <= Some(70);
                } else {
                    t___1610 = false;
                }
                if t___1610 {
                    t___1612 = cp__714.wrapping_sub(65).wrapping_add(10);
                } else {
                    if Some(97) <= Some(cp__714) {
                        t___1611 = Some(cp__714) <= Some(102);
                    } else {
                        t___1611 = false;
                    }
                    if t___1611 {
                        t___1612 = cp__714.wrapping_sub(97).wrapping_add(10);
                    } else {
                        return__307 = -1;
                        break 'fn__711;
                    }
                }
            }
            let digit__715: i32 = t___1612;
            n__712 = n__712.wrapping_mul(16).wrapping_add(digit__715);
            t___2353 = temper_core::string::next( & sourceText__708, i__713);
            i__713 = t___2353;
        }
        return__307 = n__712;
    }
    return return__307;
}
fn parseJsonStringTo__360(sourceText__692: impl temper_core::ToArcString, mut i__693: usize, sb__694: std::sync::Arc<std::sync::RwLock<String>>, errOut__695: JsonProducer) -> Option<usize> {
    let sourceText__692 = sourceText__692.to_arc_string();
    let return__306: Option<usize>;
    let mut t___2326: i32;
    let mut t___2328: usize;
    let mut t___2331: usize;
    let mut t___2336: usize;
    let mut t___2338: usize;
    let mut t___2339: usize;
    let mut t___2340: usize;
    let mut t___2341: usize;
    let mut t___2342: i32;
    let mut t___2347: i32;
    let mut t___2350: usize;
    let mut t___1570: bool;
    let mut t___1579: bool;
    let mut t___1580: bool;
    let mut t___1588: i32;
    let mut t___1589: i32;
    let mut t___1591: i32;
    let mut t___1593: i32;
    let mut t___1594: bool;
    let mut t___1595: bool;
    let mut t___1597: bool;
    let mut t___1601: bool;
    'fn__696: {
        if ! temper_core::string::has_index( & sourceText__692, i__693) {
            t___1570 = true;
        } else {
            t___2326 = temper_core::string::get( & sourceText__692, i__693);
            t___1570 = Some(t___2326) != Some(34);
        }
        if t___1570 {
            expectedTokenError__353(sourceText__692.clone(), i__693, errOut__695.clone(), "\"");
            return__306 = None;
            break 'fn__696;
        }
        t___2328 = temper_core::string::next( & sourceText__692, i__693);
        i__693 = t___2328;
        let mut leadSurrogate__697: i32 = -1;
        let mut consumed__698: usize = i__693;
        'loop___2655: loop {
            if ! temper_core::string::has_index( & sourceText__692, i__693) {
                break;
            }
            let cp__699: i32 = temper_core::string::get( & sourceText__692, i__693);
            if Some(cp__699) == Some(34) {
                break;
            }
            t___2331 = temper_core::string::next( & sourceText__692, i__693);
            let mut iNext__700: usize = t___2331;
            let end__701: usize = sourceText__692.len();
            let mut needToFlush__702: bool = false;
            if Some(cp__699) != Some(92) {
                t___1593 = cp__699;
            } else {
                needToFlush__702 = true;
                if ! temper_core::string::has_index( & sourceText__692, iNext__700) {
                    expectedTokenError__353(sourceText__692.clone(), iNext__700, errOut__695.clone(), "escape sequence");
                    return__306 = None;
                    break 'fn__696;
                }
                let esc0__704: i32 = temper_core::string::get( & sourceText__692, iNext__700);
                t___2336 = temper_core::string::next( & sourceText__692, iNext__700);
                iNext__700 = t___2336;
                if Some(esc0__704) == Some(34) {
                    t___1580 = true;
                } else {
                    if Some(esc0__704) == Some(92) {
                        t___1579 = true;
                    } else {
                        t___1579 = Some(esc0__704) == Some(47);
                    }
                    t___1580 = t___1579;
                }
                if t___1580 {
                    t___1591 = esc0__704;
                } else {
                    if Some(esc0__704) == Some(98) {
                        t___1591 = 8;
                    } else {
                        if Some(esc0__704) == Some(102) {
                            t___1591 = 12;
                        } else {
                            if Some(esc0__704) == Some(110) {
                                t___1591 = 10;
                            } else {
                                if Some(esc0__704) == Some(114) {
                                    t___1591 = 13;
                                } else {
                                    if Some(esc0__704) == Some(116) {
                                        t___1591 = 9;
                                    } else {
                                        if Some(esc0__704) == Some(117) {
                                            if temper_core::string::has_at_least( & sourceText__692, iNext__700, end__701, 4) {
                                                let startHex__706: usize = iNext__700;
                                                t___2338 = temper_core::string::next( & sourceText__692, iNext__700);
                                                iNext__700 = t___2338;
                                                t___2339 = temper_core::string::next( & sourceText__692, iNext__700);
                                                iNext__700 = t___2339;
                                                t___2340 = temper_core::string::next( & sourceText__692, iNext__700);
                                                iNext__700 = t___2340;
                                                t___2341 = temper_core::string::next( & sourceText__692, iNext__700);
                                                iNext__700 = t___2341;
                                                t___2342 = decodeHexUnsigned__361(sourceText__692.clone(), startHex__706, iNext__700);
                                                t___1588 = t___2342;
                                            } else {
                                                t___1588 = -1;
                                            }
                                            let hex__705: i32 = t___1588;
                                            if Some(hex__705) < Some(0) {
                                                expectedTokenError__353(sourceText__692.clone(), iNext__700, errOut__695.clone(), "four hex digits");
                                                return__306 = None;
                                                break 'fn__696;
                                            }
                                            t___1589 = hex__705;
                                            t___1591 = t___1589;
                                        } else {
                                            expectedTokenError__353(sourceText__692.clone(), iNext__700, errOut__695.clone(), "escape sequence");
                                            return__306 = None;
                                            break 'fn__696;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                t___1593 = t___1591;
            }
            let mut decodedCp__703: i32 = t___1593;
            if Some(leadSurrogate__697) >= Some(0) {
                needToFlush__702 = true;
                let lead__707: i32 = leadSurrogate__697;
                if Some(56320) <= Some(decodedCp__703) {
                    t___1594 = Some(decodedCp__703) <= Some(57343);
                } else {
                    t___1594 = false;
                }
                if t___1594 {
                    leadSurrogate__697 = -1;
                    decodedCp__703 = (65536 as i32).wrapping_add(lead__707.wrapping_sub(55296).wrapping_mul(1024) | decodedCp__703.wrapping_sub(56320));
                }
            } else {
                if Some(55296) <= Some(decodedCp__703) {
                    t___1595 = Some(decodedCp__703) <= Some(56319);
                } else {
                    t___1595 = false;
                }
                if t___1595 {
                    needToFlush__702 = true;
                }
            }
            if needToFlush__702 {
                temper_core::string::builder::append_between( & sb__694, sourceText__692.clone(), consumed__698, i__693);
                if Some(leadSurrogate__697) >= Some(0) {
                    'ok___2573: {
                        'orelse___946: {
                            match temper_core::string::builder::append_code_point( & sb__694, leadSurrogate__697) {
                                Ok(x) => x,
                                _ => break 'orelse___946
                            };
                            break 'ok___2573;
                        }
                        return panic!();
                    }
                }
                if Some(55296) <= Some(decodedCp__703) {
                    t___1597 = Some(decodedCp__703) <= Some(56319);
                } else {
                    t___1597 = false;
                }
                if t___1597 {
                    leadSurrogate__697 = decodedCp__703;
                } else {
                    leadSurrogate__697 = -1;
                    'ok___2574: {
                        'orelse___947: {
                            match temper_core::string::builder::append_code_point( & sb__694, decodedCp__703) {
                                Ok(x) => x,
                                _ => break 'orelse___947
                            };
                            break 'ok___2574;
                        }
                        return panic!();
                    }
                }
                consumed__698 = iNext__700;
            }
            i__693 = iNext__700;
        }
        if ! temper_core::string::has_index( & sourceText__692, i__693) {
            t___1601 = true;
        } else {
            t___2347 = temper_core::string::get( & sourceText__692, i__693);
            t___1601 = Some(t___2347) != Some(34);
        }
        if t___1601 {
            expectedTokenError__353(sourceText__692.clone(), i__693, errOut__695.clone(), "\"");
            return__306 = None;
        } else {
            if Some(leadSurrogate__697) >= Some(0) {
                'ok___2575: {
                    'orelse___948: {
                        match temper_core::string::builder::append_code_point( & sb__694, leadSurrogate__697) {
                            Ok(x) => x,
                            _ => break 'orelse___948
                        };
                        break 'ok___2575;
                    }
                    return panic!();
                }
            } else {
                temper_core::string::builder::append_between( & sb__694, sourceText__692.clone(), consumed__698, i__693);
            }
            t___2350 = temper_core::string::next( & sourceText__692, i__693);
            i__693 = t___2350;
            return__306 = Some(i__693);
        }
    }
    return return__306;
}
fn parseJsonObject__357(sourceText__674: impl temper_core::ToArcString, mut i__675: usize, out__676: JsonProducer) -> Option<usize> {
    let sourceText__674 = sourceText__674.to_arc_string();
    let return__303: Option<usize>;
    let mut t___2296: i32;
    let mut t___2299: usize;
    let mut t___2300: usize;
    let mut t___2302: i32;
    let mut t___2306: std::sync::Arc<String>;
    let mut t___2308: usize;
    let mut t___2310: i32;
    let mut t___2311: usize;
    let mut t___2315: usize;
    let mut t___2317: i32;
    let mut t___2318: usize;
    let mut t___2319: usize;
    let mut t___2321: i32;
    let mut t___1533: bool;
    let mut t___1539: bool;
    let mut t___1545: usize;
    let mut t___1547: usize;
    let mut t___1551: bool;
    let mut t___1555: usize;
    let mut t___1560: bool;
    let mut t___1565: bool;
    'fn__677: {
        'ok___2578: {
            'orelse___949: {
                if ! temper_core::string::has_index( & sourceText__674, i__675) {
                    t___1533 = true;
                } else {
                    t___2296 = temper_core::string::get( & sourceText__674, i__675);
                    t___1533 = Some(t___2296) != Some(123);
                }
                if t___1533 {
                    expectedTokenError__353(sourceText__674.clone(), i__675, out__676.clone(), "'{'");
                    return__303 = None;
                    break 'fn__677;
                }
                out__676.start_object();
                t___2299 = temper_core::string::next( & sourceText__674, i__675);
                t___2300 = skipJsonSpaces__355(sourceText__674.clone(), t___2299);
                i__675 = t___2300;
                if temper_core::string::has_index( & sourceText__674, i__675) {
                    t___2302 = temper_core::string::get( & sourceText__674, i__675);
                    t___1539 = Some(t___2302) != Some(125);
                } else {
                    t___1539 = false;
                }
                if t___1539 {
                    'loop___2656: loop {
                        let keyBuffer__678: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
                        let afterKey__679: Option<usize> = parseJsonStringTo__360(sourceText__674.clone(), i__675, keyBuffer__678.clone(), out__676.clone());
                        if ! afterKey__679.is_some() {
                            return__303 = None;
                            break 'fn__677;
                        }
                        t___2306 = temper_core::string::builder::to_string( & keyBuffer__678);
                        out__676.object_key(t___2306.clone());
                        'ok___2579: {
                            'orelse___950: {
                                t___1545 = match temper_core::string::cast_as_index(afterKey__679).ok_or_else(| | temper_core::Error::new()) {
                                    Ok(x) => x,
                                    _ => break 'orelse___950
                                };
                                t___1547 = t___1545;
                                break 'ok___2579;
                            }
                            t___1547 = panic!();
                        }
                        t___2308 = skipJsonSpaces__355(sourceText__674.clone(), t___1547);
                        i__675 = t___2308;
                        if temper_core::string::has_index( & sourceText__674, i__675) {
                            t___2310 = temper_core::string::get( & sourceText__674, i__675);
                            t___1551 = Some(t___2310) == Some(58);
                        } else {
                            t___1551 = false;
                        }
                        if t___1551 {
                            t___2311 = temper_core::string::next( & sourceText__674, i__675);
                            i__675 = t___2311;
                            let afterPropertyValue__680: Option<usize> = parseJsonValue__356(sourceText__674.clone(), i__675, out__676.clone());
                            if ! afterPropertyValue__680.is_some() {
                                return__303 = None;
                                break 'fn__677;
                            }
                            t___1555 = match temper_core::string::cast_as_index(afterPropertyValue__680).ok_or_else(| | temper_core::Error::new()) {
                                Ok(x) => x,
                                _ => break 'orelse___949
                            };
                            i__675 = t___1555;
                        } else {
                            expectedTokenError__353(sourceText__674.clone(), i__675, out__676.clone(), "':'");
                            return__303 = None;
                            break 'fn__677;
                        }
                        t___2315 = skipJsonSpaces__355(sourceText__674.clone(), i__675);
                        i__675 = t___2315;
                        if temper_core::string::has_index( & sourceText__674, i__675) {
                            t___2317 = temper_core::string::get( & sourceText__674, i__675);
                            t___1560 = Some(t___2317) == Some(44);
                        } else {
                            t___1560 = false;
                        }
                        if t___1560 {
                            t___2318 = temper_core::string::next( & sourceText__674, i__675);
                            t___2319 = skipJsonSpaces__355(sourceText__674.clone(), t___2318);
                            i__675 = t___2319;
                        } else {
                            break;
                        }
                    }
                }
                if temper_core::string::has_index( & sourceText__674, i__675) {
                    t___2321 = temper_core::string::get( & sourceText__674, i__675);
                    t___1565 = Some(t___2321) == Some(125);
                } else {
                    t___1565 = false;
                }
                if t___1565 {
                    out__676.end_object();
                    return__303 = Some(temper_core::string::next( & sourceText__674, i__675));
                } else {
                    expectedTokenError__353(sourceText__674.clone(), i__675, out__676.clone(), "'}'");
                    return__303 = None;
                }
                break 'ok___2578;
            }
            return__303 = panic!();
        }
    }
    return return__303;
}
fn parseJsonArray__358(sourceText__681: impl temper_core::ToArcString, mut i__682: usize, out__683: JsonProducer) -> Option<usize> {
    let sourceText__681 = sourceText__681.to_arc_string();
    let return__304: Option<usize>;
    let mut t___2276: i32;
    let mut t___2279: usize;
    let mut t___2280: usize;
    let mut t___2282: i32;
    let mut t___2285: usize;
    let mut t___2287: i32;
    let mut t___2288: usize;
    let mut t___2289: usize;
    let mut t___2291: i32;
    let mut t___1509: bool;
    let mut t___1515: bool;
    let mut t___1518: usize;
    let mut t___1523: bool;
    let mut t___1528: bool;
    'fn__684: {
        'ok___2581: {
            'orelse___951: {
                if ! temper_core::string::has_index( & sourceText__681, i__682) {
                    t___1509 = true;
                } else {
                    t___2276 = temper_core::string::get( & sourceText__681, i__682);
                    t___1509 = Some(t___2276) != Some(91);
                }
                if t___1509 {
                    expectedTokenError__353(sourceText__681.clone(), i__682, out__683.clone(), "'['");
                    return__304 = None;
                    break 'fn__684;
                }
                out__683.start_array();
                t___2279 = temper_core::string::next( & sourceText__681, i__682);
                t___2280 = skipJsonSpaces__355(sourceText__681.clone(), t___2279);
                i__682 = t___2280;
                if temper_core::string::has_index( & sourceText__681, i__682) {
                    t___2282 = temper_core::string::get( & sourceText__681, i__682);
                    t___1515 = Some(t___2282) != Some(93);
                } else {
                    t___1515 = false;
                }
                if t___1515 {
                    'loop___2657: loop {
                        let afterElementValue__685: Option<usize> = parseJsonValue__356(sourceText__681.clone(), i__682, out__683.clone());
                        if ! afterElementValue__685.is_some() {
                            return__304 = None;
                            break 'fn__684;
                        }
                        t___1518 = match temper_core::string::cast_as_index(afterElementValue__685).ok_or_else(| | temper_core::Error::new()) {
                            Ok(x) => x,
                            _ => break 'orelse___951
                        };
                        i__682 = t___1518;
                        t___2285 = skipJsonSpaces__355(sourceText__681.clone(), i__682);
                        i__682 = t___2285;
                        if temper_core::string::has_index( & sourceText__681, i__682) {
                            t___2287 = temper_core::string::get( & sourceText__681, i__682);
                            t___1523 = Some(t___2287) == Some(44);
                        } else {
                            t___1523 = false;
                        }
                        if t___1523 {
                            t___2288 = temper_core::string::next( & sourceText__681, i__682);
                            t___2289 = skipJsonSpaces__355(sourceText__681.clone(), t___2288);
                            i__682 = t___2289;
                        } else {
                            break;
                        }
                    }
                }
                if temper_core::string::has_index( & sourceText__681, i__682) {
                    t___2291 = temper_core::string::get( & sourceText__681, i__682);
                    t___1528 = Some(t___2291) == Some(93);
                } else {
                    t___1528 = false;
                }
                if t___1528 {
                    out__683.end_array();
                    return__304 = Some(temper_core::string::next( & sourceText__681, i__682));
                } else {
                    expectedTokenError__353(sourceText__681.clone(), i__682, out__683.clone(), "']'");
                    return__304 = None;
                }
                break 'ok___2581;
            }
            return__304 = panic!();
        }
    }
    return return__304;
}
fn parseJsonString__359(sourceText__686: impl temper_core::ToArcString, mut i__687: usize, out__688: JsonProducer) -> Option<usize> {
    let sourceText__686 = sourceText__686.to_arc_string();
    let mut t___2273: std::sync::Arc<String>;
    let sb__690: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    let after__691: Option<usize> = parseJsonStringTo__360(sourceText__686.clone(), i__687, sb__690.clone(), out__688.clone());
    if after__691.is_some() {
        t___2273 = temper_core::string::builder::to_string( & sb__690);
        out__688.string_value(t___2273.clone());
    }
    return after__691;
}
fn afterSubstring__364(string__730: impl temper_core::ToArcString, inString__731: usize, substring__732: impl temper_core::ToArcString) -> Option<usize> {
    let string__730 = string__730.to_arc_string();
    let substring__732 = substring__732.to_arc_string();
    let return__310: Option<usize>;
    let mut t___2268: usize;
    let mut t___2269: usize;
    'fn__733: {
        let mut i__734: usize = inString__731;
        let mut j__735: usize = 0usize;
        'loop___2658: loop {
            if ! temper_core::string::has_index( & substring__732, j__735) {
                break;
            }
            if ! temper_core::string::has_index( & string__730, i__734) {
                return__310 = None;
                break 'fn__733;
            }
            if Some(temper_core::string::get( & string__730, i__734)) != Some(temper_core::string::get( & substring__732, j__735)) {
                return__310 = None;
                break 'fn__733;
            }
            t___2268 = temper_core::string::next( & string__730, i__734);
            i__734 = t___2268;
            t___2269 = temper_core::string::next( & substring__732, j__735);
            j__735 = t___2269;
        }
        return__310 = Some(i__734);
    }
    return return__310;
}
fn parseJsonBoolean__362(sourceText__716: impl temper_core::ToArcString, mut i__717: usize, out__718: JsonProducer) -> Option<usize> {
    let sourceText__716 = sourceText__716.to_arc_string();
    let return__308: Option<usize>;
    let mut t___2257: i32;
    'fn__719: {
        let ch0__720: i32;
        if temper_core::string::has_index( & sourceText__716, i__717) {
            t___2257 = temper_core::string::get( & sourceText__716, i__717);
            ch0__720 = t___2257;
        } else {
            ch0__720 = 0;
        }
        let end__721: usize = sourceText__716.len();
        let keyword__722: Option<std::sync::Arc<String>>;
        let n__723: i32;
        if Some(ch0__720) == Some(102) {
            keyword__722 = Some(std::sync::Arc::new("false".to_string()));
            n__723 = 5;
        } else {
            if Some(ch0__720) == Some(116) {
                keyword__722 = Some(std::sync::Arc::new("true".to_string()));
                n__723 = 4;
            } else {
                keyword__722 = None;
                n__723 = 0;
            }
        }
        if ! keyword__722.is_none() {
            let keyword___958: std::sync::Arc<String> = keyword__722.clone().unwrap();
            if temper_core::string::has_at_least( & sourceText__716, i__717, end__721, n__723) {
                let after__724: Option<usize> = afterSubstring__364(sourceText__716.clone(), i__717, keyword___958.clone());
                if after__724.is_some() {
                    return__308 = Some(temper_core::string::cast_as_index(after__724).unwrap());
                    out__718.boolean_value(Some(n__723) == Some(4));
                    break 'fn__719;
                }
            }
        }
        expectedTokenError__353(sourceText__716.clone(), i__717, out__718.clone(), "`false` or `true`");
        return__308 = None;
    }
    return return__308;
}
fn parseJsonNull__363(sourceText__725: impl temper_core::ToArcString, i__726: usize, out__727: JsonProducer) -> Option<usize> {
    let sourceText__725 = sourceText__725.to_arc_string();
    let return__309: Option<usize>;
    'fn__728: {
        let after__729: Option<usize> = afterSubstring__364(sourceText__725.clone(), i__726, "null");
        if after__729.is_some() {
            return__309 = Some(temper_core::string::cast_as_index(after__729).unwrap());
            out__727.null_value();
            break 'fn__728;
        }
        expectedTokenError__353(sourceText__725.clone(), i__726, out__727.clone(), "`null`");
        return__309 = None;
    }
    return return__309;
}
fn parseJsonNumber__365(sourceText__736: impl temper_core::ToArcString, mut i__737: usize, out__738: JsonProducer) -> Option<usize> {
    let sourceText__736 = sourceText__736.to_arc_string();
    let return__311: Option<usize>;
    let mut t___2212: i32;
    let mut t___2213: usize;
    let mut t___2215: i32;
    let mut t___2217: usize;
    let mut t___2218: f64;
    let mut t___2219: i64;
    let mut t___2222: usize;
    let mut t___2223: f64;
    let mut t___2224: i64;
    let mut t___2228: i32;
    let mut t___2229: usize;
    let mut t___2232: usize;
    let mut t___2233: f64;
    let mut t___2236: i32;
    let mut t___2237: usize;
    let mut t___2241: usize;
    let mut t___2244: usize;
    let mut t___2246: i32;
    let mut t___1420: bool;
    let mut t___1425: bool;
    let mut t___1426: bool;
    let mut t___1434: bool;
    let mut t___1437: f64;
    let mut t___1439: i64;
    let mut t___1442: bool;
    let mut t___1443: bool;
    let mut t___1446: bool;
    let mut t___1450: bool;
    let mut t___1453: f64;
    let mut t___1456: bool;
    let mut t___1460: bool;
    let mut t___1464: bool;
    let mut t___1466: bool;
    let mut t___1467: bool;
    let mut t___1469: bool;
    let mut t___1472: bool;
    let mut t___1473: f64;
    let mut t___1474: bool;
    let mut t___1475: bool;
    'fn__739: {
        let mut isNegative__740: bool = false;
        let startOfNumber__741: usize = i__737;
        if temper_core::string::has_index( & sourceText__736, i__737) {
            t___2212 = temper_core::string::get( & sourceText__736, i__737);
            t___1420 = Some(t___2212) == Some(45);
        } else {
            t___1420 = false;
        }
        if t___1420 {
            isNegative__740 = true;
            t___2213 = temper_core::string::next( & sourceText__736, i__737);
            i__737 = t___2213;
        }
        let digit0__742: i32;
        if temper_core::string::has_index( & sourceText__736, i__737) {
            t___2215 = temper_core::string::get( & sourceText__736, i__737);
            digit0__742 = t___2215;
        } else {
            digit0__742 = -1;
        }
        if Some(digit0__742) < Some(48) {
            t___1425 = true;
        } else {
            t___1425 = Some(57) < Some(digit0__742);
        }
        if t___1425 {
            let error__743: std::sync::Arc<String>;
            if ! isNegative__740 {
                t___1426 = Some(digit0__742) != Some(46);
            } else {
                t___1426 = false;
            }
            if t___1426 {
                error__743 = std::sync::Arc::new("JSON value".to_string());
            } else {
                error__743 = std::sync::Arc::new("digit".to_string());
            }
            expectedTokenError__353(sourceText__736.clone(), i__737, out__738.clone(), error__743.clone());
            return__311 = None;
            break 'fn__739;
        }
        t___2217 = temper_core::string::next( & sourceText__736, i__737);
        i__737 = t___2217;
        let mut nDigits__744: i32 = 1;
        t___2218 = digit0__742.wrapping_sub(48) as f64;
        let mut tentativeFloat64__745: f64 = t___2218;
        t___2219 = digit0__742.wrapping_sub(48) as i64;
        let mut tentativeInt64__746: i64 = t___2219;
        let mut overflowInt64__747: bool = false;
        if Some(48) != Some(digit0__742) {
            'loop___2659: loop {
                if ! temper_core::string::has_index( & sourceText__736, i__737) {
                    break;
                }
                let possibleDigit__748: i32 = temper_core::string::get( & sourceText__736, i__737);
                if Some(48) <= Some(possibleDigit__748) {
                    t___1434 = Some(possibleDigit__748) <= Some(57);
                } else {
                    t___1434 = false;
                }
                if t___1434 {
                    t___2222 = temper_core::string::next( & sourceText__736, i__737);
                    i__737 = t___2222;
                    nDigits__744 = nDigits__744.wrapping_add(1);
                    let nextDigit__749: i32 = possibleDigit__748.wrapping_sub(48);
                    t___1437 = tentativeFloat64__745 * 10.0f64;
                    t___2223 = nextDigit__749 as f64;
                    tentativeFloat64__745 = t___1437 + t___2223;
                    let oldInt64__750: i64 = tentativeInt64__746;
                    t___1439 = tentativeInt64__746.wrapping_mul(10);
                    t___2224 = nextDigit__749 as i64;
                    tentativeInt64__746 = t___1439.wrapping_add(t___2224);
                    if Some(tentativeInt64__746) < Some(oldInt64__750) {
                        if Some((-9223372036854775808 as i64).wrapping_sub(oldInt64__750)) == Some((nextDigit__749 as i64).wrapping_neg()) {
                            if isNegative__740 {
                                t___1442 = Some(oldInt64__750) > Some(0);
                            } else {
                                t___1442 = false;
                            }
                            t___1443 = t___1442;
                        } else {
                            t___1443 = false;
                        }
                        if ! t___1443 {
                            overflowInt64__747 = true;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        let mut nDigitsAfterPoint__751: i32 = 0;
        if temper_core::string::has_index( & sourceText__736, i__737) {
            t___2228 = temper_core::string::get( & sourceText__736, i__737);
            t___1446 = Some(46) == Some(t___2228);
        } else {
            t___1446 = false;
        }
        if t___1446 {
            t___2229 = temper_core::string::next( & sourceText__736, i__737);
            i__737 = t___2229;
            let afterPoint__752: usize = i__737;
            'loop___2660: loop {
                if ! temper_core::string::has_index( & sourceText__736, i__737) {
                    break;
                }
                let possibleDigit__753: i32 = temper_core::string::get( & sourceText__736, i__737);
                if Some(48) <= Some(possibleDigit__753) {
                    t___1450 = Some(possibleDigit__753) <= Some(57);
                } else {
                    t___1450 = false;
                }
                if t___1450 {
                    t___2232 = temper_core::string::next( & sourceText__736, i__737);
                    i__737 = t___2232;
                    nDigits__744 = nDigits__744.wrapping_add(1);
                    nDigitsAfterPoint__751 = nDigitsAfterPoint__751.wrapping_add(1);
                    t___1453 = tentativeFloat64__745 * 10.0f64;
                    t___2233 = possibleDigit__753.wrapping_sub(48) as f64;
                    tentativeFloat64__745 = t___1453 + t___2233;
                } else {
                    break;
                }
            }
            if i__737 == afterPoint__752 {
                expectedTokenError__353(sourceText__736.clone(), i__737, out__738.clone(), "digit");
                return__311 = None;
                break 'fn__739;
            }
        }
        let mut nExponentDigits__754: i32 = 0;
        if temper_core::string::has_index( & sourceText__736, i__737) {
            t___2236 = temper_core::string::get( & sourceText__736, i__737);
            t___1456 = Some(101) == Some(t___2236 | 32);
        } else {
            t___1456 = false;
        }
        if t___1456 {
            t___2237 = temper_core::string::next( & sourceText__736, i__737);
            i__737 = t___2237;
            if ! temper_core::string::has_index( & sourceText__736, i__737) {
                expectedTokenError__353(sourceText__736.clone(), i__737, out__738.clone(), "sign or digit");
                return__311 = None;
                break 'fn__739;
            }
            let afterE__755: i32 = temper_core::string::get( & sourceText__736, i__737);
            if Some(afterE__755) == Some(43) {
                t___1460 = true;
            } else {
                t___1460 = Some(afterE__755) == Some(45);
            }
            if t___1460 {
                t___2241 = temper_core::string::next( & sourceText__736, i__737);
                i__737 = t___2241;
            }
            'loop___2661: loop {
                if ! temper_core::string::has_index( & sourceText__736, i__737) {
                    break;
                }
                let possibleDigit__756: i32 = temper_core::string::get( & sourceText__736, i__737);
                if Some(48) <= Some(possibleDigit__756) {
                    t___1464 = Some(possibleDigit__756) <= Some(57);
                } else {
                    t___1464 = false;
                }
                if t___1464 {
                    t___2244 = temper_core::string::next( & sourceText__736, i__737);
                    i__737 = t___2244;
                    nExponentDigits__754 = nExponentDigits__754.wrapping_add(1);
                } else {
                    break;
                }
            }
            if Some(nExponentDigits__754) == Some(0) {
                expectedTokenError__353(sourceText__736.clone(), i__737, out__738.clone(), "exponent digit");
                return__311 = None;
                break 'fn__739;
            }
        }
        let afterExponent__757: usize = i__737;
        if Some(nExponentDigits__754) == Some(0) {
            if Some(nDigitsAfterPoint__751) == Some(0) {
                t___1466 = ! overflowInt64__747;
            } else {
                t___1466 = false;
            }
            t___1467 = t___1466;
        } else {
            t___1467 = false;
        }
        if t___1467 {
            let value__758: i64;
            if isNegative__740 {
                value__758 = tentativeInt64__746.wrapping_neg();
            } else {
                value__758 = tentativeInt64__746;
            }
            if Some(-2147483648) <= Some(value__758) {
                t___1469 = Some(value__758) <= Some(2147483647);
            } else {
                t___1469 = false;
            }
            if t___1469 {
                t___2246 = value__758 as i32;
                out__738.int32_value(t___2246);
            } else {
                out__738.int64_value(value__758);
            }
            return__311 = Some(i__737);
            break 'fn__739;
        }
        let numericTokenString__759: std::sync::Arc<String> = temper_core::string::slice( & sourceText__736, startOfNumber__741, i__737);
        let mut doubleValue__760: f64 = f64::NAN;
        if Some(nExponentDigits__754) != Some(0) {
            t___1472 = true;
        } else {
            t___1472 = Some(nDigitsAfterPoint__751) != Some(0);
        }
        if t___1472 {
            'ok___2582: {
                'orelse___953: {
                    t___1473 = match temper_core::string::to_float64( & numericTokenString__759) {
                        Ok(x) => x,
                        _ => break 'orelse___953
                    };
                    doubleValue__760 = t___1473;
                    break 'ok___2582;
                }
            }
        }
        if temper_core::float64::cmp_option(Some(doubleValue__760), Some(f64::NEG_INFINITY)) != 0{
            if temper_core::float64::cmp_option(Some(doubleValue__760), Some(f64::INFINITY)) != 0{
                t___1474 = temper_core::float64::cmp_option(Some(doubleValue__760), Some(f64::NAN)) != 0;
            } else {
                t___1474 = false;
            }
            t___1475 = t___1474;
        } else {
            t___1475 = false;
        }
        if t___1475 {
            out__738.float64_value(doubleValue__760);
        } else {
            out__738.numeric_token_value(numericTokenString__759.clone());
        }
        return__311 = Some(i__737);
    }
    return return__311;
}
pub fn parse_json_to_producer(sourceText__662: impl temper_core::ToArcString, out__663: JsonProducer) {
    let sourceText__662 = sourceText__662.to_arc_string();
    let mut t___2195: usize;
    let mut t___2197: Option<JsonParseErrorReceiver>;
    let mut t___2198: usize;
    let mut t___2199: std::sync::Arc<String>;
    let mut t___1402: bool;
    let mut t___1405: usize;
    let mut i__665: usize = 0usize;
    let afterValue__666: Option<usize> = parseJsonValue__356(sourceText__662.clone(), i__665, out__663.clone());
    if afterValue__666.is_some() {
        t___1405 = temper_core::string::cast_as_index(afterValue__666).unwrap();
        t___2195 = skipJsonSpaces__355(sourceText__662.clone(), t___1405);
        i__665 = t___2195;
        if temper_core::string::has_index( & sourceText__662, i__665) {
            t___2197 = out__663.parse_error_receiver();
            t___1402 = ! t___2197.is_none();
        } else {
            t___1402 = false;
        }
        if t___1402 {
            t___2198 = sourceText__662.len();
            t___2199 = temper_core::string::slice( & sourceText__662, i__665, t___2198);
            storeJsonError__354(out__663.clone(), std::sync::Arc::new(format!("Extraneous JSON `{}`", t___2199.clone())));
        }
    }
}
pub fn parse_json(sourceText__761: impl temper_core::ToArcString) -> temper_core::Result<JsonSyntaxTree> {
    let sourceText__761 = sourceText__761.to_arc_string();
    let return__312: JsonSyntaxTree;
    let p__763: JsonSyntaxTreeProducer = JsonSyntaxTreeProducer::new();
    parse_json_to_producer(sourceText__761.clone(), JsonProducer::new(p__763.clone()));
    return__312 = p__763.to_json_syntax_tree() ? ;
    return Ok(return__312.clone());
}
pub fn boolean_json_adapter() -> JsonAdapter<bool> {
    return JsonAdapter::new(BooleanJsonAdapter::new());
}
pub fn float64_json_adapter() -> JsonAdapter<f64> {
    return JsonAdapter::new(Float64JsonAdapter::new());
}
pub fn int32_json_adapter() -> JsonAdapter<i32> {
    return JsonAdapter::new(Int32JsonAdapter::new());
}
pub fn int64_json_adapter() -> JsonAdapter<i64> {
    return JsonAdapter::new(Int64JsonAdapter::new());
}
pub fn string_json_adapter() -> JsonAdapter<std::sync::Arc<String>> {
    return JsonAdapter::new(StringJsonAdapter::new());
}
pub fn list_json_adapter<T>(adapterForT__839: JsonAdapter<T>) -> JsonAdapter<temper_core::List<T>> where T: Clone + std::marker::Send + std::marker::Sync + 'static {
    return JsonAdapter::new(ListJsonAdapter::new(adapterForT__839.clone()));
}
