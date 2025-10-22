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
                    ::std::borrow::ToOwned::to_owned("addOrUpdateProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addOrUpdateProduct"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNlpLockedBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNlpLockedBalances",),
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
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FSpotEngine.NlpLockedBalances",
                                ),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("AddOrUpdateProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddOrUpdateProduct"),
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
                    ::std::borrow::ToOwned::to_owned("PriceQuery"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PriceQuery"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            indexed: false,
                        },],
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaFv\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\\W`\x005`\xE0\x1C\x80c\xA6z\xC3\"\x11a\x01EW\x80c\xE3Cs\x8C\x11a\0\xBDW\x80c\xF2\xFD\xE3\x8B\x11a\0\x8CW\x80c\xF8\xA4.Q\x11a\0qW\x80c\xF8\xA4.Q\x14a\x085W\x80c\xFA\xB2\xC4i\x14a\x08HW\x80c\xFBH\xC3\xBD\x14a\x08]W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x08\x0FW\x80c\xF3\x9E\xEB\x10\x14a\x08\"W`\0\x80\xFD[\x80c\xE3Cs\x8C\x14a\x05\x8DW\x80c\xECzy\xC9\x14a\x06\xC8W\x80c\xEC\xD9\xCB\xA8\x14a\x07_W\x80c\xED\xF0&S\x14a\x07\xC3W`\0\x80\xFD[\x80c\xC5V\x07\xB5\x11a\x01\x14W\x80c\xDF\x14O\xD5\x11a\0\xF9W\x80c\xDF\x14O\xD5\x14a\x05FW\x80c\xE0\xB0b\x1F\x14a\x05YW\x80c\xE34\xBE3\x14a\x05lW`\0\x80\xFD[\x80c\xC5V\x07\xB5\x14a\x04\xE5W\x80c\xCB\xD0\x80\x8D\x14a\x04\xF8W`\0\x80\xFD[\x80c\xA6z\xC3\"\x14a\x04\x85W\x80c\xADs;\x8E\x14a\x04\xB0W\x80c\xAE\xD8\xE9g\x14a\x04\xC3W\x80c\xB1\xCB\x0FB\x14a\x04\xD4W`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x11a\x01\xD8W\x80c\x7F\xA2\x9DI\x11a\x01\xA7W\x80c\x896\xF7\xCD\x11a\x01\x8CW\x80c\x896\xF7\xCD\x14a\x04\"W\x80c\x8A\x1DC\xC9\x14a\x045W\x80c\x8D\xA5\xCB[\x14a\x04tW`\0\x80\xFD[\x80c\x7F\xA2\x9DI\x14a\x03\xE9W\x80c\x87\x1D\t\x12\x14a\x03\xFCW`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x14a\x03\xA8W\x80cVw\x8D?\x14a\x03\xBBW\x80cqP\x18\xA6\x14a\x03\xCEW\x80c|\x1E\x14\x87\x14a\x03\xD6W`\0\x80\xFD[\x80c0\x97+P\x11a\x02/W\x80cE\xBE~\xD6\x11a\x02\x14W\x80cE\xBE~\xD6\x14a\x03=W\x80cF\x04\xD1\x9B\x14a\x03\x84W\x80cGB\x8E{\x14a\x03\x93W`\0\x80\xFD[\x80c0\x97+P\x14a\x03\x17W\x80c9\x1B{?\x14a\x03*W`\0\x80\xFD[\x80c\x14YEz\x14a\x02aW\x80c\x15<\xA6\xC0\x14a\x02vW\x80c\x19\xA2\xAC\x88\x14a\x02\xD7W\x80c+\xAFW\xD3\x14a\x03\x01W[`\0\x80\xFD[a\x02ta\x02o6`\x04a9 V[a\x08}V[\0[a\x02ta\x02\x846`\x04a9\xB9V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xEAa\x02\xE56`\x04a9\xF0V[a\x0BaV[`@Q\x90Q`\x0F\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\ta\x0C\xE4V[`@Qa\x02\xF8\x92\x91\x90a:GV[a\x02ta\x03%6`\x04a:\xBAV[a\x0F\x0FV[a\x02ta\x0386`\x04a;>V[a\x10\xF9V[a\x03la\x03K6`\x04a;sV[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF8V[`\0`@Qa\x02\xF8\x91\x90a;\xA4V[a\x03\x9Ba\x11 V[`@Qa\x02\xF8\x91\x90a;\xCCV[a\x02ta\x03\xB66`\x04a;sV[a\x11\xA4V[a\x02\xEAa\x03\xC96`\x04a9\xF0V[a\x12;V[a\x02ta\x12}V[a\x02\xEAa\x03\xE46`\x04a<\x16V[a\x12\x89V[a\x02ta\x03\xF76`\x04a<@V[a\x13\x1BV[a\x04\x0Fa\x04\n6`\x04a<\x8EV[a\x13<V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xF8V[a\x02ta\x0406`\x04a9\xF0V[a\x13\xBCV[a\x04Ha\x04C6`\x04a<\xB1V[a\x15\x96V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x02\xF8V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x04\x98a\x04\x936`\x04a;sV[a\x16\x0BV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF8V[a\x02ta\x04\xBE6`\x04a=\x02V[a\x16\xAFV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x02ta\x04\xF36`\x04a=\xE9V[a\x18\xB2V[a\x02ta\x05\x066`\x04a>\x14V[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02ta\x05T6`\x04a>TV[a\x19\xE3V[a\x02ta\x05g6`\x04a>\x14V[a\x1A{V[a\x05\x7Fa\x05z6`\x04a<\x16V[a\x1B3V[`@Qa\x02\xF8\x92\x91\x90a>\xDCV[a\x06Xa\x05\x9B6`\x04a;sV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x82\x01T\x80\x84\x0B``\x83\x01R\x84\x90\x04\x83\x0B`\x80\x82\x01R`\x03\x90\x91\x01T\x80\x83\x0B`\xA0\x83\x01R\x92\x90\x92\x04\x90\x0B`\xC0\x82\x01R\x90V[`@Qa\x02\xF8\x91\x90`\0`\xE0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82R` \x83\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x85\x01Q\x81\x0B`@\x85\x01R``\x85\x01Q\x81\x0B``\x85\x01R`\x80\x85\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x85\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x85\x01Q\x81\x0B`\xC0\x85\x01RPP\x92\x91PPV[a\x07Ra\x06\xD66`\x04a;sV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xF8\x91\x90a?(V[a\x07ra\x07m6`\x04a;sV[a\x1B\xEDV[`@Qa\x02\xF8\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x02\xEAa\x07\xD16`\x04a<\x16V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x81R`l\x84R\x81\x81 \x92\x81R\x91\x83R\x90\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[a\x02ta\x08\x1D6`\x04a?cV[a\x1C!V[a\x04\x0Fa\x0806`\x04a?\x80V[a\x1C\xA9V[a\x02ta\x08C6`\x04a?\xA5V[a\x1D\xD2V[`@\x80Q`l\x81R`k` \x82\x01R\x01a\x02\xF8V[a\x08pa\x08k6`\x04a9\xF0V[a\x1F\x89V[`@Qa\x02\xF8\x91\x90a?\xF6V[a\x08\x89\x85\x85\x84\x84a\"2V[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x84\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x84\x86\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x87\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x89\x01\x82\x81R`\xA0\x80\x8B\x01\x84\x81R`\0`\xC0\x8D\x01\x81\x81R\x81\x80R`j\x8BR\x9CQ\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9D\x16\x17\x90\x9BU\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x97\x88\x16`\x01`\x80\x1B\x91\x89\x16\x82\x02\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x08U\x95Q\x91Q\x91\x87\x16\x91\x87\x16\x86\x02\x91\x90\x91\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\tU\x96Q\x98Q\x98\x85\x16\x98\x85\x16\x84\x02\x98\x90\x98\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\nU\x88Q\x95\x86\x01\x89Rc;\x9A\xCA\0\x80\x87R\x86\x86\x01\x81\x81R\x87\x8B\x01\x82\x81R\x88\x85\x01\x92\x83R\x88\x8B\x01\x84\x81R\x8A\x80R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x89R\x98Q\x7F\xEA#\xA1\xF6\x04v\xEF\x06\xB4\xF1 \x0F\xB4\xED3H\t=rb\xFAI8\xFF\x94|6\xDB\x93\xDC,\xC2\x80T\x93Q\x92Q\x94Q\x9AQ\x89\x16\x90\x97\x02c\xFF\xFF\xFF\xFF\x9A\x8B\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x95\x8C\x16`\x01`@\x1B\x02\x95\x90\x95\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x8C\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x9B\x90\x92\x16\x9A\x90\x9A\x17\x92\x90\x92\x17\x16\x17\x17\x90\x93\x16\x94\x90\x94\x17\x90U\x85Q\x94\x85\x01\x86R\x80\x85R\x90\x84\x01R\x92\x82\x01\x81\x90R\x91\x81\x01\x82\x90Ra\n\xD7\x91\x90a#\xCFV[`g\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x82\x04\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x02\x19\x90\x91\x16\x90U`@Q\x90\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81R`\0\x82\x81R`m` R`@\x90 [`\x01\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01`@\x1B\x90\x92\x04\x16\x10\x80\x15a\x0B\xE0WPa\x0B\xA9a$+V[`\x01\x82\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x84\x90R`@\x90 \x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11\x15[\x15a\x0C\xC8W`\x01\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x82\x90R`@\x81 T`\x02\x83\x01\x80T`\x0F\x92\x83\x0B\x93\x91\x92a\x0C$\x91\x85\x91\x0Ba@\x93V[\x82T`\x01`\x01`\x80\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP`\x01\x81\x81\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R` \x86\x90R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x82U\x94\x01\x80T\x90\x94\x16\x90\x93U\x81T\x04\x90\x91\x16\x90`\x08a\x0C\x9C\x83a@\xE2V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa\x0B\x7FV[`@\x80Q` \x81\x01\x90\x91R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x92\x91PPV[``\x80`\0`g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\raW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\r$W\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x82Wa\r\x82a=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xABW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC8Wa\r\xC8a=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x0F\tW`\0\x82\x82\x81Q\x81\x10a\x0E\x14Wa\x0E\x14aA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0E\x83\x92\x91a$\x9E\x16V[\x86\x84\x81Q\x81\x10a\x0E\x95Wa\x0E\x95aA\tV[` \x02` \x01\x01\x81\x81Qa\x0E\xA9\x91\x90a@\x93V[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x0E\xC9\x93P\x90\x91\x0B\x90a$\x9EV[\x85\x84\x81Q\x81\x10a\x0E\xDBWa\x0E\xDBaA\tV[` \x02` \x01\x01\x81\x81Qa\x0E\xEF\x91\x90a@\x93V[`\x0F\x0B\x90RPa\x0F\x02\x91P\x82\x90PaA\x1FV[\x90Pa\r\xF7V[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x10\xF2W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x0FAWa\x0FAaA\tV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`k\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x0F\xCDWa\x0F\xCDaA\tV[\x90P` \x02\x01` \x81\x01\x90a\x0F\xE2\x91\x90aA8V[`\x0F\x0Ba\x10\x03\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x10LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[`@Q\x80\x91\x03\x90\xFD[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x10hWa\x10haA\tV[\x90P` \x02\x01` \x81\x01\x90a\x10}\x91\x90aA8V[`\x0F\x0Ba\x10\x9E\x82` \x01Q\x83``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x10\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[PPP\x80a\x10\xEB\x90aA\xAAV[\x90Pa\x0F\x12V[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x11\x1A\x82\x82aA\xC6V[PPPPV[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11]W\x90P[PPPPP\x90P\x90V[`\0a\x11\xB1\x82`\x01a\x1B3V[P\x90P`\0a\x11\xD4\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xF6\x83` \x01Q\x84``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x10\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x12S\x82a\x0BaV[PP`\0\x90\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x90V[a\x12\x87`\0a%!V[V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x13\x10\x82\x82a%sV[\x92PPP[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\x11\x1A\x82\x82aB\xF9V[`\0\x80a\x01\0a\x13Ja%\xC7V[a\x13T\x91\x90aC\x9AV[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x13\xB4W`\0a\x13x\x86\x83a&\x16V[\x90P\x80`\0\x03a\x13\x88WPa\x13\xA2V[a\x13\x94\x81\x83\x88\x88a&\xDFV[a\x13\x9E\x90\x85a@\x93V[\x93PP[\x80a\x13\xAC\x81aC\xBDV[\x91PPa\x13YV[PP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0a\x14\x0Ba\x11 V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x15\x91W`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x14?Wa\x14?aA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`l\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x14\xC4\x90\x83\x90a%sV[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x15}W\x81Q`@\x83\x01Q`\0\x91a\x14\xED\x91`\x0F\x0B\x90a$\x9EV[`@\x84\x01Q\x83Q\x91\x92Pa\x15\x0E\x91a\x15\x05\x90\x84a@\x93V[`\x0F\x0B\x90a':V[`\x0F\x0B\x80\x84R`\0\x12a\x15 W`\0\x80\xFD[a\x15>\x83` \x01Q\x83`\0\x01Q`\x0F\x0Ba':\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x15O\x91\x90a@\x93V[`\x0F\x0B\x90RP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x15q\x90\x85\x90\x89\x90a'\xA3V[a\x15{\x84\x84a#\xCFV[P[PPP\x80a\x15\x8A\x90aA\xAAV[\x90Pa\x14\x10V[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x15\xBC\x84a'\xF9V[\x90P`\0a\x15\xCA\x85\x87a)'V[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x15\xFA\x84`\x01\x88a)@V[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a\x16\xA5\x92\x85\x92a\x16x\x92\x91\x90a$\x9E\x16V[a\x16\x96\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16\xA0\x91\x90aC\xD6V[a)\xD7V[a\x16\x04\x90\x83aD&V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01R\x7Fe endpoint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x17ab\x01Q\x80`\x07aDQV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x17\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0[`gTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x15\x91W`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x17\xDFWa\x17\xDFaA\tV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`\n\x19\x81\x01a\x18\x14WPa\x18\xA0V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90a\x18rW\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a\x18\x88WPPa\x18\xA0V[a\x18\x93\x82\x82\x87a)\xF3V[a\x18\x9D\x82\x82a#\xCFV[PP[\x80a\x18\xAA\x81aC\xBDV[\x91PPa\x17\xB4V[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x18\xDEWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x19\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`\0a\x1A\0\x87\x87\x87\x87a\x19\xFB6\x88\x90\x03\x88\x01\x88aD\x81V[a-\xCFV[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` R`@\x90 \x90\x91P\x83\x90a\x1A$\x82\x82aA\xC6V[PP\x80\x15a\x1ArWc\xFF\xFF\xFF\xFF\x87\x16a\x1A<W`\0\x80\xFD[`@\x80Q`\x80\x81\x01\x82Rg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82R` \x82\x01R`\0\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x1Ar\x90\x88\x90a#\xCFV[PPPPPPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90`\n\x19\x01a\x1A\xE3Wa\x1A\xE3\x83\x83a1\xF1V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra\x1B\x1E\x82\x82\x85a3kV[a\x1B)\x85\x85\x83a'\xA3V[a\x10\xF2\x85\x83a#\xCFV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a\x1B\xDF\x81\x83a%sV[\x93P\x93PPP[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x13\x15\x82a'\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[a\x1C\xA6\x81a%!V[PV[`@\x80Q`\x80\x81\x01\x82R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a\x1Dh\x83\x83a%sV[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15a\x1D\xB1W`\0a\x1D\x96a\x1D\x8F\x87a\x1D\x8A\x85aD\x9DV[a4jV[`\0a)\xD7V[\x90Pa\x1D\xA2\x81\x87aC\xD6V[\x95Pa\x1D\xAF\x84\x84\x83a3kV[P[a\x1D\xBC`\0\x84a#\xCFV[a\x1D\xC8`\0\x87\x84a'\xA3V[P\x92\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16a\x1E\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x86\x86R`l\x85R\x83\x86 \x8A\x87R\x85R\x83\x86 \x84Q\x80\x87\x01\x86R\x90T\x84\x0B\x81R\x8A\x87R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x86R\x95\x84\x90 \x84Q\x95\x86\x01\x90\x94R\x92T\x90\x91\x0B\x83R\x93\x90\x92\x91\x90`\n\x19\x01a\x1F=Wa\x1F=\x87\x87a1\xF1V[a\x1FH\x84\x83\x88a3kV[a\x1FS\x83\x82\x87a3kV[a\x1F^\x88\x88\x84a'\xA3V[a\x1Fj`\0\x88\x83a'\xA3V[a\x1Ft\x88\x85a#\xCFV[a\x1F\x7F`\0\x84a#\xCFV[PPPPPPPPV[`@\x80Q`\x80\x80\x82\x01\x83R`\0``\x80\x84\x01\x82\x81R\x84R\x84Q` \x80\x82\x01\x87R\x83\x82R\x80\x86\x01\x91\x90\x91R\x84\x86\x01\x82\x90R\x85Q\x93\x84\x01\x86R\x83\x82\x01\x83\x81R\x84R\x85Q\x80\x82\x01\x87R\x83\x81R\x90\x84\x01R\x93\x82\x01\x93\x90\x93R\x90\x91a\x1F\xEA`\x0B\x85a\x12\x89V[`\0\x85\x81R`m` R`@\x81 `\x01\x81\x01T\x92\x93P\x91`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a \x1Ba$+V[\x90P[`\x01\x83\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10\x15a \x9FWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x84\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x0B\x93\x81\x01\x93\x84R\x92\x83R`\x01\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x83\x01\x82\x90R\x83\x16\x10\x15a \x8CWPa \x9FV[P\x81a \x97\x81a@\xE2V[\x92PPa \x1EV[`\x01\x83\x01Ta \xB9\x90\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aD\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xDBWa \xDBa=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!(W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x91\x81\x01\x82\x81R\x81R` \x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xF9W\x90P[P`@\x86\x01R\x81[`\x01\x84\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x82\x16\x10\x15a!\xF7Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x85\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x0B\x81\x85\x01\x90\x81R\x81R`\x01\x90\x91\x01T`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x90\x87\x01Q\x81\x90a!\xA0\x86\x85aD\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\xBAWa!\xBAaA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R\x81QQ\x90\x88\x01Q\x80Qa!\xDD\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP\x81\x90Pa!\xEF\x81a@\xE2V[\x91PPa!0V[P`@Q\x80` \x01`@R\x80a\" `\0\x88` \x01Q`\0\x01Q\x88`\0\x01Qa\x16\xA0\x91\x90aC\xD6V[`\x0F\x0B\x90R\x85RP\x92\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\"RWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\"lWP0;\x15\x80\x15a\"lWP`\0T`\xFF\x16`\x01\x14[a\"\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a#\x01W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a#\ta4\x7FV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua#-\x82a\x1C!V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x10\xF2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0BRV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua$'\x82a4\xF2V[PPV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90aD\xECV[\x90P\x90V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a$\xE0WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a%\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x82Q`\x0F\x0B\x81\x12\x15a%\x97WP\x82Qa%\x9EV[P` \x83\x01Q[`@\x80Q` \x81\x01\x90\x91R\x83Q\x81\x90a%\xBA\x90`\x0F\x0B\x84a$\x9EV[`\x0F\x0B\x90R\x94\x93PPPPV[`g\x80T`\0\x91\x90a%\xDB\x90`\x01\x90aE\tV[\x81T\x81\x10a%\xEBWa%\xEBaA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a%\x19W`\0`g\x82\x81T\x81\x10a&:Wa&:aA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a&p\x91\x90aE V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a&\xACWPa&\x91\x85`\x01aECV[a&\x9D\x90a\x01\0aE V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a&\xCCWa&\xBDa\x01\0\x82aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a&\xD7\x81aA\x1FV[\x91PPa&\x1BV[`\0\x80a&\xEE\x85a\x01\0aE V[\x90P[\x85\x15a'1W`\x01\x86\x16\x15a'\x18Wa'\x0B\x81\x85\x85a5.V[a'\x15\x90\x83a@\x93V[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a')\x81aC\xBDV[\x91PPa&\xF1V[P\x94\x93PPPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a'~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a$\xB5Wa$\xB5aC\x84V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x90\x91U`\x0F\x0B\x15\x15a'\xEF\x83\x85\x83a6!V[a\x11\x1A\x84\x84a6\xDEV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x80\x86\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x88\x16\x84R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83R\x92\x86\x90 \x86Q\x94\x85\x01\x87RT`\x03\x81\x81\x0B\x80\x87Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x87\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x97\x86\x01\x97\x90\x97R`\x01``\x1B\x81\x04\x90\x96\x0B\x90\x84\x01R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x82\x01R\x90\x91a(\xB5\x90c;\x9A\xCA\0aE\x85V[`\x0F\x0B\x82R` \x81\x01Qa(\xD0\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x0B` \x83\x01R`@\x81\x01Qa(\xEE\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x0B`@\x83\x01R``\x81\x01Qa)\x0C\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x91\x90PV[`\0\x80a)4\x84\x84a\x12\x89V[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15a)VWa)Va;\x8EV[\x03a)jWPg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x04V[`\0\x80\x84`\x0F\x0B\x12a)\xA3W`\0\x83`\x02\x81\x11\x15a)\x8AWa)\x8Aa;\x8EV[\x14a)\x99W\x84`@\x01Qa)\x9CV[\x84Q[\x90Pa)\xCFV[`\0\x83`\x02\x81\x11\x15a)\xB7Wa)\xB7a;\x8EV[\x14a)\xC6W\x84``\x01Qa)\xCCV[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a)\xECW\x81a\x16\x04V[P\x90\x91\x90PV[`\0\x80a*\x14\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a*6\x85` \x01Q\x86``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a*H`\x0F\x83\x90\x0B\x84a':V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x83\x01T\x80\x86\x0B``\x84\x01R\x81\x90\x04\x85\x0B`\x80\x83\x01R`\x03\x90\x92\x01T\x80\x85\x0B`\xA0\x83\x01R\x91\x90\x91\x04\x83\x0B`\xC0\x82\x01R\x93\x94P\x91\x92\x91\x90\x84\x90\x0B\x83\x03a*\xDFWP`\0a+{V[\x81` \x01Q`\x0F\x0B\x84`\x0F\x0B\x12\x15a+$Wa+\x13\x82` \x01Qa\x15\x05\x86\x85``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a+\x1D\x90\x82a@\x93V[\x90Pa+{V[a+_a+Q\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a+B\x91\x90aC\xD6V[` \x85\x01Qa\x15\x05\x90\x88aC\xD6V[`\x80\x84\x01Q`\x0F\x0B\x90a$\x9EV[\x82``\x01Qa+n\x91\x90a@\x93V[a+x\x90\x82a@\x93V[\x90P[a+\x96a+\x8Bc\x01\xE13\x80a7!V[`\x0F\x83\x90\x0B\x90a':V[\x90Pa+\xB7\x88a+\xAE\x83g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[`\x0F\x0B\x90a7\x9AV[\x96P\x81`\xC0\x01Q\x92PPP`\0a+\xE6g\r\xE0\xB6\xB3\xA7d\0\0\x87a+\xDB\x91\x90aC\xD6V[`\x0F\x85\x90\x0B\x90a$\x9EV[\x90P`\0a,\x12a,\x07g\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0aC\xD6V[`\x0F\x84\x90\x0B\x90a$\x9EV[\x90P`\0a,.a,#\x83\x85aC\xD6V[`\x0F\x89\x90\x0B\x90a$\x9EV[` \x8B\x01Q\x90\x91Pa,C\x90`\x0F\x0B\x89a$\x9EV[`\x0F\x0B` \x8B\x01R`\0a,_\x83g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[\x8BQ\x90\x91Pa,q\x90`\x0F\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B\x8CR\x82\x90\x0B\x15a,\xC9Wc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra,\xBB\x8C\x82\x85a3kV[a,\xC7\x8D`\0\x83a'\xA3V[P[\x84`\x0F\x0B`\0\x14a-`W`\0a,\xF1a,\xE6c\x01\xE13\x80a7!V[`\x0F\x88\x90\x0B\x90a':V[\x90P`\0a-\x0B\x8Ca+\xAE\x84g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[` \x8E\x01Q\x90\x91Pa- \x90`\x0F\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B` \x8F\x01R\x8DQa-7\x91\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B\x8ERa-K\x90\x84\x90\x0B\x82a$\x9EV[\x92Pa-[`\x0F\x8C\x90\x0B\x82a$\x9EV[\x9APPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8E\x16\x81R`\x01`\x01`\x80\x1B\x03\x8C\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8B\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPPV[`\0\x81`@\x01Q`\x03\x0B\x82`\0\x01Q`\x03\x0B\x13\x15\x80\x15a-\xFAWPc;\x9A\xCA\0\x82`@\x01Q`\x03\x0B\x13\x15[\x80\x15a.\x14WP\x81``\x01Q`\x03\x0B\x82` \x01Q`\x03\x0B\x12\x15[\x80\x15a.+WPc;\x9A\xCA\0\x82``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a.eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`gT\x15\x80a.\xC5WP`g\x80Ta.\x80\x90`\x01\x90aE\tV[\x81T\x81\x10a.\x90Wa.\x90aA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x11[\x15a/\x94W`g\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x08\x81\x04\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04`\x07\x90\x95\x16\x85\x02a\x01\0\n\x90\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U`fT`@QcC\xB1j\x11`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x87b\xD4\"\x91a/]\x91\x8A\x91\x01c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/wW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x8BW=`\0\x80>=`\0\xFD[PPPP`\x01\x90P[\x80\x15a0\x1CW`eT`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01Q`@Qc\x1C\xB2\x81[`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R`\x0F\x91\x90\x91\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9e\x02\xB6\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x13W=`\0\x80>=`\0\xFD[PPPPa0aV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B`\x80\x83\x01R[c\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` \x90\x81R`@\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x93\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0\x95\x87\x16\x95\x90\x95\x02\x94\x90\x94\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x94\x86\x16\x94\x90\x94\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x16\x93\x90\x93\x17`\x01``\x1B\x92\x90\x94\x16\x91\x90\x91\x02\x92\x90\x92\x17`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x91\x90\x92\x16\x02\x17\x90Ua13a8\"V[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x89\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`\x0F\x86\x81\x0B`D\x83\x01R\x85\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\xAAW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x89\x16\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1\x95\x94PPPPPV[`\x01\x19\x82\x01a1\xFEWPPV[a2\x07\x82a\x0BaV[P`\0\x81`\x0F\x0B\x13\x15a2\xEDW`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R`\x0F\x85\x90\x0B\x93\x81\x01\x93\x84R\x92\x83R\x91\x90\x81\x01b\x05F\0a2La$+V[a2V\x91\x90aD&V[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x91R`\x01\x80\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x87\x81R`@\x82 \x87QQ\x81T\x90\x88\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x82\x16\x17\x82U\x97\x90\x91\x01Q\x94\x01\x80T\x94\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x90\x92U\x81T\x16\x91a2\xC1\x83a@\xE2V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\0\x81`\x0F\x0B\x12\x15a$'W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x80\x82R\x82\x90\x82\x90a3.\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP`\0\x92\x83R`m` R`@\x90\x92 \x91Q`\x02\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15a3\x99W\x81Q`@\x84\x01\x80Qa3\x8E\x90\x83\x90aC\xD6V[`\x0F\x0B\x90RPa3\xB3V[\x81Q``\x84\x01\x80Qa3\xAC\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15a3\xCCWP\x82Qa3\xD3V[P` \x83\x01Q[\x82Q`\0\x90\x83\x90a3\xE7\x90`\x0F\x0B\x84a$\x9EV[a3\xF1\x91\x90a@\x93V[\x90P`\0\x81`\x0F\x0B\x13\x15a4\x08W\x84Q\x91Pa4\x10V[\x84` \x01Q\x91P[a4\x1E`\x0F\x82\x90\x0B\x83a':V[`\x0F\x0B\x80\x85R`\0\x12\x15a4JW\x83Q`@\x86\x01\x80Qa4?\x90\x83\x90a@\x93V[`\x0F\x0B\x90RPa\x10\xF2V[\x83Q``\x86\x01\x80Qa4]\x90\x83\x90aC\xD6V[`\x0F\x0B\x90RPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a)\xECW\x81a\x16\x04V[`\0Ta\x01\0\x90\x04`\xFF\x16a4\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x10CV[a\x12\x87a8\x97V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a5:\x85a'\xF9V[\x90P`\0\x80a5I\x87\x87a)'V[\x91P\x91P`\0a5Z\x84\x84\x88a)@V[\x90Pa5f\x82\x86a@\x93V[\x94P\x82`\x0F\x0B`\0\x14a6\x16Wa5\x86g\r\xE0\xB6\xB3\xA7d\0\0`\x02aE\x85V[`\x0F\x0B\x81`\x0F\x0B\x03a5\xAFWo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94PPPPPa\x16\x04V[`\x80\x84\x01Qa5\xCF\x90a5\xC6`\x0F\x86\x90\x0B\x84a$\x9EV[`\x0F\x0B\x90a$\x9EV[a5\xD9\x90\x86a@\x93V[`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x90\x95P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPP\x93\x92PPPV[\x80\x15a6\x82Wa63a\x01\0\x83aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a6`\x91\x90aC\x9AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a6\x8Ea\x01\0\x83aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a6\xBC\x91\x90aC\x9AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a7ZWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a7\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a7\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13a%\x19W\x80\x84\x16`\x0F\x0B\x15a8\x0EWa8\x0B\x82\x86a$\x9EV[\x91P[a8\x18\x85\x86a$\x9EV[\x94P`\x02\x02a7\xE9V[`\0a86`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90aF#V[`\0Ta\x01\0\x90\x04`\xFF\x16a9\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x10CV[a\x12\x873a%!V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a98W`\0\x80\xFD[\x855a9C\x81a9\x0BV[\x94P` \x86\x015a9S\x81a9\x0BV[\x93P`@\x86\x015a9c\x81a9\x0BV[\x92P``\x86\x015a9s\x81a9\x0BV[\x91P`\x80\x86\x015a9\x83\x81a9\x0BV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a9\xA5W`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a9\xCCW`\0\x80\xFD[a9\xD5\x83a9\x91V[\x91P` \x83\x015a9\xE5\x81a9\xAAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a:\x02W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:<W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a:\x1DV[P\x94\x95\x94PPPPPV[`@\x81R`\0a:Z`@\x83\x01\x85a:\tV[\x82\x81\x03` \x84\x01Ra:l\x81\x85a:\tV[\x95\x94PPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a:\x87W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x9FW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B\xE6W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a:\xD0W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a:\xE8W`\0\x80\xFD[a:\xF4\x88\x83\x89\x01a:uV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a;\rW`\0\x80\xFD[Pa;\x1A\x87\x82\x88\x01a:uV[\x95\x98\x94\x97P\x95PPPPV[`\0`\xE0\x82\x84\x03\x12\x15a;8W`\0\x80\xFD[P\x91\x90PV[`\0\x80a\x01\0\x83\x85\x03\x12\x15a;RW`\0\x80\xFD[a;[\x83a9\x91V[\x91Pa;j\x84` \x85\x01a;&V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a;\x85W`\0\x80\xFD[a\x16\x04\x82a9\x91V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a;\xC6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a<\nW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a;\xE8V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a<)W`\0\x80\xFD[a<2\x83a9\x91V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a<TW`\0\x80\xFD[a<]\x84a9\x91V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a<qW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a9\xA5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a<\xA1W`\0\x80\xFD[\x825\x91Pa;j` \x84\x01a<\x7FV[`\0\x80`\0``\x84\x86\x03\x12\x15a<\xC6W`\0\x80\xFD[\x835\x92Pa<\xD6` \x85\x01a9\x91V[\x91Pa<\xE4`@\x85\x01a<\x7FV[\x90P\x92P\x92P\x92V[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x14W`\0\x80\xFD[\x815a\x16\x04\x81a<\xEDV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14a9\xA5W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a=YW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a=\x8AWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a=\x99\x83a=5V[\x81Ra=\xA7` \x84\x01a=5V[` \x82\x01Ra=\xB8`@\x84\x01a=5V[`@\x82\x01Ra=\xC9``\x84\x01a=5V[``\x82\x01R`\x80\x83\x015a=\xDC\x81a9\xAAV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15a=\xFCW`\0\x80\xFD[a>\x05\x83a9\x91V[\x91Pa;j\x84` \x85\x01a=GV[`\0\x80`\0``\x84\x86\x03\x12\x15a>)W`\0\x80\xFD[a>2\x84a9\x91V[\x92P` \x84\x015\x91P`@\x84\x015a>I\x81a9\xAAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15a>oW`\0\x80\xFD[a>x\x88a9\x91V[\x96Pa>\x86` \x89\x01a9\x91V[\x95P`@\x88\x015a>\x96\x81a9\xAAV[\x94P``\x88\x015a>\xA6\x81a9\xAAV[\x93Pa>\xB5\x89`\x80\x8A\x01a;&V[\x92P`\xA0a\x01_\x19\x82\x01\x12\x15a>\xCAW`\0\x80\xFD[Pa\x01`\x87\x01\x90P\x92\x95P\x92\x95P\x92\x95V[`\xA0\x81\x01a?\x17\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x0B`\x80\x83\x01R\x93\x92PPPV[`\x80\x81\x01a\x13\x15\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15a?uW`\0\x80\xFD[\x815a\x16\x04\x81a9\x0BV[`\0\x80`@\x83\x85\x03\x12\x15a?\x93W`\0\x80\xFD[\x825\x91P` \x83\x015a9\xE5\x81a9\xAAV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a?\xBBW`\0\x80\xFD[a?\xC4\x85a9\x91V[\x93P` \x85\x015\x92P`@\x85\x015a?\xDB\x81a9\xAAV[\x91P``\x85\x015a?\xEB\x81a9\xAAV[\x93\x96\x92\x95P\x90\x93PPV[` \x80\x82R\x82QQ`\x0F\x90\x81\x0B\x83\x83\x01R\x83\x82\x01QQ\x81\x0B`@\x80\x85\x01\x91\x90\x91R\x80\x85\x01Q``\x80\x86\x01R\x80Q`\x80\x86\x01\x81\x90R`\0\x94\x93\x91\x84\x01\x92\x85\x92\x91`\xA0\x88\x01\x90[\x80\x85\x10\x15a@pW\x85Q\x80QQ\x85\x0B\x83R\x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x87\x83\x01R\x94\x86\x01\x94`\x01\x94\x90\x94\x01\x93\x90\x82\x01\x90a@;V[P\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a@\xBDWa@\xBDa@}V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a@\xD9Wa@\xD9a@}V[P\x01\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA1WaA1a@}V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aAJW`\0\x80\xFD[\x815a\x16\x04\x81a9\xAAV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aA\x82W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aAfV[\x81\x81\x11\x15aA\x94W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[\x815aA\xD1\x81a9\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015aA\xFD\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015aB%\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015aBV\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015aB~\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x03\x81\x01`\xA0\x83\x015aB\xAF\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\xC0\x83\x015aB\xD7\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x11\x1AV[\x815aC\x04\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015aC,\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015aC\\\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015aB\xD7\x81a9\xAAV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aC\xB1WaC\xB1aC\x84V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aD\x01WaD\x01a@}V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aD\x1CWaD\x1Ca@}V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDHWaDHa@}V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aDxWaDxa@}V[\x02\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15aD\x93W`\0\x80\xFD[a\x16\x04\x83\x83a=GV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aD\xBAWaD\xBAa@}V[`\0\x03\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aD\xE4WaD\xE4a@}V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aD\xFEW`\0\x80\xFD[\x81Qa\x16\x04\x81a<\xEDV[`\0\x82\x82\x10\x15aE\x1BWaE\x1Ba@}V[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aDxWaDxa@}V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDHWaDHa@}V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aEyWaEyaC\x84V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aE\xB5WaE\xB5a@}V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aE\xE1WaE\xE1a@}V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aE\xFDWaE\xFDa@}V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aF\x13WaF\x13a@}V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aF5W`\0\x80\xFD[\x81Qa\x16\x04\x81a9\x0BV\xFE\xA2dipfsX\"\x12 B\x05\xD3\xC8*h\x7F\xEC(rz\x97\xFA_he\x08(Fm[|\xAA7\xA6\xCA\x17,\xF4p\xA2WdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static SPOTENGINE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\\W`\x005`\xE0\x1C\x80c\xA6z\xC3\"\x11a\x01EW\x80c\xE3Cs\x8C\x11a\0\xBDW\x80c\xF2\xFD\xE3\x8B\x11a\0\x8CW\x80c\xF8\xA4.Q\x11a\0qW\x80c\xF8\xA4.Q\x14a\x085W\x80c\xFA\xB2\xC4i\x14a\x08HW\x80c\xFBH\xC3\xBD\x14a\x08]W`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x08\x0FW\x80c\xF3\x9E\xEB\x10\x14a\x08\"W`\0\x80\xFD[\x80c\xE3Cs\x8C\x14a\x05\x8DW\x80c\xECzy\xC9\x14a\x06\xC8W\x80c\xEC\xD9\xCB\xA8\x14a\x07_W\x80c\xED\xF0&S\x14a\x07\xC3W`\0\x80\xFD[\x80c\xC5V\x07\xB5\x11a\x01\x14W\x80c\xDF\x14O\xD5\x11a\0\xF9W\x80c\xDF\x14O\xD5\x14a\x05FW\x80c\xE0\xB0b\x1F\x14a\x05YW\x80c\xE34\xBE3\x14a\x05lW`\0\x80\xFD[\x80c\xC5V\x07\xB5\x14a\x04\xE5W\x80c\xCB\xD0\x80\x8D\x14a\x04\xF8W`\0\x80\xFD[\x80c\xA6z\xC3\"\x14a\x04\x85W\x80c\xADs;\x8E\x14a\x04\xB0W\x80c\xAE\xD8\xE9g\x14a\x04\xC3W\x80c\xB1\xCB\x0FB\x14a\x04\xD4W`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x11a\x01\xD8W\x80c\x7F\xA2\x9DI\x11a\x01\xA7W\x80c\x896\xF7\xCD\x11a\x01\x8CW\x80c\x896\xF7\xCD\x14a\x04\"W\x80c\x8A\x1DC\xC9\x14a\x045W\x80c\x8D\xA5\xCB[\x14a\x04tW`\0\x80\xFD[\x80c\x7F\xA2\x9DI\x14a\x03\xE9W\x80c\x87\x1D\t\x12\x14a\x03\xFCW`\0\x80\xFD[\x80cJ\xC8\xD8\xC1\x14a\x03\xA8W\x80cVw\x8D?\x14a\x03\xBBW\x80cqP\x18\xA6\x14a\x03\xCEW\x80c|\x1E\x14\x87\x14a\x03\xD6W`\0\x80\xFD[\x80c0\x97+P\x11a\x02/W\x80cE\xBE~\xD6\x11a\x02\x14W\x80cE\xBE~\xD6\x14a\x03=W\x80cF\x04\xD1\x9B\x14a\x03\x84W\x80cGB\x8E{\x14a\x03\x93W`\0\x80\xFD[\x80c0\x97+P\x14a\x03\x17W\x80c9\x1B{?\x14a\x03*W`\0\x80\xFD[\x80c\x14YEz\x14a\x02aW\x80c\x15<\xA6\xC0\x14a\x02vW\x80c\x19\xA2\xAC\x88\x14a\x02\xD7W\x80c+\xAFW\xD3\x14a\x03\x01W[`\0\x80\xFD[a\x02ta\x02o6`\x04a9 V[a\x08}V[\0[a\x02ta\x02\x846`\x04a9\xB9V[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xEAa\x02\xE56`\x04a9\xF0V[a\x0BaV[`@Q\x90Q`\x0F\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\ta\x0C\xE4V[`@Qa\x02\xF8\x92\x91\x90a:GV[a\x02ta\x03%6`\x04a:\xBAV[a\x0F\x0FV[a\x02ta\x0386`\x04a;>V[a\x10\xF9V[a\x03la\x03K6`\x04a;sV[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF8V[`\0`@Qa\x02\xF8\x91\x90a;\xA4V[a\x03\x9Ba\x11 V[`@Qa\x02\xF8\x91\x90a;\xCCV[a\x02ta\x03\xB66`\x04a;sV[a\x11\xA4V[a\x02\xEAa\x03\xC96`\x04a9\xF0V[a\x12;V[a\x02ta\x12}V[a\x02\xEAa\x03\xE46`\x04a<\x16V[a\x12\x89V[a\x02ta\x03\xF76`\x04a<@V[a\x13\x1BV[a\x04\x0Fa\x04\n6`\x04a<\x8EV[a\x13<V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xF8V[a\x02ta\x0406`\x04a9\xF0V[a\x13\xBCV[a\x04Ha\x04C6`\x04a<\xB1V[a\x15\x96V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x80\x85\x01Q\x82\x0B\x90\x83\x01R\x92\x82\x01Q\x90\x92\x0B\x90\x82\x01R``\x01a\x02\xF8V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x04\x98a\x04\x936`\x04a;sV[a\x16\x0BV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF8V[a\x02ta\x04\xBE6`\x04a=\x02V[a\x16\xAFV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x03lV[a\x02ta\x04\xF36`\x04a=\xE9V[a\x18\xB2V[a\x02ta\x05\x066`\x04a>\x14V[c\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02ta\x05T6`\x04a>TV[a\x19\xE3V[a\x02ta\x05g6`\x04a>\x14V[a\x1A{V[a\x05\x7Fa\x05z6`\x04a<\x16V[a\x1B3V[`@Qa\x02\xF8\x92\x91\x90a>\xDCV[a\x06Xa\x05\x9B6`\x04a;sV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\xE0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x90\x81\x90\x04\x84\x0B\x94\x82\x01\x94\x90\x94R`\x02\x82\x01T\x80\x84\x0B``\x83\x01R\x84\x90\x04\x83\x0B`\x80\x82\x01R`\x03\x90\x91\x01T\x80\x83\x0B`\xA0\x83\x01R\x92\x90\x92\x04\x90\x0B`\xC0\x82\x01R\x90V[`@Qa\x02\xF8\x91\x90`\0`\xE0\x82\x01\x90P`\x01`\x01`\xA0\x1B\x03\x83Q\x16\x82R` \x83\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x85\x01Q\x81\x0B`@\x85\x01R``\x85\x01Q\x81\x0B``\x85\x01R`\x80\x85\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x85\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x85\x01Q\x81\x0B`\xC0\x85\x01RPP\x92\x91PPV[a\x07Ra\x06\xD66`\x04a;sV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xF8\x91\x90a?(V[a\x07ra\x07m6`\x04a;sV[a\x1B\xEDV[`@Qa\x02\xF8\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x02\xEAa\x07\xD16`\x04a<\x16V[`@\x80Q` \x80\x82\x01\x83R`\0\x91\x82\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x81R`l\x84R\x81\x81 \x92\x81R\x91\x83R\x90\x81\x90 \x81Q\x92\x83\x01\x90\x91RT`\x0F\x0B\x81R\x90V[a\x02ta\x08\x1D6`\x04a?cV[a\x1C!V[a\x04\x0Fa\x0806`\x04a?\x80V[a\x1C\xA9V[a\x02ta\x08C6`\x04a?\xA5V[a\x1D\xD2V[`@\x80Q`l\x81R`k` \x82\x01R\x01a\x02\xF8V[a\x08pa\x08k6`\x04a9\xF0V[a\x1F\x89V[`@Qa\x02\xF8\x91\x90a?\xF6V[a\x08\x89\x85\x85\x84\x84a\"2V[`@\x80Q`\xE0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82Rg\x0B\x1A+\xC2\xECP\0\0` \x80\x84\x01\x91\x82Rf#\x86\xF2o\xC1\0\0\x84\x86\x01\x90\x81Rf\x8E\x1B\xC9\xBF\x04\0\0``\x80\x87\x01\x91\x82Rg\r\xE0\xB6\xB3\xA7d\0\0`\x80\x80\x89\x01\x82\x81R`\xA0\x80\x8B\x01\x84\x81R`\0`\xC0\x8D\x01\x81\x81R\x81\x80R`j\x8BR\x9CQ\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x9D\x16\x17\x90\x9BU\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x97\x88\x16`\x01`\x80\x1B\x91\x89\x16\x82\x02\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\x08U\x95Q\x91Q\x91\x87\x16\x91\x87\x16\x86\x02\x91\x90\x91\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\tU\x96Q\x98Q\x98\x85\x16\x98\x85\x16\x84\x02\x98\x90\x98\x17\x7F`!\xFA\x82\xDE\x88\x19\x96\xA3\xE5\xFD-\x03/t\xDF\xE7'F\xB8\xA6lU\x10\xD4\xAB\x1A<\xB7\x89\x15\nU\x88Q\x95\x86\x01\x89Rc;\x9A\xCA\0\x80\x87R\x86\x86\x01\x81\x81R\x87\x8B\x01\x82\x81R\x88\x85\x01\x92\x83R\x88\x8B\x01\x84\x81R\x8A\x80R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x89R\x98Q\x7F\xEA#\xA1\xF6\x04v\xEF\x06\xB4\xF1 \x0F\xB4\xED3H\t=rb\xFAI8\xFF\x94|6\xDB\x93\xDC,\xC2\x80T\x93Q\x92Q\x94Q\x9AQ\x89\x16\x90\x97\x02c\xFF\xFF\xFF\xFF\x9A\x8B\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x95\x8C\x16`\x01`@\x1B\x02\x95\x90\x95\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x93\x8C\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x9B\x90\x92\x16\x9A\x90\x9A\x17\x92\x90\x92\x17\x16\x17\x17\x90\x93\x16\x94\x90\x94\x17\x90U\x85Q\x94\x85\x01\x86R\x80\x85R\x90\x84\x01R\x92\x82\x01\x81\x90R\x91\x81\x01\x82\x90Ra\n\xD7\x91\x90a#\xCFV[`g\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE`\x08\x82\x04\x01\x80T`\x07\x90\x92\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x02\x19\x90\x91\x16\x90U`@Q\x90\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81R`\0\x82\x81R`m` R`@\x90 [`\x01\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16`\x01`@\x1B\x90\x92\x04\x16\x10\x80\x15a\x0B\xE0WPa\x0B\xA9a$+V[`\x01\x82\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x84\x90R`@\x90 \x01T`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x91\x16\x11\x15[\x15a\x0C\xC8W`\x01\x81\x01T`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x82\x90R`@\x81 T`\x02\x83\x01\x80T`\x0F\x92\x83\x0B\x93\x91\x92a\x0C$\x91\x85\x91\x0Ba@\x93V[\x82T`\x01`\x01`\x80\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x91\x90\x92\x02\x19\x90\x91\x16\x17\x90UP`\x01\x81\x81\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x01`@\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R` \x86\x90R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x82U\x94\x01\x80T\x90\x94\x16\x90\x93U\x81T\x04\x90\x91\x16\x90`\x08a\x0C\x9C\x83a@\xE2V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPa\x0B\x7FV[`@\x80Q` \x81\x01\x90\x91R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x92\x91PPV[``\x80`\0`g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\raW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\r$W\x90P[PPPPP\x90P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x82Wa\r\x82a=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xABW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xC8Wa\r\xC8a=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81Q\x81\x10\x15a\x0F\tW`\0\x82\x82\x81Q\x81\x10a\x0E\x14Wa\x0E\x14aA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k\x84R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x80\x84R`\x01`\x80\x1B\x92\x83\x90\x04\x82\x0B\x98\x84\x01\x98\x90\x98R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x85\x90R\x04\x90\x91\x0B``\x82\x01R\x92\x94P\x91\x92a\x0E\x83\x92\x91a$\x9E\x16V[\x86\x84\x81Q\x81\x10a\x0E\x95Wa\x0E\x95aA\tV[` \x02` \x01\x01\x81\x81Qa\x0E\xA9\x91\x90a@\x93V[`\x0F\x90\x81\x0B\x90\x91R` \x83\x01Q``\x84\x01Qa\x0E\xC9\x93P\x90\x91\x0B\x90a$\x9EV[\x85\x84\x81Q\x81\x10a\x0E\xDBWa\x0E\xDBaA\tV[` \x02` \x01\x01\x81\x81Qa\x0E\xEF\x91\x90a@\x93V[`\x0F\x0B\x90RPa\x0F\x02\x91P\x82\x90PaA\x1FV[\x90Pa\r\xF7V[PP\x90\x91V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x84\x11\x15a\x10\xF2W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x0FAWa\x0FAaA\tV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`k\x82R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01\x90\x92\x01T\x80\x85\x0B\x95\x82\x01\x95\x90\x95R\x93\x04\x90\x91\x0B``\x83\x01R\x91P\x86\x86`\x01`\x01`\x80\x1B\x03\x85\x16\x81\x81\x10a\x0F\xCDWa\x0F\xCDaA\tV[\x90P` \x02\x01` \x81\x01\x90a\x0F\xE2\x91\x90aA8V[`\x0F\x0Ba\x10\x03\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x10LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[`@Q\x80\x91\x03\x90\xFD[P\x84\x84\x84`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\x10hWa\x10haA\tV[\x90P` \x02\x01` \x81\x01\x90a\x10}\x91\x90aA8V[`\x0F\x0Ba\x10\x9E\x82` \x01Q\x83``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x0F\x0B\x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01dDSYNC`\xD8\x1B\x81RP\x90a\x10\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[PPP\x80a\x10\xEB\x90aA\xAAV[\x90Pa\x0F\x12V[PPPPPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x11\x1A\x82\x82aA\xC6V[PPPPV[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9AW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11]W\x90P[PPPPP\x90P\x90V[`\0a\x11\xB1\x82`\x01a\x1B3V[P\x90P`\0a\x11\xD4\x82`\0\x01Q\x83`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xF6\x83` \x01Q\x84``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80`\x0F\x0B\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aMU`\xF0\x1B\x81RP\x90a\x10\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x12S\x82a\x0BaV[PP`\0\x90\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x81R\x90V[a\x12\x87`\0a%!V[V[`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x87\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81Ra\x13\x10\x82\x82a%sV[\x92PPP[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` R`@\x90 \x81\x90a\x11\x1A\x82\x82aB\xF9V[`\0\x80a\x01\0a\x13Ja%\xC7V[a\x13T\x91\x90aC\x9AV[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x13\xB4W`\0a\x13x\x86\x83a&\x16V[\x90P\x80`\0\x03a\x13\x88WPa\x13\xA2V[a\x13\x94\x81\x83\x88\x88a&\xDFV[a\x13\x9E\x90\x85a@\x93V[\x93PP[\x80a\x13\xAC\x81aC\xBDV[\x91PPa\x13YV[PP\x92\x91PPV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0a\x14\x0Ba\x11 V[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x15\x91W`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x14?Wa\x14?aA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k\x84R`@\x80\x82 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x8A\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x93\x83R`l\x86R\x81\x83 \x8A\x84R\x86R\x81\x83 \x82Q\x96\x87\x01\x90\x92R\x90T\x90\x0B\x84R\x91\x93P\x91a\x14\xC4\x90\x83\x90a%sV[\x90P`\0\x81`\0\x01Q`\x0F\x0B\x12\x15a\x15}W\x81Q`@\x83\x01Q`\0\x91a\x14\xED\x91`\x0F\x0B\x90a$\x9EV[`@\x84\x01Q\x83Q\x91\x92Pa\x15\x0E\x91a\x15\x05\x90\x84a@\x93V[`\x0F\x0B\x90a':V[`\x0F\x0B\x80\x84R`\0\x12a\x15 W`\0\x80\xFD[a\x15>\x83` \x01Q\x83`\0\x01Q`\x0F\x0Ba':\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83``\x01\x81\x81Qa\x15O\x91\x90a@\x93V[`\x0F\x0B\x90RP`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x15q\x90\x85\x90\x89\x90a'\xA3V[a\x15{\x84\x84a#\xCFV[P[PPP\x80a\x15\x8A\x90aA\xAAV[\x90Pa\x14\x10V[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x15\xBC\x84a'\xF9V[\x90P`\0a\x15\xCA\x85\x87a)'V[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x15\xFA\x84`\x01\x88a)@V[`\x0F\x0B\x90R\x92PPP[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x80\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x95\x83\x01\x86\x90R`\x01\x90\x93\x01T\x80\x84\x0B\x94\x83\x01\x94\x90\x94R\x90\x92\x04\x90\x0B``\x82\x01\x81\x90R\x92\x93\x90\x92a\x16\xA5\x92\x85\x92a\x16x\x92\x91\x90a$\x9E\x16V[a\x16\x96\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x16\xA0\x91\x90aC\xD6V[a)\xD7V[a\x16\x04\x90\x83aD&V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01R\x7Fe endpoint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x17ab\x01Q\x80`\x07aDQV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x17\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0[`gTc\xFF\xFF\xFF\xFF\x82\x16\x10\x15a\x15\x91W`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x17\xDFWa\x17\xDFaA\tV[`\0\x91\x82R` \x90\x91 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P`\n\x19\x81\x01a\x18\x14WPa\x18\xA0V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90a\x18rW\x80\x93P[\x80`@\x01Q`\x0F\x0B`\0\x03a\x18\x88WPPa\x18\xA0V[a\x18\x93\x82\x82\x87a)\xF3V[a\x18\x9D\x82\x82a#\xCFV[PP[\x80a\x18\xAA\x81aC\xBDV[\x91PPa\x17\xB4V[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x18\xDEWP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x19\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x92\x8A\x16`\x01`@\x1B\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`\0a\x1A\0\x87\x87\x87\x87a\x19\xFB6\x88\x90\x03\x88\x01\x88aD\x81V[a-\xCFV[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` R`@\x90 \x90\x91P\x83\x90a\x1A$\x82\x82aA\xC6V[PP\x80\x15a\x1ArWc\xFF\xFF\xFF\xFF\x87\x16a\x1A<W`\0\x80\xFD[`@\x80Q`\x80\x81\x01\x82Rg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82R` \x82\x01R`\0\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x1Ar\x90\x88\x90a#\xCFV[PPPPPPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x81\x81R`k` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90`\n\x19\x01a\x1A\xE3Wa\x1A\xE3\x83\x83a1\xF1V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra\x1B\x1E\x82\x82\x85a3kV[a\x1B)\x85\x85\x83a'\xA3V[a\x10\xF2\x85\x83a#\xCFV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`@\x80Q` \x81\x01\x90\x91R`\0\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x90\x93\x01T\x80\x84\x0B\x83\x86\x01R\x04\x82\x0B``\x82\x01R\x94\x84R`l\x83R\x81\x84 \x88\x85R\x83R\x92\x81\x90 \x81Q\x92\x83\x01\x90\x91RT\x90\x91\x0B\x81R\x81a\x1B\xDF\x81\x83a%sV[\x93P\x93PPP[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x13\x15\x82a'\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[a\x1C\xA6\x81a%!V[PV[`@\x80Q`\x80\x81\x01\x82R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B` \x80\x85\x01\x91\x90\x91R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x83\x0B\x85\x87\x01R\x92\x90\x92\x04\x81\x0B``\x84\x01R`\0\x86\x81R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x83R\x84\x81 \x85Q\x93\x84\x01\x90\x95R\x93T\x90\x0B\x81R\x82a\x1Dh\x83\x83a%sV[Q\x90P`\0`\x0F\x82\x90\x0B\x12\x15a\x1D\xB1W`\0a\x1D\x96a\x1D\x8F\x87a\x1D\x8A\x85aD\x9DV[a4jV[`\0a)\xD7V[\x90Pa\x1D\xA2\x81\x87aC\xD6V[\x95Pa\x1D\xAF\x84\x84\x83a3kV[P[a\x1D\xBC`\0\x84a#\xCFV[a\x1D\xC8`\0\x87\x84a'\xA3V[P\x92\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16a\x1E\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[Pc\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`k` \x90\x81R`@\x80\x83 \x81Q`\x80\x80\x82\x01\x84R\x82T`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x84\x88\x01R`\x01\x90\x94\x01T\x80\x85\x0B\x84\x87\x01R\x81\x90\x04\x84\x0B``\x80\x85\x01\x91\x90\x91R\x85Q\x92\x83\x01\x86R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8BT\x80\x86\x0B\x84R\x82\x90\x04\x85\x0B\x83\x88\x01R\x7F\xC8\xCC\x8B\xDAz\xD4\x88k\xEA>\xBB\xDA\xFA\x02\xE7\x9D7\xC3\x9B\xF4\x01\x16\x96\xB2j1\xA0\x80/\xD9E\x8CT\x80\x86\x0B\x84\x88\x01R\x91\x90\x91\x04\x84\x0B\x90\x82\x01R\x86\x86R`l\x85R\x83\x86 \x8A\x87R\x85R\x83\x86 \x84Q\x80\x87\x01\x86R\x90T\x84\x0B\x81R\x8A\x87R\x7F\x7F\xEB\xD3G\xDF\x14\xEA5\xC5)\xE5\x0F\xB2\xDDb\x9DJb&\xF5\xCC\xC8\x93q\x0F\xB4f\xF8\xB88#\xFC\x86R\x95\x84\x90 \x84Q\x95\x86\x01\x90\x94R\x92T\x90\x91\x0B\x83R\x93\x90\x92\x91\x90`\n\x19\x01a\x1F=Wa\x1F=\x87\x87a1\xF1V[a\x1FH\x84\x83\x88a3kV[a\x1FS\x83\x82\x87a3kV[a\x1F^\x88\x88\x84a'\xA3V[a\x1Fj`\0\x88\x83a'\xA3V[a\x1Ft\x88\x85a#\xCFV[a\x1F\x7F`\0\x84a#\xCFV[PPPPPPPPV[`@\x80Q`\x80\x80\x82\x01\x83R`\0``\x80\x84\x01\x82\x81R\x84R\x84Q` \x80\x82\x01\x87R\x83\x82R\x80\x86\x01\x91\x90\x91R\x84\x86\x01\x82\x90R\x85Q\x93\x84\x01\x86R\x83\x82\x01\x83\x81R\x84R\x85Q\x80\x82\x01\x87R\x83\x81R\x90\x84\x01R\x93\x82\x01\x93\x90\x93R\x90\x91a\x1F\xEA`\x0B\x85a\x12\x89V[`\0\x85\x81R`m` R`@\x81 `\x01\x81\x01T\x92\x93P\x91`\x01`@\x1B\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90a \x1Ba$+V[\x90P[`\x01\x83\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x83\x16\x10\x15a \x9FWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R` \x84\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x0B\x93\x81\x01\x93\x84R\x92\x83R`\x01\x01T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x91\x83\x01\x82\x90R\x83\x16\x10\x15a \x8CWPa \x9FV[P\x81a \x97\x81a@\xE2V[\x92PPa \x1EV[`\x01\x83\x01Ta \xB9\x90\x83\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aD\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xDBWa \xDBa=\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!(W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x91\x81\x01\x82\x81R\x81R` \x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xF9W\x90P[P`@\x86\x01R\x81[`\x01\x84\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x82\x16\x10\x15a!\xF7Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x85\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T`\x0F\x0B\x81\x85\x01\x90\x81R\x81R`\x01\x90\x91\x01T`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x90\x87\x01Q\x81\x90a!\xA0\x86\x85aD\xC3V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\xBAWa!\xBAaA\tV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R\x81QQ\x90\x88\x01Q\x80Qa!\xDD\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP\x81\x90Pa!\xEF\x81a@\xE2V[\x91PPa!0V[P`@Q\x80` \x01`@R\x80a\" `\0\x88` \x01Q`\0\x01Q\x88`\0\x01Qa\x16\xA0\x91\x90aC\xD6V[`\x0F\x0B\x90R\x85RP\x92\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\"RWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\"lWP0;\x15\x80\x15a\"lWP`\0T`\xFF\x16`\x01\x14[a\"\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x10CV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a#\x01W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a#\ta4\x7FV[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua#-\x82a\x1C!V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x10\xF2W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0BRV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua$'\x82a4\xF2V[PPV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a$uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90aD\xECV[\x90P\x90V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a$\xE0WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a%\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x80\x82R\x82Q`\x0F\x0B\x81\x12\x15a%\x97WP\x82Qa%\x9EV[P` \x83\x01Q[`@\x80Q` \x81\x01\x90\x91R\x83Q\x81\x90a%\xBA\x90`\x0F\x0B\x84a$\x9EV[`\x0F\x0B\x90R\x94\x93PPPPV[`g\x80T`\0\x91\x90a%\xDB\x90`\x01\x90aE\tV[\x81T\x81\x10a%\xEBWa%\xEBaA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a%\x19W`\0`g\x82\x81T\x81\x10a&:Wa&:aA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a&p\x91\x90aE V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a&\xACWPa&\x91\x85`\x01aECV[a&\x9D\x90a\x01\0aE V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a&\xCCWa&\xBDa\x01\0\x82aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a&\xD7\x81aA\x1FV[\x91PPa&\x1BV[`\0\x80a&\xEE\x85a\x01\0aE V[\x90P[\x85\x15a'1W`\x01\x86\x16\x15a'\x18Wa'\x0B\x81\x85\x85a5.V[a'\x15\x90\x83a@\x93V[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a')\x81aC\xBDV[\x91PPa&\xF1V[P\x94\x93PPPPV[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a'~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a$\xB5Wa$\xB5aC\x84V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81Q\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x90\x91U`\x0F\x0B\x15\x15a'\xEF\x83\x85\x83a6!V[a\x11\x1A\x84\x84a6\xDEV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x80\x86\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x88\x16\x84R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83R\x92\x86\x90 \x86Q\x94\x85\x01\x87RT`\x03\x81\x81\x0B\x80\x87Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x87\x01\x94\x90\x94R`\x01`@\x1B\x82\x04\x81\x0B\x97\x86\x01\x97\x90\x97R`\x01``\x1B\x81\x04\x90\x96\x0B\x90\x84\x01R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x82\x01R\x90\x91a(\xB5\x90c;\x9A\xCA\0aE\x85V[`\x0F\x0B\x82R` \x81\x01Qa(\xD0\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x0B` \x83\x01R`@\x81\x01Qa(\xEE\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x0B`@\x83\x01R``\x81\x01Qa)\x0C\x90`\x03\x0Bc;\x9A\xCA\0aE\x85V[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x91\x90PV[`\0\x80a)4\x84\x84a\x12\x89V[Q\x94`\0\x94P\x92PPPV[`\0`\x02\x82`\x02\x81\x11\x15a)VWa)Va;\x8EV[\x03a)jWPg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x04V[`\0\x80\x84`\x0F\x0B\x12a)\xA3W`\0\x83`\x02\x81\x11\x15a)\x8AWa)\x8Aa;\x8EV[\x14a)\x99W\x84`@\x01Qa)\x9CV[\x84Q[\x90Pa)\xCFV[`\0\x83`\x02\x81\x11\x15a)\xB7Wa)\xB7a;\x8EV[\x14a)\xC6W\x84``\x01Qa)\xCCV[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a)\xECW\x81a\x16\x04V[P\x90\x91\x90PV[`\0\x80a*\x14\x84`\0\x01Q\x85`@\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a*6\x85` \x01Q\x86``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a*H`\x0F\x83\x90\x0B\x84a':V[c\xFF\xFF\xFF\xFF\x88\x16`\0\x90\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\xE0\x81\x01\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01T`\x0F\x81\x81\x0B\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x81\x90\x04\x85\x0B\x93\x82\x01\x84\x90R`\x02\x83\x01T\x80\x86\x0B``\x84\x01R\x81\x90\x04\x85\x0B`\x80\x83\x01R`\x03\x90\x92\x01T\x80\x85\x0B`\xA0\x83\x01R\x91\x90\x91\x04\x83\x0B`\xC0\x82\x01R\x93\x94P\x91\x92\x91\x90\x84\x90\x0B\x83\x03a*\xDFWP`\0a+{V[\x81` \x01Q`\x0F\x0B\x84`\x0F\x0B\x12\x15a+$Wa+\x13\x82` \x01Qa\x15\x05\x86\x85``\x01Q`\x0F\x0Ba$\x9E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a+\x1D\x90\x82a@\x93V[\x90Pa+{V[a+_a+Q\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a+B\x91\x90aC\xD6V[` \x85\x01Qa\x15\x05\x90\x88aC\xD6V[`\x80\x84\x01Q`\x0F\x0B\x90a$\x9EV[\x82``\x01Qa+n\x91\x90a@\x93V[a+x\x90\x82a@\x93V[\x90P[a+\x96a+\x8Bc\x01\xE13\x80a7!V[`\x0F\x83\x90\x0B\x90a':V[\x90Pa+\xB7\x88a+\xAE\x83g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[`\x0F\x0B\x90a7\x9AV[\x96P\x81`\xC0\x01Q\x92PPP`\0a+\xE6g\r\xE0\xB6\xB3\xA7d\0\0\x87a+\xDB\x91\x90aC\xD6V[`\x0F\x85\x90\x0B\x90a$\x9EV[\x90P`\0a,\x12a,\x07g\x02\xC6\x8A\xF0\xBB\x14\0\0g\r\xE0\xB6\xB3\xA7d\0\0aC\xD6V[`\x0F\x84\x90\x0B\x90a$\x9EV[\x90P`\0a,.a,#\x83\x85aC\xD6V[`\x0F\x89\x90\x0B\x90a$\x9EV[` \x8B\x01Q\x90\x91Pa,C\x90`\x0F\x0B\x89a$\x9EV[`\x0F\x0B` \x8B\x01R`\0a,_\x83g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[\x8BQ\x90\x91Pa,q\x90`\x0F\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B\x8CR\x82\x90\x0B\x15a,\xC9Wc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`l` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R\x90T`\x0F\x0B\x81Ra,\xBB\x8C\x82\x85a3kV[a,\xC7\x8D`\0\x83a'\xA3V[P[\x84`\x0F\x0B`\0\x14a-`W`\0a,\xF1a,\xE6c\x01\xE13\x80a7!V[`\x0F\x88\x90\x0B\x90a':V[\x90P`\0a-\x0B\x8Ca+\xAE\x84g\r\xE0\xB6\xB3\xA7d\0\0a@\x93V[` \x8E\x01Q\x90\x91Pa- \x90`\x0F\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B` \x8F\x01R\x8DQa-7\x91\x0B\x82a$\x9EV[`\x0F\x90\x81\x0B\x8ERa-K\x90\x84\x90\x0B\x82a$\x9EV[\x92Pa-[`\x0F\x8C\x90\x0B\x82a$\x9EV[\x9APPP[`@\x80Qc\xFF\xFF\xFF\xFF\x8E\x16\x81R`\x01`\x01`\x80\x1B\x03\x8C\x16` \x82\x01R`\x0F\x83\x81\x0B\x82\x84\x01R\x8B\x81\x0B``\x83\x01R\x84\x90\x0B`\x80\x82\x01R\x90Q\x7Fj\xC0eP\xB1\xD7uj\xFB\x13\xAE\x15\xBD\xB7\xF0\t\x83\x8E\xEBI\x18h\xF6\xCE\xA5fIh\xB8\xEDq\xFD\x91\x81\x90\x03`\xA0\x01\x90\xA1PPPPPPPPPPPPV[`\0\x81`@\x01Q`\x03\x0B\x82`\0\x01Q`\x03\x0B\x13\x15\x80\x15a-\xFAWPc;\x9A\xCA\0\x82`@\x01Q`\x03\x0B\x13\x15[\x80\x15a.\x14WP\x81``\x01Q`\x03\x0B\x82` \x01Q`\x03\x0B\x12\x15[\x80\x15a.+WPc;\x9A\xCA\0\x82``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a.eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P`gT\x15\x80a.\xC5WP`g\x80Ta.\x80\x90`\x01\x90aE\tV[\x81T\x81\x10a.\x90Wa.\x90aA\tV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x11[\x15a/\x94W`g\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x08\x81\x04\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04`\x07\x90\x95\x16\x85\x02a\x01\0\n\x90\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U`fT`@QcC\xB1j\x11`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x87b\xD4\"\x91a/]\x91\x8A\x91\x01c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/wW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x8BW=`\0\x80>=`\0\xFD[PPPP`\x01\x90P[\x80\x15a0\x1CW`eT`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01Q`@Qc\x1C\xB2\x81[`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R`\x0F\x91\x90\x91\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9e\x02\xB6\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x13W=`\0\x80>=`\0\xFD[PPPPa0aV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B`\x80\x83\x01R[c\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` \x90\x81R`@\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x93\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0\x95\x87\x16\x95\x90\x95\x02\x94\x90\x94\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x94\x86\x16\x94\x90\x94\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x16\x93\x90\x93\x17`\x01``\x1B\x92\x90\x94\x16\x91\x90\x91\x02\x92\x90\x92\x17`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x91\x90\x92\x16\x02\x17\x90Ua13a8\"V[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x89\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`\x0F\x86\x81\x0B`D\x83\x01R\x85\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\xAAW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x89\x16\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1\x95\x94PPPPPV[`\x01\x19\x82\x01a1\xFEWPPV[a2\x07\x82a\x0BaV[P`\0\x81`\x0F\x0B\x13\x15a2\xEDW`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R`\x0F\x85\x90\x0B\x93\x81\x01\x93\x84R\x92\x83R\x91\x90\x81\x01b\x05F\0a2La$+V[a2V\x91\x90aD&V[`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x90\x91R`\x01\x80\x84\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R` \x87\x81R`@\x82 \x87QQ\x81T\x90\x88\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x82\x16\x17\x82U\x97\x90\x91\x01Q\x94\x01\x80T\x94\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x90\x92U\x81T\x16\x91a2\xC1\x83a@\xE2V[\x91\x90a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\0\x81`\x0F\x0B\x12\x15a$'W`\0\x82\x81R`m` \x90\x81R`@\x91\x82\x90 \x82Q\x91\x82\x01\x90\x92R`\x02\x90\x91\x01T`\x0F\x0B\x80\x82R\x82\x90\x82\x90a3.\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP`\0\x92\x83R`m` R`@\x90\x92 \x91Q`\x02\x90\x92\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91UPV[`\0\x82`\0\x01Q`\x0F\x0B\x13\x15a3\x99W\x81Q`@\x84\x01\x80Qa3\x8E\x90\x83\x90aC\xD6V[`\x0F\x0B\x90RPa3\xB3V[\x81Q``\x84\x01\x80Qa3\xAC\x90\x83\x90a@\x93V[`\x0F\x0B\x90RP[`\0\x80\x83`\0\x01Q`\x0F\x0B\x13\x15a3\xCCWP\x82Qa3\xD3V[P` \x83\x01Q[\x82Q`\0\x90\x83\x90a3\xE7\x90`\x0F\x0B\x84a$\x9EV[a3\xF1\x91\x90a@\x93V[\x90P`\0\x81`\x0F\x0B\x13\x15a4\x08W\x84Q\x91Pa4\x10V[\x84` \x01Q\x91P[a4\x1E`\x0F\x82\x90\x0B\x83a':V[`\x0F\x0B\x80\x85R`\0\x12\x15a4JW\x83Q`@\x86\x01\x80Qa4?\x90\x83\x90a@\x93V[`\x0F\x0B\x90RPa\x10\xF2V[\x83Q``\x86\x01\x80Qa4]\x90\x83\x90aC\xD6V[`\x0F\x0B\x90RPPPPPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a)\xECW\x81a\x16\x04V[`\0Ta\x01\0\x90\x04`\xFF\x16a4\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x10CV[a\x12\x87a8\x97V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a5:\x85a'\xF9V[\x90P`\0\x80a5I\x87\x87a)'V[\x91P\x91P`\0a5Z\x84\x84\x88a)@V[\x90Pa5f\x82\x86a@\x93V[\x94P\x82`\x0F\x0B`\0\x14a6\x16Wa5\x86g\r\xE0\xB6\xB3\xA7d\0\0`\x02aE\x85V[`\x0F\x0B\x81`\x0F\x0B\x03a5\xAFWo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x94PPPPPa\x16\x04V[`\x80\x84\x01Qa5\xCF\x90a5\xC6`\x0F\x86\x90\x0B\x84a$\x9EV[`\x0F\x0B\x90a$\x9EV[a5\xD9\x90\x86a@\x93V[`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x90\x95P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPP\x93\x92PPPV[\x80\x15a6\x82Wa63a\x01\0\x83aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a6`\x91\x90aC\x9AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a6\x8Ea\x01\0\x83aEbV[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a6\xBC\x91\x90aC\x9AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a7ZWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a7\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[P\x92\x91PPV[`\0\x80\x82`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a7\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10C\x91\x90aAUV[Pg\r\xE0\xB6\xB3\xA7d\0\0`\x01[\x83`\x0F\x0B\x81`\x0F\x0B\x13a%\x19W\x80\x84\x16`\x0F\x0B\x15a8\x0EWa8\x0B\x82\x86a$\x9EV[\x91P[a8\x18\x85\x86a$\x9EV[\x94P`\x02\x02a7\xE9V[`\0a86`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x99\x91\x90aF#V[`\0Ta\x01\0\x90\x04`\xFF\x16a9\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x10CV[a\x12\x873a%!V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a98W`\0\x80\xFD[\x855a9C\x81a9\x0BV[\x94P` \x86\x015a9S\x81a9\x0BV[\x93P`@\x86\x015a9c\x81a9\x0BV[\x92P``\x86\x015a9s\x81a9\x0BV[\x91P`\x80\x86\x015a9\x83\x81a9\x0BV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a9\xA5W`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a9\xCCW`\0\x80\xFD[a9\xD5\x83a9\x91V[\x91P` \x83\x015a9\xE5\x81a9\xAAV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a:\x02W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a:<W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a:\x1DV[P\x94\x95\x94PPPPPV[`@\x81R`\0a:Z`@\x83\x01\x85a:\tV[\x82\x81\x03` \x84\x01Ra:l\x81\x85a:\tV[\x95\x94PPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a:\x87W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x9FW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B\xE6W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a:\xD0W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a:\xE8W`\0\x80\xFD[a:\xF4\x88\x83\x89\x01a:uV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a;\rW`\0\x80\xFD[Pa;\x1A\x87\x82\x88\x01a:uV[\x95\x98\x94\x97P\x95PPPPV[`\0`\xE0\x82\x84\x03\x12\x15a;8W`\0\x80\xFD[P\x91\x90PV[`\0\x80a\x01\0\x83\x85\x03\x12\x15a;RW`\0\x80\xFD[a;[\x83a9\x91V[\x91Pa;j\x84` \x85\x01a;&V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a;\x85W`\0\x80\xFD[a\x16\x04\x82a9\x91V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a;\xC6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a<\nW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a;\xE8V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a<)W`\0\x80\xFD[a<2\x83a9\x91V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a<TW`\0\x80\xFD[a<]\x84a9\x91V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a<qW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a9\xA5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a<\xA1W`\0\x80\xFD[\x825\x91Pa;j` \x84\x01a<\x7FV[`\0\x80`\0``\x84\x86\x03\x12\x15a<\xC6W`\0\x80\xFD[\x835\x92Pa<\xD6` \x85\x01a9\x91V[\x91Pa<\xE4`@\x85\x01a<\x7FV[\x90P\x92P\x92P\x92V[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x14W`\0\x80\xFD[\x815a\x16\x04\x81a<\xEDV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x805`\x03\x81\x90\x0B\x81\x14a9\xA5W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a=YW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a=\x8AWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a=\x99\x83a=5V[\x81Ra=\xA7` \x84\x01a=5V[` \x82\x01Ra=\xB8`@\x84\x01a=5V[`@\x82\x01Ra=\xC9``\x84\x01a=5V[``\x82\x01R`\x80\x83\x015a=\xDC\x81a9\xAAV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15a=\xFCW`\0\x80\xFD[a>\x05\x83a9\x91V[\x91Pa;j\x84` \x85\x01a=GV[`\0\x80`\0``\x84\x86\x03\x12\x15a>)W`\0\x80\xFD[a>2\x84a9\x91V[\x92P` \x84\x015\x91P`@\x84\x015a>I\x81a9\xAAV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15a>oW`\0\x80\xFD[a>x\x88a9\x91V[\x96Pa>\x86` \x89\x01a9\x91V[\x95P`@\x88\x015a>\x96\x81a9\xAAV[\x94P``\x88\x015a>\xA6\x81a9\xAAV[\x93Pa>\xB5\x89`\x80\x8A\x01a;&V[\x92P`\xA0a\x01_\x19\x82\x01\x12\x15a>\xCAW`\0\x80\xFD[Pa\x01`\x87\x01\x90P\x92\x95P\x92\x95P\x92\x95V[`\xA0\x81\x01a?\x17\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x0B`\x80\x83\x01R\x93\x92PPPV[`\x80\x81\x01a\x13\x15\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15a?uW`\0\x80\xFD[\x815a\x16\x04\x81a9\x0BV[`\0\x80`@\x83\x85\x03\x12\x15a?\x93W`\0\x80\xFD[\x825\x91P` \x83\x015a9\xE5\x81a9\xAAV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a?\xBBW`\0\x80\xFD[a?\xC4\x85a9\x91V[\x93P` \x85\x015\x92P`@\x85\x015a?\xDB\x81a9\xAAV[\x91P``\x85\x015a?\xEB\x81a9\xAAV[\x93\x96\x92\x95P\x90\x93PPV[` \x80\x82R\x82QQ`\x0F\x90\x81\x0B\x83\x83\x01R\x83\x82\x01QQ\x81\x0B`@\x80\x85\x01\x91\x90\x91R\x80\x85\x01Q``\x80\x86\x01R\x80Q`\x80\x86\x01\x81\x90R`\0\x94\x93\x91\x84\x01\x92\x85\x92\x91`\xA0\x88\x01\x90[\x80\x85\x10\x15a@pW\x85Q\x80QQ\x85\x0B\x83R\x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x87\x83\x01R\x94\x86\x01\x94`\x01\x94\x90\x94\x01\x93\x90\x82\x01\x90a@;V[P\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a@\xBDWa@\xBDa@}V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a@\xD9Wa@\xD9a@}V[P\x01\x93\x92PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA1WaA1a@}V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aAJW`\0\x80\xFD[\x815a\x16\x04\x81a9\xAAV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aA\x82W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aAfV[\x81\x81\x11\x15aA\x94W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[\x815aA\xD1\x81a9\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83T\x16\x17\x82UP`\x01\x81\x01` \x83\x015aA\xFD\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`@\x83\x015aB%\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x02\x81\x01``\x83\x015aBV\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\x80\x83\x015aB~\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UPP`\x03\x81\x01`\xA0\x83\x015aB\xAF\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP`\xC0\x83\x015aB\xD7\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x11\x1AV[\x815aC\x04\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015aC,\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015aC\\\x81a9\xAAV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015aB\xD7\x81a9\xAAV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aC\xB1WaC\xB1aC\x84V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a@\xFFWa@\xFFa@}V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aD\x01WaD\x01a@}V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aD\x1CWaD\x1Ca@}V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDHWaDHa@}V[\x01\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aDxWaDxa@}V[\x02\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15aD\x93W`\0\x80\xFD[a\x16\x04\x83\x83a=GV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aD\xBAWaD\xBAa@}V[`\0\x03\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aD\xE4WaD\xE4a@}V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aD\xFEW`\0\x80\xFD[\x81Qa\x16\x04\x81a<\xEDV[`\0\x82\x82\x10\x15aE\x1BWaE\x1Ba@}V[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aDxWaDxa@}V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDHWaDHa@}V[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aEyWaEyaC\x84V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aE\xB5WaE\xB5a@}V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aE\xE1WaE\xE1a@}V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aE\xFDWaE\xFDa@}V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aF\x13WaF\x13a@}V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aF5W`\0\x80\xFD[\x81Qa\x16\x04\x81a9\x0BV\xFE\xA2dipfsX\"\x12 B\x05\xD3\xC8*h\x7F\xEC(rz\x97\xFA_he\x08(Fm[|\xAA7\xA6\xCA\x17,\xF4p\xA2WdsolcC\0\x08\r\x003";
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
        ///Calls the contract's `addOrUpdateProduct` (0xdf144fd5) function
        pub fn add_or_update_product(
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
                    [223, 20, 79, 213],
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
        ///Calls the contract's `getNlpLockedBalances` (0xfb48c3bd) function
        pub fn get_nlp_locked_balances(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, NlpLockedBalances> {
            self.0
                .method_hash([251, 72, 195, 189], subaccount)
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
        ///Gets the contract's `AddOrUpdateProduct` event
        pub fn add_or_update_product_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddOrUpdateProductFilter>
        {
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
        ///Gets the contract's `PriceQuery` event
        pub fn price_query_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PriceQueryFilter> {
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
    #[ethevent(name = "AddOrUpdateProduct", abi = "AddOrUpdateProduct(uint32)")]
    pub struct AddOrUpdateProductFilter {
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
    #[ethevent(name = "PriceQuery", abi = "PriceQuery(uint32)")]
    pub struct PriceQueryFilter {
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
        AddOrUpdateProductFilter(AddOrUpdateProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        InitializedFilter(InitializedFilter),
        InterestPaymentFilter(InterestPaymentFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceQueryFilter(PriceQueryFilter),
        ProductUpdateFilter(ProductUpdateFilter),
        SpotBalanceFilter(SpotBalanceFilter),
    }
    impl ::ethers::contract::EthLogDecode for SpotEngineEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddOrUpdateProductFilter::decode_log(log) {
                return Ok(SpotEngineEvents::AddOrUpdateProductFilter(decoded));
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
            if let Ok(decoded) = PriceQueryFilter::decode_log(log) {
                return Ok(SpotEngineEvents::PriceQueryFilter(decoded));
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
                Self::AddOrUpdateProductFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterestPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceQueryFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotBalanceFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddOrUpdateProductFilter> for SpotEngineEvents {
        fn from(value: AddOrUpdateProductFilter) -> Self {
            Self::AddOrUpdateProductFilter(value)
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
    impl ::core::convert::From<PriceQueryFilter> for SpotEngineEvents {
        fn from(value: PriceQueryFilter) -> Self {
            Self::PriceQueryFilter(value)
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
    ///Container type for all input parameters for the `addOrUpdateProduct` function with signature `addOrUpdateProduct(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0xdf144fd5`
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
        name = "addOrUpdateProduct",
        abi = "addOrUpdateProduct(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct AddOrUpdateProductCall {
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
    ///Container type for all input parameters for the `getNlpLockedBalances` function with signature `getNlpLockedBalances(bytes32)` and selector `0xfb48c3bd`
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
    #[ethcall(name = "getNlpLockedBalances", abi = "getNlpLockedBalances(bytes32)")]
    pub struct GetNlpLockedBalancesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
        AddOrUpdateProduct(AddOrUpdateProductCall),
        AssertUtilization(AssertUtilizationCall),
        GetBalance(GetBalanceCall),
        GetClearinghouse(GetClearinghouseCall),
        GetConfig(GetConfigCall),
        GetCoreRisk(GetCoreRiskCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetHealthContribution(GetHealthContributionCall),
        GetNlpLockedBalances(GetNlpLockedBalancesCall),
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
        UpdateBalance(UpdateBalanceCall),
        UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(
            UpdateBalanceWithProductIdAndSubaccountAndQuoteDeltaCall,
        ),
        UpdatePrice(UpdatePriceCall),
        UpdateQuoteFromInsurance(UpdateQuoteFromInsuranceCall),
        UpdateRisk(UpdateRiskCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpotEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddOrUpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddOrUpdateProduct(decoded));
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
                <GetNlpLockedBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNlpLockedBalances(decoded));
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
                Self::AddOrUpdateProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::GetNlpLockedBalances(element) => {
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
                Self::UpdateBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AddOrUpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCoreRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthContribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNlpLockedBalances(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UpdateBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalanceWithProductIdAndSubaccountAndQuoteDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateQuoteFromInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddOrUpdateProductCall> for SpotEngineCalls {
        fn from(value: AddOrUpdateProductCall) -> Self {
            Self::AddOrUpdateProduct(value)
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
    impl ::core::convert::From<GetNlpLockedBalancesCall> for SpotEngineCalls {
        fn from(value: GetNlpLockedBalancesCall) -> Self {
            Self::GetNlpLockedBalances(value)
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
    ///Container type for all return fields from the `getNlpLockedBalances` function with signature `getNlpLockedBalances(bytes32)` and selector `0xfb48c3bd`
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
    pub struct GetNlpLockedBalancesReturn(pub NlpLockedBalances);
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
    ///`NlpLockedBalances((int128),(int128),((int128),uint128)[])`
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
    pub struct NlpLockedBalances {
        pub balance_unlocked: Balance,
        pub balance_locked: Balance,
        pub balances: ::std::vec::Vec<NlpLockedBalance>,
    }
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
    ///`NlpLockedBalance((int128),uint128)`
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
    pub struct NlpLockedBalance {
        pub balance: Balance,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub unlocked_at: u128,
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
