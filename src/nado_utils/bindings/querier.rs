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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa5\xF3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ct\x174\x04\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\"W\x80c\xC4\xD6m\xE8\x14a\x02=W\x80c\xD7\xB2)\xB6\x14a\x02RW\x80c\xEE\x99(\xC9\x14a\x02rW`\0\x80\xFD[\x80ct\x174\x04\x14a\x01\xCDW\x80cu\xA5\xAB<\x14a\x01\xEDW\x80c\x97\x05\xF20\x14a\x02\rW`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xC8W\x80c%\x93\xEB_\x14a\x01MW\x80c1TmQ\x14a\x01mW\x80cW#e?\x14a\x01\x8DW\x80c]p.\x1A\x14a\x01\xADW`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xEFW\x80c\x02\xEE:R\x14a\x01\x18W\x80c\x1A\xE1\x0B\xC5\x14a\x01-W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a$\x9EV[a\x02\x92V[`@Qa\x01\x0F\x91\x90a$\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x03rV[`@Qa\x01\x0F\x91\x90a'\xAEV[a\x01@a\x01;6`\x04a$\x9EV[a\x03\xC0V[`@Qa\x01\x0F\x91\x90a'\xF0V[a\x01`a\x01[6`\x04a)\x07V[a\x059V[`@Qa\x01\x0F\x91\x90a)\xC4V[a\x01\x80a\x01{6`\x04a)\x07V[a\x06:V[`@Qa\x01\x0F\x91\x90a*%V[a\x01\xA0a\x01\x9B6`\x04a$\x9EV[a\x07'V[`@Qa\x01\x0F\x91\x90a*8V[a\x01\xC0a\x01\xBB6`\x04a*GV[a\t\x01V[`@Qa\x01\x0F\x91\x90a+9V[a\x01\xE0a\x01\xDB6`\x04a,AV[a\x19\xA7V[`@Qa\x01\x0F\x91\x90a,qV[a\x02\0a\x01\xFB6`\x04a,\x92V[a\x1AoV[`@Qa\x01\x0F\x91\x90a,\xCFV[a\x02\x15a\x1BCV[`@Qa\x01\x0F\x91\x90a-\x1EV[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[a\x02Pa\x02K6`\x04a-\xBEV[a\x1D8V[\0[a\x02ea\x02`6`\x04a,AV[a\x1F>V[`@Qa\x01\x0F\x91\x90a-\xDBV[a\x02\x85a\x02\x806`\x04a,\x92V[a\x1F\xEBV[`@Qa\x01\x0F\x91\x90a-\xE9V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\xBEa \xB9V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03.\x91\x90a.\x13V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03\x91a!,V[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\xA9\x84a\x1AoV[\x81R` \x01a\x03\xB7\x83a\x1F\xEBV[\x90R\x93\x92PPPV[a\x03\xC8a#vV[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04B\x91\x90a/aV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBD\x91\x90a/\x96V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05&W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05/V[a\x05/\x86a\x02\x92V[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05UWa\x05Ua'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xB9W\x81` \x01[a\x05\xA6`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05sW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x063W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a0$V[` \x02` \x01\x01Q\x90Pa\x05\xFC\x85\x82a\x1F>V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\x14Wa\x06\x14a0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06+\x90a0PV[\x91PPa\x05\xBFV[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06VWa\x06Va'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xADW\x81` \x01[a\x06\x9A`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06tW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x063W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xDCWa\x06\xDCa0$V[` \x02` \x01\x01Q\x90Pa\x06\xF0\x85\x82a\x19\xA7V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x08Wa\x07\x08a0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\x1F\x90a0PV[\x91PPa\x06\xB3V[a\x07/a$\x06V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA9\x91\x90a0sV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08$\x91\x90a/\x96V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBF\x91\x90a0\xE5V[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05&W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05/V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\t\xACa!,V[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\t\xD1WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\nkW\x81\x84\x82\x81Q\x81\x10a\n\"Wa\n\"a0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\nYW\x83\x81\x81Q\x81\x10a\nHWa\nHa0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\nc\x81a1\x97V[\x91PPa\n\x06V[P`\0[\x82Q\x81\x10\x15a\n\xD4W\x81\x83\x82\x81Q\x81\x10a\n\x8BWa\n\x8Ba0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xC2W\x82\x81\x81Q\x81\x10a\n\xB1Wa\n\xB1a0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n\xCC\x81a1\x97V[\x91PPa\noV[Pa\n\xE0\x81`\x01a1\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xF8Wa\n\xF8a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B+W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\x16W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0B\x8EW`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0BpWa\x0Bpa0$V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0B\x86\x90a1\x97V[\x91PPa\x0B4V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xAAWa\x0B\xAAa'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x01W\x81` \x01[a\x0B\xEE`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xC8W\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C!Wa\x0C!a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x85W\x81` \x01[a\x0Cr`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C?W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xA5Wa\x0C\xA5a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xDEW\x81` \x01[a\x0C\xCBa$\x06V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xC3W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xFFWa\x0C\xFFa'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r8W\x81` \x01[a\r%a#vV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x1DW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10|W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\rkWa\rka0$V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF5\x91\x90a0sV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0ENW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Er\x91\x90a/\x96V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\xB8Wa\x0E\xB8a0$V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90a0\xE5V[\x81R` \x01\x85\x81R` \x01a\x0Fr\x87a\x02\x92V[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x0F\x8A\x82a0PV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xAFWa\x0F\xAFa0$V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x10eW\x83Q`\0\x90a\x0F\xFE\x90\x84\x90a\x0F\xF5\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x0F\xF0Wa\x0F\xF0a1\xC8V[a\"-V[`\x0F\x0B\x90a\"\xC6V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\x1DWa\x10\x1Da0$V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x109Wa\x109a0$V[` \x02` \x01\x01\x81\x81Qa\x10M\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x10^\x90P\x81a2-V[\x90Pa\x0F\xBDV[PPPPPP\x80a\x10u\x90a0PV[\x90Pa\rBV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13FW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xA9Wa\x10\xA9a0$V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x113\x91\x90a/aV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB0\x91\x90a/\x96V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xF6Wa\x11\xF6a0$V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x126\x87a\x02\x92V[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12N\x82a0PV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12sWa\x12sa0$V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13/W`\0\x84` \x01Qa\x12\xBE\x84a\x0F\xF5\x88`\0\x01Qa\x0F\xF5\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x0F\xF0Wa\x0F\xF0a1\xC8V[a\x12\xC8\x91\x90a1\xDEV[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xE7Wa\x12\xE7a0$V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x13\x03Wa\x13\x03a0$V[` \x02` \x01\x01\x81\x81Qa\x13\x17\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x13(\x90P\x81a2-V[\x90Pa\x12\x81V[PPPPPP\x80a\x13?\x90a0PV[\x90Pa\x10\x80V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBF\x91\x90a2LV[\x90P[\x80\x15a\x17OW`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17GW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x14\x12Wa\x14\x12a1\xC8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x140\x93\x92\x91\x90a2eV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14s\x91\x90a2\x96V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x14\x88WPa\x175V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x14\xB9Wa\x14\xB9a1\xC8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD7\x93\x92\x91\x90a2eV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1A\x91\x90a2\x96V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15=WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15IWPPa\x175V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x15xW\x81Q\x83Qa\x15q\x91\x90a\x15l\x90a2\xB2V[a#EV[\x90Pa\x15\x9BV[\x81Q\x83Qa\x15\x8F\x91\x90a\x15\x8A\x90a2\xB2V[a#aV[a\x15\x98\x90a2\xB2V[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x15\xB3\x91\x90a1\xDEV[a\x15\xBD\x91\x90a2\xD8V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x16\rW`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xEA\x91\x90a3-V[a\x15\xF4\x91\x90a2\xD8V[a\x16\x06\x90g\r\xE0\xB6\xB3\xA7d\0\0a3-V[\x90Pa\x16FV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16'\x91\x90a3-V[a\x161\x91\x90a2\xD8V[a\x16C\x90g\r\xE0\xB6\xB3\xA7d\0\0a3-V[\x90P[`\0`\x02a\x16ya\x16W\x85\x85a3-V[a\x0F\xF5\x89` \x01Q\x89` \x01Qa\x16n\x91\x90a1\xDEV[`\x0F\x89\x90\x0B\x90a\"\xC6V[a\x16\x83\x91\x90a2\xD8V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\xA2Wa\x16\xA2a0$V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\xBEWa\x16\xBEa0$V[` \x02` \x01\x01\x81\x81Qa\x16\xD2\x91\x90a1\xDEV[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x16\xF8Wa\x16\xF8a0$V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17\x14Wa\x17\x14a0$V[` \x02` \x01\x01\x81\x81Qa\x17(\x91\x90a1\xDEV[`\x0F\x0B\x90RPPPPPPP[\x80a\x17?\x81a2-V[\x91PPa\x13\xDBV[PPPa\x13\xC2V[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19\x9CW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x18\xF3W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\x9EWa\x17\x9Ea0$V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xC0Wa\x17\xC0a0$V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x18\\W\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xF0Wa\x17\xF0a0$V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x12Wa\x18\x12a0$V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x189Wa\x189a0$V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18Q\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x18\xE3V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18{Wa\x18{a0$V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x9DWa\x18\x9Da0$V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xC4Wa\x18\xC4a0$V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x18\xDC\x91\x90a3-V[`\x0F\x0B\x90RP[a\x18\xEC\x81a3}V[\x90Pa\x17gV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x13Wa\x19\x13a0$V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19>Wa\x19>a0$V[` \x02` \x01\x01Q`\0\x01Qa\x19T\x91\x90a3-V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19sWa\x19sa0$V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x19\x95\x90a3}V[\x90Pa\x17RV[P\x92\x95\x94PPPPPV[a\x19\xCD`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AG\x91\x90a0sV[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x8BWa\x1A\x8Ba'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xC4W\x81` \x01[a\x1A\xB1a$\x06V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xA9W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B=W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xF3Wa\x1A\xF3a0$V[` \x02` \x01\x01Q\x90Pa\x1B\x06\x81a\x07'V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x1EWa\x1B\x1Ea0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B5\x90a0PV[\x91PPa\x1A\xCAV[P\x91\x90PV[`@\x80Q` \x81\x01\x90\x91R``\x81R`\x01T`@\x80Qc\x1C\x88m\x0B`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x1C\x88m\x0B\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xC3\x91\x90\x81\x01\x90a3\x99V[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xE1Wa\x1B\xE1a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C3W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x1B\xFFW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1D#W`@Q\x80`\x80\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1C_Wa\x1C_a0$V[` \x02` \x01\x01Q`\0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\x8CWa\x1C\x8Ca0$V[` \x02` \x01\x01Q` \x01Q\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\xAFWa\x1C\xAFa0$V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\xDBWa\x1C\xDBa0$V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1D\x05Wa\x1D\x05a0$V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x1D\x1B\x90a1\x97V[\x91PPa\x1C9V[P`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1D\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x13\x91\x90a4\x97V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1EU\x91`\x04\x01a4\xB4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ErW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x96\x91\x90a4\x97V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1E\xDA\x90`\x01\x90`\x04\x01a4\xB4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x1B\x91\x90a4\x97V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x1Fq`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AG\x91\x90a/aV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x07Wa \x07a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a @W\x81` \x01[a -a#vV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a %W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B=W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a oWa oa0$V[` \x02` \x01\x01Q\x90Pa \x82\x81a\x03\xC0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a \x9AWa \x9Aa0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a \xB1\x90a0PV[\x91PPa FV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!'\x91\x90a4\x97V[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAA\x91\x90\x81\x01\x90a4\xCEV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"'\x91\x90\x81\x01\x90a4\xCEV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"CWa\"Ca1\xC8V[\x03a\"WWPg\r\xE0\xB6\xB3\xA7d\0\0a\"\xBFV[`\0\x80\x84`\x0F\x0B\x12a\"\x90W`\0\x83`\x02\x81\x11\x15a\"wWa\"wa1\xC8V[\x14a\"\x86W\x84`@\x01Qa\"\x89V[\x84Q[\x90Pa\"\xBCV[`\0\x83`\x02\x81\x11\x15a\"\xA4Wa\"\xA4a1\xC8V[\x14a\"\xB3W\x84``\x01Qa\"\xB9V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\x04WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1D\x8D\x91\x90a5hV[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a#ZW\x81a\"\xBFV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a#ZW\x81a\"\xBFV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a#\xB5V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\x9BW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a$\xB0W`\0\x80\xFD[\x815a\"\xBF\x81a$\x89V[`\x80\x81\x01a\x1Ai\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa%Y`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa%\xBCa\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa%\xD1a\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa&t\x87\x83Qa$\xF6V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a&aV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa&\xF6`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa'6`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa'\x9A\x87\x83Qa&\x93V[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\x87V[` \x81R`\0\x82Q`@` \x84\x01Ra'\xCA``\x84\x01\x82a&MV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra'\xE7\x82\x82a'sV[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1Ai\x82\x84a&\x93V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(8Wa(8a'\xFFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(gWa(ga'\xFFV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\x89Wa(\x89a'\xFFV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a(\xA4W`\0\x80\xFD[\x815` a(\xB9a(\xB4\x83a(oV[a(>V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a(\xD8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xFCW\x805a(\xEF\x81a$\x89V[\x83R\x91\x83\x01\x91\x83\x01a(\xDCV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a)\x1AW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)8W`\0\x80\xFD[a)D\x85\x82\x86\x01a(\x93V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa&H` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa)\xB1\x87\x83Qa)NV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a)\x9EV[` \x81R`\0a\"\xBF` \x83\x01\x84a)\x8AV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa*\x12\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a)\xEBV[` \x81R`\0a\"\xBF` \x83\x01\x84a)\xD7V[a\x02\xC0\x81\x01a\x1Ai\x82\x84a$\xF6V[`\0` \x82\x84\x03\x12\x15a*YW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa*\xA7\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*tV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a++W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a+\x16W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a*\xF7V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a*\xD9V[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa+Y`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra+va\x01`\x85\x01\x83a*`V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra+\x94\x84\x83a*\xBAV[\x93P`\x80\x87\x01Q\x91Pa+\xAF`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra+\xDD\x84\x83a)\xD7V[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra+\xFC\x85\x84a)\x8AV[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra,\x1B\x85\x84a&MV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa,7\x83\x82a'sV[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,TW`\0\x80\xFD[\x825\x91P` \x83\x015a,f\x81a$\x89V[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1AiV[`\0` \x82\x84\x03\x12\x15a,\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xBBW`\0\x80\xFD[a,\xC7\x84\x82\x85\x01a(\x93V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\x12Wa,\xFE\x83\x85Qa$\xF6V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a,\xEBV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R`@\x80\x84\x01\x85Q\x83\x84\x87\x01R\x81\x81Q\x80\x84R``\x93P\x83\x88\x01\x91P\x85\x83\x01\x92P`\0[\x81\x81\x10\x15a-\x9BW\x83Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x85\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x84\x01R\x92\x86\x01\x92`\x80\x90\x92\x01\x91`\x01\x01a-HV[P\x90\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x9BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-\xD0W`\0\x80\xFD[\x815a\"\xBF\x81a-\xA9V[`\x80\x81\x01a\x1Ai\x82\x84a)NV[` \x81R`\0a\"\xBF` \x83\x01\x84a'sV[\x80Q`\x0F\x81\x90\x0B\x81\x14a.\x0EW`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a.%W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.HWa.Ha'\xFFV[`@R\x82Qa.V\x81a$\x89V[\x81Ra.d` \x84\x01a-\xFCV[` \x82\x01Ra.u`@\x84\x01a-\xFCV[`@\x82\x01Ra.\x86``\x84\x01a-\xFCV[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a.\xA4W`\0\x80\xFD[a.\xACa(\x15V[\x90Pa.\xB7\x82a-\xFCV[\x81Ra.\xC5` \x83\x01a-\xFCV[` \x82\x01Ra.\xD6`@\x83\x01a-\xFCV[`@\x82\x01Ra.\xE7``\x83\x01a-\xFCV[``\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a/\x04W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/'Wa/'a'\xFFV[`@R\x90P\x80a/6\x83a-\xFCV[\x81Ra/D` \x84\x01a-\xFCV[` \x82\x01Ra/U`@\x84\x01a-\xFCV[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a/tW`\0\x80\xFD[a/~\x84\x84a.\x92V[\x91Pa/\x8D\x84`\x80\x85\x01a.\xF2V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a/\xA8W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\xCBWa/\xCBa'\xFFV[`@Ra/\xD7\x83a-\xFCV[\x81Ra/\xE5` \x84\x01a-\xFCV[` \x82\x01Ra/\xF6`@\x84\x01a-\xFCV[`@\x82\x01Ra0\x07``\x84\x01a-\xFCV[``\x82\x01Ra0\x18`\x80\x84\x01a-\xFCV[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a0iWa0ia0:V[`\x01\x01\x93\x92PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a0\x87W`\0\x80\xFD[a0\x91\x85\x85a.\x92V[\x92P` `\x7F\x19\x82\x01\x12\x15a0\xA5W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0\xC9Wa0\xC9a'\xFFV[`@Ra0\xD8`\x80\x85\x01a-\xFCV[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a0\xF7W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\x1AWa1\x1Aa'\xFFV[`@R\x82Qa1(\x81a-\xA9V[\x81Ra16` \x84\x01a-\xFCV[` \x82\x01Ra1G`@\x84\x01a-\xFCV[`@\x82\x01Ra1X``\x84\x01a-\xFCV[``\x82\x01Ra1i`\x80\x84\x01a-\xFCV[`\x80\x82\x01Ra1z`\xA0\x84\x01a-\xFCV[`\xA0\x82\x01Ra1\x8B`\xC0\x84\x01a-\xFCV[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a1\xA9Wa1\xA9a0:V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a1\xC3Wa1\xC3a0:V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a2\x08Wa2\x08a0:V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a2$Wa2$a0:V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a2CWa2Ca0:V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2^W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a2\x88Wa2\x88a1\xC8V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a2\xA8W`\0\x80\xFD[a\"\xBF\x83\x83a.\xF2V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a2\xCFWa2\xCFa0:V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a2\xFDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3$Wa3$a0:V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a3XWa3Xa0:V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a3sWa3sa0:V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a0iWa0ia0:V[`\0` \x80\x83\x85\x03\x12\x15a3\xACW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3\xC4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a3\xD8W`\0\x80\xFD[\x81Qa3\xE6a(\xB4\x82a(oV[\x81\x81R`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a4\x05W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a4\x8BW`\x80\x85\x8A\x03\x12\x15a4#W`\0\x80\x81\xFD[a4+a(\x15V[\x85Q\x85\x81\x16\x81\x14a4<W`\0\x80\x81\xFD[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa4U\x81a-\xA9V[\x90\x82\x01R``\x86\x81\x01Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4uW`\0\x80\x81\xFD[\x90\x82\x01R\x82R`\x80\x94\x90\x94\x01\x93\x90\x85\x01\x90a4\nV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a4\xA9W`\0\x80\xFD[\x81Qa\"\xBF\x81a-\xA9V[` \x81\x01`\x02\x83\x10a4\xC8Wa4\xC8a1\xC8V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a4\xE1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xF8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a5\tW`\0\x80\xFD[\x80Qa5\x17a(\xB4\x82a(oV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a56W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a5]W\x83Qa5N\x81a$\x89V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a5;V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a5\x95W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a5yV[\x81\x81\x11\x15a5\xA7W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 s\x12i\xA0@\xAE0c\x1B\xA2l\xAA\x06\xC0\xCDU\x1CH\xE6\x1Eq\x90\xE3#\xE2\xB5\x1F\xFC\xAE_&kdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static QUERIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80ct\x174\x04\x11a\0\x8CW\x80c\xB1\xCB\x0FB\x11a\0fW\x80c\xB1\xCB\x0FB\x14a\x02\"W\x80c\xC4\xD6m\xE8\x14a\x02=W\x80c\xD7\xB2)\xB6\x14a\x02RW\x80c\xEE\x99(\xC9\x14a\x02rW`\0\x80\xFD[\x80ct\x174\x04\x14a\x01\xCDW\x80cu\xA5\xAB<\x14a\x01\xEDW\x80c\x97\x05\xF20\x14a\x02\rW`\0\x80\xFD[\x80c%\x93\xEB_\x11a\0\xC8W\x80c%\x93\xEB_\x14a\x01MW\x80c1TmQ\x14a\x01mW\x80cW#e?\x14a\x01\x8DW\x80c]p.\x1A\x14a\x01\xADW`\0\x80\xFD[\x80c\x01\xCF\xA9\xD1\x14a\0\xEFW\x80c\x02\xEE:R\x14a\x01\x18W\x80c\x1A\xE1\x0B\xC5\x14a\x01-W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a$\x9EV[a\x02\x92V[`@Qa\x01\x0F\x91\x90a$\xBBV[`@Q\x80\x91\x03\x90\xF3[a\x01 a\x03rV[`@Qa\x01\x0F\x91\x90a'\xAEV[a\x01@a\x01;6`\x04a$\x9EV[a\x03\xC0V[`@Qa\x01\x0F\x91\x90a'\xF0V[a\x01`a\x01[6`\x04a)\x07V[a\x059V[`@Qa\x01\x0F\x91\x90a)\xC4V[a\x01\x80a\x01{6`\x04a)\x07V[a\x06:V[`@Qa\x01\x0F\x91\x90a*%V[a\x01\xA0a\x01\x9B6`\x04a$\x9EV[a\x07'V[`@Qa\x01\x0F\x91\x90a*8V[a\x01\xC0a\x01\xBB6`\x04a*GV[a\t\x01V[`@Qa\x01\x0F\x91\x90a+9V[a\x01\xE0a\x01\xDB6`\x04a,AV[a\x19\xA7V[`@Qa\x01\x0F\x91\x90a,qV[a\x02\0a\x01\xFB6`\x04a,\x92V[a\x1AoV[`@Qa\x01\x0F\x91\x90a,\xCFV[a\x02\x15a\x1BCV[`@Qa\x01\x0F\x91\x90a-\x1EV[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[a\x02Pa\x02K6`\x04a-\xBEV[a\x1D8V[\0[a\x02ea\x02`6`\x04a,AV[a\x1F>V[`@Qa\x01\x0F\x91\x90a-\xDBV[a\x02\x85a\x02\x806`\x04a,\x92V[a\x1F\xEBV[`@Qa\x01\x0F\x91\x90a-\xE9V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R\x90a\x02\xBEa \xB9V[`@Qc\x1D\x02\x9BM`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x1D\x02\x9BM\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03.\x91\x90a.\x13V[\x90P`@Q\x80`\x80\x01`@R\x80\x82`@\x01Q`\x0F\x0B\x81R` \x01`\0`\x0F\x0B\x81R` \x01\x82` \x01Q`\x0F\x0B\x81R` \x01\x82``\x01Q`\x0F\x0B\x81RP\x91PP\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x80a\x03\x91a!,V[\x91P\x91P`@Q\x80`@\x01`@R\x80a\x03\xA9\x84a\x1AoV[\x81R` \x01a\x03\xB7\x83a\x1F\xEBV[\x90R\x93\x92PPPV[a\x03\xC8a#vV[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04B\x91\x90a/aV[P`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xBD\x91\x90a/\x96V[\x90P`@Q\x80`\xA0\x01`@R\x80\x85c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x80\x01Q`\x0F\x0B\x81R` \x01\x82\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05&W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05/V[a\x05/\x86a\x02\x92V[\x90R\x94\x93PPPPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05UWa\x05Ua'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xB9W\x81` \x01[a\x05\xA6`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05sW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x063W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a0$V[` \x02` \x01\x01Q\x90Pa\x05\xFC\x85\x82a\x1F>V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\x14Wa\x06\x14a0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x06+\x90a0PV[\x91PPa\x05\xBFV[P\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06VWa\x06Va'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xADW\x81` \x01[a\x06\x9A`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06tW\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x063W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x06\xDCWa\x06\xDCa0$V[` \x02` \x01\x01Q\x90Pa\x06\xF0\x85\x82a\x19\xA7V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x07\x08Wa\x07\x08a0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x07\x1F\x90a0PV[\x91PPa\x06\xB3V[a\x07/a$\x06V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0`$\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA9\x91\x90a0sV[P`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08$\x91\x90a/\x96V[`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x87\x16\x80\x82R`\x80\x84\x01Q`\x0F\x0B` \x83\x01R\x81\x83\x01\x84\x90R`\x02T\x92Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x92\x93P\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBF\x91\x90a0\xE5V[\x81R` \x01\x83\x81R` \x01\x85c\xFF\xFF\xFF\xFF\x16`\0\x03a\x05&W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x05/V[`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x92\x82\x01\x83\x90R\x82\x82\x01\x83\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x91\x90\x91R`\0\x80a\t\xACa!,V[\x86\x85R`\x01` \x86\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x94P\x90\x92P\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\t\xD1WPP`@\x84\x01R`\0\x80[\x83Q\x81\x10\x15a\nkW\x81\x84\x82\x81Q\x81\x10a\n\"Wa\n\"a0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\nYW\x83\x81\x81Q\x81\x10a\nHWa\nHa0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\nc\x81a1\x97V[\x91PPa\n\x06V[P`\0[\x82Q\x81\x10\x15a\n\xD4W\x81\x83\x82\x81Q\x81\x10a\n\x8BWa\n\x8Ba0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\n\xC2W\x82\x81\x81Q\x81\x10a\n\xB1Wa\n\xB1a0$V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x91P[\x80a\n\xCC\x81a1\x97V[\x91PPa\noV[Pa\n\xE0\x81`\x01a1\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xF8Wa\n\xF8a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B+W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\x16W\x90P[P``\x85\x01R`\0[\x81\x81\x11a\x0B\x8EW`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x85``\x01Q\x82\x81Q\x81\x10a\x0BpWa\x0Bpa0$V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x0B\x86\x90a1\x97V[\x91PPa\x0B4V[PP\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xAAWa\x0B\xAAa'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x01W\x81` \x01[a\x0B\xEE`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0B\xC8W\x90P[P`\xC0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C!Wa\x0C!a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x85W\x81` \x01[a\x0Cr`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C?W\x90P[P`\xE0\x84\x01R\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xA5Wa\x0C\xA5a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xDEW\x81` \x01[a\x0C\xCBa$\x06V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xC3W\x90P[Pa\x01\0\x84\x01R\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xFFWa\x0C\xFFa'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r8W\x81` \x01[a\r%a#vV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\r\x1DW\x90P[Pa\x01 \x84\x01R`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x10|W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\rkWa\rka0$V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF5\x91\x90a0sV[`\x02T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0ENW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Er\x91\x90a/\x96V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xC0\x01Q\x8A`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0E\xB8Wa\x0E\xB8a0$V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`@\x80Q`\xC0\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x16\x80\x82R`\x0F\x85\x90\x0B\x93\x82\x01\x93\x90\x93R\x80\x82\x01\x85\x90R`\x02T\x91Qc8\xD0\xDC\xE3`\xE2\x1B\x81R`\x04\x81\x01\x93\x90\x93R\x91``\x83\x01\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F:W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F^\x91\x90a0\xE5V[\x81R` \x01\x85\x81R` \x01a\x0Fr\x87a\x02\x92V[\x90Ra\x01\0\x8A\x01Q`\x80\x8B\x01\x80Q\x90a\x0F\x8A\x82a0PV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x0F\xAFWa\x0F\xAFa0$V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x10eW\x83Q`\0\x90a\x0F\xFE\x90\x84\x90a\x0F\xF5\x90\x81\x88\x82`\xFF\x89\x16`\x02\x81\x11\x15a\x0F\xF0Wa\x0F\xF0a1\xC8V[a\"-V[`\x0F\x0B\x90a\"\xC6V[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\x1DWa\x10\x1Da0$V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x109Wa\x109a0$V[` \x02` \x01\x01\x81\x81Qa\x10M\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x10^\x90P\x81a2-V[\x90Pa\x0F\xBDV[PPPPPP\x80a\x10u\x90a0PV[\x90Pa\rBV[P`\0[\x81Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x13FW`\0\x82\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x10\xA9Wa\x10\xA9a0$V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x8A\x90R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x113\x91\x90a/aV[`\x03T`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x92\x94P\x90\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x11\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB0\x91\x90a/\x96V[\x90P`\0\x81`\x80\x01Q\x90P`@Q\x80`@\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x81RP\x89`\xE0\x01Q\x8A`\xA0\x01Qc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x11\xF6Wa\x11\xF6a0$V[` \x02` \x01\x01\x81\x90RP`@Q\x80`\xA0\x01`@R\x80\x86c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82`\x0F\x0B\x81R` \x01\x83\x81R` \x01\x85\x81R` \x01a\x126\x87a\x02\x92V[\x90Ra\x01 \x8A\x01Q`\xA0\x8B\x01\x80Q\x90a\x12N\x82a0PV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RPc\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12sWa\x12sa0$V[` \x02` \x01\x01\x81\x90RP`\0[`\x03\x81`\xFF\x16\x10\x15a\x13/W`\0\x84` \x01Qa\x12\xBE\x84a\x0F\xF5\x88`\0\x01Qa\x0F\xF5\x89\x8B`\0\x01Q\x89`\xFF\x16`\x02\x81\x11\x15a\x0F\xF0Wa\x0F\xF0a1\xC8V[a\x12\xC8\x91\x90a1\xDEV[\x90P\x80\x8B``\x01Q\x88c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x12\xE7Wa\x12\xE7a0$V[` \x02` \x01\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x13\x03Wa\x13\x03a0$V[` \x02` \x01\x01\x81\x81Qa\x13\x17\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x13(\x90P\x81a2-V[\x90Pa\x12\x81V[PPPPPP\x80a\x13?\x90a0PV[\x90Pa\x10\x80V[P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xF1m\xEC\x06`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBF\x91\x90a2LV[\x90P[\x80\x15a\x17OW`\x10\x81\x90\x1C\x90`\xFF\x80\x82\x16\x91`\x08\x1C\x16`\0[`\x03\x81`\xFF\x16\x10\x15a\x17GW`\x03T`\0\x90`\x01`\x01`\xA0\x1B\x03\x16c\x8A\x1DC\xC9\x8B\x85`\xFF\x86\x16`\x02\x81\x11\x15a\x14\x12Wa\x14\x12a1\xC8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x140\x93\x92\x91\x90a2eV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14s\x91\x90a2\x96V[\x80Q\x90\x91P`\x0F\x0B`\0\x03a\x14\x88WPa\x175V[`\x02\x80T`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8A\x1DC\xC9\x90\x8D\x90\x88\x90`\xFF\x88\x16\x90\x81\x11\x15a\x14\xB9Wa\x14\xB9a1\xC8V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD7\x93\x92\x91\x90a2eV[```@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x1A\x91\x90a2\x96V[\x80Q\x90\x91P`\x0F\x0B\x15\x80a\x15=WP\x81Q\x81Q`\0`\x0F\x91\x82\x0B\x81\x12\x92\x90\x91\x0B\x13\x14[\x15a\x15IWPPa\x175V[`\0\x80\x82`\0\x01Q`\x0F\x0B\x13\x15a\x15xW\x81Q\x83Qa\x15q\x91\x90a\x15l\x90a2\xB2V[a#EV[\x90Pa\x15\x9BV[\x81Q\x83Qa\x15\x8F\x91\x90a\x15\x8A\x90a2\xB2V[a#aV[a\x15\x98\x90a2\xB2V[\x90P[`\0`\x02\x84`@\x01Q\x84`@\x01Qa\x15\xB3\x91\x90a1\xDEV[a\x15\xBD\x91\x90a2\xD8V[\x90P`\0\x80\x84`\0\x01Q`\x0F\x0B\x13\x15a\x16\rW`\x05\x85`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xEA\x91\x90a3-V[a\x15\xF4\x91\x90a2\xD8V[a\x16\x06\x90g\r\xE0\xB6\xB3\xA7d\0\0a3-V[\x90Pa\x16FV[`\x05\x84`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x16'\x91\x90a3-V[a\x161\x91\x90a2\xD8V[a\x16C\x90g\r\xE0\xB6\xB3\xA7d\0\0a3-V[\x90P[`\0`\x02a\x16ya\x16W\x85\x85a3-V[a\x0F\xF5\x89` \x01Q\x89` \x01Qa\x16n\x91\x90a1\xDEV[`\x0F\x89\x90\x0B\x90a\"\xC6V[a\x16\x83\x91\x90a2\xD8V[\x90P\x80\x8D``\x01Q\x8Ac\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x16\xA2Wa\x16\xA2a0$V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x16\xBEWa\x16\xBEa0$V[` \x02` \x01\x01\x81\x81Qa\x16\xD2\x91\x90a1\xDEV[`\x0F\x0B\x90RP``\x8D\x01Q\x80Q\x82\x91\x90c\xFF\xFF\xFF\xFF\x8B\x16\x90\x81\x10a\x16\xF8Wa\x16\xF8a0$V[` \x02` \x01\x01Q\x88`\xFF\x16\x81Q\x81\x10a\x17\x14Wa\x17\x14a0$V[` \x02` \x01\x01\x81\x81Qa\x17(\x91\x90a1\xDEV[`\x0F\x0B\x90RPPPPPPP[\x80a\x17?\x81a2-V[\x91PPa\x13\xDBV[PPPa\x13\xC2V[`\0[`\x03\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x19\x9CW`\0[\x85``\x01QQ\x81`\x01`\x01`\x80\x1B\x03\x16\x10\x15a\x18\xF3W`\0\x86``\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\x9EWa\x17\x9Ea0$V[` \x02` \x01\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xC0Wa\x17\xC0a0$V[` \x02` \x01\x01Q`\x0F\x0B\x13\x15a\x18\\W\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x17\xF0Wa\x17\xF0a0$V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x12Wa\x18\x12a0$V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x189Wa\x189a0$V[` \x02` \x01\x01Q`\0\x01\x81\x81Qa\x18Q\x91\x90a1\xDEV[`\x0F\x0B\x90RPa\x18\xE3V[\x85``\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18{Wa\x18{a0$V[` \x02` \x01\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\x9DWa\x18\x9Da0$V[` \x02` \x01\x01Q\x86`@\x01Q\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x18\xC4Wa\x18\xC4a0$V[` \x02` \x01\x01Q` \x01\x81\x81Qa\x18\xDC\x91\x90a3-V[`\x0F\x0B\x90RP[a\x18\xEC\x81a3}V[\x90Pa\x17gV[P\x84`@\x01Q\x81`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19\x13Wa\x19\x13a0$V[` \x02` \x01\x01Q` \x01Q\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19>Wa\x19>a0$V[` \x02` \x01\x01Q`\0\x01Qa\x19T\x91\x90a3-V[\x85`@\x01Q\x82`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x19sWa\x19sa0$V[` \x02` \x01\x01Q`@\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x80a\x19\x95\x90a3}V[\x90Pa\x17RV[P\x92\x95\x94PPPPPV[a\x19\xCD`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q` \x81\x81\x01\x90\x94R\x90\x81R\x90\x91\x82\x01R\x90V[`\x02T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AG\x91\x90a0sV[\x91PP`@Q\x80`@\x01`@R\x80\x84c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x82\x81RP\x91PP[\x92\x91PPV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\x8BWa\x1A\x8Ba'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xC4W\x81` \x01[a\x1A\xB1a$\x06V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1A\xA9W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B=W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1A\xF3Wa\x1A\xF3a0$V[` \x02` \x01\x01Q\x90Pa\x1B\x06\x81a\x07'V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a\x1B\x1EWa\x1B\x1Ea0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x1B5\x90a0PV[\x91PPa\x1A\xCAV[P\x91\x90PV[`@\x80Q` \x81\x01\x90\x91R``\x81R`\x01T`@\x80Qc\x1C\x88m\x0B`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x1C\x88m\x0B\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1B\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\xC3\x91\x90\x81\x01\x90a3\x99V[\x90P`\0\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xE1Wa\x1B\xE1a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C3W\x81` \x01[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x1B\xFFW\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x1D#W`@Q\x80`\x80\x01`@R\x80\x84\x83\x81Q\x81\x10a\x1C_Wa\x1C_a0$V[` \x02` \x01\x01Q`\0\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\x8CWa\x1C\x8Ca0$V[` \x02` \x01\x01Q` \x01Q\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\xAFWa\x1C\xAFa0$V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x83\x81Q\x81\x10a\x1C\xDBWa\x1C\xDBa0$V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81RP\x82\x82\x81Q\x81\x10a\x1D\x05Wa\x1D\x05a0$V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x1D\x1B\x90a1\x97V[\x91PPa\x1C9V[P`@\x80Q` \x81\x01\x90\x91R\x90\x81R\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x1D\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7Falready initialized.\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@\x80Qc\xAE\xD8\xE9g`\xE0\x1B\x81R\x90Qc\xAE\xD8\xE9g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1D\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x13\x91\x90a4\x97V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0\x80T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x92\x16\x91c].\x9A\xD1\x91a\x1EU\x91`\x04\x01a4\xB4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ErW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x96\x91\x90a4\x97V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\0T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x1E\xDA\x90`\x01\x90`\x04\x01a4\xB4V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x1B\x91\x90a4\x97V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[a\x1Fq`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R\x82Q``\x81\x01\x84R\x81\x81R` \x81\x81\x01\x83\x90R\x93\x81\x01\x91\x90\x91R\x90\x91\x82\x01R\x90V[`\x03T`@Qc\xE34\xBE3`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE34\xBE3\x90`D\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AG\x91\x90a/aV[``\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \x07Wa \x07a'\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a @W\x81` \x01[a -a#vV[\x81R` \x01\x90`\x01\x90\x03\x90\x81a %W\x90P[P\x90P`\0[\x82Q\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1B=W`\0\x83\x82c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a oWa oa0$V[` \x02` \x01\x01Q\x90Pa \x82\x81a\x03\xC0V[\x83\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a \x9AWa \x9Aa0$V[` \x02` \x01\x01\x81\x90RPP\x80\x80a \xB1\x90a0PV[\x91PPa FV[`\x01T`@\x80Qc#\xD3\xE3\xB3`\xE2\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x8FO\x8E\xCC\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!'\x91\x90a4\x97V[\x90P\x90V[``\x80`\x02`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xAA\x91\x90\x81\x01\x90a4\xCEV[\x91P`\x03`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"'\x91\x90\x81\x01\x90a4\xCEV[\x90P\x90\x91V[`\0`\x02\x82`\x02\x81\x11\x15a\"CWa\"Ca1\xC8V[\x03a\"WWPg\r\xE0\xB6\xB3\xA7d\0\0a\"\xBFV[`\0\x80\x84`\x0F\x0B\x12a\"\x90W`\0\x83`\x02\x81\x11\x15a\"wWa\"wa1\xC8V[\x14a\"\x86W\x84`@\x01Qa\"\x89V[\x84Q[\x90Pa\"\xBCV[`\0\x83`\x02\x81\x11\x15a\"\xA4Wa\"\xA4a1\xC8V[\x14a\"\xB3W\x84``\x01Qa\"\xB9V[\x84` \x01Q[\x90P[\x90P[\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90a#\x04WP`\x01`\x01`\x7F\x1B\x03\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90a#=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1D\x8D\x91\x90a5hV[P\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x12a#ZW\x81a\"\xBFV[P\x90\x91\x90PV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x13a#ZW\x81a\"\xBFV[`@\x80Q`\xA0\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x84Q\x92\x83\x01\x85R\x81\x83R\x82\x01\x81\x90R\x81\x84\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01R\x90\x91\x82\x01\x90[\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01\x90\x81R`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01R\x91\x01R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x83Q`\xA0\x81\x01\x85R\x82\x81R\x90\x81\x01\x82\x90R\x80\x84\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91R\x90\x91\x82\x01\x90\x81R`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x81\x01\x82\x90R\x92\x82\x01\x81\x90R``\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R\x91\x01\x90a#\xB5V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$\x9BW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a$\xB0W`\0\x80\xFD[\x815a\"\xBF\x81a$\x89V[`\x80\x81\x01a\x1Ai\x82\x84\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa%Y`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81Q\x16`\xE0\x84\x01R` \x81\x01Q`\x0F\x0Ba\x01\0\x84\x01R`@\x81\x01Q`\x0F\x0Ba\x01 \x84\x01R``\x81\x01Q`\x0F\x0Ba\x01@\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01`\x84\x01R`\xA0\x81\x01Qa%\xBCa\x01\x80\x85\x01\x82`\x0F\x0B\x90RV[P`\xC0\x01Qa%\xD1a\x01\xA0\x84\x01\x82`\x0F\x0B\x90RV[P`\x80\x81\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01\xC0\x85\x01R` \x82\x01Q\x81\x0Ba\x01\xE0\x85\x01R`@\x82\x01Q\x81\x0Ba\x02\0\x85\x01R``\x82\x01Q\x90\x0Ba\x02 \x84\x01RP`\xA0\x01Q\x80Q`\x0F\x90\x81\x0Ba\x02@\x84\x01R` \x82\x01Q\x81\x0Ba\x02`\x84\x01R`@\x82\x01Q\x81\x0Ba\x02\x80\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x02\xA0\x90\x91\x01RV[PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa&t\x87\x83Qa$\xF6V[a\x02\xC0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a&aV[P\x94\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Qa&\xF6`@\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P``\x81\x01Qa'6`\xE0\x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x0F\x0B``\x83\x01RPPV[P`\x80\x01Q\x80Q`\x0F\x90\x81\x0Ba\x01`\x84\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x90\x91\x01Q\x90\x0Ba\x01\xC0\x90\x91\x01RV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa'\x9A\x87\x83Qa&\x93V[a\x01\xE0\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a'\x87V[` \x81R`\0\x82Q`@` \x84\x01Ra'\xCA``\x84\x01\x82a&MV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra'\xE7\x82\x82a'sV[\x95\x94PPPPPV[a\x01\xE0\x81\x01a\x1Ai\x82\x84a&\x93V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(8Wa(8a'\xFFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a(gWa(ga'\xFFV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\x89Wa(\x89a'\xFFV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a(\xA4W`\0\x80\xFD[\x815` a(\xB9a(\xB4\x83a(oV[a(>V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a(\xD8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xFCW\x805a(\xEF\x81a$\x89V[\x83R\x91\x83\x01\x91\x83\x01a(\xDCV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a)\x1AW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)8W`\0\x80\xFD[a)D\x85\x82\x86\x01a(\x93V[\x91PP\x92P\x92\x90PV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Qa&H` \x84\x01\x82\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa)\xB1\x87\x83Qa)NV[`\x80\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a)\x9EV[` \x81R`\0a\"\xBF` \x83\x01\x84a)\x8AV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa*\x12\x87\x83Q\x80Qc\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x01QQ`\x0F\x0B\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a)\xEBV[` \x81R`\0a\"\xBF` \x83\x01\x84a)\xD7V[a\x02\xC0\x81\x01a\x1Ai\x82\x84a$\xF6V[`\0` \x82\x84\x03\x12\x15a*YW`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a&\x88Wa*\xA7\x87\x83Q\x80Q`\x0F\x0B\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01RPPV[``\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a*tV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a++W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15a+\x16W\x83Q`\x0F\x0B\x83R\x92\x89\x01\x92\x91\x89\x01\x91`\x01\x01a*\xF7V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01a*\xD9V[P\x92\x98\x97PPPPPPPPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qa+Y`@\x84\x01\x82\x15\x15\x90RV[P`@\x83\x01Qa\x01@\x80``\x85\x01Ra+va\x01`\x85\x01\x83a*`V[\x91P``\x85\x01Q`\x1F\x19\x80\x86\x85\x03\x01`\x80\x87\x01Ra+\x94\x84\x83a*\xBAV[\x93P`\x80\x87\x01Q\x91Pa+\xAF`\xA0\x87\x01\x83c\xFF\xFF\xFF\xFF\x16\x90RV[`\xA0\x87\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x88\x01R\x91P`\xC0\x87\x01Q\x91P\x80\x86\x85\x03\x01`\xE0\x87\x01Ra+\xDD\x84\x83a)\xD7V[\x93P`\xE0\x87\x01Q\x91Pa\x01\0\x81\x87\x86\x03\x01\x81\x88\x01Ra+\xFC\x85\x84a)\x8AV[\x94P\x80\x88\x01Q\x92PPa\x01 \x81\x87\x86\x03\x01\x81\x88\x01Ra,\x1B\x85\x84a&MV[\x90\x88\x01Q\x87\x82\x03\x90\x92\x01\x84\x88\x01R\x93P\x90Pa,7\x83\x82a'sV[\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a,TW`\0\x80\xFD[\x825\x91P` \x83\x015a,f\x81a$\x89V[\x80\x91PP\x92P\x92\x90PV[\x81Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x83\x01QQ`\x0F\x0B\x90\x82\x01R`@\x81\x01a\x1AiV[`\0` \x82\x84\x03\x12\x15a,\xA4W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xBBW`\0\x80\xFD[a,\xC7\x84\x82\x85\x01a(\x93V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a-\x12Wa,\xFE\x83\x85Qa$\xF6V[\x92\x84\x01\x92a\x02\xC0\x92\x90\x92\x01\x91`\x01\x01a,\xEBV[P\x90\x96\x95PPPPPPV[`\0` \x80\x83R`@\x80\x84\x01\x85Q\x83\x84\x87\x01R\x81\x81Q\x80\x84R``\x93P\x83\x88\x01\x91P\x85\x83\x01\x92P`\0[\x81\x81\x10\x15a-\x9BW\x83Q\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84R\x87\x81\x01Q\x88\x85\x01R\x86\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x87\x85\x01R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x85\x84\x01R\x92\x86\x01\x92`\x80\x90\x92\x01\x91`\x01\x01a-HV[P\x90\x98\x97PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x9BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a-\xD0W`\0\x80\xFD[\x815a\"\xBF\x81a-\xA9V[`\x80\x81\x01a\x1Ai\x82\x84a)NV[` \x81R`\0a\"\xBF` \x83\x01\x84a'sV[\x80Q`\x0F\x81\x90\x0B\x81\x14a.\x0EW`\0\x80\xFD[\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a.%W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a.HWa.Ha'\xFFV[`@R\x82Qa.V\x81a$\x89V[\x81Ra.d` \x84\x01a-\xFCV[` \x82\x01Ra.u`@\x84\x01a-\xFCV[`@\x82\x01Ra.\x86``\x84\x01a-\xFCV[``\x82\x01R\x93\x92PPPV[`\0`\x80\x82\x84\x03\x12\x15a.\xA4W`\0\x80\xFD[a.\xACa(\x15V[\x90Pa.\xB7\x82a-\xFCV[\x81Ra.\xC5` \x83\x01a-\xFCV[` \x82\x01Ra.\xD6`@\x83\x01a-\xFCV[`@\x82\x01Ra.\xE7``\x83\x01a-\xFCV[``\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a/\x04W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/'Wa/'a'\xFFV[`@R\x90P\x80a/6\x83a-\xFCV[\x81Ra/D` \x84\x01a-\xFCV[` \x82\x01Ra/U`@\x84\x01a-\xFCV[`@\x82\x01RP\x92\x91PPV[`\0\x80`\xE0\x83\x85\x03\x12\x15a/tW`\0\x80\xFD[a/~\x84\x84a.\x92V[\x91Pa/\x8D\x84`\x80\x85\x01a.\xF2V[\x90P\x92P\x92\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a/\xA8W`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/\xCBWa/\xCBa'\xFFV[`@Ra/\xD7\x83a-\xFCV[\x81Ra/\xE5` \x84\x01a-\xFCV[` \x82\x01Ra/\xF6`@\x84\x01a-\xFCV[`@\x82\x01Ra0\x07``\x84\x01a-\xFCV[``\x82\x01Ra0\x18`\x80\x84\x01a-\xFCV[`\x80\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x03a0iWa0ia0:V[`\x01\x01\x93\x92PPPV[`\0\x80\x82\x84\x03`\xA0\x81\x12\x15a0\x87W`\0\x80\xFD[a0\x91\x85\x85a.\x92V[\x92P` `\x7F\x19\x82\x01\x12\x15a0\xA5W`\0\x80\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a0\xC9Wa0\xC9a'\xFFV[`@Ra0\xD8`\x80\x85\x01a-\xFCV[\x81R\x80\x91PP\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a0\xF7W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a1\x1AWa1\x1Aa'\xFFV[`@R\x82Qa1(\x81a-\xA9V[\x81Ra16` \x84\x01a-\xFCV[` \x82\x01Ra1G`@\x84\x01a-\xFCV[`@\x82\x01Ra1X``\x84\x01a-\xFCV[``\x82\x01Ra1i`\x80\x84\x01a-\xFCV[`\x80\x82\x01Ra1z`\xA0\x84\x01a-\xFCV[`\xA0\x82\x01Ra1\x8B`\xC0\x84\x01a-\xFCV[`\xC0\x82\x01R\x93\x92PPPV[`\0`\x01\x82\x01a1\xA9Wa1\xA9a0:V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a1\xC3Wa1\xC3a0:V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15a2\x08Wa2\x08a0:V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15a2$Wa2$a0:V[P\x01\x93\x92PPPV[`\0`\xFF\x82\x16`\xFF\x81\x03a2CWa2Ca0:V[`\x01\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a2^W`\0\x80\xFD[PQ\x91\x90PV[\x83\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01R``\x81\x01`\x03\x83\x10a2\x88Wa2\x88a1\xC8V[\x82`@\x83\x01R\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a2\xA8W`\0\x80\xFD[a\"\xBF\x83\x83a.\xF2V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a2\xCFWa2\xCFa0:V[`\0\x03\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80a2\xFDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15a3$Wa3$a0:V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15a3XWa3Xa0:V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15a3sWa3sa0:V[P\x90\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a0iWa0ia0:V[`\0` \x80\x83\x85\x03\x12\x15a3\xACW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3\xC4W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a3\xD8W`\0\x80\xFD[\x81Qa3\xE6a(\xB4\x82a(oV[\x81\x81R`\x07\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15a4\x05W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a4\x8BW`\x80\x85\x8A\x03\x12\x15a4#W`\0\x80\x81\xFD[a4+a(\x15V[\x85Q\x85\x81\x16\x81\x14a4<W`\0\x80\x81\xFD[\x81R\x85\x87\x01Q\x87\x82\x01R`@\x80\x87\x01Qa4U\x81a-\xA9V[\x90\x82\x01R``\x86\x81\x01Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a4uW`\0\x80\x81\xFD[\x90\x82\x01R\x82R`\x80\x94\x90\x94\x01\x93\x90\x85\x01\x90a4\nV[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a4\xA9W`\0\x80\xFD[\x81Qa\"\xBF\x81a-\xA9V[` \x81\x01`\x02\x83\x10a4\xC8Wa4\xC8a1\xC8V[\x91\x90R\x90V[`\0` \x80\x83\x85\x03\x12\x15a4\xE1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xF8W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a5\tW`\0\x80\xFD[\x80Qa5\x17a(\xB4\x82a(oV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a56W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a5]W\x83Qa5N\x81a$\x89V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a5;V[\x97\x96PPPPPPPV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a5\x95W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a5yV[\x81\x81\x11\x15a5\xA7W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 s\x12i\xA0@\xAE0c\x1B\xA2l\xAA\x06\xC0\xCDU\x1CH\xE6\x1Eq\x90\xE3#\xE2\xB5\x1F\xFC\xAE_&kdsolcC\0\x08\r\x003";
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
