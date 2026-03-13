#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
struct ChangesetErrorStruct {
    field: std::sync::Arc<String>, message: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct ChangesetError(std::sync::Arc<ChangesetErrorStruct>);
#[derive(Clone)]
pub struct ChangesetErrorBuilder {
    pub field: std::sync::Arc<String>, pub message: std::sync::Arc<String>
}
impl ChangesetErrorBuilder {
    pub fn build(self) -> ChangesetError {
        ChangesetError::new(self.field, self.message)
    }
}
impl ChangesetError {
    pub fn new(field__303: impl temper_core::ToArcString, message__304: impl temper_core::ToArcString) -> ChangesetError {
        let field__303 = field__303.to_arc_string();
        let message__304 = message__304.to_arc_string();
        let field;
        let message;
        field = field__303.clone();
        message = message__304.clone();
        let selfish = ChangesetError(std::sync::Arc::new(ChangesetErrorStruct {
                    field, message
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn message(& self) -> std::sync::Arc<String> {
        return self.0.message.clone();
    }
}
temper_core::impl_any_value_trait!(ChangesetError, []);
pub enum ChangesetEnum {
    ChangesetImpl(ChangesetImpl)
}
pub trait ChangesetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> ChangesetEnum;
    fn clone_boxed(& self) -> Changeset;
    fn table_def(& self) -> TableDef;
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
    fn errors(& self) -> temper_core::List<ChangesetError>;
    fn is_valid(& self) -> bool;
    fn cast(& self, allowedFields__314: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__317: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__320: SafeIdentifier, min__321: i32, max__322: i32) -> Changeset;
    fn validate_int(& self, field__325: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__328: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__331: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__334: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__339: i32) -> temper_core::Result<SqlFragment>;
}
#[derive(Clone)]
pub struct Changeset(std::sync::Arc<dyn ChangesetTrait>);
impl Changeset {
    pub fn new(selfish: impl ChangesetTrait + 'static) -> Changeset {
        Changeset(std::sync::Arc::new(selfish))
    }
}
impl ChangesetTrait for Changeset {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> Changeset {
        ChangesetTrait::clone_boxed( & ( * self.0))
    }
    fn table_def(& self) -> TableDef {
        ChangesetTrait::table_def( & ( * self.0))
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        ChangesetTrait::changes( & ( * self.0))
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        ChangesetTrait::errors( & ( * self.0))
    }
    fn is_valid(& self) -> bool {
        ChangesetTrait::is_valid( & ( * self.0))
    }
    fn cast(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::cast( & ( * self.0), arg1)
    }
    fn validate_required(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::validate_required( & ( * self.0), arg1)
    }
    fn validate_length(& self, arg1: SafeIdentifier, arg2: i32, arg3: i32) -> Changeset {
        ChangesetTrait::validate_length( & ( * self.0), arg1, arg2, arg3)
    }
    fn validate_int(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int( & ( * self.0), arg1)
    }
    fn validate_int64(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int64( & ( * self.0), arg1)
    }
    fn validate_float(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_float( & ( * self.0), arg1)
    }
    fn validate_bool(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_bool( & ( * self.0), arg1)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_insert_sql( & ( * self.0))
    }
    fn to_update_sql(& self, arg1: i32) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_update_sql( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(Changeset);
impl std::ops::Deref for Changeset {
    type Target = dyn ChangesetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ChangesetImplStruct {
    table_def: TableDef, params: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, changes: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, errors: temper_core::List<ChangesetError>, is_valid: bool
}
#[derive(Clone)]
pub (crate) struct ChangesetImpl(std::sync::Arc<ChangesetImplStruct>);
impl ChangesetImpl {
    pub fn table_def(& self) -> TableDef {
        return self.0.table_def.clone();
    }
    pub fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        return self.0.changes.clone();
    }
    pub fn errors(& self) -> temper_core::List<ChangesetError> {
        return self.0.errors.clone();
    }
    pub fn is_valid(& self) -> bool {
        return self.0.is_valid;
    }
    pub fn cast(& self, allowedFields__355: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__355 = allowedFields__355.to_list();
        let mb__357: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__96: ChangesetImpl, mb__357: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__4731(& self, f__358: SafeIdentifier) {
                let mut t___4729: std::sync::Arc<String>;
                let mut t___4726: std::sync::Arc<String> = f__358.sql_value();
                let val__359: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__96.0.params, t___4726.clone(), std::sync::Arc::new("".to_string()));
                if ! val__359.is_empty() {
                    t___4729 = f__358.sql_value();
                    temper_core::MapBuilder::set( & self.mb__357, t___4729.clone(), val__359.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__96: self.clone(), mb__357: mb__357.clone()
        };
        let fn__4731 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__358: SafeIdentifier | closure_group.fn__4731(f__358))
        };
        temper_core::listed::list_for_each( & allowedFields__355, & ( * fn__4731.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__357), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__361: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__361 = fields__361.to_list();
        let return__183: Changeset;
        let mut t___4724: temper_core::List<ChangesetError>;
        let mut t___2816: TableDef;
        let mut t___2817: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2818: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__362: {
            if ! self.0.is_valid {
                return__183 = Changeset::new(self.clone());
                break 'fn__362;
            }
            let eb__363: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__364: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__97: ChangesetImpl, eb__363: temper_core::ListBuilder<ChangesetError>, valid__364: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__4720(& self, f__365: SafeIdentifier) {
                    let mut t___4718: ChangesetError;
                    let mut t___4715: std::sync::Arc<String> = f__365.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__97.0.changes, t___4715.clone()) {
                        t___4718 = ChangesetError::new(f__365.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__363, t___4718.clone(), None);
                        {
                            * self.valid__364.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__97: self.clone(), eb__363: eb__363.clone(), valid__364: valid__364.clone()
            };
            let fn__4720 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__365: SafeIdentifier | closure_group.fn__4720(f__365))
            };
            temper_core::listed::list_for_each( & fields__361, & ( * fn__4720.clone()));
            t___2816 = self.0.table_def.clone();
            t___2817 = self.0.params.clone();
            t___2818 = self.0.changes.clone();
            t___4724 = temper_core::ListedTrait::to_list( & eb__363);
            return__183 = Changeset::new(ChangesetImpl::new(t___2816.clone(), t___2817.clone(), t___2818.clone(), t___4724.clone(), temper_core::read_locked( & valid__364)));
        }
        return return__183.clone();
    }
    pub fn validate_length(& self, field__367: SafeIdentifier, min__368: i32, max__369: i32) -> Changeset {
        let return__184: Changeset;
        let mut t___4702: std::sync::Arc<String>;
        let mut t___4713: temper_core::List<ChangesetError>;
        let mut t___2799: bool;
        let mut t___2805: TableDef;
        let mut t___2806: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2807: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__370: {
            if ! self.0.is_valid {
                return__184 = Changeset::new(self.clone());
                break 'fn__370;
            }
            t___4702 = field__367.sql_value();
            let val__371: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4702.clone(), std::sync::Arc::new("".to_string()));
            let len__372: i32 = temper_core::string::count_between( & val__371, 0usize, val__371.len());
            if Some(len__372) < Some(min__368) {
                t___2799 = true;
            } else {
                t___2799 = Some(len__372) > Some(max__369);
            }
            if t___2799 {
                let msg__373: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__368, max__369));
                let eb__374: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__374, ChangesetError::new(field__367.sql_value(), msg__373.clone()), None);
                t___2805 = self.0.table_def.clone();
                t___2806 = self.0.params.clone();
                t___2807 = self.0.changes.clone();
                t___4713 = temper_core::ListedTrait::to_list( & eb__374);
                return__184 = Changeset::new(ChangesetImpl::new(t___2805.clone(), t___2806.clone(), t___2807.clone(), t___4713.clone(), false));
                break 'fn__370;
            }
            return__184 = Changeset::new(self.clone());
        }
        return return__184.clone();
    }
    pub fn validate_int(& self, field__376: SafeIdentifier) -> Changeset {
        let return__185: Changeset;
        let mut t___4693: std::sync::Arc<String>;
        let mut t___4700: temper_core::List<ChangesetError>;
        let mut t___2790: TableDef;
        let mut t___2791: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2792: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__377: {
            if ! self.0.is_valid {
                return__185 = Changeset::new(self.clone());
                break 'fn__377;
            }
            t___4693 = field__376.sql_value();
            let val__378: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4693.clone(), std::sync::Arc::new("".to_string()));
            if val__378.is_empty() {
                return__185 = Changeset::new(self.clone());
                break 'fn__377;
            }
            let parseOk__379: bool;
            'ok___4832: {
                'orelse___1025: {
                    match temper_core::string::to_int( & val__378, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1025
                    };
                    parseOk__379 = true;
                    break 'ok___4832;
                }
                parseOk__379 = false;
            }
            if ! parseOk__379 {
                let eb__380: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__380, ChangesetError::new(field__376.sql_value(), "must be an integer"), None);
                t___2790 = self.0.table_def.clone();
                t___2791 = self.0.params.clone();
                t___2792 = self.0.changes.clone();
                t___4700 = temper_core::ListedTrait::to_list( & eb__380);
                return__185 = Changeset::new(ChangesetImpl::new(t___2790.clone(), t___2791.clone(), t___2792.clone(), t___4700.clone(), false));
                break 'fn__377;
            }
            return__185 = Changeset::new(self.clone());
        }
        return return__185.clone();
    }
    pub fn validate_int64(& self, field__382: SafeIdentifier) -> Changeset {
        let return__186: Changeset;
        let mut t___4684: std::sync::Arc<String>;
        let mut t___4691: temper_core::List<ChangesetError>;
        let mut t___2777: TableDef;
        let mut t___2778: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2779: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__383: {
            if ! self.0.is_valid {
                return__186 = Changeset::new(self.clone());
                break 'fn__383;
            }
            t___4684 = field__382.sql_value();
            let val__384: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4684.clone(), std::sync::Arc::new("".to_string()));
            if val__384.is_empty() {
                return__186 = Changeset::new(self.clone());
                break 'fn__383;
            }
            let parseOk__385: bool;
            'ok___4834: {
                'orelse___1026: {
                    match temper_core::string::to_int64( & val__384, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1026
                    };
                    parseOk__385 = true;
                    break 'ok___4834;
                }
                parseOk__385 = false;
            }
            if ! parseOk__385 {
                let eb__386: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__386, ChangesetError::new(field__382.sql_value(), "must be a 64-bit integer"), None);
                t___2777 = self.0.table_def.clone();
                t___2778 = self.0.params.clone();
                t___2779 = self.0.changes.clone();
                t___4691 = temper_core::ListedTrait::to_list( & eb__386);
                return__186 = Changeset::new(ChangesetImpl::new(t___2777.clone(), t___2778.clone(), t___2779.clone(), t___4691.clone(), false));
                break 'fn__383;
            }
            return__186 = Changeset::new(self.clone());
        }
        return return__186.clone();
    }
    pub fn validate_float(& self, field__388: SafeIdentifier) -> Changeset {
        let return__187: Changeset;
        let mut t___4675: std::sync::Arc<String>;
        let mut t___4682: temper_core::List<ChangesetError>;
        let mut t___2764: TableDef;
        let mut t___2765: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2766: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__389: {
            if ! self.0.is_valid {
                return__187 = Changeset::new(self.clone());
                break 'fn__389;
            }
            t___4675 = field__388.sql_value();
            let val__390: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4675.clone(), std::sync::Arc::new("".to_string()));
            if val__390.is_empty() {
                return__187 = Changeset::new(self.clone());
                break 'fn__389;
            }
            let parseOk__391: bool;
            'ok___4836: {
                'orelse___1027: {
                    match temper_core::string::to_float64( & val__390) {
                        Ok(x) => x,
                        _ => break 'orelse___1027
                    };
                    parseOk__391 = true;
                    break 'ok___4836;
                }
                parseOk__391 = false;
            }
            if ! parseOk__391 {
                let eb__392: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__392, ChangesetError::new(field__388.sql_value(), "must be a number"), None);
                t___2764 = self.0.table_def.clone();
                t___2765 = self.0.params.clone();
                t___2766 = self.0.changes.clone();
                t___4682 = temper_core::ListedTrait::to_list( & eb__392);
                return__187 = Changeset::new(ChangesetImpl::new(t___2764.clone(), t___2765.clone(), t___2766.clone(), t___4682.clone(), false));
                break 'fn__389;
            }
            return__187 = Changeset::new(self.clone());
        }
        return return__187.clone();
    }
    pub fn validate_bool(& self, field__394: SafeIdentifier) -> Changeset {
        let return__188: Changeset;
        let mut t___4666: std::sync::Arc<String>;
        let mut t___4673: temper_core::List<ChangesetError>;
        let mut t___2739: bool;
        let mut t___2740: bool;
        let mut t___2742: bool;
        let mut t___2743: bool;
        let mut t___2745: bool;
        let mut t___2751: TableDef;
        let mut t___2752: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2753: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__395: {
            if ! self.0.is_valid {
                return__188 = Changeset::new(self.clone());
                break 'fn__395;
            }
            t___4666 = field__394.sql_value();
            let val__396: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4666.clone(), std::sync::Arc::new("".to_string()));
            if val__396.is_empty() {
                return__188 = Changeset::new(self.clone());
                break 'fn__395;
            }
            let isTrue__397: bool;
            if Some(val__396.as_str()) == Some("true") {
                isTrue__397 = true;
            } else {
                if Some(val__396.as_str()) == Some("1") {
                    t___2740 = true;
                } else {
                    if Some(val__396.as_str()) == Some("yes") {
                        t___2739 = true;
                    } else {
                        t___2739 = Some(val__396.as_str()) == Some("on");
                    }
                    t___2740 = t___2739;
                }
                isTrue__397 = t___2740;
            }
            let isFalse__398: bool;
            if Some(val__396.as_str()) == Some("false") {
                isFalse__398 = true;
            } else {
                if Some(val__396.as_str()) == Some("0") {
                    t___2743 = true;
                } else {
                    if Some(val__396.as_str()) == Some("no") {
                        t___2742 = true;
                    } else {
                        t___2742 = Some(val__396.as_str()) == Some("off");
                    }
                    t___2743 = t___2742;
                }
                isFalse__398 = t___2743;
            }
            if ! isTrue__397 {
                t___2745 = ! isFalse__398;
            } else {
                t___2745 = false;
            }
            if t___2745 {
                let eb__399: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__399, ChangesetError::new(field__394.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___2751 = self.0.table_def.clone();
                t___2752 = self.0.params.clone();
                t___2753 = self.0.changes.clone();
                t___4673 = temper_core::ListedTrait::to_list( & eb__399);
                return__188 = Changeset::new(ChangesetImpl::new(t___2751.clone(), t___2752.clone(), t___2753.clone(), t___4673.clone(), false));
                break 'fn__395;
            }
            return__188 = Changeset::new(self.clone());
        }
        return return__188.clone();
    }
    fn parse_bool_sql_part(& self, val__401: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__401 = val__401.to_arc_string();
        let return__189: SqlBoolean;
        let mut t___2728: bool;
        let mut t___2729: bool;
        let mut t___2730: bool;
        let mut t___2732: bool;
        let mut t___2733: bool;
        let mut t___2734: bool;
        'fn__402: {
            if Some(val__401.as_str()) == Some("true") {
                t___2730 = true;
            } else {
                if Some(val__401.as_str()) == Some("1") {
                    t___2729 = true;
                } else {
                    if Some(val__401.as_str()) == Some("yes") {
                        t___2728 = true;
                    } else {
                        t___2728 = Some(val__401.as_str()) == Some("on");
                    }
                    t___2729 = t___2728;
                }
                t___2730 = t___2729;
            }
            if t___2730 {
                return__189 = SqlBoolean::new(true);
                break 'fn__402;
            }
            if Some(val__401.as_str()) == Some("false") {
                t___2734 = true;
            } else {
                if Some(val__401.as_str()) == Some("0") {
                    t___2733 = true;
                } else {
                    if Some(val__401.as_str()) == Some("no") {
                        t___2732 = true;
                    } else {
                        t___2732 = Some(val__401.as_str()) == Some("off");
                    }
                    t___2733 = t___2732;
                }
                t___2734 = t___2733;
            }
            if t___2734 {
                return__189 = SqlBoolean::new(false);
                break 'fn__402;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__189.clone());
    }
    fn value_to_sql_part(& self, fieldDef__404: FieldDef, val__405: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__405 = val__405.to_arc_string();
        let return__190: SqlPart;
        let mut t___2715: i32;
        let mut t___2718: i64;
        let mut t___2721: f64;
        let mut t___2726: temper_std::temporal::Date;
        'fn__406: {
            let ft__407: FieldType = fieldDef__404.field_type();
            if temper_core::is::<StringField>(ft__407.clone()) {
                return__190 = SqlPart::new(SqlString::new(val__405.clone()));
                break 'fn__406;
            }
            if temper_core::is::<IntField>(ft__407.clone()) {
                t___2715 = temper_core::string::to_int( & val__405, None) ? ;
                return__190 = SqlPart::new(SqlInt32::new(t___2715));
                break 'fn__406;
            }
            if temper_core::is::<Int64Field>(ft__407.clone()) {
                t___2718 = temper_core::string::to_int64( & val__405, None) ? ;
                return__190 = SqlPart::new(SqlInt64::new(t___2718));
                break 'fn__406;
            }
            if temper_core::is::<FloatField>(ft__407.clone()) {
                t___2721 = temper_core::string::to_float64( & val__405) ? ;
                return__190 = SqlPart::new(SqlFloat64::new(t___2721));
                break 'fn__406;
            }
            if temper_core::is::<BoolField>(ft__407.clone()) {
                return__190 = SqlPart::new(self.parse_bool_sql_part(val__405.clone()) ? );
                break 'fn__406;
            }
            if temper_core::is::<DateField>(ft__407.clone()) {
                t___2726 = temper_std::temporal::Date::from_iso_string(val__405.clone()) ? ;
                return__190 = SqlPart::new(SqlDate::new(t___2726.clone()));
                break 'fn__406;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__190.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___4615: i32;
        let mut t___4620: std::sync::Arc<String>;
        let mut t___4621: bool;
        let mut t___4626: i32;
        let mut t___4628: std::sync::Arc<String>;
        let mut t___4631: std::sync::Arc<String>;
        let mut t___4646: i32;
        let mut t___2680: bool;
        let mut t___2688: FieldDef;
        let mut t___2692: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__410: i32 = 0;
        'loop___4894: loop {
            t___4615 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__410) < Some(t___4615)) {
                break;
            }
            let f__411: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__410);
            if ! f__411.nullable() {
                t___4620 = f__411.name().sql_value();
                t___4621 = temper_core::MappedTrait::has( & self.0.changes, t___4620.clone());
                t___2680 = ! t___4621;
            } else {
                t___2680 = false;
            }
            if t___2680 {
                return Err(temper_core::Error::new());
            }
            i__410 = i__410.wrapping_add(1);
        }
        let pairs__412: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__412)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__413: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__414: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__415: i32 = 0;
        'loop___4895: loop {
            t___4626 = temper_core::ListedTrait::len( & pairs__412);
            if ! (Some(i__415) < Some(t___4626)) {
                break;
            }
            let pair__416: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__412, i__415);
            t___4628 = pair__416.key();
            t___2688 = self.0.table_def.field(t___4628.clone()) ? ;
            let fd__417: FieldDef = t___2688.clone();
            temper_core::listed::add( & colNames__413, pair__416.key(), None);
            t___4631 = pair__416.value();
            t___2692 = self.value_to_sql_part(fd__417.clone(), t___4631.clone()) ? ;
            temper_core::listed::add( & valParts__414, t___2692.clone(), None);
            i__415 = i__415.wrapping_add(1);
        }
        let b__418: SqlBuilder = SqlBuilder::new();
        b__418.append_safe("INSERT INTO ");
        b__418.append_safe(self.0.table_def.table_name().sql_value());
        b__418.append_safe(" (");
        let mut t___4639: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__413);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__4613(& self, c__419: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__419 = c__419.to_arc_string();
                return c__419.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__4613 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__419: std::sync::Arc<String> | closure_group.fn__4613(c__419))
        };
        b__418.append_safe(temper_core::listed::join( & t___4639, std::sync::Arc::new(", ".to_string()), & ( * fn__4613.clone())));
        b__418.append_safe(") VALUES (");
        b__418.append_part(temper_core::ListedTrait::get( & valParts__414, 0));
        let mut j__420: i32 = 1;
        'loop___4896: loop {
            t___4646 = temper_core::ListedTrait::len( & valParts__414);
            if ! (Some(j__420) < Some(t___4646)) {
                break;
            }
            b__418.append_safe(", ");
            b__418.append_part(temper_core::ListedTrait::get( & valParts__414, j__420));
            j__420 = j__420.wrapping_add(1);
        }
        b__418.append_safe(")");
        return Ok(b__418.accumulated());
    }
    pub fn to_update_sql(& self, id__422: i32) -> temper_core::Result<SqlFragment> {
        let mut t___4601: i32;
        let mut t___4604: std::sync::Arc<String>;
        let mut t___4608: std::sync::Arc<String>;
        let mut t___2662: FieldDef;
        let mut t___2667: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__424: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__424)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__425: SqlBuilder = SqlBuilder::new();
        b__425.append_safe("UPDATE ");
        b__425.append_safe(self.0.table_def.table_name().sql_value());
        b__425.append_safe(" SET ");
        let mut i__426: i32 = 0;
        'loop___4897: loop {
            t___4601 = temper_core::ListedTrait::len( & pairs__424);
            if ! (Some(i__426) < Some(t___4601)) {
                break;
            }
            if Some(i__426) > Some(0) {
                b__425.append_safe(", ");
            }
            let pair__427: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__424, i__426);
            t___4604 = pair__427.key();
            t___2662 = self.0.table_def.field(t___4604.clone()) ? ;
            let fd__428: FieldDef = t___2662.clone();
            b__425.append_safe(pair__427.key());
            b__425.append_safe(" = ");
            t___4608 = pair__427.value();
            t___2667 = self.value_to_sql_part(fd__428.clone(), t___4608.clone()) ? ;
            b__425.append_part(t___2667.clone());
            i__426 = i__426.wrapping_add(1);
        }
        b__425.append_safe(" WHERE id = ");
        b__425.append_int32(id__422);
        return Ok(b__425.accumulated());
    }
    pub fn new(_tableDef__430: TableDef, _params__431: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__432: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__433: impl temper_core::ToList<ChangesetError>, _isValid__434: bool) -> ChangesetImpl {
        let _errors__433 = _errors__433.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__430.clone();
        params = _params__431.clone();
        changes = _changes__432.clone();
        errors = _errors__433.clone();
        is_valid = _isValid__434;
        let selfish = ChangesetImpl(std::sync::Arc::new(ChangesetImplStruct {
                    table_def, params, changes, errors, is_valid
        }));
        return selfish;
    }
}
impl ChangesetTrait for ChangesetImpl {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetEnum::ChangesetImpl(self.clone())
    }
    fn clone_boxed(& self) -> Changeset {
        Changeset::new(self.clone())
    }
    fn table_def(& self) -> TableDef {
        self.table_def()
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        self.changes()
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        self.errors()
    }
    fn is_valid(& self) -> bool {
        self.is_valid()
    }
    fn cast(& self, allowedFields__355: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__355)
    }
    fn validate_required(& self, fields__361: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__361)
    }
    fn validate_length(& self, field__367: SafeIdentifier, min__368: i32, max__369: i32) -> Changeset {
        self.validate_length(field__367, min__368, max__369)
    }
    fn validate_int(& self, field__376: SafeIdentifier) -> Changeset {
        self.validate_int(field__376)
    }
    fn validate_int64(& self, field__382: SafeIdentifier) -> Changeset {
        self.validate_int64(field__382)
    }
    fn validate_float(& self, field__388: SafeIdentifier) -> Changeset {
        self.validate_float(field__388)
    }
    fn validate_bool(& self, field__394: SafeIdentifier) -> Changeset {
        self.validate_bool(field__394)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__422: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__422)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
struct OrderClauseStruct {
    field: SafeIdentifier, ascending: bool
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: SafeIdentifier, pub ascending: bool
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.ascending)
    }
}
impl OrderClause {
    pub fn new(field__532: SafeIdentifier, ascending__533: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__532.clone();
        ascending = ascending__533;
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, ascending
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn ascending(& self) -> bool {
        return self.0.ascending;
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<SqlFragment>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<SqlFragment>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val)
    }
}
impl Query {
    pub fn r#where(& self, condition__541: SqlFragment) -> Query {
        let nb__543: temper_core::ListBuilder<SqlFragment> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__543, condition__541.clone(), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__543), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val);
    }
    pub fn select(& self, fields__545: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__545 = fields__545.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__545.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val);
    }
    pub fn order_by(& self, field__548: SafeIdentifier, ascending__549: bool) -> Query {
        let nb__551: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__551, OrderClause::new(field__548.clone(), ascending__549), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__551), self.0.limit_val, self.0.offset_val);
    }
    pub fn limit(& self, n__553: i32) -> temper_core::Result<Query> {
        if Some(n__553) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__553), self.0.offset_val));
    }
    pub fn offset(& self, n__556: i32) -> temper_core::Result<Query> {
        if Some(n__556) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__556)));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___4185: i32;
        let b__560: SqlBuilder = SqlBuilder::new();
        b__560.append_safe("SELECT ");
        if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
            b__560.append_safe("*");
        } else {
            #[derive(Clone)]
            struct ClosureGroup___4 {}
            impl ClosureGroup___4 {
                fn fn__4170(& self, f__561: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__561.sql_value();
                }
            }
            let closure_group = ClosureGroup___4 {};
            let fn__4170 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__561: SafeIdentifier | closure_group.fn__4170(f__561))
            };
            b__560.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__4170.clone())));
        }
        b__560.append_safe(" FROM ");
        b__560.append_safe(self.0.table_name.sql_value());
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__560.append_safe(" WHERE ");
            b__560.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0));
            let mut i__562: i32 = 1;
            'loop___4906: loop {
                t___4185 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__562) < Some(t___4185)) {
                    break;
                }
                b__560.append_safe(" AND ");
                b__560.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__562));
                i__562 = i__562.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__560.append_safe(" ORDER BY ");
            let mut first__563: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___5 {
                first__563: std::sync::Arc<std::sync::RwLock<bool>>, b__560: SqlBuilder
            }
            impl ClosureGroup___5 {
                fn fn__4169(& self, oc__564: OrderClause) {
                    let mut t___2283: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__563) {
                        self.b__560.append_safe(", ");
                    }
                    {
                        * self.first__563.write().unwrap() = false;
                    }
                    let mut t___4164: std::sync::Arc<String> = oc__564.field().sql_value();
                    self.b__560.append_safe(t___4164.clone());
                    if oc__564.ascending() {
                        t___2283 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___2283 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__560.append_safe(t___2283.clone());
                }
            }
            let closure_group = ClosureGroup___5 {
                first__563: first__563.clone(), b__560: b__560.clone()
            };
            let fn__4169 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__564: OrderClause | closure_group.fn__4169(oc__564))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__4169.clone()));
        }
        let lv__565: Option<i32> = self.0.limit_val;
        if ! lv__565.is_none() {
            let lv___1068: i32 = lv__565.unwrap();
            b__560.append_safe(" LIMIT ");
            b__560.append_int32(lv___1068);
        }
        let ov__566: Option<i32> = self.0.offset_val;
        if ! ov__566.is_none() {
            let ov___1069: i32 = ov__566.unwrap();
            b__560.append_safe(" OFFSET ");
            b__560.append_int32(ov___1069);
        }
        return b__560.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__568: i32) -> temper_core::Result<SqlFragment> {
        let return__212: SqlFragment;
        let mut t___2276: Query;
        if Some(defaultLimit__568) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__212 = self.to_sql();
        } else {
            t___2276 = self.limit(defaultLimit__568) ? ;
            return__212 = t___2276.to_sql();
        }
        return Ok(return__212.clone());
    }
    pub fn new(tableName__571: SafeIdentifier, conditions__572: impl temper_core::ToList<SqlFragment>, selectedFields__573: impl temper_core::ToList<SafeIdentifier>, orderClauses__574: impl temper_core::ToList<OrderClause>, limitVal__575: Option<i32>, offsetVal__576: Option<i32>) -> Query {
        let conditions__572 = conditions__572.to_list();
        let selectedFields__573 = selectedFields__573.to_list();
        let orderClauses__574 = orderClauses__574.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        table_name = tableName__571.clone();
        conditions = conditions__572.clone();
        selected_fields = selectedFields__573.clone();
        order_clauses = orderClauses__574.clone();
        limit_val = limitVal__575;
        offset_val = offsetVal__576;
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<SqlFragment> {
        return self.0.conditions.clone();
    }
    pub fn selected_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.selected_fields.clone();
    }
    pub fn order_clauses(& self) -> temper_core::List<OrderClause> {
        return self.0.order_clauses.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
    pub fn offset_val(& self) -> Option<i32> {
        return self.0.offset_val;
    }
}
temper_core::impl_any_value_trait!(Query, []);
pub enum SafeIdentifierEnum {
    ValidatedIdentifier(ValidatedIdentifier)
}
pub trait SafeIdentifierTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SafeIdentifierEnum;
    fn clone_boxed(& self) -> SafeIdentifier;
    fn sql_value(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct SafeIdentifier(std::sync::Arc<dyn SafeIdentifierTrait>);
impl SafeIdentifier {
    pub fn new(selfish: impl SafeIdentifierTrait + 'static) -> SafeIdentifier {
        SafeIdentifier(std::sync::Arc::new(selfish))
    }
}
impl SafeIdentifierTrait for SafeIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifierTrait::clone_boxed( & ( * self.0))
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        SafeIdentifierTrait::sql_value( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(SafeIdentifier);
impl std::ops::Deref for SafeIdentifier {
    type Target = dyn SafeIdentifierTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ValidatedIdentifierStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub (crate) struct ValidatedIdentifier(std::sync::Arc<ValidatedIdentifierStruct>);
impl ValidatedIdentifier {
    pub fn sql_value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn new(_value__625: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__625 = _value__625.to_arc_string();
        let value;
        value = _value__625.clone();
        let selfish = ValidatedIdentifier(std::sync::Arc::new(ValidatedIdentifierStruct {
                    value
        }));
        return selfish;
    }
}
impl SafeIdentifierTrait for ValidatedIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierEnum::ValidatedIdentifier(self.clone())
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifier::new(self.clone())
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        self.sql_value()
    }
}
temper_core::impl_any_value_trait!(ValidatedIdentifier, [SafeIdentifier]);
pub enum FieldTypeEnum {
    StringField(StringField), IntField(IntField), Int64Field(Int64Field), FloatField(FloatField), BoolField(BoolField), DateField(DateField)
}
pub trait FieldTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> FieldTypeEnum;
    fn clone_boxed(& self) -> FieldType;
}
#[derive(Clone)]
pub struct FieldType(std::sync::Arc<dyn FieldTypeTrait>);
impl FieldType {
    pub fn new(selfish: impl FieldTypeTrait + 'static) -> FieldType {
        FieldType(std::sync::Arc::new(selfish))
    }
}
impl FieldTypeTrait for FieldType {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> FieldType {
        FieldTypeTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(FieldType);
impl std::ops::Deref for FieldType {
    type Target = dyn FieldTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct StringFieldStruct {}
#[derive(Clone)]
pub struct StringField(std::sync::Arc<StringFieldStruct>);
impl StringField {
    pub fn new() -> StringField {
        let selfish = StringField(std::sync::Arc::new(StringFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for StringField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::StringField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(StringField, [FieldType]);
struct IntFieldStruct {}
#[derive(Clone)]
pub struct IntField(std::sync::Arc<IntFieldStruct>);
impl IntField {
    pub fn new() -> IntField {
        let selfish = IntField(std::sync::Arc::new(IntFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for IntField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::IntField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(IntField, [FieldType]);
struct Int64FieldStruct {}
#[derive(Clone)]
pub struct Int64Field(std::sync::Arc<Int64FieldStruct>);
impl Int64Field {
    pub fn new() -> Int64Field {
        let selfish = Int64Field(std::sync::Arc::new(Int64FieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for Int64Field {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::Int64Field(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Int64Field, [FieldType]);
struct FloatFieldStruct {}
#[derive(Clone)]
pub struct FloatField(std::sync::Arc<FloatFieldStruct>);
impl FloatField {
    pub fn new() -> FloatField {
        let selfish = FloatField(std::sync::Arc::new(FloatFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for FloatField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::FloatField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(FloatField, [FieldType]);
struct BoolFieldStruct {}
#[derive(Clone)]
pub struct BoolField(std::sync::Arc<BoolFieldStruct>);
impl BoolField {
    pub fn new() -> BoolField {
        let selfish = BoolField(std::sync::Arc::new(BoolFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for BoolField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::BoolField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(BoolField, [FieldType]);
struct DateFieldStruct {}
#[derive(Clone)]
pub struct DateField(std::sync::Arc<DateFieldStruct>);
impl DateField {
    pub fn new() -> DateField {
        let selfish = DateField(std::sync::Arc::new(DateFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for DateField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::DateField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(DateField, [FieldType]);
struct FieldDefStruct {
    name: SafeIdentifier, field_type: FieldType, nullable: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable)
    }
}
impl FieldDef {
    pub fn new(name__643: SafeIdentifier, fieldType__644: FieldType, nullable__645: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__643.clone();
        field_type = fieldType__644.clone();
        nullable = nullable__645;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable
        }));
        return selfish;
    }
    pub fn name(& self) -> SafeIdentifier {
        return self.0.name.clone();
    }
    pub fn field_type(& self) -> FieldType {
        return self.0.field_type.clone();
    }
    pub fn nullable(& self) -> bool {
        return self.0.nullable;
    }
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields)
    }
}
impl TableDef {
    pub fn field(& self, name__649: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__649 = name__649.to_arc_string();
        let return__241: FieldDef;
        'fn__650: {
            let this__2984: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__2985: i32 = temper_core::ListedTrait::len( & this__2984);
            let mut i__2986: i32 = 0;
            'loop___4913: while Some(i__2986) < Some(n__2985) {
                let el__2987: FieldDef = temper_core::ListedTrait::get( & this__2984, i__2986);
                i__2986 = i__2986.wrapping_add(1);
                let f__651: FieldDef = el__2987.clone();
                if Some(f__651.name().sql_value().as_str()) == Some(name__649.as_str()) {
                    return__241 = f__651.clone();
                    break 'fn__650;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__241.clone());
    }
    pub fn new(tableName__653: SafeIdentifier, fields__654: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__654 = fields__654.to_list();
        let table_name;
        let fields;
        table_name = tableName__653.clone();
        fields = fields__654.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__676: impl temper_core::ToArcString) {
        let sqlSource__676 = sqlSource__676.to_arc_string();
        let mut t___4789: SqlSource = SqlSource::new(sqlSource__676.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4789.clone()), None);
    }
    pub fn append_fragment(& self, fragment__679: SqlFragment) {
        let mut t___4787: temper_core::List<SqlPart> = fragment__679.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___4787.clone()), None);
    }
    pub fn append_part(& self, part__682: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__682.clone(), None);
    }
    pub fn append_part_list(& self, values__685: impl temper_core::ToList<SqlPart>) {
        let values__685 = values__685.to_list();
        #[derive(Clone)]
        struct ClosureGroup___6 {
            this__121: SqlBuilder
        }
        impl ClosureGroup___6 {
            fn fn__4783(& self, x__687: SqlPart) {
                self.this__121.append_part(x__687.clone());
            }
        }
        let closure_group = ClosureGroup___6 {
            this__121: self.clone()
        };
        let fn__4783 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__687: SqlPart | closure_group.fn__4783(x__687))
        };
        self.append_list(temper_core::ToListed::to_listed(values__685.clone()), fn__4783.clone());
    }
    pub fn append_boolean(& self, value__689: bool) {
        let mut t___4780: SqlBoolean = SqlBoolean::new(value__689);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4780.clone()), None);
    }
    pub fn append_boolean_list(& self, values__692: impl temper_core::ToListed<bool>) {
        let values__692 = values__692.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___7 {
            this__123: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__4777(& self, x__694: bool) {
                self.this__123.append_boolean(x__694);
            }
        }
        let closure_group = ClosureGroup___7 {
            this__123: self.clone()
        };
        let fn__4777 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__694: bool | closure_group.fn__4777(x__694))
        };
        self.append_list(values__692.clone(), fn__4777.clone());
    }
    pub fn append_date(& self, value__696: temper_std::temporal::Date) {
        let mut t___4774: SqlDate = SqlDate::new(value__696.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4774.clone()), None);
    }
    pub fn append_date_list(& self, values__699: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__699 = values__699.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            this__125: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__4771(& self, x__701: temper_std::temporal::Date) {
                self.this__125.append_date(x__701.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            this__125: self.clone()
        };
        let fn__4771 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__701: temper_std::temporal::Date | closure_group.fn__4771(x__701))
        };
        self.append_list(values__699.clone(), fn__4771.clone());
    }
    pub fn append_float64(& self, value__703: f64) {
        let mut t___4768: SqlFloat64 = SqlFloat64::new(value__703);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4768.clone()), None);
    }
    pub fn append_float64_list(& self, values__706: impl temper_core::ToListed<f64>) {
        let values__706 = values__706.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__127: SqlBuilder
        }
        impl ClosureGroup___9 {
            fn fn__4765(& self, x__708: f64) {
                self.this__127.append_float64(x__708);
            }
        }
        let closure_group = ClosureGroup___9 {
            this__127: self.clone()
        };
        let fn__4765 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__708: f64 | closure_group.fn__4765(x__708))
        };
        self.append_list(values__706.clone(), fn__4765.clone());
    }
    pub fn append_int32(& self, value__710: i32) {
        let mut t___4762: SqlInt32 = SqlInt32::new(value__710);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4762.clone()), None);
    }
    pub fn append_int32_list(& self, values__713: impl temper_core::ToListed<i32>) {
        let values__713 = values__713.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__129: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__4759(& self, x__715: i32) {
                self.this__129.append_int32(x__715);
            }
        }
        let closure_group = ClosureGroup___10 {
            this__129: self.clone()
        };
        let fn__4759 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__715: i32 | closure_group.fn__4759(x__715))
        };
        self.append_list(values__713.clone(), fn__4759.clone());
    }
    pub fn append_int64(& self, value__717: i64) {
        let mut t___4756: SqlInt64 = SqlInt64::new(value__717);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4756.clone()), None);
    }
    pub fn append_int64_list(& self, values__720: impl temper_core::ToListed<i64>) {
        let values__720 = values__720.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__131: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__4753(& self, x__722: i64) {
                self.this__131.append_int64(x__722);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__131: self.clone()
        };
        let fn__4753 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__722: i64 | closure_group.fn__4753(x__722))
        };
        self.append_list(values__720.clone(), fn__4753.clone());
    }
    pub fn append_string(& self, value__724: impl temper_core::ToArcString) {
        let value__724 = value__724.to_arc_string();
        let mut t___4750: SqlString = SqlString::new(value__724.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___4750.clone()), None);
    }
    pub fn append_string_list(& self, values__727: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__727 = values__727.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__133: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__4747(& self, x__729: impl temper_core::ToArcString) {
                let x__729 = x__729.to_arc_string();
                self.this__133.append_string(x__729.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__133: self.clone()
        };
        let fn__4747 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__729: std::sync::Arc<String> | closure_group.fn__4747(x__729))
        };
        self.append_list(values__727.clone(), fn__4747.clone());
    }
    fn append_list<T>(& self, values__731: impl temper_core::ToListed<T>, appendValue__732: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__731 = values__731.to_listed();
        let mut t___4742: i32;
        let mut t___4744: T;
        let mut i__734: i32 = 0;
        'loop___4914: loop {
            t___4742 = temper_core::ListedTrait::len( & ( * values__731));
            if ! (Some(i__734) < Some(t___4742)) {
                break;
            }
            if Some(i__734) > Some(0) {
                self.append_safe(", ");
            }
            t___4744 = temper_core::ListedTrait::get( & ( * values__731), i__734);
            appendValue__732(t___4744.clone());
            i__734 = i__734.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___4739: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___4739.clone();
        let selfish = SqlBuilder(std::sync::Arc::new(SqlBuilderStruct {
                    buffer
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(SqlBuilder, []);
struct SqlFragmentStruct {
    parts: temper_core::List<SqlPart>
}
#[derive(Clone)]
pub struct SqlFragment(std::sync::Arc<SqlFragmentStruct>);
impl SqlFragment {
    pub fn to_source(& self) -> SqlSource {
        return SqlSource::new(self.to_string());
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let mut t___4809: i32;
        let builder__746: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__747: i32 = 0;
        'loop___4915: loop {
            t___4809 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__747) < Some(t___4809)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__747).format_to(builder__746.clone());
            i__747 = i__747.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__746);
    }
    pub fn new(parts__749: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__749 = parts__749.to_list();
        let parts;
        parts = parts__749.clone();
        let selfish = SqlFragment(std::sync::Arc::new(SqlFragmentStruct {
                    parts
        }));
        return selfish;
    }
    pub fn parts(& self) -> temper_core::List<SqlPart> {
        return self.0.parts.clone();
    }
}
temper_core::impl_any_value_trait!(SqlFragment, []);
pub enum SqlPartEnum {
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__751: std::sync::Arc<std::sync::RwLock<String>>);
}
#[derive(Clone)]
pub struct SqlPart(std::sync::Arc<dyn SqlPartTrait>);
impl SqlPart {
    pub fn new(selfish: impl SqlPartTrait + 'static) -> SqlPart {
        SqlPart(std::sync::Arc::new(selfish))
    }
}
impl SqlPartTrait for SqlPart {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPartTrait::clone_boxed( & ( * self.0))
    }
    fn format_to(& self, arg1: std::sync::Arc<std::sync::RwLock<String>>) -> () {
        SqlPartTrait::format_to( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(SqlPart);
impl std::ops::Deref for SqlPart {
    type Target = dyn SqlPartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SqlSourceStruct {
    source: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlSource(std::sync::Arc<SqlSourceStruct>);
impl SqlSource {
    pub fn format_to(& self, builder__755: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__755, self.0.source.clone());
    }
    pub fn new(source__758: impl temper_core::ToArcString) -> SqlSource {
        let source__758 = source__758.to_arc_string();
        let source;
        source = source__758.clone();
        let selfish = SqlSource(std::sync::Arc::new(SqlSourceStruct {
                    source
        }));
        return selfish;
    }
    pub fn source(& self) -> std::sync::Arc<String> {
        return self.0.source.clone();
    }
}
impl SqlPartTrait for SqlSource {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlSource(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__755: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__755)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__761: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___2868: std::sync::Arc<String>;
        if self.0.value {
            t___2868 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___2868 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__761, t___2868.clone());
    }
    pub fn new(value__764: bool) -> SqlBoolean {
        let value;
        value = value__764;
        let selfish = SqlBoolean(std::sync::Arc::new(SqlBooleanStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> bool {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlBoolean {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlBoolean(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__761: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__761)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__767: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__767, "'");
        let mut t___4792: std::sync::Arc<String> = self.0.value.to_string();
        temper_core::string::builder::append( & builder__767, t___4792.clone());
        temper_core::string::builder::append( & builder__767, "'");
    }
    pub fn new(value__770: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__770.clone();
        let selfish = SqlDate(std::sync::Arc::new(SqlDateStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> temper_std::temporal::Date {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlDate {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDate(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__767: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__767)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__773: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___4795: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        temper_core::string::builder::append( & builder__773, t___4795.clone());
    }
    pub fn new(value__776: f64) -> SqlFloat64 {
        let value;
        value = value__776;
        let selfish = SqlFloat64(std::sync::Arc::new(SqlFloat64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> f64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlFloat64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlFloat64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__773: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__773)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__779: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___4799: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__779, t___4799.clone());
    }
    pub fn new(value__782: i32) -> SqlInt32 {
        let value;
        value = value__782;
        let selfish = SqlInt32(std::sync::Arc::new(SqlInt32Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt32 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt32(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__779: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__779)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__785: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___4797: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__785, t___4797.clone());
    }
    pub fn new(value__788: i64) -> SqlInt64 {
        let value;
        value = value__788;
        let selfish = SqlInt64(std::sync::Arc::new(SqlInt64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__785: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__785)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__791: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__791, "'");
        #[derive(Clone)]
        struct ClosureGroup___13 {
            builder__791: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___13 {
            fn fn__4802(& self, c__793: i32) {
                if Some(c__793) == Some(39) {
                    temper_core::string::builder::append( & self.builder__791, "''");
                } else {
                    'ok___4850: {
                        'orelse___1024: {
                            match temper_core::string::builder::append_code_point( & self.builder__791, c__793) {
                                Ok(x) => x,
                                _ => break 'orelse___1024
                            };
                            break 'ok___4850;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___13 {
            builder__791: builder__791.clone()
        };
        let fn__4802 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__793: i32 | closure_group.fn__4802(c__793))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__4802.clone()));
        temper_core::string::builder::append( & builder__791, "'");
    }
    pub fn new(value__795: impl temper_core::ToArcString) -> SqlString {
        let value__795 = value__795.to_arc_string();
        let value;
        value = value__795.clone();
        let selfish = SqlString(std::sync::Arc::new(SqlStringStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlString {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlString(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__791: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__791)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__435: TableDef, params__436: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___4591: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__435.clone(), params__436.clone(), t___4591.clone(), [], true));
}
fn isIdentStart__296(c__626: i32) -> bool {
    let return__221: bool;
    let mut t___2636: bool;
    let mut t___2637: bool;
    if Some(c__626) >= Some(97) {
        t___2636 = Some(c__626) <= Some(122);
    } else {
        t___2636 = false;
    }
    if t___2636 {
        return__221 = true;
    } else {
        if Some(c__626) >= Some(65) {
            t___2637 = Some(c__626) <= Some(90);
        } else {
            t___2637 = false;
        }
        if t___2637 {
            return__221 = true;
        } else {
            return__221 = Some(c__626) == Some(95);
        }
    }
    return return__221;
}
fn isIdentPart__297(c__628: i32) -> bool {
    let return__222: bool;
    if isIdentStart__296(c__628) {
        return__222 = true;
    } else {
        if Some(c__628) >= Some(48) {
            return__222 = Some(c__628) <= Some(57);
        } else {
            return__222 = false;
        }
    }
    return return__222;
}
pub fn safe_identifier(name__630: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__630 = name__630.to_arc_string();
    let mut t___4589: usize;
    if name__630.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__632: usize = 0usize;
    if ! isIdentStart__296(temper_core::string::get( & name__630, idx__632)) {
        return Err(temper_core::Error::new());
    }
    let mut t___4586: usize = temper_core::string::next( & name__630, idx__632);
    idx__632 = t___4586;
    'loop___4916: loop {
        if ! temper_core::string::has_index( & name__630, idx__632) {
            break;
        }
        if ! isIdentPart__297(temper_core::string::get( & name__630, idx__632)) {
            return Err(temper_core::Error::new());
        }
        t___4589 = temper_core::string::next( & name__630, idx__632);
        idx__632 = t___4589;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__630.clone())));
}
fn csid__293(name__438: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__438 = name__438.to_arc_string();
    let return__194: SafeIdentifier;
    let mut t___2624: SafeIdentifier;
    'ok___4857: {
        'orelse___1028: {
            t___2624 = match safe_identifier(name__438.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1028
            };
            return__194 = t___2624.clone();
            break 'ok___4857;
        }
        return__194 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__194.clone();
}
fn userTable__294() -> TableDef {
    return TableDef::new(csid__293("users"), [FieldDef::new(csid__293("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__293("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__293("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__293("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__293("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__525: TableDef, id__526: i32) -> SqlFragment {
    let b__528: SqlBuilder = SqlBuilder::new();
    b__528.append_safe("DELETE FROM ");
    b__528.append_safe(tableDef__525.table_name().sql_value());
    b__528.append_safe(" WHERE id = ");
    b__528.append_int32(id__526);
    return b__528.accumulated();
}
pub fn from(tableName__577: SafeIdentifier) -> Query {
    return Query::new(tableName__577.clone(), [], [], [], None, None);
}
fn sid__295(name__579: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__579 = name__579.to_arc_string();
    let return__214: SafeIdentifier;
    let mut t___2261: SafeIdentifier;
    'ok___4868: {
        'orelse___1036: {
            t___2261 = match safe_identifier(name__579.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1036
            };
            return__214 = t___2261.clone();
            break 'ok___4868;
        }
        return__214 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__214.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__888() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___20 = temper_std::testing::Test::new();
        let params__442: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___4547: TableDef = userTable__294();
        let mut t___4548: SafeIdentifier = csid__293("name");
        let mut t___4549: SafeIdentifier = csid__293("email");
        let cs__443: Changeset = changeset(t___4547.clone(), params__442.clone()).cast(std::sync::Arc::new(vec![t___4548.clone(), t___4549.clone()]));
        let mut t___4552: bool = temper_core::MappedTrait::has( & cs__443.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___14 {}
        impl ClosureGroup___14 {
            fn fn__4542(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___14 {};
        let fn__4542 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4542())
        };
        test___20.assert(t___4552, fn__4542.clone());
        let mut t___4556: bool = temper_core::MappedTrait::has( & cs__443.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___15 {}
        impl ClosureGroup___15 {
            fn fn__4541(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___15 {};
        let fn__4541 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4541())
        };
        test___20.assert(t___4556, fn__4541.clone());
        let mut t___4562: bool = ! temper_core::MappedTrait::has( & cs__443.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__4540(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__4540 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4540())
        };
        test___20.assert(t___4562, fn__4540.clone());
        let mut t___4564: bool = cs__443.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__4539(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__4539 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4539())
        };
        test___20.assert(t___4564, fn__4539.clone());
        test___20.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__889() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___21 = temper_std::testing::Test::new();
        let params__445: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___4525: TableDef = userTable__294();
        let mut t___4526: SafeIdentifier = csid__293("name");
        let cs__446: Changeset = changeset(t___4525.clone(), params__445.clone()).cast(std::sync::Arc::new(vec![t___4526.clone()])).cast(std::sync::Arc::new(vec![csid__293("email")]));
        let mut t___4533: bool = ! temper_core::MappedTrait::has( & cs__446.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__4521(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__4521 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4521())
        };
        test___21.assert(t___4533, fn__4521.clone());
        let mut t___4536: bool = temper_core::MappedTrait::has( & cs__446.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__4520(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__4520 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4520())
        };
        test___21.assert(t___4536, fn__4520.clone());
        test___21.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__890() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let params__448: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___4507: TableDef = userTable__294();
        let mut t___4508: SafeIdentifier = csid__293("name");
        let mut t___4509: SafeIdentifier = csid__293("email");
        let cs__449: Changeset = changeset(t___4507.clone(), params__448.clone()).cast(std::sync::Arc::new(vec![t___4508.clone(), t___4509.clone()]));
        let mut t___4514: bool = ! temper_core::MappedTrait::has( & cs__449.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__4503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__4503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4503())
        };
        test___22.assert(t___4514, fn__4503.clone());
        let mut t___4517: bool = temper_core::MappedTrait::has( & cs__449.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__4502(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__4502 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4502())
        };
        test___22.assert(t___4517, fn__4502.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__891() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let params__451: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___4489: TableDef = userTable__294();
        let mut t___4490: SafeIdentifier = csid__293("name");
        let cs__452: Changeset = changeset(t___4489.clone(), params__451.clone()).cast(std::sync::Arc::new(vec![t___4490.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name")]));
        let mut t___4494: bool = cs__452.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__4486(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__4486 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4486())
        };
        test___23.assert(t___4494, fn__4486.clone());
        let mut t___4500: bool = Some(temper_core::ListedTrait::len( & cs__452.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__4485(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__4485 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4485())
        };
        test___23.assert(t___4500, fn__4485.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__892() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__454: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4465: TableDef = userTable__294();
        let mut t___4466: SafeIdentifier = csid__293("name");
        let cs__455: Changeset = changeset(t___4465.clone(), params__454.clone()).cast(std::sync::Arc::new(vec![t___4466.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name")]));
        let mut t___4472: bool = ! cs__455.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__4463(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__4463 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4463())
        };
        test___24.assert(t___4472, fn__4463.clone());
        let mut t___4477: bool = Some(temper_core::ListedTrait::len( & cs__455.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__4462(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__4462 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4462())
        };
        test___24.assert(t___4477, fn__4462.clone());
        let mut t___4483: bool = Some(temper_core::ListedTrait::get( & cs__455.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__4461(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__4461 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4461())
        };
        test___24.assert(t___4483, fn__4461.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__893() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__457: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___4453: TableDef = userTable__294();
        let mut t___4454: SafeIdentifier = csid__293("name");
        let cs__458: Changeset = changeset(t___4453.clone(), params__457.clone()).cast(std::sync::Arc::new(vec![t___4454.clone()])).validate_length(csid__293("name"), 2, 50);
        let mut t___4458: bool = cs__458.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__4450(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__4450 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4450())
        };
        test___25.assert(t___4458, fn__4450.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__894() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__460: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___4441: TableDef = userTable__294();
        let mut t___4442: SafeIdentifier = csid__293("name");
        let cs__461: Changeset = changeset(t___4441.clone(), params__460.clone()).cast(std::sync::Arc::new(vec![t___4442.clone()])).validate_length(csid__293("name"), 2, 50);
        let mut t___4448: bool = ! cs__461.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__4438(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__4438 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4438())
        };
        test___26.assert(t___4448, fn__4438.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__895() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__463: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___4429: TableDef = userTable__294();
        let mut t___4430: SafeIdentifier = csid__293("name");
        let cs__464: Changeset = changeset(t___4429.clone(), params__463.clone()).cast(std::sync::Arc::new(vec![t___4430.clone()])).validate_length(csid__293("name"), 2, 10);
        let mut t___4436: bool = ! cs__464.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__4426(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__4426 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4426())
        };
        test___27.assert(t___4436, fn__4426.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__896() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__466: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___4418: TableDef = userTable__294();
        let mut t___4419: SafeIdentifier = csid__293("age");
        let cs__467: Changeset = changeset(t___4418.clone(), params__466.clone()).cast(std::sync::Arc::new(vec![t___4419.clone()])).validate_int(csid__293("age"));
        let mut t___4423: bool = cs__467.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__4415(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__4415 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4415())
        };
        test___28.assert(t___4423, fn__4415.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__897() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__469: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___4406: TableDef = userTable__294();
        let mut t___4407: SafeIdentifier = csid__293("age");
        let cs__470: Changeset = changeset(t___4406.clone(), params__469.clone()).cast(std::sync::Arc::new(vec![t___4407.clone()])).validate_int(csid__293("age"));
        let mut t___4413: bool = ! cs__470.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__4403(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__4403 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4403())
        };
        test___29.assert(t___4413, fn__4403.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__898() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__472: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___4395: TableDef = userTable__294();
        let mut t___4396: SafeIdentifier = csid__293("score");
        let cs__473: Changeset = changeset(t___4395.clone(), params__472.clone()).cast(std::sync::Arc::new(vec![t___4396.clone()])).validate_float(csid__293("score"));
        let mut t___4400: bool = cs__473.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__4392(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__4392 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4392())
        };
        test___30.assert(t___4400, fn__4392.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__899() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__475: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___4384: TableDef = userTable__294();
        let mut t___4385: SafeIdentifier = csid__293("age");
        let cs__476: Changeset = changeset(t___4384.clone(), params__475.clone()).cast(std::sync::Arc::new(vec![t___4385.clone()])).validate_int64(csid__293("age"));
        let mut t___4389: bool = cs__476.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__4381(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__4381 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4381())
        };
        test___31.assert(t___4389, fn__4381.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__900() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__478: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___4372: TableDef = userTable__294();
        let mut t___4373: SafeIdentifier = csid__293("age");
        let cs__479: Changeset = changeset(t___4372.clone(), params__478.clone()).cast(std::sync::Arc::new(vec![t___4373.clone()])).validate_int64(csid__293("age"));
        let mut t___4379: bool = ! cs__479.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__4369(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__4369 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4369())
        };
        test___32.assert(t___4379, fn__4369.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__901() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___35 {
            test___33: temper_std::testing::Test
        }
        impl ClosureGroup___35 {
            fn fn__4366(& self, v__481: impl temper_core::ToArcString) {
                let v__481 = v__481.to_arc_string();
                let params__482: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__481.clone())]);
                let mut t___4358: TableDef = userTable__294();
                let mut t___4359: SafeIdentifier = csid__293("active");
                let cs__483: Changeset = changeset(t___4358.clone(), params__482.clone()).cast(std::sync::Arc::new(vec![t___4359.clone()])).validate_bool(csid__293("active"));
                let mut t___4363: bool = cs__483.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___36 {
                    v__481: std::sync::Arc<String>
                }
                impl ClosureGroup___36 {
                    fn fn__4355(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__481.clone()));
                    }
                }
                let closure_group = ClosureGroup___36 {
                    v__481: v__481.clone()
                };
                let fn__4355 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4355())
                };
                self.test___33.assert(t___4363, fn__4355.clone());
            }
        }
        let closure_group = ClosureGroup___35 {
            test___33: test___33.clone()
        };
        let fn__4366 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__481: std::sync::Arc<String> | closure_group.fn__4366(v__481))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__4366.clone()));
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__902() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___37 {
            test___34: temper_std::testing::Test
        }
        impl ClosureGroup___37 {
            fn fn__4352(& self, v__485: impl temper_core::ToArcString) {
                let v__485 = v__485.to_arc_string();
                let params__486: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__485.clone())]);
                let mut t___4344: TableDef = userTable__294();
                let mut t___4345: SafeIdentifier = csid__293("active");
                let cs__487: Changeset = changeset(t___4344.clone(), params__486.clone()).cast(std::sync::Arc::new(vec![t___4345.clone()])).validate_bool(csid__293("active"));
                let mut t___4349: bool = cs__487.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___38 {
                    v__485: std::sync::Arc<String>
                }
                impl ClosureGroup___38 {
                    fn fn__4341(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__485.clone()));
                    }
                }
                let closure_group = ClosureGroup___38 {
                    v__485: v__485.clone()
                };
                let fn__4341 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4341())
                };
                self.test___34.assert(t___4349, fn__4341.clone());
            }
        }
        let closure_group = ClosureGroup___37 {
            test___34: test___34.clone()
        };
        let fn__4352 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__485: std::sync::Arc<String> | closure_group.fn__4352(v__485))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__4352.clone()));
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__903() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___39 {
            test___35: temper_std::testing::Test
        }
        impl ClosureGroup___39 {
            fn fn__4338(& self, v__489: impl temper_core::ToArcString) {
                let v__489 = v__489.to_arc_string();
                let params__490: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__489.clone())]);
                let mut t___4329: TableDef = userTable__294();
                let mut t___4330: SafeIdentifier = csid__293("active");
                let cs__491: Changeset = changeset(t___4329.clone(), params__490.clone()).cast(std::sync::Arc::new(vec![t___4330.clone()])).validate_bool(csid__293("active"));
                let mut t___4336: bool = ! cs__491.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___40 {
                    v__489: std::sync::Arc<String>
                }
                impl ClosureGroup___40 {
                    fn fn__4326(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__489.clone()));
                    }
                }
                let closure_group = ClosureGroup___40 {
                    v__489: v__489.clone()
                };
                let fn__4326 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4326())
                };
                self.test___35.assert(t___4336, fn__4326.clone());
            }
        }
        let closure_group = ClosureGroup___39 {
            test___35: test___35.clone()
        };
        let fn__4338 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__489: std::sync::Arc<String> | closure_group.fn__4338(v__489))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__4338.clone()));
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__904() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let mut t___2425: SqlFragment;
        let params__493: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___4314: TableDef = userTable__294();
        let mut t___4315: SafeIdentifier = csid__293("name");
        let mut t___4316: SafeIdentifier = csid__293("email");
        let cs__494: Changeset = changeset(t___4314.clone(), params__493.clone()).cast(std::sync::Arc::new(vec![t___4315.clone(), t___4316.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name"), csid__293("email")]));
        let sqlFrag__495: SqlFragment;
        'ok___4859: {
            'orelse___1029: {
                t___2425 = match cs__494.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1029
                };
                sqlFrag__495 = t___2425.clone();
                break 'ok___4859;
            }
            sqlFrag__495 = panic!();
        }
        let s__496: std::sync::Arc<String> = sqlFrag__495.to_string();
        let mut t___4323: bool = temper_core::string::index_of( & s__496, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___41 {
            s__496: std::sync::Arc<String>
        }
        impl ClosureGroup___41 {
            fn fn__4310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__496.clone()));
            }
        }
        let closure_group = ClosureGroup___41 {
            s__496: s__496.clone()
        };
        let fn__4310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4310())
        };
        test___36.assert(t___4323, fn__4310.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__905() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        let mut t___2404: SqlFragment;
        let params__498: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___4294: TableDef = userTable__294();
        let mut t___4295: SafeIdentifier = csid__293("name");
        let mut t___4296: SafeIdentifier = csid__293("email");
        let cs__499: Changeset = changeset(t___4294.clone(), params__498.clone()).cast(std::sync::Arc::new(vec![t___4295.clone(), t___4296.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name"), csid__293("email")]));
        let sqlFrag__500: SqlFragment;
        'ok___4862: {
            'orelse___1030: {
                t___2404 = match cs__499.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1030
                };
                sqlFrag__500 = t___2404.clone();
                break 'ok___4862;
            }
            sqlFrag__500 = panic!();
        }
        let s__501: std::sync::Arc<String> = sqlFrag__500.to_string();
        let mut t___4303: bool = temper_core::string::index_of( & s__501, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            s__501: std::sync::Arc<String>
        }
        impl ClosureGroup___42 {
            fn fn__4290(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__501.clone()));
            }
        }
        let closure_group = ClosureGroup___42 {
            s__501: s__501.clone()
        };
        let fn__4290 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4290())
        };
        test___37.assert(t___4303, fn__4290.clone());
        let mut t___4307: bool = temper_core::string::index_of( & s__501, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___43 {
            s__501: std::sync::Arc<String>
        }
        impl ClosureGroup___43 {
            fn fn__4289(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__501.clone()));
            }
        }
        let closure_group = ClosureGroup___43 {
            s__501: s__501.clone()
        };
        let fn__4289 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4289())
        };
        test___37.assert(t___4307, fn__4289.clone());
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__906() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let mut t___2387: SqlFragment;
        let params__503: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___4276: TableDef = userTable__294();
        let mut t___4277: SafeIdentifier = csid__293("name");
        let mut t___4278: SafeIdentifier = csid__293("email");
        let mut t___4279: SafeIdentifier = csid__293("age");
        let cs__504: Changeset = changeset(t___4276.clone(), params__503.clone()).cast(std::sync::Arc::new(vec![t___4277.clone(), t___4278.clone(), t___4279.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name"), csid__293("email")]));
        let sqlFrag__505: SqlFragment;
        'ok___4863: {
            'orelse___1031: {
                t___2387 = match cs__504.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1031
                };
                sqlFrag__505 = t___2387.clone();
                break 'ok___4863;
            }
            sqlFrag__505 = panic!();
        }
        let s__506: std::sync::Arc<String> = sqlFrag__505.to_string();
        let mut t___4286: bool = temper_core::string::index_of( & s__506, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            s__506: std::sync::Arc<String>
        }
        impl ClosureGroup___44 {
            fn fn__4271(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__506.clone()));
            }
        }
        let closure_group = ClosureGroup___44 {
            s__506: s__506.clone()
        };
        let fn__4271 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4271())
        };
        test___38.assert(t___4286, fn__4271.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__907() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let params__508: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4264: TableDef = userTable__294();
        let mut t___4265: SafeIdentifier = csid__293("name");
        let cs__509: Changeset = changeset(t___4264.clone(), params__508.clone()).cast(std::sync::Arc::new(vec![t___4265.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name")]));
        let didBubble__510: bool;
        'ok___4864: {
            'orelse___1032: {
                match cs__509.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1032
                };
                didBubble__510 = false;
                break 'ok___4864;
            }
            didBubble__510 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___45 {}
        impl ClosureGroup___45 {
            fn fn__4262(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___45 {};
        let fn__4262 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4262())
        };
        test___39.assert(didBubble__510, fn__4262.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__908() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let strictTable__512: TableDef = TableDef::new(csid__293("posts"), [FieldDef::new(csid__293("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__293("body"), FieldType::new(StringField::new()), true)]);
        let params__513: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___4255: SafeIdentifier = csid__293("body");
        let cs__514: Changeset = changeset(strictTable__512.clone(), params__513.clone()).cast(std::sync::Arc::new(vec![t___4255.clone()]));
        let mut t___4257: bool = cs__514.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___46 {}
        impl ClosureGroup___46 {
            fn fn__4244(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___46 {};
        let fn__4244 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4244())
        };
        test___40.assert(t___4257, fn__4244.clone());
        let didBubble__515: bool;
        'ok___4865: {
            'orelse___1033: {
                match cs__514.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1033
                };
                didBubble__515 = false;
                break 'ok___4865;
            }
            didBubble__515 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___47 {}
        impl ClosureGroup___47 {
            fn fn__4243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___47 {};
        let fn__4243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4243())
        };
        test___40.assert(didBubble__515, fn__4243.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__909() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let mut t___2347: SqlFragment;
        let params__517: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___4234: TableDef = userTable__294();
        let mut t___4235: SafeIdentifier = csid__293("name");
        let cs__518: Changeset = changeset(t___4234.clone(), params__517.clone()).cast(std::sync::Arc::new(vec![t___4235.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name")]));
        let sqlFrag__519: SqlFragment;
        'ok___4866: {
            'orelse___1034: {
                t___2347 = match cs__518.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1034
                };
                sqlFrag__519 = t___2347.clone();
                break 'ok___4866;
            }
            sqlFrag__519 = panic!();
        }
        let s__520: std::sync::Arc<String> = sqlFrag__519.to_string();
        let mut t___4241: bool = Some(s__520.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__520: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__4231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__520.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__520: s__520.clone()
        };
        let fn__4231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4231())
        };
        test___41.assert(t___4241, fn__4231.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__910() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let params__522: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4224: TableDef = userTable__294();
        let mut t___4225: SafeIdentifier = csid__293("name");
        let cs__523: Changeset = changeset(t___4224.clone(), params__522.clone()).cast(std::sync::Arc::new(vec![t___4225.clone()])).validate_required(std::sync::Arc::new(vec![csid__293("name")]));
        let didBubble__524: bool;
        'ok___4867: {
            'orelse___1035: {
                match cs__523.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1035
                };
                didBubble__524 = false;
                break 'ok___4867;
            }
            didBubble__524 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___49 {}
        impl ClosureGroup___49 {
            fn fn__4222(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___49 {};
        let fn__4222 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4222())
        };
        test___42.assert(didBubble__524, fn__4222.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__935() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let q__582: Query = from(sid__295("users"));
        let mut t___4157: bool = Some(q__582.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__4152(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__4152 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4152())
        };
        test___43.assert(t___4157, fn__4152.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__936() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let mut t___4143: SafeIdentifier = sid__295("users");
        let mut t___4144: SafeIdentifier = sid__295("id");
        let mut t___4145: SafeIdentifier = sid__295("name");
        let q__584: Query = from(t___4143.clone()).select([t___4144.clone(), t___4145.clone()]);
        let mut t___4150: bool = Some(q__584.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__4142(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__4142 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4142())
        };
        test___44.assert(t___4150, fn__4142.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__937() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let mut t___4131: SafeIdentifier = sid__295("users");
        let mut t___4132: SqlBuilder = SqlBuilder::new();
        t___4132.append_safe("age > ");
        t___4132.append_int32(18);
        let mut t___4135: SqlFragment = t___4132.accumulated();
        let q__586: Query = from(t___4131.clone()).r#where(t___4135.clone());
        let mut t___4140: bool = Some(q__586.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__4130(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__4130 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4130())
        };
        test___45.assert(t___4140, fn__4130.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__939() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let mut t___4119: SafeIdentifier = sid__295("users");
        let mut t___4120: SqlBuilder = SqlBuilder::new();
        t___4120.append_safe("active = ");
        t___4120.append_boolean(true);
        let mut t___4123: SqlFragment = t___4120.accumulated();
        let q__588: Query = from(t___4119.clone()).r#where(t___4123.clone());
        let mut t___4128: bool = Some(q__588.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__4118(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__4118 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4118())
        };
        test___46.assert(t___4128, fn__4118.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__941() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let mut t___4102: SafeIdentifier = sid__295("users");
        let mut t___4103: SqlBuilder = SqlBuilder::new();
        t___4103.append_safe("age > ");
        t___4103.append_int32(18);
        let mut t___4106: SqlFragment = t___4103.accumulated();
        let mut t___4107: Query = from(t___4102.clone()).r#where(t___4106.clone());
        let mut t___4108: SqlBuilder = SqlBuilder::new();
        t___4108.append_safe("active = ");
        t___4108.append_boolean(true);
        let q__590: Query = t___4107.r#where(t___4108.accumulated());
        let mut t___4116: bool = Some(q__590.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__4101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__4101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4101())
        };
        test___47.assert(t___4116, fn__4101.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__944() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___4093: SafeIdentifier = sid__295("users");
        let mut t___4094: SafeIdentifier = sid__295("name");
        let q__592: Query = from(t___4093.clone()).order_by(t___4094.clone(), true);
        let mut t___4099: bool = Some(q__592.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__4092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__4092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4092())
        };
        test___48.assert(t___4099, fn__4092.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__945() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___4084: SafeIdentifier = sid__295("users");
        let mut t___4085: SafeIdentifier = sid__295("created_at");
        let q__594: Query = from(t___4084.clone()).order_by(t___4085.clone(), false);
        let mut t___4090: bool = Some(q__594.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__4083(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__4083 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4083())
        };
        test___49.assert(t___4090, fn__4083.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__946() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___2195: Query;
        let mut t___2196: Query;
        let q__596: Query;
        'ok___4869: {
            'orelse___1037: {
                t___2195 = match from(sid__295("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1037
                };
                t___2196 = match t___2195.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1037
                };
                q__596 = t___2196.clone();
                break 'ok___4869;
            }
            q__596 = panic!();
        }
        let mut t___4081: bool = Some(q__596.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__4076(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__4076 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4076())
        };
        test___50.assert(t___4081, fn__4076.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__947() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let didBubble__598: bool;
        'ok___4870: {
            'orelse___1038: {
                match from(sid__295("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1038
                };
                didBubble__598 = false;
                break 'ok___4870;
            }
            didBubble__598 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__4072(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__4072 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4072())
        };
        test___51.assert(didBubble__598, fn__4072.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__948() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let didBubble__600: bool;
        'ok___4871: {
            'orelse___1039: {
                match from(sid__295("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1039
                };
                didBubble__600 = false;
                break 'ok___4871;
            }
            didBubble__600 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__4068(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__4068 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4068())
        };
        test___52.assert(didBubble__600, fn__4068.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__949() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___4046: SafeIdentifier;
        let mut t___4047: SafeIdentifier;
        let mut t___4048: SafeIdentifier;
        let mut t___4049: SafeIdentifier;
        let mut t___4050: Query;
        let mut t___4051: SqlBuilder;
        let mut t___4055: Query;
        let mut t___4056: SqlBuilder;
        let mut t___2181: Query;
        let mut t___2182: Query;
        let minAge__602: i32 = 21;
        let q__603: Query;
        'ok___4872: {
            'orelse___1040: {
                t___4046 = sid__295("users");
                t___4047 = sid__295("id");
                t___4048 = sid__295("name");
                t___4049 = sid__295("email");
                t___4050 = from(t___4046.clone()).select([t___4047.clone(), t___4048.clone(), t___4049.clone()]);
                t___4051 = SqlBuilder::new();
                t___4051.append_safe("age >= ");
                t___4051.append_int32(21);
                t___4055 = t___4050.r#where(t___4051.accumulated());
                t___4056 = SqlBuilder::new();
                t___4056.append_safe("active = ");
                t___4056.append_boolean(true);
                t___2181 = match t___4055.r#where(t___4056.accumulated()).order_by(sid__295("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1040
                };
                t___2182 = match t___2181.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1040
                };
                q__603 = t___2182.clone();
                break 'ok___4872;
            }
            q__603 = panic!();
        }
        let mut t___4066: bool = Some(q__603.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__4045(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__4045 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4045())
        };
        test___53.assert(t___4066, fn__4045.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__952() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let mut t___2158: SqlFragment;
        let mut t___2159: SqlFragment;
        let q__605: Query = from(sid__295("users"));
        'ok___4873: {
            'orelse___1041: {
                t___2158 = match q__605.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1041
                };
                t___2159 = t___2158.clone();
                break 'ok___4873;
            }
            t___2159 = panic!();
        }
        let s__606: std::sync::Arc<String> = t___2159.to_string();
        let mut t___4043: bool = Some(s__606.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___61 {
            s__606: std::sync::Arc<String>
        }
        impl ClosureGroup___61 {
            fn fn__4039(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__606.clone()));
            }
        }
        let closure_group = ClosureGroup___61 {
            s__606: s__606.clone()
        };
        let fn__4039 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4039())
        };
        test___54.assert(t___4043, fn__4039.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__953() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let mut t___2150: Query;
        let mut t___2153: SqlFragment;
        let mut t___2154: SqlFragment;
        let q__608: Query;
        'ok___4874: {
            'orelse___1042: {
                t___2150 = match from(sid__295("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1042
                };
                q__608 = t___2150.clone();
                break 'ok___4874;
            }
            q__608 = panic!();
        }
        'ok___4875: {
            'orelse___1043: {
                t___2153 = match q__608.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1043
                };
                t___2154 = t___2153.clone();
                break 'ok___4875;
            }
            t___2154 = panic!();
        }
        let s__609: std::sync::Arc<String> = t___2154.to_string();
        let mut t___4037: bool = Some(s__609.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___62 {
            s__609: std::sync::Arc<String>
        }
        impl ClosureGroup___62 {
            fn fn__4033(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__609.clone()));
            }
        }
        let closure_group = ClosureGroup___62 {
            s__609: s__609.clone()
        };
        let fn__4033 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4033())
        };
        test___55.assert(t___4037, fn__4033.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__954() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let didBubble__611: bool;
        'ok___4876: {
            'orelse___1044: {
                match from(sid__295("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1044
                };
                didBubble__611 = false;
                break 'ok___4876;
            }
            didBubble__611 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__4029(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__4029 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4029())
        };
        test___56.assert(didBubble__611, fn__4029.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__955() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let evil__613: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___4013: SafeIdentifier = sid__295("users");
        let mut t___4014: SqlBuilder = SqlBuilder::new();
        t___4014.append_safe("name = ");
        t___4014.append_string("'; DROP TABLE users; --");
        let mut t___4017: SqlFragment = t___4014.accumulated();
        let q__614: Query = from(t___4013.clone()).r#where(t___4017.clone());
        let s__615: std::sync::Arc<String> = q__614.to_sql().to_string();
        let mut t___4022: bool = temper_core::string::index_of( & s__615, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___64 {
            s__615: std::sync::Arc<String>
        }
        impl ClosureGroup___64 {
            fn fn__4012(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__615.clone()));
            }
        }
        let closure_group = ClosureGroup___64 {
            s__615: s__615.clone()
        };
        let fn__4012 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4012())
        };
        test___57.assert(t___4022, fn__4012.clone());
        let mut t___4026: bool = temper_core::string::index_of( & s__615, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___65 {
            s__615: std::sync::Arc<String>
        }
        impl ClosureGroup___65 {
            fn fn__4011(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__615.clone()));
            }
        }
        let closure_group = ClosureGroup___65 {
            s__615: s__615.clone()
        };
        let fn__4011 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4011())
        };
        test___57.assert(t___4026, fn__4011.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__957() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let attack__617: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__618: bool;
        'ok___4877: {
            'orelse___1045: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1045
                };
                didBubble__618 = false;
                break 'ok___4877;
            }
            didBubble__618 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___66 {}
        impl ClosureGroup___66 {
            fn fn__4008(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___66 {};
        let fn__4008 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4008())
        };
        test___58.assert(didBubble__618, fn__4008.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__958() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___2123: SafeIdentifier;
        let id__656: SafeIdentifier;
        'ok___4878: {
            'orelse___1046: {
                t___2123 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1046
                };
                id__656 = t___2123.clone();
                break 'ok___4878;
            }
            id__656 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4006: bool = Some(id__656.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___67 {}
        impl ClosureGroup___67 {
            fn fn__4003(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___67 {};
        let fn__4003 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4003())
        };
        test___65.assert(t___4006, fn__4003.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__959() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let didBubble__658: bool;
        'ok___4879: {
            'orelse___1047: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1047
                };
                didBubble__658 = false;
                break 'ok___4879;
            }
            didBubble__658 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__4000(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__4000 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4000())
        };
        test___66.assert(didBubble__658, fn__4000.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__960() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let didBubble__660: bool;
        'ok___4880: {
            'orelse___1048: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1048
                };
                didBubble__660 = false;
                break 'ok___4880;
            }
            didBubble__660 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__3997(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__3997 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3997())
        };
        test___67.assert(didBubble__660, fn__3997.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__961() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let cases__662: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___70 {
            test___68: temper_std::testing::Test
        }
        impl ClosureGroup___70 {
            fn fn__3994(& self, c__663: impl temper_core::ToArcString) {
                let c__663 = c__663.to_arc_string();
                let didBubble__664: bool;
                'ok___4881: {
                    'orelse___1049: {
                        match safe_identifier(c__663.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1049
                        };
                        didBubble__664 = false;
                        break 'ok___4881;
                    }
                    didBubble__664 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___71 {
                    c__663: std::sync::Arc<String>
                }
                impl ClosureGroup___71 {
                    fn fn__3991(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__663.clone()));
                    }
                }
                let closure_group = ClosureGroup___71 {
                    c__663: c__663.clone()
                };
                let fn__3991 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__3991())
                };
                self.test___68.assert(didBubble__664, fn__3991.clone());
            }
        }
        let closure_group = ClosureGroup___70 {
            test___68: test___68.clone()
        };
        let fn__3994 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__663: std::sync::Arc<String> | closure_group.fn__3994(c__663))
        };
        temper_core::listed::list_for_each( & cases__662, & ( * fn__3994.clone()));
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__962() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let mut t___2100: SafeIdentifier;
        let mut t___2101: SafeIdentifier;
        let mut t___2102: SafeIdentifier;
        let mut t___2103: SafeIdentifier;
        let mut t___2106: SafeIdentifier;
        let mut t___2107: SafeIdentifier;
        let mut t___2111: FieldDef;
        'ok___4882: {
            'orelse___1050: {
                t___2100 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1050
                };
                t___2101 = t___2100.clone();
                break 'ok___4882;
            }
            t___2101 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___4883: {
            'orelse___1051: {
                t___2102 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1051
                };
                t___2103 = t___2102.clone();
                break 'ok___4883;
            }
            t___2103 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___3981: StringField = StringField::new();
        let mut t___3982: FieldDef = FieldDef::new(t___2103.clone(), FieldType::new(t___3981.clone()), false);
        'ok___4884: {
            'orelse___1052: {
                t___2106 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1052
                };
                t___2107 = t___2106.clone();
                break 'ok___4884;
            }
            t___2107 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___3983: IntField = IntField::new();
        let mut t___3984: FieldDef = FieldDef::new(t___2107.clone(), FieldType::new(t___3983.clone()), false);
        let td__666: TableDef = TableDef::new(t___2101.clone(), [t___3982.clone(), t___3984.clone()]);
        let f__667: FieldDef;
        'ok___4885: {
            'orelse___1053: {
                t___2111 = match td__666.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1053
                };
                f__667 = t___2111.clone();
                break 'ok___4885;
            }
            f__667 = panic!();
        }
        let mut t___3989: bool = Some(f__667.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__3980(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__3980 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3980())
        };
        test___69.assert(t___3989, fn__3980.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__963() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let mut t___2091: SafeIdentifier;
        let mut t___2092: SafeIdentifier;
        let mut t___2093: SafeIdentifier;
        let mut t___2094: SafeIdentifier;
        'ok___4886: {
            'orelse___1054: {
                t___2091 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1054
                };
                t___2092 = t___2091.clone();
                break 'ok___4886;
            }
            t___2092 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___4887: {
            'orelse___1055: {
                t___2093 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1055
                };
                t___2094 = t___2093.clone();
                break 'ok___4887;
            }
            t___2094 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___3975: StringField = StringField::new();
        let mut t___3976: FieldDef = FieldDef::new(t___2094.clone(), FieldType::new(t___3975.clone()), false);
        let td__669: TableDef = TableDef::new(t___2092.clone(), [t___3976.clone()]);
        let didBubble__670: bool;
        'ok___4888: {
            'orelse___1056: {
                match td__669.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1056
                };
                didBubble__670 = false;
                break 'ok___4888;
            }
            didBubble__670 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__3974(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__3974 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3974())
        };
        test___70.assert(didBubble__670, fn__3974.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__964() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___2079: SafeIdentifier;
        let mut t___2080: SafeIdentifier;
        let mut t___2083: SafeIdentifier;
        let mut t___2084: SafeIdentifier;
        'ok___4889: {
            'orelse___1057: {
                t___2079 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1057
                };
                t___2080 = t___2079.clone();
                break 'ok___4889;
            }
            t___2080 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___3963: StringField = StringField::new();
        let required__672: FieldDef = FieldDef::new(t___2080.clone(), FieldType::new(t___3963.clone()), false);
        'ok___4890: {
            'orelse___1058: {
                t___2083 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1058
                };
                t___2084 = t___2083.clone();
                break 'ok___4890;
            }
            t___2084 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___3965: StringField = StringField::new();
        let optional__673: FieldDef = FieldDef::new(t___2084.clone(), FieldType::new(t___3965.clone()), true);
        let mut t___3969: bool = ! required__672.nullable();
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__3962(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__3962 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3962())
        };
        test___71.assert(t___3969, fn__3962.clone());
        let mut t___3971: bool = optional__673.nullable();
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__3961(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__3961 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3961())
        };
        test___71.assert(t___3971, fn__3961.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__965() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn build__797(& self, name__799: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__799 = name__799.to_arc_string();
                let mut t___3943: SqlBuilder = SqlBuilder::new();
                t___3943.append_safe("select * from hi where name = ");
                t___3943.append_string(name__799.clone());
                return t___3943.accumulated().to_string();
            }
            fn buildWrong__798(& self, name__801: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__801 = name__801.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__801.clone()));
            }
        }
        let closure_group = ClosureGroup___76 {};
        let build__797 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__799: std::sync::Arc<String> | closure_group.build__797(name__799))
        };
        let buildWrong__798 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__801: std::sync::Arc<String> | closure_group.buildWrong__798(name__801))
        };
        let actual___967: std::sync::Arc<String> = build__797(std::sync::Arc::new("world".to_string()));
        let mut t___3953: bool = Some(actual___967.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___77 {
            actual___967: std::sync::Arc<String>
        }
        impl ClosureGroup___77 {
            fn fn__3950(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___967.clone()));
            }
        }
        let closure_group = ClosureGroup___77 {
            actual___967: actual___967.clone()
        };
        let fn__3950 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3950())
        };
        test___73.assert(t___3953, fn__3950.clone());
        let bobbyTables__803: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___969: std::sync::Arc<String> = build__797(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___3957: bool = Some(actual___969.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___78 {
            actual___969: std::sync::Arc<String>
        }
        impl ClosureGroup___78 {
            fn fn__3949(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___969.clone()));
            }
        }
        let closure_group = ClosureGroup___78 {
            actual___969: actual___969.clone()
        };
        let fn__3949 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3949())
        };
        test___73.assert(t___3957, fn__3949.clone());
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__3948(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__3948 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3948())
        };
        test___73.assert(true, fn__3948.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__973() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___3911: SqlBuilder = SqlBuilder::new();
        t___3911.append_safe("v = ");
        t___3911.append_string("");
        let actual___974: std::sync::Arc<String> = t___3911.accumulated().to_string();
        let mut t___3917: bool = Some(actual___974.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___80 {
            actual___974: std::sync::Arc<String>
        }
        impl ClosureGroup___80 {
            fn fn__3910(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___974.clone()));
            }
        }
        let closure_group = ClosureGroup___80 {
            actual___974: actual___974.clone()
        };
        let fn__3910 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3910())
        };
        test___74.assert(t___3917, fn__3910.clone());
        let mut t___3919: SqlBuilder = SqlBuilder::new();
        t___3919.append_safe("v = ");
        t___3919.append_string("a''b");
        let actual___977: std::sync::Arc<String> = t___3919.accumulated().to_string();
        let mut t___3925: bool = Some(actual___977.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___81 {
            actual___977: std::sync::Arc<String>
        }
        impl ClosureGroup___81 {
            fn fn__3909(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___977.clone()));
            }
        }
        let closure_group = ClosureGroup___81 {
            actual___977: actual___977.clone()
        };
        let fn__3909 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3909())
        };
        test___74.assert(t___3925, fn__3909.clone());
        let mut t___3927: SqlBuilder = SqlBuilder::new();
        t___3927.append_safe("v = ");
        t___3927.append_string("Hello 世界");
        let actual___980: std::sync::Arc<String> = t___3927.accumulated().to_string();
        let mut t___3933: bool = Some(actual___980.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___82 {
            actual___980: std::sync::Arc<String>
        }
        impl ClosureGroup___82 {
            fn fn__3908(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___980.clone()));
            }
        }
        let closure_group = ClosureGroup___82 {
            actual___980: actual___980.clone()
        };
        let fn__3908 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3908())
        };
        test___74.assert(t___3933, fn__3908.clone());
        let mut t___3935: SqlBuilder = SqlBuilder::new();
        t___3935.append_safe("v = ");
        t___3935.append_string("Line1\x0aLine2");
        let actual___983: std::sync::Arc<String> = t___3935.accumulated().to_string();
        let mut t___3941: bool = Some(actual___983.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___83 {
            actual___983: std::sync::Arc<String>
        }
        impl ClosureGroup___83 {
            fn fn__3907(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___983.clone()));
            }
        }
        let closure_group = ClosureGroup___83 {
            actual___983: actual___983.clone()
        };
        let fn__3907 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3907())
        };
        test___74.assert(t___3941, fn__3907.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__986() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___2024: temper_std::temporal::Date;
        let mut t___3882: SqlBuilder = SqlBuilder::new();
        t___3882.append_safe("select ");
        t___3882.append_int32(42);
        t___3882.append_safe(", ");
        t___3882.append_int64(43);
        t___3882.append_safe(", ");
        t___3882.append_float64(19.99f64);
        t___3882.append_safe(", ");
        t___3882.append_boolean(true);
        t___3882.append_safe(", ");
        t___3882.append_boolean(false);
        let actual___987: std::sync::Arc<String> = t___3882.accumulated().to_string();
        let mut t___3896: bool = Some(actual___987.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___84 {
            actual___987: std::sync::Arc<String>
        }
        impl ClosureGroup___84 {
            fn fn__3881(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___987.clone()));
            }
        }
        let closure_group = ClosureGroup___84 {
            actual___987: actual___987.clone()
        };
        let fn__3881 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3881())
        };
        test___75.assert(t___3896, fn__3881.clone());
        let date__806: temper_std::temporal::Date;
        'ok___4891: {
            'orelse___1059: {
                t___2024 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1059
                };
                date__806 = t___2024.clone();
                break 'ok___4891;
            }
            date__806 = panic!();
        }
        let mut t___3898: SqlBuilder = SqlBuilder::new();
        t___3898.append_safe("insert into t values (");
        t___3898.append_date(date__806.clone());
        t___3898.append_safe(")");
        let actual___990: std::sync::Arc<String> = t___3898.accumulated().to_string();
        let mut t___3905: bool = Some(actual___990.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___85 {
            actual___990: std::sync::Arc<String>
        }
        impl ClosureGroup___85 {
            fn fn__3880(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___990.clone()));
            }
        }
        let closure_group = ClosureGroup___85 {
            actual___990: actual___990.clone()
        };
        let fn__3880 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3880())
        };
        test___75.assert(t___3905, fn__3880.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn lists__993() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___1996: temper_std::temporal::Date;
        let mut t___1997: temper_std::temporal::Date;
        let mut t___1998: temper_std::temporal::Date;
        let mut t___1999: temper_std::temporal::Date;
        let mut t___3826: SqlBuilder = SqlBuilder::new();
        t___3826.append_safe("v IN (");
        t___3826.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___3826.append_safe(")");
        let actual___994: std::sync::Arc<String> = t___3826.accumulated().to_string();
        let mut t___3833: bool = Some(actual___994.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___86 {
            actual___994: std::sync::Arc<String>
        }
        impl ClosureGroup___86 {
            fn fn__3825(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___994.clone()));
            }
        }
        let closure_group = ClosureGroup___86 {
            actual___994: actual___994.clone()
        };
        let fn__3825 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3825())
        };
        test___76.assert(t___3833, fn__3825.clone());
        let mut t___3835: SqlBuilder = SqlBuilder::new();
        t___3835.append_safe("v IN (");
        t___3835.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___3835.append_safe(")");
        let actual___997: std::sync::Arc<String> = t___3835.accumulated().to_string();
        let mut t___3842: bool = Some(actual___997.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___87 {
            actual___997: std::sync::Arc<String>
        }
        impl ClosureGroup___87 {
            fn fn__3824(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___997.clone()));
            }
        }
        let closure_group = ClosureGroup___87 {
            actual___997: actual___997.clone()
        };
        let fn__3824 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3824())
        };
        test___76.assert(t___3842, fn__3824.clone());
        let mut t___3844: SqlBuilder = SqlBuilder::new();
        t___3844.append_safe("v IN (");
        t___3844.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___3844.append_safe(")");
        let actual___1000: std::sync::Arc<String> = t___3844.accumulated().to_string();
        let mut t___3851: bool = Some(actual___1000.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___88 {
            actual___1000: std::sync::Arc<String>
        }
        impl ClosureGroup___88 {
            fn fn__3823(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1000.clone()));
            }
        }
        let closure_group = ClosureGroup___88 {
            actual___1000: actual___1000.clone()
        };
        let fn__3823 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3823())
        };
        test___76.assert(t___3851, fn__3823.clone());
        let mut t___3853: SqlBuilder = SqlBuilder::new();
        t___3853.append_safe("v IN (");
        t___3853.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___3853.append_safe(")");
        let actual___1003: std::sync::Arc<String> = t___3853.accumulated().to_string();
        let mut t___3860: bool = Some(actual___1003.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___89 {
            actual___1003: std::sync::Arc<String>
        }
        impl ClosureGroup___89 {
            fn fn__3822(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1003.clone()));
            }
        }
        let closure_group = ClosureGroup___89 {
            actual___1003: actual___1003.clone()
        };
        let fn__3822 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3822())
        };
        test___76.assert(t___3860, fn__3822.clone());
        let mut t___3862: SqlBuilder = SqlBuilder::new();
        t___3862.append_safe("v IN (");
        t___3862.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___3862.append_safe(")");
        let actual___1006: std::sync::Arc<String> = t___3862.accumulated().to_string();
        let mut t___3869: bool = Some(actual___1006.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___90 {
            actual___1006: std::sync::Arc<String>
        }
        impl ClosureGroup___90 {
            fn fn__3821(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1006.clone()));
            }
        }
        let closure_group = ClosureGroup___90 {
            actual___1006: actual___1006.clone()
        };
        let fn__3821 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3821())
        };
        test___76.assert(t___3869, fn__3821.clone());
        'ok___4892: {
            'orelse___1060: {
                t___1996 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1060
                };
                t___1997 = t___1996.clone();
                break 'ok___4892;
            }
            t___1997 = panic!();
        }
        'ok___4893: {
            'orelse___1061: {
                t___1998 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1061
                };
                t___1999 = t___1998.clone();
                break 'ok___4893;
            }
            t___1999 = panic!();
        }
        let dates__808: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___1997.clone(), t___1999.clone()]);
        let mut t___3871: SqlBuilder = SqlBuilder::new();
        t___3871.append_safe("v IN (");
        t___3871.append_date_list(temper_core::ToListed::to_listed(dates__808.clone()));
        t___3871.append_safe(")");
        let actual___1009: std::sync::Arc<String> = t___3871.accumulated().to_string();
        let mut t___3878: bool = Some(actual___1009.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___91 {
            actual___1009: std::sync::Arc<String>
        }
        impl ClosureGroup___91 {
            fn fn__3820(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1009.clone()));
            }
        }
        let closure_group = ClosureGroup___91 {
            actual___1009: actual___1009.clone()
        };
        let fn__3820 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3820())
        };
        test___76.assert(t___3878, fn__3820.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1012() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let name__810: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___3789: SqlBuilder = SqlBuilder::new();
        t___3789.append_safe("where p.last_name = ");
        t___3789.append_string("Someone");
        let condition__811: SqlFragment = t___3789.accumulated();
        let mut t___3793: SqlBuilder = SqlBuilder::new();
        t___3793.append_safe("select p.id from person p ");
        t___3793.append_fragment(condition__811.clone());
        let actual___1014: std::sync::Arc<String> = t___3793.accumulated().to_string();
        let mut t___3799: bool = Some(actual___1014.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___92 {
            actual___1014: std::sync::Arc<String>
        }
        impl ClosureGroup___92 {
            fn fn__3788(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1014.clone()));
            }
        }
        let closure_group = ClosureGroup___92 {
            actual___1014: actual___1014.clone()
        };
        let fn__3788 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3788())
        };
        test___77.assert(t___3799, fn__3788.clone());
        let mut t___3801: SqlBuilder = SqlBuilder::new();
        t___3801.append_safe("select p.id from person p ");
        t___3801.append_part(SqlPart::new(condition__811.to_source()));
        let actual___1017: std::sync::Arc<String> = t___3801.accumulated().to_string();
        let mut t___3808: bool = Some(actual___1017.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___93 {
            actual___1017: std::sync::Arc<String>
        }
        impl ClosureGroup___93 {
            fn fn__3787(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1017.clone()));
            }
        }
        let closure_group = ClosureGroup___93 {
            actual___1017: actual___1017.clone()
        };
        let fn__3787 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3787())
        };
        test___77.assert(t___3808, fn__3787.clone());
        let parts__812: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___3812: SqlBuilder = SqlBuilder::new();
        t___3812.append_safe("select ");
        t___3812.append_part_list(parts__812.clone());
        let actual___1020: std::sync::Arc<String> = t___3812.accumulated().to_string();
        let mut t___3818: bool = Some(actual___1020.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___94 {
            actual___1020: std::sync::Arc<String>
        }
        impl ClosureGroup___94 {
            fn fn__3786(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1020.clone()));
            }
        }
        let closure_group = ClosureGroup___94 {
            actual___1020: actual___1020.clone()
        };
        let fn__3786 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__3786())
        };
        test___77.assert(t___3818, fn__3786.clone());
        test___77.soft_fail_to_hard()
    }
    use super::*;
}
