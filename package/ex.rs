#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tari_template_lib::prelude::*;
use tari_template_lib::template_dependencies::*;
#[allow(non_snake_case)]
pub mod SuperBruPool_template {
    use ::tari_template_lib::template_dependencies::*;
    use super::*;
    use tari_template_abi::rust::collections::HashMap;
    #[serde(crate = "self::serde")]
    pub struct SuperBruPool {
        treasury: Vault,
        team_1: String,
        team_2: String,
        prediction_difference: HashMap<ComponentAddress, i32>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SuperBruPool {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "SuperBruPool",
                "treasury",
                &self.treasury,
                "team_1",
                &self.team_1,
                "team_2",
                &self.team_2,
                "prediction_difference",
                &&self.prediction_difference,
            )
        }
    }
    impl SuperBruPool {
        pub fn create_pool(
            token_symbol: String,
            team_1: String,
            team_2: String,
            num_players: u64,
        ) -> Self {
            let members = ResourceBuilder::fungible(&token_symbol)
                .initial_supply(num_players)
                .build_bucket();
            Self {
                treasury: Vault::from_bucket(members),
                team_1,
                team_2,
                prediction_difference: HashMap::new(),
            }
        }
        pub fn make_prediction(&mut self, difference: i64, membership: Bucket) {
            if membership.resource_address() != treasury.resource_address {
                ::core::panicking::panic_fmt(format_args!("error1"));
            }
            if membership.value() < 1 {
                ::core::panicking::panic_fmt(format_args!("error2"));
            }
            membership.burn();
            prediction_difference.insert(CallerContext.public_key, difference);
        }
        pub fn take_free_coins(&mut self) -> Bucket {
            debug("Withdrawing 1 coins from faucet");
            self.treasury.withdraw(Amount::new(1))
        }
    }
    impl ::tari_template_lib::component::interface::ComponentInterface for SuperBruPool {
        type Component = SuperBruPoolComponent;
        fn create_with_options(
            self,
            access_rules: ::tari_template_lib::auth::AccessRules,
            component_id: Option<::tari_template_lib::Hash>,
        ) -> Self::Component {
            let address = engine().create_component(self, access_rules, component_id);
            SuperBruPoolComponent { address }
        }
    }
    #[serde(transparent, crate = "self::serde")]
    pub struct SuperBruPoolComponent {
        address: tari_template_lib::models::ComponentAddress,
    }
    impl ::tari_template_lib::component::interface::ComponentInstanceInterface
    for SuperBruPoolComponent {
        fn set_access_rules(self, rules: tari_template_lib::auth::AccessRules) -> Self {
            engine().component_manager(self.address).set_access_rules(rules);
            self
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SuperBruPool_abi() -> *mut u8 {
    use ::tari_template_lib::template_dependencies::{
        encode_with_len, ArgDef, FunctionDef, TemplateDef, Type, wrap_ptr,
    };
    let template = TemplateDef {
        template_name: "SuperBruPool".to_string(),
        functions: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                FunctionDef {
                    name: "create_pool".to_string(),
                    arguments: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ArgDef {
                                name: "token_symbol".to_string(),
                                arg_type: Type::String,
                            },
                            ArgDef {
                                name: "team_1".to_string(),
                                arg_type: Type::String,
                            },
                            ArgDef {
                                name: "team_2".to_string(),
                                arg_type: Type::String,
                            },
                            ArgDef {
                                name: "num_players".to_string(),
                                arg_type: Type::U64,
                            },
                        ]),
                    ),
                    output: Type::Other {
                        name: "SuperBruPoolComponent".to_string(),
                    },
                    is_mut: false,
                },
                FunctionDef {
                    name: "make_prediction".to_string(),
                    arguments: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ArgDef {
                                name: "self".to_string(),
                                arg_type: Type::Other {
                                    name: "&mut self".to_string(),
                                },
                            },
                            ArgDef {
                                name: "difference".to_string(),
                                arg_type: Type::I64,
                            },
                            ArgDef {
                                name: "membership".to_string(),
                                arg_type: Type::Other {
                                    name: "Bucket".to_string(),
                                },
                            },
                        ]),
                    ),
                    output: Type::Unit,
                    is_mut: true,
                },
                FunctionDef {
                    name: "take_free_coins".to_string(),
                    arguments: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ArgDef {
                                name: "self".to_string(),
                                arg_type: Type::Other {
                                    name: "&mut self".to_string(),
                                },
                            },
                        ]),
                    ),
                    output: Type::Other {
                        name: "Bucket".to_string(),
                    },
                    is_mut: true,
                },
            ]),
        ),
    };
    let buf = encode_with_len(&template);
    wrap_ptr(buf)
}
#[no_mangle]
pub unsafe extern "C" fn SuperBruPool_main(
    call_info: *mut u8,
    call_info_len: usize,
) -> *mut u8 {
    use ::tari_template_lib::{
        template_dependencies::{decode_exact, encode_with_len, CallInfo, wrap_ptr},
        init_context, panic_hook::register_panic_hook,
    };
    register_panic_hook();
    if call_info.is_null() {
        ::core::panicking::panic_fmt(format_args!("call_info is null"));
    }
    let call_data = unsafe {
        Vec::from_raw_parts(call_info, call_info_len, call_info_len)
    };
    let call_info: CallInfo = decode_exact(&call_data)
        .expect("Failed to decode CallArgs");
    init_context(&call_info);
    engine()
        .emit_log(
            LogLevel::Debug,
            {
                let res = ::alloc::fmt::format(
                    format_args!(
                        "Dispatcher called with function {0}",
                        call_info.func_name,
                    ),
                );
                res
            },
        );
    let result;
    match call_info.func_name.as_str() {
        "create_pool" => {
            match (&call_info.args.len(), &4usize) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!(
                                    "Call had unexpected number of args. Got = {0} expected = {1}",
                                    call_info.args.len(),
                                    4usize,
                                ),
                            ),
                        );
                    }
                }
            };
            let arg_0 = decode_exact::<String>(&call_info.args[0usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        0usize,
                        "create_pool",
                        e,
                    ),
                ));
            let arg_1 = decode_exact::<String>(&call_info.args[1usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        1usize,
                        "create_pool",
                        e,
                    ),
                ));
            let arg_2 = decode_exact::<String>(&call_info.args[2usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        2usize,
                        "create_pool",
                        e,
                    ),
                ));
            let arg_3 = decode_exact::<u64>(&call_info.args[3usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        3usize,
                        "create_pool",
                        e,
                    ),
                ));
            let rtn = SuperBruPool_template::SuperBruPool::create_pool(
                arg_0,
                arg_1,
                arg_2,
                arg_3,
            );
            let rtn = engine()
                .create_component(
                    rtn,
                    ::tari_template_lib::auth::AccessRules::with_default_allow(),
                    None,
                );
            result = encode_with_len(&rtn);
        }
        "make_prediction" => {
            match (&call_info.args.len(), &3usize) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!(
                                    "Call had unexpected number of args. Got = {0} expected = {1}",
                                    call_info.args.len(),
                                    3usize,
                                ),
                            ),
                        );
                    }
                }
            };
            let component_address = decode_exact::<
                ::tari_template_lib::models::ComponentAddress,
            >(&call_info.args[0usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode component instance for function \'{0}\': {1}",
                        "make_prediction",
                        e,
                    ),
                ));
            let component_manager = engine().component_manager(component_address);
            let mut state = component_manager
                .get_state::<SuperBruPool_template::SuperBruPool>();
            let arg_1 = decode_exact::<i64>(&call_info.args[1usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        1usize,
                        "make_prediction",
                        e,
                    ),
                ));
            let arg_2 = decode_exact::<Bucket>(&call_info.args[2usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode argument at position {0} for function \'{1}\': {2}",
                        2usize,
                        "make_prediction",
                        e,
                    ),
                ));
            let rtn = SuperBruPool_template::SuperBruPool::make_prediction(
                &mut state,
                arg_1,
                arg_2,
            );
            result = encode_with_len(&rtn);
            component_manager.set_state(state);
        }
        "take_free_coins" => {
            match (&call_info.args.len(), &1usize) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!(
                                    "Call had unexpected number of args. Got = {0} expected = {1}",
                                    call_info.args.len(),
                                    1usize,
                                ),
                            ),
                        );
                    }
                }
            };
            let component_address = decode_exact::<
                ::tari_template_lib::models::ComponentAddress,
            >(&call_info.args[0usize])
                .unwrap_or_else(|e| ::core::panicking::panic_fmt(
                    format_args!(
                        "failed to decode component instance for function \'{0}\': {1}",
                        "take_free_coins",
                        e,
                    ),
                ));
            let component_manager = engine().component_manager(component_address);
            let mut state = component_manager
                .get_state::<SuperBruPool_template::SuperBruPool>();
            let rtn = SuperBruPool_template::SuperBruPool::take_free_coins(&mut state);
            result = encode_with_len(&rtn);
            component_manager.set_state(state);
        }
        _ => ::core::panicking::panic_fmt(format_args!("invalid function name")),
    };
    wrap_ptr(result)
}
