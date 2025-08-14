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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa2@\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80c]p.\x1A\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\x02W\x80c\xC4\xD6m\xE8\x14a\x02\x1DW\x80c\xD7\xB2)\xB6\x14a\x022W\x80c\xEE\x99(\xC9\x14a\x02RW`\0\x80\xFD[\x80c]p.\x1A\x14a\x01\xA2W\x80ct\x174\x04\x14a\x01\xC2W\x80cu\xA5\xAB<\x14a\x01\xE2W`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xBDW\x80c%\x93\xEB_\x14a\x01BW\x80c1TmQ\x14a\x01bW\x80cW#e?\x14a\x01\x82W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xE4W\x80c\x02\xEE:R\x14a\x01\rW\x80c\x1A\xE1\x0B\xC5\x14a\x01\"W[`\0\x80\xFD[a\0\xF7a\0\xF26`\x04a\"}V[a\x02rV[`@Qa\x01\x04\x91\x90a\"\x9AV[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x03RV[`@Qa\x01\x04\x91\x90a%\x8DV[a\x015a\x0106`\x04a\"}V[a\x03\xA0V[`@Qa\x01\x04\x91\x90a%\xCFV[a\x01Ua\x01P6`\x04a&\xBDV[a\x05\x17V[`@Qa\x01\x04\x91\x90a'zV[a\x01ua\x01p6`\x04a&\xBDV[a\x06\x18V[`@Qa\x01\x04\x91\x90a'\xDBV[a\x01\x95a\x01\x906`\x04a\"}V[a\x07\x05V[`@Qa\x01\x04\x91\x90a'\xEEV[a\x01\xB5a\x01\xB06`\x04a'\xFDV[a\x08\xDDV[`@Qa\x01\x04\x91\x90a(\xEFV[a\x01\xD5a\x01\xD06`\x04a)\xF7V[a\x19{V[`@Qa\x01\x04\x91\x90a*'V[a\x01\xF5a\x01\xF06`\x04a*HV[a\x1ACV[`@Qa\x01\x04\x91\x90a*\x85V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x04V[a\x020a\x02+6`\x04a*\xE9V[a\x1B\x17V[\0[a\x02Ea\x02@6`\x04a)\xF7V[a\x1D\x1DV[`@Qa\x01\x04\x91\x90a+\x06V[a\x02ea\x02`6`\x04a*HV[a\x1D\xCAV[`@Qa\x01\x04\x91\x90a+\x14V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\x9Ea\x1E\x98V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0E\x91\x90a+>V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03qa\x1F\x0BV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\x89\x84a\x1ACV[\x81R` \x01a\x03\x97\x83a\x1D\xCAV[\x90R\x93\x92PPPV[a\x03\xA8a!UV[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\"\x91\x90a,\xACV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a,\xE1V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05\x04W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05\rV[a\x05\r\x86a\x02rV[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x053Wa\x053a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x97W\x81` \x01[a\x05\x84`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05QW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xC6Wa\x05\xC6a-oV[` \x02` \x01\x01Q\x90Pa\x05\xDA\x85\x82a\x1D\x1DV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xF2Wa\x05\xF2a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06\t\x90a-\x9BV[\x91PPa\x05\x9DV[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x064Wa\x064a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x8BW\x81` \x01[a\x06x`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06RW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xBAWa\x06\xBAa-oV[` \x02` \x01\x01Q\x90Pa\x06\xCE\x85\x82a\x19{V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xE6Wa\x06\xE6a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06\xFD\x90a-\x9BV[\x91PPa\x06\x91V[a\x07\ra!\xE5V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x87\x91\x90a-\xBEV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\0\x91\x90a,\xE1V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9B\x91\x90a.0V[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05\x04W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05\rV[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\t\x88a\x1F\x0BV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\t\xADWPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\nGW\x81\x84\x82\x81Q\x81\x10a\t\xFEWa\t\xFEa-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n5W\x83\x81\x81Q\x81\x10a\n$Wa\n$a-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n?\x81a.\xE2V[\x91PPa\t\xE2V[P`\0[\x82Q\x81\x10\x15a\n\xB0W\x81\x83\x82\x81Q\x81\x10a\ngWa\nga-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\x9EW\x82\x81\x81Q\x81\x10a\n\x8DWa\n\x8Da-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n\xA8\x81a.\xE2V[\x91PPa\nKV[Pa\n\xBC\x81`\x01a.\xFBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xD4Wa\n\xD4a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x07W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xF2W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0BjW`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0BLWa\x0BLa-oV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0Bb\x90a.\xE2V[\x91PPa\x0B\x10V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x86Wa\x0B\x86a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDDW\x81` \x01[a\x0B\xCA`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xA4W\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xFDWa\x0B\xFDa%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CaW\x81` \x01[a\x0CN`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x1BW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x81Wa\x0C\x81a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBAW\x81` \x01[a\x0C\xA7a!\xE5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x9FW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xDBWa\x0C\xDBa%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x14W\x81` \x01[a\r\x01a!UV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xF9W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10VW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\rGWa\rGa-oV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a-\xBEV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EL\x91\x90a,\xE1V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\x92Wa\x0E\x92a-oV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F8\x91\x90a.0V[\x81R` \x01\x85\x81R` \x01a\x0FL\x87a\x02rV[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x0Fd\x82a-\x9BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\x89Wa\x0F\x89a-oV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x10?W\x83Q`\0\x90a\x0F\xD8\x90\x84\x90a\x0F\xCF\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x0F\xCAWa\x0F\xCAa/\x13V[a \x0CV[`\x0F\x0B\x90a \xA5V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xF7Wa\x0F\xF7a-oV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x10\x13Wa\x10\x13a-oV[` \x02` \x01\x01\x81\x81Qa\x10'\x91\x90a/)V[`\x0F\x0B\x90RPa\x108\x90P\x81a/xV[\x90Pa\x0F\x97V[PPPPPP\x80a\x10O\x90a-\x9BV[\x90Pa\r\x1EV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\x1EW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\x83Wa\x10\x83a-oV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\r\x91\x90a,\xACV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x88\x91\x90a,\xE1V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xCEWa\x11\xCEa-oV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x12\x0E\x87a\x02rV[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12&\x82a-\x9BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12KWa\x12Ka-oV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13\x07W`\0\x84` \x01Qa\x12\x96\x84a\x0F\xCF\x88`\0\x01Qa\x0F\xCF\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x0F\xCAWa\x0F\xCAa/\x13V[a\x12\xA0\x91\x90a/)V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xBFWa\x12\xBFa-oV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\xDBWa\x12\xDBa-oV[` \x02` \x01\x01\x81\x81Qa\x12\xEF\x91\x90a/)V[`\x0F\x0B\x90RPa\x13\0\x90P\x81a/xV[\x90Pa\x12YV[PPPPPP\x80a\x13\x17\x90a-\x9BV[\x90Pa\x10ZV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x97\x91\x90a/\x97V[\x90P[\x80\x15a\x17#W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17\x1BW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x13\xEAWa\x13\xEAa/\x13V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x08\x93\x92\x91\x90a/\xB0V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14I\x91\x90a/\xE1V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x14^WPa\x17\tV[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x14\x8FWa\x14\x8Fa/\x13V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xAD\x93\x92\x91\x90a/\xB0V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xEE\x91\x90a/\xE1V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15\x11WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15\x1DWPPa\x17\tV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x15LW\x81Q\x83Qa\x15E\x91\x90a\x15@\x90a/\xFDV[a!$V[\x90Pa\x15oV[\x81Q\x83Qa\x15c\x91\x90a\x15^\x90a/\xFDV[a!@V[a\x15l\x90a/\xFDV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x15\x87\x91\x90a/)V[a\x15\x91\x91\x90a0#V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x15\xE1W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xBE\x91\x90a0xV[a\x15\xC8\x91\x90a0#V[a\x15\xDA\x90g\r\xE0\xB6\xB3\xA7d\0\0a0xV[\x90Pa\x16\x1AV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xFB\x91\x90a0xV[a\x16\x05\x91\x90a0#V[a\x16\x17\x90g\r\xE0\xB6\xB3\xA7d\0\0a0xV[\x90P[`\0`\x02a\x16Ma\x16+\x85\x85a0xV[a\x0F\xCF\x89` \x01Q\x89` \x01Qa\x16B\x91\x90a/)V[`\x0F\x89\x90\x0B\x90a \xA5V[a\x16W\x91\x90a0#V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16vWa\x16va-oV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\x92Wa\x16\x92a-oV[` \x02` \x01\x01\x81\x81Qa\x16\xA6\x91\x90a/)V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x16\xCCWa\x16\xCCa-oV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\xE8Wa\x16\xE8a-oV[` \x02` \x01\x01\x81\x81Qa\x16\xFC\x91\x90a/)V[`\x0F\x0B\x90RPPPPPPP[\x80a\x17\x13\x81a/xV[\x91PPa\x13\xB3V[PPPa\x13\x9AV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19pW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x18\xC7W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17rWa\x17ra-oV[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\x94Wa\x17\x94a-oV[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x180W\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xC4Wa\x17\xC4a-oV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xE6Wa\x17\xE6a-oV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\rWa\x18\ra-oV[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18%\x91\x90a/)V[`\x0F\x0B\x90RPa\x18\xB7V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18OWa\x18Oa-oV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18qWa\x18qa-oV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x98Wa\x18\x98a-oV[` \x02` \x01\x01Q` \x01\x81\x81Qa\x18\xB0\x91\x90a0xV[`\x0F\x0B\x90RP[a\x18\xC0\x81a0\xC8V[\x90Pa\x17;V[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xE7Wa\x18\xE7a-oV[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x12Wa\x19\x12a-oV[` \x02` \x01\x01Q`\0\x01Qa\x19(\x91\x90a0xV[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19GWa\x19Ga-oV[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x19i\x90a0\xC8V[\x90Pa\x17&V[P\x92\x95\x94PPPPPV[a\x19\xA1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1B\x91\x90a-\xBEV[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A_Wa\x1A_a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x98W\x81` \x01[a\x1A\x85a!\xE5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A}W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xC7Wa\x1A\xC7a-oV[` \x02` \x01\x01Q\x90Pa\x1A\xDA\x81a\x07\x05V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xF2Wa\x1A\xF2a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B\t\x90a-\x9BV[\x91PPa\x1A\x9EV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1BuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF2\x91\x90a0\xE4V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1C4\x91`\x04\x01a1\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CQW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cu\x91\x90a0\xE4V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1C\xB9\x90`\x01\x90`\x04\x01a1\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFA\x91\x90a0\xE4V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x1DP`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1B\x91\x90a,\xACV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE6Wa\x1D\xE6a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x1FW\x81` \x01[a\x1E\x0Ca!UV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\x04W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1ENWa\x1ENa-oV[` \x02` \x01\x01Q\x90Pa\x1Ea\x81a\x03\xA0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1EyWa\x1Eya-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1E\x90\x90a-\x9BV[\x91PPa\x1E%V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x06\x91\x90a0\xE4V[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FaW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x89\x91\x90\x81\x01\x90a1\x1BV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x06\x91\x90\x81\x01\x90a1\x1BV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a \"Wa \"a/\x13V[\x03a 6WPg\r\xE0\xB6\xB3\xA7d\0\0a \x9EV[`\0\x80\x84`\x0F\x0B\x12a oW`\0\x83`\x02\x81\x11\x15a VWa Va/\x13V[\x14a eW\x84`@\x01Qa hV[\x84Q[\x90Pa \x9BV[`\0\x83`\x02\x81\x11\x15a \x83Wa \x83a/\x13V[\x14a \x92W\x84``\x01Qa \x98V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a \xE3WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a!\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Bl\x91\x90a1\xB5V[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a!9W\x81a \x9EV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a!9W\x81a \x9EV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a!\x94V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"zW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\"\x8FW`\0\x80\xFD[\x815a \x9E\x81a\"hV[`\x80\x81\x01a\x1A=\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa#8`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa#\x9Ba\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa#\xB0a\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa$S\x87\x83Qa\"\xD5V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a$@V[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa$\xD5`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa%\x15`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa%y\x87\x83Qa$rV[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a%fV[` \x81R`\0\x82Q`@` \x84\x01Ra%\xA9``\x84\x01\x82a$,V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra%\xC6\x82\x82a%RV[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1A=\x82\x84a$rV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\x1DWa&\x1Da%\xDEV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&?Wa&?a%\xDEV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&ZW`\0\x80\xFD[\x815` a&oa&j\x83a&%V[a%\xF4V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a&\x8EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a&\xB2W\x805a&\xA5\x81a\"hV[\x83R\x91\x83\x01\x91\x83\x01a&\x92V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a&\xD0W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xEEW`\0\x80\xFD[a&\xFA\x85\x82\x86\x01a&IV[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa$'` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa'g\x87\x83Qa'\x04V[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'TV[` \x81R`\0a \x9E` \x83\x01\x84a'@V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa'\xC8\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\xA1V[` \x81R`\0a \x9E` \x83\x01\x84a'\x8DV[a\x02\xC0\x81\x01a\x1A=\x82\x84a\"\xD5V[`\0` \x82\x84\x03\x12\x15a(\x0FW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa(]\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a(*V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a(\xE1W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a(\xCCW\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a(\xADV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a(\x8FV[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa)\x0F`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra),a\x01`\x85\x01\x83a(\x16V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra)J\x84\x83a(pV[\x93P`\x80\x87\x01Q\x91Pa)e`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra)\x93\x84\x83a'\x8DV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra)\xB2\x85\x84a'@V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra)\xD1\x85\x84a$,V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa)\xED\x83\x82a%RV[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a*\nW`\0\x80\xFD[\x825\x91P` \x83\x015a*\x1C\x81a\"hV[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1A=V[`\0` \x82\x84\x03\x12\x15a*ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*qW`\0\x80\xFD[a*}\x84\x82\x85\x01a&IV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a*\xC8Wa*\xB4\x83\x85Qa\"\xD5V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a*\xA1V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"zW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a*\xFBW`\0\x80\xFD[\x815a \x9E\x81a*\xD4V[`\x80\x81\x01a\x1A=\x82\x84a'\x04V[` \x81R`\0a \x9E` \x83\x01\x84a%RV[\x80Q`\x0F\x81\x90\x0B\x81\x14a+9W`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a+PW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a+sWa+sa%\xDEV[`@R\x82Qa+\x81\x81a\"hV[\x81Ra+\x8F` \x84\x01a+'V[` \x82\x01Ra+\xA0`@\x84\x01a+'V[`@\x82\x01Ra+\xB1``\x84\x01a+'V[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a+\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a+\xF2Wa+\xF2a%\xDEV[`@R\x90P\x80a,\x01\x83a+'V[\x81Ra,\x0F` \x84\x01a+'V[` \x82\x01Ra, `@\x84\x01a+'V[`@\x82\x01Ra,1``\x84\x01a+'V[``\x82\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a,OW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a,rWa,ra%\xDEV[`@R\x90P\x80a,\x81\x83a+'V[\x81Ra,\x8F` \x84\x01a+'V[` \x82\x01Ra,\xA0`@\x84\x01a+'V[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a,\xBFW`\0\x80\xFD[a,\xC9\x84\x84a+\xBDV[\x91Pa,\xD8\x84`\x80\x85\x01a,=V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a,\xF3W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-\x16Wa-\x16a%\xDEV[`@Ra-\"\x83a+'V[\x81Ra-0` \x84\x01a+'V[` \x82\x01Ra-A`@\x84\x01a+'V[`@\x82\x01Ra-R``\x84\x01a+'V[``\x82\x01Ra-c`\x80\x84\x01a+'V[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a-\xB4Wa-\xB4a-\x85V[`\x01\x01\x93\x92PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a-\xD2W`\0\x80\xFD[a-\xDC\x85\x85a+\xBDV[\x92P` `\x7F\x19\x82\x01\x12\x15a-\xF0W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.\x14Wa.\x14a%\xDEV[`@Ra.#`\x80\x85\x01a+'V[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a.BW`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.eWa.ea%\xDEV[`@R\x82Qa.s\x81a*\xD4V[\x81Ra.\x81` \x84\x01a+'V[` \x82\x01Ra.\x92`@\x84\x01a+'V[`@\x82\x01Ra.\xA3``\x84\x01a+'V[``\x82\x01Ra.\xB4`\x80\x84\x01a+'V[`\x80\x82\x01Ra.\xC5`\xA0\x84\x01a+'V[`\xA0\x82\x01Ra.\xD6`\xC0\x84\x01a+'V[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a.\xF4Wa.\xF4a-\x85V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a/\x0EWa/\x0Ea-\x85V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a/SWa/Sa-\x85V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a/oWa/oa-\x85V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a/\x8EWa/\x8Ea-\x85V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a/\xA9W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a/\xD3Wa/\xD3a/\x13V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a/\xF3W`\0\x80\xFD[a \x9E\x83\x83a,=V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a0\x1AWa0\x1Aa-\x85V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a0HWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a0oWa0oa-\x85V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a0\xA3Wa0\xA3a-\x85V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a0\xBEWa0\xBEa-\x85V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a-\xB4Wa-\xB4a-\x85V[`\0` \x82\x84\x03\x12\x15a0\xF6W`\0\x80\xFD[\x81Qa \x9E\x81a*\xD4V[` \x81\x01`\x02\x83\x10a1\x15Wa1\x15a/\x13V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a1.W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a1VW`\0\x80\xFD[\x80Qa1da&j\x82a&%V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a1\x83W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a1\xAAW\x83Qa1\x9B\x81a\"hV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a1\x88V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a1\xE2W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a1\xC6V[\x81\x81\x11\x15a1\xF4W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x15\nc\xEBI\"\xBF:Zb9\x8F\xB6<\x90U\xBF\xAD\xC8\xBC\xBF\xB8\xA3\x1Cz\xA3[\x14\xF8\xEC JdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xDFW`\x005`\xE0\x1C\x80c]p.\x1A\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\x02W\x80c\xC4\xD6m\xE8\x14a\x02\x1DW\x80c\xD7\xB2)\xB6\x14a\x022W\x80c\xEE\x99(\xC9\x14a\x02RW`\0\x80\xFD[\x80c]p.\x1A\x14a\x01\xA2W\x80ct\x174\x04\x14a\x01\xC2W\x80cu\xA5\xAB<\x14a\x01\xE2W`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xBDW\x80c%\x93\xEB_\x14a\x01BW\x80c1TmQ\x14a\x01bW\x80cW#e?\x14a\x01\x82W`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xE4W\x80c\x02\xEE:R\x14a\x01\rW\x80c\x1A\xE1\x0B\xC5\x14a\x01\"W[`\0\x80\xFD[a\0\xF7a\0\xF26`\x04a\"}V[a\x02rV[`@Qa\x01\x04\x91\x90a\"\x9AV[`@Q\x80\x91\x03\x90\xF3[a\x01\x15a\x03RV[`@Qa\x01\x04\x91\x90a%\x8DV[a\x015a\x0106`\x04a\"}V[a\x03\xA0V[`@Qa\x01\x04\x91\x90a%\xCFV[a\x01Ua\x01P6`\x04a&\xBDV[a\x05\x17V[`@Qa\x01\x04\x91\x90a'zV[a\x01ua\x01p6`\x04a&\xBDV[a\x06\x18V[`@Qa\x01\x04\x91\x90a'\xDBV[a\x01\x95a\x01\x906`\x04a\"}V[a\x07\x05V[`@Qa\x01\x04\x91\x90a'\xEEV[a\x01\xB5a\x01\xB06`\x04a'\xFDV[a\x08\xDDV[`@Qa\x01\x04\x91\x90a(\xEFV[a\x01\xD5a\x01\xD06`\x04a)\xF7V[a\x19{V[`@Qa\x01\x04\x91\x90a*'V[a\x01\xF5a\x01\xF06`\x04a*HV[a\x1ACV[`@Qa\x01\x04\x91\x90a*\x85V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x04V[a\x020a\x02+6`\x04a*\xE9V[a\x1B\x17V[\0[a\x02Ea\x02@6`\x04a)\xF7V[a\x1D\x1DV[`@Qa\x01\x04\x91\x90a+\x06V[a\x02ea\x02`6`\x04a*HV[a\x1D\xCAV[`@Qa\x01\x04\x91\x90a+\x14V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\x9Ea\x1E\x98V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x0E\x91\x90a+>V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03qa\x1F\x0BV[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\x89\x84a\x1ACV[\x81R` \x01a\x03\x97\x83a\x1D\xCAV[\x90R\x93\x92PPPV[a\x03\xA8a!UV[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\"\x91\x90a,\xACV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a,\xE1V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05\x04W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05\rV[a\x05\r\x86a\x02rV[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x053Wa\x053a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x97W\x81` \x01[a\x05\x84`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05QW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xC6Wa\x05\xC6a-oV[` \x02` \x01\x01Q\x90Pa\x05\xDA\x85\x82a\x1D\x1DV[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xF2Wa\x05\xF2a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06\t\x90a-\x9BV[\x91PPa\x05\x9DV[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x064Wa\x064a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\x8BW\x81` \x01[a\x06x`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06RW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x06\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xBAWa\x06\xBAa-oV[` \x02` \x01\x01Q\x90Pa\x06\xCE\x85\x82a\x19{V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xE6Wa\x06\xE6a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06\xFD\x90a-\x9BV[\x91PPa\x06\x91V[a\x07\ra!\xE5V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07cW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x87\x91\x90a-\xBEV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\0\x91\x90a,\xE1V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x9B\x91\x90a.0V[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05\x04W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05\rV[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\t\x88a\x1F\x0BV[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\t\xADWPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\nGW\x81\x84\x82\x81Q\x81\x10a\t\xFEWa\t\xFEa-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n5W\x83\x81\x81Q\x81\x10a\n$Wa\n$a-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n?\x81a.\xE2V[\x91PPa\t\xE2V[P`\0[\x82Q\x81\x10\x15a\n\xB0W\x81\x83\x82\x81Q\x81\x10a\ngWa\nga-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\x9EW\x82\x81\x81Q\x81\x10a\n\x8DWa\n\x8Da-oV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n\xA8\x81a.\xE2V[\x91PPa\nKV[Pa\n\xBC\x81`\x01a.\xFBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xD4Wa\n\xD4a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x07W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xF2W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0BjW`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0BLWa\x0BLa-oV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0Bb\x90a.\xE2V[\x91PPa\x0B\x10V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x86Wa\x0B\x86a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDDW\x81` \x01[a\x0B\xCA`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xA4W\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xFDWa\x0B\xFDa%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CaW\x81` \x01[a\x0CN`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x1BW\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x81Wa\x0C\x81a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBAW\x81` \x01[a\x0C\xA7a!\xE5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\x9FW\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xDBWa\x0C\xDBa%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x14W\x81` \x01[a\r\x01a!UV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xF9W\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10VW`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\rGWa\rGa-oV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a-\xBEV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EL\x91\x90a,\xE1V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\x92Wa\x0E\x92a-oV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F8\x91\x90a.0V[\x81R` \x01\x85\x81R` \x01a\x0FL\x87a\x02rV[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x0Fd\x82a-\x9BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\x89Wa\x0F\x89a-oV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x10?W\x83Q`\0\x90a\x0F\xD8\x90\x84\x90a\x0F\xCF\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x0F\xCAWa\x0F\xCAa/\x13V[a \x0CV[`\x0F\x0B\x90a \xA5V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xF7Wa\x0F\xF7a-oV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x10\x13Wa\x10\x13a-oV[` \x02` \x01\x01\x81\x81Qa\x10'\x91\x90a/)V[`\x0F\x0B\x90RPa\x108\x90P\x81a/xV[\x90Pa\x0F\x97V[PPPPPP\x80a\x10O\x90a-\x9BV[\x90Pa\r\x1EV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13\x1EW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\x83Wa\x10\x83a-oV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\r\x91\x90a,\xACV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x88\x91\x90a,\xE1V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xCEWa\x11\xCEa-oV[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x12\x0E\x87a\x02rV[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12&\x82a-\x9BV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12KWa\x12Ka-oV[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13\x07W`\0\x84` \x01Qa\x12\x96\x84a\x0F\xCF\x88`\0\x01Qa\x0F\xCF\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x0F\xCAWa\x0F\xCAa/\x13V[a\x12\xA0\x91\x90a/)V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xBFWa\x12\xBFa-oV[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x12\xDBWa\x12\xDBa-oV[` \x02` \x01\x01\x81\x81Qa\x12\xEF\x91\x90a/)V[`\x0F\x0B\x90RPa\x13\0\x90P\x81a/xV[\x90Pa\x12YV[PPPPPP\x80a\x13\x17\x90a-\x9BV[\x90Pa\x10ZV[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x97\x91\x90a/\x97V[\x90P[\x80\x15a\x17#W`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17\x1BW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x13\xEAWa\x13\xEAa/\x13V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x08\x93\x92\x91\x90a/\xB0V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14I\x91\x90a/\xE1V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x14^WPa\x17\tV[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x14\x8FWa\x14\x8Fa/\x13V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xAD\x93\x92\x91\x90a/\xB0V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xEE\x91\x90a/\xE1V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15\x11WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15\x1DWPPa\x17\tV[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x15LW\x81Q\x83Qa\x15E\x91\x90a\x15@\x90a/\xFDV[a!$V[\x90Pa\x15oV[\x81Q\x83Qa\x15c\x91\x90a\x15^\x90a/\xFDV[a!@V[a\x15l\x90a/\xFDV[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x15\x87\x91\x90a/)V[a\x15\x91\x91\x90a0#V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x15\xE1W`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xBE\x91\x90a0xV[a\x15\xC8\x91\x90a0#V[a\x15\xDA\x90g\r\xE0\xB6\xB3\xA7d\0\0a0xV[\x90Pa\x16\x1AV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xFB\x91\x90a0xV[a\x16\x05\x91\x90a0#V[a\x16\x17\x90g\r\xE0\xB6\xB3\xA7d\0\0a0xV[\x90P[`\0`\x02a\x16Ma\x16+\x85\x85a0xV[a\x0F\xCF\x89` \x01Q\x89` \x01Qa\x16B\x91\x90a/)V[`\x0F\x89\x90\x0B\x90a \xA5V[a\x16W\x91\x90a0#V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16vWa\x16va-oV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\x92Wa\x16\x92a-oV[` \x02` \x01\x01\x81\x81Qa\x16\xA6\x91\x90a/)V[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x16\xCCWa\x16\xCCa-oV[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\xE8Wa\x16\xE8a-oV[` \x02` \x01\x01\x81\x81Qa\x16\xFC\x91\x90a/)V[`\x0F\x0B\x90RPPPPPPP[\x80a\x17\x13\x81a/xV[\x91PPa\x13\xB3V[PPPa\x13\x9AV[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19pW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x18\xC7W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17rWa\x17ra-oV[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\x94Wa\x17\x94a-oV[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x180W\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xC4Wa\x17\xC4a-oV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xE6Wa\x17\xE6a-oV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\rWa\x18\ra-oV[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18%\x91\x90a/)V[`\x0F\x0B\x90RPa\x18\xB7V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18OWa\x18Oa-oV[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18qWa\x18qa-oV[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x98Wa\x18\x98a-oV[` \x02` \x01\x01Q` \x01\x81\x81Qa\x18\xB0\x91\x90a0xV[`\x0F\x0B\x90RP[a\x18\xC0\x81a0\xC8V[\x90Pa\x17;V[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xE7Wa\x18\xE7a-oV[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x12Wa\x19\x12a-oV[` \x02` \x01\x01Q`\0\x01Qa\x19(\x91\x90a0xV[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19GWa\x19Ga-oV[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x19i\x90a0\xC8V[\x90Pa\x17&V[P\x92\x95\x94PPPPPV[a\x19\xA1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1B\x91\x90a-\xBEV[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A_Wa\x1A_a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x98W\x81` \x01[a\x1A\x85a!\xE5V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A}W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xC7Wa\x1A\xC7a-oV[` \x02` \x01\x01Q\x90Pa\x1A\xDA\x81a\x07\x05V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xF2Wa\x1A\xF2a-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B\t\x90a-\x9BV[\x91PPa\x1A\x9EV[P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1BuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF2\x91\x90a0\xE4V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1C4\x91`\x04\x01a1\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CQW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cu\x91\x90a0\xE4V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1C\xB9\x90`\x01\x90`\x04\x01a1\x01V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFA\x91\x90a0\xE4V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x1DP`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x1B\x91\x90a,\xACV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xE6Wa\x1D\xE6a%\xDEV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x1FW\x81` \x01[a\x1E\x0Ca!UV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\x04W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B\x11W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1ENWa\x1ENa-oV[` \x02` \x01\x01Q\x90Pa\x1Ea\x81a\x03\xA0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1EyWa\x1Eya-oV[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1E\x90\x90a-\x9BV[\x91PPa\x1E%V[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1E\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x06\x91\x90a0\xE4V[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FaW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x89\x91\x90\x81\x01\x90a1\x1BV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xDEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x06\x91\x90\x81\x01\x90a1\x1BV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a \"Wa \"a/\x13V[\x03a 6WPg\r\xE0\xB6\xB3\xA7d\0\0a \x9EV[`\0\x80\x84`\x0F\x0B\x12a oW`\0\x83`\x02\x81\x11\x15a VWa Va/\x13V[\x14a eW\x84`@\x01Qa hV[\x84Q[\x90Pa \x9BV[`\0\x83`\x02\x81\x11\x15a \x83Wa \x83a/\x13V[\x14a \x92W\x84``\x01Qa \x98V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a \xE3WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a!\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1Bl\x91\x90a1\xB5V[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a!9W\x81a \x9EV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a!9W\x81a \x9EV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a!\x94V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\"zW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\"\x8FW`\0\x80\xFD[\x815a \x9E\x81a\"hV[`\x80\x81\x01a\x1A=\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa#8`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa#\x9Ba\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa#\xB0a\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa$S\x87\x83Qa\"\xD5V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a$@V[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa$\xD5`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa%\x15`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa%y\x87\x83Qa$rV[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a%fV[` \x81R`\0\x82Q`@` \x84\x01Ra%\xA9``\x84\x01\x82a$,V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra%\xC6\x82\x82a%RV[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1A=\x82\x84a$rV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\x1DWa&\x1Da%\xDEV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&?Wa&?a%\xDEV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a&ZW`\0\x80\xFD[\x815` a&oa&j\x83a&%V[a%\xF4V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a&\x8EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a&\xB2W\x805a&\xA5\x81a\"hV[\x83R\x91\x83\x01\x91\x83\x01a&\x92V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a&\xD0W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xEEW`\0\x80\xFD[a&\xFA\x85\x82\x86\x01a&IV[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa$'` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa'g\x87\x83Qa'\x04V[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'TV[` \x81R`\0a \x9E` \x83\x01\x84a'@V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa'\xC8\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\xA1V[` \x81R`\0a \x9E` \x83\x01\x84a'\x8DV[a\x02\xC0\x81\x01a\x1A=\x82\x84a\"\xD5V[`\0` \x82\x84\x03\x12\x15a(\x0FW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a$gWa(]\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a(*V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a(\xE1W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a(\xCCW\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a(\xADV[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a(\x8FV[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa)\x0F`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra),a\x01`\x85\x01\x83a(\x16V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra)J\x84\x83a(pV[\x93P`\x80\x87\x01Q\x91Pa)e`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra)\x93\x84\x83a'\x8DV[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra)\xB2\x85\x84a'@V[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra)\xD1\x85\x84a$,V[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa)\xED\x83\x82a%RV[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a*\nW`\0\x80\xFD[\x825\x91P` \x83\x015a*\x1C\x81a\"hV[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1A=V[`\0` \x82\x84\x03\x12\x15a*ZW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*qW`\0\x80\xFD[a*}\x84\x82\x85\x01a&IV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a*\xC8Wa*\xB4\x83\x85Qa\"\xD5V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a*\xA1V[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"zW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a*\xFBW`\0\x80\xFD[\x815a \x9E\x81a*\xD4V[`\x80\x81\x01a\x1A=\x82\x84a'\x04V[` \x81R`\0a \x9E` \x83\x01\x84a%RV[\x80Q`\x0F\x81\x90\x0B\x81\x14a+9W`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a+PW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a+sWa+sa%\xDEV[`@R\x82Qa+\x81\x81a\"hV[\x81Ra+\x8F` \x84\x01a+'V[` \x82\x01Ra+\xA0`@\x84\x01a+'V[`@\x82\x01Ra+\xB1``\x84\x01a+'V[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a+\xCFW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a+\xF2Wa+\xF2a%\xDEV[`@R\x90P\x80a,\x01\x83a+'V[\x81Ra,\x0F` \x84\x01a+'V[` \x82\x01Ra, `@\x84\x01a+'V[`@\x82\x01Ra,1``\x84\x01a+'V[``\x82\x01RP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a,OW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a,rWa,ra%\xDEV[`@R\x90P\x80a,\x81\x83a+'V[\x81Ra,\x8F` \x84\x01a+'V[` \x82\x01Ra,\xA0`@\x84\x01a+'V[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a,\xBFW`\0\x80\xFD[a,\xC9\x84\x84a+\xBDV[\x91Pa,\xD8\x84`\x80\x85\x01a,=V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a,\xF3W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a-\x16Wa-\x16a%\xDEV[`@Ra-\"\x83a+'V[\x81Ra-0` \x84\x01a+'V[` \x82\x01Ra-A`@\x84\x01a+'V[`@\x82\x01Ra-R``\x84\x01a+'V[``\x82\x01Ra-c`\x80\x84\x01a+'V[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a-\xB4Wa-\xB4a-\x85V[`\x01\x01\x93\x92PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a-\xD2W`\0\x80\xFD[a-\xDC\x85\x85a+\xBDV[\x92P` `\x7F\x19\x82\x01\x12\x15a-\xF0W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.\x14Wa.\x14a%\xDEV[`@Ra.#`\x80\x85\x01a+'V[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a.BW`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.eWa.ea%\xDEV[`@R\x82Qa.s\x81a*\xD4V[\x81Ra.\x81` \x84\x01a+'V[` \x82\x01Ra.\x92`@\x84\x01a+'V[`@\x82\x01Ra.\xA3``\x84\x01a+'V[``\x82\x01Ra.\xB4`\x80\x84\x01a+'V[`\x80\x82\x01Ra.\xC5`\xA0\x84\x01a+'V[`\xA0\x82\x01Ra.\xD6`\xC0\x84\x01a+'V[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a.\xF4Wa.\xF4a-\x85V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a/\x0EWa/\x0Ea-\x85V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a/SWa/Sa-\x85V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a/oWa/oa-\x85V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a/\x8EWa/\x8Ea-\x85V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a/\xA9W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a/\xD3Wa/\xD3a/\x13V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a/\xF3W`\0\x80\xFD[a \x9E\x83\x83a,=V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a0\x1AWa0\x1Aa-\x85V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a0HWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a0oWa0oa-\x85V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a0\xA3Wa0\xA3a-\x85V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a0\xBEWa0\xBEa-\x85V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a-\xB4Wa-\xB4a-\x85V[`\0` \x82\x84\x03\x12\x15a0\xF6W`\0\x80\xFD[\x81Qa \x9E\x81a*\xD4V[` \x81\x01`\x02\x83\x10a1\x15Wa1\x15a/\x13V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a1.W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a1VW`\0\x80\xFD[\x80Qa1da&j\x82a&%V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a1\x83W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a1\xAAW\x83Qa1\x9B\x81a\"hV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a1\x88V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a1\xE2W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a1\xC6V[\x81\x81\x11\x15a1\xF4W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x15\nc\xEBI\"\xBF:Zb9\x8F\xB6<\x90U\xBF\xAD\xC8\xBC\xBF\xB8\xA3\x1Cz\xA3[\x14\xF8\xEC JdsolcC\0\x08\r\x003";
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
