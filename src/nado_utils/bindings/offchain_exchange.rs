pub use offchain_exchange::*;
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
pub mod offchain_exchange {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assertProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transaction"),
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
                    ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IEndpoint.CreateIsolatedSubaccount",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("linkedSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dropDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropDigest"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("digest"),
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
                    ::std::borrow::ToOwned::to_owned("dropOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropOrder"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dropOrderChecked"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dumpFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dumpFees"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("filledAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("filledAmounts"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getAllFeeTiers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllFeeTiers"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("users"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCollectedFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedMakerFees",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedTakerFees",),
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
                    ::std::borrow::ToOwned::to_owned("getCustomFeeAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCustomFeeAddresses",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("startAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("limit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDigest"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getFeeFractionX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeeFractionX18"),
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
                                name: ::std::borrow::ToOwned::to_owned("taker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getFeeRatesX18"),
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
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getIsolatedSubaccountByDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getIsolatedSubaccountByDigest",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("digest"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getIsolatedSubaccounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getIsolatedSubaccounts",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarginByDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMarginByDigest"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("digest"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getMarketInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMarketInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("m"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.MarketInfo",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMinSize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMinSize"),
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
                    ::std::borrow::ToOwned::to_owned("getOrderFilledAmounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOrderFilledAmounts",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.Order"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getParentSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getParentSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRawMarketInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRawMarketInfo"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.MarketInfoStore",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSizeIncrement"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSizeIncrement"),
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
                    ::std::borrow::ToOwned::to_owned("getSlots"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlots"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("filledAmountsSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerFeesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerFeesSlot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketInfoSlot"),
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
                    ::std::borrow::ToOwned::to_owned("getTierFeeRateX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTierFeeRateX18"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IOffchainExchange.FeeRates",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("incrementFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerFee"),
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
                                name: ::std::borrow::ToOwned::to_owned("_endpoint"),
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
                    ::std::borrow::ToOwned::to_owned("isIsolatedSubaccountActive"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isIsolatedSubaccountActive",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("parent"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("matchOrders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.MatchOrdersWithSigner",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("modifyFilledAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("takerDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("orderVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("orderVersion"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("setFeeTier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeTier"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFilledAmount"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("filledAmount"),
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
                    ::std::borrow::ToOwned::to_owned("tryCloseIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tryCloseIsolatedSubaccount",),
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
                    ::std::borrow::ToOwned::to_owned("updateCollectedFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateCollectedFees",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collectedFees"),
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
                    ::std::borrow::ToOwned::to_owned("updateFeeTier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFeeTier"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newTier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateMarket"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txn"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.UpdateTierFeeRates",
                                ),
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
                    ::std::borrow::ToOwned::to_owned("CloseIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CloseIsolatedSubaccount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isolatedSubaccount",),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("parentSubaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTierUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeTierUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeTier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FillOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FillOrder"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("priceX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("expiration"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("appendix"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isolated"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isTaker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("baseDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quoteDelta"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static OFFCHAINEXCHANGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaP^\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD3W`\x005`\xE0\x1C\x80cp|\x8BX\x11a\x01\x86W\x80c\xBF)\x17L\x11a\0\xE3W\x80c\xED\xC6\xD3{\x11a\0\x97W\x80c\xF6\xEE{K\x11a\0qW\x80c\xF6\xEE{K\x14a\x08\x01W\x80c\xFA\xB2\xC4i\x14a\x08\x14W\x80c\xFF\x0B\xE9\xEF\x14a\x08yW`\0\x80\xFD[\x80c\xED\xC6\xD3{\x14a\x07\xBBW\x80c\xF2\xB2c1\x14a\x07\xDBW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xEEW`\0\x80\xFD[\x80c\xC8\xD6\xDB\xCB\x11a\0\xC8W\x80c\xC8\xD6\xDB\xCB\x14a\x07!W\x80c\xDE\x10x\xBD\x14a\x074W\x80c\xE1\xE7\x18\x8D\x14a\x07pW`\0\x80\xFD[\x80c\xBF)\x17L\x14a\x06\xFBW\x80c\xC7\x1E\xDE`\x14a\x07\x0EW`\0\x80\xFD[\x80c\x8D\xC3\xF4h\x11a\x01:W\x80c\xB5kjl\x11a\x01\x1FW\x80c\xB5kjl\x14a\x06\xC2W\x80c\xB5\xCB\xD7\x0E\x14a\x06\xD5W\x80c\xB6\n\xAA|\x14a\x06\xE8W`\0\x80\xFD[\x80c\x8D\xC3\xF4h\x14a\x06\x9EW\x80c\xAE\xD8\xE9g\x14a\x06\xB1W`\0\x80\xFD[\x80c\x81&\t\xF1\x11a\x01kW\x80c\x81&\t\xF1\x14a\x06&W\x80c\x84R\x80\x93\x14a\x06gW\x80c\x8D\xA5\xCB[\x14a\x06yW`\0\x80\xFD[\x80cp|\x8BX\x14a\x06\x16W\x80cqP\x18\xA6\x14a\x06\x1EW`\0\x80\xFD[\x80c4\xF9\xA4\xA4\x11a\x024W\x80c@\xF1\xA3M\x11a\x01\xE8W\x80cRi\x99`\x11a\x01\xCDW\x80cRi\x99`\x14a\x05\xABW\x80cj\xC3\xEE\x0B\x14a\x05\xBEW\x80ck\xAA\x1B\x83\x14a\x05\xE1W`\0\x80\xFD[\x80c@\xF1\xA3M\x14a\x05bW\x80cH\\\xC9U\x14a\x05\x98W`\0\x80\xFD[\x80c8]D\x8D\x11a\x02\x19W\x80c8]D\x8D\x14a\x04\x93W\x80c>\xB0\xF4\xB3\x14a\x04\xA6W\x80c?\xCE\xEA(\x14a\x05BW`\0\x80\xFD[\x80c4\xF9\xA4\xA4\x14a\x04mW\x80c5\xEDNm\x14a\x04\x80W`\0\x80\xFD[\x80c\x1D\x02\x9BM\x11a\x02\x8BW\x80c\x1FL\xE0\x16\x11a\x02pW\x80c\x1FL\xE0\x16\x14a\x03\xF9W\x80c*k?\xFE\x14a\x04\x0CW\x80c+\xD2\x86\x17\x14a\x04,W`\0\x80\xFD[\x80c\x1D\x02\x9BM\x14a\x03\x7FW\x80c\x1E\xD0\x1D\xAD\x14a\x03\xD9W`\0\x80\xFD[\x80c\x13\xB5m\xDB\x11a\x02\xBCW\x80c\x13\xB5m\xDB\x14a\x03\x19W\x80c\x1A+-\x16\x14a\x03GW\x80c\x1A\xB0\xA5\x88\x14a\x03jW`\0\x80\xFD[\x80c\x01\xE2\xAB\xD5\x14a\x02\xD8W\x80c\x0F,\x87\x8E\x14a\x02\xECW[`\0\x80\xFD[`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xFFa\x02\xFA6`\x04aA\xA5V[a\x08\x8CV[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x02\xE3V[a\x039a\x03'6`\x04aA\xD5V[`\0\x90\x81R`\xA5` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xE3V[a\x03Za\x03U6`\x04aA\xEEV[a\x08\xAEV[`@Q\x90\x15\x15\x81R` \x01a\x02\xE3V[a\x03}a\x03x6`\x04aB\x10V[a\t\x03V[\0[a\x03\x92a\x03\x8D6`\x04aBKV[a\x11\x05V[`@Qa\x02\xE3\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x03\xECa\x03\xE76`\x04aB\xF8V[a\x11\xC6V[`@Qa\x02\xE3\x91\x90aC\x97V[a\x03}a\x04\x076`\x04aC\xF0V[a\x12\xABV[a\x039a\x04\x1A6`\x04aA\xD5V[`\0\x90\x81R`\xA7` R`@\x90 T\x90V[a\x03}a\x04:6`\x04aD;V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xA0` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x039a\x04{6`\x04aE\xA8V[a\x13$V[a\x02\xFFa\x04\x8E6`\x04aFcV[a\x18\0V[a\x03}a\x04\xA16`\x04aD;V[a\x18\xC6V[a\x05\x15a\x04\xB46`\x04aBKV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x9B\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RT`\x07\x81\x81\x0B\x85R`\x01`@\x1B\x82\x04\x90\x0B\x91\x84\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x04`\x0F\x0B\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x07\x90\x81\x0B\x82R` \x80\x85\x01Q\x90\x91\x0B\x90\x82\x01R\x91\x81\x01Q`\x0F\x0B\x90\x82\x01R``\x01a\x02\xE3V[a\x05Ua\x05P6`\x04aF\x91V[a\x1A\nV[`@Qa\x02\xE3\x91\x90aF\xAFV[a\x05\x85a\x05p6`\x04aA\xD5V[`\x9C` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xE3V[a\x03}a\x05\xA66`\x04aF\xF0V[a\x1B:V[a\x02\xFFa\x05\xB96`\x04aG\x1EV[a\x1D\xE6V[a\x05\x85a\x05\xCC6`\x04aA\xD5V[`\0\x90\x81R`\xA8` R`@\x90 T`\x0F\x0B\x90V[a\x05\xF4a\x05\xEF6`\x04aF\x91V[a\x1E-V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x93\x84\x01Q\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x02\xE3V[a\x03}a\x1E\xB6V[a\x03}a#<V[a\x03}a\x0646`\x04aGfV[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x03}a\x06u6`\x04aG\x94V[PPV[`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE3V[a\x03}a\x06\xAC6`\x04aH\x06V[a#PV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x06\x86V[a\x039a\x06\xD06`\x04aH\x06V[a#\xD3V[a\x05\x85a\x06\xE36`\x04aH3V[a%MV[a\x05\x85a\x06\xF66`\x04aBKV[a%wV[a\x03}a\x07\t6`\x04aH\x06V[a%\x9EV[a\x03}a\x07\x1C6`\x04aHoV[a%\xEAV[a\x03}a\x07/6`\x04aH\xF4V[a)\x87V[a\x03}a\x07B6`\x04aIPV[`\0\x91\x82R`\x9C` R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03}a\x07~6`\x04aA\xD5V[`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UV[a\x07\xCEa\x07\xC96`\x04aA\xD5V[a*ZV[`@Qa\x02\xE3\x91\x90aIuV[a\x05\x85a\x07\xE96`\x04aBKV[a+dV[a\x03}a\x07\xFC6`\x04aI\xADV[a+\x92V[a\x03}a\x08\x0F6`\x04aA\xD5V[a,\x1FV[`@\x80Q`\x9C\x81R\x7FL}\xA8\xC9\xFEpW+2\x9Dp\x9D\x9E+\xDEpx;r\r\t\xC8B\x18g\xCC\xB7\xB85v\x06_` \x82\x01R\x7F\xA1\x06\x7F\xD9I\x04\xBEg\xF1J\xA0\xB46yq\xBB1L\x8F>\xF4K\xED\xF9\xE7\xF1\xCE\xCE\x82\x1CS\x8D\x91\x81\x01\x91\x90\x91R`\x9B``\x82\x01R`\x80\x01a\x02\xE3V[a\x02\xFFa\x08\x876`\x04aBKV[a,(V[`\0\x80`\0a\x08\x9B\x85\x85a,ZV[` \x81\x01Q\x90Q\x90\x96\x90\x95P\x93PPPPV[`\0\x80[`\n\x81\x10\x15a\x08\xF7W`\0\x84\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x83\x03a\x08\xE5W`\x01\x91PPa\x08\xFDV[\x80a\x08\xEF\x81aI\xE0V[\x91PPa\x08\xB2V[P`\0\x90P[\x92\x91PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\t\x97a\t\x84\x83\x80aI\xF9V[a\t\x92\x90` \x81\x01\x90aBKV[a,\xBBV[`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x90\x91P`\0a\t\xF2\x83``\x01Qa\x11\x05V[\x90P`\0a\n\0\x85\x80aI\xF9V[a\n\x0E\x90` \x81\x01\x90aJ\x19V[a\n\x17\x90aJ/V[\x90P`\0a\n%\x86\x80aI\xF9V[a\n3\x90`@\x81\x01\x90aJ\x19V[a\n<\x90aJ/V[\x90P`@Q\x80a\x01 \x01`@R\x80a\n\\\x87``\x01Q\x85`\0\x01Qa#\xD3V[\x81R` \x01a\ns\x87``\x01Q\x84`\0\x01Qa#\xD3V[\x81R\x83QQ` \x80\x83\x01\x91\x90\x91R\x83QQ`@\x80\x84\x01\x91\x90\x91R\x84Q\x81\x01Q`\x0F\x0B``\x84\x01R`\0`\x80\x84\x01\x81\x90R`\xA0\x84\x01\x81\x90R`\xC0\x84\x01\x81\x90R`\xE0\x90\x93\x01\x83\x90R\x83Q\x83R`\xA7\x90\x91R\x90 T\x90\x94P\x15a\n\xE2W\x83Q`\0\x90\x81R`\xA7` R`@\x90 T\x82QR[` \x80\x85\x01Q`\0\x90\x81R`\xA7\x90\x91R`@\x90 T\x15a\x0B\x15W` \x80\x85\x01Q`\0\x90\x81R`\xA7\x90\x91R`@\x90 T\x81QR[\x81Q`@\x90\x81\x01Q`\x0F\x0B`\xA0\x86\x01R\x84Qa\x0BI\x91\x87\x91\x86\x91\x86\x91\x90`\x01\x90a\x0BD\x90\x8D\x01` \x8E\x01aI\xADV[a-\xDDV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0B\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0B\xA3\x85\x84\x83\x87` \x01Q`\0\x8B`@\x01` \x81\x01\x90a\x0BD\x91\x90aI\xADV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a\x0B\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0B\xED`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x15a\x0E\x1CW\x81Q`\xA0\x01Q`\x02`\x0C\x91\x90\x91\x1C`\x03\x16\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0CAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P\x81Q`@\x01Q`\x0F\x0B`\0\x90\x81\x12\x90a\x0Ca`\x80\x89\x01``\x8A\x01aJ\xF7V[`\x0F\x0B\x13\x15\x15\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0C\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\rHWa\x0C\xC6`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x82`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15\x80\x15a\r\tWPa\x0C\xEE`\x80\x87\x01``\x88\x01aJ\xF7V[a\x0C\xF7\x90aK\x14V[`\x0F\x0B\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\rBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\r\xD6V[a\rX`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x82`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15\x80\x15a\r\x9BWPa\r\x80`\x80\x87\x01``\x88\x01aJ\xF7V[a\r\x89\x90aK\x14V[`\x0F\x0B\x81`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\r\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P[a\r\xE6`\x80\x87\x01``\x88\x01aJ\xF7V[\x82Q`\x0F\x91\x90\x91\x0B`@\x90\x91\x01Ra\x0E\x04`\x80\x87\x01``\x88\x01aJ\xF7V[a\x0E\r\x90aK\x14V[\x81Q`\x0F\x91\x90\x91\x0B`@\x90\x91\x01R[\x81Q`@\x90\x81\x01Q\x82Q\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x04\x83RcOCBM`\xE0\x1B` \x84\x01R`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x03a\x0EnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\x0E\xDEW\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x0E\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0F5V[\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x0F3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P[a\x0FJ\x85\x84\x84`\0\x01Q\x84`\0\x01Q\x88a0\xA3V[``\x85\x01Q\x82QQa\x01\0\x86\x01Q\x86Q`\0\x90\x81R`\x9C` \x90\x81R`@\x90\x91 T\x85Q\x90\x91\x01Qa\x0F\x9C\x94\x93\x92\x88\x92\x90\x91a\x0F\x8C\x91`\x0F\x91\x82\x0B\x91\x0Ba2\xA8V[a\x0F\x95\x90aK\x14V[`\x01a3#V[`\x0F\x90\x81\x0Ba\x01\0\x87\x01\x81\x90R\x91\x90\x0B`\xC0\x86\x01R\x83Q\x83QQ`\xE0\x87\x01Qa\x0F\xC9\x93\x89\x93\x92\x91\x90a3kV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIM`\xF0\x1B` \x82\x01RP``\x80\x84\x01Q\x90\x86\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U\x81QQ`\x01\x14a\x10\x90W`\xE0\x84\x01Q\x84Q`\0\x90\x81R`\x9C` R`@\x81 \x80T\x90\x91\x90a\x10h\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x80QQ`\x01\x14a\x10\xEEW`\xE0\x84\x01Q` \x80\x86\x01Q`\0\x90\x81R`\x9C\x90\x91R`@\x81 \x80T\x90\x91\x90a\x10\xC6\x90\x84\x90`\x0F\x0BaK\x89V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[a\x10\xFD\x85``\x01Q\x83\x86a5WV[PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x86R`\x9B\x85R\x87\x86 \x88Q\x94\x85\x01\x89RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x81\x0B\x86\x88\x01R`\x01`\x80\x1B\x90\x91\x04`\x0F\x90\x81\x0B\x86\x8B\x01\x90\x81R\x92\x88R`\xA3\x90\x96R\x97\x90\x95 T\x16\x85R\x92Q\x90\x91\x0B\x90\x91R\x80Q\x91\x92\x90\x91a\x11\x98\x91\x0Bc;\x9A\xCA\0aK\xD9V[`\x0F\x0B` \x80\x84\x01\x91\x90\x91R\x81\x01Qa\x11\xB8\x90`\x07\x0Bc;\x9A\xCA\0aK\xD9V[`\x0F\x0B`@\x83\x01RP\x91\x90PV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE4Wa\x11\xE4aBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\rW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\xA4W`\xA0`\0\x85\x83\x81Q\x81\x10a\x122Wa\x122aLwV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x82\x82\x81Q\x81\x10a\x12}Wa\x12}aLwV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x12\x9C\x81aI\xE0V[\x91PPa\x12\x13V[P\x92\x91PPV[a\x12\xB5\x83\x82a6JV[a\x12\xBF\x83\x83a6\x95V[a\x12\xC9\x81\x83aK:V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x10\x90a\x12\xF8\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`eT`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[\x82QQb\xFF\xFF\xFF\x16biso\x14\x15`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x13\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P\x82Q`\xA0\x01Q`\x08\x1C`\x01\x90\x81\x16\x14`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x14\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0a\x147\x84` \x01Q\x85`\0\x01Qa#\xD3V[`\0\x81\x81R`\xA7` R`@\x90 T\x90\x91P\x15a\x14dW`\0\x90\x81R`\xA7` R`@\x90 T\x90Pa\x08\xFDV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIS`\xF0\x1B` \x82\x01RP\x83QQ``\x1C`\0\x81\x81R`\xA4` R`@\x81 T\x90\x80[\x82\x81`\x01\x90\x1B\x11a\x15\x11W`\x01\x81\x1B\x83\x16\x15a\x14\xFFW\x87QQ`\0\x90\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a\x14\xFDW`\0a\x14\xDA\x82a6\xA2V[\x90P\x89` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x03a\x14\xFBWP\x91Pa\x15\x11V[P[P[a\x15\n`\x01\x82aL\x8DV[\x90Pa\x14\x96V[P\x80a\x16\xA8W\x86Q`\xA0\x01Q`\x0B\x1C`\x01\x90\x81\x16\x03a\x15\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FReduce-only order cannot create `D\x82\x01R\x7Fisolated subaccount\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[a\x15\xA5`\x01a\x04\0aL\xA5V[\x82\x03a\x15\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FToo many isolated subaccounts\0\0\0`D\x82\x01R`d\x01a\tlV[`\0[`\x01\x83\x16\x15a\x16\x16W`\x01\x92\x83\x1C\x92a\x16\x0F\x90\x82aL\xBCV[\x90Pa\x15\xF6V[` \x88\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xA4\x83R`@\x80\x82 \x80T`\x01`\xFF\x88\x16\x90\x81\x1B\x90\x91\x17\x90\x91U\x8CQQ`\x18\x96\x90\x96\x1Bc\xFF\0\0\0\x16\x93\x85\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16``\x8A\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x93\x90\x93\x17biso\x17\x80\x83R`\xA5\x85R\x81\x83 \x95\x90\x95U\x8BQQ\x82R`\xA6\x84R\x80\x82 \x92\x82R\x91\x90\x92R\x90 \x81\x90U\x90P[`\0\x84\x81R`\xA7` R`@\x81 \x82\x90U\x87Q`\xA0\x01Qa\x16\xC8\x90a6\xC6V[\x90P`\0\x81`\x0F\x0B\x13\x15a\x17\xF5W`\0\x85\x81R`\xA8` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x84\x16\x17\x90U`\x9DT\x89QQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE0\xB0b\x1F\x91a\x17\x1E\x85aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x81W=`\0\x80>=`\0\xFD[PP`\x9DT`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x85\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xE0\xB0b\x1F\x91P`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xF0W=`\0\x80>=`\0\xFD[PPPP[P\x96\x95PPPPPPV[`\0\x80\x84\x15a\x18RW`\0\x85\x81R`\x9C` R`@\x81 \x80T\x85\x92\x90a\x18*\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83\x15a\x18\xA1W`\0\x84\x81R`\x9C` R`@\x81 \x80T\x85\x92\x90a\x18y\x90\x84\x90`\x0F\x0BaK\x89V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPP`\0\x91\x82R`\x9C` R`@\x80\x83 T\x91\x83R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[`\x9AT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pc\xFF\xFF\xFF\xFF\x81\x16\x15\x80\x15\x90a\x199WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xA1` R`@\x90 T`\xFF\x16\x15[\x15a\x19\xA4W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xA1` R`@\x81 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\xA2\x80T\x91\x82\x01\x81U\x90\x91R\x7F\xAA\xF4\xF5\x8D\xE9\x93\0\xCF\xAD\xC4XWU\xF3v\xD5\xFAt}[\xC5a\xD5\xBD\x9Dq\r\xE1\xF9\x1B\xF4-\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xD9\xE1\x9A\xDB|\xA8\x8F\xC3\xB0\xA9\xBE\xD1\xEF\xAAa\xB7\x7F\xEA\xAAs)\xB58\xE4\x8A\xA5\xFA\xA1g\xF1\xE2\xCB\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[```\0a\x1A\x18\x83\x85aL\xE1V[`\xA2T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x83\x16\x11\x15a\x1A3W\x80\x91P[\x80c\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x1AKW\x80\x94P[`\0a\x1AW\x86\x84aM\tV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AuWa\x1AuaBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x9EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x85[\x83c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B0W`\xA2\x81c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xD0Wa\x1A\xD0aLwV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x1A\xF0\x89\x84aM\tV[c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x06Wa\x1B\x06aLwV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B(\x81aM.V[\x91PPa\x1A\xA3V[P\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1BZWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1BtWP0;\x15\x80\x15a\x1BtWP`\0T`\xFF\x16`\x01\x14[a\x1B\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C\tW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C\x11a6\xE5V[a\x1C\x1A\x82a7XV[a\x1Cu`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cNado`\xE0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7\x82V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1C\xB5\x90`\0\x90`\x04\x01aMQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xF6\x91\x90aMyV[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1D:\x90`\x01\x90`\x04\x01aMQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D{\x91\x90aMyV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x1D\xE1W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80`\0a\x1D\xF5\x86\x86a#\xD3V[\x90P`\0a\x1E\x03\x87\x86a#\xD3V[`\0\x92\x83R`\x9C` R`@\x80\x84 T\x91\x84R\x90\x92 T`\x0F\x92\x83\x0B\x98\x92\x0B\x96P\x90\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1EoWP`@\x80Q\x80\x82\x01\x90\x91R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01Ra\x08\xFDV[Pc\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x82R\x82\x90 \x82Q\x80\x84\x01\x90\x93RT`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x90\x91\x04\x90\x0B\x90\x82\x01R\x90V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[`\x9DT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x94\x91\x90\x81\x01\x90aM\x96V[\x90P`\x01[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a!.W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1F\xC2Wa\x1F\xC2aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a $WPPa!\x1CV[`\x9DTc\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\xA3` R`@\x90\x81\x90 T\x84\x82\x01Q\x91Qc\xE0\xB0b\x1F`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R`\x01`$\x83\x01R`\x0F\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x95W`\0\x80\xFD[PZ\xF1\x15\x80\x15a \xA9W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a!&\x81aM.V[\x91PPa\x1F\x99V[P`\x9E`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAA\x91\x90\x81\x01\x90aM\x96V[\x90P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06uW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\xD8Wa!\xD8aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a\":WPPa#*V[`\x9ET`@\x82\x81\x01Q\x90Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`$\x82\x01R`\0`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xB7W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a#4\x81aM.V[\x91PPa!\xAFV[a#Da7\xF7V[a#N`\0a8QV[V[`\0a#\\\x83\x83a#\xD3V[`@\x80\x84\x01Q`\0\x83\x81R`\x9C` R\x91\x90\x91 T\x91\x92P`\x0F\x91\x82\x0B\x91\x0B\x14\x80a#\x8FWPa#\x8F\x82``\x01Qa8\xA3V[\x15a\x1D\xE1W`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UPPV[`\0\x80`@Q\x80`\xA0\x01`@R\x80`c\x81R` \x01aO\xC6`c\x919\x90P`\0\x81\x80Q\x90` \x01 \x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q`@Q` \x01a$s\x97\x96\x95\x94\x93\x92\x91\x90\x96\x87R` \x87\x01\x95\x90\x95R`\x0F\x93\x84\x0B`@\x87\x01R\x91\x90\x92\x0B``\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa$\xB7`fT\x90V[`gT`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01Rc\xFF\xFF\xFF\xFF\x87\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa%C\x81\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x96\x95PPPPPPV[`\0\x80a%Z\x85\x85a,ZV[\x90P\x82a%hW\x80Qa%nV[\x80` \x01Q[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x08\xFD\x90`\x07\x0Bc;\x9A\xCA\0aK\xD9V[`\0a%\xAA\x83\x83a#\xD3V[`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a&WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16a)\x1EW`\x9DT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\xD7\x91\x90\x81\x01\x90aM\x96V[\x90P`\0`\x9E`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'V\x91\x90\x81\x01\x90aM\x96V[\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(RW`\0c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\x8AWa'\x8AaLwV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x03\x15a(@W`@Q\x80`@\x01`@R\x80\x85`@\x01Q`\x0F\x0B\x81R` \x01\x85``\x01Q`\x0F\x0B\x81RP`\x9F`\0\x86`\0\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xFEWa'\xFEaLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x90\x91\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U[\x80a(J\x81aM.V[\x91PPa'[V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x18W`@Q\x80`@\x01`@R\x80\x85`@\x01Q`\x0F\x0B\x81R` \x01\x85``\x01Q`\x0F\x0B\x81RP`\x9F`\0\x86`\0\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a(\xC5Wa(\xC5aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x90\x91\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U\x80a)\x10\x81aM.V[\x91PPa(VV[PPPPV[`@\x80Q\x80\x82\x01\x82R\x82\x82\x01Q`\x0F\x90\x81\x0B\x82R``\x84\x01Q\x90\x0B` \x80\x83\x01\x91\x82R\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\x9F\x83R\x85\x81 \x83\x88\x01Q\x90\x92\x16\x81R\x91R\x92\x90\x92 \x90Q\x91Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U[PV[c\xFF\xFF\xFF\xFF\x83\x81\x16\x14a)\xBDWc\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x90\x81R`\xA3` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x85\x16\x91\x90\x91\x17\x90U[a)\xCBc;\x9A\xCA\0\x82aN;V[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\x0Ec;\x9A\xCA\0\x83aN;V[c\xFF\xFF\xFF\xFF\x90\x94\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x90\x95\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x95\x16\x94\x90\x94\x17\x90\x93UPPPV[```\0\x80[`\n\x81\x10\x15a*\xAAW`\0\x84\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a*\x97Wa*\x94`\x01\x84aL\x8DV[\x92P[P\x80a*\xA2\x81aI\xE0V[\x91PPa*`V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xC6Wa*\xC6aBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\n\x81\x10\x15a+\\W`\0\x85\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a+IW\x80\x83a+)\x86aN\x82V[\x95P\x85\x81Q\x81\x10a+<Wa+<aLwV[` \x02` \x01\x01\x81\x81RPP[P\x80a+T\x81aI\xE0V[\x91PPa*\xF5V[P\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x08\xFD\x90`\x01`@\x1B\x90\x04`\x07\x0Bc;\x9A\xCA\0aK\xD9V[a+\x9Aa7\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[a)\x84\x81a8QV[a)\x84\x81a8\xCAV[`\0\x80a,:a,7\x84a<\xBAV[\x90V[T`\x0F\x0B\x91Pa,La,7\x84a=\x16V[T\x91\x93`\x0F\x92\x90\x92\x0B\x92PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rb\xFF\xFF\xFF\x83\x16biso\x03a,\x8FW`\0\x92\x83R`\xA5` R`@\x90\x92 T\x91[``\x83\x90\x1C`\0\x90\x81R`\xA0` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16a,\xB3\x81\x84a\x1E-V[\x94\x93PPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-R\x91\x90aMyV[`\x9ET\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a-\xA3WP`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R`\0` \x83\x01R`\x01\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x9DT`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[\x83Q`\xA0\x01Q`\0\x90`\xFF\x16`\x01\x14a-\xF8WP`\0a%CV[\x84QQ`\0\x19\x01a.\x0BWP`\x01a%CV[\x84Q\x83\x15a.4W`\xA0\x81\x01Q`\t\x1C`\x03\x90\x81\x16\x03a./W`\0\x91PPa%CV[a.PV[a.A\x81`\xA0\x01Qa=YV[\x15a.PW`\0\x91PPa%CV[`\0\x85\x81R`\x9C` R`@\x90\x81\x90 T\x90\x82\x01\x80Q`\x0F\x92\x90\x92\x0B\x91\x82\x91\x90a.{\x90\x83\x90aK\x89V[`\x0F\x0B\x90RP`\xA0\x82\x01Q`\x0B\x1C`\x01\x90\x81\x16\x03a04W`\0\x89`@\x01Qa/%W` \x8A\x01Q``\x8B\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x1F\x91\x90aN\x99V[Qa/\xA5V[\x89Q``\x8B\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA3\x91\x90aN\xE5V[Q[\x90P`\0\x81`\x0F\x0B\x13\x15\x15`\0\x84`@\x01Q`\x0F\x0B\x13\x15\x15\x03a/\xCEW`\0`@\x84\x01Ra02V[`\0\x83`@\x01Q`\x0F\x0B\x13\x15a0\x02Wa/\xF5\x83`@\x01Q\x82a/\xF0\x90aK\x14V[a=\x82V[`\x0F\x0B`@\x84\x01Ra02V[`\0\x83`@\x01Q`\x0F\x0B\x12\x15a02Wa0)\x83`@\x01Q\x82a0$\x90aK\x14V[a=\x9EV[`\x0F\x0B`@\x84\x01R[P[`\0\x82` \x01Q`\x0F\x0B\x13\x80a0XWP`\xA0\x82\x01Q`\x02`\x0C\x91\x90\x91\x1C`\x03\x16\x10\x15[\x80\x15a0nWP\x86QQ`\x02\x14\x80a0nWP`\x01[\x80\x15a0\x80WP`@\x82\x01Q`\x0F\x0B\x15\x15[\x80\x15a0\x96WPa0\x94\x82``\x01Qa8\xA3V[\x15[\x99\x98PPPPPPPPPV[`\0\x80`\0\x85`@\x01Q`\x0F\x0B\x12\x15a0\xD3Wa0\xCC\x85`@\x01Q\x85`@\x01Qa0$\x90aK\x14V[\x91Pa1\0V[`\0\x85`@\x01Q`\x0F\x0B\x13\x15a0\xF9Wa0\xCC\x85`@\x01Q\x85`@\x01Qa/\xF0\x90aK\x14V[PPa2\xA1V[`@\x86\x01Qa1\x0F\x90\x83aOWV[a1\x19\x90\x83aK\x89V[\x91P`\0a17\x85` \x01Q\x84`\x0F\x0Ba2\xA8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa1B\x81aK\x14V[\x91P`\0a1]\x89``\x01Q\x87`\0\x01Q\x8A\x85`\0\x80a3#V[\x80\x93P\x81\x92PPP\x83\x87`@\x01\x81\x81Qa1w\x91\x90aK\x89V[`\x0F\x0B\x90RP`@\x86\x01\x80Q\x85\x91\x90a1\x91\x90\x83\x90aK:V[`\x0F\x0B\x90RP\x87Q\x86Qa1\xB0\x91\x8B\x91a1\xAA\x88aK\x14V[\x86a3kV[`\x0F\x84\x81\x0B`\xE0\x87\x01R\x83\x90\x0Ba\x01\0\x86\x01R\x85Q` \x80\x87\x01Q``\x8C\x81\x01Q\x92\x8A\x01Q`\x80\x80\x8B\x01Q\x92\x8C\x01Q\x90\x8C\x01Q`\xA0\x8D\x01Q\x94\x95c\xFF\xFF\xFF\xFF\x16\x94\x7F\xB5c\xBD7\"b\x0Ez\xF6\xC3\xDA\xE1\t\x89|\xA2\xF4_\xBB\xC5\x97_\xB6U;\xB2\xD5;w\xE5K\xF3\x94\x92\x91\x90`\x01`\x08\x82\x90\x1C\x81\x16\x14`\0\x8B\x8Fa2-\x90aK\x14V[`@\x80Q`\x0F\x9A\x8B\x0B\x81R\x98\x8A\x0B` \x8A\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16\x90\x89\x01R\x95\x90\x94\x16``\x87\x01R`\x01`\x01`\x80\x1B\x03\x90\x92\x16`\x80\x86\x01R\x15\x15`\xA0\x85\x01R\x15\x15`\xC0\x84\x01R\x83\x0B`\xE0\x83\x01R\x82\x0Ba\x01\0\x82\x01R\x90\x87\x90\x0Ba\x01 \x82\x01Ra\x01@\x01`@Q\x80\x91\x03\x90\xA4PPPP[PPPPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a2\xEAWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a+\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[`\0\x80`\0\x80a37\x8A\x8A\x8A\x8A\x8A\x8Aa=\xB3V[\x91P\x91P\x84\x15a3PWa3K\x8A\x83a6JV[a3ZV[a3Z\x8A\x83a6\x95V[\x90\x92P\x90P[\x96P\x96\x94PPPPPV[\x84`@\x01Q\x15a3\xFCW\x84Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a3\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a3\xF3W=`\0\x80>=`\0\xFD[PPPPa2\xA1V[c\xFF\xFF\xFF\xFF\x84\x16a4^W` \x85\x01Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01a3\xC5V[` \x85\x01Q``\x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xC2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xD6W=`\0\x80>=`\0\xFD[PPP` \x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a58W`\0\x80\xFD[PZ\xF1\x15\x80\x15a5LW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x81\x01Q\x81Q\x83Q` \x81\x01Q`\xA0\x80\x86\x01Q``\x84\x01Q`\x80\x85\x01Q\x92\x90\x94\x01Qc\xFF\xFF\xFF\xFF\x8A\x16\x94\x7F\xB5c\xBD7\"b\x0Ez\xF6\xC3\xDA\xE1\t\x89|\xA2\xF4_\xBB\xC5\x97_\xB6U;\xB2\xD5;w\xE5K\xF3\x94\x93\x90\x91`\x01`\x08\x82\x90\x1C\x81\x16\x14`\x01\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8Da\x01\0\x01Q`@Qa6=\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90`\x0F\x9A\x8B\x0B\x81R\x98\x8A\x0B` \x8A\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8A\x01R\x95\x90\x96\x16``\x88\x01R`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16`\x80\x87\x01R\x90\x15\x15`\xA0\x86\x01R\x15\x15`\xC0\x85\x01R\x84\x0B`\xE0\x84\x01R\x90\x83\x0Ba\x01\0\x83\x01R\x90\x91\x0Ba\x01 \x82\x01Ra\x01@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[\x80a6Wa,7\x84a=\x16V[\x80T`\0\x90a6j\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPV[\x80a6Wa,7\x84a<\xBAV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a6\xBBWP`\0\x91\x90PV[P` \x1Ca\xFF\xFF\x16\x90V[`\0a\x08\xFDg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x90\x1C\x16d\xE8\xD4\xA5\x10\0aOyV[`\0Ta\x01\0\x90\x04`\xFF\x16a7PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a#Na?\x16V[a7`a7\xF7V[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a7\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a\x06u\x82\x82a?\x8AV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a#NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\tlV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0a8\xADa@\x0FV[`\x01`\x01`\x80\x1B\x03\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x90P\x91\x90PV[`\0a8\xD5\x82a6\xA2V[\x90P\x80c\xFF\xFF\xFF\xFF\x16`\0\x03a8\xE9WPPV[`\x9ET`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9c\x91\x90aN\xE5V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x1D\xE1W`\0a9}\x84a@\x82V[`\0\x85\x81R`\xA5` \x90\x81R`@\x90\x91 T\x90\x84\x01Q\x91\x92P``\x86\x90\x1C\x91`\x0F\x0B\x15a:\xBFW`\x9ET` \x85\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90\x87\x90\x89\x90`\0\x90a9\xCE\x90aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x92\x90\x92R`\x0F\x90\x81\x0B`D\x84\x01R\x0B`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a:9W=`\0\x80>=`\0\xFD[PP`\x9ET` \x87\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x8A\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\0`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xF8\xA4.Q\x91P`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a:\xBAW=`\0\x80>=`\0\xFD[PPPP[`\x9DT`@Qc|\x1E\x14\x87`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x89\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c|\x1E\x14\x87\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;4\x91\x90aN\x99V[Q\x90P`\x0F\x81\x90\x0B\x15a<5W`\x9DT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F`\0\x89a;^\x85aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\xADW`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xC1W=`\0\x80>=`\0\xFD[PP`\x9DT`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x85\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xE0\xB0b\x1F\x91P`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<0W=`\0\x80>=`\0\xFD[PPPP[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xA4` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x8A\x16\x90\x81\x1B`\0\x19\x18\x90\x91\x16\x90\x91U\x85\x84R`\xA6\x83R\x81\x84 \x90\x84R\x82R\x80\x83 \x83\x90U\x89\x83R`\xA5\x90\x91R\x80\x82 \x82\x90UQ\x83\x91\x89\x91\x7F\x02|\xE6\xFB\xFB[M\x17\xA0\xEE6\xB5\x92\xEF\r\xB7{\x06\r\xF7\xE0#\xBC\x84\xE6\x8C_\x06d\xC9\xB8:\x91\x90\xA3PPPPPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x7F\xA1\x06\x7F\xD9I\x04\xBEg\xF1J\xA0\xB46yq\xBB1L\x8F>\xF4K\xED\xF9\xE7\xF1\xCE\xCE\x82\x1CS\x8D\x91\x81\x01\x91\x90\x91R`\0\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x7FL}\xA8\xC9\xFEpW+2\x9Dp\x9D\x9E+\xDEpx;r\r\t\xC8B\x18g\xCC\xB7\xB85v\x06_\x91\x81\x01\x91\x90\x91R`\0\x90``\x01a<\xF9V[`\0`\x03`\t\x83\x90\x1C\x16`\x01\x81\x14\x80a={WP\x80`\x01`\x01`\x80\x1B\x03\x16`\x02\x14[\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a=\x97W\x81a={V[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a=\x97W\x81a={V[`\0\x80`\0\x19\x87\x01a=\xCAWP`\0\x90P\x83a3`V[`\0\x83\x15a>uW\x84`\x0F\x0B`\0\x03a>\x06W` \x87\x01Qa=\xEC\x90\x82aK:V[\x90P`\0\x86`\x0F\x0B\x12\x15a>\x06Wa>\x03\x81aK\x14V[\x90P[` \x87\x01Q`\0\x90a> a>\x1B\x89\x89aK:V[a@\xA5V[a>*\x91\x90aK\x89V[\x90Pa><\x81a/\xF0\x89`\x0F\x0Ba@\xC0V[\x90P`\0\x81`\x0F\x0B\x13\x15a>oW`\0\x87`\x0F\x0B\x12\x15a>bWa>_\x81aK\x14V[\x90P[a>l\x81\x83aK:V[\x91P[Pa>\x82V[a>\x7F\x86\x82aK:V[\x90P[`\0a>\x8F\x89\x8B\x87a%MV[a>\xA1\x90g\r\xE0\xB6\xB3\xA7d\0\0aK\x89V[\x90P`\0\x80\x83`\x0F\x0B\x13a>\xC2Wa>\xBD`\x0F\x84\x90\x0B\x83aA*V[a>\xD0V[a>\xD0`\x0F\x84\x90\x0B\x83a2\xA8V[\x90P`\0a>\xDE\x82\x85aK\x89V[\x90P\x80\x8A``\x01\x81\x81Qa>\xF2\x91\x90aK:V[`\x0F\x0B\x90RP\x80a?\x03\x81\x8BaK\x89V[\x95P\x95PPPPP\x96P\x96\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a?\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a#N3a8QV[`\0Ta\x01\0\x90\x04`\xFF\x16a?\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `f\x91\x90\x91U`gUV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a@YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@}\x91\x90aO\xA8V[\x90P\x90V[`\0b\xFF\xFF\xFF\x82\x16biso\x14a@\x9BWP`\0\x91\x90PV[P`\x18\x1C`\xFF\x16\x90V[`\0\x80\x82`\x0F\x0B\x12a@\xB7W\x81a\x08\xFDV[a\x08\xFD\x82aK\x14V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03aA\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\x0F\x0B\x12aA#W\x81a\x08\xFDV[P`\0\x03\x90V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90aAnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a2\xBFWa2\xBFaN%V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a)\x84W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aA\xB8W`\0\x80\xFD[\x825\x91P` \x83\x015aA\xCA\x81aA\x93V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aA\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aB\x01W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15aB\"W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB9W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a={W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB]W`\0\x80\xFD[\x815a={\x81aA\x93V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xA7WaB\xA7aBhV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xC9WaB\xC9aBhV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)\x84W`\0\x80\xFD[\x805aB\xF3\x81aB\xD3V[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15aC\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\"W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aC3W`\0\x80\xFD[\x805aCFaCA\x82aB\xAFV[aB~V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aCeW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aC\x8CW\x835aC}\x81aB\xD3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aCjV[\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aC\xB3V[P\x90\x96\x95PPPPPPV[\x80`\x0F\x0B\x81\x14a)\x84W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aD\x05W`\0\x80\xFD[\x835aD\x10\x81aA\x93V[\x92P` \x84\x015aD \x81aC\xE1V[\x91P`@\x84\x015aD0\x81aC\xE1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aDNW`\0\x80\xFD[\x825aDY\x81aB\xD3V[\x91P` \x83\x015aA\xCA\x81aA\x93V[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aB\xF3W`\0\x80\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a)\x84W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15aD\xA8W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD\xCBWaD\xCBaBhV[`@R\x825\x81R\x90P\x80` \x83\x015aD\xE3\x81aC\xE1V[` \x82\x01R`@\x83\x015aD\xF6\x81aC\xE1V[`@\x82\x01RaE\x07``\x84\x01aDiV[``\x82\x01RaE\x18`\x80\x84\x01aDiV[`\x80\x82\x01R`\xA0\x83\x015aE+\x81aD\x81V[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aEIW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEcWaEcaBhV[aEv`\x1F\x82\x01`\x1F\x19\x16` \x01aB~V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\x8BW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aE\xBBW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aE\xD3W`\0\x80\xFD[\x90\x84\x01\x90a\x01\0\x82\x87\x03\x12\x15aE\xE8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aF\x03WaF\x03aBhV[`@RaF\x10\x87\x84aD\x96V[\x81R`\xC0\x83\x015aF \x81aA\x93V[` \x82\x01R`\xE0\x83\x015\x82\x81\x11\x15aF7W`\0\x80\xFD[aFC\x88\x82\x86\x01aE8V[`@\x83\x01RP\x93PaFZ\x91PP` \x84\x01aB\xE8V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aFxW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015aD0\x81aC\xE1V[`\0\x80`@\x83\x85\x03\x12\x15aF\xA4W`\0\x80\xFD[\x825aDY\x81aA\x93V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\xCBV[`\0\x80`@\x83\x85\x03\x12\x15aG\x03W`\0\x80\xFD[\x825aG\x0E\x81aB\xD3V[\x91P` \x83\x015aA\xCA\x81aB\xD3V[`\0\x80`\0a\x01\xA0\x84\x86\x03\x12\x15aG4W`\0\x80\xFD[\x835aG?\x81aA\x93V[\x92PaGN\x85` \x86\x01aD\x96V[\x91PaG]\x85`\xE0\x86\x01aD\x96V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aGyW`\0\x80\xFD[\x825aG\x84\x81aA\x93V[\x91P` \x83\x015aA\xCA\x81aC\xE1V[`\0\x80` \x83\x85\x03\x12\x15aG\xA7W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aG\xBFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aG\xD3W`\0\x80\xFD[\x815\x81\x81\x11\x15aG\xE2W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15aG\xF4W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\xE0\x83\x85\x03\x12\x15aH\x19W`\0\x80\xFD[\x825aH$\x81aA\x93V[\x91PaFZ\x84` \x85\x01aD\x96V[`\0\x80`\0``\x84\x86\x03\x12\x15aHHW`\0\x80\xFD[\x835\x92P` \x84\x015aHZ\x81aA\x93V[\x91P`@\x84\x015\x80\x15\x15\x81\x14aD0W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aH\x81W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aH\xA4WaH\xA4aBhV[`@R\x825aH\xB2\x81aA\x93V[\x81R` \x83\x015aH\xC2\x81aA\x93V[` \x82\x01R`@\x83\x015aH\xD5\x81aC\xE1V[`@\x82\x01R``\x83\x015aH\xE8\x81aC\xE1V[``\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aI\nW`\0\x80\xFD[\x845aI\x15\x81aA\x93V[\x93P` \x85\x015aI%\x81aA\x93V[\x92P`@\x85\x015aI5\x81aC\xE1V[\x91P``\x85\x015aIE\x81aC\xE1V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15aIcW`\0\x80\xFD[\x825\x91P` \x83\x015aA\xCA\x81aC\xE1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aI\x91V[`\0` \x82\x84\x03\x12\x15aI\xBFW`\0\x80\xFD[\x815a={\x81aB\xD3V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aI\xF2WaI\xF2aI\xCAV[P`\x01\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aJ\x0FW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aJ\x0FW`\0\x80\xFD[`\0`\xE0\x826\x03\x12\x15aJAW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aJeWaJeaBhV[\x81`@RaJs6\x86aD\x96V[\x83R`\xC0\x85\x015\x91P\x80\x82\x11\x15aJ\x89W`\0\x80\xFD[PaJ\x966\x82\x86\x01aE8V[` \x83\x01RP\x92\x91PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aJ\xCFW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aJ\xB3V[\x81\x81\x11\x15aJ\xE1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK\tW`\0\x80\xFD[\x815a={\x81aC\xE1V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aK1WaK1aI\xCAV[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aKdWaKdaI\xCAV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aK\x80WaK\x80aI\xCAV[P\x01\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aK\xB4WaK\xB4aI\xCAV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aK\xCFWaK\xCFaI\xCAV[P\x90\x03\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aL\tWaL\taI\xCAV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aL5WaL5aI\xCAV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aLQWaLQaI\xCAV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aLgWaLgaI\xCAV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aL\xA0WaL\xA0aI\xCAV[P\x01\x90V[`\0\x82\x82\x10\x15aL\xB7WaL\xB7aI\xCAV[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15aL\xD9WaL\xD9aI\xCAV[\x01\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aM\0WaM\0aI\xCAV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aM&WaM&aI\xCAV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aMGWaMGaI\xCAV[`\x01\x01\x93\x92PPPV[` \x81\x01`\x02\x83\x10aMsWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aM\x8BW`\0\x80\xFD[\x81Qa={\x81aB\xD3V[`\0` \x80\x83\x85\x03\x12\x15aM\xA9W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\xC0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aM\xD1W`\0\x80\xFD[\x80QaM\xDFaCA\x82aB\xAFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aM\xFEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aC\x8CW\x83QaN\x16\x81aA\x93V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aN\x03V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80aNRWaNRaN%V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15aNyWaNyaI\xCAV[\x90\x05\x93\x92PPPV[`\0\x81aN\x91WaN\x91aI\xCAV[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aN\xABW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aN\xCEWaN\xCEaBhV[`@R\x82QaN\xDC\x81aC\xE1V[\x81R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15aN\xF7W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aO\x1AWaO\x1AaBhV[`@R\x82QaO(\x81aC\xE1V[\x81R` \x83\x01QaO8\x81aC\xE1V[` \x82\x01R`@\x83\x01QaOK\x81aC\xE1V[`@\x82\x01R\x93\x92PPPV[`\0\x82`\x0F\x0B\x80aOjWaOjaN%V[\x80\x83`\x0F\x0B\x07\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aO\x9FWaO\x9FaI\xCAV[\x02\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\xBAW`\0\x80\xFD[\x81Qa={\x81aD\x81V\xFEOrder(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce,uint128 appendix)\xA2dipfsX\"\x12 \x08O\xDCM\x924\x97gjM%m,o\xE2\"\xC5\x9ER\x05\xEA\xB0\x98x\xED\x83\x8F[M\xB7b\x1AdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static OFFCHAINEXCHANGE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD3W`\x005`\xE0\x1C\x80cp|\x8BX\x11a\x01\x86W\x80c\xBF)\x17L\x11a\0\xE3W\x80c\xED\xC6\xD3{\x11a\0\x97W\x80c\xF6\xEE{K\x11a\0qW\x80c\xF6\xEE{K\x14a\x08\x01W\x80c\xFA\xB2\xC4i\x14a\x08\x14W\x80c\xFF\x0B\xE9\xEF\x14a\x08yW`\0\x80\xFD[\x80c\xED\xC6\xD3{\x14a\x07\xBBW\x80c\xF2\xB2c1\x14a\x07\xDBW\x80c\xF2\xFD\xE3\x8B\x14a\x07\xEEW`\0\x80\xFD[\x80c\xC8\xD6\xDB\xCB\x11a\0\xC8W\x80c\xC8\xD6\xDB\xCB\x14a\x07!W\x80c\xDE\x10x\xBD\x14a\x074W\x80c\xE1\xE7\x18\x8D\x14a\x07pW`\0\x80\xFD[\x80c\xBF)\x17L\x14a\x06\xFBW\x80c\xC7\x1E\xDE`\x14a\x07\x0EW`\0\x80\xFD[\x80c\x8D\xC3\xF4h\x11a\x01:W\x80c\xB5kjl\x11a\x01\x1FW\x80c\xB5kjl\x14a\x06\xC2W\x80c\xB5\xCB\xD7\x0E\x14a\x06\xD5W\x80c\xB6\n\xAA|\x14a\x06\xE8W`\0\x80\xFD[\x80c\x8D\xC3\xF4h\x14a\x06\x9EW\x80c\xAE\xD8\xE9g\x14a\x06\xB1W`\0\x80\xFD[\x80c\x81&\t\xF1\x11a\x01kW\x80c\x81&\t\xF1\x14a\x06&W\x80c\x84R\x80\x93\x14a\x06gW\x80c\x8D\xA5\xCB[\x14a\x06yW`\0\x80\xFD[\x80cp|\x8BX\x14a\x06\x16W\x80cqP\x18\xA6\x14a\x06\x1EW`\0\x80\xFD[\x80c4\xF9\xA4\xA4\x11a\x024W\x80c@\xF1\xA3M\x11a\x01\xE8W\x80cRi\x99`\x11a\x01\xCDW\x80cRi\x99`\x14a\x05\xABW\x80cj\xC3\xEE\x0B\x14a\x05\xBEW\x80ck\xAA\x1B\x83\x14a\x05\xE1W`\0\x80\xFD[\x80c@\xF1\xA3M\x14a\x05bW\x80cH\\\xC9U\x14a\x05\x98W`\0\x80\xFD[\x80c8]D\x8D\x11a\x02\x19W\x80c8]D\x8D\x14a\x04\x93W\x80c>\xB0\xF4\xB3\x14a\x04\xA6W\x80c?\xCE\xEA(\x14a\x05BW`\0\x80\xFD[\x80c4\xF9\xA4\xA4\x14a\x04mW\x80c5\xEDNm\x14a\x04\x80W`\0\x80\xFD[\x80c\x1D\x02\x9BM\x11a\x02\x8BW\x80c\x1FL\xE0\x16\x11a\x02pW\x80c\x1FL\xE0\x16\x14a\x03\xF9W\x80c*k?\xFE\x14a\x04\x0CW\x80c+\xD2\x86\x17\x14a\x04,W`\0\x80\xFD[\x80c\x1D\x02\x9BM\x14a\x03\x7FW\x80c\x1E\xD0\x1D\xAD\x14a\x03\xD9W`\0\x80\xFD[\x80c\x13\xB5m\xDB\x11a\x02\xBCW\x80c\x13\xB5m\xDB\x14a\x03\x19W\x80c\x1A+-\x16\x14a\x03GW\x80c\x1A\xB0\xA5\x88\x14a\x03jW`\0\x80\xFD[\x80c\x01\xE2\xAB\xD5\x14a\x02\xD8W\x80c\x0F,\x87\x8E\x14a\x02\xECW[`\0\x80\xFD[`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xFFa\x02\xFA6`\x04aA\xA5V[a\x08\x8CV[`@\x80Q`\x0F\x93\x84\x0B\x81R\x91\x90\x92\x0B` \x82\x01R\x01a\x02\xE3V[a\x039a\x03'6`\x04aA\xD5V[`\0\x90\x81R`\xA5` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xE3V[a\x03Za\x03U6`\x04aA\xEEV[a\x08\xAEV[`@Q\x90\x15\x15\x81R` \x01a\x02\xE3V[a\x03}a\x03x6`\x04aB\x10V[a\t\x03V[\0[a\x03\x92a\x03\x8D6`\x04aBKV[a\x11\x05V[`@Qa\x02\xE3\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[a\x03\xECa\x03\xE76`\x04aB\xF8V[a\x11\xC6V[`@Qa\x02\xE3\x91\x90aC\x97V[a\x03}a\x04\x076`\x04aC\xF0V[a\x12\xABV[a\x039a\x04\x1A6`\x04aA\xD5V[`\0\x90\x81R`\xA7` R`@\x90 T\x90V[a\x03}a\x04:6`\x04aD;V[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xA0` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x039a\x04{6`\x04aE\xA8V[a\x13$V[a\x02\xFFa\x04\x8E6`\x04aFcV[a\x18\0V[a\x03}a\x04\xA16`\x04aD;V[a\x18\xC6V[a\x05\x15a\x04\xB46`\x04aBKV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x9B\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RT`\x07\x81\x81\x0B\x85R`\x01`@\x1B\x82\x04\x90\x0B\x91\x84\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x04`\x0F\x0B\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x07\x90\x81\x0B\x82R` \x80\x85\x01Q\x90\x91\x0B\x90\x82\x01R\x91\x81\x01Q`\x0F\x0B\x90\x82\x01R``\x01a\x02\xE3V[a\x05Ua\x05P6`\x04aF\x91V[a\x1A\nV[`@Qa\x02\xE3\x91\x90aF\xAFV[a\x05\x85a\x05p6`\x04aA\xD5V[`\x9C` R`\0\x90\x81R`@\x90 T`\x0F\x0B\x81V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x02\xE3V[a\x03}a\x05\xA66`\x04aF\xF0V[a\x1B:V[a\x02\xFFa\x05\xB96`\x04aG\x1EV[a\x1D\xE6V[a\x05\x85a\x05\xCC6`\x04aA\xD5V[`\0\x90\x81R`\xA8` R`@\x90 T`\x0F\x0B\x90V[a\x05\xF4a\x05\xEF6`\x04aF\x91V[a\x1E-V[`@\x80Q\x82Q`\x0F\x90\x81\x0B\x82R` \x93\x84\x01Q\x90\x0B\x92\x81\x01\x92\x90\x92R\x01a\x02\xE3V[a\x03}a\x1E\xB6V[a\x03}a#<V[a\x03}a\x0646`\x04aGfV[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90UV[a\x03}a\x06u6`\x04aG\x94V[PPV[`3T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE3V[a\x03}a\x06\xAC6`\x04aH\x06V[a#PV[`eT`\x01`\x01`\xA0\x1B\x03\x16a\x06\x86V[a\x039a\x06\xD06`\x04aH\x06V[a#\xD3V[a\x05\x85a\x06\xE36`\x04aH3V[a%MV[a\x05\x85a\x06\xF66`\x04aBKV[a%wV[a\x03}a\x07\t6`\x04aH\x06V[a%\x9EV[a\x03}a\x07\x1C6`\x04aHoV[a%\xEAV[a\x03}a\x07/6`\x04aH\xF4V[a)\x87V[a\x03}a\x07B6`\x04aIPV[`\0\x91\x82R`\x9C` R`@\x90\x91 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03}a\x07~6`\x04aA\xD5V[`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UV[a\x07\xCEa\x07\xC96`\x04aA\xD5V[a*ZV[`@Qa\x02\xE3\x91\x90aIuV[a\x05\x85a\x07\xE96`\x04aBKV[a+dV[a\x03}a\x07\xFC6`\x04aI\xADV[a+\x92V[a\x03}a\x08\x0F6`\x04aA\xD5V[a,\x1FV[`@\x80Q`\x9C\x81R\x7FL}\xA8\xC9\xFEpW+2\x9Dp\x9D\x9E+\xDEpx;r\r\t\xC8B\x18g\xCC\xB7\xB85v\x06_` \x82\x01R\x7F\xA1\x06\x7F\xD9I\x04\xBEg\xF1J\xA0\xB46yq\xBB1L\x8F>\xF4K\xED\xF9\xE7\xF1\xCE\xCE\x82\x1CS\x8D\x91\x81\x01\x91\x90\x91R`\x9B``\x82\x01R`\x80\x01a\x02\xE3V[a\x02\xFFa\x08\x876`\x04aBKV[a,(V[`\0\x80`\0a\x08\x9B\x85\x85a,ZV[` \x81\x01Q\x90Q\x90\x96\x90\x95P\x93PPPPV[`\0\x80[`\n\x81\x10\x15a\x08\xF7W`\0\x84\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x83\x03a\x08\xE5W`\x01\x91PPa\x08\xFDV[\x80a\x08\xEF\x81aI\xE0V[\x91PPa\x08\xB2V[P`\0\x90P[\x92\x91PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\t\x97a\t\x84\x83\x80aI\xF9V[a\t\x92\x90` \x81\x01\x90aBKV[a,\xBBV[`@\x80Qa\x01 \x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x90\x91P`\0a\t\xF2\x83``\x01Qa\x11\x05V[\x90P`\0a\n\0\x85\x80aI\xF9V[a\n\x0E\x90` \x81\x01\x90aJ\x19V[a\n\x17\x90aJ/V[\x90P`\0a\n%\x86\x80aI\xF9V[a\n3\x90`@\x81\x01\x90aJ\x19V[a\n<\x90aJ/V[\x90P`@Q\x80a\x01 \x01`@R\x80a\n\\\x87``\x01Q\x85`\0\x01Qa#\xD3V[\x81R` \x01a\ns\x87``\x01Q\x84`\0\x01Qa#\xD3V[\x81R\x83QQ` \x80\x83\x01\x91\x90\x91R\x83QQ`@\x80\x84\x01\x91\x90\x91R\x84Q\x81\x01Q`\x0F\x0B``\x84\x01R`\0`\x80\x84\x01\x81\x90R`\xA0\x84\x01\x81\x90R`\xC0\x84\x01\x81\x90R`\xE0\x90\x93\x01\x83\x90R\x83Q\x83R`\xA7\x90\x91R\x90 T\x90\x94P\x15a\n\xE2W\x83Q`\0\x90\x81R`\xA7` R`@\x90 T\x82QR[` \x80\x85\x01Q`\0\x90\x81R`\xA7\x90\x91R`@\x90 T\x15a\x0B\x15W` \x80\x85\x01Q`\0\x90\x81R`\xA7\x90\x91R`@\x90 T\x81QR[\x81Q`@\x90\x81\x01Q`\x0F\x0B`\xA0\x86\x01R\x84Qa\x0BI\x91\x87\x91\x86\x91\x86\x91\x90`\x01\x90a\x0BD\x90\x8D\x01` \x8E\x01aI\xADV[a-\xDDV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0B\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0B\xA3\x85\x84\x83\x87` \x01Q`\0\x8B`@\x01` \x81\x01\x90a\x0BD\x91\x90aI\xADV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIM`\xF0\x1B\x81RP\x90a\x0B\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0B\xED`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x15a\x0E\x1CW\x81Q`\xA0\x01Q`\x02`\x0C\x91\x90\x91\x1C`\x03\x16\x10\x15`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0CAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P\x81Q`@\x01Q`\x0F\x0B`\0\x90\x81\x12\x90a\x0Ca`\x80\x89\x01``\x8A\x01aJ\xF7V[`\x0F\x0B\x13\x15\x15\x14`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\x0C\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\rHWa\x0C\xC6`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x82`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15\x80\x15a\r\tWPa\x0C\xEE`\x80\x87\x01``\x88\x01aJ\xF7V[a\x0C\xF7\x90aK\x14V[`\x0F\x0B\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\rBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\r\xD6V[a\rX`\x80\x87\x01``\x88\x01aJ\xF7V[`\x0F\x0B\x82`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15\x80\x15a\r\x9BWPa\r\x80`\x80\x87\x01``\x88\x01aJ\xF7V[a\r\x89\x90aK\x14V[`\x0F\x0B\x81`\0\x01Q`@\x01Q`\x0F\x0B\x12\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x12U`\xF2\x1B\x81RP\x90a\r\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P[a\r\xE6`\x80\x87\x01``\x88\x01aJ\xF7V[\x82Q`\x0F\x91\x90\x91\x0B`@\x90\x91\x01Ra\x0E\x04`\x80\x87\x01``\x88\x01aJ\xF7V[a\x0E\r\x90aK\x14V[\x81Q`\x0F\x91\x90\x91\x0B`@\x90\x91\x01R[\x81Q`@\x90\x81\x01Q\x82Q\x82\x01Q\x82Q\x80\x84\x01\x90\x93R`\x04\x83RcOCBM`\xE0\x1B` \x84\x01R`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x03a\x0EnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x81`\0\x01Q`@\x01Q`\x0F\x0B\x13\x15a\x0E\xDEW\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x12\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x0E\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pa\x0F5V[\x81`\0\x01Q` \x01Q`\x0F\x0B\x81`\0\x01Q` \x01Q`\x0F\x0B\x13\x15`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cOCBM`\xE0\x1B\x81RP\x90a\x0F3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P[a\x0FJ\x85\x84\x84`\0\x01Q\x84`\0\x01Q\x88a0\xA3V[``\x85\x01Q\x82QQa\x01\0\x86\x01Q\x86Q`\0\x90\x81R`\x9C` \x90\x81R`@\x90\x91 T\x85Q\x90\x91\x01Qa\x0F\x9C\x94\x93\x92\x88\x92\x90\x91a\x0F\x8C\x91`\x0F\x91\x82\x0B\x91\x0Ba2\xA8V[a\x0F\x95\x90aK\x14V[`\x01a3#V[`\x0F\x90\x81\x0Ba\x01\0\x87\x01\x81\x90R\x91\x90\x0B`\xC0\x86\x01R\x83Q\x83QQ`\xE0\x87\x01Qa\x0F\xC9\x93\x89\x93\x92\x91\x90a3kV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x12U`\xF2\x1B` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIM`\xF0\x1B` \x82\x01RP``\x80\x84\x01Q\x90\x86\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x92\x83\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U\x81QQ`\x01\x14a\x10\x90W`\xE0\x84\x01Q\x84Q`\0\x90\x81R`\x9C` R`@\x81 \x80T\x90\x91\x90a\x10h\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x80QQ`\x01\x14a\x10\xEEW`\xE0\x84\x01Q` \x80\x86\x01Q`\0\x90\x81R`\x9C\x90\x91R`@\x81 \x80T\x90\x91\x90a\x10\xC6\x90\x84\x90`\x0F\x0BaK\x89V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[a\x10\xFD\x85``\x01Q\x83\x86a5WV[PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16\x80\x86R`\x9B\x85R\x87\x86 \x88Q\x94\x85\x01\x89RT`\x07\x81\x81\x0B\x86R`\x01`@\x1B\x82\x04\x81\x0B\x86\x88\x01R`\x01`\x80\x1B\x90\x91\x04`\x0F\x90\x81\x0B\x86\x8B\x01\x90\x81R\x92\x88R`\xA3\x90\x96R\x97\x90\x95 T\x16\x85R\x92Q\x90\x91\x0B\x90\x91R\x80Q\x91\x92\x90\x91a\x11\x98\x91\x0Bc;\x9A\xCA\0aK\xD9V[`\x0F\x0B` \x80\x84\x01\x91\x90\x91R\x81\x01Qa\x11\xB8\x90`\x07\x0Bc;\x9A\xCA\0aK\xD9V[`\x0F\x0B`@\x83\x01RP\x91\x90PV[```\0\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE4Wa\x11\xE4aBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\rW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12\xA4W`\xA0`\0\x85\x83\x81Q\x81\x10a\x122Wa\x122aLwV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x82\x82\x81Q\x81\x10a\x12}Wa\x12}aLwV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x12\x9C\x81aI\xE0V[\x91PPa\x12\x13V[P\x92\x91PPV[a\x12\xB5\x83\x82a6JV[a\x12\xBF\x83\x83a6\x95V[a\x12\xC9\x81\x83aK:V[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x9B` R`@\x90 \x80T`\x10\x90a\x12\xF8\x90\x84\x90`\x01`\x80\x1B\x90\x04`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`eT`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[\x82QQb\xFF\xFF\xFF\x16biso\x14\x15`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x13\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P\x82Q`\xA0\x01Q`\x08\x1C`\x01\x90\x81\x16\x14`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x14\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0a\x147\x84` \x01Q\x85`\0\x01Qa#\xD3V[`\0\x81\x81R`\xA7` R`@\x90 T\x90\x91P\x15a\x14dW`\0\x90\x81R`\xA7` R`@\x90 T\x90Pa\x08\xFDV[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81RaIS`\xF0\x1B` \x82\x01RP\x83QQ``\x1C`\0\x81\x81R`\xA4` R`@\x81 T\x90\x80[\x82\x81`\x01\x90\x1B\x11a\x15\x11W`\x01\x81\x1B\x83\x16\x15a\x14\xFFW\x87QQ`\0\x90\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a\x14\xFDW`\0a\x14\xDA\x82a6\xA2V[\x90P\x89` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x03a\x14\xFBWP\x91Pa\x15\x11V[P[P[a\x15\n`\x01\x82aL\x8DV[\x90Pa\x14\x96V[P\x80a\x16\xA8W\x86Q`\xA0\x01Q`\x0B\x1C`\x01\x90\x81\x16\x03a\x15\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FReduce-only order cannot create `D\x82\x01R\x7Fisolated subaccount\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[a\x15\xA5`\x01a\x04\0aL\xA5V[\x82\x03a\x15\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FToo many isolated subaccounts\0\0\0`D\x82\x01R`d\x01a\tlV[`\0[`\x01\x83\x16\x15a\x16\x16W`\x01\x92\x83\x1C\x92a\x16\x0F\x90\x82aL\xBCV[\x90Pa\x15\xF6V[` \x88\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xA4\x83R`@\x80\x82 \x80T`\x01`\xFF\x88\x16\x90\x81\x1B\x90\x91\x17\x90\x91U\x8CQQ`\x18\x96\x90\x96\x1Bc\xFF\0\0\0\x16\x93\x85\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16``\x8A\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x93\x90\x93\x17biso\x17\x80\x83R`\xA5\x85R\x81\x83 \x95\x90\x95U\x8BQQ\x82R`\xA6\x84R\x80\x82 \x92\x82R\x91\x90\x92R\x90 \x81\x90U\x90P[`\0\x84\x81R`\xA7` R`@\x81 \x82\x90U\x87Q`\xA0\x01Qa\x16\xC8\x90a6\xC6V[\x90P`\0\x81`\x0F\x0B\x13\x15a\x17\xF5W`\0\x85\x81R`\xA8` R`@\x81 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x84\x16\x17\x90U`\x9DT\x89QQ`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xE0\xB0b\x1F\x91a\x17\x1E\x85aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x81W=`\0\x80>=`\0\xFD[PP`\x9DT`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x85\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xE0\xB0b\x1F\x91P`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\xDCW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xF0W=`\0\x80>=`\0\xFD[PPPP[P\x96\x95PPPPPPV[`\0\x80\x84\x15a\x18RW`\0\x85\x81R`\x9C` R`@\x81 \x80T\x85\x92\x90a\x18*\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[\x83\x15a\x18\xA1W`\0\x84\x81R`\x9C` R`@\x81 \x80T\x85\x92\x90a\x18y\x90\x84\x90`\x0F\x0BaK\x89V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[PPP`\0\x91\x82R`\x9C` R`@\x80\x83 T\x91\x83R\x90\x91 T`\x0F\x91\x82\x0B\x92\x91\x0B\x90V[`\x9AT`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01R\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[Pc\xFF\xFF\xFF\xFF\x81\x16\x15\x80\x15\x90a\x199WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xA1` R`@\x90 T`\xFF\x16\x15[\x15a\x19\xA4W`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xA1` R`@\x81 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`\xA2\x80T\x91\x82\x01\x81U\x90\x91R\x7F\xAA\xF4\xF5\x8D\xE9\x93\0\xCF\xAD\xC4XWU\xF3v\xD5\xFAt}[\xC5a\xD5\xBD\x9Dq\r\xE1\xF9\x1B\xF4-\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\xD9\xE1\x9A\xDB|\xA8\x8F\xC3\xB0\xA9\xBE\xD1\xEF\xAAa\xB7\x7F\xEA\xAAs)\xB58\xE4\x8A\xA5\xFA\xA1g\xF1\xE2\xCB\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[```\0a\x1A\x18\x83\x85aL\xE1V[`\xA2T\x90\x91Pc\xFF\xFF\xFF\xFF\x80\x82\x16\x90\x83\x16\x11\x15a\x1A3W\x80\x91P[\x80c\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x1AKW\x80\x94P[`\0a\x1AW\x86\x84aM\tV[c\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1AuWa\x1AuaBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x9EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x85[\x83c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B0W`\xA2\x81c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xD0Wa\x1A\xD0aLwV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x82a\x1A\xF0\x89\x84aM\tV[c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x06Wa\x1B\x06aLwV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B(\x81aM.V[\x91PPa\x1A\xA3V[P\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1BZWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1BtWP0;\x15\x80\x15a\x1BtWP`\0T`\xFF\x16`\x01\x14[a\x1B\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1C\tW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1C\x11a6\xE5V[a\x1C\x1A\x82a7XV[a\x1Cu`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cNado`\xE0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7\x82V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x1C\xB5\x90`\0\x90`\x04\x01aMQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xF6\x91\x90aMyV[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x9AT`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1D:\x90`\x01\x90`\x04\x01aMQV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D{\x91\x90aMyV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80\x15a\x1D\xE1W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80`\0a\x1D\xF5\x86\x86a#\xD3V[\x90P`\0a\x1E\x03\x87\x86a#\xD3V[`\0\x92\x83R`\x9C` R`@\x80\x84 T\x91\x84R\x90\x92 T`\x0F\x92\x83\x0B\x98\x92\x0B\x96P\x90\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1EoWP`@\x80Q\x80\x82\x01\x90\x91R`\0\x81Re\xB5\xE6 \xF4\x80\0` \x82\x01Ra\x08\xFDV[Pc\xFF\xFF\xFF\xFF\x91\x82\x16`\0\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x82R\x82\x90 \x82Q\x80\x84\x01\x90\x93RT`\x0F\x81\x81\x0B\x84R`\x01`\x80\x1B\x90\x91\x04\x90\x0B\x90\x82\x01R\x90V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[`\x9DT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x94\x91\x90\x81\x01\x90aM\x96V[\x90P`\x01[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a!.W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1F\xC2Wa\x1F\xC2aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a $WPPa!\x1CV[`\x9DTc\xFF\xFF\xFF\xFF\x83\x81\x16`\0\x90\x81R`\xA3` R`@\x90\x81\x90 T\x84\x82\x01Q\x91Qc\xE0\xB0b\x1F`\xE0\x1B\x81R\x92\x16`\x04\x83\x01R`\x01`$\x83\x01R`\x0F\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \x95W`\0\x80\xFD[PZ\xF1\x15\x80\x15a \xA9W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a!&\x81aM.V[\x91PPa\x1F\x99V[P`\x9E`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAA\x91\x90\x81\x01\x90aM\x96V[\x90P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06uW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\xD8Wa!\xD8aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B\x83R`@\x80\x82 \x81Q``\x81\x01\x83R\x90T`\x07\x81\x81\x0B\x83R`\x01`@\x1B\x82\x04\x90\x0B\x95\x82\x01\x95\x90\x95R`\x01`\x80\x1B\x90\x94\x04`\x0F\x0B\x90\x84\x01\x81\x90R\x91\x93P\x03a\":WPPa#*V[`\x9ET`@\x82\x81\x01Q\x90Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`$\x82\x01R`\0`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\"\xA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\"\xB7W=`\0\x80>=`\0\xFD[PP`\0`@\x80\x85\x01\x82\x81Rc\xFF\xFF\xFF\xFF\x90\x96\x16\x82R`\x9B` \x90\x81R\x91 \x84Q\x81T\x92\x90\x95\x01Q\x95Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x94\x16\x97\x90\x96\x16\x96\x90\x96\x17\x91\x90\x91\x17\x94\x90\x94\x16\x92\x90\x92\x17\x90\x92UPP[\x80a#4\x81aM.V[\x91PPa!\xAFV[a#Da7\xF7V[a#N`\0a8QV[V[`\0a#\\\x83\x83a#\xD3V[`@\x80\x84\x01Q`\0\x83\x81R`\x9C` R\x91\x90\x91 T\x91\x92P`\x0F\x91\x82\x0B\x91\x0B\x14\x80a#\x8FWPa#\x8F\x82``\x01Qa8\xA3V[\x15a\x1D\xE1W`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UPPV[`\0\x80`@Q\x80`\xA0\x01`@R\x80`c\x81R` \x01aO\xC6`c\x919\x90P`\0\x81\x80Q\x90` \x01 \x84`\0\x01Q\x85` \x01Q\x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q`@Q` \x01a$s\x97\x96\x95\x94\x93\x92\x91\x90\x96\x87R` \x87\x01\x95\x90\x95R`\x0F\x93\x84\x0B`@\x87\x01R\x91\x90\x92\x0B``\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\x80\x85\x01R\x16`\xA0\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa$\xB7`fT\x90V[`gT`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01Rc\xFF\xFF\xFF\xFF\x87\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa%C\x81\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[\x96\x95PPPPPPV[`\0\x80a%Z\x85\x85a,ZV[\x90P\x82a%hW\x80Qa%nV[\x80` \x01Q[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x08\xFD\x90`\x07\x0Bc;\x9A\xCA\0aK\xD9V[`\0a%\xAA\x83\x83a#\xD3V[`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x80T`\x01`\x01`\x80\x1B\x03\x19\x90\x81\x16\x90\x91U`\xA7\x83R\x81\x84 \x84\x90U`\xA8\x90\x92R\x90\x91 \x80T\x90\x91\x16\x90UPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a&WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSequencerGated: caller is not th`D\x82\x01Ri\x19H\x19[\x99\x1C\x1B\xDA[\x9D`\xB2\x1B`d\x82\x01R`\x84\x01a\tlV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16a)\x1EW`\x9DT`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a&\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\xD7\x91\x90\x81\x01\x90aM\x96V[\x90P`\0`\x9E`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra'V\x91\x90\x81\x01\x90aM\x96V[\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(RW`\0c\xFF\xFF\xFF\xFF\x16\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\x8AWa'\x8AaLwV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x03\x15a(@W`@Q\x80`@\x01`@R\x80\x85`@\x01Q`\x0F\x0B\x81R` \x01\x85``\x01Q`\x0F\x0B\x81RP`\x9F`\0\x86`\0\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x85\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xFEWa'\xFEaLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x90\x91\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U[\x80a(J\x81aM.V[\x91PPa'[V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x18W`@Q\x80`@\x01`@R\x80\x85`@\x01Q`\x0F\x0B\x81R` \x01\x85``\x01Q`\x0F\x0B\x81RP`\x9F`\0\x86`\0\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x84c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a(\xC5Wa(\xC5aLwV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x81\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x90\x91\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U\x80a)\x10\x81aM.V[\x91PPa(VV[PPPPV[`@\x80Q\x80\x82\x01\x82R\x82\x82\x01Q`\x0F\x90\x81\x0B\x82R``\x84\x01Q\x90\x0B` \x80\x83\x01\x91\x82R\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\x9F\x83R\x85\x81 \x83\x88\x01Q\x90\x92\x16\x81R\x91R\x92\x90\x92 \x90Q\x91Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x92\x16\x91\x90\x91\x17\x90U[PV[c\xFF\xFF\xFF\xFF\x83\x81\x16\x14a)\xBDWc\xFF\xFF\xFF\xFF\x84\x81\x16`\0\x90\x81R`\xA3` R`@\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16\x91\x85\x16\x91\x90\x91\x17\x90U[a)\xCBc;\x9A\xCA\0\x82aN;V[c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\x0Ec;\x9A\xCA\0\x83aN;V[c\xFF\xFF\xFF\xFF\x90\x94\x16`\0\x90\x81R`\x9B` R`@\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x90\x95\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x95\x16\x94\x90\x94\x17\x90\x93UPPPV[```\0\x80[`\n\x81\x10\x15a*\xAAW`\0\x84\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a*\x97Wa*\x94`\x01\x84aL\x8DV[\x92P[P\x80a*\xA2\x81aI\xE0V[\x91PPa*`V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xC6Wa*\xC6aBhV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\n\x81\x10\x15a+\\W`\0\x85\x81R`\xA6` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 T\x80\x15a+IW\x80\x83a+)\x86aN\x82V[\x95P\x85\x81Q\x81\x10a+<Wa+<aLwV[` \x02` \x01\x01\x81\x81RPP[P\x80a+T\x81aI\xE0V[\x91PPa*\xF5V[P\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x08\xFD\x90`\x01`@\x1B\x90\x04`\x07\x0Bc;\x9A\xCA\0aK\xD9V[a+\x9Aa7\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\tlV[a)\x84\x81a8QV[a)\x84\x81a8\xCAV[`\0\x80a,:a,7\x84a<\xBAV[\x90V[T`\x0F\x0B\x91Pa,La,7\x84a=\x16V[T\x91\x93`\x0F\x92\x90\x92\x0B\x92PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Rb\xFF\xFF\xFF\x83\x16biso\x03a,\x8FW`\0\x92\x83R`\xA5` R`@\x90\x92 T\x91[``\x83\x90\x1C`\0\x90\x81R`\xA0` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16a,\xB3\x81\x84a\x1E-V[\x94\x93PPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\x9AT`@Qc\xDE\xB1N\xC3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDE\xB1N\xC3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-R\x91\x90aMyV[`\x9ET\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a-\xA3WP`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R`\0` \x83\x01R`\x01\x90\x82\x01Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x9DT`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x90\x92\x16``\x83\x01RP\x90V[\x83Q`\xA0\x01Q`\0\x90`\xFF\x16`\x01\x14a-\xF8WP`\0a%CV[\x84QQ`\0\x19\x01a.\x0BWP`\x01a%CV[\x84Q\x83\x15a.4W`\xA0\x81\x01Q`\t\x1C`\x03\x90\x81\x16\x03a./W`\0\x91PPa%CV[a.PV[a.A\x81`\xA0\x01Qa=YV[\x15a.PW`\0\x91PPa%CV[`\0\x85\x81R`\x9C` R`@\x90\x81\x90 T\x90\x82\x01\x80Q`\x0F\x92\x90\x92\x0B\x91\x82\x91\x90a.{\x90\x83\x90aK\x89V[`\x0F\x0B\x90RP`\xA0\x82\x01Q`\x0B\x1C`\x01\x90\x81\x16\x03a04W`\0\x89`@\x01Qa/%W` \x8A\x01Q``\x8B\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xFBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x1F\x91\x90aN\x99V[Qa/\xA5V[\x89Q``\x8B\x01Q\x84Q`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA3\x91\x90aN\xE5V[Q[\x90P`\0\x81`\x0F\x0B\x13\x15\x15`\0\x84`@\x01Q`\x0F\x0B\x13\x15\x15\x03a/\xCEW`\0`@\x84\x01Ra02V[`\0\x83`@\x01Q`\x0F\x0B\x13\x15a0\x02Wa/\xF5\x83`@\x01Q\x82a/\xF0\x90aK\x14V[a=\x82V[`\x0F\x0B`@\x84\x01Ra02V[`\0\x83`@\x01Q`\x0F\x0B\x12\x15a02Wa0)\x83`@\x01Q\x82a0$\x90aK\x14V[a=\x9EV[`\x0F\x0B`@\x84\x01R[P[`\0\x82` \x01Q`\x0F\x0B\x13\x80a0XWP`\xA0\x82\x01Q`\x02`\x0C\x91\x90\x91\x1C`\x03\x16\x10\x15[\x80\x15a0nWP\x86QQ`\x02\x14\x80a0nWP`\x01[\x80\x15a0\x80WP`@\x82\x01Q`\x0F\x0B\x15\x15[\x80\x15a0\x96WPa0\x94\x82``\x01Qa8\xA3V[\x15[\x99\x98PPPPPPPPPV[`\0\x80`\0\x85`@\x01Q`\x0F\x0B\x12\x15a0\xD3Wa0\xCC\x85`@\x01Q\x85`@\x01Qa0$\x90aK\x14V[\x91Pa1\0V[`\0\x85`@\x01Q`\x0F\x0B\x13\x15a0\xF9Wa0\xCC\x85`@\x01Q\x85`@\x01Qa/\xF0\x90aK\x14V[PPa2\xA1V[`@\x86\x01Qa1\x0F\x90\x83aOWV[a1\x19\x90\x83aK\x89V[\x91P`\0a17\x85` \x01Q\x84`\x0F\x0Ba2\xA8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa1B\x81aK\x14V[\x91P`\0a1]\x89``\x01Q\x87`\0\x01Q\x8A\x85`\0\x80a3#V[\x80\x93P\x81\x92PPP\x83\x87`@\x01\x81\x81Qa1w\x91\x90aK\x89V[`\x0F\x0B\x90RP`@\x86\x01\x80Q\x85\x91\x90a1\x91\x90\x83\x90aK:V[`\x0F\x0B\x90RP\x87Q\x86Qa1\xB0\x91\x8B\x91a1\xAA\x88aK\x14V[\x86a3kV[`\x0F\x84\x81\x0B`\xE0\x87\x01R\x83\x90\x0Ba\x01\0\x86\x01R\x85Q` \x80\x87\x01Q``\x8C\x81\x01Q\x92\x8A\x01Q`\x80\x80\x8B\x01Q\x92\x8C\x01Q\x90\x8C\x01Q`\xA0\x8D\x01Q\x94\x95c\xFF\xFF\xFF\xFF\x16\x94\x7F\xB5c\xBD7\"b\x0Ez\xF6\xC3\xDA\xE1\t\x89|\xA2\xF4_\xBB\xC5\x97_\xB6U;\xB2\xD5;w\xE5K\xF3\x94\x92\x91\x90`\x01`\x08\x82\x90\x1C\x81\x16\x14`\0\x8B\x8Fa2-\x90aK\x14V[`@\x80Q`\x0F\x9A\x8B\x0B\x81R\x98\x8A\x0B` \x8A\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16\x90\x89\x01R\x95\x90\x94\x16``\x87\x01R`\x01`\x01`\x80\x1B\x03\x90\x92\x16`\x80\x86\x01R\x15\x15`\xA0\x85\x01R\x15\x15`\xC0\x84\x01R\x83\x0B`\xE0\x83\x01R\x82\x0Ba\x01\0\x82\x01R\x90\x87\x90\x0Ba\x01 \x82\x01Ra\x01@\x01`@Q\x80\x91\x03\x90\xA4PPPP[PPPPPV[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x85\x81\x0B\x90\x85\x90\x0B\x02[\x05\x90Po\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a2\xEAWP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a+\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[`\0\x80`\0\x80a37\x8A\x8A\x8A\x8A\x8A\x8Aa=\xB3V[\x91P\x91P\x84\x15a3PWa3K\x8A\x83a6JV[a3ZV[a3Z\x8A\x83a6\x95V[\x90\x92P\x90P[\x96P\x96\x94PPPPPV[\x84`@\x01Q\x15a3\xFCW\x84Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a3\xDFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a3\xF3W=`\0\x80>=`\0\xFD[PPPPa2\xA1V[c\xFF\xFF\xFF\xFF\x84\x16a4^W` \x85\x01Q``\x86\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x81\x0B`D\x83\x01R\x83\x90\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90`\x84\x01a3\xC5V[` \x85\x01Q``\x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xC2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xD6W=`\0\x80>=`\0\xFD[PPP` \x86\x01Q`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x84\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91Pc\xE0\xB0b\x1F\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a58W`\0\x80\xFD[PZ\xF1\x15\x80\x15a5LW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x81\x01Q\x81Q\x83Q` \x81\x01Q`\xA0\x80\x86\x01Q``\x84\x01Q`\x80\x85\x01Q\x92\x90\x94\x01Qc\xFF\xFF\xFF\xFF\x8A\x16\x94\x7F\xB5c\xBD7\"b\x0Ez\xF6\xC3\xDA\xE1\t\x89|\xA2\xF4_\xBB\xC5\x97_\xB6U;\xB2\xD5;w\xE5K\xF3\x94\x93\x90\x91`\x01`\x08\x82\x90\x1C\x81\x16\x14`\x01\x8B`\xC0\x01Q\x8C`\xE0\x01Q\x8Da\x01\0\x01Q`@Qa6=\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90`\x0F\x9A\x8B\x0B\x81R\x98\x8A\x0B` \x8A\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8A\x01R\x95\x90\x96\x16``\x88\x01R`\x01`\x01`\x80\x1B\x03\x93\x90\x93\x16`\x80\x87\x01R\x90\x15\x15`\xA0\x86\x01R\x15\x15`\xC0\x85\x01R\x84\x0B`\xE0\x84\x01R\x90\x83\x0Ba\x01\0\x83\x01R\x90\x91\x0Ba\x01 \x82\x01Ra\x01@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[\x80a6Wa,7\x84a=\x16V[\x80T`\0\x90a6j\x90\x84\x90`\x0F\x0BaK:V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPV[\x80a6Wa,7\x84a<\xBAV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a6\xBBWP`\0\x91\x90PV[P` \x1Ca\xFF\xFF\x16\x90V[`\0a\x08\xFDg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x84\x90\x1C\x16d\xE8\xD4\xA5\x10\0aOyV[`\0Ta\x01\0\x90\x04`\xFF\x16a7PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a#Na?\x16V[a7`a7\xF7V[`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Ta\x01\0\x90\x04`\xFF\x16a7\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a\x06u\x82\x82a?\x8AV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a#NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\tlV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0a8\xADa@\x0FV[`\x01`\x01`\x80\x1B\x03\x16\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15\x90P\x91\x90PV[`\0a8\xD5\x82a6\xA2V[\x90P\x80c\xFF\xFF\xFF\xFF\x16`\0\x03a8\xE9WPPV[`\x9ET`@Qc|\x1E\x14\x87`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c|\x1E\x14\x87\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9c\x91\x90aN\xE5V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x1D\xE1W`\0a9}\x84a@\x82V[`\0\x85\x81R`\xA5` \x90\x81R`@\x90\x91 T\x90\x84\x01Q\x91\x92P``\x86\x90\x1C\x91`\x0F\x0B\x15a:\xBFW`\x9ET` \x85\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xF8\xA4.Q\x90\x87\x90\x89\x90`\0\x90a9\xCE\x90aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x04\x85\x01R`$\x84\x01\x92\x90\x92R`\x0F\x90\x81\x0B`D\x84\x01R\x0B`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a:9W=`\0\x80>=`\0\xFD[PP`\x9ET` \x87\x01Q`@Qc\xF8\xA4.Q`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x8A\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\0`D\x82\x01R`\x0F\x91\x90\x91\x0B`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xF8\xA4.Q\x91P`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a:\xA6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a:\xBAW=`\0\x80>=`\0\xFD[PPPP[`\x9DT`@Qc|\x1E\x14\x87`\xE0\x1B\x81R`\0`\x04\x82\x01\x81\x90R`$\x82\x01\x89\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c|\x1E\x14\x87\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;4\x91\x90aN\x99V[Q\x90P`\x0F\x81\x90\x0B\x15a<5W`\x9DT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F`\0\x89a;^\x85aK\x14V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a;\xADW`\0\x80\xFD[PZ\xF1\x15\x80\x15a;\xC1W=`\0\x80>=`\0\xFD[PP`\x9DT`@Qc\xE0\xB0b\x1F`\xE0\x1B\x81R`\0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x0F\x85\x90\x0B`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc\xE0\xB0b\x1F\x91P`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<0W=`\0\x80>=`\0\xFD[PPPP[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xA4` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x8A\x16\x90\x81\x1B`\0\x19\x18\x90\x91\x16\x90\x91U\x85\x84R`\xA6\x83R\x81\x84 \x90\x84R\x82R\x80\x83 \x83\x90U\x89\x83R`\xA5\x90\x91R\x80\x82 \x82\x90UQ\x83\x91\x89\x91\x7F\x02|\xE6\xFB\xFB[M\x17\xA0\xEE6\xB5\x92\xEF\r\xB7{\x06\r\xF7\xE0#\xBC\x84\xE6\x8C_\x06d\xC9\xB8:\x91\x90\xA3PPPPPPPV[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x7F\xA1\x06\x7F\xD9I\x04\xBEg\xF1J\xA0\xB46yq\xBB1L\x8F>\xF4K\xED\xF9\xE7\xF1\xCE\xCE\x82\x1CS\x8D\x91\x81\x01\x91\x90\x91R`\0\x90``\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R\x7FL}\xA8\xC9\xFEpW+2\x9Dp\x9D\x9E+\xDEpx;r\r\t\xC8B\x18g\xCC\xB7\xB85v\x06_\x91\x81\x01\x91\x90\x91R`\0\x90``\x01a<\xF9V[`\0`\x03`\t\x83\x90\x1C\x16`\x01\x81\x14\x80a={WP\x80`\x01`\x01`\x80\x1B\x03\x16`\x02\x14[\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a=\x97W\x81a={V[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a=\x97W\x81a={V[`\0\x80`\0\x19\x87\x01a=\xCAWP`\0\x90P\x83a3`V[`\0\x83\x15a>uW\x84`\x0F\x0B`\0\x03a>\x06W` \x87\x01Qa=\xEC\x90\x82aK:V[\x90P`\0\x86`\x0F\x0B\x12\x15a>\x06Wa>\x03\x81aK\x14V[\x90P[` \x87\x01Q`\0\x90a> a>\x1B\x89\x89aK:V[a@\xA5V[a>*\x91\x90aK\x89V[\x90Pa><\x81a/\xF0\x89`\x0F\x0Ba@\xC0V[\x90P`\0\x81`\x0F\x0B\x13\x15a>oW`\0\x87`\x0F\x0B\x12\x15a>bWa>_\x81aK\x14V[\x90P[a>l\x81\x83aK:V[\x91P[Pa>\x82V[a>\x7F\x86\x82aK:V[\x90P[`\0a>\x8F\x89\x8B\x87a%MV[a>\xA1\x90g\r\xE0\xB6\xB3\xA7d\0\0aK\x89V[\x90P`\0\x80\x83`\x0F\x0B\x13a>\xC2Wa>\xBD`\x0F\x84\x90\x0B\x83aA*V[a>\xD0V[a>\xD0`\x0F\x84\x90\x0B\x83a2\xA8V[\x90P`\0a>\xDE\x82\x85aK\x89V[\x90P\x80\x8A``\x01\x81\x81Qa>\xF2\x91\x90aK:V[`\x0F\x0B\x90RP\x80a?\x03\x81\x8BaK\x89V[\x95P\x95PPPPP\x96P\x96\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a?\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[a#N3a8QV[`\0Ta\x01\0\x90\x04`\xFF\x16a?\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\tlV[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `f\x91\x90\x91U`gUV[`eT`@\x80Qc*\xBFh\xDD`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cU~\xD1\xBA\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a@YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@}\x91\x90aO\xA8V[\x90P\x90V[`\0b\xFF\xFF\xFF\x82\x16biso\x14a@\x9BWP`\0\x91\x90PV[P`\x18\x1C`\xFF\x16\x90V[`\0\x80\x82`\x0F\x0B\x12a@\xB7W\x81a\x08\xFDV[a\x08\xFD\x82aK\x14V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra'\xA3`\xF1\x1B` \x82\x01R`\0\x90`\x0F\x83\x90\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03aA\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\x0F\x0B\x12aA#W\x81a\x08\xFDV[P`\0\x03\x90V[`\0\x81`\x0F\x0B`\0\x14\x15`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\"!-`\xE9\x1B\x81RP\x90aAnW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\tl\x91\x90aJ\xA2V[P`\0\x82`\x0F\x0Bg\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x0B\x85`\x0F\x0B\x02\x81a2\xBFWa2\xBFaN%V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a)\x84W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aA\xB8W`\0\x80\xFD[\x825\x91P` \x83\x015aA\xCA\x81aA\x93V[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aA\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aB\x01W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15aB\"W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB9W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a={W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB]W`\0\x80\xFD[\x815a={\x81aA\x93V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aB\xA7WaB\xA7aBhV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB\xC9WaB\xC9aBhV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)\x84W`\0\x80\xFD[\x805aB\xF3\x81aB\xD3V[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15aC\x0BW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\"W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aC3W`\0\x80\xFD[\x805aCFaCA\x82aB\xAFV[aB~V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aCeW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aC\x8CW\x835aC}\x81aB\xD3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aCjV[\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aC\xB3V[P\x90\x96\x95PPPPPPV[\x80`\x0F\x0B\x81\x14a)\x84W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aD\x05W`\0\x80\xFD[\x835aD\x10\x81aA\x93V[\x92P` \x84\x015aD \x81aC\xE1V[\x91P`@\x84\x015aD0\x81aC\xE1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aDNW`\0\x80\xFD[\x825aDY\x81aB\xD3V[\x91P` \x83\x015aA\xCA\x81aA\x93V[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aB\xF3W`\0\x80\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a)\x84W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15aD\xA8W`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aD\xCBWaD\xCBaBhV[`@R\x825\x81R\x90P\x80` \x83\x015aD\xE3\x81aC\xE1V[` \x82\x01R`@\x83\x015aD\xF6\x81aC\xE1V[`@\x82\x01RaE\x07``\x84\x01aDiV[``\x82\x01RaE\x18`\x80\x84\x01aDiV[`\x80\x82\x01R`\xA0\x83\x015aE+\x81aD\x81V[`\xA0\x91\x90\x91\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aEIW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEcWaEcaBhV[aEv`\x1F\x82\x01`\x1F\x19\x16` \x01aB~V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aE\x8BW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aE\xBBW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aE\xD3W`\0\x80\xFD[\x90\x84\x01\x90a\x01\0\x82\x87\x03\x12\x15aE\xE8W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aF\x03WaF\x03aBhV[`@RaF\x10\x87\x84aD\x96V[\x81R`\xC0\x83\x015aF \x81aA\x93V[` \x82\x01R`\xE0\x83\x015\x82\x81\x11\x15aF7W`\0\x80\xFD[aFC\x88\x82\x86\x01aE8V[`@\x83\x01RP\x93PaFZ\x91PP` \x84\x01aB\xE8V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aFxW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015aD0\x81aC\xE1V[`\0\x80`@\x83\x85\x03\x12\x15aF\xA4W`\0\x80\xFD[\x825aDY\x81aA\x93V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\xCBV[`\0\x80`@\x83\x85\x03\x12\x15aG\x03W`\0\x80\xFD[\x825aG\x0E\x81aB\xD3V[\x91P` \x83\x015aA\xCA\x81aB\xD3V[`\0\x80`\0a\x01\xA0\x84\x86\x03\x12\x15aG4W`\0\x80\xFD[\x835aG?\x81aA\x93V[\x92PaGN\x85` \x86\x01aD\x96V[\x91PaG]\x85`\xE0\x86\x01aD\x96V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15aGyW`\0\x80\xFD[\x825aG\x84\x81aA\x93V[\x91P` \x83\x015aA\xCA\x81aC\xE1V[`\0\x80` \x83\x85\x03\x12\x15aG\xA7W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aG\xBFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aG\xD3W`\0\x80\xFD[\x815\x81\x81\x11\x15aG\xE2W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15aG\xF4W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\xE0\x83\x85\x03\x12\x15aH\x19W`\0\x80\xFD[\x825aH$\x81aA\x93V[\x91PaFZ\x84` \x85\x01aD\x96V[`\0\x80`\0``\x84\x86\x03\x12\x15aHHW`\0\x80\xFD[\x835\x92P` \x84\x015aHZ\x81aA\x93V[\x91P`@\x84\x015\x80\x15\x15\x81\x14aD0W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15aH\x81W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aH\xA4WaH\xA4aBhV[`@R\x825aH\xB2\x81aA\x93V[\x81R` \x83\x015aH\xC2\x81aA\x93V[` \x82\x01R`@\x83\x015aH\xD5\x81aC\xE1V[`@\x82\x01R``\x83\x015aH\xE8\x81aC\xE1V[``\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aI\nW`\0\x80\xFD[\x845aI\x15\x81aA\x93V[\x93P` \x85\x015aI%\x81aA\x93V[\x92P`@\x85\x015aI5\x81aC\xE1V[\x91P``\x85\x015aIE\x81aC\xE1V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`@\x83\x85\x03\x12\x15aIcW`\0\x80\xFD[\x825\x91P` \x83\x015aA\xCA\x81aC\xE1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aC\xD5W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aI\x91V[`\0` \x82\x84\x03\x12\x15aI\xBFW`\0\x80\xFD[\x815a={\x81aB\xD3V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aI\xF2WaI\xF2aI\xCAV[P`\x01\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aJ\x0FW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aJ\x0FW`\0\x80\xFD[`\0`\xE0\x826\x03\x12\x15aJAW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15aJeWaJeaBhV[\x81`@RaJs6\x86aD\x96V[\x83R`\xC0\x85\x015\x91P\x80\x82\x11\x15aJ\x89W`\0\x80\xFD[PaJ\x966\x82\x86\x01aE8V[` \x83\x01RP\x92\x91PPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aJ\xCFW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aJ\xB3V[\x81\x81\x11\x15aJ\xE1W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK\tW`\0\x80\xFD[\x815a={\x81aC\xE1V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aK1WaK1aI\xCAV[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aKdWaKdaI\xCAV[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aK\x80WaK\x80aI\xCAV[P\x01\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aK\xB4WaK\xB4aI\xCAV[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aK\xCFWaK\xCFaI\xCAV[P\x90\x03\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\x01`\x01`\x7F\x1B\x03`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15aL\tWaL\taI\xCAV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15aL5WaL5aI\xCAV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15aLQWaLQaI\xCAV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15aLgWaLgaI\xCAV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aL\xA0WaL\xA0aI\xCAV[P\x01\x90V[`\0\x82\x82\x10\x15aL\xB7WaL\xB7aI\xCAV[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15aL\xD9WaL\xD9aI\xCAV[\x01\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aM\0WaM\0aI\xCAV[\x01\x94\x93PPPPV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aM&WaM&aI\xCAV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03aMGWaMGaI\xCAV[`\x01\x01\x93\x92PPPV[` \x81\x01`\x02\x83\x10aMsWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aM\x8BW`\0\x80\xFD[\x81Qa={\x81aB\xD3V[`\0` \x80\x83\x85\x03\x12\x15aM\xA9W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM\xC0W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aM\xD1W`\0\x80\xFD[\x80QaM\xDFaCA\x82aB\xAFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aM\xFEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aC\x8CW\x83QaN\x16\x81aA\x93V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aN\x03V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80aNRWaNRaN%V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15aNyWaNyaI\xCAV[\x90\x05\x93\x92PPPV[`\0\x81aN\x91WaN\x91aI\xCAV[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aN\xABW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aN\xCEWaN\xCEaBhV[`@R\x82QaN\xDC\x81aC\xE1V[\x81R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15aN\xF7W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aO\x1AWaO\x1AaBhV[`@R\x82QaO(\x81aC\xE1V[\x81R` \x83\x01QaO8\x81aC\xE1V[` \x82\x01R`@\x83\x01QaOK\x81aC\xE1V[`@\x82\x01R\x93\x92PPPV[`\0\x82`\x0F\x0B\x80aOjWaOjaN%V[\x80\x83`\x0F\x0B\x07\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aO\x9FWaO\x9FaI\xCAV[\x02\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\xBAW`\0\x80\xFD[\x81Qa={\x81aD\x81V\xFEOrder(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce,uint128 appendix)\xA2dipfsX\"\x12 \x08O\xDCM\x924\x97gjM%m,o\xE2\"\xC5\x9ER\x05\xEA\xB0\x98x\xED\x83\x8F[M\xB7b\x1AdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static OFFCHAINEXCHANGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct OffchainExchange<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for OffchainExchange<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OffchainExchange<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OffchainExchange<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OffchainExchange<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(OffchainExchange))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OffchainExchange<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                OFFCHAINEXCHANGE_ABI.clone(),
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
                OFFCHAINEXCHANGE_ABI.clone(),
                OFFCHAINEXCHANGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assertProduct` (0x84528093) function
        pub fn assert_product(
            &self,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 82, 128, 147], transaction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createIsolatedSubaccount` (0x34f9a4a4) function
        pub fn create_isolated_subaccount(
            &self,
            txn: CreateIsolatedSubaccount,
            linked_signer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([52, 249, 164, 164], (txn, linked_signer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropDigest` (0xe1e7188d) function
        pub fn drop_digest(
            &self,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 231, 24, 141], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropOrder` (0xbf29174c) function
        pub fn drop_order(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 41, 23, 76], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dropOrderChecked` (0x8dc3f468) function
        pub fn drop_order_checked(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 195, 244, 104], (product_id, order))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpFees` (0x707c8b58) function
        pub fn dump_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `filledAmounts` (0x40f1a34d) function
        pub fn filled_amounts(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([64, 241, 163, 77], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllFeeTiers` (0x1ed01dad) function
        pub fn get_all_fee_tiers(
            &self,
            users: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([30, 208, 29, 173], users)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCollectedFees` (0xff0be9ef) function
        pub fn get_collected_fees(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([255, 11, 233, 239], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCustomFeeAddresses` (0x3fceea28) function
        pub fn get_custom_fee_addresses(
            &self,
            start_at: u32,
            limit: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 206, 234, 40], (start_at, limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDigest` (0xb56b6a6c) function
        pub fn get_digest(
            &self,
            product_id: u32,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([181, 107, 106, 108], (product_id, order))
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
        ///Calls the contract's `getFeeFractionX18` (0xb5cbd70e) function
        pub fn get_fee_fraction_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            taker: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([181, 203, 215, 14], (subaccount, product_id, taker))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFeeRatesX18` (0x0f2c878e) function
        pub fn get_fee_rates_x18(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([15, 44, 135, 142], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getIsolatedSubaccountByDigest` (0x2a6b3ffe) function
        pub fn get_isolated_subaccount_by_digest(
            &self,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([42, 107, 63, 254], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getIsolatedSubaccounts` (0xedc6d37b) function
        pub fn get_isolated_subaccounts(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([237, 198, 211, 123], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarginByDigest` (0x6ac3ee0b) function
        pub fn get_margin_by_digest(
            &self,
            digest: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([106, 195, 238, 11], digest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarketInfo` (0x1d029b4d) function
        pub fn get_market_info(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, MarketInfo> {
            self.0
                .method_hash([29, 2, 155, 77], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMinSize` (0xb60aaa7c) function
        pub fn get_min_size(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([182, 10, 170, 124], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderFilledAmounts` (0x52699960) function
        pub fn get_order_filled_amounts(
            &self,
            product_id: u32,
            order_1: Order,
            order_2: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash([82, 105, 153, 96], (product_id, order_1, order_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getParentSubaccount` (0x13b56ddb) function
        pub fn get_parent_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 181, 109, 219], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRawMarketInfo` (0x3eb0f4b3) function
        pub fn get_raw_market_info(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, MarketInfoStore> {
            self.0
                .method_hash([62, 176, 244, 179], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSizeIncrement` (0xf2b26331) function
        pub fn get_size_increment(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([242, 178, 99, 49], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlots` (0xfab2c469) function
        pub fn get_slots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([250, 178, 196, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTierFeeRateX18` (0x6baa1b83) function
        pub fn get_tier_fee_rate_x18(
            &self,
            tier: u32,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, FeeRates> {
            self.0
                .method_hash([107, 170, 27, 131], (tier, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementFees` (0x1f4ce016) function
        pub fn increment_fees(
            &self,
            product_id: u32,
            maker_fee: i128,
            taker_fee: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 76, 224, 22], (product_id, maker_fee, taker_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (clearinghouse, endpoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isIsolatedSubaccountActive` (0x1a2b2d16) function
        pub fn is_isolated_subaccount_active(
            &self,
            parent: [u8; 32],
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([26, 43, 45, 22], (parent, subaccount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrders` (0x1ab0a588) function
        pub fn match_orders(
            &self,
            txn: MatchOrdersWithSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 176, 165, 136], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyFilledAmount` (0x35ed4e6d) function
        pub fn modify_filled_amount(
            &self,
            taker_digest: [u8; 32],
            maker_digest: [u8; 32],
            taker_delta: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (i128, i128)> {
            self.0
                .method_hash(
                    [53, 237, 78, 109],
                    (taker_digest, maker_digest, taker_delta),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `orderVersion` (0x01e2abd5) function
        pub fn order_version(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([1, 226, 171, 213], ())
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
        ///Calls the contract's `setFeeTier` (0x2bd28617) function
        pub fn set_fee_tier(
            &self,
            user: ::ethers::core::types::Address,
            tier: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 210, 134, 23], (user, tier))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFilledAmount` (0xde1078bd) function
        pub fn set_filled_amount(
            &self,
            digest: [u8; 32],
            filled_amount: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 16, 120, 189], (digest, filled_amount))
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
        ///Calls the contract's `tryCloseIsolatedSubaccount` (0xf6ee7b4b) function
        pub fn try_close_isolated_subaccount(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 238, 123, 75], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateCollectedFees` (0x812609f1) function
        pub fn update_collected_fees(
            &self,
            product_id: u32,
            collected_fees: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 38, 9, 241], (product_id, collected_fees))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeTier` (0x385d448d) function
        pub fn update_fee_tier(
            &self,
            user: ::ethers::core::types::Address,
            new_tier: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 93, 68, 141], (user, new_tier))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMarket` (0xc8d6dbcb) function
        pub fn update_market(
            &self,
            product_id: u32,
            quote_id: u32,
            size_increment: i128,
            min_size: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 214, 219, 203],
                    (product_id, quote_id, size_increment, min_size),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateTierFeeRates` (0xc71ede60) function
        pub fn update_tier_fee_rates(
            &self,
            txn: UpdateTierFeeRates,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([199, 30, 222, 96], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CloseIsolatedSubaccount` event
        pub fn close_isolated_subaccount_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CloseIsolatedSubaccountFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeTierUpdate` event
        pub fn fee_tier_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeTierUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FillOrder` event
        pub fn fill_order_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FillOrderFilter> {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OffchainExchangeEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for OffchainExchange<M>
    {
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
    #[ethevent(
        name = "CloseIsolatedSubaccount",
        abi = "CloseIsolatedSubaccount(bytes32,bytes32)"
    )]
    pub struct CloseIsolatedSubaccountFilter {
        #[ethevent(indexed)]
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub isolated_subaccount: [u8; 32],
        #[ethevent(indexed)]
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub parent_subaccount: [u8; 32],
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
    #[ethevent(name = "FeeTierUpdate", abi = "FeeTierUpdate(address,uint32)")]
    pub struct FeeTierUpdateFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub fee_tier: u32,
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
        name = "FillOrder",
        abi = "FillOrder(uint32,bytes32,bytes32,int128,int128,uint64,uint64,uint128,bool,bool,int128,int128,int128)"
    )]
    pub struct FillOrderFilter {
        #[ethevent(indexed)]
        pub product_id: u32,
        #[ethevent(indexed)]
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub digest: [u8; 32],
        #[ethevent(indexed)]
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub appendix: u128,
        pub isolated: bool,
        pub is_taker: bool,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub fee_amount: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub base_delta: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub quote_delta: i128,
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
    pub enum OffchainExchangeEvents {
        CloseIsolatedSubaccountFilter(CloseIsolatedSubaccountFilter),
        FeeTierUpdateFilter(FeeTierUpdateFilter),
        FillOrderFilter(FillOrderFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for OffchainExchangeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CloseIsolatedSubaccountFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::CloseIsolatedSubaccountFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = FeeTierUpdateFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::FeeTierUpdateFilter(decoded));
            }
            if let Ok(decoded) = FillOrderFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::FillOrderFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(OffchainExchangeEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OffchainExchangeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CloseIsolatedSubaccountFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeTierUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CloseIsolatedSubaccountFilter> for OffchainExchangeEvents {
        fn from(value: CloseIsolatedSubaccountFilter) -> Self {
            Self::CloseIsolatedSubaccountFilter(value)
        }
    }
    impl ::core::convert::From<FeeTierUpdateFilter> for OffchainExchangeEvents {
        fn from(value: FeeTierUpdateFilter) -> Self {
            Self::FeeTierUpdateFilter(value)
        }
    }
    impl ::core::convert::From<FillOrderFilter> for OffchainExchangeEvents {
        fn from(value: FillOrderFilter) -> Self {
            Self::FillOrderFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for OffchainExchangeEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for OffchainExchangeEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `assertProduct` function with signature `assertProduct(bytes)` and selector `0x84528093`
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
    #[ethcall(name = "assertProduct", abi = "assertProduct(bytes)")]
    pub struct AssertProductCall {
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes),address)` and selector `0x34f9a4a4`
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
        name = "createIsolatedSubaccount",
        abi = "createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes),address)"
    )]
    pub struct CreateIsolatedSubaccountCall {
        pub txn: CreateIsolatedSubaccount,
        pub linked_signer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `dropDigest` function with signature `dropDigest(bytes32)` and selector `0xe1e7188d`
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
    #[ethcall(name = "dropDigest", abi = "dropDigest(bytes32)")]
    pub struct DropDigestCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `dropOrder` function with signature `dropOrder(uint32,(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0xbf29174c`
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
        name = "dropOrder",
        abi = "dropOrder(uint32,(bytes32,int128,int128,uint64,uint64,uint128))"
    )]
    pub struct DropOrderCall {
        pub product_id: u32,
        pub order: Order,
    }
    ///Container type for all input parameters for the `dropOrderChecked` function with signature `dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0x8dc3f468`
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
        name = "dropOrderChecked",
        abi = "dropOrderChecked(uint32,(bytes32,int128,int128,uint64,uint64,uint128))"
    )]
    pub struct DropOrderCheckedCall {
        pub product_id: u32,
        pub order: Order,
    }
    ///Container type for all input parameters for the `dumpFees` function with signature `dumpFees()` and selector `0x707c8b58`
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
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    ///Container type for all input parameters for the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
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
    #[ethcall(name = "filledAmounts", abi = "filledAmounts(bytes32)")]
    pub struct FilledAmountsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `getAllFeeTiers` function with signature `getAllFeeTiers(address[])` and selector `0x1ed01dad`
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
    #[ethcall(name = "getAllFeeTiers", abi = "getAllFeeTiers(address[])")]
    pub struct GetAllFeeTiersCall {
        pub users: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `0xff0be9ef`
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
    #[ethcall(name = "getCollectedFees", abi = "getCollectedFees(uint32)")]
    pub struct GetCollectedFeesCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `0x3fceea28`
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
        name = "getCustomFeeAddresses",
        abi = "getCustomFeeAddresses(uint32,uint32)"
    )]
    pub struct GetCustomFeeAddressesCall {
        pub start_at: u32,
        pub limit: u32,
    }
    ///Container type for all input parameters for the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0xb56b6a6c`
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
        name = "getDigest",
        abi = "getDigest(uint32,(bytes32,int128,int128,uint64,uint64,uint128))"
    )]
    pub struct GetDigestCall {
        pub product_id: u32,
        pub order: Order,
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
    ///Container type for all input parameters for the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `0xb5cbd70e`
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
        name = "getFeeFractionX18",
        abi = "getFeeFractionX18(bytes32,uint32,bool)"
    )]
    pub struct GetFeeFractionX18Call {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub taker: bool,
    }
    ///Container type for all input parameters for the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `0x0f2c878e`
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
    #[ethcall(name = "getFeeRatesX18", abi = "getFeeRatesX18(bytes32,uint32)")]
    pub struct GetFeeRatesX18Call {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getIsolatedSubaccountByDigest` function with signature `getIsolatedSubaccountByDigest(bytes32)` and selector `0x2a6b3ffe`
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
        name = "getIsolatedSubaccountByDigest",
        abi = "getIsolatedSubaccountByDigest(bytes32)"
    )]
    pub struct GetIsolatedSubaccountByDigestCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `getIsolatedSubaccounts` function with signature `getIsolatedSubaccounts(bytes32)` and selector `0xedc6d37b`
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
        name = "getIsolatedSubaccounts",
        abi = "getIsolatedSubaccounts(bytes32)"
    )]
    pub struct GetIsolatedSubaccountsCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getMarginByDigest` function with signature `getMarginByDigest(bytes32)` and selector `0x6ac3ee0b`
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
    #[ethcall(name = "getMarginByDigest", abi = "getMarginByDigest(bytes32)")]
    pub struct GetMarginByDigestCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub digest: [u8; 32],
    }
    ///Container type for all input parameters for the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `0x1d029b4d`
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
    #[ethcall(name = "getMarketInfo", abi = "getMarketInfo(uint32)")]
    pub struct GetMarketInfoCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getMinSize` function with signature `getMinSize(uint32)` and selector `0xb60aaa7c`
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
    #[ethcall(name = "getMinSize", abi = "getMinSize(uint32)")]
    pub struct GetMinSizeCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64,uint128),(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0x52699960`
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
        name = "getOrderFilledAmounts",
        abi = "getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64,uint128),(bytes32,int128,int128,uint64,uint64,uint128))"
    )]
    pub struct GetOrderFilledAmountsCall {
        pub product_id: u32,
        pub order_1: Order,
        pub order_2: Order,
    }
    ///Container type for all input parameters for the `getParentSubaccount` function with signature `getParentSubaccount(bytes32)` and selector `0x13b56ddb`
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
    #[ethcall(name = "getParentSubaccount", abi = "getParentSubaccount(bytes32)")]
    pub struct GetParentSubaccountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getRawMarketInfo` function with signature `getRawMarketInfo(uint32)` and selector `0x3eb0f4b3`
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
    #[ethcall(name = "getRawMarketInfo", abi = "getRawMarketInfo(uint32)")]
    pub struct GetRawMarketInfoCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `0xf2b26331`
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
    #[ethcall(name = "getSizeIncrement", abi = "getSizeIncrement(uint32)")]
    pub struct GetSizeIncrementCall {
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
    ///Container type for all input parameters for the `getTierFeeRateX18` function with signature `getTierFeeRateX18(uint32,uint32)` and selector `0x6baa1b83`
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
    #[ethcall(name = "getTierFeeRateX18", abi = "getTierFeeRateX18(uint32,uint32)")]
    pub struct GetTierFeeRateX18Call {
        pub tier: u32,
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `incrementFees` function with signature `incrementFees(uint32,int128,int128)` and selector `0x1f4ce016`
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
    #[ethcall(name = "incrementFees", abi = "incrementFees(uint32,int128,int128)")]
    pub struct IncrementFeesCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub maker_fee: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_fee: i128,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isIsolatedSubaccountActive` function with signature `isIsolatedSubaccountActive(bytes32,bytes32)` and selector `0x1a2b2d16`
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
        name = "isIsolatedSubaccountActive",
        abi = "isIsolatedSubaccountActive(bytes32,bytes32)"
    )]
    pub struct IsIsolatedSubaccountActiveCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub parent: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),address,address,int128))` and selector `0x1ab0a588`
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
        name = "matchOrders",
        abi = "matchOrders(((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),address,address,int128))"
    )]
    pub struct MatchOrdersCall {
        pub txn: MatchOrdersWithSigner,
    }
    ///Container type for all input parameters for the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `0x35ed4e6d`
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
        name = "modifyFilledAmount",
        abi = "modifyFilledAmount(bytes32,bytes32,int128)"
    )]
    pub struct ModifyFilledAmountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub taker_digest: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub maker_digest: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_delta: i128,
    }
    ///Container type for all input parameters for the `orderVersion` function with signature `orderVersion()` and selector `0x01e2abd5`
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
    #[ethcall(name = "orderVersion", abi = "orderVersion()")]
    pub struct OrderVersionCall;
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
    ///Container type for all input parameters for the `setFeeTier` function with signature `setFeeTier(address,uint32)` and selector `0x2bd28617`
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
    #[ethcall(name = "setFeeTier", abi = "setFeeTier(address,uint32)")]
    pub struct SetFeeTierCall {
        pub user: ::ethers::core::types::Address,
        pub tier: u32,
    }
    ///Container type for all input parameters for the `setFilledAmount` function with signature `setFilledAmount(bytes32,int128)` and selector `0xde1078bd`
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
    #[ethcall(name = "setFilledAmount", abi = "setFilledAmount(bytes32,int128)")]
    pub struct SetFilledAmountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub digest: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub filled_amount: i128,
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
    ///Container type for all input parameters for the `tryCloseIsolatedSubaccount` function with signature `tryCloseIsolatedSubaccount(bytes32)` and selector `0xf6ee7b4b`
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
        name = "tryCloseIsolatedSubaccount",
        abi = "tryCloseIsolatedSubaccount(bytes32)"
    )]
    pub struct TryCloseIsolatedSubaccountCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `updateCollectedFees` function with signature `updateCollectedFees(uint32,int128)` and selector `0x812609f1`
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
        name = "updateCollectedFees",
        abi = "updateCollectedFees(uint32,int128)"
    )]
    pub struct UpdateCollectedFeesCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_fees: i128,
    }
    ///Container type for all input parameters for the `updateFeeTier` function with signature `updateFeeTier(address,uint32)` and selector `0x385d448d`
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
    #[ethcall(name = "updateFeeTier", abi = "updateFeeTier(address,uint32)")]
    pub struct UpdateFeeTierCall {
        pub user: ::ethers::core::types::Address,
        pub new_tier: u32,
    }
    ///Container type for all input parameters for the `updateMarket` function with signature `updateMarket(uint32,uint32,int128,int128)` and selector `0xc8d6dbcb`
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
        name = "updateMarket",
        abi = "updateMarket(uint32,uint32,int128,int128)"
    )]
    pub struct UpdateMarketCall {
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
    }
    ///Container type for all input parameters for the `updateTierFeeRates` function with signature `updateTierFeeRates((uint32,uint32,int128,int128))` and selector `0xc71ede60`
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
        name = "updateTierFeeRates",
        abi = "updateTierFeeRates((uint32,uint32,int128,int128))"
    )]
    pub struct UpdateTierFeeRatesCall {
        pub txn: UpdateTierFeeRates,
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
    pub enum OffchainExchangeCalls {
        AssertProduct(AssertProductCall),
        CreateIsolatedSubaccount(CreateIsolatedSubaccountCall),
        DropDigest(DropDigestCall),
        DropOrder(DropOrderCall),
        DropOrderChecked(DropOrderCheckedCall),
        DumpFees(DumpFeesCall),
        FilledAmounts(FilledAmountsCall),
        GetAllFeeTiers(GetAllFeeTiersCall),
        GetCollectedFees(GetCollectedFeesCall),
        GetCustomFeeAddresses(GetCustomFeeAddressesCall),
        GetDigest(GetDigestCall),
        GetEndpoint(GetEndpointCall),
        GetFeeFractionX18(GetFeeFractionX18Call),
        GetFeeRatesX18(GetFeeRatesX18Call),
        GetIsolatedSubaccountByDigest(GetIsolatedSubaccountByDigestCall),
        GetIsolatedSubaccounts(GetIsolatedSubaccountsCall),
        GetMarginByDigest(GetMarginByDigestCall),
        GetMarketInfo(GetMarketInfoCall),
        GetMinSize(GetMinSizeCall),
        GetOrderFilledAmounts(GetOrderFilledAmountsCall),
        GetParentSubaccount(GetParentSubaccountCall),
        GetRawMarketInfo(GetRawMarketInfoCall),
        GetSizeIncrement(GetSizeIncrementCall),
        GetSlots(GetSlotsCall),
        GetTierFeeRateX18(GetTierFeeRateX18Call),
        IncrementFees(IncrementFeesCall),
        Initialize(InitializeCall),
        IsIsolatedSubaccountActive(IsIsolatedSubaccountActiveCall),
        MatchOrders(MatchOrdersCall),
        ModifyFilledAmount(ModifyFilledAmountCall),
        OrderVersion(OrderVersionCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetFeeTier(SetFeeTierCall),
        SetFilledAmount(SetFilledAmountCall),
        TransferOwnership(TransferOwnershipCall),
        TryCloseIsolatedSubaccount(TryCloseIsolatedSubaccountCall),
        UpdateCollectedFees(UpdateCollectedFeesCall),
        UpdateFeeTier(UpdateFeeTierCall),
        UpdateMarket(UpdateMarketCall),
        UpdateTierFeeRates(UpdateTierFeeRatesCall),
    }
    impl ::ethers::core::abi::AbiDecode for OffchainExchangeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssertProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssertProduct(decoded));
            }
            if let Ok(decoded) =
                <CreateIsolatedSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateIsolatedSubaccount(decoded));
            }
            if let Ok(decoded) = <DropDigestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropDigest(decoded));
            }
            if let Ok(decoded) = <DropOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DropOrder(decoded));
            }
            if let Ok(decoded) =
                <DropOrderCheckedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DropOrderChecked(decoded));
            }
            if let Ok(decoded) = <DumpFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DumpFees(decoded));
            }
            if let Ok(decoded) = <FilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetAllFeeTiersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllFeeTiers(decoded));
            }
            if let Ok(decoded) =
                <GetCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCollectedFees(decoded));
            }
            if let Ok(decoded) =
                <GetCustomFeeAddressesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCustomFeeAddresses(decoded));
            }
            if let Ok(decoded) = <GetDigestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDigest(decoded));
            }
            if let Ok(decoded) = <GetEndpointCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetEndpoint(decoded));
            }
            if let Ok(decoded) =
                <GetFeeFractionX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeeFractionX18(decoded));
            }
            if let Ok(decoded) =
                <GetFeeRatesX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetFeeRatesX18(decoded));
            }
            if let Ok(decoded) =
                <GetIsolatedSubaccountByDigestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetIsolatedSubaccountByDigest(decoded));
            }
            if let Ok(decoded) =
                <GetIsolatedSubaccountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetIsolatedSubaccounts(decoded));
            }
            if let Ok(decoded) =
                <GetMarginByDigestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMarginByDigest(decoded));
            }
            if let Ok(decoded) = <GetMarketInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMarketInfo(decoded));
            }
            if let Ok(decoded) = <GetMinSizeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMinSize(decoded));
            }
            if let Ok(decoded) =
                <GetOrderFilledAmountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOrderFilledAmounts(decoded));
            }
            if let Ok(decoded) =
                <GetParentSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetParentSubaccount(decoded));
            }
            if let Ok(decoded) =
                <GetRawMarketInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRawMarketInfo(decoded));
            }
            if let Ok(decoded) =
                <GetSizeIncrementCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSizeIncrement(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
            }
            if let Ok(decoded) =
                <GetTierFeeRateX18Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTierFeeRateX18(decoded));
            }
            if let Ok(decoded) = <IncrementFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementFees(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsIsolatedSubaccountActiveCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsIsolatedSubaccountActive(decoded));
            }
            if let Ok(decoded) = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <ModifyFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModifyFilledAmount(decoded));
            }
            if let Ok(decoded) = <OrderVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OrderVersion(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetFeeTierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeTier(decoded));
            }
            if let Ok(decoded) =
                <SetFilledAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFilledAmount(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TryCloseIsolatedSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TryCloseIsolatedSubaccount(decoded));
            }
            if let Ok(decoded) =
                <UpdateCollectedFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateCollectedFees(decoded));
            }
            if let Ok(decoded) = <UpdateFeeTierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeTier(decoded));
            }
            if let Ok(decoded) = <UpdateMarketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMarket(decoded));
            }
            if let Ok(decoded) =
                <UpdateTierFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateTierFeeRates(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OffchainExchangeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssertProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateIsolatedSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DropDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DropOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DropOrderChecked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DumpFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FilledAmounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllFeeTiers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCollectedFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCustomFeeAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEndpoint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeFractionX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetFeeRatesX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetIsolatedSubaccountByDigest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetIsolatedSubaccounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarginByDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMarketInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMinSize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOrderFilledAmounts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetParentSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRawMarketInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSizeIncrement(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTierFeeRateX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsIsolatedSubaccountActive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ModifyFilledAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeTier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFilledAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TryCloseIsolatedSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateCollectedFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeTier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateTierFeeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OffchainExchangeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssertProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateIsolatedSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::DropOrderChecked(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FilledAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllFeeTiers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCollectedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCustomFeeAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEndpoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeFractionX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFeeRatesX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetIsolatedSubaccountByDigest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetIsolatedSubaccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarginByDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarketInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMinSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderFilledAmounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetParentSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRawMarketInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSizeIncrement(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTierFeeRateX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsIsolatedSubaccountActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyFilledAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeTier(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFilledAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TryCloseIsolatedSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateCollectedFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeTier(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateTierFeeRates(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssertProductCall> for OffchainExchangeCalls {
        fn from(value: AssertProductCall) -> Self {
            Self::AssertProduct(value)
        }
    }
    impl ::core::convert::From<CreateIsolatedSubaccountCall> for OffchainExchangeCalls {
        fn from(value: CreateIsolatedSubaccountCall) -> Self {
            Self::CreateIsolatedSubaccount(value)
        }
    }
    impl ::core::convert::From<DropDigestCall> for OffchainExchangeCalls {
        fn from(value: DropDigestCall) -> Self {
            Self::DropDigest(value)
        }
    }
    impl ::core::convert::From<DropOrderCall> for OffchainExchangeCalls {
        fn from(value: DropOrderCall) -> Self {
            Self::DropOrder(value)
        }
    }
    impl ::core::convert::From<DropOrderCheckedCall> for OffchainExchangeCalls {
        fn from(value: DropOrderCheckedCall) -> Self {
            Self::DropOrderChecked(value)
        }
    }
    impl ::core::convert::From<DumpFeesCall> for OffchainExchangeCalls {
        fn from(value: DumpFeesCall) -> Self {
            Self::DumpFees(value)
        }
    }
    impl ::core::convert::From<FilledAmountsCall> for OffchainExchangeCalls {
        fn from(value: FilledAmountsCall) -> Self {
            Self::FilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetAllFeeTiersCall> for OffchainExchangeCalls {
        fn from(value: GetAllFeeTiersCall) -> Self {
            Self::GetAllFeeTiers(value)
        }
    }
    impl ::core::convert::From<GetCollectedFeesCall> for OffchainExchangeCalls {
        fn from(value: GetCollectedFeesCall) -> Self {
            Self::GetCollectedFees(value)
        }
    }
    impl ::core::convert::From<GetCustomFeeAddressesCall> for OffchainExchangeCalls {
        fn from(value: GetCustomFeeAddressesCall) -> Self {
            Self::GetCustomFeeAddresses(value)
        }
    }
    impl ::core::convert::From<GetDigestCall> for OffchainExchangeCalls {
        fn from(value: GetDigestCall) -> Self {
            Self::GetDigest(value)
        }
    }
    impl ::core::convert::From<GetEndpointCall> for OffchainExchangeCalls {
        fn from(value: GetEndpointCall) -> Self {
            Self::GetEndpoint(value)
        }
    }
    impl ::core::convert::From<GetFeeFractionX18Call> for OffchainExchangeCalls {
        fn from(value: GetFeeFractionX18Call) -> Self {
            Self::GetFeeFractionX18(value)
        }
    }
    impl ::core::convert::From<GetFeeRatesX18Call> for OffchainExchangeCalls {
        fn from(value: GetFeeRatesX18Call) -> Self {
            Self::GetFeeRatesX18(value)
        }
    }
    impl ::core::convert::From<GetIsolatedSubaccountByDigestCall> for OffchainExchangeCalls {
        fn from(value: GetIsolatedSubaccountByDigestCall) -> Self {
            Self::GetIsolatedSubaccountByDigest(value)
        }
    }
    impl ::core::convert::From<GetIsolatedSubaccountsCall> for OffchainExchangeCalls {
        fn from(value: GetIsolatedSubaccountsCall) -> Self {
            Self::GetIsolatedSubaccounts(value)
        }
    }
    impl ::core::convert::From<GetMarginByDigestCall> for OffchainExchangeCalls {
        fn from(value: GetMarginByDigestCall) -> Self {
            Self::GetMarginByDigest(value)
        }
    }
    impl ::core::convert::From<GetMarketInfoCall> for OffchainExchangeCalls {
        fn from(value: GetMarketInfoCall) -> Self {
            Self::GetMarketInfo(value)
        }
    }
    impl ::core::convert::From<GetMinSizeCall> for OffchainExchangeCalls {
        fn from(value: GetMinSizeCall) -> Self {
            Self::GetMinSize(value)
        }
    }
    impl ::core::convert::From<GetOrderFilledAmountsCall> for OffchainExchangeCalls {
        fn from(value: GetOrderFilledAmountsCall) -> Self {
            Self::GetOrderFilledAmounts(value)
        }
    }
    impl ::core::convert::From<GetParentSubaccountCall> for OffchainExchangeCalls {
        fn from(value: GetParentSubaccountCall) -> Self {
            Self::GetParentSubaccount(value)
        }
    }
    impl ::core::convert::From<GetRawMarketInfoCall> for OffchainExchangeCalls {
        fn from(value: GetRawMarketInfoCall) -> Self {
            Self::GetRawMarketInfo(value)
        }
    }
    impl ::core::convert::From<GetSizeIncrementCall> for OffchainExchangeCalls {
        fn from(value: GetSizeIncrementCall) -> Self {
            Self::GetSizeIncrement(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for OffchainExchangeCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
        }
    }
    impl ::core::convert::From<GetTierFeeRateX18Call> for OffchainExchangeCalls {
        fn from(value: GetTierFeeRateX18Call) -> Self {
            Self::GetTierFeeRateX18(value)
        }
    }
    impl ::core::convert::From<IncrementFeesCall> for OffchainExchangeCalls {
        fn from(value: IncrementFeesCall) -> Self {
            Self::IncrementFees(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for OffchainExchangeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsIsolatedSubaccountActiveCall> for OffchainExchangeCalls {
        fn from(value: IsIsolatedSubaccountActiveCall) -> Self {
            Self::IsIsolatedSubaccountActive(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for OffchainExchangeCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<ModifyFilledAmountCall> for OffchainExchangeCalls {
        fn from(value: ModifyFilledAmountCall) -> Self {
            Self::ModifyFilledAmount(value)
        }
    }
    impl ::core::convert::From<OrderVersionCall> for OffchainExchangeCalls {
        fn from(value: OrderVersionCall) -> Self {
            Self::OrderVersion(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for OffchainExchangeCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for OffchainExchangeCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetFeeTierCall> for OffchainExchangeCalls {
        fn from(value: SetFeeTierCall) -> Self {
            Self::SetFeeTier(value)
        }
    }
    impl ::core::convert::From<SetFilledAmountCall> for OffchainExchangeCalls {
        fn from(value: SetFilledAmountCall) -> Self {
            Self::SetFilledAmount(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for OffchainExchangeCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TryCloseIsolatedSubaccountCall> for OffchainExchangeCalls {
        fn from(value: TryCloseIsolatedSubaccountCall) -> Self {
            Self::TryCloseIsolatedSubaccount(value)
        }
    }
    impl ::core::convert::From<UpdateCollectedFeesCall> for OffchainExchangeCalls {
        fn from(value: UpdateCollectedFeesCall) -> Self {
            Self::UpdateCollectedFees(value)
        }
    }
    impl ::core::convert::From<UpdateFeeTierCall> for OffchainExchangeCalls {
        fn from(value: UpdateFeeTierCall) -> Self {
            Self::UpdateFeeTier(value)
        }
    }
    impl ::core::convert::From<UpdateMarketCall> for OffchainExchangeCalls {
        fn from(value: UpdateMarketCall) -> Self {
            Self::UpdateMarket(value)
        }
    }
    impl ::core::convert::From<UpdateTierFeeRatesCall> for OffchainExchangeCalls {
        fn from(value: UpdateTierFeeRatesCall) -> Self {
            Self::UpdateTierFeeRates(value)
        }
    }
    ///Container type for all return fields from the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes),address)` and selector `0x34f9a4a4`
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
    pub struct CreateIsolatedSubaccountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `filledAmounts` function with signature `filledAmounts(bytes32)` and selector `0x40f1a34d`
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
    pub struct FilledAmountsReturn(pub i128);
    ///Container type for all return fields from the `getAllFeeTiers` function with signature `getAllFeeTiers(address[])` and selector `0x1ed01dad`
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
    pub struct GetAllFeeTiersReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getCollectedFees` function with signature `getCollectedFees(uint32)` and selector `0xff0be9ef`
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
    pub struct GetCollectedFeesReturn {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_maker_fees: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_taker_fees: i128,
    }
    ///Container type for all return fields from the `getCustomFeeAddresses` function with signature `getCustomFeeAddresses(uint32,uint32)` and selector `0x3fceea28`
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
    pub struct GetCustomFeeAddressesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getDigest` function with signature `getDigest(uint32,(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0xb56b6a6c`
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
    pub struct GetDigestReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `getFeeFractionX18` function with signature `getFeeFractionX18(bytes32,uint32,bool)` and selector `0xb5cbd70e`
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
    pub struct GetFeeFractionX18Return(pub i128);
    ///Container type for all return fields from the `getFeeRatesX18` function with signature `getFeeRatesX18(bytes32,uint32)` and selector `0x0f2c878e`
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
    pub struct GetFeeRatesX18Return(pub i128, pub i128);
    ///Container type for all return fields from the `getIsolatedSubaccountByDigest` function with signature `getIsolatedSubaccountByDigest(bytes32)` and selector `0x2a6b3ffe`
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
    pub struct GetIsolatedSubaccountByDigestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getIsolatedSubaccounts` function with signature `getIsolatedSubaccounts(bytes32)` and selector `0xedc6d37b`
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
    pub struct GetIsolatedSubaccountsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `getMarginByDigest` function with signature `getMarginByDigest(bytes32)` and selector `0x6ac3ee0b`
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
    pub struct GetMarginByDigestReturn(pub i128);
    ///Container type for all return fields from the `getMarketInfo` function with signature `getMarketInfo(uint32)` and selector `0x1d029b4d`
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
    pub struct GetMarketInfoReturn {
        pub m: MarketInfo,
    }
    ///Container type for all return fields from the `getMinSize` function with signature `getMinSize(uint32)` and selector `0xb60aaa7c`
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
    pub struct GetMinSizeReturn(pub i128);
    ///Container type for all return fields from the `getOrderFilledAmounts` function with signature `getOrderFilledAmounts(uint32,(bytes32,int128,int128,uint64,uint64,uint128),(bytes32,int128,int128,uint64,uint64,uint128))` and selector `0x52699960`
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
    pub struct GetOrderFilledAmountsReturn(pub i128, pub i128);
    ///Container type for all return fields from the `getParentSubaccount` function with signature `getParentSubaccount(bytes32)` and selector `0x13b56ddb`
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
    pub struct GetParentSubaccountReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRawMarketInfo` function with signature `getRawMarketInfo(uint32)` and selector `0x3eb0f4b3`
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
    pub struct GetRawMarketInfoReturn(pub MarketInfoStore);
    ///Container type for all return fields from the `getSizeIncrement` function with signature `getSizeIncrement(uint32)` and selector `0xf2b26331`
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
    pub struct GetSizeIncrementReturn(pub i128);
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
        pub filled_amounts_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub taker_fees_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub maker_fees_slot: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub market_info_slot: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getTierFeeRateX18` function with signature `getTierFeeRateX18(uint32,uint32)` and selector `0x6baa1b83`
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
    pub struct GetTierFeeRateX18Return(pub FeeRates);
    ///Container type for all return fields from the `isIsolatedSubaccountActive` function with signature `isIsolatedSubaccountActive(bytes32,bytes32)` and selector `0x1a2b2d16`
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
    pub struct IsIsolatedSubaccountActiveReturn(pub bool);
    ///Container type for all return fields from the `modifyFilledAmount` function with signature `modifyFilledAmount(bytes32,bytes32,int128)` and selector `0x35ed4e6d`
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
    pub struct ModifyFilledAmountReturn(pub i128, pub i128);
    ///Container type for all return fields from the `orderVersion` function with signature `orderVersion()` and selector `0x01e2abd5`
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
    pub struct OrderVersionReturn(pub u128);
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
    ///`CreateIsolatedSubaccount((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes)`
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
    pub struct CreateIsolatedSubaccount {
        pub order: Order,
        pub product_id: u32,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`MatchOrders(uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes))`
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
    pub struct MatchOrders {
        pub product_id: u32,
        pub taker: SignedOrder,
        pub maker: SignedOrder,
    }
    ///`MatchOrdersWithSigner((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),address,address,int128)`
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
    pub struct MatchOrdersWithSigner {
        pub match_orders: MatchOrders,
        pub taker_linked_signer: ::ethers::core::types::Address,
        pub maker_linked_signer: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_amount_delta: i128,
    }
    ///`Order(bytes32,int128,int128,uint64,uint64,uint128)`
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
    pub struct Order {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub appendix: u128,
    }
    ///`SignedOrder((bytes32,int128,int128,uint64,uint64,uint128),bytes)`
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
    pub struct SignedOrder {
        pub order: Order,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`UpdateTierFeeRates(uint32,uint32,int128,int128)`
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
    pub struct UpdateTierFeeRates {
        pub tier: u32,
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub maker_rate_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_rate_x18: i128,
    }
    ///`FeeRates(int128,int128)`
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
    pub struct FeeRates {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub maker_rate_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_rate_x18: i128,
    }
    ///`MarketInfo(uint32,int128,int128,int128)`
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
    pub struct MarketInfo {
        pub quote_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_fees: i128,
    }
    ///`MarketInfoStore(int64,int64,int128)`
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
    pub struct MarketInfoStore {
        pub min_size: i64,
        pub size_increment: i64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_fees: i128,
    }
}
