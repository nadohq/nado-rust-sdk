pub use spot_engine::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod spot_engine {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addProduct"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quoteId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sizeIncrement"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minSize"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("config"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertUtilization"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getClearinghouse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getClearinghouse"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConfig"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCoreRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCoreRisk"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("healthType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.HealthType",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IProductEngine.CoreRisk",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEndpoint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEndpoint"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEngineType"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEngineType"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum IProductEngine.EngineType",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHealthContribution"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHealthContribution",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("healthType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.HealthType",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("health"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNlpUnlockedBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNlpUnlockedBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getProductIds"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISpotEngine.BalanceNormalized",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawState"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRisk"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct RiskHelper.Risk"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSlots"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlots"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balancesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("statesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStateAndBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStateAndBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getToken"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("balance"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTotalBalances"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deposits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("borrows"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_offchainExchange"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_endpoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_admin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("manualAssert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("manualAssert"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalDeposits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("totalBorrows"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("config"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Config",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setState"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("state"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ISpotEngine.State"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("socializeSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("socializeSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tryUnlockNlpBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tryUnlockNlpBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISpotEngine.Balance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedUpdateProductTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedUpdateProductTx",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISpotEngine.UpdateProductTx",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISpotEngine.UpdateProductTx",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("productId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quoteDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int128"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("rawTxn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateQuoteFromInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateQuoteFromInsurance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("insurance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateRisk"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct RiskHelper.RiskStore",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateStates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateStates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("dt"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalanceUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BalanceUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InterestPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("InterestPayment"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("depositRateMultiplierX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("borrowRateMultiplierX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProductUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProductUpdate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SpotBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SpotBalance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "lastCumulativeMultiplierX18",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SPOTENGINE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaK1\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02wW`\x005`\xE0\x1C\x80c\x8A\x1DC\xC9\x11a\x01`W\x80c\xE0\xB0b\x1F\x11a\0\xD8W\x80c\xED\xF0&S\x11a\0\x8CW\x80c\xF3\x9E\xEB\x10\x11a\0qW\x80c\xF3\x9E\xEB\x10\x14a\x07\xBFW\x80c\xF8\xA4.Q\x14a\x07\xD2W\x80c\xFA\xB2\xC4i\x14a\x07\xE5W`\0\x80\xFD[\x80c\xED\xF0&S\x14a\x07`W\x80c\xF2\xFD\xE3\x8B\x14a\x07\xACW`\0\x80\xFD[\x80c\xE3Cs\x8C\x11a\0\xBDW\x80c\xE3Cs\x8C\x14a\x05\x8DW\x80c\xECzy\xC9\x14a\x06eW\x80c\xEC\xD9\xCB\xA8\x14a\x06\xFCW`\0\x80\xFD[\x80c\xE0\xB0b\x1F\x14a\x05YW\x80c\xE34\xBE3\x14a\x05lW`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x11a\x01/W\x80c\xC5V\x07\xB5\x11a\x01\x14W\x80c\xC5V\x07\xB5\x14a\x04\xE5W\x80c\xC9\xFE\x9A\xC3\x14a\x04\xF8W\x80c\xCB\xD0\x80\x8D\x14a\x05\x0BW`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x14a\x04\xC3W\x80c\xB1\xCB\x0FB\x14a\x04\xD4W`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x045W\x80c\x8D\xA5\xCB[\x14a\x04tW\x80c\xA6z\xC3\"\x14a\x04\x85W\x80c\xADs;\x8E\x14a\x04\xB0W`\0\x80\xFD[\x80cF\x04\xD1\x9B\x11a\x01\xF3W\x80cqP\x18\xA6\x11a\x01\xC2W\x80c\x7F\xA2\x9DI\x11a\x01\xA7W\x80c\x7F\xA2\x9DI\x14a\x03\xE9W\x80c\x87\x1D\t\x12\x14a\x03\xFCW\x80c\x896\xF7\xCD\x14a\x04\"W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x03\xCEW\x80c|\x1E\x14\x87\x14a\x03\xD6W`\0\x80\xFD[\x80cF\x04\xD1\x9B\x14a\x03\x84W\x80cGB\x8E{\x14a\x03\x93W\x80cJ\xC8\xD8\xC1\x14a\x03\xA8W\x80cVw\x8D?\x14a\x03\xBBW`\0\x80\xFD[\x80c%\xBC\xE8\xC9\x11a\x02JW\x80c0\x97+P\x11a\x02/W\x80c0\x97+P\x14a\x03\x17W\x80c9\x1B{?\x14a\x03*W\x80cE\xBE~\xD6\x14a\x03=W`\0\x80\xFD[\x80c%\xBC\xE8\xC9\x14a\x02\xEEW\x80c+\xAFW\xD3\x14a\x03\x01W`\0\x80\xFD[\x80c\x14YEz\x14a\x02|W\x80c\x15<\xA6\xC0\x14a\x02\x91W\x80c\x19\xA2\xAC\x88\x14a\x02\xA4W\x80c!\x92Ai\x14a\x02\xCEW[`\0\x80\xFD[a\x02\x8Fa\x02\x8A6`\x04a;]V[a\x07\xFAV[\0[a\x02\x8Fa\x02\x9F6`\x04a<\x01V[a\n\xDEV[a\x02\xB7a\x02\xB26`\x04a<8V[a\x0B\x7FV[`@Q\x90Q`\x0F\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xE1a\x02\xDC6`\x04a<QV[a\r\x02V[`@Qa\x02\xC5\x91\x90a<jV[a\x02\x8Fa\x02\xFC6`\x04a=dV[a\r\xA0V[a\x03\ta\x0E,V[`@Qa\x02\xC5\x92\x91\x90a>*V[a\x02\x8Fa\x03%6`\x04a>\x94V[a\x10WV[a\x02\x8Fa\x0386`\x04a?\0V[a\x128V[a\x03la\x03K6`\x04a?5V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[`\0`@Qa\x02\xC5\x91\x90a?fV[a\x03\x9Ba\x12_V[`@Qa\x02\xC5\x91\x90a?\x8EV[a\x02\x8Fa\x03\xB66`\x04a?5V[a\x12\xE3V[a\x02\xB7a\x03\xC96`\x04a<8V[a\x13zV[a\x02\x8Fa\x13\xBCV[a\x02\xB7a\x03\xE46`\x04a?\xD8V[a\x13\xC8V[a\x02\x8Fa\x03\xF76`\x04a@\x02V[a\x14XV[a\x04\x0Fa\x04\n6`\x04a@PV[a\x14yV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xC5V[a\x02\x8Fa\x0406`\x04a<8V[a\x14\xF9V[a\x04Ha\x04C6`\x04a@sV[a\x16\xD3V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x02\xC5V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x04\x98a\x04\x936`\x04a?5V[a\x17HV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[a\x02\x8Fa\x04\xBE6`\x04a@\xC4V[a\x17\xECV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x02\x8Fa\x04\xF36`\x04aA\xE4V[a\x19\xDCV[a\x02\x8Fa\x05\x066`\x04aB\x0FV[a\x1B\rV[a\x02\x8Fa\x05\x196`\x04aB\x81V[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\x8Fa\x05g6`\x04aB\x81V[a\x1E\xE0V[a\x05\x7Fa\x05z6`\x04a?\xD8V[a\x1F\x98V[`@Qa\x02\xC5\x92\x91\x90aB\xC1V[a\x06Xa\x05\x9B6`\x04a?5V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x82\x01T\x80\x84\x0B``\x83\x01R\x84\x90\x04\x83\x0B`\x80\x82\x01R`\x03\x90\x91\x01T\x80\x83\x0B`\xA0\x83\x01R\x92\x90\x92\x04\x90\x0B`\xC0\x82\x01R\x90V[`@Qa\x02\xC5\x91\x90aC\rV[a\x06\xEFa\x06s6`\x04a?5V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xC5\x91\x90aCtV[a\x07\x0Fa\x07\n6`\x04a?5V[a RV[`@Qa\x02\xC5\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x02\xB7a\x07n6`\x04a?\xD8V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x81R`l\x84R\x81\x81 \x92\x81R\x91\x83R\x90\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[a\x02\x8Fa\x07\xBA6`\x04aC\xAFV[a \x86V[a\x04\x0Fa\x07\xCD6`\x04aC\xCCV[a!\x0EV[a\x02\x8Fa\x07\xE06`\x04aC\xF1V[a\"7V[`@\x80Q`l\x81R`k` \x82\x01R\x01a\x02\xC5V[a\x08\x06\x85\x85\x84\x84a#\xEEV[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x84\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x84\x86\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x87\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x89\x01\x82\x81R`\xA0\x80\x8B\x01\x84\x81R`\0`\xC0\x8D\x01\x81\x81R\x81\x80R`j\x8BR\x9CQ\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9D\x16\x17\x90\x9BU\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x97\x88\x16`\x01`\x80\x1B\x91\x89\x16\x82\x02\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x08U\x95Q\x91Q\x91\x87\x16\x91\x87\x16\x86\x02\x91\x90\x91\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\tU\x96Q\x98Q\x98\x85\x16\x98\x85\x16\x84\x02\x98\x90\x98\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\nU\x88Q\x95\x86\x01\x89Rc;\x9A\xCA\0\x80\x87R\x86\x86\x01\x81\x81R\x87\x8B\x01\x82\x81R\x88\x85\x01\x92\x83R\x88\x8B\x01\x84\x81R\x8A\x80R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x89R\x98Q\x7F\xEA#\xA1\xF6\x04v\xEF\x06\xB4\xF1 \x0F\xB4\xED3H\t=rb\xFAI8\xFF\x94|6\xDB\x93\xDC,\xC2\x80T\x93Q\x92Q\x94Q\x9AQ\x89\x16\x90\x97\x02c\xFF\xFF\xFF\xFF\x9A\x8B\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x95\x8C\x16`\x01`@\x1B\x02\x95\x90\x95\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x8C\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x9B\x90\x92\x16\x9A\x90\x9A\x17\x92\x90\x92\x17\x16\x17\x17\x90\x93\x16\x94\x90\x94\x17\x90U\x85Q\x94\x85\x01\x86R\x80\x85R\x90\x84\x01R\x92\x82\x01\x81\x90R\x91\x81\x01\x82\x90Ra\nT\x91\x90a%\x8BV[`g\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x82\x04\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x02\x19\x90\x91\x16\x90U`@Q\x90\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[`@Q\x80\x91\x03\x90\xFD[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q` \x81\x01\x90\x91R`\0\x81R`\0\x82\x81R`m` R`@\x90 [`\x01\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01`@\x1B\x90\x92\x04\x16\x10\x80\x15a\x0B\xFEWPa\x0B\xC7a%\xE7V[`\x01\x82\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x84\x90R`@\x90 \x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11\x15[\x15a\x0C\xE6W`\x01\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x82\x90R`@\x81 T`\x02\x83\x01\x80T`\x0F\x92\x83\x0B\x93\x91\x92a\x0CB\x91\x85\x91\x0BaD\xADV[\x82T`\x01`\x01`\x80\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP`\x01\x81\x81\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R` \x86\x90R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x82U\x94\x01\x80T\x90\x94\x16\x90\x93U\x81T\x04\x90\x91\x16\x90`\x08a\x0C\xBA\x83aD\xFCV[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa\x0B\x9DV[`@\x80Q` \x81\x01\x90\x91R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x92\x91PPV[a\r\x8B`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x84Q`\xE0\x81\x01\x86R\x82\x81R\x90\x81\x01\x82\x90R\x93\x84\x01\x81\x90R``\x84\x81\x01\x82\x90R`\x80\x85\x01\x82\x90R\x91\x84\x01\x81\x90R`\xC0\x84\x01R\x90\x91\x90\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[a\r\x9A6\x83\x90\x03\x83\x01\x83aE#V[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x86\x16a\r\xB0W`\0\x80\xFD[a\r\xCB\x86\x86\x86\x86a\r\xC66\x87\x90\x03\x87\x01\x87aF+V[a&ZV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`j` R`@\x90 \x82\x90a\r\xEC\x82\x82aFGV[PP`@\x80Q`\x80\x81\x01\x82Rg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82R` \x82\x01R`\0\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0E$\x90\x87\x90a%\x8BV[PPPPPPV[``\x80`\0`g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xA9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0ElW\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xCAWa\x0E\xCAa@\xE1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x10Wa\x0F\x10a@\xE1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x10QW`\0\x82\x82\x81Q\x81\x10a\x0F\\Wa\x0F\\aGzV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0F\xCB\x92\x91a*\xFB\x16V[\x86\x84\x81Q\x81\x10a\x0F\xDDWa\x0F\xDDaGzV[` \x02` \x01\x01\x81\x81Qa\x0F\xF1\x91\x90aD\xADV[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x10\x11\x93P\x90\x91\x0B\x90a*\xFBV[\x85\x84\x81Q\x81\x10a\x10#Wa\x10#aGzV[` \x02` \x01\x01\x81\x81Qa\x107\x91\x90aD\xADV[`\x0F\x0B\x90RPa\x10J\x91P\x82\x90PaG\x90V[\x90Pa\x0F?V[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x121W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x10\x89Wa\x10\x89aGzV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`k\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x11\x15Wa\x11\x15aGzV[\x90P` \x02\x01` \x81\x01\x90a\x11*\x91\x90aG\xA9V[`\x0F\x0Ba\x11K\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x11\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x11\xA7Wa\x11\xA7aGzV[\x90P` \x02\x01` \x81\x01\x90a\x11\xBC\x91\x90aG\xA9V[`\x0F\x0Ba\x11\xDD\x82` \x01Q\x83``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x12\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[PPP\x80a\x12*\x90aG\xC6V[\x90Pa\x10ZV[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x12Y\x82\x82aFGV[PPPPV[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\xD9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x12\x9CW\x90P[PPPPP\x90P\x90V[`\0a\x12\xF0\x82`\x01a\x1F\x98V[P\x90P`\0a\x13\x13\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x135\x83` \x01Q\x84``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x121W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x13\x92\x82a\x0B\x7FV[PP`\0\x90\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x90V[a\x13\xC6`\0a+~V[V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x14O\x82\x82a+\xD0V[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\x12Y\x82\x82aG\xE2V[`\0\x80a\x01\0a\x14\x87a,$V[a\x14\x91\x91\x90aH\x83V[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x14\xF1W`\0a\x14\xB5\x86\x83a,sV[\x90P\x80`\0\x03a\x14\xC5WPa\x14\xDFV[a\x14\xD1\x81\x83\x88\x88a-<V[a\x14\xDB\x90\x85aD\xADV[\x93PP[\x80a\x14\xE9\x81aH\xA6V[\x91PPa\x14\x96V[PP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0a\x15Ha\x12_V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x16\xCEW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x15|Wa\x15|aGzV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`l\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x16\x01\x90\x83\x90a+\xD0V[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x16\xBAW\x81Q`@\x83\x01Q`\0\x91a\x16*\x91`\x0F\x0B\x90a*\xFBV[`@\x84\x01Q\x83Q\x91\x92Pa\x16K\x91a\x16B\x90\x84aD\xADV[`\x0F\x0B\x90a-\x97V[`\x0F\x0B\x80\x84R`\0\x12a\x16]W`\0\x80\xFD[a\x16{\x83` \x01Q\x83`\0\x01Q`\x0F\x0Ba-\x97\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x16\x8C\x91\x90aD\xADV[`\x0F\x0B\x90RP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x16\xAE\x90\x85\x90\x89\x90a.\0V[a\x16\xB8\x84\x84a%\x8BV[P[PPP\x80a\x16\xC7\x90aG\xC6V[\x90Pa\x15MV[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x16\xF9\x84a.VV[\x90P`\0a\x17\x07\x85\x87a.\xABV[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x177\x84`\x01\x88a.\xC4V[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a\x17\xE2\x92\x85\x92a\x17\xB5\x92\x91\x90a*\xFB\x16V[a\x17\xD3\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x17\xDD\x91\x90aH\xBFV[a/[V[a\x17A\x90\x83aI\x0FV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\"V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x18\x8Bb\x01Q\x80`\x07aI:V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x18\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0[`gTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x16\xCEW`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x19\tWa\x19\taGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`\x98\x19\x81\x01a\x19>WPa\x19\xCAV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90a\x19\x9CW\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a\x19\xB2WPPa\x19\xCAV[a\x19\xBD\x82\x82\x87a/wV[a\x19\xC7\x82\x82a%\x8BV[PP[\x80a\x19\xD4\x81aH\xA6V[\x91PPa\x18\xDEV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1A\x08WP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x1ABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\"V[`\0a\x1B\x88\x82\x84\x01\x84aE#V[`\x80\x81\x01Q\x81Q\x91\x92P\x90c\xFF\xFF\xFF\xFF\x16\x15a\x1EFW\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1B\xCAWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a\x1B\xFDWP``\x82\x01QQ\x82Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x1C7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T`\x03\x81\x81\x0B\x83Rd\x01\0\0\0\0\x82\x04\x81\x0B\x83\x86\x01\x90\x81R`\x01`@\x1B\x83\x04\x82\x0B\x84\x86\x01\x90\x81R`\x01``\x1B\x84\x04\x83\x0B``\x80\x87\x01\x91\x82R`\x01`\x80\x1B\x90\x95\x04`\x0F\x0B`\x80\x87\x01R\x89Q\x84\x0B\x86R\x96\x89\x01Q\x83\x0B\x90\x91R\x93\x87\x01Q\x81\x0B\x90\x93R\x85\x01Q\x90\x91\x0B\x90\x91R\x90P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x92\x83R`@\x90\x81\x90 \x84Q\x81T\x94\x86\x01Q\x92\x86\x01Q``\x87\x01Q`\x80\x90\x97\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x97\x86\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x87\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x87\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x98\x16\x93\x90\x96\x16\x92\x90\x92\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x92\x90\x92\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90Ua\x1D\xBDa3SV[\x83Q` \x85\x01Q`@\x80\x87\x01Q\x90Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R`\x0F\x91\x82\x0B`D\x84\x01R\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E@W=`\0\x80>=`\0\xFD[PPPPP[P``\x81\x81\x01Q\x91Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17`\x01\x83\x01U\x92\x84\x01Q`\x80\x85\x01Q\x90\x83\x16\x90\x83\x16\x84\x02\x17`\x02\x82\x01U`\xA0\x84\x01Q`\xC0\x90\x94\x01Q\x93\x82\x16\x93\x90\x91\x16\x90\x91\x02\x91\x90\x91\x17`\x03\x90\x91\x01UPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90`\x98\x19\x01a\x1FHWa\x1FH\x83\x83a3\xC8V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra\x1F\x83\x82\x82\x85a5BV[a\x1F\x8E\x85\x85\x83a.\0V[a\x121\x85\x83a%\x8BV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a D\x81\x83a+\xD0V[\x93P\x93PPP[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\r\x9A\x82a.VV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\"V[a!\x0B\x81a+~V[PV[`@\x80Q`\x80\x81\x01\x82R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a!\xCD\x83\x83a+\xD0V[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15a\"\x16W`\0a!\xFBa!\xF4\x87a!\xEF\x85aIjV[a6AV[`\0a/[V[\x90Pa\"\x07\x81\x87aH\xBFV[\x95Pa\"\x14\x84\x84\x83a5BV[P[a\"!`\0\x84a%\x8BV[a\"-`\0\x87\x84a.\0V[P\x92\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16a\"tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x86\x86R`l\x85R\x83\x86 \x8A\x87R\x85R\x83\x86 \x84Q\x80\x87\x01\x86R\x90T\x84\x0B\x81R\x8A\x87R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x86R\x95\x84\x90 \x84Q\x95\x86\x01\x90\x94R\x92T\x90\x91\x0B\x83R\x93\x90\x92\x91\x90`\x98\x19\x01a#\xA2Wa#\xA2\x87\x87a3\xC8V[a#\xAD\x84\x83\x88a5BV[a#\xB8\x83\x82\x87a5BV[a#\xC3\x88\x88\x84a.\0V[a#\xCF`\0\x88\x83a.\0V[a#\xD9\x88\x85a%\x8BV[a#\xE4`\0\x84a%\x8BV[PPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a$\x0EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a$(WP0;\x15\x80\x15a$(WP`\0T`\xFF\x16`\x01\x14[a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\"V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a$\xBDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a$\xC5a6VV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua$\xE9\x82a \x86V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x121W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\n\xCFV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua%\xE3\x82a6\xC9V[PPV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&U\x91\x90aI\x90V[\x90P\x90V[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a&\x83WPc;\x9A\xCA\0\x81`@\x01Q`\x03\x0B\x13\x15[\x80\x15a&\x9DWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a&\xB4WPc;\x9A\xCA\0\x81``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a&\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x87\x81\x16`\0\x81\x81R` \x93\x84R`@\x90\x81\x90 \x85Q\x81T\x95\x87\x01Q\x87\x84\x01Q``\x89\x01Q`\x80\x90\x99\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x99\x88\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x89\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x89\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x9A\x16\x94\x90\x98\x16\x93\x90\x93\x17\x97\x90\x97\x17\x16\x94\x90\x94\x17\x94\x90\x94\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91U`fT\x90QcC\xB1j\x11`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x87b\xD4\"\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a(\x11W=`\0\x80>=`\0\xFD[PP`g\x80T`\x01\x80\x82\x01\x83U`\0\x83\x81R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x84\x04\x01\x80T`\x07\x90\x94\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x95\x16\x94\x8D\x16\x02\x93\x90\x93\x17\x90\x92U\x91T\x90\x93Pa(}\x92PaI\xADV[\x90P[\x80\x15a*@W`ga(\x93`\x01\x83aI\xADV[\x81T\x81\x10a(\xA3Wa(\xA3aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16`g\x82\x81T\x81\x10a(\xE2Wa(\xE2aGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15a*)W`\0`g\x82\x81T\x81\x10a)!Wa)!aGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`ga)T`\x01\x84aI\xADV[\x81T\x81\x10a)dWa)daGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16`g\x83\x81T\x81\x10a)\x9DWa)\x9DaGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`g`\x01\x84a)\xE0\x91\x90aI\xADV[\x81T\x81\x10a)\xF0Wa)\xF0aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa*.V[a*@V[\x80a*8\x81aI\xC4V[\x91PPa(\x80V[Pa*Ia3SV[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`\x0F\x85\x81\x0B`D\x83\x01R\x84\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xC0W=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x88\x16\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x92P` \x01\x90Pa\n\xCFV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a+=WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a+vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x82Q`\x0F\x0B\x81\x12\x15a+\xF4WP\x82Qa+\xFBV[P` \x83\x01Q[`@\x80Q` \x81\x01\x90\x91R\x83Q\x81\x90a,\x17\x90`\x0F\x0B\x84a*\xFBV[`\x0F\x0B\x90R\x94\x93PPPPV[`g\x80T`\0\x91\x90a,8\x90`\x01\x90aI\xADV[\x81T\x81\x10a,HWa,HaGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a+vW`\0`g\x82\x81T\x81\x10a,\x97Wa,\x97aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a,\xCD\x91\x90aI\xDBV[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a-\tWPa,\xEE\x85`\x01aI\xFEV[a,\xFA\x90a\x01\0aI\xDBV[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a-)Wa-\x1Aa\x01\0\x82aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a-4\x81aG\x90V[\x91PPa,xV[`\0\x80a-K\x85a\x01\0aI\xDBV[\x90P[\x85\x15a-\x8EW`\x01\x86\x16\x15a-uWa-h\x81\x85\x85a7\x05V[a-r\x90\x83aD\xADV[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a-\x86\x81aH\xA6V[\x91PPa-NV[P\x94\x93PPPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a-\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a+\x12Wa+\x12aHmV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x90\x91U`\x0F\x0B\x15\x15a.L\x83\x85\x83a7\xBEV[a\x12Y\x84\x84a8{V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\r\x9A\x82\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0ya8\xBEV[`\0\x80a.\xB8\x84\x84a\x13\xC8V[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15a.\xDAWa.\xDAa?PV[\x03a.\xEEWPg\r\xE0\xB6\xB3\xA7d\0\0a\x17AV[`\0\x80\x84`\x0F\x0B\x12a/'W`\0\x83`\x02\x81\x11\x15a/\x0EWa/\x0Ea?PV[\x14a/\x1DW\x84`@\x01Qa/ V[\x84Q[\x90Pa/SV[`\0\x83`\x02\x81\x11\x15a/;Wa/;a?PV[\x14a/JW\x84``\x01Qa/PV[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a/pW\x81a\x17AV[P\x90\x91\x90PV[`\0\x80a/\x98\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a/\xBA\x85` \x01Q\x86``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a/\xCC`\x0F\x83\x90\x0B\x84a-\x97V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x83\x01T\x80\x86\x0B``\x84\x01R\x81\x90\x04\x85\x0B`\x80\x83\x01R`\x03\x90\x92\x01T\x80\x85\x0B`\xA0\x83\x01R\x91\x90\x91\x04\x83\x0B`\xC0\x82\x01R\x93\x94P\x91\x92\x91\x90\x84\x90\x0B\x83\x03a0cWP`\0a0\xFFV[\x81` \x01Q`\x0F\x0B\x84`\x0F\x0B\x12\x15a0\xA8Wa0\x97\x82` \x01Qa\x16B\x86\x85``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a0\xA1\x90\x82aD\xADV[\x90Pa0\xFFV[a0\xE3a0\xD5\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a0\xC6\x91\x90aH\xBFV[` \x85\x01Qa\x16B\x90\x88aH\xBFV[`\x80\x84\x01Q`\x0F\x0B\x90a*\xFBV[\x82``\x01Qa0\xF2\x91\x90aD\xADV[a0\xFC\x90\x82aD\xADV[\x90P[a1\x1Aa1\x0Fc\x01\xE13\x80a9\xD3V[`\x0F\x83\x90\x0B\x90a-\x97V[\x90Pa1;\x88a12\x83g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[`\x0F\x0B\x90a:LV[\x96P\x81`\xC0\x01Q\x92PPP`\0a1jg\r\xE0\xB6\xB3\xA7d\0\0\x87a1_\x91\x90aH\xBFV[`\x0F\x85\x90\x0B\x90a*\xFBV[\x90P`\0a1\x96a1\x8Bg\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0aH\xBFV[`\x0F\x84\x90\x0B\x90a*\xFBV[\x90P`\0a1\xB2a1\xA7\x83\x85aH\xBFV[`\x0F\x89\x90\x0B\x90a*\xFBV[` \x8B\x01Q\x90\x91Pa1\xC7\x90`\x0F\x0B\x89a*\xFBV[`\x0F\x0B` \x8B\x01R`\0a1\xE3\x83g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[\x8BQ\x90\x91Pa1\xF5\x90`\x0F\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B\x8CR\x82\x90\x0B\x15a2MWc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra2?\x8C\x82\x85a5BV[a2K\x8D`\0\x83a.\0V[P[\x84`\x0F\x0B`\0\x14a2\xE4W`\0a2ua2jc\x01\xE13\x80a9\xD3V[`\x0F\x88\x90\x0B\x90a-\x97V[\x90P`\0a2\x8F\x8Ca12\x84g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[` \x8E\x01Q\x90\x91Pa2\xA4\x90`\x0F\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B` \x8F\x01R\x8DQa2\xBB\x91\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B\x8ERa2\xCF\x90\x84\x90\x0B\x82a*\xFBV[\x92Pa2\xDF`\x0F\x8C\x90\x0B\x82a*\xFBV[\x9APPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8E\x16\x81R`\x01`\x01`\x80\x1B\x03\x8C\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8B\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPPV[`\0a3g`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&U\x91\x90aJ@V[`\x01\x19\x82\x01a3\xD5WPPV[a3\xDE\x82a\x0B\x7FV[P`\0\x81`\x0F\x0B\x13\x15a4\xC4W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R`\x0F\x85\x90\x0B\x93\x81\x01\x93\x84R\x92\x83R\x91\x90\x81\x01b\x05F\0a4#a%\xE7V[a4-\x91\x90aI\x0FV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x91R`\x01\x80\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x87\x81R`@\x82 \x87QQ\x81T\x90\x88\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x82\x16\x17\x82U\x97\x90\x91\x01Q\x94\x01\x80T\x94\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x90\x92U\x81T\x16\x91a4\x98\x83aD\xFCV[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\0\x81`\x0F\x0B\x12\x15a%\xE3W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x80\x82R\x82\x90\x82\x90a5\x05\x90\x83\x90aD\xADV[`\x0F\x0B\x90RP`\0\x92\x83R`m` R`@\x90\x92 \x91Q`\x02\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15a5pW\x81Q`@\x84\x01\x80Qa5e\x90\x83\x90aH\xBFV[`\x0F\x0B\x90RPa5\x8AV[\x81Q``\x84\x01\x80Qa5\x83\x90\x83\x90aD\xADV[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15a5\xA3WP\x82Qa5\xAAV[P` \x83\x01Q[\x82Q`\0\x90\x83\x90a5\xBE\x90`\x0F\x0B\x84a*\xFBV[a5\xC8\x91\x90aD\xADV[\x90P`\0\x81`\x0F\x0B\x13\x15a5\xDFW\x84Q\x91Pa5\xE7V[\x84` \x01Q\x91P[a5\xF5`\x0F\x82\x90\x0B\x83a-\x97V[`\x0F\x0B\x80\x85R`\0\x12\x15a6!W\x83Q`@\x86\x01\x80Qa6\x16\x90\x83\x90aD\xADV[`\x0F\x0B\x90RPa\x121V[\x83Q``\x86\x01\x80Qa64\x90\x83\x90aH\xBFV[`\x0F\x0B\x90RPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a/pW\x81a\x17AV[`\0Ta\x01\0\x90\x04`\xFF\x16a6\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x0B\"V[a\x13\xC6a:\xD4V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a7\x11\x85a.VV[\x90P`\0\x80a7 \x87\x87a.\xABV[\x91P\x91P`\0a71\x84\x84\x88a.\xC4V[\x90Pa7=\x82\x86aD\xADV[\x94P\x82`\x0F\x0B`\0\x14a7\xB3Wa7]g\r\xE0\xB6\xB3\xA7d\0\0`\x02aJ]V[`\x0F\x0B\x81`\x0F\x0B\x03a7\x86Wo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94PPPPPa\x17AV[`\x80\x84\x01Qa7\xA6\x90a7\x9D`\x0F\x86\x90\x0B\x84a*\xFBV[`\x0F\x0B\x90a*\xFBV[a7\xB0\x90\x86aD\xADV[\x94P[PPPP\x93\x92PPPV[\x80\x15a8\x1FWa7\xD0a\x01\0\x83aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a7\xFD\x91\x90aH\x83V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a8+a\x01\0\x83aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a8Y\x91\x90aH\x83V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x83\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x03\x81\x81\x0B\x80\x84Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x81\x04\x90\x93\x0B``\x82\x01R`\x01`\x80\x1B\x90\x92\x04`\x0F\x0B`\x80\x83\x01Ra9`\x90c;\x9A\xCA\0aJ]V[`\x0F\x0B\x82R` \x81\x01Qa9{\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x0B` \x83\x01R`@\x81\x01Qa9\x99\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x0B`@\x83\x01R``\x81\x01Qa9\xB7\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x92\x91PPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a:\x0CWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a:EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a:\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13a+vW\x80\x84\x16`\x0F\x0B\x15a:\xC0Wa:\xBD\x82\x86a*\xFBV[\x91P[a:\xCA\x85\x86a*\xFBV[\x94P`\x02\x02a:\x9BV[`\0Ta\x01\0\x90\x04`\xFF\x16a;?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x0B\"V[a\x13\xC63a+~V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x0BW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a;uW`\0\x80\xFD[\x855a;\x80\x81a;HV[\x94P` \x86\x015a;\x90\x81a;HV[\x93P`@\x86\x015a;\xA0\x81a;HV[\x92P``\x86\x015a;\xB0\x81a;HV[\x91P`\x80\x86\x015a;\xC0\x81a;HV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a;\xE2W`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a!\x0BW`\0\x80\xFD[\x805a;\xE2\x81a;\xE7V[`\0\x80`@\x83\x85\x03\x12\x15a<\x14W`\0\x80\xFD[a<\x1D\x83a;\xCEV[\x91P` \x83\x015a<-\x81a;\xE7V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a<JW`\0\x80\xFD[P5\x91\x90PV[`\0a\x01\xE0\x82\x84\x03\x12\x15a<dW`\0\x80\xFD[P\x91\x90PV[`\0a\x01\xE0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qa=\x02``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\x80\x83\x01Q\x80Q`\x03\x0Ba\x01@\x84\x01R` \x81\x01Q`\x03\x0Ba\x01`\x84\x01R`@\x81\x01Q`\x03\x0Ba\x01\x80\x84\x01R``\x81\x01Q`\x03\x0Ba\x01\xA0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xC0\x84\x01RP\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a<dW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15a=\x7FW`\0\x80\xFD[a=\x88\x88a;\xCEV[\x96Pa=\x96` \x89\x01a;\xCEV[\x95P`@\x88\x015a=\xA6\x81a;\xE7V[\x94P``\x88\x015a=\xB6\x81a;\xE7V[\x93Pa=\xC5\x89`\x80\x8A\x01a=RV[\x92P`\xA0a\x01_\x19\x82\x01\x12\x15a=\xDAW`\0\x80\xFD[Pa\x01`\x87\x01\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a>\x1FW\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a>=`@\x83\x01\x85a=\xECV[\x82\x81\x03` \x84\x01Ra\x14O\x81\x85a=\xECV[`\0\x80\x83`\x1F\x84\x01\x12a>aW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>yW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a KW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a>\xAAW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a>\xC2W`\0\x80\xFD[a>\xCE\x88\x83\x89\x01a>OV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a>\xE7W`\0\x80\xFD[Pa>\xF4\x87\x82\x88\x01a>OV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80a\x01\0\x83\x85\x03\x12\x15a?\x14W`\0\x80\xFD[a?\x1D\x83a;\xCEV[\x91Pa?,\x84` \x85\x01a=RV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a?GW`\0\x80\xFD[a\x17A\x82a;\xCEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a?\x88WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a?\xCCW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a?\xAAV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a?\xEBW`\0\x80\xFD[a?\xF4\x83a;\xCEV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a@\x16W`\0\x80\xFD[a@\x1F\x84a;\xCEV[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a@3W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a;\xE2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a@cW`\0\x80\xFD[\x825\x91Pa?,` \x84\x01a@AV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x88W`\0\x80\xFD[\x835\x92Pa@\x98` \x85\x01a;\xCEV[\x91Pa@\xA6`@\x85\x01a@AV[\x90P\x92P\x92P\x92V[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a!\x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\xD6W`\0\x80\xFD[\x815a\x17A\x81a@\xAFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aA(WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aA(WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14a;\xE2W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15aA\x83W`\0\x80\xFD[aA\x8Ba@\xF7V[\x90PaA\x96\x82aA_V[\x81RaA\xA4` \x83\x01aA_V[` \x82\x01RaA\xB5`@\x83\x01aA_V[`@\x82\x01RaA\xC6``\x83\x01aA_V[``\x82\x01R`\x80\x82\x015aA\xD9\x81a;\xE7V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15aA\xF7W`\0\x80\xFD[aB\0\x83a;\xCEV[\x91Pa?,\x84` \x85\x01aAqV[`\0\x80` \x83\x85\x03\x12\x15aB\"W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aB:W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aBNW`\0\x80\xFD[\x815\x81\x81\x11\x15aB]W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15aBoW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aB\x96W`\0\x80\xFD[aB\x9F\x84a;\xCEV[\x92P` \x84\x015\x91P`@\x84\x015aB\xB6\x81a;\xE7V[\x80\x91PP\x92P\x92P\x92V[`\xA0\x81\x01aB\xFC\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x0B`\x80\x83\x01R\x93\x92PPPV[`\xE0\x81\x01a\r\x9A\x82\x84`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[`\x80\x81\x01a\r\x9A\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15aC\xC1W`\0\x80\xFD[\x815a\x17A\x81a;HV[`\0\x80`@\x83\x85\x03\x12\x15aC\xDFW`\0\x80\xFD[\x825\x91P` \x83\x015a<-\x81a;\xE7V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aD\x07W`\0\x80\xFD[aD\x10\x85a;\xCEV[\x93P` \x85\x015\x92P`@\x85\x015aD'\x81a;\xE7V[\x91P``\x85\x015aD7\x81a;\xE7V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aDoW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aDSV[\x81\x81\x11\x15aD\x81W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aD\xD7WaD\xD7aD\x97V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aD\xF3WaD\xF3aD\x97V[P\x01\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[`\x01\x01\x93\x92PPPV[`\0\x81\x83\x03a\x01\xE0\x81\x12\x15aE7W`\0\x80\xFD[aE?a@\xF7V[aEH\x84a;\xCEV[\x81R` \x84\x015aEX\x81a;\xE7V[` \x82\x01R`@\x84\x015aEk\x81a;\xE7V[`@\x82\x01R`\xE0`_\x19\x83\x01\x12\x15aE\x82W`\0\x80\xFD[aE\x8AaA.V[\x91P``\x84\x015aE\x9A\x81a;HV[\x82R`\x80\x84\x015aE\xAA\x81a;\xE7V[` \x83\x01R`\xA0\x84\x015aE\xBD\x81a;\xE7V[`@\x83\x01R`\xC0\x84\x015aE\xD0\x81a;\xE7V[``\x83\x01RaE\xE1`\xE0\x85\x01a;\xF6V[`\x80\x83\x01RaE\xF3a\x01\0\x85\x01a;\xF6V[`\xA0\x83\x01RaF\x05a\x01 \x85\x01a;\xF6V[`\xC0\x83\x01R\x81``\x82\x01RaF\x1E\x85a\x01@\x86\x01aAqV[`\x80\x82\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15aF=W`\0\x80\xFD[a\x17A\x83\x83aAqV[\x815aFR\x81a;HV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015aF~\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015aF\xA6\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015aF\xD7\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015aF\xFF\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x03\x81\x01`\xA0\x83\x015aG0\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\xC0\x83\x015aGX\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x12YV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aG\xA2WaG\xA2aD\x97V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aG\xBBW`\0\x80\xFD[\x815a\x17A\x81a;\xE7V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[\x815aG\xED\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015aH\x15\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015aHE\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015aGX\x81a;\xE7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aH\x9AWaH\x9AaHmV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aH\xEAWaH\xEAaD\x97V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aI\x05WaI\x05aD\x97V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI1WaI1aD\x97V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aIaWaIaaD\x97V[\x02\x94\x93PPPPV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aI\x87WaI\x87aD\x97V[`\0\x03\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aI\xA2W`\0\x80\xFD[\x81Qa\x17A\x81a@\xAFV[`\0\x82\x82\x10\x15aI\xBFWaI\xBFaD\x97V[P\x03\x90V[`\0\x81aI\xD3WaI\xD3aD\x97V[P`\0\x19\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aIaWaIaaD\x97V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI1WaI1aD\x97V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aJ4WaJ4aHmV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJRW`\0\x80\xFD[\x81Qa\x17A\x81a;HV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aJ\x8DWaJ\x8DaD\x97V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aJ\xB9WaJ\xB9aD\x97V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aJ\xD5WaJ\xD5aD\x97V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aJ\xEBWaJ\xEBaD\x97V[PPP\x92\x90\x91\x02\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xE8\x1DJ\xDDK\xEFz \x8D\xA6\x9A\xC7\x1B]\xC7\xE5\x0BrjlP\x96e\xED\x12\xDF\xA6\xF5\xEBj\x07:dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static SPOTENGINE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02wW`\x005`\xE0\x1C\x80c\x8A\x1DC\xC9\x11a\x01`W\x80c\xE0\xB0b\x1F\x11a\0\xD8W\x80c\xED\xF0&S\x11a\0\x8CW\x80c\xF3\x9E\xEB\x10\x11a\0qW\x80c\xF3\x9E\xEB\x10\x14a\x07\xBFW\x80c\xF8\xA4.Q\x14a\x07\xD2W\x80c\xFA\xB2\xC4i\x14a\x07\xE5W`\0\x80\xFD[\x80c\xED\xF0&S\x14a\x07`W\x80c\xF2\xFD\xE3\x8B\x14a\x07\xACW`\0\x80\xFD[\x80c\xE3Cs\x8C\x11a\0\xBDW\x80c\xE3Cs\x8C\x14a\x05\x8DW\x80c\xECzy\xC9\x14a\x06eW\x80c\xEC\xD9\xCB\xA8\x14a\x06\xFCW`\0\x80\xFD[\x80c\xE0\xB0b\x1F\x14a\x05YW\x80c\xE34\xBE3\x14a\x05lW`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x11a\x01/W\x80c\xC5V\x07\xB5\x11a\x01\x14W\x80c\xC5V\x07\xB5\x14a\x04\xE5W\x80c\xC9\xFE\x9A\xC3\x14a\x04\xF8W\x80c\xCB\xD0\x80\x8D\x14a\x05\x0BW`\0\x80\xFD[\x80c\xAE\xD8\xE9g\x14a\x04\xC3W\x80c\xB1\xCB\x0FB\x14a\x04\xD4W`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x045W\x80c\x8D\xA5\xCB[\x14a\x04tW\x80c\xA6z\xC3\"\x14a\x04\x85W\x80c\xADs;\x8E\x14a\x04\xB0W`\0\x80\xFD[\x80cF\x04\xD1\x9B\x11a\x01\xF3W\x80cqP\x18\xA6\x11a\x01\xC2W\x80c\x7F\xA2\x9DI\x11a\x01\xA7W\x80c\x7F\xA2\x9DI\x14a\x03\xE9W\x80c\x87\x1D\t\x12\x14a\x03\xFCW\x80c\x896\xF7\xCD\x14a\x04\"W`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x03\xCEW\x80c|\x1E\x14\x87\x14a\x03\xD6W`\0\x80\xFD[\x80cF\x04\xD1\x9B\x14a\x03\x84W\x80cGB\x8E{\x14a\x03\x93W\x80cJ\xC8\xD8\xC1\x14a\x03\xA8W\x80cVw\x8D?\x14a\x03\xBBW`\0\x80\xFD[\x80c%\xBC\xE8\xC9\x11a\x02JW\x80c0\x97+P\x11a\x02/W\x80c0\x97+P\x14a\x03\x17W\x80c9\x1B{?\x14a\x03*W\x80cE\xBE~\xD6\x14a\x03=W`\0\x80\xFD[\x80c%\xBC\xE8\xC9\x14a\x02\xEEW\x80c+\xAFW\xD3\x14a\x03\x01W`\0\x80\xFD[\x80c\x14YEz\x14a\x02|W\x80c\x15<\xA6\xC0\x14a\x02\x91W\x80c\x19\xA2\xAC\x88\x14a\x02\xA4W\x80c!\x92Ai\x14a\x02\xCEW[`\0\x80\xFD[a\x02\x8Fa\x02\x8A6`\x04a;]V[a\x07\xFAV[\0[a\x02\x8Fa\x02\x9F6`\x04a<\x01V[a\n\xDEV[a\x02\xB7a\x02\xB26`\x04a<8V[a\x0B\x7FV[`@Q\x90Q`\x0F\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xE1a\x02\xDC6`\x04a<QV[a\r\x02V[`@Qa\x02\xC5\x91\x90a<jV[a\x02\x8Fa\x02\xFC6`\x04a=dV[a\r\xA0V[a\x03\ta\x0E,V[`@Qa\x02\xC5\x92\x91\x90a>*V[a\x02\x8Fa\x03%6`\x04a>\x94V[a\x10WV[a\x02\x8Fa\x0386`\x04a?\0V[a\x128V[a\x03la\x03K6`\x04a?5V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[`\0`@Qa\x02\xC5\x91\x90a?fV[a\x03\x9Ba\x12_V[`@Qa\x02\xC5\x91\x90a?\x8EV[a\x02\x8Fa\x03\xB66`\x04a?5V[a\x12\xE3V[a\x02\xB7a\x03\xC96`\x04a<8V[a\x13zV[a\x02\x8Fa\x13\xBCV[a\x02\xB7a\x03\xE46`\x04a?\xD8V[a\x13\xC8V[a\x02\x8Fa\x03\xF76`\x04a@\x02V[a\x14XV[a\x04\x0Fa\x04\n6`\x04a@PV[a\x14yV[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xC5V[a\x02\x8Fa\x0406`\x04a<8V[a\x14\xF9V[a\x04Ha\x04C6`\x04a@sV[a\x16\xD3V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x02\xC5V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x04\x98a\x04\x936`\x04a?5V[a\x17HV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC5V[a\x02\x8Fa\x04\xBE6`\x04a@\xC4V[a\x17\xECV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x02\x8Fa\x04\xF36`\x04aA\xE4V[a\x19\xDCV[a\x02\x8Fa\x05\x066`\x04aB\x0FV[a\x1B\rV[a\x02\x8Fa\x05\x196`\x04aB\x81V[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\x8Fa\x05g6`\x04aB\x81V[a\x1E\xE0V[a\x05\x7Fa\x05z6`\x04a?\xD8V[a\x1F\x98V[`@Qa\x02\xC5\x92\x91\x90aB\xC1V[a\x06Xa\x05\x9B6`\x04a?5V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x82\x01T\x80\x84\x0B``\x83\x01R\x84\x90\x04\x83\x0B`\x80\x82\x01R`\x03\x90\x91\x01T\x80\x83\x0B`\xA0\x83\x01R\x92\x90\x92\x04\x90\x0B`\xC0\x82\x01R\x90V[`@Qa\x02\xC5\x91\x90aC\rV[a\x06\xEFa\x06s6`\x04a?5V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xC5\x91\x90aCtV[a\x07\x0Fa\x07\n6`\x04a?5V[a RV[`@Qa\x02\xC5\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x02\xB7a\x07n6`\x04a?\xD8V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x81R`l\x84R\x81\x81 \x92\x81R\x91\x83R\x90\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[a\x02\x8Fa\x07\xBA6`\x04aC\xAFV[a \x86V[a\x04\x0Fa\x07\xCD6`\x04aC\xCCV[a!\x0EV[a\x02\x8Fa\x07\xE06`\x04aC\xF1V[a\"7V[`@\x80Q`l\x81R`k` \x82\x01R\x01a\x02\xC5V[a\x08\x06\x85\x85\x84\x84a#\xEEV[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x84\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x84\x86\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x87\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x89\x01\x82\x81R`\xA0\x80\x8B\x01\x84\x81R`\0`\xC0\x8D\x01\x81\x81R\x81\x80R`j\x8BR\x9CQ\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9D\x16\x17\x90\x9BU\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x97\x88\x16`\x01`\x80\x1B\x91\x89\x16\x82\x02\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x08U\x95Q\x91Q\x91\x87\x16\x91\x87\x16\x86\x02\x91\x90\x91\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\tU\x96Q\x98Q\x98\x85\x16\x98\x85\x16\x84\x02\x98\x90\x98\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\nU\x88Q\x95\x86\x01\x89Rc;\x9A\xCA\0\x80\x87R\x86\x86\x01\x81\x81R\x87\x8B\x01\x82\x81R\x88\x85\x01\x92\x83R\x88\x8B\x01\x84\x81R\x8A\x80R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x89R\x98Q\x7F\xEA#\xA1\xF6\x04v\xEF\x06\xB4\xF1 \x0F\xB4\xED3H\t=rb\xFAI8\xFF\x94|6\xDB\x93\xDC,\xC2\x80T\x93Q\x92Q\x94Q\x9AQ\x89\x16\x90\x97\x02c\xFF\xFF\xFF\xFF\x9A\x8B\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x95\x8C\x16`\x01`@\x1B\x02\x95\x90\x95\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x8C\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x9B\x90\x92\x16\x9A\x90\x9A\x17\x92\x90\x92\x17\x16\x17\x17\x90\x93\x16\x94\x90\x94\x17\x90U\x85Q\x94\x85\x01\x86R\x80\x85R\x90\x84\x01R\x92\x82\x01\x81\x90R\x91\x81\x01\x82\x90Ra\nT\x91\x90a%\x8BV[`g\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x82\x04\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x02\x19\x90\x91\x16\x90U`@Q\x90\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B+W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[`@Q\x80\x91\x03\x90\xFD[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q` \x81\x01\x90\x91R`\0\x81R`\0\x82\x81R`m` R`@\x90 [`\x01\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01`@\x1B\x90\x92\x04\x16\x10\x80\x15a\x0B\xFEWPa\x0B\xC7a%\xE7V[`\x01\x82\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x84\x90R`@\x90 \x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11\x15[\x15a\x0C\xE6W`\x01\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x82\x90R`@\x81 T`\x02\x83\x01\x80T`\x0F\x92\x83\x0B\x93\x91\x92a\x0CB\x91\x85\x91\x0BaD\xADV[\x82T`\x01`\x01`\x80\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP`\x01\x81\x81\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R` \x86\x90R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x82U\x94\x01\x80T\x90\x94\x16\x90\x93U\x81T\x04\x90\x91\x16\x90`\x08a\x0C\xBA\x83aD\xFCV[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa\x0B\x9DV[`@\x80Q` \x81\x01\x90\x91R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x92\x91PPV[a\r\x8B`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x84Q`\xE0\x81\x01\x86R\x82\x81R\x90\x81\x01\x82\x90R\x93\x84\x01\x81\x90R``\x84\x81\x01\x82\x90R`\x80\x85\x01\x82\x90R\x91\x84\x01\x81\x90R`\xC0\x84\x01R\x90\x91\x90\x82\x01\x90\x81R`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x91\x01R\x90V[a\r\x9A6\x83\x90\x03\x83\x01\x83aE#V[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x86\x16a\r\xB0W`\0\x80\xFD[a\r\xCB\x86\x86\x86\x86a\r\xC66\x87\x90\x03\x87\x01\x87aF+V[a&ZV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`j` R`@\x90 \x82\x90a\r\xEC\x82\x82aFGV[PP`@\x80Q`\x80\x81\x01\x82Rg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82R` \x82\x01R`\0\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x0E$\x90\x87\x90a%\x8BV[PPPPPPV[``\x80`\0`g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\xA9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x0ElW\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xCAWa\x0E\xCAa@\xE1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x10Wa\x0F\x10a@\xE1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x10QW`\0\x82\x82\x81Q\x81\x10a\x0F\\Wa\x0F\\aGzV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0F\xCB\x92\x91a*\xFB\x16V[\x86\x84\x81Q\x81\x10a\x0F\xDDWa\x0F\xDDaGzV[` \x02` \x01\x01\x81\x81Qa\x0F\xF1\x91\x90aD\xADV[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x10\x11\x93P\x90\x91\x0B\x90a*\xFBV[\x85\x84\x81Q\x81\x10a\x10#Wa\x10#aGzV[` \x02` \x01\x01\x81\x81Qa\x107\x91\x90aD\xADV[`\x0F\x0B\x90RPa\x10J\x91P\x82\x90PaG\x90V[\x90Pa\x0F?V[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x121W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x10\x89Wa\x10\x89aGzV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`k\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x11\x15Wa\x11\x15aGzV[\x90P` \x02\x01` \x81\x01\x90a\x11*\x91\x90aG\xA9V[`\x0F\x0Ba\x11K\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x11\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x11\xA7Wa\x11\xA7aGzV[\x90P` \x02\x01` \x81\x01\x90a\x11\xBC\x91\x90aG\xA9V[`\x0F\x0Ba\x11\xDD\x82` \x01Q\x83``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x12\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[PPP\x80a\x12*\x90aG\xC6V[\x90Pa\x10ZV[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x12Y\x82\x82aFGV[PPPPV[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x12\xD9W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x12\x9CW\x90P[PPPPP\x90P\x90V[`\0a\x12\xF0\x82`\x01a\x1F\x98V[P\x90P`\0a\x13\x13\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x135\x83` \x01Q\x84``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x121W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x13\x92\x82a\x0B\x7FV[PP`\0\x90\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x90V[a\x13\xC6`\0a+~V[V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x14O\x82\x82a+\xD0V[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\x12Y\x82\x82aG\xE2V[`\0\x80a\x01\0a\x14\x87a,$V[a\x14\x91\x91\x90aH\x83V[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x14\xF1W`\0a\x14\xB5\x86\x83a,sV[\x90P\x80`\0\x03a\x14\xC5WPa\x14\xDFV[a\x14\xD1\x81\x83\x88\x88a-<V[a\x14\xDB\x90\x85aD\xADV[\x93PP[\x80a\x14\xE9\x81aH\xA6V[\x91PPa\x14\x96V[PP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0a\x15Ha\x12_V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x16\xCEW`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x15|Wa\x15|aGzV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`l\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x16\x01\x90\x83\x90a+\xD0V[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x16\xBAW\x81Q`@\x83\x01Q`\0\x91a\x16*\x91`\x0F\x0B\x90a*\xFBV[`@\x84\x01Q\x83Q\x91\x92Pa\x16K\x91a\x16B\x90\x84aD\xADV[`\x0F\x0B\x90a-\x97V[`\x0F\x0B\x80\x84R`\0\x12a\x16]W`\0\x80\xFD[a\x16{\x83` \x01Q\x83`\0\x01Q`\x0F\x0Ba-\x97\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x16\x8C\x91\x90aD\xADV[`\x0F\x0B\x90RP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x16\xAE\x90\x85\x90\x89\x90a.\0V[a\x16\xB8\x84\x84a%\x8BV[P[PPP\x80a\x16\xC7\x90aG\xC6V[\x90Pa\x15MV[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x16\xF9\x84a.VV[\x90P`\0a\x17\x07\x85\x87a.\xABV[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x177\x84`\x01\x88a.\xC4V[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a\x17\xE2\x92\x85\x92a\x17\xB5\x92\x91\x90a*\xFB\x16V[a\x17\xD3\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x17\xDD\x91\x90aH\xBFV[a/[V[a\x17A\x90\x83aI\x0FV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\"V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x18\x8Bb\x01Q\x80`\x07aI:V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x18\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0[`gTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x16\xCEW`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x19\tWa\x19\taGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`\x98\x19\x81\x01a\x19>WPa\x19\xCAV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90a\x19\x9CW\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a\x19\xB2WPPa\x19\xCAV[a\x19\xBD\x82\x82\x87a/wV[a\x19\xC7\x82\x82a%\x8BV[PP[\x80a\x19\xD4\x81aH\xA6V[\x91PPa\x18\xDEV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1A\x08WP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x1ABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\"V[`\0a\x1B\x88\x82\x84\x01\x84aE#V[`\x80\x81\x01Q\x81Q\x91\x92P\x90c\xFF\xFF\xFF\xFF\x16\x15a\x1EFW\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1B\xCAWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a\x1B\xFDWP``\x82\x01QQ\x82Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x1C7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x81Q`\xA0\x81\x01\x83R\x90T`\x03\x81\x81\x0B\x83Rd\x01\0\0\0\0\x82\x04\x81\x0B\x83\x86\x01\x90\x81R`\x01`@\x1B\x83\x04\x82\x0B\x84\x86\x01\x90\x81R`\x01``\x1B\x84\x04\x83\x0B``\x80\x87\x01\x91\x82R`\x01`\x80\x1B\x90\x95\x04`\x0F\x0B`\x80\x87\x01R\x89Q\x84\x0B\x86R\x96\x89\x01Q\x83\x0B\x90\x91R\x93\x87\x01Q\x81\x0B\x90\x93R\x85\x01Q\x90\x91\x0B\x90\x91R\x90P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x92\x83R`@\x90\x81\x90 \x84Q\x81T\x94\x86\x01Q\x92\x86\x01Q``\x87\x01Q`\x80\x90\x97\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x97\x86\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x87\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x87\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x98\x16\x93\x90\x96\x16\x92\x90\x92\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x92\x90\x92\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90Ua\x1D\xBDa3SV[\x83Q` \x85\x01Q`@\x80\x87\x01Q\x90Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x93\x84\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R`\x0F\x91\x82\x0B`D\x84\x01R\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E@W=`\0\x80>=`\0\xFD[PPPPP[P``\x81\x81\x01Q\x91Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17`\x01\x83\x01U\x92\x84\x01Q`\x80\x85\x01Q\x90\x83\x16\x90\x83\x16\x84\x02\x17`\x02\x82\x01U`\xA0\x84\x01Q`\xC0\x90\x94\x01Q\x93\x82\x16\x93\x90\x91\x16\x90\x91\x02\x91\x90\x91\x17`\x03\x90\x91\x01UPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90`\x98\x19\x01a\x1FHWa\x1FH\x83\x83a3\xC8V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra\x1F\x83\x82\x82\x85a5BV[a\x1F\x8E\x85\x85\x83a.\0V[a\x121\x85\x83a%\x8BV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a D\x81\x83a+\xD0V[\x93P\x93PPP[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\r\x9A\x82a.VV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\"V[a!\x0B\x81a+~V[PV[`@\x80Q`\x80\x81\x01\x82R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a!\xCD\x83\x83a+\xD0V[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15a\"\x16W`\0a!\xFBa!\xF4\x87a!\xEF\x85aIjV[a6AV[`\0a/[V[\x90Pa\"\x07\x81\x87aH\xBFV[\x95Pa\"\x14\x84\x84\x83a5BV[P[a\"!`\0\x84a%\x8BV[a\"-`\0\x87\x84a.\0V[P\x92\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16a\"tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x86\x86R`l\x85R\x83\x86 \x8A\x87R\x85R\x83\x86 \x84Q\x80\x87\x01\x86R\x90T\x84\x0B\x81R\x8A\x87R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x86R\x95\x84\x90 \x84Q\x95\x86\x01\x90\x94R\x92T\x90\x91\x0B\x83R\x93\x90\x92\x91\x90`\x98\x19\x01a#\xA2Wa#\xA2\x87\x87a3\xC8V[a#\xAD\x84\x83\x88a5BV[a#\xB8\x83\x82\x87a5BV[a#\xC3\x88\x88\x84a.\0V[a#\xCF`\0\x88\x83a.\0V[a#\xD9\x88\x85a%\x8BV[a#\xE4`\0\x84a%\x8BV[PPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a$\x0EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a$(WP0;\x15\x80\x15a$(WP`\0T`\xFF\x16`\x01\x14[a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\"V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a$\xBDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a$\xC5a6VV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua$\xE9\x82a \x86V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x121W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\n\xCFV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua%\xE3\x82a6\xC9V[PPV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&U\x91\x90aI\x90V[\x90P\x90V[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a&\x83WPc;\x9A\xCA\0\x81`@\x01Q`\x03\x0B\x13\x15[\x80\x15a&\x9DWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[\x80\x15a&\xB4WPc;\x9A\xCA\0\x81``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a&\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x87\x81\x16`\0\x81\x81R` \x93\x84R`@\x90\x81\x90 \x85Q\x81T\x95\x87\x01Q\x87\x84\x01Q``\x89\x01Q`\x80\x90\x99\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x99\x88\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x89\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x89\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x9A\x16\x94\x90\x98\x16\x93\x90\x93\x17\x97\x90\x97\x17\x16\x94\x90\x94\x17\x94\x90\x94\x17\x92\x90\x92\x16\x93\x90\x93\x17\x90\x91U`fT\x90QcC\xB1j\x11`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90c\x87b\xD4\"\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a(\x11W=`\0\x80>=`\0\xFD[PP`g\x80T`\x01\x80\x82\x01\x83U`\0\x83\x81R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x84\x04\x01\x80T`\x07\x90\x94\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x95\x16\x94\x8D\x16\x02\x93\x90\x93\x17\x90\x92U\x91T\x90\x93Pa(}\x92PaI\xADV[\x90P[\x80\x15a*@W`ga(\x93`\x01\x83aI\xADV[\x81T\x81\x10a(\xA3Wa(\xA3aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16`g\x82\x81T\x81\x10a(\xE2Wa(\xE2aGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x10\x15a*)W`\0`g\x82\x81T\x81\x10a)!Wa)!aGzV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`ga)T`\x01\x84aI\xADV[\x81T\x81\x10a)dWa)daGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16`g\x83\x81T\x81\x10a)\x9DWa)\x9DaGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80`g`\x01\x84a)\xE0\x91\x90aI\xADV[\x81T\x81\x10a)\xF0Wa)\xF0aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa*.V[a*@V[\x80a*8\x81aI\xC4V[\x91PPa(\x80V[Pa*Ia3SV[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R`\x0F\x85\x81\x0B`D\x83\x01R\x84\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xACW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xC0W=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x88\x16\x81R\x7F2\x86\xB09K\xF15\x02E)\x0Br&\xC9.\xD1\x86\xBDqo(\x93\x8Eb\xDB\xB8\x95)\x8F\x01\x81r\x92P` \x01\x90Pa\n\xCFV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a+=WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a+vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x82Q`\x0F\x0B\x81\x12\x15a+\xF4WP\x82Qa+\xFBV[P` \x83\x01Q[`@\x80Q` \x81\x01\x90\x91R\x83Q\x81\x90a,\x17\x90`\x0F\x0B\x84a*\xFBV[`\x0F\x0B\x90R\x94\x93PPPPV[`g\x80T`\0\x91\x90a,8\x90`\x01\x90aI\xADV[\x81T\x81\x10a,HWa,HaGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a+vW`\0`g\x82\x81T\x81\x10a,\x97Wa,\x97aGzV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a,\xCD\x91\x90aI\xDBV[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a-\tWPa,\xEE\x85`\x01aI\xFEV[a,\xFA\x90a\x01\0aI\xDBV[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a-)Wa-\x1Aa\x01\0\x82aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a-4\x81aG\x90V[\x91PPa,xV[`\0\x80a-K\x85a\x01\0aI\xDBV[\x90P[\x85\x15a-\x8EW`\x01\x86\x16\x15a-uWa-h\x81\x85\x85a7\x05V[a-r\x90\x83aD\xADV[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a-\x86\x81aH\xA6V[\x91PPa-NV[P\x94\x93PPPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a-\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a+\x12Wa+\x12aHmV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x90\x91U`\x0F\x0B\x15\x15a.L\x83\x85\x83a7\xBEV[a\x12Y\x84\x84a8{V[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\r\x9A\x82\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0ya8\xBEV[`\0\x80a.\xB8\x84\x84a\x13\xC8V[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15a.\xDAWa.\xDAa?PV[\x03a.\xEEWPg\r\xE0\xB6\xB3\xA7d\0\0a\x17AV[`\0\x80\x84`\x0F\x0B\x12a/'W`\0\x83`\x02\x81\x11\x15a/\x0EWa/\x0Ea?PV[\x14a/\x1DW\x84`@\x01Qa/ V[\x84Q[\x90Pa/SV[`\0\x83`\x02\x81\x11\x15a/;Wa/;a?PV[\x14a/JW\x84``\x01Qa/PV[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a/pW\x81a\x17AV[P\x90\x91\x90PV[`\0\x80a/\x98\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a/\xBA\x85` \x01Q\x86``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a/\xCC`\x0F\x83\x90\x0B\x84a-\x97V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x83\x01T\x80\x86\x0B``\x84\x01R\x81\x90\x04\x85\x0B`\x80\x83\x01R`\x03\x90\x92\x01T\x80\x85\x0B`\xA0\x83\x01R\x91\x90\x91\x04\x83\x0B`\xC0\x82\x01R\x93\x94P\x91\x92\x91\x90\x84\x90\x0B\x83\x03a0cWP`\0a0\xFFV[\x81` \x01Q`\x0F\x0B\x84`\x0F\x0B\x12\x15a0\xA8Wa0\x97\x82` \x01Qa\x16B\x86\x85``\x01Q`\x0F\x0Ba*\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a0\xA1\x90\x82aD\xADV[\x90Pa0\xFFV[a0\xE3a0\xD5\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a0\xC6\x91\x90aH\xBFV[` \x85\x01Qa\x16B\x90\x88aH\xBFV[`\x80\x84\x01Q`\x0F\x0B\x90a*\xFBV[\x82``\x01Qa0\xF2\x91\x90aD\xADV[a0\xFC\x90\x82aD\xADV[\x90P[a1\x1Aa1\x0Fc\x01\xE13\x80a9\xD3V[`\x0F\x83\x90\x0B\x90a-\x97V[\x90Pa1;\x88a12\x83g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[`\x0F\x0B\x90a:LV[\x96P\x81`\xC0\x01Q\x92PPP`\0a1jg\r\xE0\xB6\xB3\xA7d\0\0\x87a1_\x91\x90aH\xBFV[`\x0F\x85\x90\x0B\x90a*\xFBV[\x90P`\0a1\x96a1\x8Bg\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0aH\xBFV[`\x0F\x84\x90\x0B\x90a*\xFBV[\x90P`\0a1\xB2a1\xA7\x83\x85aH\xBFV[`\x0F\x89\x90\x0B\x90a*\xFBV[` \x8B\x01Q\x90\x91Pa1\xC7\x90`\x0F\x0B\x89a*\xFBV[`\x0F\x0B` \x8B\x01R`\0a1\xE3\x83g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[\x8BQ\x90\x91Pa1\xF5\x90`\x0F\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B\x8CR\x82\x90\x0B\x15a2MWc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra2?\x8C\x82\x85a5BV[a2K\x8D`\0\x83a.\0V[P[\x84`\x0F\x0B`\0\x14a2\xE4W`\0a2ua2jc\x01\xE13\x80a9\xD3V[`\x0F\x88\x90\x0B\x90a-\x97V[\x90P`\0a2\x8F\x8Ca12\x84g\r\xE0\xB6\xB3\xA7d\0\0aD\xADV[` \x8E\x01Q\x90\x91Pa2\xA4\x90`\x0F\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B` \x8F\x01R\x8DQa2\xBB\x91\x0B\x82a*\xFBV[`\x0F\x90\x81\x0B\x8ERa2\xCF\x90\x84\x90\x0B\x82a*\xFBV[\x92Pa2\xDF`\x0F\x8C\x90\x0B\x82a*\xFBV[\x9APPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8E\x16\x81R`\x01`\x01`\x80\x1B\x03\x8C\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8B\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPPV[`\0a3g`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&U\x91\x90aJ@V[`\x01\x19\x82\x01a3\xD5WPPV[a3\xDE\x82a\x0B\x7FV[P`\0\x81`\x0F\x0B\x13\x15a4\xC4W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R`\x0F\x85\x90\x0B\x93\x81\x01\x93\x84R\x92\x83R\x91\x90\x81\x01b\x05F\0a4#a%\xE7V[a4-\x91\x90aI\x0FV[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x91R`\x01\x80\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x87\x81R`@\x82 \x87QQ\x81T\x90\x88\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x82\x16\x17\x82U\x97\x90\x91\x01Q\x94\x01\x80T\x94\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x90\x92U\x81T\x16\x91a4\x98\x83aD\xFCV[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\0\x81`\x0F\x0B\x12\x15a%\xE3W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x80\x82R\x82\x90\x82\x90a5\x05\x90\x83\x90aD\xADV[`\x0F\x0B\x90RP`\0\x92\x83R`m` R`@\x90\x92 \x91Q`\x02\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15a5pW\x81Q`@\x84\x01\x80Qa5e\x90\x83\x90aH\xBFV[`\x0F\x0B\x90RPa5\x8AV[\x81Q``\x84\x01\x80Qa5\x83\x90\x83\x90aD\xADV[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15a5\xA3WP\x82Qa5\xAAV[P` \x83\x01Q[\x82Q`\0\x90\x83\x90a5\xBE\x90`\x0F\x0B\x84a*\xFBV[a5\xC8\x91\x90aD\xADV[\x90P`\0\x81`\x0F\x0B\x13\x15a5\xDFW\x84Q\x91Pa5\xE7V[\x84` \x01Q\x91P[a5\xF5`\x0F\x82\x90\x0B\x83a-\x97V[`\x0F\x0B\x80\x85R`\0\x12\x15a6!W\x83Q`@\x86\x01\x80Qa6\x16\x90\x83\x90aD\xADV[`\x0F\x0B\x90RPa\x121V[\x83Q``\x86\x01\x80Qa64\x90\x83\x90aH\xBFV[`\x0F\x0B\x90RPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a/pW\x81a\x17AV[`\0Ta\x01\0\x90\x04`\xFF\x16a6\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x0B\"V[a\x13\xC6a:\xD4V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a7\x11\x85a.VV[\x90P`\0\x80a7 \x87\x87a.\xABV[\x91P\x91P`\0a71\x84\x84\x88a.\xC4V[\x90Pa7=\x82\x86aD\xADV[\x94P\x82`\x0F\x0B`\0\x14a7\xB3Wa7]g\r\xE0\xB6\xB3\xA7d\0\0`\x02aJ]V[`\x0F\x0B\x81`\x0F\x0B\x03a7\x86Wo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94PPPPPa\x17AV[`\x80\x84\x01Qa7\xA6\x90a7\x9D`\x0F\x86\x90\x0B\x84a*\xFBV[`\x0F\x0B\x90a*\xFBV[a7\xB0\x90\x86aD\xADV[\x94P[PPPP\x93\x92PPPV[\x80\x15a8\x1FWa7\xD0a\x01\0\x83aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a7\xFD\x91\x90aH\x83V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a8+a\x01\0\x83aJ\x1DV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a8Y\x91\x90aH\x83V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R` \x83\x81R`@\x91\x82\x90 \x82Q`\xA0\x81\x01\x84R\x90T`\x03\x81\x81\x0B\x80\x84Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01``\x1B\x81\x04\x90\x93\x0B``\x82\x01R`\x01`\x80\x1B\x90\x92\x04`\x0F\x0B`\x80\x83\x01Ra9`\x90c;\x9A\xCA\0aJ]V[`\x0F\x0B\x82R` \x81\x01Qa9{\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x0B` \x83\x01R`@\x81\x01Qa9\x99\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x0B`@\x83\x01R``\x81\x01Qa9\xB7\x90`\x03\x0Bc;\x9A\xCA\0aJ]V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x92\x91PPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a:\x0CWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a:EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a:\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\"\x91\x90aDBV[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13a+vW\x80\x84\x16`\x0F\x0B\x15a:\xC0Wa:\xBD\x82\x86a*\xFBV[\x91P[a:\xCA\x85\x86a*\xFBV[\x94P`\x02\x02a:\x9BV[`\0Ta\x01\0\x90\x04`\xFF\x16a;?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x0B\"V[a\x13\xC63a+~V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x0BW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a;uW`\0\x80\xFD[\x855a;\x80\x81a;HV[\x94P` \x86\x015a;\x90\x81a;HV[\x93P`@\x86\x015a;\xA0\x81a;HV[\x92P``\x86\x015a;\xB0\x81a;HV[\x91P`\x80\x86\x015a;\xC0\x81a;HV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a;\xE2W`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a!\x0BW`\0\x80\xFD[\x805a;\xE2\x81a;\xE7V[`\0\x80`@\x83\x85\x03\x12\x15a<\x14W`\0\x80\xFD[a<\x1D\x83a;\xCEV[\x91P` \x83\x015a<-\x81a;\xE7V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a<JW`\0\x80\xFD[P5\x91\x90PV[`\0a\x01\xE0\x82\x84\x03\x12\x15a<dW`\0\x80\xFD[P\x91\x90PV[`\0a\x01\xE0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qa=\x02``\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\x80\x83\x01Q\x80Q`\x03\x0Ba\x01@\x84\x01R` \x81\x01Q`\x03\x0Ba\x01`\x84\x01R`@\x81\x01Q`\x03\x0Ba\x01\x80\x84\x01R``\x81\x01Q`\x03\x0Ba\x01\xA0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xC0\x84\x01RP\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a<dW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15a=\x7FW`\0\x80\xFD[a=\x88\x88a;\xCEV[\x96Pa=\x96` \x89\x01a;\xCEV[\x95P`@\x88\x015a=\xA6\x81a;\xE7V[\x94P``\x88\x015a=\xB6\x81a;\xE7V[\x93Pa=\xC5\x89`\x80\x8A\x01a=RV[\x92P`\xA0a\x01_\x19\x82\x01\x12\x15a=\xDAW`\0\x80\xFD[Pa\x01`\x87\x01\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a>\x1FW\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a>=`@\x83\x01\x85a=\xECV[\x82\x81\x03` \x84\x01Ra\x14O\x81\x85a=\xECV[`\0\x80\x83`\x1F\x84\x01\x12a>aW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>yW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a KW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a>\xAAW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a>\xC2W`\0\x80\xFD[a>\xCE\x88\x83\x89\x01a>OV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a>\xE7W`\0\x80\xFD[Pa>\xF4\x87\x82\x88\x01a>OV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80a\x01\0\x83\x85\x03\x12\x15a?\x14W`\0\x80\xFD[a?\x1D\x83a;\xCEV[\x91Pa?,\x84` \x85\x01a=RV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a?GW`\0\x80\xFD[a\x17A\x82a;\xCEV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a?\x88WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a?\xCCW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a?\xAAV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a?\xEBW`\0\x80\xFD[a?\xF4\x83a;\xCEV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a@\x16W`\0\x80\xFD[a@\x1F\x84a;\xCEV[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a@3W`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a;\xE2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a@cW`\0\x80\xFD[\x825\x91Pa?,` \x84\x01a@AV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x88W`\0\x80\xFD[\x835\x92Pa@\x98` \x85\x01a;\xCEV[\x91Pa@\xA6`@\x85\x01a@AV[\x90P\x92P\x92P\x92V[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a!\x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\xD6W`\0\x80\xFD[\x815a\x17A\x81a@\xAFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aA(WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aA(WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14a;\xE2W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15aA\x83W`\0\x80\xFD[aA\x8Ba@\xF7V[\x90PaA\x96\x82aA_V[\x81RaA\xA4` \x83\x01aA_V[` \x82\x01RaA\xB5`@\x83\x01aA_V[`@\x82\x01RaA\xC6``\x83\x01aA_V[``\x82\x01R`\x80\x82\x015aA\xD9\x81a;\xE7V[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15aA\xF7W`\0\x80\xFD[aB\0\x83a;\xCEV[\x91Pa?,\x84` \x85\x01aAqV[`\0\x80` \x83\x85\x03\x12\x15aB\"W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aB:W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aBNW`\0\x80\xFD[\x815\x81\x81\x11\x15aB]W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15aBoW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aB\x96W`\0\x80\xFD[aB\x9F\x84a;\xCEV[\x92P` \x84\x015\x91P`@\x84\x015aB\xB6\x81a;\xE7V[\x80\x91PP\x92P\x92P\x92V[`\xA0\x81\x01aB\xFC\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x0B`\x80\x83\x01R\x93\x92PPPV[`\xE0\x81\x01a\r\x9A\x82\x84`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[`\x80\x81\x01a\r\x9A\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15aC\xC1W`\0\x80\xFD[\x815a\x17A\x81a;HV[`\0\x80`@\x83\x85\x03\x12\x15aC\xDFW`\0\x80\xFD[\x825\x91P` \x83\x015a<-\x81a;\xE7V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aD\x07W`\0\x80\xFD[aD\x10\x85a;\xCEV[\x93P` \x85\x015\x92P`@\x85\x015aD'\x81a;\xE7V[\x91P``\x85\x015aD7\x81a;\xE7V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aDoW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aDSV[\x81\x81\x11\x15aD\x81W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aD\xD7WaD\xD7aD\x97V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aD\xF3WaD\xF3aD\x97V[P\x01\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[`\x01\x01\x93\x92PPPV[`\0\x81\x83\x03a\x01\xE0\x81\x12\x15aE7W`\0\x80\xFD[aE?a@\xF7V[aEH\x84a;\xCEV[\x81R` \x84\x015aEX\x81a;\xE7V[` \x82\x01R`@\x84\x015aEk\x81a;\xE7V[`@\x82\x01R`\xE0`_\x19\x83\x01\x12\x15aE\x82W`\0\x80\xFD[aE\x8AaA.V[\x91P``\x84\x015aE\x9A\x81a;HV[\x82R`\x80\x84\x015aE\xAA\x81a;\xE7V[` \x83\x01R`\xA0\x84\x015aE\xBD\x81a;\xE7V[`@\x83\x01R`\xC0\x84\x015aE\xD0\x81a;\xE7V[``\x83\x01RaE\xE1`\xE0\x85\x01a;\xF6V[`\x80\x83\x01RaE\xF3a\x01\0\x85\x01a;\xF6V[`\xA0\x83\x01RaF\x05a\x01 \x85\x01a;\xF6V[`\xC0\x83\x01R\x81``\x82\x01RaF\x1E\x85a\x01@\x86\x01aAqV[`\x80\x82\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15aF=W`\0\x80\xFD[a\x17A\x83\x83aAqV[\x815aFR\x81a;HV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015aF~\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015aF\xA6\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015aF\xD7\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015aF\xFF\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x03\x81\x01`\xA0\x83\x015aG0\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\xC0\x83\x015aGX\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x12YV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aG\xA2WaG\xA2aD\x97V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aG\xBBW`\0\x80\xFD[\x815a\x17A\x81a;\xE7V[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[\x815aG\xED\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015aH\x15\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015aHE\x81a;\xE7V[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015aGX\x81a;\xE7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aH\x9AWaH\x9AaHmV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aE\x19WaE\x19aD\x97V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aH\xEAWaH\xEAaD\x97V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aI\x05WaI\x05aD\x97V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI1WaI1aD\x97V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aIaWaIaaD\x97V[\x02\x94\x93PPPPV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aI\x87WaI\x87aD\x97V[`\0\x03\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aI\xA2W`\0\x80\xFD[\x81Qa\x17A\x81a@\xAFV[`\0\x82\x82\x10\x15aI\xBFWaI\xBFaD\x97V[P\x03\x90V[`\0\x81aI\xD3WaI\xD3aD\x97V[P`\0\x19\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aIaWaIaaD\x97V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI1WaI1aD\x97V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aJ4WaJ4aHmV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJRW`\0\x80\xFD[\x81Qa\x17A\x81a;HV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aJ\x8DWaJ\x8DaD\x97V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aJ\xB9WaJ\xB9aD\x97V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aJ\xD5WaJ\xD5aD\x97V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aJ\xEBWaJ\xEBaD\x97V[PPP\x92\x90\x91\x02\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \xE8\x1DJ\xDDK\xEFz \x8D\xA6\x9A\xC7\x1B]\xC7\xE5\x0BrjlP\x96e\xED\x12\xDF\xA6\xF5\xEBj\x07:dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static SPOTENGINE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct SpotEngine<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SpotEngine<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SpotEngine<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SpotEngine<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SpotEngine<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SpotEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SpotEngine<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SPOTENGINE_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SPOTENGINE_ABI.clone(),
                SPOTENGINE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addProduct` (0x25bce8c9) function
        pub fn add_product(
            &self,
            product_id: u32,
            quote_id: u32,
            size_increment: i128,
            min_size: i128,
            config: Config,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [37, 188, 232, 201],
                    (
                        product_id,
                        quote_id,
                        size_increment,
                        min_size,
                        config,
                        risk_store,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertUtilization` (0x4ac8d8c1) function
        pub fn assert_utilization(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 200, 216, 193], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBalance` (0x7c1e1487) function
        pub fn get_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([124, 30, 20, 135], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getClearinghouse` (0xb1cb0f42) function
        pub fn get_clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 203, 15, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConfig` (0xe343738c) function
        pub fn get_config(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Config> {
            self.0
                .method_hash([227, 67, 115, 140], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCoreRisk` (0x8a1d43c9) function
        pub fn get_core_risk(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, CoreRisk> {
            self.0
                .method_hash([138, 29, 67, 201], (subaccount, product_id, health_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEndpoint` (0xaed8e967) function
        pub fn get_endpoint(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([174, 216, 233, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEngineType` (0x4604d19b) function
        pub fn get_engine_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([70, 4, 209, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthContribution` (0x871d0912) function
        pub fn get_health_contribution(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([135, 29, 9, 18], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNlpUnlockedBalance` (0x56778d3f) function
        pub fn get_nlp_unlocked_balance(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([86, 119, 141, 63], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProductIds` (0x47428e7b) function
        pub fn get_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([71, 66, 142, 123], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawBalance` (0xedf02653) function
        pub fn get_raw_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, BalanceNormalized> {
            self.0
                .method_hash([237, 240, 38, 83], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawState` (0xec7a79c9) function
        pub fn get_raw_state(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, State> {
            self.0
                .method_hash([236, 122, 121, 201], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRisk` (0xecd9cba8) function
        pub fn get_risk(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, Risk> {
            self.0
                .method_hash([236, 217, 203, 168], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlots` (0xfab2c469) function
        pub fn get_slots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([250, 178, 196, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStateAndBalance` (0xe334be33) function
        pub fn get_state_and_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (State, Balance)> {
            self.0
                .method_hash([227, 52, 190, 51], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getToken` (0x45be7ed6) function
        pub fn get_token(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([69, 190, 126, 214], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenBalance` (0xa67ac322) function
        pub fn get_token_balance(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([166, 122, 195, 34], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalBalances` (0x2baf57d3) function
        pub fn get_total_balances(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i128>, ::std::vec::Vec<i128>),
        > {
            self.0
                .method_hash([43, 175, 87, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, offchain_exchange, quote, endpoint, admin),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0x30972b50) function
        pub fn manual_assert(
            &self,
            total_deposits: ::std::vec::Vec<i128>,
            total_borrows: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 151, 43, 80], (total_deposits, total_borrows))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBalance` (0xcbd0808d) function
        pub fn set_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 208, 128, 141], (product_id, subaccount, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x391b7b3f) function
        pub fn set_config(
            &self,
            product_id: u32,
            config: Config,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 27, 123, 63], (product_id, config))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setState` (0x7fa29d49) function
        pub fn set_state(
            &self,
            product_id: u32,
            state: State,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 162, 157, 73], (product_id, state))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `socializeSubaccount` (0x8936f7cd) function
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 54, 247, 205], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tryUnlockNlpBalance` (0x19a2ac88) function
        pub fn try_unlock_nlp_balance(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
            self.0
                .method_hash([25, 162, 172, 136], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedUpdateProductTx` (0x21924169) function
        pub fn unsigned_update_product_tx(
            &self,
            p: UpdateProductTx,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateProductTx> {
            self.0
                .method_hash([33, 146, 65, 105], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBalance` (0xe0b0621f) function
        pub fn update_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 176, 98, 31], (product_id, subaccount, amount_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBalance` (0xf8a42e51) function
        pub fn update_balance_with_product_id_and_subaccount_and_quote_delta(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
            quote_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 164, 46, 81],
                    (product_id, subaccount, amount_delta, quote_delta),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePrice` (0x153ca6c0) function
        pub fn update_price(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 60, 166, 192], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateProduct` (0xc9fe9ac3) function
        pub fn update_product(
            &self,
            raw_txn: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 254, 154, 195], raw_txn)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateQuoteFromInsurance` (0xf39eeb10) function
        pub fn update_quote_from_insurance(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([243, 158, 235, 16], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRisk` (0xc55607b5) function
        pub fn update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 86, 7, 181], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateStates` (0xad733b8e) function
        pub fn update_states(&self, dt: u128) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 115, 59, 142], dt)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddProduct` event
        pub fn add_product_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddProductFilter> {
            self.0.event()
        }
        ///Gets the contract's `BalanceUpdate` event
        pub fn balance_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BalanceUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `InterestPayment` event
        pub fn interest_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InterestPaymentFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ProductUpdate` event
        pub fn product_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProductUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SpotBalance` event
        pub fn spot_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SpotBalanceFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SpotEngineEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SpotEngine<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "AddProduct", abi = "AddProduct(uint32)")]
    pub struct AddProductFilter {
        pub product_id: u32,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "BalanceUpdate", abi = "BalanceUpdate(uint32,bytes32)")]
    pub struct BalanceUpdateFilter {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "InterestPayment",
        abi = "InterestPayment(uint32,uint128,int128,int128,int128)"
    )]
    pub struct InterestPaymentFilter {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub dt: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub deposit_rate_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub borrow_rate_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub fee_amount: i128,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ProductUpdate", abi = "ProductUpdate(uint32)")]
    pub struct ProductUpdateFilter {
        pub product_id: u32,
    }
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SpotBalance",
        abi = "SpotBalance(bytes32,uint32,int128,int128)"
    )]
    pub struct SpotBalanceFilter {
        #[ethevent(indexed)]
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[ethevent(indexed)]
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_multiplier_x18: i128,
    }
    ///Container type for all of the contract's events
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum SpotEngineEvents {
        AddProductFilter(AddProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        InterestPaymentFilter(InterestPaymentFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProductUpdateFilter(ProductUpdateFilter),
        SpotBalanceFilter(SpotBalanceFilter),
    }
    impl ::ethers::contract::EthLogDecode for SpotEngineEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddProductFilter::decode_log(log) {
                return Ok(SpotEngineEvents::AddProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SpotEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = InterestPaymentFilter::decode_log(log) {
                return Ok(SpotEngineEvents::InterestPaymentFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SpotEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(SpotEngineEvents::ProductUpdateFilter(decoded));
            }
            if let Ok(decoded) = SpotBalanceFilter::decode_log(log) {
                return Ok(SpotEngineEvents::SpotBalanceFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SpotEngineEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProductFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterestPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotBalanceFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductFilter> for SpotEngineEvents {
        fn from(value: AddProductFilter) -> Self {
            Self::AddProductFilter(value)
        }
    }
    impl ::core::convert::From<BalanceUpdateFilter> for SpotEngineEvents {
        fn from(value: BalanceUpdateFilter) -> Self {
            Self::BalanceUpdateFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SpotEngineEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<InterestPaymentFilter> for SpotEngineEvents {
        fn from(value: InterestPaymentFilter) -> Self {
            Self::InterestPaymentFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SpotEngineEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProductUpdateFilter> for SpotEngineEvents {
        fn from(value: ProductUpdateFilter) -> Self {
            Self::ProductUpdateFilter(value)
        }
    }
    impl ::core::convert::From<SpotBalanceFilter> for SpotEngineEvents {
        fn from(value: SpotBalanceFilter) -> Self {
            Self::SpotBalanceFilter(value)
        }
    }
    ///Container type for all input parameters for the `addProduct` function with signature `addProduct(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0x25bce8c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addProduct",
        abi = "addProduct(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct AddProductCall {
        pub product_id: u32,
        pub quote_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `assertUtilization` function with signature `assertUtilization(uint32)` and selector `0x4ac8d8c1`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "assertUtilization", abi = "assertUtilization(uint32)")]
    pub struct AssertUtilizationCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getBalance", abi = "getBalance(uint32,bytes32)")]
    pub struct GetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getClearinghouse", abi = "getClearinghouse()")]
    pub struct GetClearinghouseCall;
    ///Container type for all input parameters for the `getConfig` function with signature `getConfig(uint32)` and selector `0xe343738c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getConfig", abi = "getConfig(uint32)")]
    pub struct GetConfigCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `0x8a1d43c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCoreRisk", abi = "getCoreRisk(bytes32,uint32,uint8)")]
    pub struct GetCoreRiskCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getEndpoint", abi = "getEndpoint()")]
    pub struct GetEndpointCall;
    ///Container type for all input parameters for the `getEngineType` function with signature `getEngineType()` and selector `0x4604d19b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getEngineType", abi = "getEngineType()")]
    pub struct GetEngineTypeCall;
    ///Container type for all input parameters for the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `0x871d0912`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getHealthContribution",
        abi = "getHealthContribution(bytes32,uint8)"
    )]
    pub struct GetHealthContributionCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getNlpUnlockedBalance` function with signature `getNlpUnlockedBalance(bytes32)` and selector `0x56778d3f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNlpUnlockedBalance", abi = "getNlpUnlockedBalance(bytes32)")]
    pub struct GetNlpUnlockedBalanceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getProductIds", abi = "getProductIds()")]
    pub struct GetProductIdsCall;
    ///Container type for all input parameters for the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `0xedf02653`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRawBalance", abi = "getRawBalance(uint32,bytes32)")]
    pub struct GetRawBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getRawState` function with signature `getRawState(uint32)` and selector `0xec7a79c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRawState", abi = "getRawState(uint32)")]
    pub struct GetRawStateCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRisk", abi = "getRisk(uint32)")]
    pub struct GetRiskCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSlots` function with signature `getSlots()` and selector `0xfab2c469`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSlots", abi = "getSlots()")]
    pub struct GetSlotsCall;
    ///Container type for all input parameters for the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getStateAndBalance",
        abi = "getStateAndBalance(uint32,bytes32)"
    )]
    pub struct GetStateAndBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getToken` function with signature `getToken(uint32)` and selector `0x45be7ed6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getToken", abi = "getToken(uint32)")]
    pub struct GetTokenCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `0xa67ac322`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTokenBalance", abi = "getTokenBalance(uint32)")]
    pub struct GetTokenBalanceCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getTotalBalances` function with signature `getTotalBalances()` and selector `0x2baf57d3`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getTotalBalances", abi = "getTotalBalances()")]
    pub struct GetTotalBalancesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address)` and selector `0x1459457a`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert(int128[],int128[])` and selector `0x30972b50`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "manualAssert", abi = "manualAssert(int128[],int128[])")]
    pub struct ManualAssertCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub total_deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub total_borrows: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setBalance` function with signature `setBalance(uint32,bytes32,int128)` and selector `0xcbd0808d`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setBalance", abi = "setBalance(uint32,bytes32,int128)")]
    pub struct SetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(uint32,(address,int128,int128,int128,int128,int128,int128))` and selector `0x391b7b3f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setConfig",
        abi = "setConfig(uint32,(address,int128,int128,int128,int128,int128,int128))"
    )]
    pub struct SetConfigCall {
        pub product_id: u32,
        pub config: Config,
    }
    ///Container type for all input parameters for the `setState` function with signature `setState(uint32,(int128,int128,int128,int128))` and selector `0x7fa29d49`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setState",
        abi = "setState(uint32,(int128,int128,int128,int128))"
    )]
    pub struct SetStateCall {
        pub product_id: u32,
        pub state: State,
    }
    ///Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32)` and selector `0x8936f7cd`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "socializeSubaccount", abi = "socializeSubaccount(bytes32)")]
    pub struct SocializeSubaccountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `tryUnlockNlpBalance` function with signature `tryUnlockNlpBalance(bytes32)` and selector `0x19a2ac88`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "tryUnlockNlpBalance", abi = "tryUnlockNlpBalance(bytes32)")]
    pub struct TryUnlockNlpBalanceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `0x21924169`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unsignedUpdateProductTx",
        abi = "unsignedUpdateProductTx((uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))"
    )]
    pub struct UnsignedUpdateProductTxCall {
        pub p: UpdateProductTx,
    }
    ///Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128)` and selector `0xe0b0621f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateBalance", abi = "updateBalance(uint32,bytes32,int128)")]
    pub struct UpdateBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_delta: i128,
    }
    ///Container type for all input parameters for the `updateBalance` function with signature `updateBalance(uint32,bytes32,int128,int128)` and selector `0xf8a42e51`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateBalance",
        abi = "updateBalance(uint32,bytes32,int128,int128)"
    )]
    pub struct UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_delta: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_delta: i128,
    }
    ///Container type for all input parameters for the `updatePrice` function with signature `updatePrice(uint32,int128)` and selector `0x153ca6c0`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updatePrice", abi = "updatePrice(uint32,int128)")]
    pub struct UpdatePriceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all input parameters for the `updateProduct` function with signature `updateProduct(bytes)` and selector `0xc9fe9ac3`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateProduct", abi = "updateProduct(bytes)")]
    pub struct UpdateProductCall {
        pub raw_txn: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `0xf39eeb10`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateQuoteFromInsurance",
        abi = "updateQuoteFromInsurance(bytes32,int128)"
    )]
    pub struct UpdateQuoteFromInsuranceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub insurance: i128,
    }
    ///Container type for all input parameters for the `updateRisk` function with signature `updateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0xc55607b5`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateRisk",
        abi = "updateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct UpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `updateStates` function with signature `updateStates(uint128)` and selector `0xad733b8e`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateStates", abi = "updateStates(uint128)")]
    pub struct UpdateStatesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub dt: u128,
    }
    ///Container type for all of the contract's call
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum SpotEngineCalls {
        AddProduct(AddProductCall),
        AssertUtilization(AssertUtilizationCall),
        GetBalance(GetBalanceCall),
        GetClearinghouse(GetClearinghouseCall),
        GetConfig(GetConfigCall),
        GetCoreRisk(GetCoreRiskCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetHealthContribution(GetHealthContributionCall),
        GetNlpUnlockedBalance(GetNlpUnlockedBalanceCall),
        GetProductIds(GetProductIdsCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawState(GetRawStateCall),
        GetRisk(GetRiskCall),
        GetSlots(GetSlotsCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        GetToken(GetTokenCall),
        GetTokenBalance(GetTokenBalanceCall),
        GetTotalBalances(GetTotalBalancesCall),
        Initialize(InitializeCall),
        ManualAssert(ManualAssertCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBalance(SetBalanceCall),
        SetConfig(SetConfigCall),
        SetState(SetStateCall),
        SocializeSubaccount(SocializeSubaccountCall),
        TransferOwnership(TransferOwnershipCall),
        TryUnlockNlpBalance(TryUnlockNlpBalanceCall),
        UnsignedUpdateProductTx(UnsignedUpdateProductTxCall),
        UpdateBalance(UpdateBalanceCall),
        UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(
            UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall,
        ),
        UpdatePrice(UpdatePriceCall),
        UpdateProduct(UpdateProductCall),
        UpdateQuoteFromInsurance(UpdateQuoteFromInsuranceCall),
        UpdateRisk(UpdateRiskCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpotEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddProductCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddProduct(decoded));
            }
            if let Ok(decoded) =
                <AssertUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssertUtilization(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
            }
            if let Ok(decoded) = <GetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetConfig(decoded));
            }
            if let Ok(decoded) = <GetCoreRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCoreRisk(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) = <GetEngineTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEngineType(decoded));
            }
            if let Ok(decoded) =
                <GetHealthContributionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHealthContribution(decoded));
            }
            if let Ok(decoded) =
                <GetNlpUnlockedBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNlpUnlockedBalance(decoded));
            }
            if let Ok(decoded) = <GetProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetProductIds(decoded));
            }
            if let Ok(decoded) = <GetRawBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawBalance(decoded));
            }
            if let Ok(decoded) = <GetRawStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRawState(decoded));
            }
            if let Ok(decoded) = <GetRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRisk(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
            }
            if let Ok(decoded) =
                <GetStateAndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStateAndBalance(decoded));
            }
            if let Ok(decoded) = <GetTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetToken(decoded));
            }
            if let Ok(decoded) =
                <GetTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTokenBalance(decoded));
            }
            if let Ok(decoded) =
                <GetTotalBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTotalBalances(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBalance(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetState(decoded));
            }
            if let Ok(decoded) =
                <SocializeSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TryUnlockNlpBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TryUnlockNlpBalance(decoded));
            }
            if let Ok(decoded) =
                <UnsignedUpdateProductTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedUpdateProductTx(decoded));
            }
            if let Ok(decoded) = <UpdateBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateBalance(decoded));
            }
            if let Ok(decoded) = <UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(decoded),
                );
            }
            if let Ok(decoded) = <UpdatePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePrice(decoded));
            }
            if let Ok(decoded) = <UpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateProduct(decoded));
            }
            if let Ok(decoded) =
                <UpdateQuoteFromInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateQuoteFromInsurance(decoded));
            }
            if let Ok(decoded) = <UpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateRisk(decoded));
            }
            if let Ok(decoded) = <UpdateStatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateStates(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpotEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssertUtilization(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCoreRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEngineType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHealthContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNlpUnlockedBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProductIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStateAndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTokenBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTotalBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SocializeSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TryUnlockNlpBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedUpdateProductTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateQuoteFromInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateStates(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SpotEngineCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCoreRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthContribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNlpUnlockedBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStateAndBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SocializeSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryUnlockNlpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedUpdateProductTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateQuoteFromInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddProductCall> for SpotEngineCalls {
        fn from(value: AddProductCall) -> Self {
            Self::AddProduct(value)
        }
    }
    impl ::core::convert::From<AssertUtilizationCall> for SpotEngineCalls {
        fn from(value: AssertUtilizationCall) -> Self {
            Self::AssertUtilization(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for SpotEngineCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for SpotEngineCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetConfigCall> for SpotEngineCalls {
        fn from(value: GetConfigCall) -> Self {
            Self::GetConfig(value)
        }
    }
    impl ::core::convert::From<GetCoreRiskCall> for SpotEngineCalls {
        fn from(value: GetCoreRiskCall) -> Self {
            Self::GetCoreRisk(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for SpotEngineCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetEngineTypeCall> for SpotEngineCalls {
        fn from(value: GetEngineTypeCall) -> Self {
            Self::GetEngineType(value)
        }
    }
    impl ::core::convert::From<GetHealthContributionCall> for SpotEngineCalls {
        fn from(value: GetHealthContributionCall) -> Self {
            Self::GetHealthContribution(value)
        }
    }
    impl ::core::convert::From<GetNlpUnlockedBalanceCall> for SpotEngineCalls {
        fn from(value: GetNlpUnlockedBalanceCall) -> Self {
            Self::GetNlpUnlockedBalance(value)
        }
    }
    impl ::core::convert::From<GetProductIdsCall> for SpotEngineCalls {
        fn from(value: GetProductIdsCall) -> Self {
            Self::GetProductIds(value)
        }
    }
    impl ::core::convert::From<GetRawBalanceCall> for SpotEngineCalls {
        fn from(value: GetRawBalanceCall) -> Self {
            Self::GetRawBalance(value)
        }
    }
    impl ::core::convert::From<GetRawStateCall> for SpotEngineCalls {
        fn from(value: GetRawStateCall) -> Self {
            Self::GetRawState(value)
        }
    }
    impl ::core::convert::From<GetRiskCall> for SpotEngineCalls {
        fn from(value: GetRiskCall) -> Self {
            Self::GetRisk(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for SpotEngineCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
        }
    }
    impl ::core::convert::From<GetStateAndBalanceCall> for SpotEngineCalls {
        fn from(value: GetStateAndBalanceCall) -> Self {
            Self::GetStateAndBalance(value)
        }
    }
    impl ::core::convert::From<GetTokenCall> for SpotEngineCalls {
        fn from(value: GetTokenCall) -> Self {
            Self::GetToken(value)
        }
    }
    impl ::core::convert::From<GetTokenBalanceCall> for SpotEngineCalls {
        fn from(value: GetTokenBalanceCall) -> Self {
            Self::GetTokenBalance(value)
        }
    }
    impl ::core::convert::From<GetTotalBalancesCall> for SpotEngineCalls {
        fn from(value: GetTotalBalancesCall) -> Self {
            Self::GetTotalBalances(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SpotEngineCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for SpotEngineCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SpotEngineCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for SpotEngineCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetBalanceCall> for SpotEngineCalls {
        fn from(value: SetBalanceCall) -> Self {
            Self::SetBalance(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for SpotEngineCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for SpotEngineCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SocializeSubaccountCall> for SpotEngineCalls {
        fn from(value: SocializeSubaccountCall) -> Self {
            Self::SocializeSubaccount(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SpotEngineCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TryUnlockNlpBalanceCall> for SpotEngineCalls {
        fn from(value: TryUnlockNlpBalanceCall) -> Self {
            Self::TryUnlockNlpBalance(value)
        }
    }
    impl ::core::convert::From<UnsignedUpdateProductTxCall> for SpotEngineCalls {
        fn from(value: UnsignedUpdateProductTxCall) -> Self {
            Self::UnsignedUpdateProductTx(value)
        }
    }
    impl ::core::convert::From<UpdateBalanceCall> for SpotEngineCalls {
        fn from(value: UpdateBalanceCall) -> Self {
            Self::UpdateBalance(value)
        }
    }
    impl ::core::convert::From<UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall>
        for SpotEngineCalls
    {
        fn from(value: UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall) -> Self {
            Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(value)
        }
    }
    impl ::core::convert::From<UpdatePriceCall> for SpotEngineCalls {
        fn from(value: UpdatePriceCall) -> Self {
            Self::UpdatePrice(value)
        }
    }
    impl ::core::convert::From<UpdateProductCall> for SpotEngineCalls {
        fn from(value: UpdateProductCall) -> Self {
            Self::UpdateProduct(value)
        }
    }
    impl ::core::convert::From<UpdateQuoteFromInsuranceCall> for SpotEngineCalls {
        fn from(value: UpdateQuoteFromInsuranceCall) -> Self {
            Self::UpdateQuoteFromInsurance(value)
        }
    }
    impl ::core::convert::From<UpdateRiskCall> for SpotEngineCalls {
        fn from(value: UpdateRiskCall) -> Self {
            Self::UpdateRisk(value)
        }
    }
    impl ::core::convert::From<UpdateStatesCall> for SpotEngineCalls {
        fn from(value: UpdateStatesCall) -> Self {
            Self::UpdateStates(value)
        }
    }
    ///Container type for all return fields from the `getBalance` function with signature `getBalance(uint32,bytes32)` and selector `0x7c1e1487`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetBalanceReturn(pub Balance);
    ///Container type for all return fields from the `getClearinghouse` function with signature `getClearinghouse()` and selector `0xb1cb0f42`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetClearinghouseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getConfig` function with signature `getConfig(uint32)` and selector `0xe343738c`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetConfigReturn(pub Config);
    ///Container type for all return fields from the `getCoreRisk` function with signature `getCoreRisk(bytes32,uint32,uint8)` and selector `0x8a1d43c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCoreRiskReturn(pub CoreRisk);
    ///Container type for all return fields from the `getEndpoint` function with signature `getEndpoint()` and selector `0xaed8e967`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetEndpointReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getEngineType` function with signature `getEngineType()` and selector `0x4604d19b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetEngineTypeReturn(pub u8);
    ///Container type for all return fields from the `getHealthContribution` function with signature `getHealthContribution(bytes32,uint8)` and selector `0x871d0912`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHealthContributionReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub health: i128,
    }
    ///Container type for all return fields from the `getNlpUnlockedBalance` function with signature `getNlpUnlockedBalance(bytes32)` and selector `0x56778d3f`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNlpUnlockedBalanceReturn(pub Balance);
    ///Container type for all return fields from the `getProductIds` function with signature `getProductIds()` and selector `0x47428e7b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getRawBalance` function with signature `getRawBalance(uint32,bytes32)` and selector `0xedf02653`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRawBalanceReturn(pub BalanceNormalized);
    ///Container type for all return fields from the `getRawState` function with signature `getRawState(uint32)` and selector `0xec7a79c9`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRawStateReturn(pub State);
    ///Container type for all return fields from the `getRisk` function with signature `getRisk(uint32)` and selector `0xecd9cba8`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRiskReturn(pub Risk);
    ///Container type for all return fields from the `getSlots` function with signature `getSlots()` and selector `0xfab2c469`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSlotsReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub balances_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub states_slot: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStateAndBalance` function with signature `getStateAndBalance(uint32,bytes32)` and selector `0xe334be33`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStateAndBalanceReturn(pub State, pub Balance);
    ///Container type for all return fields from the `getToken` function with signature `getToken(uint32)` and selector `0x45be7ed6`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getTokenBalance` function with signature `getTokenBalance(uint32)` and selector `0xa67ac322`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTokenBalanceReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub balance: u128,
    }
    ///Container type for all return fields from the `getTotalBalances` function with signature `getTotalBalances()` and selector `0x2baf57d3`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalBalancesReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub deposits: ::std::vec::Vec<i128>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub borrows: ::std::vec::Vec<i128>,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tryUnlockNlpBalance` function with signature `tryUnlockNlpBalance(bytes32)` and selector `0x19a2ac88`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TryUnlockNlpBalanceReturn(pub Balance);
    ///Container type for all return fields from the `unsignedUpdateProductTx` function with signature `unsignedUpdateProductTx((uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128)))` and selector `0x21924169`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UnsignedUpdateProductTxReturn(pub UpdateProductTx);
    ///Container type for all return fields from the `updateQuoteFromInsurance` function with signature `updateQuoteFromInsurance(bytes32,int128)` and selector `0xf39eeb10`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateQuoteFromInsuranceReturn(pub i128);
    ///`CoreRisk(int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CoreRisk {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight: i128,
    }
    ///`Balance(int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[archive(check_bytes)]
    pub struct Balance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
    }
    ///`BalanceNormalized(int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceNormalized {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount_normalized: i128,
    }
    ///`Config(address,int128,int128,int128,int128,int128,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Config {
        pub token: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_inflection_util_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_floor_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_small_cap_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub interest_large_cap_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub withdraw_fee_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_deposit_rate_x18: i128,
    }
    ///`State(int128,int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[archive(check_bytes)]
    pub struct State {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_deposits_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_borrows_multiplier_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub total_deposits_normalized: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub total_borrows_normalized: i128,
    }
    ///`UpdateProductTx(uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpdateProductTx {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        pub config: Config,
        pub risk_store: RiskStore,
    }
    ///`Risk(int128,int128,int128,int128,int128)`
    #[derive(
        rkyv::Archive,
        rkyv::Serialize,
        rkyv::Deserialize,
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[archive(check_bytes)]
    pub struct Risk {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_initial_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub long_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub short_weight_maintenance_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///`RiskStore(int32,int32,int32,int32,int128)`
    #[derive(
        serde::Serialize,
        serde::Deserialize,
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
}
