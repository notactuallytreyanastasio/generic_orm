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
struct TestStruct {
    failed_on_assert: bool, passing: bool, messages: temper_core::ListBuilder<std::sync::Arc<String>>
}
#[derive(Clone)]
pub struct Test(std::sync::Arc<std::sync::RwLock<TestStruct>>);
impl Test {
    pub fn assert(& self, success__43: bool, message__44: std::sync::Arc<dyn Fn () -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) {
        let mut t___406: std::sync::Arc<String>;
        if ! success__43 {
            self.0.write().unwrap().passing = false;
            t___406 = message__44();
            temper_core::listed::add( & self.0.read().unwrap().messages, t___406.clone(), None);
        }
    }
    pub fn assert_hard(& self, success__47: bool, message__48: std::sync::Arc<dyn Fn () -> std::sync::Arc<String> + std::marker::Send + std::marker::Sync>) -> temper_core::Result<()> {
        self.assert(success__47, message__48.clone());
        if ! success__47 {
            self.0.write().unwrap().failed_on_assert = true;
            Err(temper_core::Error::with_optional_message(self.messages_combined())) ? ;
        }
        return Ok(());
    }
    pub fn soft_fail_to_hard(& self) -> temper_core::Result<()> {
        if self.has_unhandled_fail() {
            self.0.write().unwrap().failed_on_assert = true;
            Err(temper_core::Error::with_optional_message(self.messages_combined())) ? ;
        }
        return Ok(());
    }
    pub fn passing(& self) -> bool {
        return self.0.read().unwrap().passing;
    }
    pub fn messages(& self) -> temper_core::List<std::sync::Arc<String>> {
        return temper_core::ListedTrait::to_list( & self.0.read().unwrap().messages);
    }
    pub fn failed_on_assert(& self) -> bool {
        return self.0.read().unwrap().failed_on_assert;
    }
    pub fn has_unhandled_fail(& self) -> bool {
        let mut t___264: bool;
        if self.0.read().unwrap().failed_on_assert {
            t___264 = true;
        } else {
            t___264 = self.0.read().unwrap().passing;
        }
        return ! t___264;
    }
    pub fn messages_combined(& self) -> Option<std::sync::Arc<String>> {
        let return__35: Option<std::sync::Arc<String>>;
        if temper_core::ListedTrait::is_empty( & self.0.read().unwrap().messages) {
            return__35 = None;
        } else {
            #[derive(Clone)]
            struct ClosureGroup___1 {}
            impl ClosureGroup___1 {
                fn fn__399(& self, it__64: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                    let it__64 = it__64.to_arc_string();
                    return it__64.clone();
                }
            }
            let closure_group = ClosureGroup___1 {};
            let fn__399 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | it__64: std::sync::Arc<String> | closure_group.fn__399(it__64))
            };
            return__35 = Some(temper_core::listed::join( & self.0.read().unwrap().messages, std::sync::Arc::new(", ".to_string()), & ( * fn__399.clone())));
        }
        return return__35.clone();
    }
    pub fn new() -> Test {
        let failed_on_assert;
        let passing;
        let messages;
        failed_on_assert = false;
        passing = true;
        let mut t___398: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        messages = t___398.clone();
        let selfish = Test(std::sync::Arc::new(std::sync::RwLock::new(TestStruct {
                        failed_on_assert, passing, messages
        })));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(Test, []);
pub fn process_test_cases(testCases__69: impl temper_core::ToList<(std::sync::Arc<String>, std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync>)>) -> temper_core::List<(std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>)> {
    let testCases__69 = testCases__69.to_list();
    #[derive(Clone)]
    struct ClosureGroup___2 {}
    impl ClosureGroup___2 {
        fn fn__395(& self, testCase__71: (std::sync::Arc<String>, std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync>)) ->(std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>) {
            let mut t___390: bool;
            let mut t___393: temper_core::List<std::sync::Arc<String>>;
            let mut t___246: bool;
            let mut t___248: bool;
            let key__73: std::sync::Arc<String> = testCase__71.key();
            let fun__74: std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync> = testCase__71.value();
            let test__75: Test = Test::new();
            let mut hadBubble__76: bool = false;
            'ok___2605: {
                'orelse___125: {
                    match fun__74(test__75.clone()) {
                        Ok(x) => x,
                        _ => break 'orelse___125
                    };
                    break 'ok___2605;
                }
                hadBubble__76 = true;
            }
            let messages__77: temper_core::List<std::sync::Arc<String>> = test__75.messages();
            let failures__78: temper_core::List<std::sync::Arc<String>>;
            if test__75.passing() {
                t___246 = ! hadBubble__76;
            } else {
                t___246 = false;
            }
            if t___246 {
                failures__78 = std::sync::Arc::new(vec![]);
            } else {
                if hadBubble__76 {
                    t___390 = test__75.failed_on_assert();
                    t___248 = ! t___390;
                } else {
                    t___248 = false;
                }
                if t___248 {
                    let allMessages__79: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::ListedTrait::to_list_builder( & messages__77);
                    temper_core::listed::add( & allMessages__79, std::sync::Arc::new("Bubble".to_string()), None);
                    t___393 = temper_core::ListedTrait::to_list( & allMessages__79);
                    failures__78 = t___393.clone();
                } else {
                    failures__78 = messages__77.clone();
                }
            }
            return (key__73.clone(), failures__78.clone());
        }
    }
    let closure_group = ClosureGroup___2 {};
    let fn__395 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | testCase__71: (std::sync::Arc<String>, std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync>) | closure_group.fn__395(testCase__71))
    };
    return temper_core::listed::map( & testCases__69, & ( * fn__395.clone()));
}
fn escapeXml__41(s__103: impl temper_core::ToArcString) -> std::sync::Arc<String> {
    let s__103 = s__103.to_arc_string();
    let return__40: std::sync::Arc<String>;
    let mut t___381: usize;
    let mut t___382: usize;
    let mut t___225: bool;
    let mut t___226: bool;
    let mut t___227: bool;
    let mut t___228: bool;
    let mut t___230: std::sync::Arc<String>;
    let mut t___231: std::sync::Arc<String>;
    let sb__105: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    let end__106: usize = s__103.len();
    let mut emitted__107: usize = 0usize;
    let mut i__108: usize = 0usize;
    'loop___2665: while Some(i__108) < Some(end__106) {
        'continue___408: {
            let c__109: i32 = temper_core::string::get( & s__103, i__108);
            if Some(c__109) == Some(38) {
                t___231 = std::sync::Arc::new("&amp;".to_string());
            } else {
                if Some(c__109) == Some(60) {
                    t___231 = std::sync::Arc::new("&lt;".to_string());
                } else {
                    if Some(c__109) == Some(62) {
                        t___231 = std::sync::Arc::new("&gt;".to_string());
                    } else {
                        if Some(c__109) == Some(39) {
                            t___231 = std::sync::Arc::new("&#39;".to_string());
                        } else {
                            if Some(c__109) == Some(34) {
                                t___231 = std::sync::Arc::new("&#34;".to_string());
                            } else {
                                if Some(c__109) == Some(10) {
                                    t___226 = true;
                                } else {
                                    if Some(c__109) == Some(13) {
                                        t___225 = true;
                                    } else {
                                        t___225 = Some(c__109) == Some(9);
                                    }
                                    t___226 = t___225;
                                }
                                if t___226 {
                                    break 'continue___408;
                                } else {
                                    if Some(c__109) < Some(32) {
                                        t___228 = true;
                                    } else {
                                        if Some(c__109) == Some(65534) {
                                            t___227 = true;
                                        } else {
                                            t___227 = Some(c__109) == Some(65535);
                                        }
                                        t___228 = t___227;
                                    }
                                    if t___228 {
                                        t___230 = std::sync::Arc::new(format!("[0x{}]", temper_core::int_to_string(c__109, Some(16))));
                                    } else {
                                        break 'continue___408;
                                    }
                                    t___231 = t___230.clone();
                                }
                            }
                        }
                    }
                }
            }
            let esc__110: std::sync::Arc<String> = t___231.clone();
            temper_core::string::builder::append_between( & sb__105, s__103.clone(), emitted__107, i__108);
            temper_core::string::builder::append( & sb__105, esc__110.clone());
            t___381 = temper_core::string::next( & s__103, i__108);
            emitted__107 = t___381;
        }
        t___382 = temper_core::string::next( & s__103, i__108);
        i__108 = t___382;
    }
    if emitted__107 == 0usize {
        return__40 = s__103.clone();
    } else {
        temper_core::string::builder::append_between( & sb__105, s__103.clone(), emitted__107, end__106);
        return__40 = temper_core::string::builder::to_string( & sb__105);
    }
    return return__40.clone();
}
pub fn report_test_results(testResults__80: impl temper_core::ToList<(std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>)>, writeLine__81: std::sync::Arc<dyn Fn (std::sync::Arc<String>) + std::marker::Send + std::marker::Sync>) {
    let testResults__80 = testResults__80.to_list();
    let mut t___360: i32;
    let mut t___363: std::sync::Arc<String>;
    let mut t___369: std::sync::Arc<String>;
    writeLine__81(std::sync::Arc::new("<testsuites>".to_string()));
    let total__83: std::sync::Arc<String> = temper_core::int_to_string(temper_core::ListedTrait::len( & testResults__80), None);
    #[derive(Clone)]
    struct ClosureGroup___3 {}
    impl ClosureGroup___3 {
        fn fn__352(& self, fails__85: i32, testResult__86: (std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>)) -> i32 {
            let mut t___203: i32;
            if temper_core::ListedTrait::is_empty( & testResult__86.value()) {
                t___203 = 0;
            } else {
                t___203 = 1;
            }
            return fails__85.wrapping_add(t___203);
        }
    }
    let closure_group = ClosureGroup___3 {};
    let fn__352 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | fails__85: i32, testResult__86: (std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>) | closure_group.fn__352(fails__85, testResult__86))
    };
    let fails__84: std::sync::Arc<String> = temper_core::int_to_string(temper_core::listed::reduce_from( & testResults__80, 0, & ( * fn__352.clone())), None);
    let totals__88: std::sync::Arc<String> = std::sync::Arc::new(format!("tests='{}' failures='{}'", total__83.clone(), fails__84.clone()));
    writeLine__81(std::sync::Arc::new(format!("  <testsuite name='suite' {} time='0.0'>", totals__88.clone())));
    let mut i__89: i32 = 0;
    'loop___2666: loop {
        t___360 = temper_core::ListedTrait::len( & testResults__80);
        if ! (Some(i__89) < Some(t___360)) {
            break;
        }
        let testResult__90: (std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>) = temper_core::ListedTrait::get( & testResults__80, i__89);
        let failureMessages__91: temper_core::List<std::sync::Arc<String>> = testResult__90.value();
        t___363 = testResult__90.key();
        let name__92: std::sync::Arc<String> = escapeXml__41(t___363.clone());
        let basics__93: std::sync::Arc<String> = std::sync::Arc::new(format!("name='{}' classname='{}' time='0.0'", name__92.clone(), name__92.clone()));
        if temper_core::ListedTrait::is_empty( & failureMessages__91) {
            writeLine__81(std::sync::Arc::new(format!("    <testcase {} />", basics__93.clone())));
        } else {
            writeLine__81(std::sync::Arc::new(format!("    <testcase {}>", basics__93.clone())));
            #[derive(Clone)]
            struct ClosureGroup___4 {}
            impl ClosureGroup___4 {
                fn fn__351(& self, it__95: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                    let it__95 = it__95.to_arc_string();
                    return it__95.clone();
                }
            }
            let closure_group = ClosureGroup___4 {};
            let fn__351 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | it__95: std::sync::Arc<String> | closure_group.fn__351(it__95))
            };
            t___369 = temper_core::listed::join( & failureMessages__91, std::sync::Arc::new(", ".to_string()), & ( * fn__351.clone()));
            let message__94: std::sync::Arc<String> = escapeXml__41(t___369.clone());
            writeLine__81(std::sync::Arc::new(format!("      <failure message='{}' />", message__94.clone())));
            writeLine__81(std::sync::Arc::new("    </testcase>".to_string()));
        }
        i__89 = i__89.wrapping_add(1);
    }
    writeLine__81(std::sync::Arc::new("  </testsuite>".to_string()));
    writeLine__81(std::sync::Arc::new("</testsuites>".to_string()));
}
pub fn run_test_cases(testCases__96: impl temper_core::ToList<(std::sync::Arc<String>, std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync>)>) -> std::sync::Arc<String> {
    let testCases__96 = testCases__96.to_list();
    let report__98: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
    let mut t___345: temper_core::List<(std::sync::Arc<String>, temper_core::List<std::sync::Arc<String>>)> = process_test_cases(testCases__96.clone());
    #[derive(Clone)]
    struct ClosureGroup___5 {
        report__98: std::sync::Arc<std::sync::RwLock<String>>
    }
    impl ClosureGroup___5 {
        fn fn__343(& self, line__99: impl temper_core::ToArcString) {
            let line__99 = line__99.to_arc_string();
            temper_core::string::builder::append( & self.report__98, line__99.clone());
            temper_core::string::builder::append( & self.report__98, "\x0a");
        }
    }
    let closure_group = ClosureGroup___5 {
        report__98: report__98.clone()
    };
    let fn__343 = {
        let closure_group = closure_group.clone();
        std::sync::Arc::new(move | line__99: std::sync::Arc<String> | closure_group.fn__343(line__99))
    };
    report_test_results(t___345.clone(), fn__343.clone());
    return temper_core::string::builder::to_string( & report__98);
}
pub fn run_test(testFun__100: std::sync::Arc<dyn Fn (Test) -> temper_core::Result<()> + std::marker::Send + std::marker::Sync>) -> temper_core::Result<()> {
    let test__102: Test = Test::new();
    'ok___2620: {
        'orelse___126: {
            match testFun__100(test__102.clone()) {
                Ok(x) => x,
                _ => break 'orelse___126
            };
            break 'ok___2620;
        }
        #[derive(Clone)]
        struct ClosureGroup___6 {}
        impl ClosureGroup___6 {
            fn fn__337(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bubble during test running".to_string());
            }
        }
        let closure_group = ClosureGroup___6 {};
        let fn__337 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__337())
        };
        test__102.assert(false, fn__337.clone());
    }
    test__102.soft_fail_to_hard() ? ;
    return Ok(());
}
