pub use querier::*;
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
pub mod querier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAllProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllProducts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.ProductInfo",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBookInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBookInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("bookInfo"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.BookInfo"),
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
                    ::std::borrow::ToOwned::to_owned("getInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInsurance"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getNlpPoolInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNlpPoolInfo"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.NlpPoolInfo",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPerpBalance"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.PerpBalance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPerpBalances"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("perpBalances"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.PerpBalance[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPerpProduct"),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.PerpProduct",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPerpProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPerpProducts"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("perpProducts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.PerpProduct[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotBalance"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.SpotBalance",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotBalances"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotBalances"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spotBalances"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.SpotBalance[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotProduct"),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
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
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.SpotProduct",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotProducts"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spotProducts"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
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
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.SpotProduct[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubaccountInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubaccountInfo"),
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
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Int(
                                                    128usize
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Int(
                                                        128usize
                                                    ),
                                                ],
                                            ),
                                        ],),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct FQuerier.SubaccountInfo",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QUERIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa6\xAC\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c]p.\x1A\x11a\0\x97W\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02HW\x80c\xC4\xD6m\xE8\x14a\x02cW\x80c\xD7\xB2)\xB6\x14a\x02xW\x80c\xEE\x99(\xC9\x14a\x02\x98W`\0\x80\xFD[\x80c]p.\x1A\x14a\x01\xD3W\x80ct\x174\x04\x14a\x01\xF3W\x80cu\xA5\xAB<\x14a\x02\x13W\x80c\x97\x05\xF20\x14a\x023W`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xD3W\x80c%\x93\xEB_\x14a\x01XW\x80c&z\x8D\xA0\x14a\x01xW\x80c1TmQ\x14a\x01\x93W\x80cW#e?\x14a\x01\xB3W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xFAW\x80c\x02\xEE:R\x14a\x01#W\x80c\x1A\xE1\x0B\xC5\x14a\x018W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a%<V[a\x02\xB8V[`@Qa\x01\x1A\x91\x90a%YV[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x03\x98V[`@Qa\x01\x1A\x91\x90a(LV[a\x01Ka\x01F6`\x04a%<V[a\x03\xE6V[`@Qa\x01\x1A\x91\x90a(\x8EV[a\x01ka\x01f6`\x04a)\xA5V[a\x05_V[`@Qa\x01\x1A\x91\x90a*bV[a\x01\x80a\x06`V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x01\x1AV[a\x01\xA6a\x01\xA16`\x04a)\xA5V[a\x06\xDDV[`@Qa\x01\x1A\x91\x90a*\xC3V[a\x01\xC6a\x01\xC16`\x04a%<V[a\x07\xCAV[`@Qa\x01\x1A\x91\x90a*\xD6V[a\x01\xE6a\x01\xE16`\x04a*\xE5V[a\t\xA4V[`@Qa\x01\x1A\x91\x90a+\xD7V[a\x02\x06a\x02\x016`\x04a,\xDFV[a\x1AJV[`@Qa\x01\x1A\x91\x90a-\x0FV[a\x02&a\x02!6`\x04a-0V[a\x1B\x12V[`@Qa\x01\x1A\x91\x90a-mV[a\x02;a\x1B\xE6V[`@Qa\x01\x1A\x91\x90a-\xBCV[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x02va\x02q6`\x04a.\\V[a\x1D\xDBV[\0[a\x02\x8Ba\x02\x866`\x04a,\xDFV[a\x1F\xE1V[`@Qa\x01\x1A\x91\x90a.yV[a\x02\xABa\x02\xA66`\x04a-0V[a \x8EV[`@Qa\x01\x1A\x91\x90a.\x87V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\xE4a!\\V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x030W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03T\x91\x90a.\xB1V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03\xB7a!\xCAV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\xCF\x84a\x1B\x12V[\x81R` \x01a\x03\xDD\x83a \x8EV[\x90R\x93\x92PPPV[a\x03\xEEa$\x14V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04h\x91\x90a/\xFFV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE3\x91\x90a04V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05LW`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05UV[a\x05U\x86a\x02\xB8V[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05{Wa\x05{a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xDFW\x81` \x01[a\x05\xCC`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\x99W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06YW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\x0EWa\x06\x0Ea0\xC2V[` \x02` \x01\x01Q\x90Pa\x06\"\x85\x82a\x1F\xE1V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06:Wa\x06:a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06Q\x90a0\xEEV[\x91PPa\x05\xE5V[P\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c&z\x8D\xA0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a1\x11V[\x90P\x90V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF9Wa\x06\xF9a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07PW\x81` \x01[a\x07=`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x17W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06YW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x7FWa\x07\x7Fa0\xC2V[` \x02` \x01\x01Q\x90Pa\x07\x93\x85\x82a\x1AJV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xABWa\x07\xABa0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xC2\x90a0\xEEV[\x91PPa\x07VV[a\x07\xD2a$\xA4V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08L\x91\x90a1,V[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC7\x91\x90a04V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tb\x91\x90a1\x9EV[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05LW`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05UV[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\nOa!\xCAV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\ntWPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\x0EW\x81\x84\x82\x81Q\x81\x10a\n\xC5Wa\n\xC5a0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xFCW\x83\x81\x81Q\x81\x10a\n\xEBWa\n\xEBa0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\x06\x81a2PV[\x91PPa\n\xA9V[P`\0[\x82Q\x81\x10\x15a\x0BwW\x81\x83\x82\x81Q\x81\x10a\x0B.Wa\x0B.a0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0BeW\x82\x81\x81Q\x81\x10a\x0BTWa\x0BTa0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0Bo\x81a2PV[\x91PPa\x0B\x12V[Pa\x0B\x83\x81`\x01a2iV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9BWa\x0B\x9Ba(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xCEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xB9W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C1W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\x13Wa\x0C\x13a0\xC2V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C)\x90a2PV[\x91PPa\x0B\xD7V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CMWa\x0CMa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xA4W\x81` \x01[a\x0C\x91`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0CkW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC4Wa\x0C\xC4a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r(W\x81` \x01[a\r\x15`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xE2W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rHWa\rHa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x81W\x81` \x01[a\rna$\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rfW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA2Wa\r\xA2a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xDBW\x81` \x01[a\r\xC8a$\x14V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xC0W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x1FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\x0EWa\x0E\x0Ea0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x98\x91\x90a1,V[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x15\x91\x90a04V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F[Wa\x0F[a0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x01\x91\x90a1\x9EV[\x81R` \x01\x85\x81R` \x01a\x10\x15\x87a\x02\xB8V[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x10-\x82a0\xEEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10RWa\x10Ra0\xC2V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x11\x08W\x83Q`\0\x90a\x10\xA1\x90\x84\x90a\x10\x98\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a2\x81V[a\"\xCBV[`\x0F\x0B\x90a#dV[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xC0Wa\x10\xC0a0\xC2V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x10\xDCWa\x10\xDCa0\xC2V[` \x02` \x01\x01\x81\x81Qa\x10\xF0\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x11\x01\x90P\x81a2\xE6V[\x90Pa\x10`V[PPPPPP\x80a\x11\x18\x90a0\xEEV[\x90Pa\r\xE5V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\xE9W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11LWa\x11La0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD6\x91\x90a/\xFFV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12S\x91\x90a04V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x99Wa\x12\x99a0\xC2V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x12\xD9\x87a\x02\xB8V[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12\xF1\x82a0\xEEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\x16Wa\x13\x16a0\xC2V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13\xD2W`\0\x84` \x01Qa\x13a\x84a\x10\x98\x88`\0\x01Qa\x10\x98\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a2\x81V[a\x13k\x91\x90a2\x97V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\x8AWa\x13\x8Aa0\xC2V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x13\xA6Wa\x13\xA6a0\xC2V[` \x02` \x01\x01\x81\x81Qa\x13\xBA\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x13\xCB\x90P\x81a2\xE6V[\x90Pa\x13$V[PPPPPP\x80a\x13\xE2\x90a0\xEEV[\x90Pa\x11#V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14b\x91\x90a3\x05V[\x90P[\x80\x15a\x17\xF2W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17\xEAW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x14\xB5Wa\x14\xB5a2\x81V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD3\x93\x92\x91\x90a3\x1EV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x16\x91\x90a3OV[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x15+WPa\x17\xD8V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x15\\Wa\x15\\a2\x81V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15z\x93\x92\x91\x90a3\x1EV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBD\x91\x90a3OV[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15\xE0WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15\xECWPPa\x17\xD8V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x16\x1BW\x81Q\x83Qa\x16\x14\x91\x90a\x16\x0F\x90a3kV[a#\xE3V[\x90Pa\x16>V[\x81Q\x83Qa\x162\x91\x90a\x16-\x90a3kV[a#\xFFV[a\x16;\x90a3kV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x16V\x91\x90a2\x97V[a\x16`\x91\x90a3\x91V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x16\xB0W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x8D\x91\x90a3\xE6V[a\x16\x97\x91\x90a3\x91V[a\x16\xA9\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\xE6V[\x90Pa\x16\xE9V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xCA\x91\x90a3\xE6V[a\x16\xD4\x91\x90a3\x91V[a\x16\xE6\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\xE6V[\x90P[`\0`\x02a\x17\x1Ca\x16\xFA\x85\x85a3\xE6V[a\x10\x98\x89` \x01Q\x89` \x01Qa\x17\x11\x91\x90a2\x97V[`\x0F\x89\x90\x0B\x90a#dV[a\x17&\x91\x90a3\x91V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x17EWa\x17Ea0\xC2V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17aWa\x17aa0\xC2V[` \x02` \x01\x01\x81\x81Qa\x17u\x91\x90a2\x97V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x17\x9BWa\x17\x9Ba0\xC2V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17\xB7Wa\x17\xB7a0\xC2V[` \x02` \x01\x01\x81\x81Qa\x17\xCB\x91\x90a2\x97V[`\x0F\x0B\x90RPPPPPPP[\x80a\x17\xE2\x81a2\xE6V[\x91PPa\x14~V[PPPa\x14eV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1A?W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19\x96W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18AWa\x18Aa0\xC2V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18cWa\x18ca0\xC2V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x18\xFFW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x93Wa\x18\x93a0\xC2V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a0\xC2V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xDCWa\x18\xDCa0\xC2V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18\xF4\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x19\x86V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x1EWa\x19\x1Ea0\xC2V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19@Wa\x19@a0\xC2V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19gWa\x19ga0\xC2V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x19\x7F\x91\x90a3\xE6V[`\x0F\x0B\x90RP[a\x19\x8F\x81a46V[\x90Pa\x18\nV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\xB6Wa\x19\xB6a0\xC2V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\xE1Wa\x19\xE1a0\xC2V[` \x02` \x01\x01Q`\0\x01Qa\x19\xF7\x91\x90a3\xE6V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\x16Wa\x1A\x16a0\xC2V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1A8\x90a46V[\x90Pa\x17\xF5V[P\x92\x95\x94PPPPPV[a\x1Ap`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEA\x91\x90a1,V[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B.Wa\x1B.a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1BgW\x81` \x01[a\x1BTa$\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1BLW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\xE0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x96Wa\x1B\x96a0\xC2V[` \x02` \x01\x01Q\x90Pa\x1B\xA9\x81a\x07\xCAV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\xC1Wa\x1B\xC1a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B\xD8\x90a0\xEEV[\x91PPa\x1BmV[P\x91\x90PV[`@\x80Q` \x81\x01\x90\x91R``\x81R`\x01T`@\x80Qc\x1C\x88m\x0B`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x1C\x88m\x0B\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1C>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Cf\x91\x90\x81\x01\x90a4RV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x84Wa\x1C\x84a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xD6W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x1C\xA2W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1D\xC6W`@Q\x80`\x80\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1D\x02Wa\x1D\x02a0\xC2V[` \x02` \x01\x01Q`\0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1D/Wa\x1D/a0\xC2V[` \x02` \x01\x01Q` \x01Q\x81R` \x01\x84\x83\x81Q\x81\x10a\x1DRWa\x1DRa0\xC2V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1D~Wa\x1D~a0\xC2V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1D\xA8Wa\x1D\xA8a0\xC2V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x1D\xBE\x90a2PV[\x91PPa\x1C\xDCV[P`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB6\x91\x90a5PV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1E\xF8\x91`\x04\x01a5mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F9\x91\x90a5PV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F}\x90`\x01\x90`\x04\x01a5mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBE\x91\x90a5PV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a \x14`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEA\x91\x90a/\xFFV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xAAWa \xAAa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xE3W\x81` \x01[a \xD0a$\x14V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xC8W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\xE0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\x12Wa!\x12a0\xC2V[` \x02` \x01\x01Q\x90Pa!%\x81a\x03\xE6V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!=Wa!=a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a!T\x90a0\xEEV[\x91PPa \xE9V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a5PV[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\" W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"H\x91\x90\x81\x01\x90a5\x87V[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\xC5\x91\x90\x81\x01\x90a5\x87V[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"\xE1Wa\"\xE1a2\x81V[\x03a\"\xF5WPg\r\xE0\xB6\xB3\xA7d\0\0a#]V[`\0\x80\x84`\x0F\x0B\x12a#.W`\0\x83`\x02\x81\x11\x15a#\x15Wa#\x15a2\x81V[\x14a#$W\x84`@\x01Qa#'V[\x84Q[\x90Pa#ZV[`\0\x83`\x02\x81\x11\x15a#BWa#Ba2\x81V[\x14a#QW\x84``\x01Qa#WV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xA2WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E0\x91\x90a6!V[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a#\xF8W\x81a#]V[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a#\xF8W\x81a#]V[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a$SV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a%9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a%NW`\0\x80\xFD[\x815a#]\x81a%'V[`\x80\x81\x01a\x1B\x0C\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa%\xF7`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa&Za\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa&oa\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa'\x12\x87\x83Qa%\x94V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a&\xFFV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa'\x94`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa'\xD4`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa(8\x87\x83Qa'1V[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a(%V[` \x81R`\0\x82Q`@` \x84\x01Ra(h``\x84\x01\x82a&\xEBV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra(\x85\x82\x82a(\x11V[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1B\x0C\x82\x84a'1V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(\xD6Wa(\xD6a(\x9DV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x05Wa)\x05a(\x9DV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)'Wa)'a(\x9DV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a)BW`\0\x80\xFD[\x815` a)Wa)R\x83a)\rV[a(\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)vW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a)\x9AW\x805a)\x8D\x81a%'V[\x83R\x91\x83\x01\x91\x83\x01a)zV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a)\xB8W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xD6W`\0\x80\xFD[a)\xE2\x85\x82\x86\x01a)1V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa&\xE6` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa*O\x87\x83Qa)\xECV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*<V[` \x81R`\0a#]` \x83\x01\x84a*(V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa*\xB0\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*\x89V[` \x81R`\0a#]` \x83\x01\x84a*uV[a\x02\xC0\x81\x01a\x1B\x0C\x82\x84a%\x94V[`\0` \x82\x84\x03\x12\x15a*\xF7W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa+E\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a+\x12V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a+\xC9W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a+\xB4W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a+\x95V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a+wV[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa+\xF7`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra,\x14a\x01`\x85\x01\x83a*\xFEV[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra,2\x84\x83a+XV[\x93P`\x80\x87\x01Q\x91Pa,M`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra,{\x84\x83a*uV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra,\x9A\x85\x84a*(V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra,\xB9\x85\x84a&\xEBV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa,\xD5\x83\x82a(\x11V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,\xF2W`\0\x80\xFD[\x825\x91P` \x83\x015a-\x04\x81a%'V[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1B\x0CV[`\0` \x82\x84\x03\x12\x15a-BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-YW`\0\x80\xFD[a-e\x84\x82\x85\x01a)1V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\xB0Wa-\x9C\x83\x85Qa%\x94V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a-\x89V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R`@\x80\x84\x01\x85Q\x83\x84\x87\x01R\x81\x81Q\x80\x84R``\x93P\x83\x88\x01\x91P\x85\x83\x01\x92P`\0[\x81\x81\x10\x15a.9W\x83Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x85\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x84\x01R\x92\x86\x01\x92`\x80\x90\x92\x01\x91`\x01\x01a-\xE6V[P\x90\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a.nW`\0\x80\xFD[\x815a#]\x81a.GV[`\x80\x81\x01a\x1B\x0C\x82\x84a)\xECV[` \x81R`\0a#]` \x83\x01\x84a(\x11V[\x80Q`\x0F\x81\x90\x0B\x81\x14a.\xACW`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a.\xC3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.\xE6Wa.\xE6a(\x9DV[`@R\x82Qa.\xF4\x81a%'V[\x81Ra/\x02` \x84\x01a.\x9AV[` \x82\x01Ra/\x13`@\x84\x01a.\x9AV[`@\x82\x01Ra/$``\x84\x01a.\x9AV[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a/BW`\0\x80\xFD[a/Ja(\xB3V[\x90Pa/U\x82a.\x9AV[\x81Ra/c` \x83\x01a.\x9AV[` \x82\x01Ra/t`@\x83\x01a.\x9AV[`@\x82\x01Ra/\x85``\x83\x01a.\x9AV[``\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a/\xA2W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\xC5Wa/\xC5a(\x9DV[`@R\x90P\x80a/\xD4\x83a.\x9AV[\x81Ra/\xE2` \x84\x01a.\x9AV[` \x82\x01Ra/\xF3`@\x84\x01a.\x9AV[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a0\x12W`\0\x80\xFD[a0\x1C\x84\x84a/0V[\x91Pa0+\x84`\x80\x85\x01a/\x90V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a0FW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0iWa0ia(\x9DV[`@Ra0u\x83a.\x9AV[\x81Ra0\x83` \x84\x01a.\x9AV[` \x82\x01Ra0\x94`@\x84\x01a.\x9AV[`@\x82\x01Ra0\xA5``\x84\x01a.\x9AV[``\x82\x01Ra0\xB6`\x80\x84\x01a.\x9AV[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a1\x07Wa1\x07a0\xD8V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a1#W`\0\x80\xFD[a#]\x82a.\x9AV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a1@W`\0\x80\xFD[a1J\x85\x85a/0V[\x92P` `\x7F\x19\x82\x01\x12\x15a1^W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\x82Wa1\x82a(\x9DV[`@Ra1\x91`\x80\x85\x01a.\x9AV[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a1\xB0W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\xD3Wa1\xD3a(\x9DV[`@R\x82Qa1\xE1\x81a.GV[\x81Ra1\xEF` \x84\x01a.\x9AV[` \x82\x01Ra2\0`@\x84\x01a.\x9AV[`@\x82\x01Ra2\x11``\x84\x01a.\x9AV[``\x82\x01Ra2\"`\x80\x84\x01a.\x9AV[`\x80\x82\x01Ra23`\xA0\x84\x01a.\x9AV[`\xA0\x82\x01Ra2D`\xC0\x84\x01a.\x9AV[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a2bWa2ba0\xD8V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a2|Wa2|a0\xD8V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a2\xC1Wa2\xC1a0\xD8V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a2\xDDWa2\xDDa0\xD8V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a2\xFCWa2\xFCa0\xD8V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\x17W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a3AWa3Aa2\x81V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a3aW`\0\x80\xFD[a#]\x83\x83a/\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a3\x88Wa3\x88a0\xD8V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a3\xB6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3\xDDWa3\xDDa0\xD8V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a4\x11Wa4\x11a0\xD8V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a4,Wa4,a0\xD8V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a1\x07Wa1\x07a0\xD8V[`\0` \x80\x83\x85\x03\x12\x15a4eW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4}W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a4\x91W`\0\x80\xFD[\x81Qa4\x9Fa)R\x82a)\rV[\x81\x81R`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a4\xBEW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a5DW`\x80\x85\x8A\x03\x12\x15a4\xDCW`\0\x80\x81\xFD[a4\xE4a(\xB3V[\x85Q\x85\x81\x16\x81\x14a4\xF5W`\0\x80\x81\xFD[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa5\x0E\x81a.GV[\x90\x82\x01R``\x86\x81\x01Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a5.W`\0\x80\x81\xFD[\x90\x82\x01R\x82R`\x80\x94\x90\x94\x01\x93\x90\x85\x01\x90a4\xC3V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a5bW`\0\x80\xFD[\x81Qa#]\x81a.GV[` \x81\x01`\x02\x83\x10a5\x81Wa5\x81a2\x81V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a5\x9AW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xB1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a5\xC2W`\0\x80\xFD[\x80Qa5\xD0a)R\x82a)\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a5\xEFW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\x16W\x83Qa6\x07\x81a%'V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a5\xF4V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6NW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a62V[\x81\x81\x11\x15a6`W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB1\xFCZX\x96MF\xD1f\x0B\xE7\x97\x11B\xBD\xD9\xA7\xB5\xC2\xE10\xAC?\xD1L&D\xA3\xFB!\xAA_dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c]p.\x1A\x11a\0\x97W\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02HW\x80c\xC4\xD6m\xE8\x14a\x02cW\x80c\xD7\xB2)\xB6\x14a\x02xW\x80c\xEE\x99(\xC9\x14a\x02\x98W`\0\x80\xFD[\x80c]p.\x1A\x14a\x01\xD3W\x80ct\x174\x04\x14a\x01\xF3W\x80cu\xA5\xAB<\x14a\x02\x13W\x80c\x97\x05\xF20\x14a\x023W`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xD3W\x80c%\x93\xEB_\x14a\x01XW\x80c&z\x8D\xA0\x14a\x01xW\x80c1TmQ\x14a\x01\x93W\x80cW#e?\x14a\x01\xB3W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xFAW\x80c\x02\xEE:R\x14a\x01#W\x80c\x1A\xE1\x0B\xC5\x14a\x018W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a%<V[a\x02\xB8V[`@Qa\x01\x1A\x91\x90a%YV[`@Q\x80\x91\x03\x90\xF3[a\x01+a\x03\x98V[`@Qa\x01\x1A\x91\x90a(LV[a\x01Ka\x01F6`\x04a%<V[a\x03\xE6V[`@Qa\x01\x1A\x91\x90a(\x8EV[a\x01ka\x01f6`\x04a)\xA5V[a\x05_V[`@Qa\x01\x1A\x91\x90a*bV[a\x01\x80a\x06`V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x01\x1AV[a\x01\xA6a\x01\xA16`\x04a)\xA5V[a\x06\xDDV[`@Qa\x01\x1A\x91\x90a*\xC3V[a\x01\xC6a\x01\xC16`\x04a%<V[a\x07\xCAV[`@Qa\x01\x1A\x91\x90a*\xD6V[a\x01\xE6a\x01\xE16`\x04a*\xE5V[a\t\xA4V[`@Qa\x01\x1A\x91\x90a+\xD7V[a\x02\x06a\x02\x016`\x04a,\xDFV[a\x1AJV[`@Qa\x01\x1A\x91\x90a-\x0FV[a\x02&a\x02!6`\x04a-0V[a\x1B\x12V[`@Qa\x01\x1A\x91\x90a-mV[a\x02;a\x1B\xE6V[`@Qa\x01\x1A\x91\x90a-\xBCV[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x02va\x02q6`\x04a.\\V[a\x1D\xDBV[\0[a\x02\x8Ba\x02\x866`\x04a,\xDFV[a\x1F\xE1V[`@Qa\x01\x1A\x91\x90a.yV[a\x02\xABa\x02\xA66`\x04a-0V[a \x8EV[`@Qa\x01\x1A\x91\x90a.\x87V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\xE4a!\\V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x030W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03T\x91\x90a.\xB1V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03\xB7a!\xCAV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\xCF\x84a\x1B\x12V[\x81R` \x01a\x03\xDD\x83a \x8EV[\x90R\x93\x92PPPV[a\x03\xEEa$\x14V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04h\x91\x90a/\xFFV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE3\x91\x90a04V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05LW`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05UV[a\x05U\x86a\x02\xB8V[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05{Wa\x05{a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xDFW\x81` \x01[a\x05\xCC`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\x99W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06YW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\x0EWa\x06\x0Ea0\xC2V[` \x02` \x01\x01Q\x90Pa\x06\"\x85\x82a\x1F\xE1V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06:Wa\x06:a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06Q\x90a0\xEEV[\x91PPa\x05\xE5V[P\x92\x91PPV[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c&z\x8D\xA0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a1\x11V[\x90P\x90V[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xF9Wa\x06\xF9a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07PW\x81` \x01[a\x07=`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\x17W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06YW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x7FWa\x07\x7Fa0\xC2V[` \x02` \x01\x01Q\x90Pa\x07\x93\x85\x82a\x1AJV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\xABWa\x07\xABa0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\xC2\x90a0\xEEV[\x91PPa\x07VV[a\x07\xD2a$\xA4V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08L\x91\x90a1,V[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC7\x91\x90a04V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tb\x91\x90a1\x9EV[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05LW`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05UV[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\nOa!\xCAV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\ntWPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\x0B\x0EW\x81\x84\x82\x81Q\x81\x10a\n\xC5Wa\n\xC5a0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xFCW\x83\x81\x81Q\x81\x10a\n\xEBWa\n\xEBa0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0B\x06\x81a2PV[\x91PPa\n\xA9V[P`\0[\x82Q\x81\x10\x15a\x0BwW\x81\x83\x82\x81Q\x81\x10a\x0B.Wa\x0B.a0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0BeW\x82\x81\x81Q\x81\x10a\x0BTWa\x0BTa0\xC2V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\x0Bo\x81a2PV[\x91PPa\x0B\x12V[Pa\x0B\x83\x81`\x01a2iV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9BWa\x0B\x9Ba(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xCEW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xB9W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0C1W`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0C\x13Wa\x0C\x13a0\xC2V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0C)\x90a2PV[\x91PPa\x0B\xD7V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CMWa\x0CMa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xA4W\x81` \x01[a\x0C\x91`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0CkW\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xC4Wa\x0C\xC4a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r(W\x81` \x01[a\r\x15`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xE2W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\rHWa\rHa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x81W\x81` \x01[a\rna$\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\rfW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xA2Wa\r\xA2a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xDBW\x81` \x01[a\r\xC8a$\x14V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\xC0W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x11\x1FW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\x0EWa\x0E\x0Ea0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EtW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x98\x91\x90a1,V[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x15\x91\x90a04V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F[Wa\x0F[a0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x01\x91\x90a1\x9EV[\x81R` \x01\x85\x81R` \x01a\x10\x15\x87a\x02\xB8V[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x10-\x82a0\xEEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10RWa\x10Ra0\xC2V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x11\x08W\x83Q`\0\x90a\x10\xA1\x90\x84\x90a\x10\x98\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a2\x81V[a\"\xCBV[`\x0F\x0B\x90a#dV[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xC0Wa\x10\xC0a0\xC2V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x10\xDCWa\x10\xDCa0\xC2V[` \x02` \x01\x01\x81\x81Qa\x10\xF0\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x11\x01\x90P\x81a2\xE6V[\x90Pa\x10`V[PPPPPP\x80a\x11\x18\x90a0\xEEV[\x90Pa\r\xE5V[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\xE9W`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11LWa\x11La0\xC2V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD6\x91\x90a/\xFFV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12S\x91\x90a04V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\x99Wa\x12\x99a0\xC2V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x12\xD9\x87a\x02\xB8V[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12\xF1\x82a0\xEEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\x16Wa\x13\x16a0\xC2V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13\xD2W`\0\x84` \x01Qa\x13a\x84a\x10\x98\x88`\0\x01Qa\x10\x98\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a2\x81V[a\x13k\x91\x90a2\x97V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x13\x8AWa\x13\x8Aa0\xC2V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x13\xA6Wa\x13\xA6a0\xC2V[` \x02` \x01\x01\x81\x81Qa\x13\xBA\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x13\xCB\x90P\x81a2\xE6V[\x90Pa\x13$V[PPPPPP\x80a\x13\xE2\x90a0\xEEV[\x90Pa\x11#V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14b\x91\x90a3\x05V[\x90P[\x80\x15a\x17\xF2W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17\xEAW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x14\xB5Wa\x14\xB5a2\x81V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD3\x93\x92\x91\x90a3\x1EV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x16\x91\x90a3OV[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x15+WPa\x17\xD8V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x15\\Wa\x15\\a2\x81V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15z\x93\x92\x91\x90a3\x1EV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x15\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBD\x91\x90a3OV[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15\xE0WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15\xECWPPa\x17\xD8V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x16\x1BW\x81Q\x83Qa\x16\x14\x91\x90a\x16\x0F\x90a3kV[a#\xE3V[\x90Pa\x16>V[\x81Q\x83Qa\x162\x91\x90a\x16-\x90a3kV[a#\xFFV[a\x16;\x90a3kV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x16V\x91\x90a2\x97V[a\x16`\x91\x90a3\x91V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x16\xB0W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x8D\x91\x90a3\xE6V[a\x16\x97\x91\x90a3\x91V[a\x16\xA9\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\xE6V[\x90Pa\x16\xE9V[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xCA\x91\x90a3\xE6V[a\x16\xD4\x91\x90a3\x91V[a\x16\xE6\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\xE6V[\x90P[`\0`\x02a\x17\x1Ca\x16\xFA\x85\x85a3\xE6V[a\x10\x98\x89` \x01Q\x89` \x01Qa\x17\x11\x91\x90a2\x97V[`\x0F\x89\x90\x0B\x90a#dV[a\x17&\x91\x90a3\x91V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x17EWa\x17Ea0\xC2V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17aWa\x17aa0\xC2V[` \x02` \x01\x01\x81\x81Qa\x17u\x91\x90a2\x97V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x17\x9BWa\x17\x9Ba0\xC2V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17\xB7Wa\x17\xB7a0\xC2V[` \x02` \x01\x01\x81\x81Qa\x17\xCB\x91\x90a2\x97V[`\x0F\x0B\x90RPPPPPPP[\x80a\x17\xE2\x81a2\xE6V[\x91PPa\x14~V[PPPa\x14eV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x1A?W`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19\x96W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18AWa\x18Aa0\xC2V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18cWa\x18ca0\xC2V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x18\xFFW\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x93Wa\x18\x93a0\xC2V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a0\xC2V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xDCWa\x18\xDCa0\xC2V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18\xF4\x91\x90a2\x97V[`\x0F\x0B\x90RPa\x19\x86V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x1EWa\x19\x1Ea0\xC2V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19@Wa\x19@a0\xC2V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19gWa\x19ga0\xC2V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x19\x7F\x91\x90a3\xE6V[`\x0F\x0B\x90RP[a\x19\x8F\x81a46V[\x90Pa\x18\nV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\xB6Wa\x19\xB6a0\xC2V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\xE1Wa\x19\xE1a0\xC2V[` \x02` \x01\x01Q`\0\x01Qa\x19\xF7\x91\x90a3\xE6V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x1A\x16Wa\x1A\x16a0\xC2V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x1A8\x90a46V[\x90Pa\x17\xF5V[P\x92\x95\x94PPPPPV[a\x1Ap`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEA\x91\x90a1,V[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B.Wa\x1B.a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1BgW\x81` \x01[a\x1BTa$\xA4V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1BLW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\xE0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x96Wa\x1B\x96a0\xC2V[` \x02` \x01\x01Q\x90Pa\x1B\xA9\x81a\x07\xCAV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\xC1Wa\x1B\xC1a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B\xD8\x90a0\xEEV[\x91PPa\x1BmV[P\x91\x90PV[`@\x80Q` \x81\x01\x90\x91R``\x81R`\x01T`@\x80Qc\x1C\x88m\x0B`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x1C\x88m\x0B\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1C>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1Cf\x91\x90\x81\x01\x90a4RV[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x84Wa\x1C\x84a(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xD6W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x1C\xA2W\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1D\xC6W`@Q\x80`\x80\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1D\x02Wa\x1D\x02a0\xC2V[` \x02` \x01\x01Q`\0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1D/Wa\x1D/a0\xC2V[` \x02` \x01\x01Q` \x01Q\x81R` \x01\x84\x83\x81Q\x81\x10a\x1DRWa\x1DRa0\xC2V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1D~Wa\x1D~a0\xC2V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1D\xA8Wa\x1D\xA8a0\xC2V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x1D\xBE\x90a2PV[\x91PPa\x1C\xDCV[P`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xB6\x91\x90a5PV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1E\xF8\x91`\x04\x01a5mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F9\x91\x90a5PV[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1F}\x90`\x01\x90`\x04\x01a5mV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xBE\x91\x90a5PV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a \x14`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xEA\x91\x90a/\xFFV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xAAWa \xAAa(\x9DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \xE3W\x81` \x01[a \xD0a$\x14V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a \xC8W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\xE0W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!\x12Wa!\x12a0\xC2V[` \x02` \x01\x01Q\x90Pa!%\x81a\x03\xE6V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a!=Wa!=a0\xC2V[` \x02` \x01\x01\x81\x90RPP\x80\x80a!T\x90a0\xEEV[\x91PPa \xE9V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD8\x91\x90a5PV[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\" W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"H\x91\x90\x81\x01\x90a5\x87V[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\xC5\x91\x90\x81\x01\x90a5\x87V[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"\xE1Wa\"\xE1a2\x81V[\x03a\"\xF5WPg\r\xE0\xB6\xB3\xA7d\0\0a#]V[`\0\x80\x84`\x0F\x0B\x12a#.W`\0\x83`\x02\x81\x11\x15a#\x15Wa#\x15a2\x81V[\x14a#$W\x84`@\x01Qa#'V[\x84Q[\x90Pa#ZV[`\0\x83`\x02\x81\x11\x15a#BWa#Ba2\x81V[\x14a#QW\x84``\x01Qa#WV[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\xA2WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1E0\x91\x90a6!V[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a#\xF8W\x81a#]V[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a#\xF8W\x81a#]V[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a$SV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a%9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a%NW`\0\x80\xFD[\x815a#]\x81a%'V[`\x80\x81\x01a\x1B\x0C\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa%\xF7`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa&Za\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa&oa\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa'\x12\x87\x83Qa%\x94V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a&\xFFV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa'\x94`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa'\xD4`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa(8\x87\x83Qa'1V[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a(%V[` \x81R`\0\x82Q`@` \x84\x01Ra(h``\x84\x01\x82a&\xEBV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra(\x85\x82\x82a(\x11V[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1B\x0C\x82\x84a'1V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(\xD6Wa(\xD6a(\x9DV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)\x05Wa)\x05a(\x9DV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)'Wa)'a(\x9DV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a)BW`\0\x80\xFD[\x815` a)Wa)R\x83a)\rV[a(\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)vW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a)\x9AW\x805a)\x8D\x81a%'V[\x83R\x91\x83\x01\x91\x83\x01a)zV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a)\xB8W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)\xD6W`\0\x80\xFD[a)\xE2\x85\x82\x86\x01a)1V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa&\xE6` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa*O\x87\x83Qa)\xECV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*<V[` \x81R`\0a#]` \x83\x01\x84a*(V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa*\xB0\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*\x89V[` \x81R`\0a#]` \x83\x01\x84a*uV[a\x02\xC0\x81\x01a\x1B\x0C\x82\x84a%\x94V[`\0` \x82\x84\x03\x12\x15a*\xF7W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a'&Wa+E\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a+\x12V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a+\xC9W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a+\xB4W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a+\x95V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a+wV[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa+\xF7`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra,\x14a\x01`\x85\x01\x83a*\xFEV[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra,2\x84\x83a+XV[\x93P`\x80\x87\x01Q\x91Pa,M`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra,{\x84\x83a*uV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra,\x9A\x85\x84a*(V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra,\xB9\x85\x84a&\xEBV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa,\xD5\x83\x82a(\x11V[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,\xF2W`\0\x80\xFD[\x825\x91P` \x83\x015a-\x04\x81a%'V[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1B\x0CV[`\0` \x82\x84\x03\x12\x15a-BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-YW`\0\x80\xFD[a-e\x84\x82\x85\x01a)1V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\xB0Wa-\x9C\x83\x85Qa%\x94V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a-\x89V[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R`@\x80\x84\x01\x85Q\x83\x84\x87\x01R\x81\x81Q\x80\x84R``\x93P\x83\x88\x01\x91P\x85\x83\x01\x92P`\0[\x81\x81\x10\x15a.9W\x83Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x85\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x84\x01R\x92\x86\x01\x92`\x80\x90\x92\x01\x91`\x01\x01a-\xE6V[P\x90\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a.nW`\0\x80\xFD[\x815a#]\x81a.GV[`\x80\x81\x01a\x1B\x0C\x82\x84a)\xECV[` \x81R`\0a#]` \x83\x01\x84a(\x11V[\x80Q`\x0F\x81\x90\x0B\x81\x14a.\xACW`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a.\xC3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.\xE6Wa.\xE6a(\x9DV[`@R\x82Qa.\xF4\x81a%'V[\x81Ra/\x02` \x84\x01a.\x9AV[` \x82\x01Ra/\x13`@\x84\x01a.\x9AV[`@\x82\x01Ra/$``\x84\x01a.\x9AV[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a/BW`\0\x80\xFD[a/Ja(\xB3V[\x90Pa/U\x82a.\x9AV[\x81Ra/c` \x83\x01a.\x9AV[` \x82\x01Ra/t`@\x83\x01a.\x9AV[`@\x82\x01Ra/\x85``\x83\x01a.\x9AV[``\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a/\xA2W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\xC5Wa/\xC5a(\x9DV[`@R\x90P\x80a/\xD4\x83a.\x9AV[\x81Ra/\xE2` \x84\x01a.\x9AV[` \x82\x01Ra/\xF3`@\x84\x01a.\x9AV[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a0\x12W`\0\x80\xFD[a0\x1C\x84\x84a/0V[\x91Pa0+\x84`\x80\x85\x01a/\x90V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a0FW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0iWa0ia(\x9DV[`@Ra0u\x83a.\x9AV[\x81Ra0\x83` \x84\x01a.\x9AV[` \x82\x01Ra0\x94`@\x84\x01a.\x9AV[`@\x82\x01Ra0\xA5``\x84\x01a.\x9AV[``\x82\x01Ra0\xB6`\x80\x84\x01a.\x9AV[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a1\x07Wa1\x07a0\xD8V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a1#W`\0\x80\xFD[a#]\x82a.\x9AV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a1@W`\0\x80\xFD[a1J\x85\x85a/0V[\x92P` `\x7F\x19\x82\x01\x12\x15a1^W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\x82Wa1\x82a(\x9DV[`@Ra1\x91`\x80\x85\x01a.\x9AV[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a1\xB0W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\xD3Wa1\xD3a(\x9DV[`@R\x82Qa1\xE1\x81a.GV[\x81Ra1\xEF` \x84\x01a.\x9AV[` \x82\x01Ra2\0`@\x84\x01a.\x9AV[`@\x82\x01Ra2\x11``\x84\x01a.\x9AV[``\x82\x01Ra2\"`\x80\x84\x01a.\x9AV[`\x80\x82\x01Ra23`\xA0\x84\x01a.\x9AV[`\xA0\x82\x01Ra2D`\xC0\x84\x01a.\x9AV[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a2bWa2ba0\xD8V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a2|Wa2|a0\xD8V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a2\xC1Wa2\xC1a0\xD8V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a2\xDDWa2\xDDa0\xD8V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a2\xFCWa2\xFCa0\xD8V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a3\x17W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a3AWa3Aa2\x81V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a3aW`\0\x80\xFD[a#]\x83\x83a/\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a3\x88Wa3\x88a0\xD8V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a3\xB6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3\xDDWa3\xDDa0\xD8V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a4\x11Wa4\x11a0\xD8V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a4,Wa4,a0\xD8V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a1\x07Wa1\x07a0\xD8V[`\0` \x80\x83\x85\x03\x12\x15a4eW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4}W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a4\x91W`\0\x80\xFD[\x81Qa4\x9Fa)R\x82a)\rV[\x81\x81R`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a4\xBEW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a5DW`\x80\x85\x8A\x03\x12\x15a4\xDCW`\0\x80\x81\xFD[a4\xE4a(\xB3V[\x85Q\x85\x81\x16\x81\x14a4\xF5W`\0\x80\x81\xFD[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa5\x0E\x81a.GV[\x90\x82\x01R``\x86\x81\x01Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a5.W`\0\x80\x81\xFD[\x90\x82\x01R\x82R`\x80\x94\x90\x94\x01\x93\x90\x85\x01\x90a4\xC3V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a5bW`\0\x80\xFD[\x81Qa#]\x81a.GV[` \x81\x01`\x02\x83\x10a5\x81Wa5\x81a2\x81V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a5\x9AW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\xB1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a5\xC2W`\0\x80\xFD[\x80Qa5\xD0a)R\x82a)\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a5\xEFW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\x16W\x83Qa6\x07\x81a%'V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a5\xF4V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a6NW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a62V[\x81\x81\x11\x15a6`W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xB1\xFCZX\x96MF\xD1f\x0B\xE7\x97\x11B\xBD\xD9\xA7\xB5\xC2\xE10\xAC?\xD1L&D\xA3\xFB!\xAA_dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static QUERIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Querier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Querier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Querier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Querier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Querier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Querier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Querier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                QUERIER_ABI.clone(),
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
                QUERIER_ABI.clone(),
                QUERIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getAllProducts` (0x02ee3a52) function
        pub fn get_all_products(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProductInfo> {
            self.0
                .method_hash([2, 238, 58, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBookInfo` (0x01cfa9d1) function
        pub fn get_book_info(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, BookInfo> {
            self.0
                .method_hash([1, 207, 169, 209], product_id)
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
        ///Calls the contract's `getInsurance` (0x267a8da0) function
        pub fn get_insurance(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([38, 122, 141, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNlpPoolInfo` (0x9705f230) function
        pub fn get_nlp_pool_info(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, NlpPoolInfo> {
            self.0
                .method_hash([151, 5, 242, 48], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpBalance` (0xd7b229b6) function
        pub fn get_perp_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpBalance> {
            self.0
                .method_hash([215, 178, 41, 182], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpBalances` (0x2593eb5f) function
        pub fn get_perp_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpBalance>> {
            self.0
                .method_hash([37, 147, 235, 95], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpProduct` (0x1ae10bc5) function
        pub fn get_perp_product(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpProduct> {
            self.0
                .method_hash([26, 225, 11, 197], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPerpProducts` (0xee9928c9) function
        pub fn get_perp_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PerpProduct>> {
            self.0
                .method_hash([238, 153, 40, 201], product_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotBalance` (0x74173404) function
        pub fn get_spot_balance(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotBalance> {
            self.0
                .method_hash([116, 23, 52, 4], (subaccount, product_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotBalances` (0x31546d51) function
        pub fn get_spot_balances(
            &self,
            subaccount: [u8; 32],
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotBalance>> {
            self.0
                .method_hash([49, 84, 109, 81], (subaccount, product_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotProduct` (0x5723653f) function
        pub fn get_spot_product(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotProduct> {
            self.0
                .method_hash([87, 35, 101, 63], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotProducts` (0x75a5ab3c) function
        pub fn get_spot_products(
            &self,
            product_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SpotProduct>> {
            self.0
                .method_hash([117, 165, 171, 60], product_ids)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountInfo` (0x5d702e1a) function
        pub fn get_subaccount_info(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, SubaccountInfo> {
            self.0
                .method_hash([93, 112, 46, 26], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            clearinghouse: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], clearinghouse)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Querier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
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
    #[ethcall(name = "getAllProducts", abi = "getAllProducts()")]
    pub struct GetAllProductsCall;
    ///Container type for all input parameters for the `getBookInfo` function with signature `getBookInfo(uint32)` and selector `0x01cfa9d1`
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
    #[ethcall(name = "getBookInfo", abi = "getBookInfo(uint32)")]
    pub struct GetBookInfoCall {
        pub product_id: u32,
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
    ///Container type for all input parameters for the `getInsurance` function with signature `getInsurance()` and selector `0x267a8da0`
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
    #[ethcall(name = "getInsurance", abi = "getInsurance()")]
    pub struct GetInsuranceCall;
    ///Container type for all input parameters for the `getNlpPoolInfo` function with signature `getNlpPoolInfo()` and selector `0x9705f230`
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
    #[ethcall(name = "getNlpPoolInfo", abi = "getNlpPoolInfo()")]
    pub struct GetNlpPoolInfoCall;
    ///Container type for all input parameters for the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
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
    #[ethcall(name = "getPerpBalance", abi = "getPerpBalance(bytes32,uint32)")]
    pub struct GetPerpBalanceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `0x2593eb5f`
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
    #[ethcall(name = "getPerpBalances", abi = "getPerpBalances(bytes32,uint32[])")]
    pub struct GetPerpBalancesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `0x1ae10bc5`
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
    #[ethcall(name = "getPerpProduct", abi = "getPerpProduct(uint32)")]
    pub struct GetPerpProductCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `0xee9928c9`
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
    #[ethcall(name = "getPerpProducts", abi = "getPerpProducts(uint32[])")]
    pub struct GetPerpProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `0x74173404`
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
    #[ethcall(name = "getSpotBalance", abi = "getSpotBalance(bytes32,uint32)")]
    pub struct GetSpotBalanceCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `0x31546d51`
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
    #[ethcall(name = "getSpotBalances", abi = "getSpotBalances(bytes32,uint32[])")]
    pub struct GetSpotBalancesCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `0x5723653f`
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
    #[ethcall(name = "getSpotProduct", abi = "getSpotProduct(uint32)")]
    pub struct GetSpotProductCall {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `0x75a5ab3c`
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
    #[ethcall(name = "getSpotProducts", abi = "getSpotProducts(uint32[])")]
    pub struct GetSpotProductsCall {
        pub product_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `0x5d702e1a`
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
    #[ethcall(name = "getSubaccountInfo", abi = "getSubaccountInfo(bytes32)")]
    pub struct GetSubaccountInfoCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub clearinghouse: ::ethers::core::types::Address,
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
    pub enum QuerierCalls {
        GetAllProducts(GetAllProductsCall),
        GetBookInfo(GetBookInfoCall),
        GetClearinghouse(GetClearinghouseCall),
        GetInsurance(GetInsuranceCall),
        GetNlpPoolInfo(GetNlpPoolInfoCall),
        GetPerpBalance(GetPerpBalanceCall),
        GetPerpBalances(GetPerpBalancesCall),
        GetPerpProduct(GetPerpProductCall),
        GetPerpProducts(GetPerpProductsCall),
        GetSpotBalance(GetSpotBalanceCall),
        GetSpotBalances(GetSpotBalancesCall),
        GetSpotProduct(GetSpotProductCall),
        GetSpotProducts(GetSpotProductsCall),
        GetSubaccountInfo(GetSubaccountInfoCall),
        Initialize(InitializeCall),
    }
    impl ::ethers::core::abi::AbiDecode for QuerierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetAllProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllProducts(decoded));
            }
            if let Ok(decoded) = <GetBookInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBookInfo(decoded));
            }
            if let Ok(decoded) =
                <GetClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetClearinghouse(decoded));
            }
            if let Ok(decoded) = <GetInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInsurance(decoded));
            }
            if let Ok(decoded) =
                <GetNlpPoolInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNlpPoolInfo(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpBalance(decoded));
            }
            if let Ok(decoded) =
                <GetPerpBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpBalances(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpProduct(decoded));
            }
            if let Ok(decoded) =
                <GetPerpProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPerpProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotBalance(decoded));
            }
            if let Ok(decoded) =
                <GetSpotBalancesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotBalances(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotProduct(decoded));
            }
            if let Ok(decoded) =
                <GetSpotProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotProducts(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountInfo(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QuerierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAllProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBookInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetClearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInsurance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNlpPoolInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPerpProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotBalances(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotProducts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for QuerierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAllProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBookInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetClearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNlpPoolInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPerpProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotBalances(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAllProductsCall> for QuerierCalls {
        fn from(value: GetAllProductsCall) -> Self {
            Self::GetAllProducts(value)
        }
    }
    impl ::core::convert::From<GetBookInfoCall> for QuerierCalls {
        fn from(value: GetBookInfoCall) -> Self {
            Self::GetBookInfo(value)
        }
    }
    impl ::core::convert::From<GetClearinghouseCall> for QuerierCalls {
        fn from(value: GetClearinghouseCall) -> Self {
            Self::GetClearinghouse(value)
        }
    }
    impl ::core::convert::From<GetInsuranceCall> for QuerierCalls {
        fn from(value: GetInsuranceCall) -> Self {
            Self::GetInsurance(value)
        }
    }
    impl ::core::convert::From<GetNlpPoolInfoCall> for QuerierCalls {
        fn from(value: GetNlpPoolInfoCall) -> Self {
            Self::GetNlpPoolInfo(value)
        }
    }
    impl ::core::convert::From<GetPerpBalanceCall> for QuerierCalls {
        fn from(value: GetPerpBalanceCall) -> Self {
            Self::GetPerpBalance(value)
        }
    }
    impl ::core::convert::From<GetPerpBalancesCall> for QuerierCalls {
        fn from(value: GetPerpBalancesCall) -> Self {
            Self::GetPerpBalances(value)
        }
    }
    impl ::core::convert::From<GetPerpProductCall> for QuerierCalls {
        fn from(value: GetPerpProductCall) -> Self {
            Self::GetPerpProduct(value)
        }
    }
    impl ::core::convert::From<GetPerpProductsCall> for QuerierCalls {
        fn from(value: GetPerpProductsCall) -> Self {
            Self::GetPerpProducts(value)
        }
    }
    impl ::core::convert::From<GetSpotBalanceCall> for QuerierCalls {
        fn from(value: GetSpotBalanceCall) -> Self {
            Self::GetSpotBalance(value)
        }
    }
    impl ::core::convert::From<GetSpotBalancesCall> for QuerierCalls {
        fn from(value: GetSpotBalancesCall) -> Self {
            Self::GetSpotBalances(value)
        }
    }
    impl ::core::convert::From<GetSpotProductCall> for QuerierCalls {
        fn from(value: GetSpotProductCall) -> Self {
            Self::GetSpotProduct(value)
        }
    }
    impl ::core::convert::From<GetSpotProductsCall> for QuerierCalls {
        fn from(value: GetSpotProductsCall) -> Self {
            Self::GetSpotProducts(value)
        }
    }
    impl ::core::convert::From<GetSubaccountInfoCall> for QuerierCalls {
        fn from(value: GetSubaccountInfoCall) -> Self {
            Self::GetSubaccountInfo(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for QuerierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    ///Container type for all return fields from the `getAllProducts` function with signature `getAllProducts()` and selector `0x02ee3a52`
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
    pub struct GetAllProductsReturn(pub ProductInfo);
    ///Container type for all return fields from the `getBookInfo` function with signature `getBookInfo(uint32)` and selector `0x01cfa9d1`
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
    pub struct GetBookInfoReturn {
        pub book_info: BookInfo,
    }
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
    ///Container type for all return fields from the `getInsurance` function with signature `getInsurance()` and selector `0x267a8da0`
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
    pub struct GetInsuranceReturn(pub i128);
    ///Container type for all return fields from the `getNlpPoolInfo` function with signature `getNlpPoolInfo()` and selector `0x9705f230`
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
    pub struct GetNlpPoolInfoReturn(pub NlpPoolInfo);
    ///Container type for all return fields from the `getPerpBalance` function with signature `getPerpBalance(bytes32,uint32)` and selector `0xd7b229b6`
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
    pub struct GetPerpBalanceReturn(pub PerpBalance);
    ///Container type for all return fields from the `getPerpBalances` function with signature `getPerpBalances(bytes32,uint32[])` and selector `0x2593eb5f`
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
    pub struct GetPerpBalancesReturn {
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
    }
    ///Container type for all return fields from the `getPerpProduct` function with signature `getPerpProduct(uint32)` and selector `0x1ae10bc5`
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
    pub struct GetPerpProductReturn(pub PerpProduct);
    ///Container type for all return fields from the `getPerpProducts` function with signature `getPerpProducts(uint32[])` and selector `0xee9928c9`
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
    pub struct GetPerpProductsReturn {
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///Container type for all return fields from the `getSpotBalance` function with signature `getSpotBalance(bytes32,uint32)` and selector `0x74173404`
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
    pub struct GetSpotBalanceReturn(pub SpotBalance);
    ///Container type for all return fields from the `getSpotBalances` function with signature `getSpotBalances(bytes32,uint32[])` and selector `0x31546d51`
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
    pub struct GetSpotBalancesReturn {
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
    }
    ///Container type for all return fields from the `getSpotProduct` function with signature `getSpotProduct(uint32)` and selector `0x5723653f`
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
    pub struct GetSpotProductReturn(pub SpotProduct);
    ///Container type for all return fields from the `getSpotProducts` function with signature `getSpotProducts(uint32[])` and selector `0x75a5ab3c`
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
    pub struct GetSpotProductsReturn {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
    }
    ///Container type for all return fields from the `getSubaccountInfo` function with signature `getSubaccountInfo(bytes32)` and selector `0x5d702e1a`
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
    pub struct GetSubaccountInfoReturn(pub SubaccountInfo);
    ///`BookInfo(int128,int128,int128,int128)`
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
    pub struct BookInfo {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub size_increment: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_increment_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub min_size: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub collected_fees: i128,
    }
    ///`HealthInfo(int128,int128,int128)`
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
    pub struct HealthInfo {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub assets: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub liabilities: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub health: i128,
    }
    ///`NlpPool(uint64,bytes32,address,uint128)`
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
    pub struct NlpPool {
        pub pool_id: u64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub balance_weight_x18: u128,
    }
    ///`NlpPoolInfo((uint64,bytes32,address,uint128)[])`
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
    pub struct NlpPoolInfo {
        pub nlp_pools: ::std::vec::Vec<NlpPool>,
    }
    ///`PerpBalance(uint32,(int128,int128,int128))`
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
    pub struct PerpBalance {
        pub product_id: u32,
        pub balance: crate::bindings::perp_engine::Balance,
    }
    ///`PerpProduct(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))`
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
    pub struct PerpProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub state: crate::bindings::perp_engine::State,
        pub book_info: BookInfo,
    }
    ///`ProductInfo((uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))[])`
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
    pub struct ProductInfo {
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///`SpotBalance(uint32,(int128))`
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
    pub struct SpotBalance {
        pub product_id: u32,
        pub balance: crate::bindings::spot_engine::Balance,
    }
    ///`SpotProduct(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))`
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
    pub struct SpotProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        pub risk: Risk,
        pub config: Config,
        pub state: crate::bindings::spot_engine::State,
        pub book_info: BookInfo,
    }
    ///`SubaccountInfo(bytes32,bool,(int128,int128,int128)[],int128[][],uint32,uint32,(uint32,(int128))[],(uint32,(int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(address,int128,int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))[],(uint32,int128,(int128,int128,int128,int128,int128),(int128,int128,int128,int128),(int128,int128,int128,int128))[])`
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
    pub struct SubaccountInfo {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub exists: bool,
        pub healths: ::std::vec::Vec<HealthInfo>,
        pub health_contributions: ::std::vec::Vec<::std::vec::Vec<i128>>,
        pub spot_count: u32,
        pub perp_count: u32,
        pub spot_balances: ::std::vec::Vec<SpotBalance>,
        pub perp_balances: ::std::vec::Vec<PerpBalance>,
        pub spot_products: ::std::vec::Vec<SpotProduct>,
        pub perp_products: ::std::vec::Vec<PerpProduct>,
    }
    ///`IperpEngineBalance(int128,int128,int128)`
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
    pub struct IperpEngineBalance {
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
    ///`IperpEngineState(int128,int128,int128,int128)`
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
    pub struct IperpEngineState {
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
    ///`IspotEngineBalance(int128)`
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
    pub struct IspotEngineBalance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
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
    ///`IspotEngineState(int128,int128,int128,int128)`
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
    pub struct IspotEngineState {
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
}
