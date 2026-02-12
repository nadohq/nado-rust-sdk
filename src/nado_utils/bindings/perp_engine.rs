pub use perp_engine::*;
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
#[allow(clippy::module_inception)]
#[allow(clippy::useless_conversion)]
#[allow(clippy::large_enum_variant)]
pub mod perp_engine {
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
                    ::std::borrow::ToOwned::to_owned("balances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balances"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("vQuoteBalance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastCumulativeFundingX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitBalanceUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("emitBalanceUpdate"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAvailableSettle"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAvailableSettle"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
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
                    ::std::borrow::ToOwned::to_owned("getPositionPnl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPositionPnl"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
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
                                ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
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
                    ::std::borrow::ToOwned::to_owned("getSettlementState"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSettlementState"),
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
                                name: ::std::borrow::ToOwned::to_owned("availableSettle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
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
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
                                ),
                            },
                        ],
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
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
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
                                name: ::std::string::String::new(),
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
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_states"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrationFlag"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("migrationFlag"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("perpPositionClosed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpPositionClosed"),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
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
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.Balance",),
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
                                    ::std::borrow::ToOwned::to_owned("struct IPerpEngine.State"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("settlePnl"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("settlePnl"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("socializeSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("socializeSubaccount",),
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
                    ::std::borrow::ToOwned::to_owned("states"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("states"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cumulativeFundingLongX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("cumulativeFundingShortX18",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("availableSettle"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("openInterest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("updateBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                                name: ::std::borrow::ToOwned::to_owned("vQuoteDelta"),
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
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avgPriceDiffs"),
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
                    ::std::borrow::ToOwned::to_owned("FundingPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FundingPayment"),
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
                                name: ::std::borrow::ToOwned::to_owned("openInterest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PERPENGINE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa4\x06\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02&W`\x005`\xE0\x1C\x80c\x8A\x1DC\xC9\x11a\x01*W\x80c\xC5V\x07\xB5\x11a\0\xBDW\x80c\xEC\xD9\xCB\xA8\x11a\0\x8CW\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x079W\x80c\xF8\xA4.Q\x14a\x07LW\x80c\xFA\xB2\xC4i\x14a\x07_W`\0\x80\xFD[\x80c\xEC\xD9\xCB\xA8\x14a\x06[W\x80c\xED\xF0&S\x14a\x06\xBFW`\0\x80\xFD[\x80c\xC5V\x07\xB5\x14a\x05}W\x80c\xD6\xB0\xE0\xB5\x14a\x05\x90W\x80c\xE34\xBE3\x14a\x05\xA3W\x80c\xECzy\xC9\x14a\x05\xC4W`\0\x80\xFD[\x80c\xB1\xCB\x0FB\x11a\0\xF9W\x80c\xB1\xCB\x0FB\x14a\x04\xC6W\x80c\xB1\xCDK\x8F\x14a\x04\xD7W\x80c\xBFL\x8F_\x14a\x04\xEAW\x80c\xC3b\xD1\x9E\x14a\x05RW`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x04jW\x80c\x8D\xA5\xCB[\x14a\x04}W\x80c\xA4Y\x89\xAB\x14a\x04\xA2W\x80c\xAE\xD8\xE9g\x14a\x04\xB5W`\0\x80\xFD[\x80cO\xA0\xF7&\x11a\x01\xBDW\x80cqP\x18\xA6\x11a\x01\x8CW\x80c\x7F\x17\xBA\xAD\x11a\x01qW\x80c\x7F\x17\xBA\xAD\x14a\x03\xD5W\x80c\x7F\xA2\x9DI\x14a\x04DW\x80c\x87\x1D\t\x12\x14a\x04WW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x03\xADW\x80c|\x1E\x14\x87\x14a\x03\xB5W`\0\x80\xFD[\x80cO\xA0\xF7&\x14a\x03QW\x80cd\xC4,\xC2\x14a\x03dW\x80cg6\xF5\xDA\x14a\x03\x87W\x80cg\x92\xDC\xC1\x14a\x03\x9AW`\0\x80\xFD[\x80c8]\xE9\xC3\x11a\x01\xF9W\x80c8]\xE9\xC3\x14a\x02\xF8W\x80c8\x89'\xB8\x14a\x03\x0BW\x80cF\x04\xD1\x9B\x14a\x03-W\x80cGB\x8E{\x14a\x03<W`\0\x80\xFD[\x80c\x14YEz\x14a\x02+W\x80c\x15<\xA6\xC0\x14a\x02@W\x80c\x17i\"_\x14a\x02\xA1W\x80c0V\xF7\x8F\x14a\x02\xCCW[`\0\x80\xFD[a\x02>a\x0296`\x04a'\xC8V[a\x07tV[\0[a\x02>a\x02N6`\x04a(aV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xB4a\x02\xAF6`\x04a(\x98V[a\x07\x87V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xB4a\x02\xDA6`\x04a(\xC2V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 `\x01\x01T`\x0F\x0B\x90V[a\x02>a\x03\x066`\x04a(\xDDV[a\x07\xABV[a\x03\x1Ea\x03\x196`\x04a(\x98V[a\x07\xD7V[`@Qa\x02\xC3\x93\x92\x91\x90a)%V[`\x01`@Qa\x02\xC3\x91\x90a)\xADV[a\x03Da\x08FV[`@Qa\x02\xC3\x91\x90a)\xD5V[a\x02>a\x03_6`\x04a(\x98V[a\x08\xCAV[a\x03wa\x03r6`\x04a(\x98V[a\t\rV[`@Q\x90\x15\x15\x81R` \x01a\x02\xC3V[a\x02>a\x03\x956`\x04a*dV[a\tnV[a\x02>a\x03\xA86`\x04a*\xC5V[a\x0C\xC1V[a\x02>a\x0E1V[a\x03\xC8a\x03\xC36`\x04a(\x98V[a\x0E=V[`@Qa\x02\xC3\x91\x90a+\x07V[a\x04\x17a\x03\xE36`\x04a(\xC2V[`j` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x0F\x82\x81\x0B\x92`\x01`\x80\x1B\x90\x81\x90\x04\x82\x0B\x92\x80\x83\x0B\x92\x91\x90\x04\x90\x0B\x84V[`@\x80Q`\x0F\x95\x86\x0B\x81R\x93\x85\x0B` \x85\x01R\x91\x84\x0B\x91\x83\x01\x91\x90\x91R\x90\x91\x0B``\x82\x01R`\x80\x01a\x02\xC3V[a\x02>a\x04R6`\x04a+5V[a\x0E\xECV[a\x02\xB4a\x04e6`\x04a+\x83V[a\x0F\x13V[a\x03\xC8a\x04x6`\x04a+\xAFV[a\x0F\x93V[`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC3V[a\x02>a\x04\xB06`\x04a+\xEBV[a\x10\x06V[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x8AV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x8AV[a\x02\xB4a\x04\xE56`\x04a,NV[a\x10ZV[a\x05.a\x04\xF86`\x04a(\x98V[`k` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T`\x0F\x82\x81\x0B\x92`\x01`\x80\x1B\x90\x04\x81\x0B\x91\x90\x0B\x83V[`@\x80Q`\x0F\x94\x85\x0B\x81R\x92\x84\x0B` \x84\x01R\x92\x0B\x91\x81\x01\x91\x90\x91R``\x01a\x02\xC3V[`lTa\x05e\x90`\x01`\x01`\x80\x1B\x03\x16\x81V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC3V[a\x02>a\x05\x8B6`\x04a-'V[a\x12\x10V[a\x02\xB4a\x05\x9E6`\x04a-RV[a\x13OV[a\x05\xB6a\x05\xB16`\x04a(\x98V[a\x13\xF1V[`@Qa\x02\xC3\x92\x91\x90a-tV[a\x06Na\x05\xD26`\x04a(\xC2V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xC3\x91\x90a-\xD8V[a\x06na\x06i6`\x04a(\xC2V[a\x14\xCDV[`@Qa\x02\xC3\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x03\xC8a\x06\xCD6`\x04a(\x98V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85R`k\x82R\x82\x85 \x93\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82R\x80T`\x0F\x81\x81\x0B\x85R`\x01`\x80\x1B\x90\x91\x04\x81\x0B\x94\x84\x01\x94\x90\x94R`\x01\x01T\x90\x92\x0B\x91\x81\x01\x91\x90\x91R\x90V[a\x02>a\x07G6`\x04a.\x13V[a\x15\x01V[a\x02>a\x07Z6`\x04a.0V[a\x15\x89V[`@\x80Q`k\x81R`j` \x82\x01R\x01a\x02\xC3V[a\x07\x80\x85\x85\x84\x84a\x16AV[PPPPPV[`\0\x80a\x07\x94\x84\x84a\x13\xF1V[\x91PPa\x07\xA1\x81\x85a\x18\x02V[\x91PP[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81\x90a\x07\x80\x82\x82a.\x81V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x08!\x85\x85a\x13\xF1V[\x90\x92P\x90Pa\x08=a\x083\x82\x87a\x18\x02V[\x83`@\x01Qa\x18\x80V[\x92P\x92P\x92P\x92V[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xC0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x83W\x90P[PPPPP\x90P\x90V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T`\x0F\x0B\x15\x80\x15a\tgWPc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B\x15\x15[\x93\x92PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01R\x7Fe endpoint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\n\x01\x84`\x0F\x0Ba\x18\x9CV[\x90P`\0[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x07\x80W`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\n/Wa\n/a/\x05V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`j\x82R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x96\x83\x01\x96\x90\x96R`\x01\x90\x92\x01T\x80\x86\x0B\x93\x82\x01\x93\x90\x93R\x91\x04\x90\x92\x0B``\x83\x01\x81\x90R\x90\x93P\x90\x91\x03a\n\xB0WPPa\x0C\xAFV[a\n\xBEb\x01Q\x80`\x07a/1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x0B\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0a\x0B\x19\x83a\x19\x15V[`\x80\x01Q\x90P`\0\x87\x87\x86c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x0B9Wa\x0B9a/\x05V[\x90P` \x02\x01` \x81\x01\x90a\x0BN\x91\x90a/\xB6V[\x90P`\0a\x0BcfG\r\xE4\xDF\x82\0\0\x84a\x1AHV[\x90P\x80`\x0F\x0Ba\x0Bu\x83`\x0F\x0Ba\x1A\xCBV[`\x0F\x0B\x13\x15a\x0B\x9CW`\0\x82`\x0F\x0B\x13a\x0B\x97Wa\x0B\x92\x81a/\xD3V[a\x0B\x99V[\x80[\x91P[`\0a\x0B\xC3i\x12K\xC0\xDD\xD9.V\0\0\0a\x0B\xBA`\x0F\x86\x90\x0B\x8Ba\x1AHV[`\x0F\x0B\x90a\x1B5V[\x90P\x80\x85`\0\x01\x81\x81Qa\x0B\xD7\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x85\x01\x80Q\x82\x91\x90a\x0B\xF1\x90\x83\x90a/\xF9V[`\x0F\x0B\x90RP``\x85\x01Q`@Q\x7FRdv\x19\xF5\x16\x1A\x81\xBAR\xD7jS\xFB\xEA\xE1\x14/L\xD7\xE3WM\x9A\x81\r\xF8\x11\xF7`I\x1A\x91a\x0C]\x91\x89\x91\x8F\x91\x86\x90c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16` \x84\x01R`\x0F\x90\x81\x0B`@\x84\x01R\x0B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPa\x0Cs\x82\x82a\x1B\x9EV[`@Qc\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1PP[\x80a\x0C\xB9\x81a0HV[\x91PPa\n\x06V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x82\x11\x15a\x0E,W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x0C\xF3Wa\x0C\xF3a/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83\x83\x83`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\r6Wa\r6a/\x05V[\x90P` \x02\x81\x01\x90a\rH\x91\x90a0kV[`@Qa\rV\x92\x91\x90a0\xB2V[`@\x80Q\x91\x82\x90\x03\x82 c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`j` \x90\x81R\x92\x90 \x90\x92a\r\xAE\x92\x01\x81T`\x0F\x81\x81\x0B\x83R`\x80\x91\x82\x1D\x81\x0B` \x84\x01R`\x01\x90\x93\x01T\x80\x84\x0B`@\x84\x01R\x81\x1D\x90\x92\x0B``\x82\x01R\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7FDSYNC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x90a\x0E\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[PP\x80a\x0E%\x90a0\xC2V[\x90Pa\x0C\xC4V[PPPV[a\x0E;`\0a\x1B\xFAV[V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x83R`j\x82R\x85\x83 \x86Q`\x80\x81\x01\x88R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x93\x84\x01T\x80\x82\x0B\x84\x8C\x01R\x82\x90\x04\x81\x0B\x83\x89\x01R\x93\x86R`k\x85R\x88\x86 \x8A\x87R\x85R\x88\x86 \x89Q\x97\x88\x01\x8AR\x80T\x80\x86\x0B\x89R\x91\x90\x91\x04\x84\x0B\x94\x87\x01\x94\x90\x94R\x92\x01T\x90\x0B\x94\x83\x01\x94\x90\x94R\x91\x92\x91a\x07\xA1\x90\x83\x90\x83\x90\x80a\x1CYV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x0F\r\x82\x82a0\xDEV[PPPPV[`\0\x80a\x01\0a\x0F!a\x1DwV[a\x0F+\x91\x90a1\xA1V[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x0F\x8BW`\0a\x0FO\x86\x83a\x1D\xC6V[\x90P\x80`\0\x03a\x0F_WPa\x0FyV[a\x0Fk\x81\x83\x88\x88a\x1E\x8FV[a\x0Fu\x90\x85a/\xF9V[\x93PP[\x80a\x0F\x83\x81a0HV[\x91PPa\x0F0V[PP\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x0F\xB9\x84a\x19\x15V[\x90P`\0a\x0F\xC7\x85\x87a\x1E\xEAV[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x0F\xF7\x84`\x01\x88a\x1F\rV[`\x0F\x0B\x90R\x96\x95PPPPPPV[`\0a\x10#\x85\x82\x86\x86a\x10\x1E6\x88\x90\x03\x88\x01\x88a1\xC4V[a\x1F\xA4V[\x90P\x80\x15a\x07\x80W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x07\x80\x90\x86\x90a\x1B\x9EV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0a\x10\xABa\x08FV[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x12\x07W`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x10\xDFWa\x10\xDFa/\x05V[` \x02` \x01\x01Q\x90P`\0\x80a\x10\xF6\x83\x89a\x13\xF1V[\x91P\x91P`\0\x81` \x01Q`\x0F\x0B\x12\x15a\x11\xF3W`\0a\x11#\x88\x83` \x01Qa\x11\x1E\x90a/\xD3V[a\x18\x80V[\x90Pa\x11/\x81\x89a1\xE0V[\x97P\x80\x82` \x01\x81\x81Qa\x11C\x91\x90a/\xF9V[`\x0F\x0B\x90RP`@\x83\x01\x80Q\x82\x91\x90a\x11]\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R` \x84\x01Q`\0\x91\x0B\x12\x15\x90Pa\x11\xDCW`\0a\x11\x96\x84``\x01Q\x84` \x01Q`\x0F\x0Ba\x1B5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x11\x9F\x90a/\xD3V[\x90P\x80\x84`\0\x01\x81\x81Qa\x11\xB3\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x84\x01\x80Q\x82\x91\x90a\x11\xCD\x90\x83\x90a1\xE0V[`\x0F\x0B\x90RPP`\0` \x83\x01R[a\x11\xE6\x84\x84a\x1B\x9EV[a\x11\xF1\x84\x8A\x84a#\xD4V[P[PPP\x80a\x12\0\x90a0\xC2V[\x90Pa\x10\xB0V[P\x91\x93\x92PPPV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x12<WP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x12vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02o\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x19\x92\x8A\x16h\x01\0\0\0\0\0\0\0\0\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`\0\x80[\x82\x15a\tgWc\xFF\xFF\xFF\xFF\x83\x16a\x13k`\x02\x82a20V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x13\xE4W`\0\x80`\0a\x13\x87\x84\x89a\x07\xD7V[\x92P\x92P\x92P\x82\x82`@\x01\x81\x81Qa\x13\x9F\x91\x90a1\xE0V[`\x0F\x0B\x90RP` \x81\x01\x80Q\x84\x91\x90a\x13\xB9\x90\x83\x90a1\xE0V[`\x0F\x0B\x90RPa\x13\xC9\x83\x86a/\xF9V[\x94Pa\x13\xD5\x84\x83a\x1B\x9EV[a\x13\xE0\x84\x89\x83a#\xD4V[PPP[` \x84\x90\x1C\x93PPa\x13SV[`@\x80Q`\x80\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R\x85Q\x80\x82\x01\x87R\x83\x81R\x80\x83\x01\x84\x90R\x80\x87\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x89\x16\x80\x85R`j\x84R\x87\x85 \x88Q\x96\x87\x01\x89R\x80T`\x0F\x81\x81\x0B\x89R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x89\x88\x01R`\x01\x92\x83\x01T\x80\x82\x0B\x8A\x8D\x01R\x82\x90\x04\x81\x0B\x89\x87\x01R\x92\x87R`k\x86R\x89\x87 \x8B\x88R\x86R\x89\x87 \x8AQ\x95\x86\x01\x8BR\x80T\x80\x85\x0B\x87R\x91\x90\x91\x04\x83\x0B\x95\x85\x01\x95\x90\x95R\x93\x90\x93\x01T\x90\x92\x0B\x95\x81\x01\x95\x90\x95R\x92\x93\x90a\x14\xC0\x90\x83\x90\x83\x90\x80a\x1CYV[\x90\x92P\x90P[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x07\xA5\x82a\x19\x15V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xEAV[a\x15\x86\x81a\x1B\xFAV[PV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x93\x84\x01T\x80\x82\x0B\x84\x87\x01R\x82\x90\x04\x81\x0B``\x80\x85\x01\x91\x90\x91R\x97\x87R`k\x86R\x84\x87 \x8B\x88R\x86R\x95\x84\x90 \x84Q\x97\x88\x01\x85R\x80T\x80\x88\x0B\x89R\x91\x90\x91\x04\x86\x0B\x94\x87\x01\x94\x90\x94R\x92\x01T\x90\x92\x0B\x91\x83\x01\x91\x90\x91R\x90a\x16$\x82\x82\x86\x86a\x1CYV[a\x16/\x86\x86\x83a#\xD4V[a\x169\x86\x83a\x1B\x9EV[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x16aWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x16{WP0;\x15\x80\x15a\x16{WP`\0T`\xFF\x16`\x01\x14[a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xEAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\x10W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\x18a$]V[`e\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x17I\x82a\x15\x01V[`f\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x07\x80W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0\x80a\x18\x0E\x83a\x19\x15V[`\x80\x01Q\x90P\x83` \x01Qa\x183\x85`\0\x01Q\x83`\x0F\x0Ba\x1AH\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x18=\x91\x90a/\xF9V[`@Qc\xFF\xFF\xFF\xFF\x85\x16\x81R\x90\x92P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1P\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a\x18\x95W\x81a\tgV[P\x90\x91\x90PV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\x18\xD5WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x92\x91PPV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x80\x86\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x88\x16\x84R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83R\x92\x86\x90 \x86Q\x94\x85\x01\x87RT`\x03\x81\x81\x0B\x80\x87Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x87\x01\x94\x90\x94Rh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x0B\x97\x86\x01\x97\x90\x97R`\x01``\x1B\x81\x04\x90\x96\x0B\x90\x84\x01R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x82\x01R\x90\x91a\x19\xD6\x90c;\x9A\xCA\0a2SV[`\x0F\x0B\x82R` \x81\x01Qa\x19\xF1\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x0B` \x83\x01R`@\x81\x01Qa\x1A\x0F\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x0B`@\x83\x01R``\x81\x01Qa\x1A-\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x91\x90PV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\x1A\x8AWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x1A\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03a\x1B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0\x82`\x0F\x0B\x12a\x1B.W\x81a\x07\xA5V[P`\0\x03\x90V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a\x1ByW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a\x1A_Wa\x1A_a1\x8BV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua\x1B\xF6\x82a$\xD0V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82Qa\x1Cg\x90`\x0F\x0Ba\x1A\xCBV[\x84``\x01\x81\x81Qa\x1Cx\x91\x90a1\xE0V[`\x0F\x90\x81\x0B\x90\x91R\x84Q`\0\x92P\x90\x0B\x81\x12a\x1C\x98W\x84` \x01Qa\x1C\x9BV[\x84Q[\x90P`\0\x84`@\x01Q\x82a\x1C\xAF\x91\x90a1\xE0V[\x90P`\0a\x1C\xCD\x86`\0\x01Q\x83`\x0F\x0Ba\x1AH\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1C\xD7\x90\x85a1\xE0V[\x90P\x84\x86`\0\x01\x81\x81Qa\x1C\xEB\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x86\x01\x80Q\x82\x91\x90a\x1D\x05\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R\x87Q`\0\x91\x0B\x13\x15\x90Pa\x1DEW\x85Q``\x88\x01\x80Qa\x1D.\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R\x88Q\x90\x0B`@\x88\x01RPa\x1DnV[\x85Q``\x88\x01\x80Qa\x1DX\x90\x83\x90a1\xE0V[`\x0F\x90\x81\x0B\x90\x91R` \x89\x01Q\x90\x0B`@\x88\x01RP[PPPPPPPV[`g\x80T`\0\x91\x90a\x1D\x8B\x90`\x01\x90a2\xF1V[\x81T\x81\x10a\x1D\x9BWa\x1D\x9Ba/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a\x1A\xC3W`\0`g\x82\x81T\x81\x10a\x1D\xEAWa\x1D\xEAa/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a\x1E \x91\x90a3\x08V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a\x1E\\WPa\x1EA\x85`\x01a3+V[a\x1EM\x90a\x01\0a3\x08V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a\x1E|Wa\x1Ema\x01\0\x82a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a\x1E\x87\x81a3SV[\x91PPa\x1D\xCBV[`\0\x80a\x1E\x9E\x85a\x01\0a3\x08V[\x90P[\x85\x15a\x1E\xE1W`\x01\x86\x16\x15a\x1E\xC8Wa\x1E\xBB\x81\x85\x85a%\x0CV[a\x1E\xC5\x90\x83a/\xF9V[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a\x1E\xD9\x81a0HV[\x91PPa\x1E\xA1V[P\x94\x93PPPPV[`\0\x80`\0a\x1E\xF9\x85\x85a\x0E=V[\x80Q` \x90\x91\x01Q\x90\x96\x90\x95P\x93PPPPV[`\0`\x02\x82`\x02\x81\x11\x15a\x1F#Wa\x1F#a)\x97V[\x03a\x1F7WPg\r\xE0\xB6\xB3\xA7d\0\0a\tgV[`\0\x80\x84`\x0F\x0B\x12a\x1FpW`\0\x83`\x02\x81\x11\x15a\x1FWWa\x1FWa)\x97V[\x14a\x1FfW\x84`@\x01Qa\x1FiV[\x84Q[\x90Pa\x1F\x9CV[`\0\x83`\x02\x81\x11\x15a\x1F\x84Wa\x1F\x84a)\x97V[\x14a\x1F\x93W\x84``\x01Qa\x1F\x99V[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`@\x01Q`\x03\x0B\x82`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1F\xCFWPc;\x9A\xCA\0\x82`@\x01Q`\x03\x0B\x13\x15[\x80\x15a\x1F\xE9WP\x81``\x01Q`\x03\x0B\x82` \x01Q`\x03\x0B\x12\x15[\x80\x15a \0WPc;\x9A\xCA\0\x82``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a :W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`gT\x15\x80a \x9AWP`g\x80Ta U\x90`\x01\x90a2\xF1V[\x81T\x81\x10a eWa ea/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x11[\x15a!iW`g\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x08\x81\x04\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04`\x07\x90\x95\x16\x85\x02a\x01\0\n\x90\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U`fT`@QcC\xB1j\x11`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x87b\xD4\"\x91a!2\x91\x8A\x91\x01c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!LW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!`W=`\0\x80>=`\0\xFD[PPPP`\x01\x90P[\x80\x15a!\xF1W`eT`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01Q`@Qc\x1C\xB2\x81[`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R`\x0F\x91\x90\x91\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9e\x02\xB6\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xE8W=`\0\x80>=`\0\xFD[PPPPa\"6V[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B`\x80\x83\x01R[c\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` \x90\x81R`@\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x93\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0\x95\x87\x16\x95\x90\x95\x02\x94\x90\x94\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16h\x01\0\0\0\0\0\0\0\0\x94\x86\x16\x94\x90\x94\x02o\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x19\x16\x93\x90\x93\x17`\x01``\x1B\x92\x90\x94\x16\x91\x90\x91\x02\x92\x90\x92\x17`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x91\x90\x92\x16\x02\x17\x90Ua#\x16a&\x08V[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x89\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`\x0F\x86\x81\x0B`D\x83\x01R\x85\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\x8DW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x89\x16\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x84Q\x92\x85\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x81\x85\x16\x17\x82U\x91\x85\x01Q`\x01\x90\x91\x01\x80T\x91\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x90\x91\x16\x17\x90U`\x0F\x0B\x15\x15\x80a$FWP` \x82\x01Q`\x0F\x0B\x15\x15[\x90Pa$S\x83\x85\x83a&\x82V[a\x0F\r\x84\x84a\x08\xCAV[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x0E;a'?V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a%\x18\x85a\x19\x15V[\x90P`\0\x80a%'\x87\x87a\x1E\xEAV[\x91P\x91P`\0a%8\x84\x84\x88a\x1F\rV[\x90Pa%D\x82\x86a/\xF9V[\x94P\x82`\x0F\x0B`\0\x14a%\xFDWa%dg\r\xE0\xB6\xB3\xA7d\0\0`\x02a2SV[`\x0F\x0B\x81`\x0F\x0B\x03a%\x96Wa%\x82`\x80`\x01`\x01`\x7F\x1B\x03a3lV[a%\x8B\x90a/\xD3V[\x94PPPPPa\tgV[`\x80\x84\x01Qa%\xB6\x90a%\xAD`\x0F\x86\x90\x0B\x84a\x1AHV[`\x0F\x0B\x90a\x1AHV[a%\xC0\x90\x86a/\xF9V[`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x90\x95P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPP\x93\x92PPPV[`\0a&\x1C`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&}\x91\x90a3\xB3V[\x90P\x90V[\x80\x15a&\xE3Wa&\x94a\x01\0\x83a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a&\xC1\x91\x90a1\xA1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a&\xEFa\x01\0\x83a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a'\x1D\x91\x90a1\xA1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x0E;3a\x1B\xFAV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x86W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a'\xE0W`\0\x80\xFD[\x855a'\xEB\x81a'\xB3V[\x94P` \x86\x015a'\xFB\x81a'\xB3V[\x93P`@\x86\x015a(\x0B\x81a'\xB3V[\x92P``\x86\x015a(\x1B\x81a'\xB3V[\x91P`\x80\x86\x015a(+\x81a'\xB3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(MW`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a\x15\x86W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a(tW`\0\x80\xFD[a(}\x83a(9V[\x91P` \x83\x015a(\x8D\x81a(RV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xABW`\0\x80\xFD[a(\xB4\x83a(9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a(\xD4W`\0\x80\xFD[a\tg\x82a(9V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a(\xF3W`\0\x80\xFD[a(\xFC\x85a(9V[\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a)\x17W`\0\x80\xFD[P`@\x84\x01\x90P\x92P\x92P\x92V[`\x0F\x84\x90\x0B\x81Ra\x01\0\x81\x01a)k` \x83\x01\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\xA0\x84\x01R` \x84\x01Q\x81\x0B`\xC0\x84\x01R`@\x84\x01Q\x90\x0B`\xE0\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a)\xCFWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a*\x13W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a)\xF1V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a*1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*IW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x14\xC6W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a*yW`\0\x80\xFD[\x835`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a*\x90W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xACW`\0\x80\xFD[a*\xB8\x86\x82\x87\x01a*\x1FV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80` \x83\x85\x03\x12\x15a*\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xEFW`\0\x80\xFD[a*\xFB\x85\x82\x86\x01a*\x1FV[\x90\x96\x90\x95P\x93PPPPV[``\x81\x01a\x07\xA5\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a+IW`\0\x80\xFD[a+R\x84a(9V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a+fW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a(MW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\x96W`\0\x80\xFD[\x825\x91Pa+\xA6` \x84\x01a+tV[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xC4W`\0\x80\xFD[\x835\x92Pa+\xD4` \x85\x01a(9V[\x91Pa+\xE2`@\x85\x01a+tV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a,\x03W`\0\x80\xFD[a,\x0C\x86a(9V[\x94P` \x86\x015a,\x1C\x81a(RV[\x93P`@\x86\x015a,,\x81a(RV[\x92P`\xA0`_\x19\x82\x01\x12\x15a,@W`\0\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a,aW`\0\x80\xFD[\x825\x91P` \x83\x015a(\x8D\x81a(RV[\x805`\x03\x81\x90\x0B\x81\x14a(MW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a,\x97W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a,\xC8WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a,\xD7\x83a,sV[\x81Ra,\xE5` \x84\x01a,sV[` \x82\x01Ra,\xF6`@\x84\x01a,sV[`@\x82\x01Ra-\x07``\x84\x01a,sV[``\x82\x01R`\x80\x83\x015a-\x1A\x81a(RV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15a-:W`\0\x80\xFD[a-C\x83a(9V[\x91Pa+\xA6\x84` \x85\x01a,\x85V[`\0\x80`@\x83\x85\x03\x12\x15a-eW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\xE0\x81\x01a-\xAF\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\x80\x84\x01R` \x84\x01Q\x81\x0B`\xA0\x84\x01R`@\x84\x01Q\x90\x0B`\xC0\x83\x01Ra\tgV[`\x80\x81\x01a\x07\xA5\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15a.%W`\0\x80\xFD[\x815a\tg\x81a'\xB3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.FW`\0\x80\xFD[a.O\x85a(9V[\x93P` \x85\x015\x92P`@\x85\x015a.f\x81a(RV[\x91P``\x85\x015a.v\x81a(RV[\x93\x96\x92\x95P\x90\x93PPV[\x815a.\x8C\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015a.\xB4\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`@\x82\x015a.\xE0\x81a(RV[`\x01\x82\x01\x80T`\x01`\x01`\x80\x1B\x03\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x90\x91\x16\x17\x90UPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a/XWa/Xa/\x1BV[\x02\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a/\x8EW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a/rV[\x81\x81\x11\x15a/\xA0W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a/\xC8W`\0\x80\xFD[\x815a\tg\x81a(RV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a/\xF0Wa/\xF0a/\x1BV[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a0#Wa0#a/\x1BV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a0?Wa0?a/\x1BV[P\x01\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a0aWa0aa/\x1BV[`\x01\x01\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a0\x82W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a0\x9DW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14\xC6W`\0\x80\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a0aWa0aa/\x1BV[\x815a0\xE9\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015a1\x11\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015a1A\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015a1i\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x0F\rV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a1\xB8Wa1\xB8a1\x8BV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a1\xD6W`\0\x80\xFD[a\tg\x83\x83a,\x85V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a2\x0BWa2\x0Ba/\x1BV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a2&Wa2&a/\x1BV[P\x90\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a2GWa2Ga1\x8BV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a2\x83Wa2\x83a/\x1BV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a2\xAFWa2\xAFa/\x1BV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a2\xCBWa2\xCBa/\x1BV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a2\xE1Wa2\xE1a/\x1BV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82\x82\x10\x15a3\x03Wa3\x03a/\x1BV[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a/XWa/Xa/\x1BV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a3JWa3Ja/\x1BV[\x01\x94\x93PPPPV[`\0`\x01\x82\x01a3eWa3ea/\x1BV[P`\x01\x01\x90V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a3\x83Wa3\x83a1\x8BV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3\xAAWa3\xAAa/\x1BV[\x90\x05\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\xC5W`\0\x80\xFD[\x81Qa\tg\x81a'\xB3V\xFE\xA2dipfsX\"\x12 \xD3c\xE3\xD1=\xE1\xD7Z\xA6\xD7\xCB\xB2@A%\"\xD3\xEDC\xDB\xB1\xFB\x0E\t'FI\xCE\xD8*dqdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static PERPENGINE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02&W`\x005`\xE0\x1C\x80c\x8A\x1DC\xC9\x11a\x01*W\x80c\xC5V\x07\xB5\x11a\0\xBDW\x80c\xEC\xD9\xCB\xA8\x11a\0\x8CW\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x079W\x80c\xF8\xA4.Q\x14a\x07LW\x80c\xFA\xB2\xC4i\x14a\x07_W`\0\x80\xFD[\x80c\xEC\xD9\xCB\xA8\x14a\x06[W\x80c\xED\xF0&S\x14a\x06\xBFW`\0\x80\xFD[\x80c\xC5V\x07\xB5\x14a\x05}W\x80c\xD6\xB0\xE0\xB5\x14a\x05\x90W\x80c\xE34\xBE3\x14a\x05\xA3W\x80c\xECzy\xC9\x14a\x05\xC4W`\0\x80\xFD[\x80c\xB1\xCB\x0FB\x11a\0\xF9W\x80c\xB1\xCB\x0FB\x14a\x04\xC6W\x80c\xB1\xCDK\x8F\x14a\x04\xD7W\x80c\xBFL\x8F_\x14a\x04\xEAW\x80c\xC3b\xD1\x9E\x14a\x05RW`\0\x80\xFD[\x80c\x8A\x1DC\xC9\x14a\x04jW\x80c\x8D\xA5\xCB[\x14a\x04}W\x80c\xA4Y\x89\xAB\x14a\x04\xA2W\x80c\xAE\xD8\xE9g\x14a\x04\xB5W`\0\x80\xFD[\x80cO\xA0\xF7&\x11a\x01\xBDW\x80cqP\x18\xA6\x11a\x01\x8CW\x80c\x7F\x17\xBA\xAD\x11a\x01qW\x80c\x7F\x17\xBA\xAD\x14a\x03\xD5W\x80c\x7F\xA2\x9DI\x14a\x04DW\x80c\x87\x1D\t\x12\x14a\x04WW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x03\xADW\x80c|\x1E\x14\x87\x14a\x03\xB5W`\0\x80\xFD[\x80cO\xA0\xF7&\x14a\x03QW\x80cd\xC4,\xC2\x14a\x03dW\x80cg6\xF5\xDA\x14a\x03\x87W\x80cg\x92\xDC\xC1\x14a\x03\x9AW`\0\x80\xFD[\x80c8]\xE9\xC3\x11a\x01\xF9W\x80c8]\xE9\xC3\x14a\x02\xF8W\x80c8\x89'\xB8\x14a\x03\x0BW\x80cF\x04\xD1\x9B\x14a\x03-W\x80cGB\x8E{\x14a\x03<W`\0\x80\xFD[\x80c\x14YEz\x14a\x02+W\x80c\x15<\xA6\xC0\x14a\x02@W\x80c\x17i\"_\x14a\x02\xA1W\x80c0V\xF7\x8F\x14a\x02\xCCW[`\0\x80\xFD[a\x02>a\x0296`\x04a'\xC8V[a\x07tV[\0[a\x02>a\x02N6`\x04a(aV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xB4a\x02\xAF6`\x04a(\x98V[a\x07\x87V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xB4a\x02\xDA6`\x04a(\xC2V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` R`@\x90 `\x01\x01T`\x0F\x0B\x90V[a\x02>a\x03\x066`\x04a(\xDDV[a\x07\xABV[a\x03\x1Ea\x03\x196`\x04a(\x98V[a\x07\xD7V[`@Qa\x02\xC3\x93\x92\x91\x90a)%V[`\x01`@Qa\x02\xC3\x91\x90a)\xADV[a\x03Da\x08FV[`@Qa\x02\xC3\x91\x90a)\xD5V[a\x02>a\x03_6`\x04a(\x98V[a\x08\xCAV[a\x03wa\x03r6`\x04a(\x98V[a\t\rV[`@Q\x90\x15\x15\x81R` \x01a\x02\xC3V[a\x02>a\x03\x956`\x04a*dV[a\tnV[a\x02>a\x03\xA86`\x04a*\xC5V[a\x0C\xC1V[a\x02>a\x0E1V[a\x03\xC8a\x03\xC36`\x04a(\x98V[a\x0E=V[`@Qa\x02\xC3\x91\x90a+\x07V[a\x04\x17a\x03\xE36`\x04a(\xC2V[`j` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x0F\x82\x81\x0B\x92`\x01`\x80\x1B\x90\x81\x90\x04\x82\x0B\x92\x80\x83\x0B\x92\x91\x90\x04\x90\x0B\x84V[`@\x80Q`\x0F\x95\x86\x0B\x81R\x93\x85\x0B` \x85\x01R\x91\x84\x0B\x91\x83\x01\x91\x90\x91R\x90\x91\x0B``\x82\x01R`\x80\x01a\x02\xC3V[a\x02>a\x04R6`\x04a+5V[a\x0E\xECV[a\x02\xB4a\x04e6`\x04a+\x83V[a\x0F\x13V[a\x03\xC8a\x04x6`\x04a+\xAFV[a\x0F\x93V[`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC3V[a\x02>a\x04\xB06`\x04a+\xEBV[a\x10\x06V[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x8AV[`fT`\x01`\x01`\xA0\x1B\x03\x16a\x04\x8AV[a\x02\xB4a\x04\xE56`\x04a,NV[a\x10ZV[a\x05.a\x04\xF86`\x04a(\x98V[`k` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 \x80T`\x01\x90\x91\x01T`\x0F\x82\x81\x0B\x92`\x01`\x80\x1B\x90\x04\x81\x0B\x91\x90\x0B\x83V[`@\x80Q`\x0F\x94\x85\x0B\x81R\x92\x84\x0B` \x84\x01R\x92\x0B\x91\x81\x01\x91\x90\x91R``\x01a\x02\xC3V[`lTa\x05e\x90`\x01`\x01`\x80\x1B\x03\x16\x81V[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xC3V[a\x02>a\x05\x8B6`\x04a-'V[a\x12\x10V[a\x02\xB4a\x05\x9E6`\x04a-RV[a\x13OV[a\x05\xB6a\x05\xB16`\x04a(\x98V[a\x13\xF1V[`@Qa\x02\xC3\x92\x91\x90a-tV[a\x06Na\x05\xD26`\x04a(\xC2V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RPc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x94\x83\x01\x94\x90\x94R`\x01\x90\x92\x01T\x80\x84\x0B\x94\x82\x01\x94\x90\x94R\x92\x04\x90\x0B``\x82\x01R\x90V[`@Qa\x02\xC3\x91\x90a-\xD8V[a\x06na\x06i6`\x04a(\xC2V[a\x14\xCDV[`@Qa\x02\xC3\x91\x90`\0`\xA0\x82\x01\x90P\x82Q`\x0F\x0B\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R\x92\x91PPV[a\x03\xC8a\x06\xCD6`\x04a(\x98V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85R`k\x82R\x82\x85 \x93\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82R\x80T`\x0F\x81\x81\x0B\x85R`\x01`\x80\x1B\x90\x91\x04\x81\x0B\x94\x84\x01\x94\x90\x94R`\x01\x01T\x90\x92\x0B\x91\x81\x01\x91\x90\x91R\x90V[a\x02>a\x07G6`\x04a.\x13V[a\x15\x01V[a\x02>a\x07Z6`\x04a.0V[a\x15\x89V[`@\x80Q`k\x81R`j` \x82\x01R\x01a\x02\xC3V[a\x07\x80\x85\x85\x84\x84a\x16AV[PPPPPV[`\0\x80a\x07\x94\x84\x84a\x13\xF1V[\x91PPa\x07\xA1\x81\x85a\x18\x02V[\x91PP[\x92\x91PPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 \x81\x90a\x07\x80\x82\x82a.\x81V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x08!\x85\x85a\x13\xF1V[\x90\x92P\x90Pa\x08=a\x083\x82\x87a\x18\x02V[\x83`@\x01Qa\x18\x80V[\x92P\x92P\x92P\x92V[```g\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\xC0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\x83W\x90P[PPPPP\x90P\x90V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81R` \x81\x01\x83\x90R\x7Fo{\x1A\xBEv\xAA\x89t[\x8B\xF2k\x9C\xD9\xA8\xC5\xB1\x95\x1A\xB2\xB5yi\xBDz\t\x1C\xDE\"%\xC9@\x91\x01`@Q\x80\x91\x03\x90\xA1PPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x81 T`\x0F\x0B\x15\x80\x15a\tgWPc\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B\x15\x15[\x93\x92PPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01R\x7Fe endpoint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\n\x01\x84`\x0F\x0Ba\x18\x9CV[\x90P`\0[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x07\x80W`\0`g\x82c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\n/Wa\n/a/\x05V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x83R`j\x82R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x96\x83\x01\x96\x90\x96R`\x01\x90\x92\x01T\x80\x86\x0B\x93\x82\x01\x93\x90\x93R\x91\x04\x90\x92\x0B``\x83\x01\x81\x90R\x90\x93P\x90\x91\x03a\n\xB0WPPa\x0C\xAFV[a\n\xBEb\x01Q\x80`\x07a/1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87`\x01`\x01`\x80\x1B\x03\x16\x10`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bITI`\xE8\x1B\x81RP\x90a\x0B\rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0a\x0B\x19\x83a\x19\x15V[`\x80\x01Q\x90P`\0\x87\x87\x86c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x0B9Wa\x0B9a/\x05V[\x90P` \x02\x01` \x81\x01\x90a\x0BN\x91\x90a/\xB6V[\x90P`\0a\x0BcfG\r\xE4\xDF\x82\0\0\x84a\x1AHV[\x90P\x80`\x0F\x0Ba\x0Bu\x83`\x0F\x0Ba\x1A\xCBV[`\x0F\x0B\x13\x15a\x0B\x9CW`\0\x82`\x0F\x0B\x13a\x0B\x97Wa\x0B\x92\x81a/\xD3V[a\x0B\x99V[\x80[\x91P[`\0a\x0B\xC3i\x12K\xC0\xDD\xD9.V\0\0\0a\x0B\xBA`\x0F\x86\x90\x0B\x8Ba\x1AHV[`\x0F\x0B\x90a\x1B5V[\x90P\x80\x85`\0\x01\x81\x81Qa\x0B\xD7\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x85\x01\x80Q\x82\x91\x90a\x0B\xF1\x90\x83\x90a/\xF9V[`\x0F\x0B\x90RP``\x85\x01Q`@Q\x7FRdv\x19\xF5\x16\x1A\x81\xBAR\xD7jS\xFB\xEA\xE1\x14/L\xD7\xE3WM\x9A\x81\r\xF8\x11\xF7`I\x1A\x91a\x0C]\x91\x89\x91\x8F\x91\x86\x90c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16` \x84\x01R`\x0F\x90\x81\x0B`@\x84\x01R\x0B``\x82\x01R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPa\x0Cs\x82\x82a\x1B\x9EV[`@Qc\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1PP[\x80a\x0C\xB9\x81a0HV[\x91PPa\n\x06V[`\0[`\x01`\x01`\x80\x1B\x03\x81\x16\x82\x11\x15a\x0E,W`\0`g\x82`\x01`\x01`\x80\x1B\x03\x16\x81T\x81\x10a\x0C\xF3Wa\x0C\xF3a/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x83\x83\x83`\x01`\x01`\x80\x1B\x03\x16\x81\x81\x10a\r6Wa\r6a/\x05V[\x90P` \x02\x81\x01\x90a\rH\x91\x90a0kV[`@Qa\rV\x92\x91\x90a0\xB2V[`@\x80Q\x91\x82\x90\x03\x82 c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`j` \x90\x81R\x92\x90 \x90\x92a\r\xAE\x92\x01\x81T`\x0F\x81\x81\x0B\x83R`\x80\x91\x82\x1D\x81\x0B` \x84\x01R`\x01\x90\x93\x01T\x80\x84\x0B`@\x84\x01R\x81\x1D\x90\x92\x0B``\x82\x01R\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7FDSYNC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x90a\x0E\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[PP\x80a\x0E%\x90a0\xC2V[\x90Pa\x0C\xC4V[PPPV[a\x0E;`\0a\x1B\xFAV[V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x83R`j\x82R\x85\x83 \x86Q`\x80\x81\x01\x88R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x93\x84\x01T\x80\x82\x0B\x84\x8C\x01R\x82\x90\x04\x81\x0B\x83\x89\x01R\x93\x86R`k\x85R\x88\x86 \x8A\x87R\x85R\x88\x86 \x89Q\x97\x88\x01\x8AR\x80T\x80\x86\x0B\x89R\x91\x90\x91\x04\x84\x0B\x94\x87\x01\x94\x90\x94R\x92\x01T\x90\x0B\x94\x83\x01\x94\x90\x94R\x91\x92\x91a\x07\xA1\x90\x83\x90\x83\x90\x80a\x1CYV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` R`@\x90 \x81\x90a\x0F\r\x82\x82a0\xDEV[PPPPV[`\0\x80a\x01\0a\x0F!a\x1DwV[a\x0F+\x91\x90a1\xA1V[\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x0F\x8BW`\0a\x0FO\x86\x83a\x1D\xC6V[\x90P\x80`\0\x03a\x0F_WPa\x0FyV[a\x0Fk\x81\x83\x88\x88a\x1E\x8FV[a\x0Fu\x90\x85a/\xF9V[\x93PP[\x80a\x0F\x83\x81a0HV[\x91PPa\x0F0V[PP\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x0F\xB9\x84a\x19\x15V[\x90P`\0a\x0F\xC7\x85\x87a\x1E\xEAV[P\x90P`@Q\x80``\x01`@R\x80\x82`\x0F\x0B\x81R` \x01\x83`\x80\x01Q`\x0F\x0B\x81R` \x01a\x0F\xF7\x84`\x01\x88a\x1F\rV[`\x0F\x0B\x90R\x96\x95PPPPPPV[`\0a\x10#\x85\x82\x86\x86a\x10\x1E6\x88\x90\x03\x88\x01\x88a1\xC4V[a\x1F\xA4V[\x90P\x80\x15a\x07\x80W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x07\x80\x90\x86\x90a\x1B\x9EV[`fT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0a\x10\xABa\x08FV[\x90P`\0[\x81Q\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x12\x07W`\0\x82\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x10\xDFWa\x10\xDFa/\x05V[` \x02` \x01\x01Q\x90P`\0\x80a\x10\xF6\x83\x89a\x13\xF1V[\x91P\x91P`\0\x81` \x01Q`\x0F\x0B\x12\x15a\x11\xF3W`\0a\x11#\x88\x83` \x01Qa\x11\x1E\x90a/\xD3V[a\x18\x80V[\x90Pa\x11/\x81\x89a1\xE0V[\x97P\x80\x82` \x01\x81\x81Qa\x11C\x91\x90a/\xF9V[`\x0F\x0B\x90RP`@\x83\x01\x80Q\x82\x91\x90a\x11]\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R` \x84\x01Q`\0\x91\x0B\x12\x15\x90Pa\x11\xDCW`\0a\x11\x96\x84``\x01Q\x84` \x01Q`\x0F\x0Ba\x1B5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x11\x9F\x90a/\xD3V[\x90P\x80\x84`\0\x01\x81\x81Qa\x11\xB3\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x84\x01\x80Q\x82\x91\x90a\x11\xCD\x90\x83\x90a1\xE0V[`\x0F\x0B\x90RPP`\0` \x83\x01R[a\x11\xE6\x84\x84a\x1B\x9EV[a\x11\xF1\x84\x8A\x84a#\xD4V[P[PPP\x80a\x12\0\x90a0\xC2V[\x90Pa\x10\xB0V[P\x91\x93\x92PPPV[\x80`@\x01Q`\x03\x0B\x81`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x12<WP\x80``\x01Q`\x03\x0B\x81` \x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a\x12vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x80\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0yc\xFF\xFF\xFF\xFF\x93\x84\x16`\0\x90\x81R` \x91\x82R`@\x90\x81\x90 \x83Q\x81T\x93\x85\x01Q\x92\x85\x01Q``\x86\x01Q`\x80\x90\x96\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x96\x89\x16`\x01``\x1B\x02o\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x19\x92\x8A\x16h\x01\0\0\0\0\0\0\0\0\x02\x92\x90\x92\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x95\x8A\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x97\x16\x93\x90\x99\x16\x92\x90\x92\x17\x94\x90\x94\x17\x92\x90\x92\x16\x95\x90\x95\x17\x91\x90\x91\x17\x16\x17\x90\x91UPV[`\0\x80[\x82\x15a\tgWc\xFF\xFF\xFF\xFF\x83\x16a\x13k`\x02\x82a20V[c\xFF\xFF\xFF\xFF\x16`\0\x03a\x13\xE4W`\0\x80`\0a\x13\x87\x84\x89a\x07\xD7V[\x92P\x92P\x92P\x82\x82`@\x01\x81\x81Qa\x13\x9F\x91\x90a1\xE0V[`\x0F\x0B\x90RP` \x81\x01\x80Q\x84\x91\x90a\x13\xB9\x90\x83\x90a1\xE0V[`\x0F\x0B\x90RPa\x13\xC9\x83\x86a/\xF9V[\x94Pa\x13\xD5\x84\x83a\x1B\x9EV[a\x13\xE0\x84\x89\x83a#\xD4V[PPP[` \x84\x90\x1C\x93PPa\x13SV[`@\x80Q`\x80\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R\x85Q\x80\x82\x01\x87R\x83\x81R\x80\x83\x01\x84\x90R\x80\x87\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x89\x16\x80\x85R`j\x84R\x87\x85 \x88Q\x96\x87\x01\x89R\x80T`\x0F\x81\x81\x0B\x89R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x89\x88\x01R`\x01\x92\x83\x01T\x80\x82\x0B\x8A\x8D\x01R\x82\x90\x04\x81\x0B\x89\x87\x01R\x92\x87R`k\x86R\x89\x87 \x8B\x88R\x86R\x89\x87 \x8AQ\x95\x86\x01\x8BR\x80T\x80\x85\x0B\x87R\x91\x90\x91\x04\x83\x0B\x95\x85\x01\x95\x90\x95R\x93\x90\x93\x01T\x90\x92\x0B\x95\x81\x01\x95\x90\x95R\x92\x93\x90a\x14\xC0\x90\x83\x90\x83\x90\x80a\x1CYV[\x90\x92P\x90P[\x92P\x92\x90PV[`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91Ra\x07\xA5\x82a\x19\x15V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xEAV[a\x15\x86\x81a\x1B\xFAV[PV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x81\x81R`j` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x0F\x81\x81\x0B\x83R`\x01`\x80\x1B\x91\x82\x90\x04\x81\x0B\x83\x87\x01R`\x01\x93\x84\x01T\x80\x82\x0B\x84\x87\x01R\x82\x90\x04\x81\x0B``\x80\x85\x01\x91\x90\x91R\x97\x87R`k\x86R\x84\x87 \x8B\x88R\x86R\x95\x84\x90 \x84Q\x97\x88\x01\x85R\x80T\x80\x88\x0B\x89R\x91\x90\x91\x04\x86\x0B\x94\x87\x01\x94\x90\x94R\x92\x01T\x90\x92\x0B\x91\x83\x01\x91\x90\x91R\x90a\x16$\x82\x82\x86\x86a\x1CYV[a\x16/\x86\x86\x83a#\xD4V[a\x169\x86\x83a\x1B\x9EV[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x16aWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x16{WP0;\x15\x80\x15a\x16{WP`\0T`\xFF\x16`\x01\x14[a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\t\xEAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\x10W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\x18a$]V[`e\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x17I\x82a\x15\x01V[`f\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x92U\x84\x82\x16`\0\x90\x81R`h` R`@\x80\x82 \x80T`\xFF\x19\x90\x81\x16`\x01\x90\x81\x17\x90\x92U\x93\x83R\x81\x83 \x80T\x85\x16\x82\x17\x90U\x93\x88\x16\x82R\x90 \x80T\x90\x91\x16\x90\x91\x17\x90U\x80\x15a\x07\x80W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`\0\x80a\x18\x0E\x83a\x19\x15V[`\x80\x01Q\x90P\x83` \x01Qa\x183\x85`\0\x01Q\x83`\x0F\x0Ba\x1AH\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x18=\x91\x90a/\xF9V[`@Qc\xFF\xFF\xFF\xFF\x85\x16\x81R\x90\x92P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1P\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a\x18\x95W\x81a\tgV[P\x90\x91\x90PV[`\0`\x0F\x82\x90\x0Bg\r\xE0\xB6\xB3\xA7d\0\0\x02o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\x18\xD5WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x92\x91PPV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x80\x86\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x88\x16\x84R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y\x83R\x92\x86\x90 \x86Q\x94\x85\x01\x87RT`\x03\x81\x81\x0B\x80\x87Rd\x01\0\0\0\0\x83\x04\x82\x0B\x94\x87\x01\x94\x90\x94Rh\x01\0\0\0\0\0\0\0\0\x82\x04\x81\x0B\x97\x86\x01\x97\x90\x97R`\x01``\x1B\x81\x04\x90\x96\x0B\x90\x84\x01R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x82\x01R\x90\x91a\x19\xD6\x90c;\x9A\xCA\0a2SV[`\x0F\x0B\x82R` \x81\x01Qa\x19\xF1\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x0B` \x83\x01R`@\x81\x01Qa\x1A\x0F\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x0B`@\x83\x01R``\x81\x01Qa\x1A-\x90`\x03\x0Bc;\x9A\xCA\0a2SV[`\x0F\x90\x81\x0B``\x84\x01R`\x80\x91\x82\x01Q\x90\x0B\x90\x82\x01R\x91\x90PV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a\x1A\x8AWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a\x1A\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03a\x1B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0\x82`\x0F\x0B\x12a\x1B.W\x81a\x07\xA5V[P`\0\x03\x90V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90a\x1ByW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a\x1A_Wa\x1A_a1\x8BV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`j` \x90\x81R`@\x91\x82\x90 \x83Q\x91\x84\x01Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x91\x84\x16\x82\x02\x17\x82U\x92\x84\x01Q``\x85\x01Q\x90\x83\x16\x92\x16\x90\x92\x02\x17`\x01\x90\x91\x01Ua\x1B\xF6\x82a$\xD0V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[\x82Qa\x1Cg\x90`\x0F\x0Ba\x1A\xCBV[\x84``\x01\x81\x81Qa\x1Cx\x91\x90a1\xE0V[`\x0F\x90\x81\x0B\x90\x91R\x84Q`\0\x92P\x90\x0B\x81\x12a\x1C\x98W\x84` \x01Qa\x1C\x9BV[\x84Q[\x90P`\0\x84`@\x01Q\x82a\x1C\xAF\x91\x90a1\xE0V[\x90P`\0a\x1C\xCD\x86`\0\x01Q\x83`\x0F\x0Ba\x1AH\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1C\xD7\x90\x85a1\xE0V[\x90P\x84\x86`\0\x01\x81\x81Qa\x1C\xEB\x91\x90a/\xF9V[`\x0F\x0B\x90RP` \x86\x01\x80Q\x82\x91\x90a\x1D\x05\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R\x87Q`\0\x91\x0B\x13\x15\x90Pa\x1DEW\x85Q``\x88\x01\x80Qa\x1D.\x90\x83\x90a/\xF9V[`\x0F\x90\x81\x0B\x90\x91R\x88Q\x90\x0B`@\x88\x01RPa\x1DnV[\x85Q``\x88\x01\x80Qa\x1DX\x90\x83\x90a1\xE0V[`\x0F\x90\x81\x0B\x90\x91R` \x89\x01Q\x90\x0B`@\x88\x01RP[PPPPPPPV[`g\x80T`\0\x91\x90a\x1D\x8B\x90`\x01\x90a2\xF1V[\x81T\x81\x10a\x1D\x9BWa\x1D\x9Ba/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\0\x80\x80[`gT\x81\x10\x15a\x1A\xC3W`\0`g\x82\x81T\x81\x10a\x1D\xEAWa\x1D\xEAa/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x84a\x01\0a\x1E \x91\x90a3\x08V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a\x1E\\WPa\x1EA\x85`\x01a3+V[a\x1EM\x90a\x01\0a3\x08V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[\x15a\x1E|Wa\x1Ema\x01\0\x82a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x83\x17\x92P[P\x80a\x1E\x87\x81a3SV[\x91PPa\x1D\xCBV[`\0\x80a\x1E\x9E\x85a\x01\0a3\x08V[\x90P[\x85\x15a\x1E\xE1W`\x01\x86\x16\x15a\x1E\xC8Wa\x1E\xBB\x81\x85\x85a%\x0CV[a\x1E\xC5\x90\x83a/\xF9V[\x91P[`\x01\x95\x90\x95\x1C\x94\x80a\x1E\xD9\x81a0HV[\x91PPa\x1E\xA1V[P\x94\x93PPPPV[`\0\x80`\0a\x1E\xF9\x85\x85a\x0E=V[\x80Q` \x90\x91\x01Q\x90\x96\x90\x95P\x93PPPPV[`\0`\x02\x82`\x02\x81\x11\x15a\x1F#Wa\x1F#a)\x97V[\x03a\x1F7WPg\r\xE0\xB6\xB3\xA7d\0\0a\tgV[`\0\x80\x84`\x0F\x0B\x12a\x1FpW`\0\x83`\x02\x81\x11\x15a\x1FWWa\x1FWa)\x97V[\x14a\x1FfW\x84`@\x01Qa\x1FiV[\x84Q[\x90Pa\x1F\x9CV[`\0\x83`\x02\x81\x11\x15a\x1F\x84Wa\x1F\x84a)\x97V[\x14a\x1F\x93W\x84``\x01Qa\x1F\x99V[\x84` \x01Q[\x90P[\x94\x93PPPPV[`\0\x81`@\x01Q`\x03\x0B\x82`\0\x01Q`\x03\x0B\x13\x15\x80\x15a\x1F\xCFWPc;\x9A\xCA\0\x82`@\x01Q`\x03\x0B\x13\x15[\x80\x15a\x1F\xE9WP\x81``\x01Q`\x03\x0B\x82` \x01Q`\x03\x0B\x12\x15[\x80\x15a \0WPc;\x9A\xCA\0\x82``\x01Q`\x03\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bBPC`\xE8\x1B\x81RP\x90a :W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\t\xEA\x91\x90a/aV[P`gT\x15\x80a \x9AWP`g\x80Ta U\x90`\x01\x90a2\xF1V[\x81T\x81\x10a eWa ea/\x05V[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x11[\x15a!iW`g\x80T`\x01\x81\x01\x82U`\0\x91\x90\x91R`\x08\x81\x04\x7F\x97\x87\xEE\xB9\x1F\xE3\x10\x125\xE4\xA7`c\xC7\x02>\xCB@\xF9#\xF9y\x16c\x9CY\x85\x92\xFA0\xD6\xAE\x01\x80Tc\xFF\xFF\xFF\xFF\x80\x8A\x16`\x04`\x07\x90\x95\x16\x85\x02a\x01\0\n\x90\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U`fT`@QcC\xB1j\x11`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x87b\xD4\"\x91a!2\x91\x8A\x91\x01c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!LW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!`W=`\0\x80>=`\0\xFD[PPPP`\x01\x90P[\x80\x15a!\xF1W`eT`\x01`\x01`\xA0\x1B\x03\x16`\x80\x83\x01Q`@Qc\x1C\xB2\x81[`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01R`\x0F\x91\x90\x91\x0B`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c9e\x02\xB6\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xE8W=`\0\x80>=`\0\xFD[PPPPa\"6V[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` R`@\x90 T`\x01`\x80\x1B\x90\x04`\x0F\x0B`\x80\x83\x01R[c\xFF\xFF\xFF\xFF\x86\x81\x16`\0\x90\x81R\x7F\\\tu^\x0E\x1E\x06\xC1\xC8\x97\xC3\xC3|\xBCh\xE5\x8A\xF1\xAD.R\x99k\xDF\x19\xFD<+\xDE\xCB\xC0y` \x90\x81R`@\x91\x82\x90 \x85Q\x81T\x92\x87\x01Q\x93\x87\x01Q``\x88\x01Q`\x80\x89\x01Q\x92\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x94\x90\x94\x17d\x01\0\0\0\0\x95\x87\x16\x95\x90\x95\x02\x94\x90\x94\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16h\x01\0\0\0\0\0\0\0\0\x94\x86\x16\x94\x90\x94\x02o\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x19\x16\x93\x90\x93\x17`\x01``\x1B\x92\x90\x94\x16\x91\x90\x91\x02\x92\x90\x92\x17`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x91\x90\x92\x16\x02\x17\x90Ua#\x16a&\x08V[`@Qc\xC8\xD6\xDB\xCB`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x89\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`\x0F\x86\x81\x0B`D\x83\x01R\x85\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xC8\xD6\xDB\xCB\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a#yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a#\x8DW=`\0\x80>=`\0\xFD[PP`@Qc\xFF\xFF\xFF\xFF\x89\x16\x81R\x7F'\x9D\x95t\x82N\xD2[\xA9\xED\x81S\xD4+ \xC6A\xA3\xE4n\xC9\xEB=\xCD{Q\xABm\xB6s\x95m\x92P` \x01\x90P`@Q\x80\x91\x03\x90\xA1\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`k` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x84Q\x92\x85\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x81\x85\x16\x17\x82U\x91\x85\x01Q`\x01\x90\x91\x01\x80T\x91\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x90\x91\x16\x17\x90U`\x0F\x0B\x15\x15\x80a$FWP` \x82\x01Q`\x0F\x0B\x15\x15[\x90Pa$S\x83\x85\x83a&\x82V[a\x0F\r\x84\x84a\x08\xCAV[`\0Ta\x01\0\x90\x04`\xFF\x16a$\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x0E;a'?V[`@Qc\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F\xE6\x19Q\"\xB3\x134\xB8\xA2\xBD^\xC6O\r\xD6\xAC:\xB8e\xACT\xC2\xA0A?\xB8-\xFB\"\xADd2\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a%\x18\x85a\x19\x15V[\x90P`\0\x80a%'\x87\x87a\x1E\xEAV[\x91P\x91P`\0a%8\x84\x84\x88a\x1F\rV[\x90Pa%D\x82\x86a/\xF9V[\x94P\x82`\x0F\x0B`\0\x14a%\xFDWa%dg\r\xE0\xB6\xB3\xA7d\0\0`\x02a2SV[`\x0F\x0B\x81`\x0F\x0B\x03a%\x96Wa%\x82`\x80`\x01`\x01`\x7F\x1B\x03a3lV[a%\x8B\x90a/\xD3V[\x94PPPPPa\tgV[`\x80\x84\x01Qa%\xB6\x90a%\xAD`\x0F\x86\x90\x0B\x84a\x1AHV[`\x0F\x0B\x90a\x1AHV[a%\xC0\x90\x86a/\xF9V[`@Qc\xFF\xFF\xFF\xFF\x8A\x16\x81R\x90\x95P\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPP\x93\x92PPPV[`\0a&\x1C`eT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\x8FO\x8E\xCC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&}\x91\x90a3\xB3V[\x90P\x90V[\x80\x15a&\xE3Wa&\x94a\x01\0\x83a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a&\xC1\x91\x90a1\xA1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x17\x90UPPPV[a&\xEFa\x01\0\x83a20V[c\xFF\xFF\xFF\xFF\x16`\x01\x90\x1B\x19`i`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\x85a'\x1D\x91\x90a1\xA1V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91\x16\x90UPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\t\xEAV[a\x0E;3a\x1B\xFAV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x86W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a'\xE0W`\0\x80\xFD[\x855a'\xEB\x81a'\xB3V[\x94P` \x86\x015a'\xFB\x81a'\xB3V[\x93P`@\x86\x015a(\x0B\x81a'\xB3V[\x92P``\x86\x015a(\x1B\x81a'\xB3V[\x91P`\x80\x86\x015a(+\x81a'\xB3V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(MW`\0\x80\xFD[\x91\x90PV[\x80`\x0F\x0B\x81\x14a\x15\x86W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a(tW`\0\x80\xFD[a(}\x83a(9V[\x91P` \x83\x015a(\x8D\x81a(RV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a(\xABW`\0\x80\xFD[a(\xB4\x83a(9V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a(\xD4W`\0\x80\xFD[a\tg\x82a(9V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a(\xF3W`\0\x80\xFD[a(\xFC\x85a(9V[\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a)\x17W`\0\x80\xFD[P`@\x84\x01\x90P\x92P\x92P\x92V[`\x0F\x84\x90\x0B\x81Ra\x01\0\x81\x01a)k` \x83\x01\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\xA0\x84\x01R` \x84\x01Q\x81\x0B`\xC0\x84\x01R`@\x84\x01Q\x90\x0B`\xE0\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a)\xCFWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a*\x13W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a)\xF1V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a*1W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*IW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x14\xC6W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a*yW`\0\x80\xFD[\x835`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a*\x90W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xACW`\0\x80\xFD[a*\xB8\x86\x82\x87\x01a*\x1FV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80` \x83\x85\x03\x12\x15a*\xD8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xEFW`\0\x80\xFD[a*\xFB\x85\x82\x86\x01a*\x1FV[\x90\x96\x90\x95P\x93PPPPV[``\x81\x01a\x07\xA5\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a+IW`\0\x80\xFD[a+R\x84a(9V[\x92P`\x80`\x1F\x19\x82\x01\x12\x15a+fW`\0\x80\xFD[P` \x83\x01\x90P\x92P\x92\x90PV[\x805`\x03\x81\x10a(MW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a+\x96W`\0\x80\xFD[\x825\x91Pa+\xA6` \x84\x01a+tV[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a+\xC4W`\0\x80\xFD[\x835\x92Pa+\xD4` \x85\x01a(9V[\x91Pa+\xE2`@\x85\x01a+tV[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a,\x03W`\0\x80\xFD[a,\x0C\x86a(9V[\x94P` \x86\x015a,\x1C\x81a(RV[\x93P`@\x86\x015a,,\x81a(RV[\x92P`\xA0`_\x19\x82\x01\x12\x15a,@W`\0\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[`\0\x80`@\x83\x85\x03\x12\x15a,aW`\0\x80\xFD[\x825\x91P` \x83\x015a(\x8D\x81a(RV[\x805`\x03\x81\x90\x0B\x81\x14a(MW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a,\x97W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a,\xC8WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90P\x80a,\xD7\x83a,sV[\x81Ra,\xE5` \x84\x01a,sV[` \x82\x01Ra,\xF6`@\x84\x01a,sV[`@\x82\x01Ra-\x07``\x84\x01a,sV[``\x82\x01R`\x80\x83\x015a-\x1A\x81a(RV[`\x80\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15a-:W`\0\x80\xFD[a-C\x83a(9V[\x91Pa+\xA6\x84` \x85\x01a,\x85V[`\0\x80`@\x83\x85\x03\x12\x15a-eW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\xE0\x81\x01a-\xAF\x82\x85\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[\x82Q`\x0F\x90\x81\x0B`\x80\x84\x01R` \x84\x01Q\x81\x0B`\xA0\x84\x01R`@\x84\x01Q\x90\x0B`\xC0\x83\x01Ra\tgV[`\x80\x81\x01a\x07\xA5\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15a.%W`\0\x80\xFD[\x815a\tg\x81a'\xB3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.FW`\0\x80\xFD[a.O\x85a(9V[\x93P` \x85\x015\x92P`@\x85\x015a.f\x81a(RV[\x91P``\x85\x015a.v\x81a(RV[\x93\x96\x92\x95P\x90\x93PPV[\x815a.\x8C\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015a.\xB4\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`@\x82\x015a.\xE0\x81a(RV[`\x01\x82\x01\x80T`\x01`\x01`\x80\x1B\x03\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x91\x90\x91\x16\x17\x90UPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a/XWa/Xa/\x1BV[\x02\x94\x93PPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a/\x8EW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a/rV[\x81\x81\x11\x15a/\xA0W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a/\xC8W`\0\x80\xFD[\x815a\tg\x81a(RV[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a/\xF0Wa/\xF0a/\x1BV[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a0#Wa0#a/\x1BV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a0?Wa0?a/\x1BV[P\x01\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a0aWa0aa/\x1BV[`\x01\x01\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a0\x82W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a0\x9DW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x14\xC6W`\0\x80\xFD[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a0aWa0aa/\x1BV[\x815a0\xE9\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP` \x82\x015a1\x11\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UP`\x01\x81\x01`@\x83\x015a1A\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x82\x16\x17\x82UP``\x83\x015a1i\x81a(RV[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x82\x90\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82Ua\x0F\rV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a1\xB8Wa1\xB8a1\x8BV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a1\xD6W`\0\x80\xFD[a\tg\x83\x83a,\x85V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a2\x0BWa2\x0Ba/\x1BV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a2&Wa2&a/\x1BV[P\x90\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80a2GWa2Ga1\x8BV[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15a2\x83Wa2\x83a/\x1BV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15a2\xAFWa2\xAFa/\x1BV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15a2\xCBWa2\xCBa/\x1BV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15a2\xE1Wa2\xE1a/\x1BV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0\x82\x82\x10\x15a3\x03Wa3\x03a/\x1BV[P\x03\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a/XWa/Xa/\x1BV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a3JWa3Ja/\x1BV[\x01\x94\x93PPPPV[`\0`\x01\x82\x01a3eWa3ea/\x1BV[P`\x01\x01\x90V[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a3\x83Wa3\x83a1\x8BV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3\xAAWa3\xAAa/\x1BV[\x90\x05\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3\xC5W`\0\x80\xFD[\x81Qa\tg\x81a'\xB3V\xFE\xA2dipfsX\"\x12 \xD3c\xE3\xD1=\xE1\xD7Z\xA6\xD7\xCB\xB2@A%\"\xD3\xEDC\xDB\xB1\xFB\x0E\t'FI\xCE\xD8*dqdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static PERPENGINE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct PerpEngine<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PerpEngine<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PerpEngine<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PerpEngine<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PerpEngine<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PerpEngine))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PerpEngine<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PERPENGINE_ABI.clone(),
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
                PERPENGINE_ABI.clone(),
                PERPENGINE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addOrUpdateProduct` (0xa45989ab) function
        pub fn add_or_update_product(
            &self,
            product_id: u32,
            size_increment: i128,
            min_size: i128,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 89, 137, 171],
                    (product_id, size_increment, min_size, risk_store),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balances` (0xbf4c8f5f) function
        pub fn balances(
            &self,
            p0: u32,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128, i128)> {
            self.0
                .method_hash([191, 76, 143, 95], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitBalanceUpdate` (0x4fa0f726) function
        pub fn emit_balance_update(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 160, 247, 38], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAvailableSettle` (0x3056f78f) function
        pub fn get_available_settle(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([48, 86, 247, 143], product_id)
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
        ///Calls the contract's `getPositionPnl` (0x1769225f) function
        pub fn get_position_pnl(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([23, 105, 34, 95], (product_id, subaccount))
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
        ) -> ::ethers::contract::builders::ContractCall<M, Balance> {
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
        ///Calls the contract's `getSettlementState` (0x388927b8) function
        pub fn get_settlement_state(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, State, Balance)> {
            self.0
                .method_hash([56, 137, 39, 184], (product_id, subaccount))
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
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (clearinghouse, offchain_exchange, p2, endpoint, admin),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0x6792dcc1) function
        pub fn manual_assert(
            &self,
            states: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 146, 220, 193], states)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrationFlag` (0xc362d19e) function
        pub fn migration_flag(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([195, 98, 209, 158], ())
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
        ///Calls the contract's `perpPositionClosed` (0x64c42cc2) function
        pub fn perp_position_closed(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 196, 44, 194], (product_id, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBalance` (0x385de9c3) function
        pub fn set_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            balance: Balance,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 93, 233, 195], (product_id, subaccount, balance))
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
        ///Calls the contract's `settlePnl` (0xd6b0e0b5) function
        pub fn settle_pnl(
            &self,
            subaccount: [u8; 32],
            product_ids: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([214, 176, 224, 181], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `socializeSubaccount` (0xb1cd4b8f) function
        pub fn socialize_subaccount(
            &self,
            subaccount: [u8; 32],
            insurance: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([177, 205, 75, 143], (subaccount, insurance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `states` (0x7f17baad) function
        pub fn states(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128, i128, i128)> {
            self.0
                .method_hash([127, 23, 186, 173], p0)
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
        ///Calls the contract's `updateBalance` (0xf8a42e51) function
        pub fn update_balance(
            &self,
            product_id: u32,
            subaccount: [u8; 32],
            amount_delta: i128,
            v_quote_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 164, 46, 81],
                    (product_id, subaccount, amount_delta, v_quote_delta),
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
        ///Calls the contract's `updateStates` (0x6736f5da) function
        pub fn update_states(
            &self,
            dt: u128,
            avg_price_diffs: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 54, 245, 218], (dt, avg_price_diffs))
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
        ///Gets the contract's `FundingPayment` event
        pub fn funding_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FundingPaymentFilter>
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PerpEngineEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PerpEngine<M> {
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
    #[ethevent(
        name = "FundingPayment",
        abi = "FundingPayment(uint32,uint128,int128,int128)"
    )]
    pub struct FundingPaymentFilter {
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
        pub open_interest: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub payment: i128,
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
    pub enum PerpEngineEvents {
        AddOrUpdateProductFilter(AddOrUpdateProductFilter),
        BalanceUpdateFilter(BalanceUpdateFilter),
        FundingPaymentFilter(FundingPaymentFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceQueryFilter(PriceQueryFilter),
        ProductUpdateFilter(ProductUpdateFilter),
    }
    impl ::ethers::contract::EthLogDecode for PerpEngineEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddOrUpdateProductFilter::decode_log(log) {
                return Ok(PerpEngineEvents::AddOrUpdateProductFilter(decoded));
            }
            if let Ok(decoded) = BalanceUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::BalanceUpdateFilter(decoded));
            }
            if let Ok(decoded) = FundingPaymentFilter::decode_log(log) {
                return Ok(PerpEngineEvents::FundingPaymentFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PerpEngineEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PerpEngineEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PriceQueryFilter::decode_log(log) {
                return Ok(PerpEngineEvents::PriceQueryFilter(decoded));
            }
            if let Ok(decoded) = ProductUpdateFilter::decode_log(log) {
                return Ok(PerpEngineEvents::ProductUpdateFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PerpEngineEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddOrUpdateProductFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FundingPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceQueryFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProductUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddOrUpdateProductFilter> for PerpEngineEvents {
        fn from(value: AddOrUpdateProductFilter) -> Self {
            Self::AddOrUpdateProductFilter(value)
        }
    }
    impl ::core::convert::From<BalanceUpdateFilter> for PerpEngineEvents {
        fn from(value: BalanceUpdateFilter) -> Self {
            Self::BalanceUpdateFilter(value)
        }
    }
    impl ::core::convert::From<FundingPaymentFilter> for PerpEngineEvents {
        fn from(value: FundingPaymentFilter) -> Self {
            Self::FundingPaymentFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for PerpEngineEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for PerpEngineEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PriceQueryFilter> for PerpEngineEvents {
        fn from(value: PriceQueryFilter) -> Self {
            Self::PriceQueryFilter(value)
        }
    }
    impl ::core::convert::From<ProductUpdateFilter> for PerpEngineEvents {
        fn from(value: ProductUpdateFilter) -> Self {
            Self::ProductUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `addOrUpdateProduct` function with signature `addOrUpdateProduct(uint32,int128,int128,(int32,int32,int32,int32,int128))` and selector `0xa45989ab`
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
        abi = "addOrUpdateProduct(uint32,int128,int128,(int32,int32,int32,int32,int128))"
    )]
    pub struct AddOrUpdateProductCall {
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
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `balances` function with signature `balances(uint32,bytes32)` and selector `0xbf4c8f5f`
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
    #[ethcall(name = "balances", abi = "balances(uint32,bytes32)")]
    pub struct BalancesCall(pub u32, pub [u8; 32]);
    ///Container type for all input parameters for the `emitBalanceUpdate` function with signature `emitBalanceUpdate(uint32,bytes32)` and selector `0x4fa0f726`
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
    #[ethcall(name = "emitBalanceUpdate", abi = "emitBalanceUpdate(uint32,bytes32)")]
    pub struct EmitBalanceUpdateCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `0x3056f78f`
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
    #[ethcall(name = "getAvailableSettle", abi = "getAvailableSettle(uint32)")]
    pub struct GetAvailableSettleCall {
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
    ///Container type for all input parameters for the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `0x1769225f`
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
    #[ethcall(name = "getPositionPnl", abi = "getPositionPnl(uint32,bytes32)")]
    pub struct GetPositionPnlCall {
        pub product_id: u32,
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
    ///Container type for all input parameters for the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `0x388927b8`
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
        name = "getSettlementState",
        abi = "getSettlementState(uint32,bytes32)"
    )]
    pub struct GetSettlementStateCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
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
        pub p2: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert(bytes[])` and selector `0x6792dcc1`
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
    #[ethcall(name = "manualAssert", abi = "manualAssert(bytes[])")]
    pub struct ManualAssertCall {
        pub states: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `migrationFlag` function with signature `migrationFlag()` and selector `0xc362d19e`
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
    #[ethcall(name = "migrationFlag", abi = "migrationFlag()")]
    pub struct MigrationFlagCall;
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
    ///Container type for all input parameters for the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `0x64c42cc2`
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
        name = "perpPositionClosed",
        abi = "perpPositionClosed(uint32,bytes32)"
    )]
    pub struct PerpPositionClosedCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
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
    ///Container type for all input parameters for the `setBalance` function with signature `setBalance(uint32,bytes32,(int128,int128,int128))` and selector `0x385de9c3`
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
        name = "setBalance",
        abi = "setBalance(uint32,bytes32,(int128,int128,int128))"
    )]
    pub struct SetBalanceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub balance: Balance,
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
    ///Container type for all input parameters for the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `0xd6b0e0b5`
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
    #[ethcall(name = "settlePnl", abi = "settlePnl(bytes32,uint256)")]
    pub struct SettlePnlCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub product_ids: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `0xb1cd4b8f`
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
        name = "socializeSubaccount",
        abi = "socializeSubaccount(bytes32,int128)"
    )]
    pub struct SocializeSubaccountCall {
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
    ///Container type for all input parameters for the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    #[ethcall(name = "states", abi = "states(uint32)")]
    pub struct StatesCall(pub u32);
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub v_quote_delta: i128,
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
    ///Container type for all input parameters for the `updateStates` function with signature `updateStates(uint128,int128[])` and selector `0x6736f5da`
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
    #[ethcall(name = "updateStates", abi = "updateStates(uint128,int128[])")]
    pub struct UpdateStatesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub dt: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub avg_price_diffs: ::std::vec::Vec<i128>,
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
    pub enum PerpEngineCalls {
        AddOrUpdateProduct(AddOrUpdateProductCall),
        Balances(BalancesCall),
        EmitBalanceUpdate(EmitBalanceUpdateCall),
        GetAvailableSettle(GetAvailableSettleCall),
        GetBalance(GetBalanceCall),
        GetClearinghouse(GetClearinghouseCall),
        GetCoreRisk(GetCoreRiskCall),
        GetEndpoint(GetEndpointCall),
        GetEngineType(GetEngineTypeCall),
        GetHealthContribution(GetHealthContributionCall),
        GetPositionPnl(GetPositionPnlCall),
        GetProductIds(GetProductIdsCall),
        GetRawBalance(GetRawBalanceCall),
        GetRawState(GetRawStateCall),
        GetRisk(GetRiskCall),
        GetSettlementState(GetSettlementStateCall),
        GetSlots(GetSlotsCall),
        GetStateAndBalance(GetStateAndBalanceCall),
        Initialize(InitializeCall),
        ManualAssert(ManualAssertCall),
        MigrationFlag(MigrationFlagCall),
        Owner(OwnerCall),
        PerpPositionClosed(PerpPositionClosedCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBalance(SetBalanceCall),
        SetState(SetStateCall),
        SettlePnl(SettlePnlCall),
        SocializeSubaccount(SocializeSubaccountCall),
        States(StatesCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateBalance(UpdateBalanceCall),
        UpdatePrice(UpdatePriceCall),
        UpdateRisk(UpdateRiskCall),
        UpdateStates(UpdateStatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for PerpEngineCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddOrUpdateProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddOrUpdateProduct(decoded));
            }
            if let Ok(decoded) = <BalancesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balances(decoded));
            }
            if let Ok(decoded) =
                <EmitBalanceUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitBalanceUpdate(decoded));
            }
            if let Ok(decoded) =
                <GetAvailableSettleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAvailableSettle(decoded));
            }
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
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
                <GetPositionPnlCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPositionPnl(decoded));
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
            if let Ok(decoded) =
                <GetSettlementStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSettlementState(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
            }
            if let Ok(decoded) =
                <GetStateAndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStateAndBalance(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MigrationFlagCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MigrationFlag(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerpPositionClosedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpPositionClosed(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBalance(decoded));
            }
            if let Ok(decoded) = <SetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetState(decoded));
            }
            if let Ok(decoded) = <SettlePnlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SettlePnl(decoded));
            }
            if let Ok(decoded) =
                <SocializeSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SocializeSubaccount(decoded));
            }
            if let Ok(decoded) = <StatesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::States(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateBalance(decoded));
            }
            if let Ok(decoded) = <UpdatePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePrice(decoded));
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
    impl ::ethers::core::abi::AbiEncode for PerpEngineCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddOrUpdateProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Balances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitBalanceUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAvailableSettle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCoreRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEngineType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHealthContribution(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositionPnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetProductIds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRawState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSettlementState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStateAndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrationFlag(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpPositionClosed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettlePnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SocializeSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::States(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateStates(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PerpEngineCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddOrUpdateProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::Balances(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitBalanceUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAvailableSettle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCoreRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthContribution(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionPnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSettlementState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStateAndBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrationFlag(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpPositionClosed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlePnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::SocializeSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::States(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddOrUpdateProductCall> for PerpEngineCalls {
        fn from(value: AddOrUpdateProductCall) -> Self {
            Self::AddOrUpdateProduct(value)
        }
    }
    impl ::core::convert::From<BalancesCall> for PerpEngineCalls {
        fn from(value: BalancesCall) -> Self {
            Self::Balances(value)
        }
    }
    impl ::core::convert::From<EmitBalanceUpdateCall> for PerpEngineCalls {
        fn from(value: EmitBalanceUpdateCall) -> Self {
            Self::EmitBalanceUpdate(value)
        }
    }
    impl ::core::convert::From<GetAvailableSettleCall> for PerpEngineCalls {
        fn from(value: GetAvailableSettleCall) -> Self {
            Self::GetAvailableSettle(value)
        }
    }
    impl ::core::convert::From<GetBalanceCall> for PerpEngineCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for PerpEngineCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetCoreRiskCall> for PerpEngineCalls {
        fn from(value: GetCoreRiskCall) -> Self {
            Self::GetCoreRisk(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for PerpEngineCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetEngineTypeCall> for PerpEngineCalls {
        fn from(value: GetEngineTypeCall) -> Self {
            Self::GetEngineType(value)
        }
    }
    impl ::core::convert::From<GetHealthContributionCall> for PerpEngineCalls {
        fn from(value: GetHealthContributionCall) -> Self {
            Self::GetHealthContribution(value)
        }
    }
    impl ::core::convert::From<GetPositionPnlCall> for PerpEngineCalls {
        fn from(value: GetPositionPnlCall) -> Self {
            Self::GetPositionPnl(value)
        }
    }
    impl ::core::convert::From<GetProductIdsCall> for PerpEngineCalls {
        fn from(value: GetProductIdsCall) -> Self {
            Self::GetProductIds(value)
        }
    }
    impl ::core::convert::From<GetRawBalanceCall> for PerpEngineCalls {
        fn from(value: GetRawBalanceCall) -> Self {
            Self::GetRawBalance(value)
        }
    }
    impl ::core::convert::From<GetRawStateCall> for PerpEngineCalls {
        fn from(value: GetRawStateCall) -> Self {
            Self::GetRawState(value)
        }
    }
    impl ::core::convert::From<GetRiskCall> for PerpEngineCalls {
        fn from(value: GetRiskCall) -> Self {
            Self::GetRisk(value)
        }
    }
    impl ::core::convert::From<GetSettlementStateCall> for PerpEngineCalls {
        fn from(value: GetSettlementStateCall) -> Self {
            Self::GetSettlementState(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for PerpEngineCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
        }
    }
    impl ::core::convert::From<GetStateAndBalanceCall> for PerpEngineCalls {
        fn from(value: GetStateAndBalanceCall) -> Self {
            Self::GetStateAndBalance(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for PerpEngineCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for PerpEngineCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<MigrationFlagCall> for PerpEngineCalls {
        fn from(value: MigrationFlagCall) -> Self {
            Self::MigrationFlag(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PerpEngineCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpPositionClosedCall> for PerpEngineCalls {
        fn from(value: PerpPositionClosedCall) -> Self {
            Self::PerpPositionClosed(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for PerpEngineCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetBalanceCall> for PerpEngineCalls {
        fn from(value: SetBalanceCall) -> Self {
            Self::SetBalance(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for PerpEngineCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SettlePnlCall> for PerpEngineCalls {
        fn from(value: SettlePnlCall) -> Self {
            Self::SettlePnl(value)
        }
    }
    impl ::core::convert::From<SocializeSubaccountCall> for PerpEngineCalls {
        fn from(value: SocializeSubaccountCall) -> Self {
            Self::SocializeSubaccount(value)
        }
    }
    impl ::core::convert::From<StatesCall> for PerpEngineCalls {
        fn from(value: StatesCall) -> Self {
            Self::States(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for PerpEngineCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateBalanceCall> for PerpEngineCalls {
        fn from(value: UpdateBalanceCall) -> Self {
            Self::UpdateBalance(value)
        }
    }
    impl ::core::convert::From<UpdatePriceCall> for PerpEngineCalls {
        fn from(value: UpdatePriceCall) -> Self {
            Self::UpdatePrice(value)
        }
    }
    impl ::core::convert::From<UpdateRiskCall> for PerpEngineCalls {
        fn from(value: UpdateRiskCall) -> Self {
            Self::UpdateRisk(value)
        }
    }
    impl ::core::convert::From<UpdateStatesCall> for PerpEngineCalls {
        fn from(value: UpdateStatesCall) -> Self {
            Self::UpdateStates(value)
        }
    }
    ///Container type for all return fields from the `balances` function with signature `balances(uint32,bytes32)` and selector `0xbf4c8f5f`
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
    pub struct BalancesReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
    }
    ///Container type for all return fields from the `getAvailableSettle` function with signature `getAvailableSettle(uint32)` and selector `0x3056f78f`
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
    pub struct GetAvailableSettleReturn(pub i128);
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
    ///Container type for all return fields from the `getPositionPnl` function with signature `getPositionPnl(uint32,bytes32)` and selector `0x1769225f`
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
    pub struct GetPositionPnlReturn(pub i128);
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
    pub struct GetRawBalanceReturn(pub Balance);
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
    ///Container type for all return fields from the `getSettlementState` function with signature `getSettlementState(uint32,bytes32)` and selector `0x388927b8`
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
    pub struct GetSettlementStateReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub available_settle: i128,
        pub state: State,
        pub balance: Balance,
    }
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
    ///Container type for all return fields from the `migrationFlag` function with signature `migrationFlag()` and selector `0xc362d19e`
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
    pub struct MigrationFlagReturn(pub u128);
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
    ///Container type for all return fields from the `perpPositionClosed` function with signature `perpPositionClosed(uint32,bytes32)` and selector `0x64c42cc2`
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
    pub struct PerpPositionClosedReturn(pub bool);
    ///Container type for all return fields from the `settlePnl` function with signature `settlePnl(bytes32,uint256)` and selector `0xd6b0e0b5`
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
    pub struct SettlePnlReturn(pub i128);
    ///Container type for all return fields from the `socializeSubaccount` function with signature `socializeSubaccount(bytes32,int128)` and selector `0xb1cd4b8f`
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
    pub struct SocializeSubaccountReturn(pub i128);
    ///Container type for all return fields from the `states` function with signature `states(uint32)` and selector `0x7f17baad`
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
    pub struct StatesReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub open_interest: i128,
    }
    ///`Balance(int128,int128,int128)`
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub v_quote_balance: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub last_cumulative_funding_x18: i128,
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
        pub cumulative_funding_long_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub cumulative_funding_short_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub available_settle: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub open_interest: i128,
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
