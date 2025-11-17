pub use endpoint::*;
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
pub mod endpoint {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addNlpPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AddNlpPool",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AddNlpPool",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertCode"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertCode",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertCode",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assertProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assertProduct"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertProduct",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.AssertProduct",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainlinkFullReport"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainlinkFullReport",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                    3usize,
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkFullReport",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                    3usize,
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkFullReport",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainlinkReportBlob"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("chainlinkReportBlob",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkReportBlob",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.ChainlinkReportBlob",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSlowModeTxInner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSlowModeTxInner",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSlowModeTxLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSlowModeTxLinkSigner",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("clearinghouse"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearinghouse"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("closeIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("closeIsolatedSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.CloseIsolatedSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.CloseIsolatedSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createIsolatedSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
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
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deleteNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deleteNlpPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DeleteNlpPool",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DeleteNlpPool",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccountName"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(12usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes12"),
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
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateralWithReferral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositCollateralWithReferral",),
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
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeSlowModeTransaction",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHealthCheckFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHealthCheckFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLinkedSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLinkedSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getLiquidationFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLiquidationFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNlpPools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNlpPools"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.NlpPool[]",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNumSubaccounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNumSubaccounts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOffchainExchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOffchainExchange",),
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
                    ::std::borrow::ToOwned::to_owned("getPriceX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceX18"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_priceX18"),
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
                    ::std::borrow::ToOwned::to_owned("getSequencer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSequencer"),
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
                    ::std::borrow::ToOwned::to_owned("getSequencerFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSequencerFee"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("nSubmissionsSlot"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSlowModeTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlowModeTx"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("idx"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeTx",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSubaccountById"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubaccountById"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccountId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("getSubaccountId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSubaccountId"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("subaccount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTakerSequencerFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTakerSequencerFee",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTime"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTime"),
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
                    ::std::borrow::ToOwned::to_owned("getTimes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTimes"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Endpoint.Times"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incrementSubmissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("incrementSubmissions",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sanctions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_sequencer"),
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
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IClearinghouse"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_verifier"),
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
                    ::std::borrow::ToOwned::to_owned("liquidationStart"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidationStart"),
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
                    ::std::borrow::ToOwned::to_owned("manualAssert"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("manualAssert"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.ManualAssert",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.ManualAssert",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrders",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MatchOrders",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("matchOrdersWithAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("matchOrdersWithAmount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.MatchOrdersWithAmount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.MatchOrdersWithAmount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nSubmissions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nSubmissions"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nlpPools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nlpPools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
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
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balanceWeightX18"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("perpTick"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpTick"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.PerpTick",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.PerpTick",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processSlowModeTransaction",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transaction"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.RebalanceXWithdraw",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.RebalanceXWithdraw",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rebate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.Rebate"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.Rebate"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("referralCodes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("referralCodes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("resyncSlowModeTxs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resyncSlowModeTxs"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setInitialPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setInitialPrice"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialPriceX18"),
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
                    ::std::borrow::ToOwned::to_owned("setPriceX18"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPriceX18"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_priceX18"),
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
                    ::std::borrow::ToOwned::to_owned("setSlowModeConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSlowModeConfig"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_slowModeConfig"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeConfig",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSlowModeTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSlowModeTx"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IEndpoint.SlowModeTx",),
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
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SettlePnl",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SettlePnl",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedBurnNlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedBurnNlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnNlp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedBurnNlp",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedCancellation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedCancellation"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellation",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellation",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedCancellationProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedCancellationProducts",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellationProducts",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct FEndpoint.SignedCancellationProducts",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedLinkSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLinkSigner",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLinkSigner",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedLiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedMintNlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedMintNlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintNlp",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedMintNlp",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedOrder"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedOrder",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SignedOrder",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedTransferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedTransferQuote",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedTransferQuote",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedTransferQuote",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("signedWithdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("signedWithdrawCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedWithdrawCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.SignedWithdrawCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("spotTick"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("spotTick"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SpotTick",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.SpotTick",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitSlowModeTransaction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSlowModeTransaction",),
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
                    ::std::borrow::ToOwned::to_owned("submitTransactions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitTransactions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transactions"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactionsChecked"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitTransactionsChecked",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transactions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("e"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signerBitmask"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactionsCheckedWithGasLimit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "submitTransactionsCheckedWithGasLimit",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transactions"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("transferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferQuote"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedBurnNlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedBurnNlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnNlp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.BurnNlp"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDelistProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDelistProduct",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DelistProduct",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.DelistProduct",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDepositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDepositCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedDepositInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedDepositInsurance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositInsurance",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositInsurance",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedLinkSigner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedLinkSigner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LinkSigner",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.LinkSigner",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedLiquidateSubaccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedLiquidateSubaccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LiquidateSubaccount",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.LiquidateSubaccount",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedMintNlp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedMintNlp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintNlp"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.MintNlp"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedTransferQuote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedTransferQuote",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.TransferQuote",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedUpdateTierFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedUpdateTierFeeRates",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
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
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedWithdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedWithdrawCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawCollateral",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unsignedWithdrawInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unsignedWithdrawInsurance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawInsurance",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.WithdrawInsurance",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFeeTier"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFeeTier"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateFeeTier",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateFeeTier",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateNlpPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateNlpPool",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdateNlpPool",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdatePrice",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IEndpoint.UpdatePrice",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                        inputs: ::std::vec![],
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
    pub static ENDPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pa\x8B\xC6\x80b\0\0\"`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\x96W`\x005`\xE0\x1C\x80c\x8CX\xE1\n\x11a\x02\xE2W\x80c\xBA\x8D\x81\x81\x11a\x01\x91W\x80c\xE9\xABw\xE5\x11a\0\xEEW\x80c\xF8S\x1Ad\x11a\0\xA2W\x80c\xFB\xF4\x19\x84\x11a\0|W\x80c\xFB\xF4\x19\x84\x14a\x0E\x9EW\x80c\xFE\0\x84,\x14a\x10\x19W\x80c\xFEr\xD8\xB7\x14a\x10mW`\0\x80\xFD[\x80c\xF8S\x1Ad\x14a\t\xF2W\x80c\xF9h\xC7\xF4\x14a\x0F\xF2W\x80c\xFA\xB2\xC4i\x14a\x10\x12W`\0\x80\xFD[\x80c\xEFd\xED\x0E\x11a\0\xD3W\x80c\xEFd\xED\x0E\x14a\x0F\x96W\x80c\xF2\xFD\xE3\x8B\x14a\x0F\xBFW\x80c\xF8\x08\x9D\x9C\x14a\x0F\xD2W`\0\x80\xFD[\x80c\xE9\xABw\xE5\x14a\x0F!W\x80c\xEERU&\x14a\x0FtW`\0\x80\xFD[\x80c\xD9v\x86\x95\x11a\x01EW\x80c\xDBZP!\x11a\x01*W\x80c\xDBZP!\x14a\x0E\xCEW\x80c\xE6\x04\xED\x9E\x14a\x0E\xEEW\x80c\xE7\xC8\x07Q\x14a\x0F\x01W`\0\x80\xFD[\x80c\xD9v\x86\x95\x14a\x0B\xF7W\x80c\xDB:\xA8F\x14a\x0E\xACW`\0\x80\xFD[\x80c\xC4\xF9\xB2_\x11a\x01vW\x80c\xC4\xF9\xB2_\x14a\x0E\x86W\x80c\xC5\x105\x9F\x14a\x0E\x97W\x80c\xD4\xDE\x8D\x9D\x14a\x0E\x9EW`\0\x80\xFD[\x80c\xBA\x8D\x81\x81\x14a\x0E&W\x80c\xBC\x85\xCA\x86\x14a\x0EfW`\0\x80\xFD[\x80c\x96\xC4|o\x11a\x02?W\x80c\x9F\x9A5\xE1\x11a\x01\xF3W\x80c\xB2\xBBcg\x11a\x01\xD8W\x80c\xB2\xBBcg\x14a\r\xD3W\x80c\xB3\x14}\x17\x14a\r\xF3W\x80c\xB7\x0E\xB2c\x14a\x0E\x13W`\0\x80\xFD[\x80c\x9F\x9A5\xE1\x14a\x0B\x93W\x80c\xA0\x82\xC5\xAA\x14a\r\xB3W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x11a\x02$W\x80c\x98\xCD2\xFE\x14a\r3W\x80c\x9A\x08\xE55\x14a\rFW\x80c\x9E\x85\x14$\x14a\r\x93W`\0\x80\xFD[\x80c\x96\xC4|o\x14a\x0CBW\x80c\x98\xC5\xB5I\x14a\x0C\xE3W`\0\x80\xFD[\x80c\x8FO\x8E\xCC\x11a\x02\x96W\x80c\x91\xC1\xE3\xD7\x11a\x02{W\x80c\x91\xC1\xE3\xD7\x14a\x0B\xE4W\x80c\x94\xFA\xEF\xE5\x14a\x0B\xF7W\x80c\x954\xDD>\x14a\x0C\"W`\0\x80\xFD[\x80c\x8FO\x8E\xCC\x14a\x0B\xB3W\x80c\x91q\xD0\x8B\x14a\x0B\xC4W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x02\xC7W\x80c\x8D\xA5\xCB[\x14a\x0BoW\x80c\x8E]X\x8C\x14a\x0B\x80W\x80c\x8F988\x14a\x0B\x93W`\0\x80\xFD[\x80c\x8CX\xE1\n\x14a\x0B\x18W\x80c\x8D\n\xCC\x9B\x14a\x0B\\W`\0\x80\xFD[\x80c9e\x02\xB6\x11a\x04IW\x80c\\[4\xEF\x11a\x03\xA6W\x80co;\nr\x11a\x03ZW\x80c}\xB6\xA2[\x11a\x034W\x80c}\xB6\xA2[\x14a\n\xD2W\x80c\x85\xC8>\x9D\x14a\n\xE5W\x80c\x872C8\x14a\x0B\x05W`\0\x80\xFD[\x80co;\nr\x14a\nmW\x80cp\xF6\x82\x1C\x14a\n\x8DW\x80cqP\x18\xA6\x14a\n\xCAW`\0\x80\xFD[\x80ce\xDD\x13f\x11a\x03\x8BW\x80ce\xDD\x13f\x14a\n%W\x80ci\x03I\xCF\x14a\n-W\x80ci\xFD\xDC\xEC\x14a\nMW`\0\x80\xFD[\x80c\\[4\xEF\x14a\t\xF2W\x80c]O_\x97\x14a\n\x12W`\0\x80\xFD[\x80cM\x96\xA9\n\x11a\x03\xFDW\x80cU~\xD1\xBA\x11a\x03\xE2W\x80cU~\xD1\xBA\x14a\t\x9CW\x80cZ\0\x92;\x14a\t\xBCW\x80c[\xB4\xC1&\x14a\t\xDCW`\0\x80\xFD[\x80cM\x96\xA9\n\x14a\tNW\x80cO\xCF\xAEX\x14a\tsW`\0\x80\xFD[\x80c>\xDF,[\x11a\x04.W\x80c>\xDF,[\x14a\x08\xEEW\x80cA\xA0\x9B\xB6\x14a\t\x0EW\x80cB\xC7M\x1D\x14a\t.W`\0\x80\xFD[\x80c9e\x02\xB6\x14a\x08\x97W\x80c<\xECK\x93\x14a\x08\xAAW`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x04\xF7W\x80c-\x035\xAB\x11a\x04\xABW\x80c2\x16\xC0b\x11a\x04\x90W\x80c2\x16\xC0b\x14a\x07\xE5W\x80c3\x8A~V\x14a\x08FW\x80c6\x8EF\x86\x14a\x08qW`\0\x80\xFD[\x80c-\x035\xAB\x14a\x07\xA0W\x80c/\x9A'D\x14a\x07\xD2W`\0\x80\xFD[\x80c\"\0`F\x11a\x04\xDCW\x80c\"\0`F\x14a\x07\\W\x80c\"\x1F\t9\x14a\x07dW\x80c\"\xD4\xA8-\x14a\x07wW`\0\x80\xFD[\x80c\x1F\x18k'\x14a\x07AW\x80c!\x04u\x89\x14a\x07TW`\0\x80\xFD[\x80c\x14sWU\x11a\x05NW\x80c\x1C\x88m\x0B\x11a\x053W\x80c\x1C\x88m\x0B\x14a\x06\xF5W\x80c\x1D\x97\xD2/\x14a\x06UW\x80c\x1D\x9E\xED\xA5\x14a\x07\nW`\0\x80\xFD[\x80c\x14sWU\x14a\x06\x8AW\x80c\x18\xED\x16\xEB\x14a\x06\xCAW`\0\x80\xFD[\x80c\rU\xE2k\x11a\x05\x7FW\x80c\rU\xE2k\x14a\x065W\x80c\x0E\xDA\xAC\xCE\x14a\x06UW\x80c\x14YEz\x14a\x06uW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\x9BW\x80c\x0B\xB9\xC3\xA2\x14a\x05\xC4W[`\0\x80\xFD[a\x05\xAEa\x05\xA96`\x04a\\\x1EV[a\x10\xC8V[`@Qa\x05\xBB\x91\x90a\\:V[`@Q\x80\x91\x03\x90\xF3[a\x05\xD7a\x05\xD26`\x04a\\vV[a\x10\xFAV[`@Qa\x05\xBB\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R` \x84\x01Q\x15\x15` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[a\x06Ha\x06C6`\x04a\\\xA4V[a\x11;V[`@Qa\x05\xBB\x91\x90a]8V[a\x06ha\x06c6`\x04a]\xAFV[a\x11LV[`@Qa\x05\xBB\x91\x90a]\xCBV[a\x06\x88a\x06\x836`\x04a^\x1DV[a\x11\x7FV[\0[a\x06\x9Da\x06\x986`\x04a^\xA0V[a\x16$V[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[`\xA3Ta\x06\xDD\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\x06\xFDa\x16GV[`@Qa\x05\xBB\x91\x90a^\xBCV[a\x07\x1Da\x07\x186`\x04a^\xA0V[a\x16\xE3V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x0F\x0B\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x06\x88a\x07O6`\x04a_\x86V[a\x17\x06V[a\x06\x88a\x17\xA2V[a\x06\xDDa\x18RV[a\x06\x88a\x07r6`\x04aa\x9EV[a\x18\x97V[a\x06\xDDa\x07\x856`\x04ab\x07V[`\0\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x06\xDDa\x07\xAE6`\x04ab V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA2` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x06\x88a\x07\xE06`\x04abTV[a\x1C\x1AV[a\x06\x88a\x07\xF36`\x04ab\xADV[\x80Q`\xA4\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\x08Ya\x08T6`\x04ac\rV[a\x1DrV[`@Q\x90Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01a\x05\xBBV[a\x08\x84a\x08\x7F6`\x04ac)V[a\x1D\x90V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x05\xBBV[a\x06\x88a\x08\xA56`\x04acUV[a\x1E$V[a\x08\xBDa\x08\xB86`\x04a\\\x1EV[a\x1F\nV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\t\x01a\x08\xFC6`\x04ac\x8EV[a\x1F6V[`@Qa\x05\xBB\x91\x90ad3V[a\t!a\t\x1C6`\x04ad\xCCV[a\x1FGV[`@Qa\x05\xBB\x91\x90aeyV[a\tAa\t<6`\x04ac\x8EV[a\x1FXV[`@Qa\x05\xBB\x91\x90ae\xBFV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\x08\x84a\t\x816`\x04ac)V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xA7` R`@\x90 T`\x0F\x0B\x90V[a\t\xA4a\x1FuV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\t\xCFa\t\xCA6`\x04ac\x8EV[a \x03V[`@Qa\x05\xBB\x91\x90ae\xF8V[a\t\xE4a !V[`@Q\x90\x81R` \x01a\x05\xBBV[a\n\x05a\n\x006`\x04af&V[a\"\x14V[`@Qa\x05\xBB\x91\x90af\xCCV[`\x99Ta\t[\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x06\x88a\"TV[a\n@a\n;6`\x04af\xDFV[a\"\x97V[`@Qa\x05\xBB\x91\x90ag\xA4V[a\n`a\n[6`\x04ag\xB7V[a\"\xEAV[`@Qa\x05\xBB\x91\x90ah)V[a\n\x80a\n{6`\x04a\\\xA4V[a\"\xFBV[`@Qa\x05\xBB\x91\x90ah<V[a\n\xA0a\n\x9B6`\x04a^\xA0V[a#\x0CV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x06\x88a#/V[a\x06\x88a\n\xE06`\x04ah\x81V[a#CV[a\n\xF8a\n\xF36`\x04ai\0V[a%\x15V[`@Qa\x05\xBB\x91\x90ai4V[a\x06\x88a\x0B\x136`\x04ai\xBEV[a%&V[a\x06\x88a\x0B&6`\x04acUV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x06\x88a\x0Bj6`\x04aj\x12V[a-\x0FV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\t[V[a\x06\x88a\x0B\x8E6`\x04ajGV[a.IV[a\x0B\xA6a\x0B\xA16`\x04a\\\x1EV[a.\xB9V[`@Qa\x05\xBB\x91\x90aj\x9BV[`\xAET`\x01`\x01`\xA0\x1B\x03\x16a\t[V[a\x0B\xD7a\x0B\xD26`\x04ad\xCCV[a.\xE5V[`@Qa\x05\xBB\x91\x90aj\xCEV[a\t[a\x0B\xF26`\x04ab\x07V[a.\xF6V[a\x0C\na\x0C\x056`\x04ac\rV[a/\xB7V[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x05\xBBV[a\x0C5a\x0C06`\x04ab V[a/\xD5V[`@Qa\x05\xBB\x91\x90akDV[a\x0CUa\x0CP6`\x04akWV[a0oV[`@Qa\x05\xBB\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x0C\xF6a\x0C\xF16`\x04a\\\x1EV[a0\xB7V[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\x06\x88a\rA6`\x04aksV[a0\xE3V[a\rYa\rT6`\x04a\\\x1EV[a2KV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\r\xA6a\r\xA16`\x04a\\vV[a2wV[`@Qa\x05\xBB\x91\x90al\x16V[a\r\xC6a\r\xC16`\x04ac\x8EV[a2\xB8V[`@Qa\x05\xBB\x91\x90almV[a\r\xE6a\r\xE16`\x04ac\x8EV[a2\xC9V[`@Qa\x05\xBB\x91\x90al\xCEV[a\x0E\x06a\x0E\x016`\x04ag\xB7V[a2\xE6V[`@Qa\x05\xBB\x91\x90am6V[a\t\xE4a\x0E!6`\x04ai\xBEV[a3\x0BV[a\x0E9a\x0E46`\x04a^\xA0V[a3\xA8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x0Eya\x0Et6`\x04a]\xAFV[a3\xCBV[`@Qa\x05\xBB\x91\x90amnV[`\xA1T`\x01`\x01`@\x1B\x03\x16a\x06\xDDV[`\0a\x08\x84V[g\r\xE0\xB6\xB3\xA7d\0\0a\x08\x84V[a\x0E\xBFa\x0E\xBA6`\x04ac\rV[a3\xFEV[`@Q\x90Q\x81R` \x01a\x05\xBBV[a\x0E\xE1a\x0E\xDC6`\x04ad\xCCV[a4\x1CV[`@Qa\x05\xBB\x91\x90am\xB1V[a\x06\x88a\x0E\xFC6`\x04aj\x12V[a4-V[a\x0F\x14a\x0F\x0F6`\x04ag\xB7V[a7\xA3V[`@Qa\x05\xBB\x91\x90an\x96V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x05\xBBV[a\x0F\x87a\x0F\x826`\x04an\xDCV[a7\xD3V[`@Qa\x05\xBB\x93\x92\x91\x90an\xF7V[a\t\xE4a\x0F\xA46`\x04an\xDCV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA0` R`@\x90 T\x90V[a\x06\x88a\x0F\xCD6`\x04ab V[a8\xEEV[a\x0F\xE5a\x0F\xE06`\x04ac\x8EV[a9{V[`@Qa\x05\xBB\x91\x90aoSV[a\x10\x05a\x10\x006`\x04ag\xB7V[a9\x8CV[`@Qa\x05\xBB\x91\x90ao\x88V[`\xA3a\t\xE4V[a\x10,a\x10'6`\x04ab\x07V[a9\xB9V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x95\x16\x85R` \x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16``\x82\x01R`\x80\x01a\x05\xBBV[a\x10\x80a\x10{6`\x04a]\xAFV[a:\x0FV[`@Qa\x05\xBB\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ap_V[\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ap\x8BV[a\x11CaYgV[a\x10\xF4\x82aq\xE0V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\"V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\x9FWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xB9WP0;\x15\x80\x15a\x11\xB9WP`\0T`\xFF\x16`\x01\x14[a\x120W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12SW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12[a:BV[a\x12\xB6`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cNado`\xE0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa:\xB5V[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x99\x80T\x86\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xAE\x80T\x88\x85\x16\x90\x84\x16\x17\x90U`\xAF\x80T\x86\x85\x16\x90\x84\x16\x17\x90U`\x9C\x80T\x93\x8A\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x131\x90`\0\x90`\x04\x01arTV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13r\x91\x90ar|V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x99T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x13\xB6\x90`\x01\x90`\x04\x01arTV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF7\x91\x90ar|V[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x91\x90\x92\x01\x82\x90R`\xA4\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x81\x80R`\xAD\x90R\x7FCI\xCF\xFE\x87\x97\n\x06Q\x90o\xE7\xEC\x1B\xC0/;4\xDF\x90\xDF\x07u\xD7V\x83\xDC\xDB\xF5l%\x85\x80T`\x01`\x01`\x80\x1B\x03\x19\x16g\r\xE0\xB6\xB3\xA7d\0\0\x17\x90U`\xAAT\x90\x03a\x15\xD6W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x02` \x83\x01\x90\x81R\x92\x82\x01\x81\x81Rg\r\xE0\xB6\xB3\xA7d\0\0``\x84\x01\x90\x81R`\xAA\x80T`\x01\x81\x01\x82U\x93R\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1A`\x04\x90\x93\x02\x92\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1B\x82\x01U\x91Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1C\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1D\x90\x91\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x80\x15a\x16\x1CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\x99V[```\xAA\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x16\xDAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`@\x1B\x03\x16\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x01\x92\x90\x92R`\x03\x01T`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R\x90\x83R\x90\x92\x01\x91\x01a\x16kV[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\xD8V[`\0[\x81\x81\x10\x15a\x17[W6`\0\x84\x84\x84\x81\x81\x10a\x17&Wa\x17&as\rV[\x90P` \x02\x81\x01\x90a\x178\x91\x90as#V[\x91P\x91Pa\x17F\x82\x82a;.V[PP\x80\x80a\x17S\x90as\x7FV[\x91PPa\x17\tV[P`\xA3\x80T\x82\x91\x90`\0\x90a\x17z\x90\x84\x90`\x01`\x01`@\x1B\x03\x16as\x98V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x07\xF3W`\xA5`\0\x82` \x01\x80Qa\x18\x10\x90as\xC3V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x18K`\x01\x83\x01\x82aY\xA0V[PPa\x17\xDBV[`\xA3\x80T`\0\x91\x90\x82\x90a\x18n\x90`\x01`\x01`@\x1B\x03\x16as\xE6V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA3T\x16\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x18\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P``\x84\x90\x1C`\0a\x18\xF2`\ng\r\xE0\xB6\xB3\xA7d\0\0at\"V[`\x0F\x0B\x90P`\x01\x86\x14\x80\x15\x90a\x19\x1DWP`\0\x86\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x19;Wa\x195g\r\xE0\xB6\xB3\xA7d\0\0`\x05atiV[`\x0F\x0B\x90P[`\x99T`@Qcd\x02\xC7\xF9`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x86\x16`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cd\x02\xC7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xB0W=`\0\x80>=`\0\xFD[PP`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01Ra\x1A:\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A*\x91\x90ar|V[3\x86`\x01`\x01`\x80\x1B\x03\x16aI\xBFV[`@\x80Q``\x80\x82\x01\x83R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x1A\x87b\x03\xF4\x80Bas\x98V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8Ac\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x1B\x0B\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1B)\x92\x91` \x01au\x10V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA5`\0\x83` \x01\x80Q\x80\x91\x90a\x1BQ\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x1B\xBD\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPPV[`\0Z\x90Pa\x1C(\x85aJ\x16V[`\0[\x83\x81\x10\x15a\x1C\xF9W6`\0\x86\x86\x84\x81\x81\x10a\x1CHWa\x1CHas\rV[\x90P` \x02\x81\x01\x90a\x1CZ\x91\x90as#V[\x91P\x91Pa\x1Ch\x82\x82a;.V[`\0Za\x1Cu\x90\x86au?V[\x90P\x85\x81\x11\x15a\x1C\xE3W`\xAFT`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xCAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1C\xDEW=`\0\x80>=`\0\xFD[PPPP[PPP\x80\x80a\x1C\xF1\x90as\x7FV[\x91PPa\x1C+V[P`\xAFT`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x1D\x17\x90\x85au?V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1DSW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1DgW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83auVV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1D\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`@Qc\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x9AT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x1EGWP`\x9BT`\x01`\x01`\xA0\x1B\x03\x163\x14[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x1E\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[Pc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`U`\xF8\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x15a\x1E\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83au\x82V[a\x1F>aZ^V[a\x10\xF4\x82av\xA6V[a\x1FOaZ\x8DV[a\x10\xF4\x82ax4V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x10\xF4\x82ax\xA4V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F\xB2W\x81Qa\x1F\xB8V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1F\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x10\xF4\x82ayPV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12'V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA5` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a!\x14\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!@\x90ay\\V[\x80\x15a!\x8DW\x80`\x1F\x10a!bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x8DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\xC7\x92\x90\x91`\x04\x01ay\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\"\0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra!\xFD\x91\x81\x01\x90ay\xB2V[`\x01[a\"\rWP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q`\xE0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x82\x90R\x82R``` \x83\x01\x81\x90R\x92\x82\x01R\x81\x81\x01\x91\x90\x91Ra\x10\xF4\x82az\x9EV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x07\xF3\x81`\0aJ^V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x10\xF4\x82a{\x17V[a\"\xF2aZ\xCCV[a\x10\xF4\x82a{\x98V[a#\x03aZ\xFEV[a\x10\xF4\x82a{\xCAV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83a{\xD6V[a#7aL\xFDV[a#A`\0aMWV[V[a#L\x86aJ\x16V[`\x9DT`\x01`\x01`\xA0\x1B\x03\x163\x14a#cW`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a#\xFDW\x81\x87\x87\x83\x81\x81\x10a#\xB0Wa#\xB0as\rV[\x90P` \x02\x81\x01\x90a#\xC2\x91\x90as#V[`@Q` \x01a#\xD4\x93\x92\x91\x90a|\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a#\xF6\x90as\x7FV[\x90Pa#\x95V[P`\xAFT`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$[W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$oW=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a%\x0BW6`\0\x88\x88\x84\x81\x81\x10a$\x93Wa$\x93as\rV[\x90P` \x02\x81\x01\x90a$\xA5\x91\x90as#V[\x91P\x91Pa$\xB3\x82\x82a;.V[`\xA3\x80T`\x01\x91\x90`\0\x90a$\xD2\x90\x84\x90`\x01`\x01`@\x1B\x03\x16as\x98V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a%\x03\x90as\x7FV[\x91PPa$vV[PPPPPPPPV[a%\x1Da[.V[a\x10\xF4\x82a|fV[30\x14a%2W`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a%GWa%Gas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a%bWa%bar>V[\x90P`\x01\x81`\x1D\x81\x11\x15a%xWa%xar>V[\x03a&<W`\0a%\x8C\x83`\x01\x81\x87a|rV[\x81\x01\x90a%\x99\x91\x90au\x82V[\x90Pa%\xA9\x81`\0\x01Q\x86aM\xA9V[\x80Qa%\xB4\x90aN\x07V[`\x99T`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&2W=`\0\x80>=`\0\xFD[PPPPPa-\tV[`\x02\x81`\x1D\x81\x11\x15a&PWa&Par>V[\x03a&\xF4W`\0a&d\x83`\x01\x81\x87a|rV[\x81\x01\x90a&q\x91\x90a|\x9CV[\x90Pa&\x81\x81`\0\x01Q\x86aM\xA9V[`\x99T\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA3T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a&\x04V[`\x07\x81`\x1D\x81\x11\x15a'\x08Wa'\x08ar>V[\x03a'vW`\x99T`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a'?\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'YW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'mW=`\0\x80>=`\0\xFD[PPPPa-\tV[`\r\x81`\x1D\x81\x11\x15a'\x8AWa'\x8Aar>V[\x03a'\xF9W`\0a'\x9E\x83`\x01\x81\x87a|rV[\x81\x01\x90a'\xAB\x91\x90ap_V[\x90Pa'\xBB\x81`\0\x01Q\x86aM\xA9V[\x80Qa'\xC6\x90aN\x96V[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xA8\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua-\tV[`\x12\x81`\x1D\x81\x11\x15a(\rWa(\rar>V[\x03a(SW`\x99T`\xA3T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a'?\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a|\xF5V[`\x14\x81`\x1D\x81\x11\x15a(gWa(gar>V[\x03a(\x9EW`\x99T`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a'?\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\t\x81`\x1D\x81\x11\x15a(\xB2Wa(\xB2ar>V[\x03a+9W`\xAE`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\x07W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\x1BW=`\0\x80>=`\0\xFD[PPPP`\0`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)\x9C\x91\x90\x81\x01\x90a}\"V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xB9Wa)\xB9a_\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a*\xC5W`\xA7`\0\x84\x83\x81Q\x81\x10a*\x07Wa*\x07as\rV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a*IWa*Ias\rV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xA7`\0\x85\x84\x81Q\x81\x10a*uWa*uas\rV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a*\xBD\x81as\x7FV[\x91PPa)\xE8V[Pa*\xD0`\x01aN\x96V[`\x99T`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a+\0\x90\x84\x90`\x04\x01a}\xBBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+.W=`\0\x80>=`\0\xFD[PPPPPPa-\tV[`\x10\x81`\x1D\x81\x11\x15a+MWa+Mar>V[\x03a+\x93W`\x99T`\xA3T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a'?\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a|\xF5V[`\x18\x81`\x1D\x81\x11\x15a+\xA7Wa+\xA7ar>V[\x03a,(W`\0a+\xBB\x83`\x01\x81\x87a|rV[\x81\x01\x90a+\xC8\x91\x90a}\xCEV[`\xAET`@\x80Qc\x068\xF6\xF3`\xE5\x1B\x81R\x83Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04\x83\x01R` \x85\x01Q\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC7\x1E\xDE`\x90`\x84\x01a&\x04V[`\x19\x81`\x1D\x81\x11\x15a,<Wa,<ar>V[\x03a,wW`\0a,P\x83`\x01\x81\x87a|rV[\x81\x01\x90a,]\x91\x90a~RV[\x90Pa,q\x81`\0\x01Q\x82` \x01QaN\xFFV[Pa-\tV[`\x1A\x81`\x1D\x81\x11\x15a,\x8BWa,\x8Bar>V[\x03a,\xC5W`\0a,\x9F\x83`\x01\x81\x87a|rV[\x81\x01\x90a,\xAC\x91\x90a~\x85V[\x90Pa,q\x81`\0\x01Q\x82` \x01Q\x83`@\x01QaP\xA3V[`\x1B\x81`\x1D\x81\x11\x15a,\xD9Wa,\xD9ar>V[\x03a\x05\x96W`\0a,\xED\x83`\x01\x81\x87a|rV[\x81\x01\x90a,\xFA\x91\x90auVV[\x90Pa,q\x81`\0\x01QaR\xDFV[PPPPV[`\0\x82\x82`\0\x81\x81\x10a-$Wa-$as\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a-?Wa-?ar>V[\x90P`\0\x81`\x1D\x81\x11\x15a-UWa-Uar>V[\x03a-\xB2W`\0a-i\x83`\x01\x81\x87a|rV[\x81\x01\x90a-v\x91\x90a\x7FKV[\x80QQ\x90\x91P`\x02\x14a-\xACW\x80Q\x80Q`\xA0\x90\x91\x01Qa-\x97\x91\x90aS\xDEV[\x80QQa-\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[Pa.\x06V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x12'V[`\xA3\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a.\x1F\x83as\xE6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra.\xB4\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra.\x92\x90a\x7F\x7FV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x18\x97V[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x7F\xA3V[a.\xEDaZ\x8DV[a\x10\xF4\x82a\x7F\xBFV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a/$W`\0\x82\x81R`\xA8` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x10\xF4V[`\xAET`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xA8\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x97\x91\x90ay\xB2V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80\x16V[`\xAC` R`\0\x90\x81R`@\x90 \x80Ta/\xEE\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\x1A\x90ay\\V[\x80\x15a0gW\x80`\x1F\x10a0<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80ZV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a~\x85V[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a1oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12'V[\x81`\xA5`\0\x83` \x01\x80Q\x80\x91\x90a1\x86\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a1\xF2\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80\xF1V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x817V[a2\xC0a[WV[a\x10\xF4\x82a\x81SV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x10\xF4\x82a\x81\xF0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x10\xF4\x82a\x82\xB3V[`\0\x80\x83\x83`\0\x81\x81\x10a3!Wa3!as\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a3<Wa3<ar>V[\x90P`\r\x81`\x1D\x81\x11\x15a3RWa3Rar>V[\x03a3\x9DW`\0a3f\x84`\x01\x81\x88a|rV[\x81\x01\x90a3s\x91\x90ap_V[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a3\x91W`\0a3\x94V[\x80Q[\x92PPPa\"\rV[P`\0\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83a~RV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a|\x9CV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x83\x12V[a4$a[\x7FV[a\x10\xF4\x82a\x83\x86V[`\0\x82\x82`\0\x81\x81\x10a4BWa4Bas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a4]Wa4]ar>V[\x90P3`\x01\x82`\x1D\x81\x11\x15a4tWa4tar>V[\x03a4~W`\0\x80\xFD[`\x07\x82`\x1D\x81\x11\x15a4\x92Wa4\x92ar>V[\x03a4\xD8W`\0a4\xA6\x84`\x01\x81\x88a|rV[\x81\x01\x90a4\xB3\x91\x90a\x80\x16V[\x90Pa4\xD2a4\xC0aT\x9AV[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aI\xBFV[Pa6&V[`\x12\x82`\x1D\x81\x11\x15a4\xECWa4\xECar>V[\x14\x80a5\tWP`\x14\x82`\x1D\x81\x11\x15a5\x07Wa5\x07ar>V[\x14[\x80a5%WP`\t\x82`\x1D\x81\x11\x15a5#Wa5#ar>V[\x14[\x80a5AWP`\x10\x82`\x1D\x81\x11\x15a5?Wa5?ar>V[\x14[\x80a5]WP`\x18\x82`\x1D\x81\x11\x15a5[Wa5[ar>V[\x14[\x80a5yWP`\x19\x82`\x1D\x81\x11\x15a5wWa5war>V[\x14[\x80a5\x95WP`\x1A\x82`\x1D\x81\x11\x15a5\x93Wa5\x93ar>V[\x14[\x80a5\xB1WP`\x1B\x82`\x1D\x81\x11\x15a5\xAFWa5\xAFar>V[\x14[\x15a5\xD5W`gT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a5\xD0W`\0\x80\xFD[a6&V[a5\xE3a5\xE0aT\x9AV[PV[`\xAB\x80Tb\x0FB@\x91\x90`\0\x90a5\xFE\x90\x84\x90`\x0F\x0Ba\x843V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a6sb\x03\xF4\x80Bas\x98V[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA5\x93P\x90a6\xDC\x82as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a7H\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[a7\xCA`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x10\xF4\x82a\x85\x13V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA5\x82R\x85\x83 `\xA4T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a8]\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta8\x89\x90ay\\V[\x80\x15a8\xD6W\x80`\x1F\x10a8\xABWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a8\xD6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a8\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a8\xF6aL\xFDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a9rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12'V[a5\xE0\x81aMWV[a9\x83a[\xB3V[a\x10\xF4\x82a\x85\xE0V[a9\xB0`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01`\0\x81RP\x90V[a\x10\xF4\x82a\x85\xECV[`\xAA\x81\x81T\x81\x10a9\xC9W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`@\x1B\x03\x90\x92\x16\x93P\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`\x80\x1B\x03\x16\x84V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a}\xCEV[`\0Ta\x01\0\x90\x04`\xFF\x16a:\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a#AaU\rV[`\0Ta\x01\0\x90\x04`\xFF\x16a; W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a;*\x82\x82aU\x81V[PPV[`\0\x82\x82`\0\x81\x81\x10a;CWa;Cas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a;^Wa;^ar>V[\x90P`\0\x81`\x1D\x81\x11\x15a;tWa;tar>V[\x03a<^W`\0a;\x88\x83`\x01\x81\x87a|rV[\x81\x01\x90a;\x95\x91\x90a\x7FKV[\x80QQ\x90\x91P`\x02\x14a<\0W\x80Q\x80Q`\xA0\x90\x91\x01Qa;\xB6\x91\x90aS\xDEV[\x80QQa;\xDF\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[`\0\x93\x92PPPV[aV\x06V[\x80QQa;\xEB\x90aN\x96V[\x80QQa<\0\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`\x99T\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a<0\x91`\x04\x01al\x16V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<JW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\x0BW=`\0\x80>=`\0\xFD[`\x02\x81`\x1D\x81\x11\x15a<rWa<rar>V[\x03a=\xC7W`\0a<\x86\x83`\x01\x81\x87a|rV[\x81\x01\x90a<\x93\x91\x90a\x86\xD2V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92Pa<\xA9\x91aS\xDEV[\x80QQa<\xC4\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ra=R\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=C\x91\x90a\x87\x06V[`\xA0\x01Q\x83Q` \x01QaVTV[`\x99T\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA3T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a<0V[`\x03\x81`\x1D\x81\x11\x15a=\xDBWa=\xDBar>V[\x03a>\xDFW`\0a=\xEF\x83`\x01\x81\x87a|rV[\x81\x01\x90a=\xFC\x91\x90a\x80\x16V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15a>HW` \x82\x01Q\x83Qa>C\x91\x90a\x87\x9AV[a>KV[`\0[`\x9AT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\x9AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\xAEW=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA6UPa.\xB4\x91PPV[`\n\x81`\x1D\x81\x11\x15a>\xF3Wa>\xF3ar>V[\x03a?\xF4W`\0a?\x07\x83`\x01\x81\x87a|rV[\x81\x01\x90a?\x14\x91\x90a\x87\xC2V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15a?\\W\x81Q\x83Qa?W\x91\x90a\x87\x9AV[a?_V[`\0[`\x9BT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a?\x98\x91\x85\x91\x90`\x04\x01a\x87\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\xC6W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA6UPa.\xB4\x91PPV[`\x04\x81`\x1D\x81\x11\x15a@\x08Wa@\x08ar>V[\x03a@\xCEW`\x99T`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90a@D\x90\x88\x90\x88\x90`\x04\x01a|\xE1V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a@bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x86\x91\x90a\x88\x18V[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a@\xC7Wc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90U[PPPPPV[`\x05\x81`\x1D\x81\x11\x15a@\xE2Wa@\xE2ar>V[\x03aAPW`\x99T`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15aAGW=`\0\x80>=`\0\xFD[PPPPPPPV[`\x06\x81`\x1D\x81\x11\x15aAdWaAdar>V[\x03aBbW`\0aAx\x83`\x01\x81\x87a|rV[\x81\x01\x90aA\x85\x91\x90a\x88GV[` \x81\x01QQQ\x90\x91PaA\x98\x90aN\x96V[`@\x81\x01QQQaA\xA8\x90aN\x96V[`\0`@Q\x80`\x80\x01`@R\x80\x83\x81R` \x01aA\xD0\x84` \x01Q`\0\x01Q`\0\x01QaW+V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aA\xF3\x84`@\x01Q`\0\x01Q`\0\x01QaW+V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\0` \x90\x92\x01\x91\x90\x91R`\xAET`@Qc\x03V\x14\xB1`\xE3\x1B\x81R\x92\x93P\x16\x90c\x1A\xB0\xA5\x88\x90aB4\x90\x84\x90`\x04\x01a\x88{V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aBNW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1DgW=`\0\x80>=`\0\xFD[`\x17\x81`\x1D\x81\x11\x15aBvWaBvar>V[\x03aCLW`\0aB\x8A\x83`\x01\x81\x87a|rV[\x81\x01\x90aB\x97\x91\x90a\x88\xD3V[\x80Q` \x01QQQ\x90\x91PaB\xAB\x90aN\x96V[\x80Q`@\x01QQQaB\xBC\x90aN\x96V[`@\x80Q`\x80\x81\x01\x90\x91R\x81Q\x81R\x81Q` \x90\x81\x01QQQ`\0\x92\x91\x82\x01\x90aB\xE5\x90a.\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R\x83Q`@\x01QQQ` \x90\x91\x01\x90aC\x07\x90a.\xF6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x85\x81\x01Q`\x0F\x0B\x92\x01\x91\x90\x91R`\xAET`@Qc\x03V\x14\xB1`\xE3\x1B\x81R\x92\x93P\x16\x90c\x1A\xB0\xA5\x88\x90aB4\x90\x84\x90`\x04\x01a\x88{V[`\x08\x81`\x1D\x81\x11\x15aC`WaC`ar>V[\x03aC\xFEW`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaC\xA8\x81`\x01aJ^V[\x80Q`\xA4\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\x15\x81`\x1D\x81\x11\x15aD\x12WaD\x12ar>V[\x03aE\x04W`\0aD&\x83`\x01\x81\x87a|rV[\x81\x01\x90aD3\x91\x90a\x89\x07V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaDI\x91aS\xDEV[\x80QQaDd\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80QQaDy\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`@\x81\x81\x01\x80Q`\x0B`\0R`\xAD` R\x7F\x8B\xFB\x1Cv\x07\xB6=uf)\xA9x\x9B\xE2\xB0N\x82<K\xA5\xF0C\xE2Es[\xCA\xB4@=\xEB:\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x99T\x83Q\x91Q``\x85\x01Q\x93Qc!vjI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93cB\xEC\xD4\x92\x93a<0\x93\x90\x92\x91`\xAA\x91`\x04\x01a\x89\xABV[`\x16\x81`\x1D\x81\x11\x15aE\x18WaE\x18ar>V[\x03aF\nW`\0aE,\x83`\x01\x81\x87a|rV[\x81\x01\x90aE9\x91\x90a\x89\x07V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaEO\x91aS\xDEV[\x80QQaEj\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80QQaE\x7F\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`@\x81\x81\x01\x80Q`\x0B`\0R`\xAD` R\x7F\x8B\xFB\x1Cv\x07\xB6=uf)\xA9x\x9B\xE2\xB0N\x82<K\xA5\xF0C\xE2Es[\xCA\xB4@=\xEB:\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x99T\x83Q\x91Q``\x85\x01Q\x93Qc\xB5\xE2-\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93c\xB5\xE2-\xBB\x93a<0\x93\x90\x92\x91`\xAA\x91`\x04\x01a\x89\xABV[`\x0B\x81`\x1D\x81\x11\x15aF\x1EWaF\x1Ear>V[\x03aFUW`\x99T`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\r\x81`\x1D\x81\x11\x15aFiWaFiar>V[\x03aF\xEFW`\0aF}\x83`\x01\x81\x87a|rV[\x81\x01\x90aF\x8A\x91\x90a\x8A\x06V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaF\xA0\x91aS\xDEV[\x80QQaF\xBB\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xA8\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x0E\x81`\x1D\x81\x11\x15aG\x03WaG\x03ar>V[\x03aG:W`\x99T`@Qc\x8F\x17\xD0A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8F\x17\xD0A\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x0F\x81`\x1D\x81\x11\x15aGNWaGNar>V[\x03aG\xF6W`\0aGb\x83`\x01\x81\x87a|rV[\x81\x01\x90aGo\x91\x90a\x8A:V[\x90PaG\x82\x81`\0\x01Q` \x01QaN\x07V[\x80QQaG\x9D\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80Q\x80Q``\x90\x91\x01QaG\xB1\x91\x90aS\xDEV[\x80QQaG\xC6\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`\x99T\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a<0\x91`\x04\x01a]\xCBV[`\x11\x81`\x1D\x81\x11\x15aH\nWaH\nar>V[\x03aHAW`\x99T`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x1C\x81`\x1D\x81\x11\x15aHUWaHUar>V[\x03aH\x8CW`\xAET`@Qc\x84R\x80\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x84R\x80\x93\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x13\x81`\x1D\x81\x11\x15aH\xA0WaH\xA0ar>V[\x03aIUW`\0aH\xB4\x83`\x01\x81\x87a|rV[\x81\x01\x90aH\xC1\x91\x90a\x8AnV[`\xAET\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c4\xF9\xA4\xA4\x90\x84\x90aH\xEA\x90a.\xF6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x07\x92\x91\x90a\x8A\xA2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aI&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aIJ\x91\x90ay\xB2V[\x90Pa@\xC7\x81aN\x07V[`\x1D\x81`\x1D\x81\x11\x15aIiWaIiar>V[\x03a\x05\x96W`\0aI}\x83`\x01\x81\x87a|rV[\x81\x01\x90aI\x8A\x91\x90a\x83\x12V[`\xAET\x81Q`@Qc\xF6\xEE{K`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF6\xEE{K\x90`$\x01a<0V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16aI\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`\x99Ta.\xB4\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA3T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14a;*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x8Bn`#\x919\x90aJ\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA5` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aK\x11\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaK=\x90ay\\V[\x80\x15aK\x8AW\x80`\x1F\x10aK_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aK\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aKmW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA5`\0\x84`@\x01\x80Q\x80\x91\x90aK\xAB\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aK\xE8`\x01\x83\x01\x82aY\xA0V[PP\x81\x80aL\x03WPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aL>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[PFazi\x03aLsW` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aA\x19\x92`\x04\x01ay\x90V[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aL\xA6\x92\x90\x91`\x04\x01ay\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aL\xC0W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aL\xD1WP`\x01[a-\tWb\x03\xD0\x90Z\x11\x15\x80aL\xF1WPaL\xED`\x02\x82a\x8A\xCDV[Z\x11\x15[\x15aL\xF8W\xFE[a-\tV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a#AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12'V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aM\xCCWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a.\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[`\0\x81\x81R`\x9F` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a5\xE0W`\xA1\x80T`\0\x90aN>\x90`\x01`\x01`@\x1B\x03\x16as\xE6V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\x9F` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA1T\x90\x93\x16\x81R`\xA0\x90\x92R\x90 UV[`\x01\x81\x14\x80aN\xA5WP`\x02\x81\x14[\x80aN\xC6WP`\0\x81\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90a;*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[`\xAATd\x01\0\0\0\0`\x01\x82\x11\x15aOUW`\xAA\x80TaO!\x90`\x01\x90au?V[\x81T\x81\x10aO1WaO1as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T`\0\x1C`\x01aOR\x91\x90a\x8A\xE1V[\x90P[aO^\x81aN\x07V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R` \x80\x82\x01\x84\x81R`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x83\x85\x01\x81\x81R`\x01`\x01`\x80\x1B\x03\x98\x89\x16``\x86\x01\x90\x81R`\xAA\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x96Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1A`\x04\x90\x98\x02\x97\x88\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x9B\x16\x17\x90\x99U\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1B\x86\x01UQ\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1C\x85\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x90\x9B\x16\x91\x90\x91\x17\x90U\x90Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1D\x90\x93\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x93\x90\x97\x16\x92\x90\x92\x17\x90\x95U\x91\x83R`\xA9\x90\x93R\x91\x90 \x80T\x90\x92\x16\x17\x90UV[`\xAAT`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb\x04\x94\xE5`\xEC\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x85\x16\x10aP\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P\x82`\x01`\x01`@\x1B\x03\x16`\0\x03aQ\xD1W`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aQRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fcannot set owner for pool 0.\0\0\0\0`D\x82\x01R`d\x01a\x12'V[`\0\x81`\x01`\x01`\x80\x1B\x03\x16\x11aQ\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7Fcannot set 0 balance weight for `D\x82\x01R\x7Fpool 0.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12'V[\x81`\xAA\x84`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aQ\xEEWaQ\xEEas\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\xAA\x84`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aR@WaR@as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x03\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x81`\xA9`\0`\xAA\x86`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aR\x96WaR\x96as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x80\x15aS\x02WP`\xAAT`\x01`\x01`@\x1B\x03\x82\x16\x10[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x94\xE5`\xEC\x1B\x81RP\x90aS<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`\x99T`\xAA\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c}\x18'}\x91\x90`\x01`\x01`@\x1B\x03\x85\x16\x90\x81\x10aSpWaSpas\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xA0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aS\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15aS\xCEW=`\0\x80>=`\0\xFD[PPPPa5\xE0\x81`\0\x80aP\xA3V[``\x82\x90\x1C`\0\x90\x81R`\xA2` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aT\x07\x83as\xE6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a.\xB4WaTX\x81`\x01`\x01`@\x1B\x03\x16aWkV[`@Q` \x01aTh\x91\x90a\x8A\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x12'\x91`\x04\x01akDV[a;*\x82\x82`\0aVTV[`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x08\x91\x90ar|V[\x90P\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16aUxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a#A3aMWV[`\0Ta\x01\0\x90\x04`\xFF\x16aU\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[`\0a\x10\xF4aV\x13aX\nV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9AT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aVp\x86a\x8B>V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aV\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15aV\xD3W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xA7` R`@\x81 \x80T\x84\x92\x90aV\xFF\x90\x84\x90`\x0F\x0Ba\x843V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x80aW7\x83a.\xF6V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aWNW\x92\x91PPV[PP`\0\x90\x81R`\xA9` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[```\0aWx\x83aX\x85V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x97WaW\x97a_\xF5V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aW\xC1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aW\xCBWP\x93\x92PPPV[`\0aU\x08\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaX9`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aX\xCEWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aX\xFAWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10aY\x18Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10aY0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10aYDWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10aYVW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x10\xF4W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80TaY\xAC\x90ay\\V[`\0\x82U\x80`\x1F\x10aY\xBCWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a5\xE0\x91\x90a[\xD3V[\x82\x80TaY\xE6\x90ay\\V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aZ\x08W`\0\x85UaZNV[\x82`\x1F\x10aZ!W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85UaZNV[\x82\x80\x01`\x01\x01\x85U\x82\x15aZNW\x91\x82\x01[\x82\x81\x11\x15aZNW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aZ3V[PaZZ\x92\x91Pa[\xD3V[P\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01aZ\xECaZ\x8DV[\x81R` \x01aZ\xF9aZ\x8DV[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@Q\x80`\xA0\x01`@R\x80a[\x92a[\xE8V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80a[\xC6aZ\xCCV[\x81R`\0` \x90\x91\x01R\x90V[[\x80\x82\x11\x15aZZW`\0\x81U`\x01\x01a[\xD4V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\\0W`\0\x80\xFD[a\"\r\x83\x83a\\\x06V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x10\xF4V[`\0`\xC0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\\\x88W`\0\x80\xFD[a\"\r\x83\x83a\\dV[`\0`\xA0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\\\xB6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\\\xCCW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\\x92V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\\\xFBW\x81\x81\x01Q\x83\x82\x01R` \x01a\\\xE3V[\x83\x81\x11\x15a-\tWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra]$\x81` \x86\x01` \x86\x01a\\\xE0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81Ra]\x83` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Ra\\\xD8`\xC0\x84\x01\x82a]\x0CV[`\0`\x80\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a]\xC1W`\0\x80\xFD[a\"\r\x83\x83a]\x9DV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x10\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5\xE0W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a^5W`\0\x80\xFD[\x855a^@\x81a^\x08V[\x94P` \x86\x015a^P\x81a^\x08V[\x93P`@\x86\x015a^`\x81a^\x08V[\x92P``\x86\x015a^p\x81a^\x08V[\x91P`\x80\x86\x015a^\x80\x81a^\x08V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0`@\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a^\xB2W`\0\x80\xFD[a\"\r\x83\x83a^\x8EV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a_.W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x01R``\x90\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x85\x01R`\x80\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a^\xD9V[P\x91\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a_MW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_dW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a_\x7FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a_\x99W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xAFW`\0\x80\xFD[a_\xBB\x85\x82\x86\x01a_;V[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5\xE0W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aa'Waa'a_\xF5V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aa@W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aaYWaaYa_\xF5V[aal`\x1F\x82\x01`\x1F\x19\x16` \x01a`\xFFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aa\x81W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aa\xB4W`\0\x80\xFD[\x845\x93P` \x85\x015aa\xC6\x81a_\xC7V[\x92Paa\xD4`@\x86\x01a_\xD9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aa\xEFW`\0\x80\xFD[aa\xFB\x87\x82\x88\x01aa/V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ab\x19W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ab2W`\0\x80\xFD[\x815a\"\r\x81a^\x08V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15abjW`\0\x80\xFD[abs\x85ab=V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ab\x8EW`\0\x80\xFD[ab\x9A\x87\x82\x88\x01a_;V[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15ab\xBFW`\0\x80\xFD[ab\xC7a`\x0BV[ab\xD0\x83ab=V[\x81Rab\xDE` \x84\x01ab=V[` \x82\x01Rab\xEF`@\x84\x01ab=V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ac\x1FW`\0\x80\xFD[a\"\r\x83\x83ab\xFBV[`\0` \x82\x84\x03\x12\x15ac;W`\0\x80\xFD[\x815a\"\r\x81a_\xC7V[\x80`\x0F\x0B\x81\x14a5\xE0W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15achW`\0\x80\xFD[\x825acs\x81a_\xC7V[\x91P` \x83\x015ac\x83\x81acFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15ac\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac\xB6W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a^\x8EV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ac\xD6V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ad\x17V[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rada`\xE0\x85\x01\x82ac\xC2V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rad~\x82\x82ad\x03V[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90ad\xB1\x81\x83a]\x0CV[\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ad\xDEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xF4W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01ad\xBAV[ae^\x82\x82Q\x80Q\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`\x80\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x82\x01Q`\xE0`\xC0\x85\x01Ra\\\xD8`\xE0\x85\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84ae\0V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ae\xA0V[` \x81R`\0\x82Q`@` \x84\x01Rae\xDB``\x84\x01\x82ad\x03V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Rad\xB1\x82\x82ae\x8CV[` \x81R`\x01`\x01`\x80\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra\\\xD8``\x84\x01\x82ae\x8CV[`\0` \x82\x84\x03\x12\x15af8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15afNW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\dV[af\x8B\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xC0``\x85\x01Raf\xA6`\xC0\x85\x01\x82a]\x0CV[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Rad\xB1\x82\x82ae\x8CV[` \x81R`\0a\"\r` \x83\x01\x84afZV[`\0` \x82\x84\x03\x12\x15af\xF1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\x07W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\"\rW`\0\x80\xFD[`\0a\x01\0ag}\x84\x84Q\x80Q\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`\x80\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[c\xFF\xFF\xFF\xFF` \x84\x01Q\x16`\xC0\x85\x01R`@\x83\x01Q\x81`\xE0\x86\x01Rad\xB1\x82\x86\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84ag\x1AV[`\0` \x82\x84\x03\x12\x15ag\xC9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xDFW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\\x06V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rah\x10``\x85\x01\x82ae\0V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Rad\xB1\x82\x82ae\0V[` \x81R`\0a\"\r` \x83\x01\x84ag\xEBV[` \x81Ra]\x83` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15ah\x9AW`\0\x80\xFD[ah\xA3\x87ab=V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xBEW`\0\x80\xFD[ah\xCA\x89\x82\x8A\x01a_;V[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91P`\x80\x87\x015`\xFF\x81\x16\x81\x14ah\xF2W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15ai\x12W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai(W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a]\x9DV[` \x81Raic` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Ra\\\xD8`\xA0\x84\x01\x82a]\x0CV[`\0\x80\x83`\x1F\x84\x01\x12ai\x8FW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xA6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a_\x7FW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15ai\xD3W`\0\x80\xFD[\x835ai\xDE\x81a^\x08V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xF9W`\0\x80\xFD[aj\x05\x86\x82\x87\x01ai}V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80` \x83\x85\x03\x12\x15aj%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aj;W`\0\x80\xFD[a_\xBB\x85\x82\x86\x01ai}V[`\0\x80`\0``\x84\x86\x03\x12\x15aj\\W`\0\x80\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14ajtW`\0\x80\xFD[\x92P` \x84\x015aj\x84\x81a_\xC7V[\x91Paj\x92`@\x85\x01a_\xD9V[\x90P\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x10\xF4V[` \x81Rak)` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Ra\\\xD8a\x01\0\x84\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84a]\x0CV[`\0`\xE0\x82\x84\x03\x12\x15akiW`\0\x80\xFD[a\"\r\x83\x83ad\xBAV[`\0\x80`@\x83\x85\x03\x12\x15ak\x86W`\0\x80\xFD[ak\x8F\x83ab=V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ak\xABW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15ak\xBFW`\0\x80\xFD[ak\xC7a`\x0BV[ak\xD0\x83ab=V[\x81R` \x83\x015ak\xE0\x81a^\x08V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ak\xF7W`\0\x80\xFD[al\x03\x88\x82\x86\x01aa/V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x10\xF4\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Ral\x9C`\xC0\x85\x01\x82ac\xC2V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Rad\xB1\x81\x83a]\x0CV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Ral\xEA``\x85\x01\x82ad\x03V[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15am+W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90am\x0BV[P\x96\x95PPPPPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Ra\\\xD8`\x80\x84\x01\x82ad\x03V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x10\xF4V[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15am\xE0W\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01am\xC1V[PPP\x83\x01Q`\xE0`\x80\x84\x01Ram\xFBa\x01\0\x84\x01\x82a]\x0CV[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Ran\x19\x83\x83ad\x03V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPan7\x82\x82ad\x03V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a_.W\x82\x84\x03\x89Ran\x84\x84\x83Qa]\x0CV[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01anlV[` \x81R\x81Q`\x0F\x0B` \x82\x01R`\0` \x83\x01Q```@\x84\x01Ran\xBF`\x80\x84\x01\x82anNV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Rad\xB1\x82\x82anNV[`\0` \x82\x84\x03\x12\x15an\xEEW`\0\x80\xFD[a\"\r\x82ab=V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rao:`\xC0\x85\x01\x82a]\x0CV[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[` \x81R`\0\x82Q`@` \x84\x01Raoo``\x84\x01\x82ag\xEBV[\x90P` \x84\x01Q`\x0F\x0B`@\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x80\x83R`\x80\x83\x01\x84Q``\x83\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P`\xA0\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15ao\xE8W`\x9F\x19\x88\x86\x03\x01\x83Rao\xD6\x85\x85Qa]\x0CV[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01ao\xBAV[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90ap\x07\x81\x83ad\x03V[\x91PP`@\x84\x01Q``\x84\x01R\x80\x91PP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15ap0W`\0\x80\xFD[ap8a`\x0BV[\x90P\x815\x81R` \x82\x015` \x82\x01RapT`@\x83\x01ab=V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15apqW`\0\x80\xFD[a\"\r\x83\x83ap\x1EV[\x805\x80\x15\x15\x81\x14a_\xF0W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15ap\x9DW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15ap\xBFWap\xBFa_\xF5V[`@R\x825ap\xCD\x81a_\xC7V[\x81Rap\xDB` \x84\x01ap{V[` \x82\x01R`@\x83\x015ap\xEE\x81a_\xC7V[`@\x82\x01R``\x83\x015aq\x01\x81acFV[``\x82\x01R`\x80\x83\x015aq\x14\x81acFV[`\x80\x82\x01R`\xA0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15aq>W`\0\x80\xFD[aqFa`3V[\x90P\x815\x81R` \x82\x015aqZ\x81a_\xC7V[` \x82\x01Raqk`@\x83\x01a_\xD9V[`@\x82\x01Raq|``\x83\x01ab=V[``\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aq\x99W`\0\x80\xFD[aq\xA1a`UV[\x90Paq\xAD\x83\x83aq,V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[aq\xD4\x84\x82\x85\x01aa/V[` \x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83aq\x87V[`\0`\x80\x82\x84\x03\x12\x15aq\xFEW`\0\x80\xFD[ar\x06a`3V[\x90P\x815\x81R` \x82\x015` \x82\x01Raqk`@\x83\x01a_\xD9V[`\0`\x80\x82\x84\x03\x12\x15ar4W`\0\x80\xFD[a\"\r\x83\x83aq\xECV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10arvWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15ar\x8EW`\0\x80\xFD[\x81Qa\"\r\x81a^\x08V[`\0`@\x82\x84\x03\x12\x15ar\xABW`\0\x80\xFD[ar\xB3a`UV[ar\xBC\x83a_\xD9V[\x81R` \x83\x015ar\xCC\x81a^\x08V[` \x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ar\xEAW`\0\x80\xFD[ar\xF2a`UV[\x825ar\xFD\x81a_\xC7V[\x81R` \x83\x015ar\xCC\x81acFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12as:W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15asTW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a_\x7FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01as\x91Was\x91asiV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15as\xBAWas\xBAasiV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80as\xDCWas\xDCasiV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03at\x02Wat\x02asiV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80at9Wat9at\x0CV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15at`Wat`asiV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15at\xA2Wat\xA2asiV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15at\xCEWat\xCEasiV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15at\xEAWat\xEAasiV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15au\0Wau\0asiV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qau1\x81`\x01\x85\x01` \x87\x01a\\\xE0V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x82\x82\x10\x15auQWauQasiV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15auhW`\0\x80\xFD[aupa`wV[auy\x83ab=V[\x81R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15au\x94W`\0\x80\xFD[au\x9Ca`\x0BV[\x825\x81R` \x83\x015au\xAE\x81a_\xC7V[` \x82\x01Rab\xEF`@\x84\x01a_\xD9V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15au\xD8Wau\xD8a_\xF5V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12au\xF3W`\0\x80\xFD[\x815` av\x08av\x03\x83au\xBFV[a`\xFFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av'W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805av>\x81a_\xC7V[\x83R\x91\x83\x01\x91\x83\x01av+V[`\0\x82`\x1F\x83\x01\x12av\\W`\0\x80\xFD[\x815` avlav\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av\x8BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805\x83R\x91\x83\x01\x91\x83\x01av\x8FV[`\0`@\x826\x03\x12\x15av\xB8W`\0\x80\xFD[av\xC0a`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15av\xD7W`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15av\xECW`\0\x80\xFD[av\xF4a`3V[\x825\x81R` \x83\x015\x82\x81\x11\x15aw\nW`\0\x80\xFD[aw\x166\x82\x86\x01au\xE2V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15aw.W`\0\x80\xFD[aw:6\x82\x86\x01avKV[`@\x83\x01RPawL``\x84\x01ab=V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15awgW`\0\x80\xFD[Paq\xD46\x82\x86\x01aa/V[`\0`\xC0\x82\x84\x03\x12\x15aw\x86W`\0\x80\xFD[aw\x8Ea`\x99V[\x90P\x815\x81R` \x82\x015aw\xA2\x81acFV[` \x82\x01R`@\x82\x015aw\xB5\x81acFV[`@\x82\x01Raw\xC6``\x83\x01ab=V[``\x82\x01Raw\xD7`\x80\x83\x01ab=V[`\x80\x82\x01Raw\xE8`\xA0\x83\x01a_\xD9V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15ax\x05W`\0\x80\xFD[ax\ra`UV[\x90Pax\x19\x83\x83awtV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[`\0a\x10\xF46\x83aw\xF3V[`\0\x82`\x1F\x83\x01\x12axQW`\0\x80\xFD[\x815` axaav\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ax\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805ax\x97\x81acFV[\x83R\x91\x83\x01\x91\x83\x01ax\x84V[`\0`@\x826\x03\x12\x15ax\xB6W`\0\x80\xFD[ax\xBEa`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ax\xD5W`\0\x80\xFD[ax\xE16\x83\x87\x01avKV[\x83R` \x85\x015\x91P\x80\x82\x11\x15ax\xF7W`\0\x80\xFD[Paq\xD46\x82\x86\x01ax@V[`\0`@\x82\x84\x03\x12\x15ay\x16W`\0\x80\xFD[ay\x1Ea`UV[\x90Pay)\x82a_\xD9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ayDW`\0\x80\xFD[aq\xD4\x84\x82\x85\x01ax@V[`\0a\x10\xF46\x83ay\x04V[`\x01\x81\x81\x1C\x90\x82\x16\x80aypW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\\\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a\\\xD8`@\x83\x01\x84a]\x0CV[`\0` \x82\x84\x03\x12\x15ay\xC4W`\0\x80\xFD[PQ\x91\x90PV[`\0``\x82\x84\x03\x12\x15ay\xDDW`\0\x80\xFD[ay\xE5a`\x0BV[\x90P\x815\x81Ray\xF7` \x83\x01a_\xD9V[` \x82\x01RapT`@\x83\x01ab=V[`\0`\xC0\x82\x84\x03\x12\x15az\x1AW`\0\x80\xFD[az\"a`3V[\x90Paz.\x83\x83ay\xCBV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15azJW`\0\x80\xFD[azV\x85\x83\x86\x01aa/V[` \x84\x01R`\x80\x84\x015\x91Pazk\x82acFV[\x81`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15az\x85W`\0\x80\xFD[Paz\x92\x84\x82\x85\x01ax@V[``\x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83az\x08V[`\0a\x01\0\x82\x84\x03\x12\x15az\xBDW`\0\x80\xFD[az\xC5a`\x0BV[\x90Paz\xD1\x83\x83awtV[\x81R`\xC0\x82\x015az\xE1\x81a_\xC7V[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15az\xFFW`\0\x80\xFD[a{\x0B\x84\x82\x85\x01aa/V[`@\x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83az\xAAV[`\0``\x82\x84\x03\x12\x15a{5W`\0\x80\xFD[a{=a`\x0BV[\x90P\x815a{J\x81a_\xC7V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a{fW`\0\x80\xFD[a{r\x85\x83\x86\x01aw\xF3V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a{\x8BW`\0\x80\xFD[Pa{\x0B\x84\x82\x85\x01aw\xF3V[`\0a\x10\xF46\x83a{#V[`\0`\xA0\x82\x84\x03\x12\x15a{\xB6W`\0\x80\xFD[a{\xBEa`UV[\x90Paq\xAD\x83\x83aq\xECV[`\0a\x10\xF46\x83a{\xA4V[`\0`@\x82\x84\x03\x12\x15a{\xE8W`\0\x80\xFD[a{\xF0a`UV[\x825a{\xFB\x81a^\x08V[\x81R` \x83\x015ar\xCC\x81a_\xC7V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a|7W`\0\x80\xFD[a|?a`UV[\x90Pa|K\x83\x83ap\x1EV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[`\0a\x10\xF46\x83a|%V[`\0\x80\x85\x85\x11\x15a|\x82W`\0\x80\xFD[\x83\x86\x11\x15a|\x8FW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\x80\x82\x84\x03\x12\x15a|\xAEW`\0\x80\xFD[a\"\r\x83\x83aq,V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\\\xD8` \x83\x01\x84\x86a|\xB8V[`@\x81R`\0a}\t`@\x83\x01\x85\x87a|\xB8V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a}5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a}KW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a}\\W`\0\x80\xFD[\x80Qa}jav\x03\x82au\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a}\x89W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a}\xB0W\x83Qa}\xA1\x81a_\xC7V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a}\x8EV[\x97\x96PPPPPPPV[` \x81R`\0a\"\r` \x83\x01\x84ae\x8CV[`\0`\x80\x82\x84\x03\x12\x15a}\xE0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a~\x02Wa~\x02a_\xF5V[`@R\x825a~\x10\x81a_\xC7V[\x81R` \x83\x015a~ \x81a_\xC7V[` \x82\x01R`@\x83\x015a~3\x81acFV[`@\x82\x01R``\x83\x015a~F\x81acFV[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a~dW`\0\x80\xFD[a~la`UV[\x825a~w\x81a^\x08V[\x81Rar\xCC` \x84\x01a_\xD9V[`\0``\x82\x84\x03\x12\x15a~\x97W`\0\x80\xFD[a~\x9Fa`\x0BV[a~\xA8\x83ab=V[\x81R` \x83\x015au\xAE\x81a^\x08V[`\0`\xC0\x82\x84\x03\x12\x15a~\xCAW`\0\x80\xFD[a~\xD2a`\x99V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a~\xF0\x81a_\xC7V[`@\x82\x01Ra\x7F\x01``\x83\x01ap{V[``\x82\x01R`\x80\x82\x015a\x7F\x14\x81acFV[`\x80\x82\x01Raw\xE8`\xA0\x83\x01ab=V[`\0`\xE0\x82\x84\x03\x12\x15a\x7F7W`\0\x80\xFD[a\x7F?a`UV[\x90Pax\x19\x83\x83a~\xB8V[`\0` \x82\x84\x03\x12\x15a\x7F]W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x7FsW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\x7F%V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\\\x18W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x7F\xB5W`\0\x80\xFD[a\"\r\x83\x83ay\xCBV[`\0a\x10\xF46\x83a\x7F%V[`\0` \x82\x84\x03\x12\x15a\x7F\xDDW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x7F\xFFWa\x7F\xFFa_\xF5V[`@R\x90P\x80a\x80\x0E\x83a_\xD9V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x80(W`\0\x80\xFD[a\"\r\x83\x83a\x7F\xCBV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x80lW`\0\x80\xFD[a\x80ta`\xBBV[\x825\x81R` \x83\x015a\x80\x86\x81a_\xC7V[` \x82\x01R`@\x83\x015a\x80\x99\x81a_\xC7V[`@\x82\x01Ra\x80\xAA``\x84\x01a\x802V[``\x82\x01Ra\x80\xBB`\x80\x84\x01a\x802V[`\x80\x82\x01Ra\x80\xCC`\xA0\x84\x01ab=V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x80\xE5W`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x81\x03W`\0\x80\xFD[a\x81\x0Ba`\x0BV[\x825a\x81\x16\x81a_\xC7V[\x81Ra\x81$` \x84\x01a_\xD9V[` \x82\x01R`@\x83\x015ab\xEF\x81a^\x08V[`\0`\xC0\x82\x84\x03\x12\x15a\x81IW`\0\x80\xFD[a\"\r\x83\x83a~\xB8V[`\0`@\x826\x03\x12\x15a\x81eW`\0\x80\xFD[a\x81ma`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x81\x84W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x81\x99W`\0\x80\xFD[a\x81\xA1a`\x0BV[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x81\xB7W`\0\x80\xFD[a\x81\xC36\x82\x86\x01au\xE2V[` \x83\x01RPa\x81\xD5`@\x84\x01ab=V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15awgW`\0\x80\xFD[`\0`@\x826\x03\x12\x15a\x82\x02W`\0\x80\xFD[a\x82\na`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x82!W`\0\x80\xFD[a\x82-6\x83\x87\x01avKV[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x82DW`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x82WW`\0\x80\xFD[\x805a\x82eav\x03\x82au\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x82\x84W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x82\xA2W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x82\x89V[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x82\xC5W`\0\x80\xFD[a\x82\xCDa`\x0BV[\x825a\x82\xD8\x81a_\xC7V[\x81R` \x83\x015a\x82\xE8\x81acFV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x06W`\0\x80\xFD[a{\x0B6\x82\x86\x01avKV[`\0` \x82\x84\x03\x12\x15a\x83$W`\0\x80\xFD[a\x83,a`wV[\x915\x82RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x83GW`\0\x80\xFD[a\x83Oa`\x0BV[\x80``\x84\x01\x85\x81\x11\x15a\x83aW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x83{W\x805\x84R` \x93\x84\x01\x93\x01a\x83cV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x83\x98W`\0\x80\xFD[a\x83\xA0a`\xDDV[a\x83\xAA6\x84a\x836V[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x83\xC6W`\0\x80\xFD[a\x83\xD26\x83\x87\x01aa/V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x83\xEBW`\0\x80\xFD[a\x83\xF76\x83\x87\x01avKV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x84\x10W`\0\x80\xFD[Pa\x84\x1D6\x82\x86\x01avKV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x84fWa\x84fasiV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x84\x8BWa\x84\x8BasiV[P\x01\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x84\xA5W`\0\x80\xFD[\x815` a\x84\xB5av\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x84\xD4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xF7W`\0\x80\x81\xFD[a\x85\x05\x89\x86\x83\x8B\x01\x01aa/V[\x84RP\x91\x83\x01\x91\x83\x01a\x84\xD8V[`\0``\x826\x03\x12\x15a\x85%W`\0\x80\xFD[a\x85-a`\x0BV[\x825a\x858\x81acFV[\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x85TW`\0\x80\xFD[a\x85`6\x83\x87\x01a\x84\x94V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a\x85yW`\0\x80\xFD[Pa{\x0B6\x82\x86\x01a\x84\x94V[`\0`@\x82\x84\x03\x12\x15a\x85\x98W`\0\x80\xFD[a\x85\xA0a`UV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xB8W`\0\x80\xFD[a\x85\xC4\x84\x82\x85\x01a{#V[\x82RP` \x82\x015a\x85\xD5\x81acFV[` \x82\x01R\x92\x91PPV[`\0a\x10\xF46\x83a\x85\x86V[`\0``\x826\x03\x12\x15a\x85\xFEW`\0\x80\xFD[a\x86\x06a`\x0BV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x86\x1DW`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12a\x860W`\0\x80\xFD[\x815` a\x86@av\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15a\x86_W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x86\x97W\x805\x86\x81\x11\x15a\x86{W`\0\x80\x81\xFD[a\x86\x896\x86\x83\x8B\x01\x01aa/V[\x84RP\x91\x83\x01\x91\x83\x01a\x86cV[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15a\x86\xAEW`\0\x80\xFD[a\x86\xBA6\x85\x89\x01avKV[\x90\x85\x01RPPP`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x86\xE4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x86\xFAW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01aq\x87V[`\0`\xE0\x82\x84\x03\x12\x15a\x87\x18W`\0\x80\xFD[a\x87 a`\xBBV[\x82Qa\x87+\x81a^\x08V[\x81R` \x83\x01Qa\x87;\x81acFV[` \x82\x01R`@\x83\x01Qa\x87N\x81acFV[`@\x82\x01R``\x83\x01Qa\x87a\x81acFV[``\x82\x01R`\x80\x83\x01Qa\x87t\x81acFV[`\x80\x82\x01R`\xA0\x83\x01Qa\x87\x87\x81acFV[`\xA0\x82\x01R`\xC0\x83\x01Qa\x80\xE5\x81acFV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x87\xBAWa\x87\xBAasiV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x87\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x87\xEAW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01ay\x04V[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a\\\xD8`@\x83\x01\x84ae\x8CV[`\0\x80`@\x83\x85\x03\x12\x15a\x88+W`\0\x80\xFD[\x82Qa\x886\x81a_\xC7V[` \x84\x01Q\x90\x92Pac\x83\x81acFV[`\0` \x82\x84\x03\x12\x15a\x88YW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88oW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a{#V[` \x81R`\0\x82Q`\x80` \x84\x01Ra\x88\x97`\xA0\x84\x01\x82ag\xEBV[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP``\x84\x01Q`\x0F\x0B`\x80\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x88\xE5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88\xFBW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\x85\x86V[`\0` \x82\x84\x03\x12\x15a\x89\x19W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x89/W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01az\x08V[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R\x80`\0 `\0[\x83\x81\x10\x15ac\xF8W\x81T`\x01`\x01`@\x1B\x03\x16\x87R`\x01\x80\x83\x01T\x84\x89\x01R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x89\x01R`\x03\x83\x01T`\x01`\x01`\x80\x1B\x03\x16``\x89\x01R`\x80\x90\x97\x01\x96`\x04\x90\x92\x01\x91\x01a\x89TV[\x84Q\x81R` \x80\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R\x83`\x0F\x0B``\x82\x01R`\xC0`\x80\x82\x01R`\0a\x89\xF4`\xC0\x83\x01\x85a\x89;V[\x82\x81\x03`\xA0\x84\x01Ra}\xB0\x81\x85ae\x8CV[`\0` \x82\x84\x03\x12\x15a\x8A\x18W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A.W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a|%V[`\0` \x82\x84\x03\x12\x15a\x8ALW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8AbW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a{\xA4V[`\0` \x82\x84\x03\x12\x15a\x8A\x80W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\x96W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01az\xAAV[`@\x81R`\0a\x8A\xB5`@\x83\x01\x85ag\x1AV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x82a\x8A\xDCWa\x8A\xDCat\x0CV[P\x04\x90V[`\0\x82\x19\x82\x11\x15a\x8A\xF4Wa\x8A\xF4asiV[P\x01\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x8B1\x81`\x19\x85\x01` \x87\x01a\\\xE0V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x8BdWa\x8BdasiV[`\0\x03\x92\x91PPV\xFEno slow mode transactions remaining\xA2dipfsX\"\x12 ,\xFB}\xA5o\x8D\x17\xA7\xB5X\x17^\x97\xC8\xEF\x97\xC6\xE8\xBD\x10\x8EG&\xE7z\x08\xCD\xECHd.\x8EdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static ENDPOINT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x05\x96W`\x005`\xE0\x1C\x80c\x8CX\xE1\n\x11a\x02\xE2W\x80c\xBA\x8D\x81\x81\x11a\x01\x91W\x80c\xE9\xABw\xE5\x11a\0\xEEW\x80c\xF8S\x1Ad\x11a\0\xA2W\x80c\xFB\xF4\x19\x84\x11a\0|W\x80c\xFB\xF4\x19\x84\x14a\x0E\x9EW\x80c\xFE\0\x84,\x14a\x10\x19W\x80c\xFEr\xD8\xB7\x14a\x10mW`\0\x80\xFD[\x80c\xF8S\x1Ad\x14a\t\xF2W\x80c\xF9h\xC7\xF4\x14a\x0F\xF2W\x80c\xFA\xB2\xC4i\x14a\x10\x12W`\0\x80\xFD[\x80c\xEFd\xED\x0E\x11a\0\xD3W\x80c\xEFd\xED\x0E\x14a\x0F\x96W\x80c\xF2\xFD\xE3\x8B\x14a\x0F\xBFW\x80c\xF8\x08\x9D\x9C\x14a\x0F\xD2W`\0\x80\xFD[\x80c\xE9\xABw\xE5\x14a\x0F!W\x80c\xEERU&\x14a\x0FtW`\0\x80\xFD[\x80c\xD9v\x86\x95\x11a\x01EW\x80c\xDBZP!\x11a\x01*W\x80c\xDBZP!\x14a\x0E\xCEW\x80c\xE6\x04\xED\x9E\x14a\x0E\xEEW\x80c\xE7\xC8\x07Q\x14a\x0F\x01W`\0\x80\xFD[\x80c\xD9v\x86\x95\x14a\x0B\xF7W\x80c\xDB:\xA8F\x14a\x0E\xACW`\0\x80\xFD[\x80c\xC4\xF9\xB2_\x11a\x01vW\x80c\xC4\xF9\xB2_\x14a\x0E\x86W\x80c\xC5\x105\x9F\x14a\x0E\x97W\x80c\xD4\xDE\x8D\x9D\x14a\x0E\x9EW`\0\x80\xFD[\x80c\xBA\x8D\x81\x81\x14a\x0E&W\x80c\xBC\x85\xCA\x86\x14a\x0EfW`\0\x80\xFD[\x80c\x96\xC4|o\x11a\x02?W\x80c\x9F\x9A5\xE1\x11a\x01\xF3W\x80c\xB2\xBBcg\x11a\x01\xD8W\x80c\xB2\xBBcg\x14a\r\xD3W\x80c\xB3\x14}\x17\x14a\r\xF3W\x80c\xB7\x0E\xB2c\x14a\x0E\x13W`\0\x80\xFD[\x80c\x9F\x9A5\xE1\x14a\x0B\x93W\x80c\xA0\x82\xC5\xAA\x14a\r\xB3W`\0\x80\xFD[\x80c\x98\xCD2\xFE\x11a\x02$W\x80c\x98\xCD2\xFE\x14a\r3W\x80c\x9A\x08\xE55\x14a\rFW\x80c\x9E\x85\x14$\x14a\r\x93W`\0\x80\xFD[\x80c\x96\xC4|o\x14a\x0CBW\x80c\x98\xC5\xB5I\x14a\x0C\xE3W`\0\x80\xFD[\x80c\x8FO\x8E\xCC\x11a\x02\x96W\x80c\x91\xC1\xE3\xD7\x11a\x02{W\x80c\x91\xC1\xE3\xD7\x14a\x0B\xE4W\x80c\x94\xFA\xEF\xE5\x14a\x0B\xF7W\x80c\x954\xDD>\x14a\x0C\"W`\0\x80\xFD[\x80c\x8FO\x8E\xCC\x14a\x0B\xB3W\x80c\x91q\xD0\x8B\x14a\x0B\xC4W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x11a\x02\xC7W\x80c\x8D\xA5\xCB[\x14a\x0BoW\x80c\x8E]X\x8C\x14a\x0B\x80W\x80c\x8F988\x14a\x0B\x93W`\0\x80\xFD[\x80c\x8CX\xE1\n\x14a\x0B\x18W\x80c\x8D\n\xCC\x9B\x14a\x0B\\W`\0\x80\xFD[\x80c9e\x02\xB6\x11a\x04IW\x80c\\[4\xEF\x11a\x03\xA6W\x80co;\nr\x11a\x03ZW\x80c}\xB6\xA2[\x11a\x034W\x80c}\xB6\xA2[\x14a\n\xD2W\x80c\x85\xC8>\x9D\x14a\n\xE5W\x80c\x872C8\x14a\x0B\x05W`\0\x80\xFD[\x80co;\nr\x14a\nmW\x80cp\xF6\x82\x1C\x14a\n\x8DW\x80cqP\x18\xA6\x14a\n\xCAW`\0\x80\xFD[\x80ce\xDD\x13f\x11a\x03\x8BW\x80ce\xDD\x13f\x14a\n%W\x80ci\x03I\xCF\x14a\n-W\x80ci\xFD\xDC\xEC\x14a\nMW`\0\x80\xFD[\x80c\\[4\xEF\x14a\t\xF2W\x80c]O_\x97\x14a\n\x12W`\0\x80\xFD[\x80cM\x96\xA9\n\x11a\x03\xFDW\x80cU~\xD1\xBA\x11a\x03\xE2W\x80cU~\xD1\xBA\x14a\t\x9CW\x80cZ\0\x92;\x14a\t\xBCW\x80c[\xB4\xC1&\x14a\t\xDCW`\0\x80\xFD[\x80cM\x96\xA9\n\x14a\tNW\x80cO\xCF\xAEX\x14a\tsW`\0\x80\xFD[\x80c>\xDF,[\x11a\x04.W\x80c>\xDF,[\x14a\x08\xEEW\x80cA\xA0\x9B\xB6\x14a\t\x0EW\x80cB\xC7M\x1D\x14a\t.W`\0\x80\xFD[\x80c9e\x02\xB6\x14a\x08\x97W\x80c<\xECK\x93\x14a\x08\xAAW`\0\x80\xFD[\x80c\x1F\x18k'\x11a\x04\xF7W\x80c-\x035\xAB\x11a\x04\xABW\x80c2\x16\xC0b\x11a\x04\x90W\x80c2\x16\xC0b\x14a\x07\xE5W\x80c3\x8A~V\x14a\x08FW\x80c6\x8EF\x86\x14a\x08qW`\0\x80\xFD[\x80c-\x035\xAB\x14a\x07\xA0W\x80c/\x9A'D\x14a\x07\xD2W`\0\x80\xFD[\x80c\"\0`F\x11a\x04\xDCW\x80c\"\0`F\x14a\x07\\W\x80c\"\x1F\t9\x14a\x07dW\x80c\"\xD4\xA8-\x14a\x07wW`\0\x80\xFD[\x80c\x1F\x18k'\x14a\x07AW\x80c!\x04u\x89\x14a\x07TW`\0\x80\xFD[\x80c\x14sWU\x11a\x05NW\x80c\x1C\x88m\x0B\x11a\x053W\x80c\x1C\x88m\x0B\x14a\x06\xF5W\x80c\x1D\x97\xD2/\x14a\x06UW\x80c\x1D\x9E\xED\xA5\x14a\x07\nW`\0\x80\xFD[\x80c\x14sWU\x14a\x06\x8AW\x80c\x18\xED\x16\xEB\x14a\x06\xCAW`\0\x80\xFD[\x80c\rU\xE2k\x11a\x05\x7FW\x80c\rU\xE2k\x14a\x065W\x80c\x0E\xDA\xAC\xCE\x14a\x06UW\x80c\x14YEz\x14a\x06uW`\0\x80\xFD[\x80c\x05\xE4-\xC7\x14a\x05\x9BW\x80c\x0B\xB9\xC3\xA2\x14a\x05\xC4W[`\0\x80\xFD[a\x05\xAEa\x05\xA96`\x04a\\\x1EV[a\x10\xC8V[`@Qa\x05\xBB\x91\x90a\\:V[`@Q\x80\x91\x03\x90\xF3[a\x05\xD7a\x05\xD26`\x04a\\vV[a\x10\xFAV[`@Qa\x05\xBB\x91\x90`\0`\xC0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R` \x84\x01Q\x15\x15` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Q`\x0F\x0B`\x80\x83\x01R`\xA0\x83\x01Q`\xA0\x83\x01R\x92\x91PPV[a\x06Ha\x06C6`\x04a\\\xA4V[a\x11;V[`@Qa\x05\xBB\x91\x90a]8V[a\x06ha\x06c6`\x04a]\xAFV[a\x11LV[`@Qa\x05\xBB\x91\x90a]\xCBV[a\x06\x88a\x06\x836`\x04a^\x1DV[a\x11\x7FV[\0[a\x06\x9Da\x06\x986`\x04a^\xA0V[a\x16$V[`@\x80Q\x82Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[`\xA3Ta\x06\xDD\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\x06\xFDa\x16GV[`@Qa\x05\xBB\x91\x90a^\xBCV[a\x07\x1Da\x07\x186`\x04a^\xA0V[a\x16\xE3V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x92\x83\x01Q`\x0F\x0B\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x06\x88a\x07O6`\x04a_\x86V[a\x17\x06V[a\x06\x88a\x17\xA2V[a\x06\xDDa\x18RV[a\x06\x88a\x07r6`\x04aa\x9EV[a\x18\x97V[a\x06\xDDa\x07\x856`\x04ab\x07V[`\0\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x06\xDDa\x07\xAE6`\x04ab V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xA2` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x90V[a\x06\x88a\x07\xE06`\x04abTV[a\x1C\x1AV[a\x06\x88a\x07\xF36`\x04ab\xADV[\x80Q`\xA4\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UV[a\x08Ya\x08T6`\x04ac\rV[a\x1DrV[`@Q\x90Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01a\x05\xBBV[a\x08\x84a\x08\x7F6`\x04ac)V[a\x1D\x90V[`@Q`\x0F\x91\x90\x91\x0B\x81R` \x01a\x05\xBBV[a\x06\x88a\x08\xA56`\x04acUV[a\x1E$V[a\x08\xBDa\x08\xB86`\x04a\\\x1EV[a\x1F\nV[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\t\x01a\x08\xFC6`\x04ac\x8EV[a\x1F6V[`@Qa\x05\xBB\x91\x90ad3V[a\t!a\t\x1C6`\x04ad\xCCV[a\x1FGV[`@Qa\x05\xBB\x91\x90aeyV[a\tAa\t<6`\x04ac\x8EV[a\x1FXV[`@Qa\x05\xBB\x91\x90ae\xBFV[`\x9DT`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\x08\x84a\t\x816`\x04ac)V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\xA7` R`@\x90 T`\x0F\x0B\x90V[a\t\xA4a\x1FuV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R` \x01a\x05\xBBV[a\t\xCFa\t\xCA6`\x04ac\x8EV[a \x03V[`@Qa\x05\xBB\x91\x90ae\xF8V[a\t\xE4a !V[`@Q\x90\x81R` \x01a\x05\xBBV[a\n\x05a\n\x006`\x04af&V[a\"\x14V[`@Qa\x05\xBB\x91\x90af\xCCV[`\x99Ta\t[\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x06\x88a\"TV[a\n@a\n;6`\x04af\xDFV[a\"\x97V[`@Qa\x05\xBB\x91\x90ag\xA4V[a\n`a\n[6`\x04ag\xB7V[a\"\xEAV[`@Qa\x05\xBB\x91\x90ah)V[a\n\x80a\n{6`\x04a\\\xA4V[a\"\xFBV[`@Qa\x05\xBB\x91\x90ah<V[a\n\xA0a\n\x9B6`\x04a^\xA0V[a#\x0CV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x06\x88a#/V[a\x06\x88a\n\xE06`\x04ah\x81V[a#CV[a\n\xF8a\n\xF36`\x04ai\0V[a%\x15V[`@Qa\x05\xBB\x91\x90ai4V[a\x06\x88a\x0B\x136`\x04ai\xBEV[a%&V[a\x06\x88a\x0B&6`\x04acUV[c\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x06\x88a\x0Bj6`\x04aj\x12V[a-\x0FV[`gT`\x01`\x01`\xA0\x1B\x03\x16a\t[V[a\x06\x88a\x0B\x8E6`\x04ajGV[a.IV[a\x0B\xA6a\x0B\xA16`\x04a\\\x1EV[a.\xB9V[`@Qa\x05\xBB\x91\x90aj\x9BV[`\xAET`\x01`\x01`\xA0\x1B\x03\x16a\t[V[a\x0B\xD7a\x0B\xD26`\x04ad\xCCV[a.\xE5V[`@Qa\x05\xBB\x91\x90aj\xCEV[a\t[a\x0B\xF26`\x04ab\x07V[a.\xF6V[a\x0C\na\x0C\x056`\x04ac\rV[a/\xB7V[`@Q\x90Q`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x05\xBBV[a\x0C5a\x0C06`\x04ab V[a/\xD5V[`@Qa\x05\xBB\x91\x90akDV[a\x0CUa\x0CP6`\x04akWV[a0oV[`@Qa\x05\xBB\x91\x90`\0`\xE0\x82\x01\x90P\x82Q\x82R` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R\x80`@\x86\x01Q\x16`@\x85\x01RPP``\x83\x01Qw\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16``\x85\x01R\x80`\x80\x86\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`@\x1B\x03`\xA0\x84\x01Q\x16`\xA0\x83\x01R`\xC0\x83\x01Q`\x17\x0B`\xC0\x83\x01R\x92\x91PPV[a\x0C\xF6a\x0C\xF16`\x04a\\\x1EV[a0\xB7V[`@\x80Q\x82Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\x06\x88a\rA6`\x04aksV[a0\xE3V[a\rYa\rT6`\x04a\\\x1EV[a2KV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\x05\xBBV[a\r\xA6a\r\xA16`\x04a\\vV[a2wV[`@Qa\x05\xBB\x91\x90al\x16V[a\r\xC6a\r\xC16`\x04ac\x8EV[a2\xB8V[`@Qa\x05\xBB\x91\x90almV[a\r\xE6a\r\xE16`\x04ac\x8EV[a2\xC9V[`@Qa\x05\xBB\x91\x90al\xCEV[a\x0E\x06a\x0E\x016`\x04ag\xB7V[a2\xE6V[`@Qa\x05\xBB\x91\x90am6V[a\t\xE4a\x0E!6`\x04ai\xBEV[a3\x0BV[a\x0E9a\x0E46`\x04a^\xA0V[a3\xA8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x05\xBBV[a\x0Eya\x0Et6`\x04a]\xAFV[a3\xCBV[`@Qa\x05\xBB\x91\x90amnV[`\xA1T`\x01`\x01`@\x1B\x03\x16a\x06\xDDV[`\0a\x08\x84V[g\r\xE0\xB6\xB3\xA7d\0\0a\x08\x84V[a\x0E\xBFa\x0E\xBA6`\x04ac\rV[a3\xFEV[`@Q\x90Q\x81R` \x01a\x05\xBBV[a\x0E\xE1a\x0E\xDC6`\x04ad\xCCV[a4\x1CV[`@Qa\x05\xBB\x91\x90am\xB1V[a\x06\x88a\x0E\xFC6`\x04aj\x12V[a4-V[a\x0F\x14a\x0F\x0F6`\x04ag\xB7V[a7\xA3V[`@Qa\x05\xBB\x91\x90an\x96V[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x83R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x92\x84\x01\x92\x83R\x84Q\x91\x82R\x91Q\x90\x91\x16\x91\x81\x01\x91\x90\x91R\x01a\x05\xBBV[a\x0F\x87a\x0F\x826`\x04an\xDCV[a7\xD3V[`@Qa\x05\xBB\x93\x92\x91\x90an\xF7V[a\t\xE4a\x0F\xA46`\x04an\xDCV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\xA0` R`@\x90 T\x90V[a\x06\x88a\x0F\xCD6`\x04ab V[a8\xEEV[a\x0F\xE5a\x0F\xE06`\x04ac\x8EV[a9{V[`@Qa\x05\xBB\x91\x90aoSV[a\x10\x05a\x10\x006`\x04ag\xB7V[a9\x8CV[`@Qa\x05\xBB\x91\x90ao\x88V[`\xA3a\t\xE4V[a\x10,a\x10'6`\x04ab\x07V[a9\xB9V[`@\x80Q`\x01`\x01`@\x1B\x03\x90\x95\x16\x85R` \x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16``\x82\x01R`\x80\x01a\x05\xBBV[a\x10\x80a\x10{6`\x04a]\xAFV[a:\x0FV[`@Qa\x05\xBB\x91\x90`\0`\x80\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ap_V[\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ap\x8BV[a\x11CaYgV[a\x10\xF4\x82aq\xE0V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\"V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\x9FWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xB9WP0;\x15\x80\x15a\x11\xB9WP`\0T`\xFF\x16`\x01\x14[a\x120W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12SW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12[a:BV[a\x12\xB6`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cNado`\xE0\x1B\x81RP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F0.0.1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa:\xB5V[`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x99\x80T\x86\x84\x16\x90\x83\x16\x81\x17\x90\x91U`\xAE\x80T\x88\x85\x16\x90\x84\x16\x17\x90U`\xAF\x80T\x86\x85\x16\x90\x84\x16\x17\x90U`\x9C\x80T\x93\x8A\x16\x93\x90\x92\x16\x92\x90\x92\x17\x90U`@Qc].\x9A\xD1`\xE0\x1B\x81Rc].\x9A\xD1\x90a\x131\x90`\0\x90`\x04\x01arTV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13r\x91\x90ar|V[`\x9A\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\x99T`@Qc].\x9A\xD1`\xE0\x1B\x81R\x91\x16\x90c].\x9A\xD1\x90a\x13\xB6\x90`\x01\x90`\x04\x01arTV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF7\x91\x90ar|V[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x91\x90\x92\x01\x82\x90R`\xA4\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90U\x81\x80R`\xAD\x90R\x7FCI\xCF\xFE\x87\x97\n\x06Q\x90o\xE7\xEC\x1B\xC0/;4\xDF\x90\xDF\x07u\xD7V\x83\xDC\xDB\xF5l%\x85\x80T`\x01`\x01`\x80\x1B\x03\x19\x16g\r\xE0\xB6\xB3\xA7d\0\0\x17\x90U`\xAAT\x90\x03a\x15\xD6W`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R`\x02` \x83\x01\x90\x81R\x92\x82\x01\x81\x81Rg\r\xE0\xB6\xB3\xA7d\0\0``\x84\x01\x90\x81R`\xAA\x80T`\x01\x81\x01\x82U\x93R\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1A`\x04\x90\x93\x02\x92\x83\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1B\x82\x01U\x91Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1C\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UQ\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1D\x90\x91\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[\x80\x15a\x16\x1CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\x99V[```\xAA\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x16\xDAW`\0\x84\x81R` \x90\x81\x90 `@\x80Q`\x80\x81\x01\x82R`\x04\x86\x02\x90\x92\x01\x80T`\x01`\x01`@\x1B\x03\x16\x83R`\x01\x80\x82\x01T\x84\x86\x01R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x84\x01\x92\x90\x92R`\x03\x01T`\x01`\x01`\x80\x1B\x03\x16``\x83\x01R\x90\x83R\x90\x92\x01\x91\x01a\x16kV[PPPP\x90P\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83ar\xD8V[`\0[\x81\x81\x10\x15a\x17[W6`\0\x84\x84\x84\x81\x81\x10a\x17&Wa\x17&as\rV[\x90P` \x02\x81\x01\x90a\x178\x91\x90as#V[\x91P\x91Pa\x17F\x82\x82a;.V[PP\x80\x80a\x17S\x90as\x7FV[\x91PPa\x17\tV[P`\xA3\x80T\x82\x91\x90`\0\x90a\x17z\x90\x84\x90`\x01`\x01`@\x1B\x03\x16as\x98V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R[\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15a\x07\xF3W`\xA5`\0\x82` \x01\x80Qa\x18\x10\x90as\xC3V[`\x01`\x01`@\x1B\x03\x16\x90\x81\x90R\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\x18K`\x01\x83\x01\x82aY\xA0V[PPa\x17\xDBV[`\xA3\x80T`\0\x91\x90\x82\x90a\x18n\x90`\x01`\x01`@\x1B\x03\x16as\xE6V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`\xA3T\x16\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`U`\xF8\x1B` \x82\x01Rbisob\xFF\xFF\xFF\x86\x16\x03a\x18\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P``\x84\x90\x1C`\0a\x18\xF2`\ng\r\xE0\xB6\xB3\xA7d\0\0at\"V[`\x0F\x0B\x90P`\x01\x86\x14\x80\x15\x90a\x19\x1DWP`\0\x86\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15[\x15a\x19;Wa\x195g\r\xE0\xB6\xB3\xA7d\0\0`\x05atiV[`\x0F\x0B\x90P[`\x99T`@Qcd\x02\xC7\xF9`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x86\x16`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cd\x02\xC7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x9CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xB0W=`\0\x80>=`\0\xFD[PP`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x89\x16`\x04\x82\x01Ra\x1A:\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91PcE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A*\x91\x90ar|V[3\x86`\x01`\x01`\x80\x1B\x03\x16aI\xBFV[`@\x80Q``\x80\x82\x01\x83R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a\x1A\x87b\x03\xF4\x80Bas\x98V[`\x01`\x01`@\x1B\x03\x16\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`@\x01`\x01`@Q\x80``\x01`@R\x80\x8B\x81R` \x01\x8Ac\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\x01`\x01`\x80\x1B\x03\x16\x81RP`@Q` \x01a\x1B\x0B\x91\x90\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x91\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1B)\x92\x91` \x01au\x10V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x81RP`\xA5`\0\x83` \x01\x80Q\x80\x91\x90a\x1BQ\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a\x1B\xBD\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPPPV[`\0Z\x90Pa\x1C(\x85aJ\x16V[`\0[\x83\x81\x10\x15a\x1C\xF9W6`\0\x86\x86\x84\x81\x81\x10a\x1CHWa\x1CHas\rV[\x90P` \x02\x81\x01\x90a\x1CZ\x91\x90as#V[\x91P\x91Pa\x1Ch\x82\x82a;.V[`\0Za\x1Cu\x90\x86au?V[\x90P\x85\x81\x11\x15a\x1C\xE3W`\xAFT`@Qc<d\xC2\x15`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c<d\xC2\x15\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xCAW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1C\xDEW=`\0\x80>=`\0\xFD[PPPP[PPP\x80\x80a\x1C\xF1\x90as\x7FV[\x91PPa\x1C+V[P`\xAFT`\x01`\x01`\xA0\x1B\x03\x16c<d\xC2\x15\x84Za\x1D\x17\x90\x85au?V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1DSW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1DgW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83auVV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x02\x83Ra\x04\x95`\xF4\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x90\x81a\x1D\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`@Qc\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xDE\x10&\xB3\\\xC7Cg\x96\x89~\x0C\x17\xF3Wm\x95;\xE9\xC3\x05])\x1D\x1D\x1F\xBF\xAB\x98\xA5M~\x90` \x01`@Q\x80\x91\x03\x90\xA1\x91\x90PV[`\x9AT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x1EGWP`\x9BT`\x01`\x01`\xA0\x1B\x03\x163\x14[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`U`\xF8\x1B\x81RP\x90a\x1E\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[Pc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAD` \x90\x81R`@\x91\x82\x90 T\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`U`\xF8\x1B\x91\x83\x01\x91\x90\x91R`\x0F\x0B\x15a\x1E\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83au\x82V[a\x1F>aZ^V[a\x10\xF4\x82av\xA6V[a\x1FOaZ\x8DV[a\x10\xF4\x82ax4V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x10\xF4\x82ax\xA4V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01\x81\x90R`\0\x92\x91\x83\x91\x11a\x1F\xB2W\x81Qa\x1F\xB8V[\x81` \x01Q[`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81RbITI`\xE8\x1B` \x82\x01R\x90\x91P`\x01`\x01`\x80\x1B\x03\x82\x16a\x1F\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01Ra\x10\xF4\x82ayPV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x82\x01\x83\x90R`\0\x92\x10a \xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fno slow mode transactions remain`D\x82\x01Rbing`\xE8\x1B`d\x82\x01R`\x84\x01a\x12'V[`@\x81\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA5` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91a!\x14\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!@\x90ay\\V[\x80\x15a!\x8DW\x80`\x1F\x10a!bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x8DV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!pW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x81\x01Q`@\x80\x83\x01Q\x90Qc\xB7\x0E\xB2c`\xE0\x1B\x81R\x92\x93P0\x92c\xB7\x0E\xB2c\x92a!\xC7\x92\x90\x91`\x04\x01ay\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\"\0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra!\xFD\x91\x81\x01\x90ay\xB2V[`\x01[a\"\rWP`\0\x92\x91PPV[\x93\x92PPPV[`@\x80Q`\xE0\x81\x01\x82R`\0`\x80\x82\x01\x81\x81R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x82\x90R\x82R``` \x83\x01\x81\x90R\x92\x82\x01R\x81\x81\x01\x91\x90\x91Ra\x10\xF4\x82az\x9EV[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91Ra\x07\xF3\x81`\0aJ^V[`@\x80Qa\x01 \x81\x01\x82R`\0``\x80\x83\x01\x82\x81R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x83\x90R`\xC0\x84\x01\x83\x90R`\xE0\x84\x01\x83\x90Ra\x01\0\x84\x01\x83\x90R\x83R` \x83\x01\x91\x90\x91R\x91\x81\x01\x91\x90\x91Ra\x10\xF4\x82a{\x17V[a\"\xF2aZ\xCCV[a\x10\xF4\x82a{\x98V[a#\x03aZ\xFEV[a\x10\xF4\x82a{\xCAV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83a{\xD6V[a#7aL\xFDV[a#A`\0aMWV[V[a#L\x86aJ\x16V[`\x9DT`\x01`\x01`\xA0\x1B\x03\x163\x14a#cW`\0\x80\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x88\x16` \x82\x01R`\0\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0[\x85\x81\x10\x15a#\xFDW\x81\x87\x87\x83\x81\x81\x10a#\xB0Wa#\xB0as\rV[\x90P` \x02\x81\x01\x90a#\xC2\x91\x90as#V[`@Q` \x01a#\xD4\x93\x92\x91\x90a|\x0BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91P\x80a#\xF6\x90as\x7FV[\x90Pa#\x95V[P`\xAFT`@Qc\x15b<[`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x81\x01\x85\x90R`\xFF\x84\x16`d\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c*\xC4x\xB6\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a$[W`\0\x80\xFD[PZ\xF1\x15\x80\x15a$oW=`\0\x80>=`\0\xFD[PPPP`\0[\x85\x81\x10\x15a%\x0BW6`\0\x88\x88\x84\x81\x81\x10a$\x93Wa$\x93as\rV[\x90P` \x02\x81\x01\x90a$\xA5\x91\x90as#V[\x91P\x91Pa$\xB3\x82\x82a;.V[`\xA3\x80T`\x01\x91\x90`\0\x90a$\xD2\x90\x84\x90`\x01`\x01`@\x1B\x03\x16as\x98V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPP\x80\x80a%\x03\x90as\x7FV[\x91PPa$vV[PPPPPPPPV[a%\x1Da[.V[a\x10\xF4\x82a|fV[30\x14a%2W`\0\x80\xFD[`\0\x82\x82`\0\x81\x81\x10a%GWa%Gas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a%bWa%bar>V[\x90P`\x01\x81`\x1D\x81\x11\x15a%xWa%xar>V[\x03a&<W`\0a%\x8C\x83`\x01\x81\x87a|rV[\x81\x01\x90a%\x99\x91\x90au\x82V[\x90Pa%\xA9\x81`\0\x01Q\x86aM\xA9V[\x80Qa%\xB4\x90aN\x07V[`\x99T`@\x80Qc3\x93\x8B\x91`\xE1\x1B\x81R\x83Q`\x04\x82\x01R` \x84\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x90\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg'\x17\"\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&2W=`\0\x80>=`\0\xFD[PPPPPa-\tV[`\x02\x81`\x1D\x81\x11\x15a&PWa&Par>V[\x03a&\xF4W`\0a&d\x83`\x01\x81\x87a|rV[\x81\x01\x90a&q\x91\x90a|\x9CV[\x90Pa&\x81\x81`\0\x01Q\x86aM\xA9V[`\x99T\x81Q` \x83\x01Q`@\x80\x85\x01Q`\xA3T\x91Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x90\x92\x16`$\x84\x01R`\x01`\x01`\x80\x1B\x03\x90\x91\x16`D\x83\x01R`\0`d\x83\x01R`\x01`\x01`@\x1B\x03\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a&\x04V[`\x07\x81`\x1D\x81\x11\x15a'\x08Wa'\x08ar>V[\x03a'vW`\x99T`@Qc\xAF\x97\x91\xD1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xAF\x97\x91\xD1\x90a'?\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'YW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'mW=`\0\x80>=`\0\xFD[PPPPa-\tV[`\r\x81`\x1D\x81\x11\x15a'\x8AWa'\x8Aar>V[\x03a'\xF9W`\0a'\x9E\x83`\x01\x81\x87a|rV[\x81\x01\x90a'\xAB\x91\x90ap_V[\x90Pa'\xBB\x81`\0\x01Q\x86aM\xA9V[\x80Qa'\xC6\x90aN\x96V[` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xA8\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90Ua-\tV[`\x12\x81`\x1D\x81\x11\x15a(\rWa(\rar>V[\x03a(SW`\x99T`\xA3T`@Qc\x9E\xEC\xEE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x9E\xEC\xEE5\x91a'?\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a|\xF5V[`\x14\x81`\x1D\x81\x11\x15a(gWa(gar>V[\x03a(\x9EW`\x99T`@Qc&\xF5\xA8\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c&\xF5\xA8\x01\x90a'?\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\t\x81`\x1D\x81\x11\x15a(\xB2Wa(\xB2ar>V[\x03a+9W`\xAE`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp|\x8BX`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\x07W`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\x1BW=`\0\x80>=`\0\xFD[PPPP`\0`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)\x9C\x91\x90\x81\x01\x90a}\"V[\x90P`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xB9Wa)\xB9a_\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a*\xC5W`\xA7`\0\x84\x83\x81Q\x81\x10a*\x07Wa*\x07as\rV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x0F\x0B\x82\x82\x81Q\x81\x10a*IWa*Ias\rV[` \x02` \x01\x01\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP`\0`\xA7`\0\x85\x84\x81Q\x81\x10a*uWa*uas\rV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a*\xBD\x81as\x7FV[\x91PPa)\xE8V[Pa*\xD0`\x01aN\x96V[`\x99T`@Qc\x8B\x94\x1D\xFB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8B\x94\x1D\xFB\x90a+\0\x90\x84\x90`\x04\x01a}\xBBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\x1AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+.W=`\0\x80>=`\0\xFD[PPPPPPa-\tV[`\x10\x81`\x1D\x81\x11\x15a+MWa+Mar>V[\x03a+\x93W`\x99T`\xA3T`@Qcl\xF3)G`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xD9\xE6R\x8E\x91a'?\x91\x87\x91\x87\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x04\x01a|\xF5V[`\x18\x81`\x1D\x81\x11\x15a+\xA7Wa+\xA7ar>V[\x03a,(W`\0a+\xBB\x83`\x01\x81\x87a|rV[\x81\x01\x90a+\xC8\x91\x90a}\xCEV[`\xAET`@\x80Qc\x068\xF6\xF3`\xE5\x1B\x81R\x83Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\x04\x83\x01R` \x85\x01Q\x16`$\x82\x01R\x90\x83\x01Q`\x0F\x90\x81\x0B`D\x83\x01R``\x84\x01Q\x90\x0B`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC7\x1E\xDE`\x90`\x84\x01a&\x04V[`\x19\x81`\x1D\x81\x11\x15a,<Wa,<ar>V[\x03a,wW`\0a,P\x83`\x01\x81\x87a|rV[\x81\x01\x90a,]\x91\x90a~RV[\x90Pa,q\x81`\0\x01Q\x82` \x01QaN\xFFV[Pa-\tV[`\x1A\x81`\x1D\x81\x11\x15a,\x8BWa,\x8Bar>V[\x03a,\xC5W`\0a,\x9F\x83`\x01\x81\x87a|rV[\x81\x01\x90a,\xAC\x91\x90a~\x85V[\x90Pa,q\x81`\0\x01Q\x82` \x01Q\x83`@\x01QaP\xA3V[`\x1B\x81`\x1D\x81\x11\x15a,\xD9Wa,\xD9ar>V[\x03a\x05\x96W`\0a,\xED\x83`\x01\x81\x87a|rV[\x81\x01\x90a,\xFA\x91\x90auVV[\x90Pa,q\x81`\0\x01QaR\xDFV[PPPPV[`\0\x82\x82`\0\x81\x81\x10a-$Wa-$as\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a-?Wa-?ar>V[\x90P`\0\x81`\x1D\x81\x11\x15a-UWa-Uar>V[\x03a-\xB2W`\0a-i\x83`\x01\x81\x87a|rV[\x81\x01\x90a-v\x91\x90a\x7FKV[\x80QQ\x90\x91P`\x02\x14a-\xACW\x80Q\x80Q`\xA0\x90\x91\x01Qa-\x97\x91\x90aS\xDEV[\x80QQa-\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[Pa.\x06V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7Fcritical error: expected liquida`D\x82\x01Rc:4\xB7\xB7`\xE1\x1B`d\x82\x01R`\x84\x01a\x12'V[`\xA3\x80T`\x01`\x01`@\x1B\x03\x16\x90`\0a.\x1F\x83as\xE6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPPPPPV[`@\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x193``\x1B\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x19\x85\x16`4\x82\x01Ra.\xB4\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra.\x92\x90a\x7F\x7FV[\x83\x83`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a-1`\xF0\x1B\x81RPa\x18\x97V[PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x7F\xA3V[a.\xEDaZ\x8DV[a\x10\xF4\x82a\x7F\xBFV[`\0b\xFF\xFF\xFF\x82\x16biso\x14a/$W`\0\x82\x81R`\xA8` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x10\xF4V[`\xAET`@Qc\x13\xB5m\xDB`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\xA8\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x13\xB5m\xDB\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\x97\x91\x90ay\xB2V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80\x16V[`\xAC` R`\0\x90\x81R`@\x90 \x80Ta/\xEE\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\x1A\x90ay\\V[\x80\x15a0gW\x80`\x1F\x10a0<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0gV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0JW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80ZV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a~\x85V[`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01\x81\x90R`\x01`\x80\x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x91\x84\x16\x14a1oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01R\x7Fnot next tx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x12'V[\x81`\xA5`\0\x83` \x01\x80Q\x80\x91\x90a1\x86\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a1\xF2\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x80\xF1V[`@\x80Q`\xC0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x817V[a2\xC0a[WV[a\x10\xF4\x82a\x81SV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01Ra\x10\xF4\x82a\x81\xF0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91Ra\x10\xF4\x82a\x82\xB3V[`\0\x80\x83\x83`\0\x81\x81\x10a3!Wa3!as\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a3<Wa3<ar>V[\x90P`\r\x81`\x1D\x81\x11\x15a3RWa3Rar>V[\x03a3\x9DW`\0a3f\x84`\x01\x81\x88a|rV[\x81\x01\x90a3s\x91\x90ap_V[\x80Q\x90\x91P``\x1C`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a3\x91W`\0a3\x94V[\x80Q[\x92PPPa\"\rV[P`\0\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x10\xF46\x83\x90\x03\x83\x01\x83a~RV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a|\x9CV[`@\x80Q` \x81\x01\x90\x91R`\0\x81Ra\x10\xF46\x83\x90\x03\x83\x01\x83a\x83\x12V[a4$a[\x7FV[a\x10\xF4\x82a\x83\x86V[`\0\x82\x82`\0\x81\x81\x10a4BWa4Bas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a4]Wa4]ar>V[\x90P3`\x01\x82`\x1D\x81\x11\x15a4tWa4tar>V[\x03a4~W`\0\x80\xFD[`\x07\x82`\x1D\x81\x11\x15a4\x92Wa4\x92ar>V[\x03a4\xD8W`\0a4\xA6\x84`\x01\x81\x88a|rV[\x81\x01\x90a4\xB3\x91\x90a\x80\x16V[\x90Pa4\xD2a4\xC0aT\x9AV[\x82Q\x84\x90`\x01`\x01`\x80\x1B\x03\x16aI\xBFV[Pa6&V[`\x12\x82`\x1D\x81\x11\x15a4\xECWa4\xECar>V[\x14\x80a5\tWP`\x14\x82`\x1D\x81\x11\x15a5\x07Wa5\x07ar>V[\x14[\x80a5%WP`\t\x82`\x1D\x81\x11\x15a5#Wa5#ar>V[\x14[\x80a5AWP`\x10\x82`\x1D\x81\x11\x15a5?Wa5?ar>V[\x14[\x80a5]WP`\x18\x82`\x1D\x81\x11\x15a5[Wa5[ar>V[\x14[\x80a5yWP`\x19\x82`\x1D\x81\x11\x15a5wWa5war>V[\x14[\x80a5\x95WP`\x1A\x82`\x1D\x81\x11\x15a5\x93Wa5\x93ar>V[\x14[\x80a5\xB1WP`\x1B\x82`\x1D\x81\x11\x15a5\xAFWa5\xAFar>V[\x14[\x15a5\xD5W`gT`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x16\x14a5\xD0W`\0\x80\xFD[a6&V[a5\xE3a5\xE0aT\x9AV[PV[`\xAB\x80Tb\x0FB@\x91\x90`\0\x90a5\xFE\x90\x84\x90`\x0F\x0Ba\x843V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP[`@\x80Q``\x80\x82\x01\x83R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x84R`\x01`@\x1B\x82\x04\x81\x16` \x85\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x82\x84\x01R\x82Q\x90\x81\x01\x90\x92R\x90\x80a6sb\x03\xF4\x80Bas\x98V[`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RPP` \x84\x01\x80Q`\xA5\x93P\x90a6\xDC\x82as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x90\x81\x16\x82R` \x80\x83\x01\x93\x90\x93R`@\x91\x82\x01`\0 \x84Q\x81T\x86\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x91\x90\x93\x16\x17\x91\x90\x91\x17\x81U\x90\x83\x01Q\x80Q\x91\x92a7H\x92`\x01\x85\x01\x92\x90\x91\x01\x90aY\xDAV[PP\x81Q`\xA4\x80T` \x85\x01Q`@\x90\x95\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x96\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x93\x90\x93\x16\x17\x90\x91UPPPPPV[a7\xCA`@Q\x80``\x01`@R\x80`\0`\x0F\x0B\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x10\xF4\x82a\x85\x13V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x81\x16\x83R`\xA5\x82R\x85\x83 `\xA4T\x87Q\x95\x86\x01\x88R\x81T\x80\x84\x16\x87R`\x01`@\x1B\x90\x81\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x94\x87\x01\x94\x90\x94R`\x01\x82\x01\x80T\x97\x98\x95\x97\x88\x97\x93\x96`\x01`\x80\x1B\x84\x04\x86\x16\x96\x90\x93\x04\x90\x94\x16\x93\x86\x92\x84\x01\x91\x90a8]\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta8\x89\x90ay\\V[\x80\x15a8\xD6W\x80`\x1F\x10a8\xABWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a8\xD6V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a8\xB9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x92P\x92P\x92P\x92P\x91\x93\x90\x92PV[a8\xF6aL\xFDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a9rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12'V[a5\xE0\x81aMWV[a9\x83a[\xB3V[a\x10\xF4\x82a\x85\xE0V[a9\xB0`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01`\0\x81RP\x90V[a\x10\xF4\x82a\x85\xECV[`\xAA\x81\x81T\x81\x10a9\xC9W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x90\x93\x01T`\x01`\x01`@\x1B\x03\x90\x92\x16\x93P\x91`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`\x80\x1B\x03\x16\x84V[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91Ra\x10\xF46\x83\x90\x03\x83\x01\x83a}\xCEV[`\0Ta\x01\0\x90\x04`\xFF\x16a:\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a#AaU\rV[`\0Ta\x01\0\x90\x04`\xFF\x16a; W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a;*\x82\x82aU\x81V[PPV[`\0\x82\x82`\0\x81\x81\x10a;CWa;Cas\rV[\x91\x90\x91\x015`\xF8\x1C\x90P`\x1D\x81\x11\x15a;^Wa;^ar>V[\x90P`\0\x81`\x1D\x81\x11\x15a;tWa;tar>V[\x03a<^W`\0a;\x88\x83`\x01\x81\x87a|rV[\x81\x01\x90a;\x95\x91\x90a\x7FKV[\x80QQ\x90\x91P`\x02\x14a<\0W\x80Q\x80Q`\xA0\x90\x91\x01Qa;\xB6\x91\x90aS\xDEV[\x80QQa;\xDF\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[`\0\x93\x92PPPV[aV\x06V[\x80QQa;\xEB\x90aN\x96V[\x80QQa<\0\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`\x99T\x81Q`@QcR\xEF\xAD\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91cR\xEF\xAD\xF1\x91a<0\x91`\x04\x01al\x16V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<JW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%\x0BW=`\0\x80>=`\0\xFD[`\x02\x81`\x1D\x81\x11\x15a<rWa<rar>V[\x03a=\xC7W`\0a<\x86\x83`\x01\x81\x87a|rV[\x81\x01\x90a<\x93\x91\x90a\x86\xD2V[\x80Q\x80Q``\x90\x91\x01Q\x91\x92Pa<\xA9\x91aS\xDEV[\x80QQa<\xC4\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80Q\x80Q`\x9AT` \x90\x92\x01Q`@Qc8\xD0\xDC\xE3`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ra=R\x92`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE3Cs\x8C\x90`$\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=C\x91\x90a\x87\x06V[`\xA0\x01Q\x83Q` \x01QaVTV[`\x99T\x81Q\x80Q` \x82\x01Q`@\x92\x83\x01Q`\xA3T\x93Qc3\xDC\xFB\x05`\xE1\x1B\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x90\x91\x16`$\x83\x01R`\x01`\x01`\x80\x1B\x03\x16`D\x82\x01R`\0`d\x82\x01R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cg\xB9\xF6\n\x90`\xA4\x01a<0V[`\x03\x81`\x1D\x81\x11\x15a=\xDBWa=\xDBar>V[\x03a>\xDFW`\0a=\xEF\x83`\x01\x81\x87a|rV[\x81\x01\x90a=\xFC\x91\x90a\x80\x16V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x83R`\x01`\x80\x1B\x90\x91\x04\x16` \x82\x01\x81\x90R\x91\x92P\x90`\0\x90\x15a>HW` \x82\x01Q\x83Qa>C\x91\x90a\x87\x9AV[a>KV[`\0[`\x9AT`@QcV\xB9\x9D\xC7`\xE1\x1B\x81R`\x01`\x01`\x80\x1B\x03\x83\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xADs;\x8E\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\x9AW`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\xAEW=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x93Q\x16`\x01`\x80\x1B\x90\x93\x02\x92\x90\x92\x17`\xA6UPa.\xB4\x91PPV[`\n\x81`\x1D\x81\x11\x15a>\xF3Wa>\xF3ar>V[\x03a?\xF4W`\0a?\x07\x83`\x01\x81\x87a|rV[\x81\x01\x90a?\x14\x91\x90a\x87\xC2V[`@\x80Q\x80\x82\x01\x90\x91R`\xA6T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x84R`\x01`\x80\x1B\x90\x92\x04\x16` \x83\x01R\x91\x92P\x90`\0\x90\x15a?\\W\x81Q\x83Qa?W\x91\x90a\x87\x9AV[a?_V[`\0[`\x9BT` \x85\x01Q`@Qc3\x9Bz\xED`\xE1\x1B\x81R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cg6\xF5\xDA\x91a?\x98\x91\x85\x91\x90`\x04\x01a\x87\xF6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\xC6W=`\0\x80>=`\0\xFD[PP\x93Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x80\x85R` \x90\x94\x01Q\x16`\x01`\x80\x1B\x02\x90\x92\x17`\xA6UPa.\xB4\x91PPV[`\x04\x81`\x1D\x81\x11\x15a@\x08Wa@\x08ar>V[\x03a@\xCEW`\x99T`@Qc\x876\xECG`\xE0\x1B\x81R`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x876\xECG\x90a@D\x90\x88\x90\x88\x90`\x04\x01a|\xE1V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a@bW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x86\x91\x90a\x88\x18V[\x91P\x91P\x81c\xFF\xFF\xFF\xFF\x16`\0\x14a@\xC7Wc\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xAD` R`@\x90 \x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x83\x16\x17\x90U[PPPPPV[`\x05\x81`\x1D\x81\x11\x15a@\xE2Wa@\xE2ar>V[\x03aAPW`\x99T`@Qc\xEDa\x85#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEDa\x85#\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aA3W`\0\x80\xFD[PZ\xF1\x15\x80\x15aAGW=`\0\x80>=`\0\xFD[PPPPPPPV[`\x06\x81`\x1D\x81\x11\x15aAdWaAdar>V[\x03aBbW`\0aAx\x83`\x01\x81\x87a|rV[\x81\x01\x90aA\x85\x91\x90a\x88GV[` \x81\x01QQQ\x90\x91PaA\x98\x90aN\x96V[`@\x81\x01QQQaA\xA8\x90aN\x96V[`\0`@Q\x80`\x80\x01`@R\x80\x83\x81R` \x01aA\xD0\x84` \x01Q`\0\x01Q`\0\x01QaW+V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01aA\xF3\x84`@\x01Q`\0\x01Q`\0\x01QaW+V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\0` \x90\x92\x01\x91\x90\x91R`\xAET`@Qc\x03V\x14\xB1`\xE3\x1B\x81R\x92\x93P\x16\x90c\x1A\xB0\xA5\x88\x90aB4\x90\x84\x90`\x04\x01a\x88{V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aBNW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1DgW=`\0\x80>=`\0\xFD[`\x17\x81`\x1D\x81\x11\x15aBvWaBvar>V[\x03aCLW`\0aB\x8A\x83`\x01\x81\x87a|rV[\x81\x01\x90aB\x97\x91\x90a\x88\xD3V[\x80Q` \x01QQQ\x90\x91PaB\xAB\x90aN\x96V[\x80Q`@\x01QQQaB\xBC\x90aN\x96V[`@\x80Q`\x80\x81\x01\x90\x91R\x81Q\x81R\x81Q` \x90\x81\x01QQQ`\0\x92\x91\x82\x01\x90aB\xE5\x90a.\xF6V[`\x01`\x01`\xA0\x1B\x03\x16\x81R\x83Q`@\x01QQQ` \x90\x91\x01\x90aC\x07\x90a.\xF6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x85\x81\x01Q`\x0F\x0B\x92\x01\x91\x90\x91R`\xAET`@Qc\x03V\x14\xB1`\xE3\x1B\x81R\x92\x93P\x16\x90c\x1A\xB0\xA5\x88\x90aB4\x90\x84\x90`\x04\x01a\x88{V[`\x08\x81`\x1D\x81\x11\x15aC`WaC`ar>V[\x03aC\xFEW`@\x80Q``\x81\x01\x82R`\xA4T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16` \x84\x01R`\x01`\x80\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91RaC\xA8\x81`\x01aJ^V[\x80Q`\xA4\x80T` \x84\x01Q`@\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90UPPPV[`\x15\x81`\x1D\x81\x11\x15aD\x12WaD\x12ar>V[\x03aE\x04W`\0aD&\x83`\x01\x81\x87a|rV[\x81\x01\x90aD3\x91\x90a\x89\x07V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaDI\x91aS\xDEV[\x80QQaDd\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80QQaDy\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`@\x81\x81\x01\x80Q`\x0B`\0R`\xAD` R\x7F\x8B\xFB\x1Cv\x07\xB6=uf)\xA9x\x9B\xE2\xB0N\x82<K\xA5\xF0C\xE2Es[\xCA\xB4@=\xEB:\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x99T\x83Q\x91Q``\x85\x01Q\x93Qc!vjI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93cB\xEC\xD4\x92\x93a<0\x93\x90\x92\x91`\xAA\x91`\x04\x01a\x89\xABV[`\x16\x81`\x1D\x81\x11\x15aE\x18WaE\x18ar>V[\x03aF\nW`\0aE,\x83`\x01\x81\x87a|rV[\x81\x01\x90aE9\x91\x90a\x89\x07V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaEO\x91aS\xDEV[\x80QQaEj\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80QQaE\x7F\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`@\x81\x81\x01\x80Q`\x0B`\0R`\xAD` R\x7F\x8B\xFB\x1Cv\x07\xB6=uf)\xA9x\x9B\xE2\xB0N\x82<K\xA5\xF0C\xE2Es[\xCA\xB4@=\xEB:\x80T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\x99T\x83Q\x91Q``\x85\x01Q\x93Qc\xB5\xE2-\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93c\xB5\xE2-\xBB\x93a<0\x93\x90\x92\x91`\xAA\x91`\x04\x01a\x89\xABV[`\x0B\x81`\x1D\x81\x11\x15aF\x1EWaF\x1Ear>V[\x03aFUW`\x99T`@Qc\xBF\x11\xB3\xB1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBF\x11\xB3\xB1\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\r\x81`\x1D\x81\x11\x15aFiWaFiar>V[\x03aF\xEFW`\0aF}\x83`\x01\x81\x87a|rV[\x81\x01\x90aF\x8A\x91\x90a\x8A\x06V[\x80Q\x80Q`@\x90\x91\x01Q\x91\x92PaF\xA0\x91aS\xDEV[\x80QQaF\xBB\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[Q` \x80\x82\x01Q\x91Q`\0\x90\x81R`\xA8\x90\x91R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16``\x92\x90\x92\x1C\x91\x90\x91\x17\x90UPPPV[`\x0E\x81`\x1D\x81\x11\x15aG\x03WaG\x03ar>V[\x03aG:W`\x99T`@Qc\x8F\x17\xD0A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x8F\x17\xD0A\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x0F\x81`\x1D\x81\x11\x15aGNWaGNar>V[\x03aG\xF6W`\0aGb\x83`\x01\x81\x87a|rV[\x81\x01\x90aGo\x91\x90a\x8A:V[\x90PaG\x82\x81`\0\x01Q` \x01QaN\x07V[\x80QQaG\x9D\x90a;*a;\xDA\x85a;\xD1\x88`\x01\x81\x8Ca|rV[\x80Q\x80Q``\x90\x91\x01QaG\xB1\x91\x90aS\xDEV[\x80QQaG\xC6\x90g\r\xE0\xB6\xB3\xA7d\0\0aT\x8EV[`\x99T\x81Q`@Qc\x1D\x97\xD2/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\x1D\x97\xD2/\x91a<0\x91`\x04\x01a]\xCBV[`\x11\x81`\x1D\x81\x11\x15aH\nWaH\nar>V[\x03aHAW`\x99T`@Qc\x18OSQ`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x18OSQ\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x1C\x81`\x1D\x81\x11\x15aHUWaHUar>V[\x03aH\x8CW`\xAET`@Qc\x84R\x80\x93`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x84R\x80\x93\x90aA\x19\x90\x86\x90\x86\x90`\x04\x01a|\xE1V[`\x13\x81`\x1D\x81\x11\x15aH\xA0WaH\xA0ar>V[\x03aIUW`\0aH\xB4\x83`\x01\x81\x87a|rV[\x81\x01\x90aH\xC1\x91\x90a\x8AnV[`\xAET\x81QQ\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c4\xF9\xA4\xA4\x90\x84\x90aH\xEA\x90a.\xF6V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aI\x07\x92\x91\x90a\x8A\xA2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aI&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aIJ\x91\x90ay\xB2V[\x90Pa@\xC7\x81aN\x07V[`\x1D\x81`\x1D\x81\x11\x15aIiWaIiar>V[\x03a\x05\x96W`\0aI}\x83`\x01\x81\x87a|rV[\x81\x01\x90aI\x8A\x91\x90a\x83\x12V[`\xAET\x81Q`@Qc\xF6\xEE{K`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF6\xEE{K\x90`$\x01a<0V[`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\x04\x95`\xF4\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16aI\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`\x99Ta.\xB4\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16\x83\x83V[`\xA3T`@\x80Q\x80\x82\x01\x90\x91R`\x02\x81Ra\t+`\xF3\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x83\x81\x16\x91\x16\x14a;*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x82`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10`@Q\x80``\x01`@R\x80`#\x81R` \x01a\x8Bn`#\x919\x90aJ\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`@\x82\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\0\x90\x81R`\xA5` \x90\x81R\x83\x82 \x84Q``\x81\x01\x86R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x92\x94\x91\x92\x91\x84\x01\x91aK\x11\x90ay\\V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaK=\x90ay\\V[\x80\x15aK\x8AW\x80`\x1F\x10aK_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aK\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aKmW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\xA5`\0\x84`@\x01\x80Q\x80\x91\x90aK\xAB\x90as\xE6V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x91R\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90aK\xE8`\x01\x83\x01\x82aY\xA0V[PP\x81\x80aL\x03WPB\x81`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x11\x15[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c)\xAA*)`\xE1\x1B\x81RP\x90aL>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[PFazi\x03aLsW` \x81\x01Q`@\x80\x83\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R0\x92c\x872C8\x92aA\x19\x92`\x04\x01ay\x90V[`\0Z` \x83\x01Q`@\x80\x85\x01Q\x90Qc\x10\xE6Hg`\xE3\x1B\x81R\x92\x93P0\x92c\x872C8\x92aL\xA6\x92\x90\x91`\x04\x01ay\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aL\xC0W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15aL\xD1WP`\x01[a-\tWb\x03\xD0\x90Z\x11\x15\x80aL\xF1WPaL\xED`\x02\x82a\x8A\xCDV[Z\x11\x15[\x15aL\xF8W\xFE[a-\tV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a#AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12'V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[``\x82\x90\x1C`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80aM\xCCWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14[`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01cSMWS`\xE0\x1B\x81RP\x90a.\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[`\0\x81\x81R`\x9F` R`@\x81 T`\x01`\x01`@\x1B\x03\x16\x90\x03a5\xE0W`\xA1\x80T`\0\x90aN>\x90`\x01`\x01`@\x1B\x03\x16as\xE6V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x83\x81\x02\x90\x83\x02\x19\x90\x91\x16\x17\x90\x92U`\0\x83\x81R`\x9F` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x94\x17\x90\x93U`\xA1T\x90\x93\x16\x81R`\xA0\x90\x92R\x90 UV[`\x01\x81\x14\x80aN\xA5WP`\x02\x81\x14[\x80aN\xC6WP`\0\x81\x81R`\x9F` R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x15\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aRS`\xF0\x1B\x81RP\x90a;*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[`\xAATd\x01\0\0\0\0`\x01\x82\x11\x15aOUW`\xAA\x80TaO!\x90`\x01\x90au?V[\x81T\x81\x10aO1WaO1as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T`\0\x1C`\x01aOR\x91\x90a\x8A\xE1V[\x90P[aO^\x81aN\x07V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`@\x1B\x03\x93\x84\x16\x81R` \x80\x82\x01\x84\x81R`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x83\x85\x01\x81\x81R`\x01`\x01`\x80\x1B\x03\x98\x89\x16``\x86\x01\x90\x81R`\xAA\x80T`\x01\x81\x01\x82U`\0\x91\x82R\x96Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1A`\x04\x90\x98\x02\x97\x88\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x9B\x16\x17\x90\x99U\x92Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1B\x86\x01UQ\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1C\x85\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x92\x90\x9B\x16\x91\x90\x91\x17\x90U\x90Q\x7FU\r=\xE9[\xE0\xBD(\xA7\x9C>\xB4\xEA\x7F\x05i,`\xB0`.H\xB4\x94a\xE7\x037\x9B\x08\xA7\x1D\x90\x93\x01\x80T`\x01`\x01`\x80\x1B\x03\x19\x16\x93\x90\x97\x16\x92\x90\x92\x17\x90\x95U\x91\x83R`\xA9\x90\x93R\x91\x90 \x80T\x90\x92\x16\x17\x90UV[`\xAAT`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb\x04\x94\xE5`\xEC\x1B` \x82\x01R\x90`\x01`\x01`@\x1B\x03\x85\x16\x10aP\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P\x82`\x01`\x01`@\x1B\x03\x16`\0\x03aQ\xD1W`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aQRW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7Fcannot set owner for pool 0.\0\0\0\0`D\x82\x01R`d\x01a\x12'V[`\0\x81`\x01`\x01`\x80\x1B\x03\x16\x11aQ\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7Fcannot set 0 balance weight for `D\x82\x01R\x7Fpool 0.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x12'V[\x81`\xAA\x84`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aQ\xEEWaQ\xEEas\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x80`\xAA\x84`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aR@WaR@as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x03\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UP\x81`\xA9`\0`\xAA\x86`\x01`\x01`@\x1B\x03\x16\x81T\x81\x10aR\x96WaR\x96as\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x80\x15aS\x02WP`\xAAT`\x01`\x01`@\x1B\x03\x82\x16\x10[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01b\x04\x94\xE5`\xEC\x1B\x81RP\x90aS<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x12'\x91\x90akDV[P`\x99T`\xAA\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c}\x18'}\x91\x90`\x01`\x01`@\x1B\x03\x85\x16\x90\x81\x10aSpWaSpas\rV[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aS\xA0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aS\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15aS\xCEW=`\0\x80>=`\0\xFD[PPPPa5\xE0\x81`\0\x80aP\xA3V[``\x82\x90\x1C`\0\x90\x81R`\xA2` R`@\x81 \x80T`\x01`\x01`@\x1B\x03\x16\x90\x82aT\x07\x83as\xE6V[\x91\x90a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x90P\x80`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x14a.\xB4WaTX\x81`\x01`\x01`@\x1B\x03\x16aWkV[`@Q` \x01aTh\x91\x90a\x8A\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x12'\x91`\x04\x01akDV[a;*\x82\x82`\0aVTV[`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aT\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aU\x08\x91\x90ar|V[\x90P\x90V[`\0Ta\x01\0\x90\x04`\xFF\x16aUxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[a#A3aMWV[`\0Ta\x01\0\x90\x04`\xFF\x16aU\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x12'V[\x81Q` \x92\x83\x01 \x81Q\x91\x90\x92\x01 `\x01\x91\x90\x91U`\x02UV[`\0a\x10\xF4aV\x13aX\nV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\x9AT`\x01`\x01`\xA0\x1B\x03\x16c\xE0\xB0b\x1F\x82\x85aVp\x86a\x8B>V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`\x0F\x0B`D\x82\x01R`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aV\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15aV\xD3W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xA7` R`@\x81 \x80T\x84\x92\x90aV\xFF\x90\x84\x90`\x0F\x0Ba\x843V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\x80\x1B\x03\x02\x19\x16\x90\x83`\x0F\x0B`\x01`\x01`\x80\x1B\x03\x16\x02\x17\x90UPPPPV[`\0\x80aW7\x83a.\xF6V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aWNW\x92\x91PPV[PP`\0\x90\x81R`\xA9` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[```\0aWx\x83aX\x85V[`\x01\x01\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x97WaW\x97a_\xF5V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15aW\xC1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[`\0\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84aW\xCBWP\x93\x92PPPV[`\0aU\x08\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaX9`\x01T\x90V[`\x02T`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\0\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aX\xCEWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aX\xFAWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10aY\x18Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10aY0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10aYDWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10aYVW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x10\xF4W`\x01\x01\x92\x91PPV[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[P\x80TaY\xAC\x90ay\\V[`\0\x82U\x80`\x1F\x10aY\xBCWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a5\xE0\x91\x90a[\xD3V[\x82\x80TaY\xE6\x90ay\\V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aZ\x08W`\0\x85UaZNV[\x82`\x1F\x10aZ!W\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85UaZNV[\x82\x80\x01`\x01\x01\x85U\x82\x15aZNW\x91\x82\x01[\x82\x81\x11\x15aZNW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aZ3V[PaZZ\x92\x91Pa[\xD3V[P\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01\x81\x90R`\x80\x83\x01R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Qa\x01\0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x83\x90R`\xC0\x82\x01\x83\x90R`\xE0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@Q\x80``\x01`@R\x80`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01aZ\xECaZ\x8DV[\x81R` \x01aZ\xF9aZ\x8DV[\x90R\x90V[`@\x80Q`\xC0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R`\xA0\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@\x80Q`\xA0\x81\x01\x82R`\0\x91\x81\x01\x82\x81R``\x80\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90\x81\x90aY\x93V[`@Q\x80`\xA0\x01`@R\x80a[\x92a[\xE8V[\x81R``` \x82\x01\x81\x90R`@\x82\x01\x81\x90R\x80\x82\x01R`\0`\x80\x90\x91\x01R\x90V[`@Q\x80`@\x01`@R\x80a[\xC6aZ\xCCV[\x81R`\0` \x90\x91\x01R\x90V[[\x80\x82\x11\x15aZZW`\0\x81U`\x01\x01a[\xD4V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\\0W`\0\x80\xFD[a\"\r\x83\x83a\\\x06V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x10\xF4V[`\0`\xC0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15a\\\x88W`\0\x80\xFD[a\"\r\x83\x83a\\dV[`\0`\xA0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\\\xB6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\\\xCCW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\\x92V[\x94\x93PPPPV[`\0[\x83\x81\x10\x15a\\\xFBW\x81\x81\x01Q\x83\x82\x01R` \x01a\\\xE3V[\x83\x81\x11\x15a-\tWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra]$\x81` \x86\x01` \x86\x01a\\\xE0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81Ra]\x83` \x82\x01\x83Q\x80Q\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0` \x83\x01Q`\xA0\x80\x84\x01Ra\\\xD8`\xC0\x84\x01\x82a]\x0CV[`\0`\x80\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a]\xC1W`\0\x80\xFD[a\"\r\x83\x83a]\x9DV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x10\xF4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a5\xE0W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a^5W`\0\x80\xFD[\x855a^@\x81a^\x08V[\x94P` \x86\x015a^P\x81a^\x08V[\x93P`@\x86\x015a^`\x81a^\x08V[\x92P``\x86\x015a^p\x81a^\x08V[\x91P`\x80\x86\x015a^\x80\x81a^\x08V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0`@\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a^\xB2W`\0\x80\xFD[a\"\r\x83\x83a^\x8EV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a_.W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x85R\x86\x81\x01Q\x87\x86\x01R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x01R``\x90\x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x85\x01R`\x80\x90\x93\x01\x92\x90\x85\x01\x90`\x01\x01a^\xD9V[P\x91\x97\x96PPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a_MW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_dW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a_\x7FW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a_\x99W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a_\xAFW`\0\x80\xFD[a_\xBB\x85\x82\x86\x01a_;V[\x90\x96\x90\x95P\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a5\xE0W`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`-Wa`-a_\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aa'Waa'a_\xF5V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aa@W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aaYWaaYa_\xF5V[aal`\x1F\x82\x01`\x1F\x19\x16` \x01a`\xFFV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aa\x81W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aa\xB4W`\0\x80\xFD[\x845\x93P` \x85\x015aa\xC6\x81a_\xC7V[\x92Paa\xD4`@\x86\x01a_\xD9V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aa\xEFW`\0\x80\xFD[aa\xFB\x87\x82\x88\x01aa/V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15ab\x19W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15ab2W`\0\x80\xFD[\x815a\"\r\x81a^\x08V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15abjW`\0\x80\xFD[abs\x85ab=V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ab\x8EW`\0\x80\xFD[ab\x9A\x87\x82\x88\x01a_;V[\x95\x98\x90\x97P\x94\x95`@\x015\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15ab\xBFW`\0\x80\xFD[ab\xC7a`\x0BV[ab\xD0\x83ab=V[\x81Rab\xDE` \x84\x01ab=V[` \x82\x01Rab\xEF`@\x84\x01ab=V[`@\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ac\x1FW`\0\x80\xFD[a\"\r\x83\x83ab\xFBV[`\0` \x82\x84\x03\x12\x15ac;W`\0\x80\xFD[\x815a\"\r\x81a_\xC7V[\x80`\x0F\x0B\x81\x14a5\xE0W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15achW`\0\x80\xFD[\x825acs\x81a_\xC7V[\x91P` \x83\x015ac\x83\x81acFV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15ac\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ac\xB6W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a^\x8EV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ac\xD6V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ad\x17V[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q`\x80\x80\x85\x01Rada`\xE0\x85\x01\x82ac\xC2V[\x90P`@\x82\x01Q`_\x19\x85\x83\x03\x01`\xA0\x86\x01Rad~\x82\x82ad\x03V[``\x93\x90\x93\x01Q`\x01`\x01`@\x1B\x03\x16`\xC0\x86\x01RPP` \x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90ad\xB1\x81\x83a]\x0CV[\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15a\\\x18W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ad\xDEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ad\xF4W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01ad\xBAV[ae^\x82\x82Q\x80Q\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`\x80\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x82\x01Q`\xE0`\xC0\x85\x01Ra\\\xD8`\xE0\x85\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84ae\0V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15ac\xF8W\x81Q`\x0F\x0B\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01ae\xA0V[` \x81R`\0\x82Q`@` \x84\x01Rae\xDB``\x84\x01\x82ad\x03V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Rad\xB1\x82\x82ae\x8CV[` \x81R`\x01`\x01`\x80\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`@\x80\x84\x01Ra\\\xD8``\x84\x01\x82ae\x8CV[`\0` \x82\x84\x03\x12\x15af8W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15afNW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\dV[af\x8B\x82\x82Q\x80Q\x82R` \x80\x82\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x82\x01Q`\xC0``\x85\x01Raf\xA6`\xC0\x85\x01\x82a]\x0CV[\x90P`@\x83\x01Q`\x0F\x0B`\x80\x85\x01R``\x83\x01Q\x84\x82\x03`\xA0\x86\x01Rad\xB1\x82\x82ae\x8CV[` \x81R`\0a\"\r` \x83\x01\x84afZV[`\0` \x82\x84\x03\x12\x15af\xF1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\x07W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\"\rW`\0\x80\xFD[`\0a\x01\0ag}\x84\x84Q\x80Q\x82R` \x81\x01Q`\x0F\x0B` \x83\x01R`@\x81\x01Q`\x0F\x0B`@\x83\x01R``\x81\x01Q`\x01`\x01`@\x1B\x03\x80\x82\x16``\x85\x01R\x80`\x80\x84\x01Q\x16`\x80\x85\x01RPP`\x01`\x01`\x80\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[c\xFF\xFF\xFF\xFF` \x84\x01Q\x16`\xC0\x85\x01R`@\x83\x01Q\x81`\xE0\x86\x01Rad\xB1\x82\x86\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84ag\x1AV[`\0` \x82\x84\x03\x12\x15ag\xC9W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ag\xDFW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\\\x06V[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R`\0` \x82\x01Q``` \x85\x01Rah\x10``\x85\x01\x82ae\0V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01Rad\xB1\x82\x82ae\0V[` \x81R`\0a\"\r` \x83\x01\x84ag\xEBV[` \x81Ra]\x83` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01R`\x01`\x01`\x80\x1B\x03`@\x82\x01Q\x16`@\x83\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x83\x01RPPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15ah\x9AW`\0\x80\xFD[ah\xA3\x87ab=V[\x95P` \x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xBEW`\0\x80\xFD[ah\xCA\x89\x82\x8A\x01a_;V[\x90\x96P\x94PP`@\x87\x015\x92P``\x87\x015\x91P`\x80\x87\x015`\xFF\x81\x16\x81\x14ah\xF2W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15ai\x12W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai(W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a]\x9DV[` \x81Raic` \x82\x01\x83Q\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x91\x01RV[`\0` \x83\x01Q`\x80\x80\x84\x01Ra\\\xD8`\xA0\x84\x01\x82a]\x0CV[`\0\x80\x83`\x1F\x84\x01\x12ai\x8FW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xA6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a_\x7FW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15ai\xD3W`\0\x80\xFD[\x835ai\xDE\x81a^\x08V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ai\xF9W`\0\x80\xFD[aj\x05\x86\x82\x87\x01ai}V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80` \x83\x85\x03\x12\x15aj%W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aj;W`\0\x80\xFD[a_\xBB\x85\x82\x86\x01ai}V[`\0\x80`\0``\x84\x86\x03\x12\x15aj\\W`\0\x80\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x81\x14ajtW`\0\x80\xFD[\x92P` \x84\x015aj\x84\x81a_\xC7V[\x91Paj\x92`@\x85\x01a_\xD9V[\x90P\x92P\x92P\x92V[\x81Q\x81R` \x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x10\xF4V[` \x81Rak)` \x82\x01\x83Q\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[`\0` \x83\x01Q`\xE0\x80\x84\x01Ra\\\xD8a\x01\0\x84\x01\x82a]\x0CV[` \x81R`\0a\"\r` \x83\x01\x84a]\x0CV[`\0`\xE0\x82\x84\x03\x12\x15akiW`\0\x80\xFD[a\"\r\x83\x83ad\xBAV[`\0\x80`@\x83\x85\x03\x12\x15ak\x86W`\0\x80\xFD[ak\x8F\x83ab=V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ak\xABW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15ak\xBFW`\0\x80\xFD[ak\xC7a`\x0BV[ak\xD0\x83ab=V[\x81R` \x83\x015ak\xE0\x81a^\x08V[` \x82\x01R`@\x83\x015\x82\x81\x11\x15ak\xF7W`\0\x80\xFD[al\x03\x88\x82\x86\x01aa/V[`@\x83\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\xC0\x81\x01a\x10\xF4\x82\x84\x80Q\x82R` \x81\x01Q` \x83\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q\x15\x15``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01R`\x01`\x01`@\x1B\x03`\xA0\x82\x01Q\x16`\xA0\x83\x01RPPV[` \x81R`\0\x82Q`@` \x84\x01R\x80Q``\x84\x01R` \x81\x01Q```\x80\x85\x01Ral\x9C`\xC0\x85\x01\x82ac\xC2V[\x90P`\x01`\x01`@\x1B\x03`@\x83\x01Q\x16`\xA0\x85\x01R` \x85\x01Q\x91P`\x1F\x19\x84\x82\x03\x01`@\x85\x01Rad\xB1\x81\x83a]\x0CV[`\0` \x80\x83R\x83Q`@\x82\x85\x01Ral\xEA``\x85\x01\x82ad\x03V[\x85\x83\x01Q\x85\x82\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x83R\x90\x84\x01\x92P`\0\x91\x84\x01\x90[\x80\x83\x10\x15am+W\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90am\x0BV[P\x96\x95PPPPPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R` \x82\x01Q`\x0F\x0B`@\x82\x01R`\0`@\x83\x01Q``\x80\x84\x01Ra\\\xD8`\x80\x84\x01\x82ad\x03V[\x81Q\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x10\xF4V[` \x80\x82R\x82Q`\0\x91\x90\x82\x84\x83\x01[`\x03\x82\x10\x15am\xE0W\x82Q\x81R\x91\x83\x01\x91`\x01\x91\x90\x91\x01\x90\x83\x01am\xC1V[PPP\x83\x01Q`\xE0`\x80\x84\x01Ram\xFBa\x01\0\x84\x01\x82a]\x0CV[\x90P`@\x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`\xA0\x86\x01Ran\x19\x83\x83ad\x03V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\xC0\x86\x01RPan7\x82\x82ad\x03V[\x91PP`\x80\x84\x01Q`\xE0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15a_.W\x82\x84\x03\x89Ran\x84\x84\x83Qa]\x0CV[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01anlV[` \x81R\x81Q`\x0F\x0B` \x82\x01R`\0` \x83\x01Q```@\x84\x01Ran\xBF`\x80\x84\x01\x82anNV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01Rad\xB1\x82\x82anNV[`\0` \x82\x84\x03\x12\x15an\xEEW`\0\x80\xFD[a\"\r\x82ab=V[``\x81R`\0`\x01`\x01`@\x1B\x03\x80\x86Q\x16``\x84\x01R`\x01`\x01`\xA0\x1B\x03` \x87\x01Q\x16`\x80\x84\x01R`@\x86\x01Q```\xA0\x85\x01Rao:`\xC0\x85\x01\x82a]\x0CV[\x95\x82\x16` \x85\x01RP\x92\x90\x92\x16`@\x90\x91\x01RP\x91\x90PV[` \x81R`\0\x82Q`@` \x84\x01Raoo``\x84\x01\x82ag\xEBV[\x90P` \x84\x01Q`\x0F\x0B`@\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x80\x83R`\x80\x83\x01\x84Q``\x83\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P`\xA0\x81`\x05\x1B\x88\x01\x01\x93P\x84\x83\x01\x92P`\0[\x81\x81\x10\x15ao\xE8W`\x9F\x19\x88\x86\x03\x01\x83Rao\xD6\x85\x85Qa]\x0CV[\x94P\x92\x85\x01\x92\x91\x85\x01\x91`\x01\x01ao\xBAV[PPPP\x90\x84\x01Q\x83\x82\x03`\x1F\x19\x01`@\x85\x01R\x90ap\x07\x81\x83ad\x03V[\x91PP`@\x84\x01Q``\x84\x01R\x80\x91PP\x92\x91PPV[`\0``\x82\x84\x03\x12\x15ap0W`\0\x80\xFD[ap8a`\x0BV[\x90P\x815\x81R` \x82\x015` \x82\x01RapT`@\x83\x01ab=V[`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15apqW`\0\x80\xFD[a\"\r\x83\x83ap\x1EV[\x805\x80\x15\x15\x81\x14a_\xF0W`\0\x80\xFD[`\0`\xC0\x82\x84\x03\x12\x15ap\x9DW`\0\x80\xFD[`@Q`\xC0\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15ap\xBFWap\xBFa_\xF5V[`@R\x825ap\xCD\x81a_\xC7V[\x81Rap\xDB` \x84\x01ap{V[` \x82\x01R`@\x83\x015ap\xEE\x81a_\xC7V[`@\x82\x01R``\x83\x015aq\x01\x81acFV[``\x82\x01R`\x80\x83\x015aq\x14\x81acFV[`\x80\x82\x01R`\xA0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15aq>W`\0\x80\xFD[aqFa`3V[\x90P\x815\x81R` \x82\x015aqZ\x81a_\xC7V[` \x82\x01Raqk`@\x83\x01a_\xD9V[`@\x82\x01Raq|``\x83\x01ab=V[``\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15aq\x99W`\0\x80\xFD[aq\xA1a`UV[\x90Paq\xAD\x83\x83aq,V[\x81R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[aq\xD4\x84\x82\x85\x01aa/V[` \x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83aq\x87V[`\0`\x80\x82\x84\x03\x12\x15aq\xFEW`\0\x80\xFD[ar\x06a`3V[\x90P\x815\x81R` \x82\x015` \x82\x01Raqk`@\x83\x01a_\xD9V[`\0`\x80\x82\x84\x03\x12\x15ar4W`\0\x80\xFD[a\"\r\x83\x83aq\xECV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10arvWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15ar\x8EW`\0\x80\xFD[\x81Qa\"\r\x81a^\x08V[`\0`@\x82\x84\x03\x12\x15ar\xABW`\0\x80\xFD[ar\xB3a`UV[ar\xBC\x83a_\xD9V[\x81R` \x83\x015ar\xCC\x81a^\x08V[` \x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15ar\xEAW`\0\x80\xFD[ar\xF2a`UV[\x825ar\xFD\x81a_\xC7V[\x81R` \x83\x015ar\xCC\x81acFV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12as:W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15asTW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a_\x7FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01as\x91Was\x91asiV[P`\x01\x01\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15as\xBAWas\xBAasiV[\x01\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x16\x80as\xDCWas\xDCasiV[`\0\x19\x01\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x81\x03at\x02Wat\x02asiV[`\x01\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80at9Wat9at\x0CV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15at`Wat`asiV[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15at\xA2Wat\xA2asiV[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15at\xCEWat\xCEasiV[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15at\xEAWat\xEAasiV[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15au\0Wau\0asiV[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qau1\x81`\x01\x85\x01` \x87\x01a\\\xE0V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x82\x82\x10\x15auQWauQasiV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15auhW`\0\x80\xFD[aupa`wV[auy\x83ab=V[\x81R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15au\x94W`\0\x80\xFD[au\x9Ca`\x0BV[\x825\x81R` \x83\x015au\xAE\x81a_\xC7V[` \x82\x01Rab\xEF`@\x84\x01a_\xD9V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15au\xD8Wau\xD8a_\xF5V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12au\xF3W`\0\x80\xFD[\x815` av\x08av\x03\x83au\xBFV[a`\xFFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av'W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805av>\x81a_\xC7V[\x83R\x91\x83\x01\x91\x83\x01av+V[`\0\x82`\x1F\x83\x01\x12av\\W`\0\x80\xFD[\x815` avlav\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15av\x8BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805\x83R\x91\x83\x01\x91\x83\x01av\x8FV[`\0`@\x826\x03\x12\x15av\xB8W`\0\x80\xFD[av\xC0a`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15av\xD7W`\0\x80\xFD[\x81\x85\x01\x91P`\x80\x826\x03\x12\x15av\xECW`\0\x80\xFD[av\xF4a`3V[\x825\x81R` \x83\x015\x82\x81\x11\x15aw\nW`\0\x80\xFD[aw\x166\x82\x86\x01au\xE2V[` \x83\x01RP`@\x83\x015\x82\x81\x11\x15aw.W`\0\x80\xFD[aw:6\x82\x86\x01avKV[`@\x83\x01RPawL``\x84\x01ab=V[``\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15awgW`\0\x80\xFD[Paq\xD46\x82\x86\x01aa/V[`\0`\xC0\x82\x84\x03\x12\x15aw\x86W`\0\x80\xFD[aw\x8Ea`\x99V[\x90P\x815\x81R` \x82\x015aw\xA2\x81acFV[` \x82\x01R`@\x82\x015aw\xB5\x81acFV[`@\x82\x01Raw\xC6``\x83\x01ab=V[``\x82\x01Raw\xD7`\x80\x83\x01ab=V[`\x80\x82\x01Raw\xE8`\xA0\x83\x01a_\xD9V[`\xA0\x82\x01R\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15ax\x05W`\0\x80\xFD[ax\ra`UV[\x90Pax\x19\x83\x83awtV[\x81R`\xC0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[`\0a\x10\xF46\x83aw\xF3V[`\0\x82`\x1F\x83\x01\x12axQW`\0\x80\xFD[\x815` axaav\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ax\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805ax\x97\x81acFV[\x83R\x91\x83\x01\x91\x83\x01ax\x84V[`\0`@\x826\x03\x12\x15ax\xB6W`\0\x80\xFD[ax\xBEa`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ax\xD5W`\0\x80\xFD[ax\xE16\x83\x87\x01avKV[\x83R` \x85\x015\x91P\x80\x82\x11\x15ax\xF7W`\0\x80\xFD[Paq\xD46\x82\x86\x01ax@V[`\0`@\x82\x84\x03\x12\x15ay\x16W`\0\x80\xFD[ay\x1Ea`UV[\x90Pay)\x82a_\xD9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ayDW`\0\x80\xFD[aq\xD4\x84\x82\x85\x01ax@V[`\0a\x10\xF46\x83ay\x04V[`\x01\x81\x81\x1C\x90\x82\x16\x80aypW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\\\x18WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a\\\xD8`@\x83\x01\x84a]\x0CV[`\0` \x82\x84\x03\x12\x15ay\xC4W`\0\x80\xFD[PQ\x91\x90PV[`\0``\x82\x84\x03\x12\x15ay\xDDW`\0\x80\xFD[ay\xE5a`\x0BV[\x90P\x815\x81Ray\xF7` \x83\x01a_\xD9V[` \x82\x01RapT`@\x83\x01ab=V[`\0`\xC0\x82\x84\x03\x12\x15az\x1AW`\0\x80\xFD[az\"a`3V[\x90Paz.\x83\x83ay\xCBV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15azJW`\0\x80\xFD[azV\x85\x83\x86\x01aa/V[` \x84\x01R`\x80\x84\x015\x91Pazk\x82acFV[\x81`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15az\x85W`\0\x80\xFD[Paz\x92\x84\x82\x85\x01ax@V[``\x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83az\x08V[`\0a\x01\0\x82\x84\x03\x12\x15az\xBDW`\0\x80\xFD[az\xC5a`\x0BV[\x90Paz\xD1\x83\x83awtV[\x81R`\xC0\x82\x015az\xE1\x81a_\xC7V[` \x82\x01R`\xE0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15az\xFFW`\0\x80\xFD[a{\x0B\x84\x82\x85\x01aa/V[`@\x83\x01RP\x92\x91PPV[`\0a\x10\xF46\x83az\xAAV[`\0``\x82\x84\x03\x12\x15a{5W`\0\x80\xFD[a{=a`\x0BV[\x90P\x815a{J\x81a_\xC7V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a{fW`\0\x80\xFD[a{r\x85\x83\x86\x01aw\xF3V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a{\x8BW`\0\x80\xFD[Pa{\x0B\x84\x82\x85\x01aw\xF3V[`\0a\x10\xF46\x83a{#V[`\0`\xA0\x82\x84\x03\x12\x15a{\xB6W`\0\x80\xFD[a{\xBEa`UV[\x90Paq\xAD\x83\x83aq\xECV[`\0a\x10\xF46\x83a{\xA4V[`\0`@\x82\x84\x03\x12\x15a{\xE8W`\0\x80\xFD[a{\xF0a`UV[\x825a{\xFB\x81a^\x08V[\x81R` \x83\x015ar\xCC\x81a_\xC7V[\x83\x81R\x81\x83` \x83\x017`\0\x91\x01` \x01\x90\x81R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a|7W`\0\x80\xFD[a|?a`UV[\x90Pa|K\x83\x83ap\x1EV[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aq\xC8W`\0\x80\xFD[`\0a\x10\xF46\x83a|%V[`\0\x80\x85\x85\x11\x15a|\x82W`\0\x80\xFD[\x83\x86\x11\x15a|\x8FW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\x80\x82\x84\x03\x12\x15a|\xAEW`\0\x80\xFD[a\"\r\x83\x83aq,V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\\\xD8` \x83\x01\x84\x86a|\xB8V[`@\x81R`\0a}\t`@\x83\x01\x85\x87a|\xB8V[\x90P`\x01`\x01`@\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[`\0` \x80\x83\x85\x03\x12\x15a}5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a}KW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a}\\W`\0\x80\xFD[\x80Qa}jav\x03\x82au\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a}\x89W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a}\xB0W\x83Qa}\xA1\x81a_\xC7V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a}\x8EV[\x97\x96PPPPPPPV[` \x81R`\0a\"\r` \x83\x01\x84ae\x8CV[`\0`\x80\x82\x84\x03\x12\x15a}\xE0W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a~\x02Wa~\x02a_\xF5V[`@R\x825a~\x10\x81a_\xC7V[\x81R` \x83\x015a~ \x81a_\xC7V[` \x82\x01R`@\x83\x015a~3\x81acFV[`@\x82\x01R``\x83\x015a~F\x81acFV[``\x82\x01R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a~dW`\0\x80\xFD[a~la`UV[\x825a~w\x81a^\x08V[\x81Rar\xCC` \x84\x01a_\xD9V[`\0``\x82\x84\x03\x12\x15a~\x97W`\0\x80\xFD[a~\x9Fa`\x0BV[a~\xA8\x83ab=V[\x81R` \x83\x015au\xAE\x81a^\x08V[`\0`\xC0\x82\x84\x03\x12\x15a~\xCAW`\0\x80\xFD[a~\xD2a`\x99V[\x90P\x815\x81R` \x82\x015` \x82\x01R`@\x82\x015a~\xF0\x81a_\xC7V[`@\x82\x01Ra\x7F\x01``\x83\x01ap{V[``\x82\x01R`\x80\x82\x015a\x7F\x14\x81acFV[`\x80\x82\x01Raw\xE8`\xA0\x83\x01ab=V[`\0`\xE0\x82\x84\x03\x12\x15a\x7F7W`\0\x80\xFD[a\x7F?a`UV[\x90Pax\x19\x83\x83a~\xB8V[`\0` \x82\x84\x03\x12\x15a\x7F]W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x7FsW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\x7F%V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\\\x18W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x7F\xB5W`\0\x80\xFD[a\"\r\x83\x83ay\xCBV[`\0a\x10\xF46\x83a\x7F%V[`\0` \x82\x84\x03\x12\x15a\x7F\xDDW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x7F\xFFWa\x7F\xFFa_\xF5V[`@R\x90P\x80a\x80\x0E\x83a_\xD9V[\x90R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x80(W`\0\x80\xFD[a\"\r\x83\x83a\x7F\xCBV[\x805w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a_\xF0W`\0\x80\xFD[`\0`\xE0\x82\x84\x03\x12\x15a\x80lW`\0\x80\xFD[a\x80ta`\xBBV[\x825\x81R` \x83\x015a\x80\x86\x81a_\xC7V[` \x82\x01R`@\x83\x015a\x80\x99\x81a_\xC7V[`@\x82\x01Ra\x80\xAA``\x84\x01a\x802V[``\x82\x01Ra\x80\xBB`\x80\x84\x01a\x802V[`\x80\x82\x01Ra\x80\xCC`\xA0\x84\x01ab=V[`\xA0\x82\x01R`\xC0\x83\x015\x80`\x17\x0B\x81\x14a\x80\xE5W`\0\x80\xFD[`\xC0\x82\x01R\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x81\x03W`\0\x80\xFD[a\x81\x0Ba`\x0BV[\x825a\x81\x16\x81a_\xC7V[\x81Ra\x81$` \x84\x01a_\xD9V[` \x82\x01R`@\x83\x015ab\xEF\x81a^\x08V[`\0`\xC0\x82\x84\x03\x12\x15a\x81IW`\0\x80\xFD[a\"\r\x83\x83a~\xB8V[`\0`@\x826\x03\x12\x15a\x81eW`\0\x80\xFD[a\x81ma`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x81\x84W`\0\x80\xFD[\x81\x85\x01\x91P``\x826\x03\x12\x15a\x81\x99W`\0\x80\xFD[a\x81\xA1a`\x0BV[\x825\x81R` \x83\x015\x82\x81\x11\x15a\x81\xB7W`\0\x80\xFD[a\x81\xC36\x82\x86\x01au\xE2V[` \x83\x01RPa\x81\xD5`@\x84\x01ab=V[`@\x82\x01R\x83R` \x85\x015\x91P\x80\x82\x11\x15awgW`\0\x80\xFD[`\0`@\x826\x03\x12\x15a\x82\x02W`\0\x80\xFD[a\x82\na`UV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x82!W`\0\x80\xFD[a\x82-6\x83\x87\x01avKV[\x83R` \x91P\x81\x85\x015\x81\x81\x11\x15a\x82DW`\0\x80\xFD[\x85\x01\x90P6`\x1F\x82\x01\x12a\x82WW`\0\x80\xFD[\x805a\x82eav\x03\x82au\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x906\x83\x11\x15a\x82\x84W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x82\xA2W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x82\x89V[\x93\x85\x01\x93\x90\x93RP\x91\x94\x93PPPPV[`\0``\x826\x03\x12\x15a\x82\xC5W`\0\x80\xFD[a\x82\xCDa`\x0BV[\x825a\x82\xD8\x81a_\xC7V[\x81R` \x83\x015a\x82\xE8\x81acFV[` \x82\x01R`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x83\x06W`\0\x80\xFD[a{\x0B6\x82\x86\x01avKV[`\0` \x82\x84\x03\x12\x15a\x83$W`\0\x80\xFD[a\x83,a`wV[\x915\x82RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x83GW`\0\x80\xFD[a\x83Oa`\x0BV[\x80``\x84\x01\x85\x81\x11\x15a\x83aW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x83{W\x805\x84R` \x93\x84\x01\x93\x01a\x83cV[P\x90\x95\x94PPPPPV[`\0`\xE0\x826\x03\x12\x15a\x83\x98W`\0\x80\xFD[a\x83\xA0a`\xDDV[a\x83\xAA6\x84a\x836V[\x81R``\x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x83\xC6W`\0\x80\xFD[a\x83\xD26\x83\x87\x01aa/V[` \x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15a\x83\xEBW`\0\x80\xFD[a\x83\xF76\x83\x87\x01avKV[`@\x84\x01R`\xA0\x85\x015\x91P\x80\x82\x11\x15a\x84\x10W`\0\x80\xFD[Pa\x84\x1D6\x82\x86\x01avKV[``\x83\x01RP`\xC0\x92\x90\x92\x015`\x80\x83\x01RP\x90V[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x03\x82\x13\x81\x15\x16\x15a\x84fWa\x84fasiV[\x82o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x03\x82\x12\x81\x16\x15a\x84\x8BWa\x84\x8BasiV[P\x01\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x84\xA5W`\0\x80\xFD[\x815` a\x84\xB5av\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x84\xD4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15am+W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x84\xF7W`\0\x80\x81\xFD[a\x85\x05\x89\x86\x83\x8B\x01\x01aa/V[\x84RP\x91\x83\x01\x91\x83\x01a\x84\xD8V[`\0``\x826\x03\x12\x15a\x85%W`\0\x80\xFD[a\x85-a`\x0BV[\x825a\x858\x81acFV[\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x85TW`\0\x80\xFD[a\x85`6\x83\x87\x01a\x84\x94V[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15a\x85yW`\0\x80\xFD[Pa{\x0B6\x82\x86\x01a\x84\x94V[`\0`@\x82\x84\x03\x12\x15a\x85\x98W`\0\x80\xFD[a\x85\xA0a`UV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x85\xB8W`\0\x80\xFD[a\x85\xC4\x84\x82\x85\x01a{#V[\x82RP` \x82\x015a\x85\xD5\x81acFV[` \x82\x01R\x92\x91PPV[`\0a\x10\xF46\x83a\x85\x86V[`\0``\x826\x03\x12\x15a\x85\xFEW`\0\x80\xFD[a\x86\x06a`\x0BV[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x86\x1DW`\0\x80\xFD[\x90\x84\x01\x906`\x1F\x83\x01\x12a\x860W`\0\x80\xFD[\x815` a\x86@av\x03\x83au\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x906\x84\x11\x15a\x86_W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x86\x97W\x805\x86\x81\x11\x15a\x86{W`\0\x80\x81\xFD[a\x86\x896\x86\x83\x8B\x01\x01aa/V[\x84RP\x91\x83\x01\x91\x83\x01a\x86cV[P\x86RP\x86\x81\x015\x93P\x82\x84\x11\x15a\x86\xAEW`\0\x80\xFD[a\x86\xBA6\x85\x89\x01avKV[\x90\x85\x01RPPP`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x86\xE4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x86\xFAW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01aq\x87V[`\0`\xE0\x82\x84\x03\x12\x15a\x87\x18W`\0\x80\xFD[a\x87 a`\xBBV[\x82Qa\x87+\x81a^\x08V[\x81R` \x83\x01Qa\x87;\x81acFV[` \x82\x01R`@\x83\x01Qa\x87N\x81acFV[`@\x82\x01R``\x83\x01Qa\x87a\x81acFV[``\x82\x01R`\x80\x83\x01Qa\x87t\x81acFV[`\x80\x82\x01R`\xA0\x83\x01Qa\x87\x87\x81acFV[`\xA0\x82\x01R`\xC0\x83\x01Qa\x80\xE5\x81acFV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x87\xBAWa\x87\xBAasiV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x87\xD4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x87\xEAW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01ay\x04V[`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a\\\xD8`@\x83\x01\x84ae\x8CV[`\0\x80`@\x83\x85\x03\x12\x15a\x88+W`\0\x80\xFD[\x82Qa\x886\x81a_\xC7V[` \x84\x01Q\x90\x92Pac\x83\x81acFV[`\0` \x82\x84\x03\x12\x15a\x88YW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88oW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a{#V[` \x81R`\0\x82Q`\x80` \x84\x01Ra\x88\x97`\xA0\x84\x01\x82ag\xEBV[\x90P` \x84\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`@\x86\x01R\x80`@\x87\x01Q\x16``\x86\x01RPP``\x84\x01Q`\x0F\x0B`\x80\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x88\xE5W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x88\xFBW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a\x85\x86V[`\0` \x82\x84\x03\x12\x15a\x89\x19W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x89/W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01az\x08V[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R\x80`\0 `\0[\x83\x81\x10\x15ac\xF8W\x81T`\x01`\x01`@\x1B\x03\x16\x87R`\x01\x80\x83\x01T\x84\x89\x01R`\x02\x83\x01T`\x01`\x01`\xA0\x1B\x03\x16`@\x89\x01R`\x03\x83\x01T`\x01`\x01`\x80\x1B\x03\x16``\x89\x01R`\x80\x90\x97\x01\x96`\x04\x90\x92\x01\x91\x01a\x89TV[\x84Q\x81R` \x80\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90\x82\x01R`@\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R\x83`\x0F\x0B``\x82\x01R`\xC0`\x80\x82\x01R`\0a\x89\xF4`\xC0\x83\x01\x85a\x89;V[\x82\x81\x03`\xA0\x84\x01Ra}\xB0\x81\x85ae\x8CV[`\0` \x82\x84\x03\x12\x15a\x8A\x18W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A.W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a|%V[`\0` \x82\x84\x03\x12\x15a\x8ALW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8AbW`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01a{\xA4V[`\0` \x82\x84\x03\x12\x15a\x8A\x80W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x8A\x96W`\0\x80\xFD[a\\\xD8\x84\x82\x85\x01az\xAAV[`@\x81R`\0a\x8A\xB5`@\x83\x01\x85ag\x1AV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\0\x82a\x8A\xDCWa\x8A\xDCat\x0CV[P\x04\x90V[`\0\x82\x19\x82\x11\x15a\x8A\xF4Wa\x8A\xF4asiV[P\x01\x90V[\x7FInvalid nonce: expected: \0\0\0\0\0\0\0\x81R`\0\x82Qa\x8B1\x81`\x19\x85\x01` \x87\x01a\\\xE0V[\x91\x90\x91\x01`\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a\x8BdWa\x8BdasiV[`\0\x03\x92\x91PPV\xFEno slow mode transactions remaining\xA2dipfsX\"\x12 ,\xFB}\xA5o\x8D\x17\xA7\xB5X\x17^\x97\xC8\xEF\x97\xC6\xE8\xBD\x10\x8EG&\xE7z\x08\xCD\xECHd.\x8EdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static ENDPOINT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Endpoint<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Endpoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Endpoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Endpoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Endpoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Endpoint))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Endpoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ENDPOINT_ABI.clone(),
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
                ENDPOINT_ABI.clone(),
                ENDPOINT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addNlpPool` (0xba8d8181) function
        pub fn add_nlp_pool(
            &self,
            p: AddNlpPool,
        ) -> ::ethers::contract::builders::ContractCall<M, AddNlpPool> {
            self.0
                .method_hash([186, 141, 129, 129], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertCode` (0xf968c7f4) function
        pub fn assert_code(
            &self,
            p: AssertCode,
        ) -> ::ethers::contract::builders::ContractCall<M, AssertCode> {
            self.0
                .method_hash([249, 104, 199, 244], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assertProduct` (0x0bb9c3a2) function
        pub fn assert_product(
            &self,
            p: AssertProduct,
        ) -> ::ethers::contract::builders::ContractCall<M, AssertProduct> {
            self.0
                .method_hash([11, 185, 195, 162], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainlinkFullReport` (0xdb5a5021) function
        pub fn chainlink_full_report(
            &self,
            p: ChainlinkFullReport,
        ) -> ::ethers::contract::builders::ContractCall<M, ChainlinkFullReport> {
            self.0
                .method_hash([219, 90, 80, 33], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainlinkReportBlob` (0x96c47c6f) function
        pub fn chainlink_report_blob(
            &self,
            p: ChainlinkReportBlob,
        ) -> ::ethers::contract::builders::ContractCall<M, ChainlinkReportBlob> {
            self.0
                .method_hash([150, 196, 124, 111], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSlowModeTxInner` (0xb70eb263) function
        pub fn check_slow_mode_tx_inner(
            &self,
            sender: ::ethers::core::types::Address,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([183, 14, 178, 99], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSlowModeTxLinkSigner` (0x5bb4c126) function
        pub fn check_slow_mode_tx_link_signer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 180, 193, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearinghouse` (0x5d4f5f97) function
        pub fn clearinghouse(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 79, 95, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeIsolatedSubaccount` (0xdb3aa846) function
        pub fn close_isolated_subaccount(
            &self,
            p: CloseIsolatedSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, CloseIsolatedSubaccount> {
            self.0
                .method_hash([219, 58, 168, 70], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createIsolatedSubaccount` (0x690349cf) function
        pub fn create_isolated_subaccount(
            &self,
            p: CreateIsolatedSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, CreateIsolatedSubaccount> {
            self.0
                .method_hash([105, 3, 73, 207], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteNlpPool` (0x338a7e56) function
        pub fn delete_nlp_pool(
            &self,
            p: DeleteNlpPool,
        ) -> ::ethers::contract::builders::ContractCall<M, DeleteNlpPool> {
            self.0
                .method_hash([51, 138, 126, 86], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateral` (0x8e5d588c) function
        pub fn deposit_collateral(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 93, 88, 140], (subaccount_name, product_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositCollateralWithReferral` (0x221f0939) function
        pub fn deposit_collateral_with_referral(
            &self,
            subaccount: [u8; 32],
            product_id: u32,
            amount: u128,
            p3: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 31, 9, 57], (subaccount, product_id, amount, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSlowModeTransaction` (0x65dd1366) function
        pub fn execute_slow_mode_transaction(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 221, 19, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealthCheckFee` (0xd4de8d9d) function
        pub fn get_health_check_fee(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([212, 222, 141, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLinkedSigner` (0x91c1e3d7) function
        pub fn get_linked_signer(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([145, 193, 227, 215], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidationFee` (0xfbf41984) function
        pub fn get_liquidation_fee(&self) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([251, 244, 25, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNlpPools` (0x1c886d0b) function
        pub fn get_nlp_pools(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<NlpPool>> {
            self.0
                .method_hash([28, 136, 109, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNumSubaccounts` (0xc4f9b25f) function
        pub fn get_num_subaccounts(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([196, 249, 178, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOffchainExchange` (0x8f4f8ecc) function
        pub fn get_offchain_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([143, 79, 142, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPriceX18` (0x368e4686) function
        pub fn get_price_x18(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([54, 142, 70, 134], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencer` (0x4d96a90a) function
        pub fn get_sequencer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([77, 150, 169, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencerFee` (0x4fcfae58) function
        pub fn get_sequencer_fee(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([79, 207, 174, 88], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlots` (0xfab2c469) function
        pub fn get_slots(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 178, 196, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSlowModeTx` (0xee525526) function
        pub fn get_slow_mode_tx(
            &self,
            idx: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (SlowModeTx, u64, u64)> {
            self.0
                .method_hash([238, 82, 85, 38], idx)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountById` (0xef64ed0e) function
        pub fn get_subaccount_by_id(
            &self,
            subaccount_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([239, 100, 237, 14], subaccount_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSubaccountId` (0x22d4a82d) function
        pub fn get_subaccount_id(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([34, 212, 168, 45], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTakerSequencerFee` (0xc510359f) function
        pub fn get_taker_sequencer_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([197, 16, 53, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTime` (0x557ed1ba) function
        pub fn get_time(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([85, 126, 209, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimes` (0xe9ab77e5) function
        pub fn get_times(&self) -> ::ethers::contract::builders::ContractCall<M, Times> {
            self.0
                .method_hash([233, 171, 119, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementSubmissions` (0x22006046) function
        pub fn increment_submissions(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([34, 0, 96, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1459457a) function
        pub fn initialize(
            &self,
            sanctions: ::ethers::core::types::Address,
            sequencer: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            clearinghouse: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 89, 69, 122],
                    (
                        sanctions,
                        sequencer,
                        offchain_exchange,
                        clearinghouse,
                        verifier,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidationStart` (0x8d0acc9b) function
        pub fn liquidation_start(
            &self,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 10, 204, 155], transaction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manualAssert` (0xe7c80751) function
        pub fn manual_assert(
            &self,
            p: ManualAssert,
        ) -> ::ethers::contract::builders::ContractCall<M, ManualAssert> {
            self.0
                .method_hash([231, 200, 7, 81], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrders` (0x69fddcec) function
        pub fn match_orders(
            &self,
            p: MatchOrders,
        ) -> ::ethers::contract::builders::ContractCall<M, MatchOrders> {
            self.0
                .method_hash([105, 253, 220, 236], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrdersWithAmount` (0xf8089d9c) function
        pub fn match_orders_with_amount(
            &self,
            p: MatchOrdersWithAmount,
        ) -> ::ethers::contract::builders::ContractCall<M, MatchOrdersWithAmount> {
            self.0
                .method_hash([248, 8, 157, 156], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nSubmissions` (0x18ed16eb) function
        pub fn n_submissions(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([24, 237, 22, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nlpPools` (0xfe00842c) function
        pub fn nlp_pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, [u8; 32], ::ethers::core::types::Address, u128),
        > {
            self.0
                .method_hash([254, 0, 132, 44], p0)
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
        ///Calls the contract's `perpTick` (0x5a00923b) function
        pub fn perp_tick(
            &self,
            p: PerpTick,
        ) -> ::ethers::contract::builders::ContractCall<M, PerpTick> {
            self.0
                .method_hash([90, 0, 146, 59], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processSlowModeTransaction` (0x87324338) function
        pub fn process_slow_mode_transaction(
            &self,
            sender: ::ethers::core::types::Address,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 50, 67, 56], (sender, transaction))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceXWithdraw` (0x9a08e535) function
        pub fn rebalance_x_withdraw(
            &self,
            p: RebalanceXWithdraw,
        ) -> ::ethers::contract::builders::ContractCall<M, RebalanceXWithdraw> {
            self.0
                .method_hash([154, 8, 229, 53], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebate` (0x42c74d1d) function
        pub fn rebate(&self, p: Rebate) -> ::ethers::contract::builders::ContractCall<M, Rebate> {
            self.0
                .method_hash([66, 199, 77, 29], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `referralCodes` (0x9534dd3e) function
        pub fn referral_codes(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 52, 221, 62], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resyncSlowModeTxs` (0x21047589) function
        pub fn resync_slow_mode_txs(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 4, 117, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInitialPrice` (0x396502b6) function
        pub fn set_initial_price(
            &self,
            product_id: u32,
            initial_price_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 101, 2, 182], (product_id, initial_price_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPriceX18` (0x8c58e10a) function
        pub fn set_price_x18(
            &self,
            product_id: u32,
            price_x18: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 88, 225, 10], (product_id, price_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSlowModeConfig` (0x3216c062) function
        pub fn set_slow_mode_config(
            &self,
            slow_mode_config: SlowModeConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 22, 192, 98], (slow_mode_config,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSlowModeTx` (0x98cd32fe) function
        pub fn set_slow_mode_tx(
            &self,
            idx: u64,
            txn: SlowModeTx,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([152, 205, 50, 254], (idx, txn))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settlePnl` (0xb2bb6367) function
        pub fn settle_pnl(
            &self,
            p: SettlePnl,
        ) -> ::ethers::contract::builders::ContractCall<M, SettlePnl> {
            self.0
                .method_hash([178, 187, 99, 103], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedBurnNlp` (0xf8531a64) function
        pub fn signed_burn_nlp(
            &self,
            p: SignedBurnNlp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedBurnNlp> {
            self.0
                .method_hash([248, 83, 26, 100], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedCancellation` (0x3edf2c5b) function
        pub fn signed_cancellation(
            &self,
            p: SignedCancellation,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedCancellation> {
            self.0
                .method_hash([62, 223, 44, 91], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedCancellationProducts` (0xa082c5aa) function
        pub fn signed_cancellation_products(
            &self,
            p: SignedCancellationProducts,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedCancellationProducts> {
            self.0
                .method_hash([160, 130, 197, 170], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedLinkSigner` (0x85c83e9d) function
        pub fn signed_link_signer(
            &self,
            p: SignedLinkSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedLinkSigner> {
            self.0
                .method_hash([133, 200, 62, 157], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedLiquidateSubaccount` (0x9171d08b) function
        pub fn signed_liquidate_subaccount(
            &self,
            p: SignedLiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedLiquidateSubaccount> {
            self.0
                .method_hash([145, 113, 208, 139], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedMintNlp` (0x5c5b34ef) function
        pub fn signed_mint_nlp(
            &self,
            p: SignedMintNlp,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedMintNlp> {
            self.0
                .method_hash([92, 91, 52, 239], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedOrder` (0x41a09bb6) function
        pub fn signed_order(
            &self,
            p: SignedOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedOrder> {
            self.0
                .method_hash([65, 160, 155, 182], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedTransferQuote` (0x6f3b0a72) function
        pub fn signed_transfer_quote(
            &self,
            p: SignedTransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedTransferQuote> {
            self.0
                .method_hash([111, 59, 10, 114], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `signedWithdrawCollateral` (0x0d55e26b) function
        pub fn signed_withdraw_collateral(
            &self,
            p: SignedWithdrawCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, SignedWithdrawCollateral> {
            self.0
                .method_hash([13, 85, 226, 107], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spotTick` (0xd9768695) function
        pub fn spot_tick(
            &self,
            p: SpotTick,
        ) -> ::ethers::contract::builders::ContractCall<M, SpotTick> {
            self.0
                .method_hash([217, 118, 134, 149], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSlowModeTransaction` (0xe604ed9e) function
        pub fn submit_slow_mode_transaction(
            &self,
            transaction: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 4, 237, 158], transaction)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactions` (0x1f186b27) function
        pub fn submit_transactions(
            &self,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 24, 107, 39], transactions)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactionsChecked` (0x7db6a25b) function
        pub fn submit_transactions_checked(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
            e: [u8; 32],
            s: [u8; 32],
            signer_bitmask: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [125, 182, 162, 91],
                    (idx, transactions, e, s, signer_bitmask),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactionsCheckedWithGasLimit` (0x2f9a2744) function
        pub fn submit_transactions_checked_with_gas_limit(
            &self,
            idx: u64,
            transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
            gas_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 154, 39, 68], (idx, transactions, gas_limit))
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
        ///Calls the contract's `transferQuote` (0x1d97d22f) function
        pub fn transfer_quote(
            &self,
            p: TransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, TransferQuote> {
            self.0
                .method_hash([29, 151, 210, 47], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedBurnNlp` (0x9f9a35e1) function
        pub fn unsigned_burn_nlp(
            &self,
            p: BurnNlp,
        ) -> ::ethers::contract::builders::ContractCall<M, BurnNlp> {
            self.0
                .method_hash([159, 154, 53, 225], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDelistProduct` (0xb3147d17) function
        pub fn unsigned_delist_product(
            &self,
            p: DelistProduct,
        ) -> ::ethers::contract::builders::ContractCall<M, DelistProduct> {
            self.0
                .method_hash([179, 20, 125, 23], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDepositCollateral` (0x3cec4b93) function
        pub fn unsigned_deposit_collateral(
            &self,
            p: DepositCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositCollateral> {
            self.0
                .method_hash([60, 236, 75, 147], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedDepositInsurance` (0x94faefe5) function
        pub fn unsigned_deposit_insurance(
            &self,
            p: DepositInsurance,
        ) -> ::ethers::contract::builders::ContractCall<M, DepositInsurance> {
            self.0
                .method_hash([148, 250, 239, 229], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedLinkSigner` (0x05e42dc7) function
        pub fn unsigned_link_signer(
            &self,
            p: LinkSigner,
        ) -> ::ethers::contract::builders::ContractCall<M, LinkSigner> {
            self.0
                .method_hash([5, 228, 45, 199], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedLiquidateSubaccount` (0x9e851424) function
        pub fn unsigned_liquidate_subaccount(
            &self,
            p: LiquidateSubaccount,
        ) -> ::ethers::contract::builders::ContractCall<M, LiquidateSubaccount> {
            self.0
                .method_hash([158, 133, 20, 36], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedMintNlp` (0x8f393838) function
        pub fn unsigned_mint_nlp(
            &self,
            p: MintNlp,
        ) -> ::ethers::contract::builders::ContractCall<M, MintNlp> {
            self.0
                .method_hash([143, 57, 56, 56], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedTransferQuote` (0x0edaacce) function
        pub fn unsigned_transfer_quote(
            &self,
            p: TransferQuote,
        ) -> ::ethers::contract::builders::ContractCall<M, TransferQuote> {
            self.0
                .method_hash([14, 218, 172, 206], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedUpdateTierFeeRates` (0xfe72d8b7) function
        pub fn unsigned_update_tier_fee_rates(
            &self,
            p: UpdateTierFeeRates,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateTierFeeRates> {
            self.0
                .method_hash([254, 114, 216, 183], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedWithdrawCollateral` (0xbc85ca86) function
        pub fn unsigned_withdraw_collateral(
            &self,
            p: WithdrawCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, WithdrawCollateral> {
            self.0
                .method_hash([188, 133, 202, 134], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsignedWithdrawInsurance` (0x14735755) function
        pub fn unsigned_withdraw_insurance(
            &self,
            p: WithdrawInsurance,
        ) -> ::ethers::contract::builders::ContractCall<M, WithdrawInsurance> {
            self.0
                .method_hash([20, 115, 87, 85], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFeeTier` (0x70f6821c) function
        pub fn update_fee_tier(
            &self,
            p: UpdateFeeTier,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateFeeTier> {
            self.0
                .method_hash([112, 246, 130, 28], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateNlpPool` (0x98c5b549) function
        pub fn update_nlp_pool(
            &self,
            p: UpdateNlpPool,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdateNlpPool> {
            self.0
                .method_hash([152, 197, 181, 73], (p,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePrice` (0x1d9eeda5) function
        pub fn update_price(
            &self,
            p: UpdatePrice,
        ) -> ::ethers::contract::builders::ContractCall<M, UpdatePrice> {
            self.0
                .method_hash([29, 158, 237, 165], (p,))
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `SubmitTransactions` event
        pub fn submit_transactions_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SubmitTransactionsFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EndpointEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Endpoint<M> {
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
    #[ethevent(name = "SubmitTransactions", abi = "SubmitTransactions()")]
    pub struct SubmitTransactionsFilter;
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
    pub enum EndpointEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PriceQueryFilter(PriceQueryFilter),
        SubmitTransactionsFilter(SubmitTransactionsFilter),
    }
    impl ::ethers::contract::EthLogDecode for EndpointEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(EndpointEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(EndpointEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PriceQueryFilter::decode_log(log) {
                return Ok(EndpointEvents::PriceQueryFilter(decoded));
            }
            if let Ok(decoded) = SubmitTransactionsFilter::decode_log(log) {
                return Ok(EndpointEvents::SubmitTransactionsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EndpointEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceQueryFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for EndpointEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for EndpointEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PriceQueryFilter> for EndpointEvents {
        fn from(value: PriceQueryFilter) -> Self {
            Self::PriceQueryFilter(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsFilter> for EndpointEvents {
        fn from(value: SubmitTransactionsFilter) -> Self {
            Self::SubmitTransactionsFilter(value)
        }
    }
    ///Container type for all input parameters for the `addNlpPool` function with signature `addNlpPool((address,uint128))` and selector `0xba8d8181`
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
    #[ethcall(name = "addNlpPool", abi = "addNlpPool((address,uint128))")]
    pub struct AddNlpPoolCall {
        pub p: AddNlpPool,
    }
    ///Container type for all input parameters for the `assertCode` function with signature `assertCode((string[],bytes32[],uint256))` and selector `0xf968c7f4`
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
    #[ethcall(name = "assertCode", abi = "assertCode((string[],bytes32[],uint256))")]
    pub struct AssertCodeCall {
        pub p: AssertCode,
    }
    ///Container type for all input parameters for the `assertProduct` function with signature `assertProduct((uint32,bool,uint32,int128,int128,bytes32))` and selector `0x0bb9c3a2`
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
        name = "assertProduct",
        abi = "assertProduct((uint32,bool,uint32,int128,int128,bytes32))"
    )]
    pub struct AssertProductCall {
        pub p: AssertProduct,
    }
    ///Container type for all input parameters for the `chainlinkFullReport` function with signature `chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))` and selector `0xdb5a5021`
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
        name = "chainlinkFullReport",
        abi = "chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))"
    )]
    pub struct ChainlinkFullReportCall {
        pub p: ChainlinkFullReport,
    }
    ///Container type for all input parameters for the `chainlinkReportBlob` function with signature `chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))` and selector `0x96c47c6f`
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
        name = "chainlinkReportBlob",
        abi = "chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))"
    )]
    pub struct ChainlinkReportBlobCall {
        pub p: ChainlinkReportBlob,
    }
    ///Container type for all input parameters for the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `0xb70eb263`
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
        name = "checkSlowModeTxInner",
        abi = "checkSlowModeTxInner(address,bytes)"
    )]
    pub struct CheckSlowModeTxInnerCall {
        pub sender: ::ethers::core::types::Address,
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `0x5bb4c126`
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
        name = "checkSlowModeTxLinkSigner",
        abi = "checkSlowModeTxLinkSigner()"
    )]
    pub struct CheckSlowModeTxLinkSignerCall;
    ///Container type for all input parameters for the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
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
    #[ethcall(name = "clearinghouse", abi = "clearinghouse()")]
    pub struct ClearinghouseCall;
    ///Container type for all input parameters for the `closeIsolatedSubaccount` function with signature `closeIsolatedSubaccount((bytes32))` and selector `0xdb3aa846`
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
        name = "closeIsolatedSubaccount",
        abi = "closeIsolatedSubaccount((bytes32))"
    )]
    pub struct CloseIsolatedSubaccountCall {
        pub p: CloseIsolatedSubaccount,
    }
    ///Container type for all input parameters for the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes))` and selector `0x690349cf`
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
        abi = "createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes))"
    )]
    pub struct CreateIsolatedSubaccountCall {
        pub p: CreateIsolatedSubaccount,
    }
    ///Container type for all input parameters for the `deleteNlpPool` function with signature `deleteNlpPool((uint64))` and selector `0x338a7e56`
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
    #[ethcall(name = "deleteNlpPool", abi = "deleteNlpPool((uint64))")]
    pub struct DeleteNlpPoolCall {
        pub p: DeleteNlpPool,
    }
    ///Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral(bytes12,uint32,uint128)` and selector `0x8e5d588c`
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
        name = "depositCollateral",
        abi = "depositCollateral(bytes12,uint32,uint128)"
    )]
    pub struct DepositCollateralCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///Container type for all input parameters for the `depositCollateralWithReferral` function with signature `depositCollateralWithReferral(bytes32,uint32,uint128,string)` and selector `0x221f0939`
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
        name = "depositCollateralWithReferral",
        abi = "depositCollateralWithReferral(bytes32,uint32,uint128,string)"
    )]
    pub struct DepositCollateralWithReferralCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub p3: ::std::string::String,
    }
    ///Container type for all input parameters for the `executeSlowModeTransaction` function with signature `executeSlowModeTransaction()` and selector `0x65dd1366`
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
        name = "executeSlowModeTransaction",
        abi = "executeSlowModeTransaction()"
    )]
    pub struct ExecuteSlowModeTransactionCall;
    ///Container type for all input parameters for the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `0xd4de8d9d`
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
    #[ethcall(name = "getHealthCheckFee", abi = "getHealthCheckFee()")]
    pub struct GetHealthCheckFeeCall;
    ///Container type for all input parameters for the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `0x91c1e3d7`
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
    #[ethcall(name = "getLinkedSigner", abi = "getLinkedSigner(bytes32)")]
    pub struct GetLinkedSignerCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `0xfbf41984`
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
    #[ethcall(name = "getLiquidationFee", abi = "getLiquidationFee()")]
    pub struct GetLiquidationFeeCall;
    ///Container type for all input parameters for the `getNlpPools` function with signature `getNlpPools()` and selector `0x1c886d0b`
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
    #[ethcall(name = "getNlpPools", abi = "getNlpPools()")]
    pub struct GetNlpPoolsCall;
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `0xc4f9b25f`
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
    #[ethcall(name = "getNumSubaccounts", abi = "getNumSubaccounts()")]
    pub struct GetNumSubaccountsCall;
    ///Container type for all input parameters for the `getOffchainExchange` function with signature `getOffchainExchange()` and selector `0x8f4f8ecc`
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
    #[ethcall(name = "getOffchainExchange", abi = "getOffchainExchange()")]
    pub struct GetOffchainExchangeCall;
    ///Container type for all input parameters for the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `0x368e4686`
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
    #[ethcall(name = "getPriceX18", abi = "getPriceX18(uint32)")]
    pub struct GetPriceX18Call {
        pub product_id: u32,
    }
    ///Container type for all input parameters for the `getSequencer` function with signature `getSequencer()` and selector `0x4d96a90a`
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
    #[ethcall(name = "getSequencer", abi = "getSequencer()")]
    pub struct GetSequencerCall;
    ///Container type for all input parameters for the `getSequencerFee` function with signature `getSequencerFee(uint32)` and selector `0x4fcfae58`
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
    #[ethcall(name = "getSequencerFee", abi = "getSequencerFee(uint32)")]
    pub struct GetSequencerFeeCall {
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
    ///Container type for all input parameters for the `getSlowModeTx` function with signature `getSlowModeTx(uint64)` and selector `0xee525526`
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
    #[ethcall(name = "getSlowModeTx", abi = "getSlowModeTx(uint64)")]
    pub struct GetSlowModeTxCall {
        pub idx: u64,
    }
    ///Container type for all input parameters for the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `0xef64ed0e`
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
    #[ethcall(name = "getSubaccountById", abi = "getSubaccountById(uint64)")]
    pub struct GetSubaccountByIdCall {
        pub subaccount_id: u64,
    }
    ///Container type for all input parameters for the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `0x22d4a82d`
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
    #[ethcall(name = "getSubaccountId", abi = "getSubaccountId(bytes32)")]
    pub struct GetSubaccountIdCall {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `0xc510359f`
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
    #[ethcall(name = "getTakerSequencerFee", abi = "getTakerSequencerFee()")]
    pub struct GetTakerSequencerFeeCall;
    ///Container type for all input parameters for the `getTime` function with signature `getTime()` and selector `0x557ed1ba`
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
    #[ethcall(name = "getTime", abi = "getTime()")]
    pub struct GetTimeCall;
    ///Container type for all input parameters for the `getTimes` function with signature `getTimes()` and selector `0xe9ab77e5`
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
    #[ethcall(name = "getTimes", abi = "getTimes()")]
    pub struct GetTimesCall;
    ///Container type for all input parameters for the `incrementSubmissions` function with signature `incrementSubmissions()` and selector `0x22006046`
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
    #[ethcall(name = "incrementSubmissions", abi = "incrementSubmissions()")]
    pub struct IncrementSubmissionsCall;
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
        pub sanctions: ::ethers::core::types::Address,
        pub sequencer: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub verifier: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidationStart` function with signature `liquidationStart(bytes)` and selector `0x8d0acc9b`
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
    #[ethcall(name = "liquidationStart", abi = "liquidationStart(bytes)")]
    pub struct LiquidationStartCall {
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `manualAssert` function with signature `manualAssert((int128,bytes[],bytes[]))` and selector `0xe7c80751`
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
    #[ethcall(name = "manualAssert", abi = "manualAssert((int128,bytes[],bytes[]))")]
    pub struct ManualAssertCall {
        pub p: ManualAssert,
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)))` and selector `0x69fddcec`
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
        abi = "matchOrders((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)))"
    )]
    pub struct MatchOrdersCall {
        pub p: MatchOrders,
    }
    ///Container type for all input parameters for the `matchOrdersWithAmount` function with signature `matchOrdersWithAmount(((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),int128))` and selector `0xf8089d9c`
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
        name = "matchOrdersWithAmount",
        abi = "matchOrdersWithAmount(((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),int128))"
    )]
    pub struct MatchOrdersWithAmountCall {
        pub p: MatchOrdersWithAmount,
    }
    ///Container type for all input parameters for the `nSubmissions` function with signature `nSubmissions()` and selector `0x18ed16eb`
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
    #[ethcall(name = "nSubmissions", abi = "nSubmissions()")]
    pub struct NsubmissionsCall;
    ///Container type for all input parameters for the `nlpPools` function with signature `nlpPools(uint256)` and selector `0xfe00842c`
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
    #[ethcall(name = "nlpPools", abi = "nlpPools(uint256)")]
    pub struct NlpPoolsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `0x5a00923b`
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
    #[ethcall(name = "perpTick", abi = "perpTick((uint128,int128[]))")]
    pub struct PerpTickCall {
        pub p: PerpTick,
    }
    ///Container type for all input parameters for the `processSlowModeTransaction` function with signature `processSlowModeTransaction(address,bytes)` and selector `0x87324338`
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
        name = "processSlowModeTransaction",
        abi = "processSlowModeTransaction(address,bytes)"
    )]
    pub struct ProcessSlowModeTransactionCall {
        pub sender: ::ethers::core::types::Address,
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw((uint32,uint128,address))` and selector `0x9a08e535`
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
        name = "rebalanceXWithdraw",
        abi = "rebalanceXWithdraw((uint32,uint128,address))"
    )]
    pub struct RebalanceXWithdrawCall {
        pub p: RebalanceXWithdraw,
    }
    ///Container type for all input parameters for the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `0x42c74d1d`
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
    #[ethcall(name = "rebate", abi = "rebate((bytes32[],int128[]))")]
    pub struct RebateCall {
        pub p: Rebate,
    }
    ///Container type for all input parameters for the `referralCodes` function with signature `referralCodes(address)` and selector `0x9534dd3e`
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
    #[ethcall(name = "referralCodes", abi = "referralCodes(address)")]
    pub struct ReferralCodesCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `resyncSlowModeTxs` function with signature `resyncSlowModeTxs()` and selector `0x21047589`
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
    #[ethcall(name = "resyncSlowModeTxs", abi = "resyncSlowModeTxs()")]
    pub struct ResyncSlowModeTxsCall;
    ///Container type for all input parameters for the `setInitialPrice` function with signature `setInitialPrice(uint32,int128)` and selector `0x396502b6`
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
    #[ethcall(name = "setInitialPrice", abi = "setInitialPrice(uint32,int128)")]
    pub struct SetInitialPriceCall {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub initial_price_x18: i128,
    }
    ///Container type for all input parameters for the `setPriceX18` function with signature `setPriceX18(uint32,int128)` and selector `0x8c58e10a`
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
    #[ethcall(name = "setPriceX18", abi = "setPriceX18(uint32,int128)")]
    pub struct SetPriceX18Call {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all input parameters for the `setSlowModeConfig` function with signature `setSlowModeConfig((uint64,uint64,uint64))` and selector `0x3216c062`
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
        name = "setSlowModeConfig",
        abi = "setSlowModeConfig((uint64,uint64,uint64))"
    )]
    pub struct SetSlowModeConfigCall {
        pub slow_mode_config: SlowModeConfig,
    }
    ///Container type for all input parameters for the `setSlowModeTx` function with signature `setSlowModeTx(uint64,(uint64,address,bytes))` and selector `0x98cd32fe`
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
        name = "setSlowModeTx",
        abi = "setSlowModeTx(uint64,(uint64,address,bytes))"
    )]
    pub struct SetSlowModeTxCall {
        pub idx: u64,
        pub txn: SlowModeTx,
    }
    ///Container type for all input parameters for the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `0xb2bb6367`
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
    #[ethcall(name = "settlePnl", abi = "settlePnl((bytes32[],uint256[]))")]
    pub struct SettlePnlCall {
        pub p: SettlePnl,
    }
    ///Container type for all input parameters for the `signedBurnNlp` function with signature `signedBurnNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))` and selector `0xf8531a64`
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
        name = "signedBurnNlp",
        abi = "signedBurnNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))"
    )]
    pub struct SignedBurnNlpCall {
        pub p: SignedBurnNlp,
    }
    ///Container type for all input parameters for the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `0x3edf2c5b`
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
        name = "signedCancellation",
        abi = "signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))"
    )]
    pub struct SignedCancellationCall {
        pub p: SignedCancellation,
    }
    ///Container type for all input parameters for the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `0xa082c5aa`
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
        name = "signedCancellationProducts",
        abi = "signedCancellationProducts(((bytes32,uint32[],uint64),bytes))"
    )]
    pub struct SignedCancellationProductsCall {
        pub p: SignedCancellationProducts,
    }
    ///Container type for all input parameters for the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `0x85c83e9d`
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
        name = "signedLinkSigner",
        abi = "signedLinkSigner(((bytes32,bytes32,uint64),bytes))"
    )]
    pub struct SignedLinkSignerCall {
        pub p: SignedLinkSigner,
    }
    ///Container type for all input parameters for the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))` and selector `0x9171d08b`
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
        name = "signedLiquidateSubaccount",
        abi = "signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))"
    )]
    pub struct SignedLiquidateSubaccountCall {
        pub p: SignedLiquidateSubaccount,
    }
    ///Container type for all input parameters for the `signedMintNlp` function with signature `signedMintNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))` and selector `0x5c5b34ef`
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
        name = "signedMintNlp",
        abi = "signedMintNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))"
    )]
    pub struct SignedMintNlpCall {
        pub p: SignedMintNlp,
    }
    ///Container type for all input parameters for the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64,uint128),bytes))` and selector `0x41a09bb6`
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
        name = "signedOrder",
        abi = "signedOrder(((bytes32,int128,int128,uint64,uint64,uint128),bytes))"
    )]
    pub struct SignedOrderCall {
        pub p: SignedOrder,
    }
    ///Container type for all input parameters for the `signedTransferQuote` function with signature `signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))` and selector `0x6f3b0a72`
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
        name = "signedTransferQuote",
        abi = "signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))"
    )]
    pub struct SignedTransferQuoteCall {
        pub p: SignedTransferQuote,
    }
    ///Container type for all input parameters for the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x0d55e26b`
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
        name = "signedWithdrawCollateral",
        abi = "signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))"
    )]
    pub struct SignedWithdrawCollateralCall {
        pub p: SignedWithdrawCollateral,
    }
    ///Container type for all input parameters for the `spotTick` function with signature `spotTick((uint128))` and selector `0xd9768695`
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
    #[ethcall(name = "spotTick", abi = "spotTick((uint128))")]
    pub struct SpotTickCall {
        pub p: SpotTick,
    }
    ///Container type for all input parameters for the `submitSlowModeTransaction` function with signature `submitSlowModeTransaction(bytes)` and selector `0xe604ed9e`
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
        name = "submitSlowModeTransaction",
        abi = "submitSlowModeTransaction(bytes)"
    )]
    pub struct SubmitSlowModeTransactionCall {
        pub transaction: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitTransactions` function with signature `submitTransactions(bytes[])` and selector `0x1f186b27`
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
    #[ethcall(name = "submitTransactions", abi = "submitTransactions(bytes[])")]
    pub struct SubmitTransactionsCall {
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `submitTransactionsChecked` function with signature `submitTransactionsChecked(uint64,bytes[],bytes32,bytes32,uint8)` and selector `0x7db6a25b`
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
        name = "submitTransactionsChecked",
        abi = "submitTransactionsChecked(uint64,bytes[],bytes32,bytes32,uint8)"
    )]
    pub struct SubmitTransactionsCheckedCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub e: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub s: [u8; 32],
        pub signer_bitmask: u8,
    }
    ///Container type for all input parameters for the `submitTransactionsCheckedWithGasLimit` function with signature `submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)` and selector `0x2f9a2744`
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
        name = "submitTransactionsCheckedWithGasLimit",
        abi = "submitTransactionsCheckedWithGasLimit(uint64,bytes[],uint256)"
    )]
    pub struct SubmitTransactionsCheckedWithGasLimitCall {
        pub idx: u64,
        pub transactions: ::std::vec::Vec<::ethers::core::types::Bytes>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub gas_limit: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `transferQuote` function with signature `transferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x1d97d22f`
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
        name = "transferQuote",
        abi = "transferQuote((bytes32,bytes32,uint128,uint64))"
    )]
    pub struct TransferQuoteCall {
        pub p: TransferQuote,
    }
    ///Container type for all input parameters for the `unsignedBurnNlp` function with signature `unsignedBurnNlp((bytes32,uint128,uint64))` and selector `0x9f9a35e1`
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
        name = "unsignedBurnNlp",
        abi = "unsignedBurnNlp((bytes32,uint128,uint64))"
    )]
    pub struct UnsignedBurnNlpCall {
        pub p: BurnNlp,
    }
    ///Container type for all input parameters for the `unsignedDelistProduct` function with signature `unsignedDelistProduct((uint32,int128,bytes32[]))` and selector `0xb3147d17`
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
        name = "unsignedDelistProduct",
        abi = "unsignedDelistProduct((uint32,int128,bytes32[]))"
    )]
    pub struct UnsignedDelistProductCall {
        pub p: DelistProduct,
    }
    ///Container type for all input parameters for the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `0x3cec4b93`
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
        name = "unsignedDepositCollateral",
        abi = "unsignedDepositCollateral((bytes32,uint32,uint128))"
    )]
    pub struct UnsignedDepositCollateralCall {
        pub p: DepositCollateral,
    }
    ///Container type for all input parameters for the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `0x94faefe5`
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
        name = "unsignedDepositInsurance",
        abi = "unsignedDepositInsurance((uint128))"
    )]
    pub struct UnsignedDepositInsuranceCall {
        pub p: DepositInsurance,
    }
    ///Container type for all input parameters for the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `0x05e42dc7`
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
        name = "unsignedLinkSigner",
        abi = "unsignedLinkSigner((bytes32,bytes32,uint64))"
    )]
    pub struct UnsignedLinkSignerCall {
        pub p: LinkSigner,
    }
    ///Container type for all input parameters for the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))` and selector `0x9e851424`
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
        name = "unsignedLiquidateSubaccount",
        abi = "unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))"
    )]
    pub struct UnsignedLiquidateSubaccountCall {
        pub p: LiquidateSubaccount,
    }
    ///Container type for all input parameters for the `unsignedMintNlp` function with signature `unsignedMintNlp((bytes32,uint128,uint64))` and selector `0x8f393838`
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
        name = "unsignedMintNlp",
        abi = "unsignedMintNlp((bytes32,uint128,uint64))"
    )]
    pub struct UnsignedMintNlpCall {
        pub p: MintNlp,
    }
    ///Container type for all input parameters for the `unsignedTransferQuote` function with signature `unsignedTransferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x0edaacce`
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
        name = "unsignedTransferQuote",
        abi = "unsignedTransferQuote((bytes32,bytes32,uint128,uint64))"
    )]
    pub struct UnsignedTransferQuoteCall {
        pub p: TransferQuote,
    }
    ///Container type for all input parameters for the `unsignedUpdateTierFeeRates` function with signature `unsignedUpdateTierFeeRates((uint32,uint32,int128,int128))` and selector `0xfe72d8b7`
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
        name = "unsignedUpdateTierFeeRates",
        abi = "unsignedUpdateTierFeeRates((uint32,uint32,int128,int128))"
    )]
    pub struct UnsignedUpdateTierFeeRatesCall {
        pub p: UpdateTierFeeRates,
    }
    ///Container type for all input parameters for the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `0xbc85ca86`
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
        name = "unsignedWithdrawCollateral",
        abi = "unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))"
    )]
    pub struct UnsignedWithdrawCollateralCall {
        pub p: WithdrawCollateral,
    }
    ///Container type for all input parameters for the `unsignedWithdrawInsurance` function with signature `unsignedWithdrawInsurance((uint128,address))` and selector `0x14735755`
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
        name = "unsignedWithdrawInsurance",
        abi = "unsignedWithdrawInsurance((uint128,address))"
    )]
    pub struct UnsignedWithdrawInsuranceCall {
        pub p: WithdrawInsurance,
    }
    ///Container type for all input parameters for the `updateFeeTier` function with signature `updateFeeTier((address,uint32))` and selector `0x70f6821c`
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
    #[ethcall(name = "updateFeeTier", abi = "updateFeeTier((address,uint32))")]
    pub struct UpdateFeeTierCall {
        pub p: UpdateFeeTier,
    }
    ///Container type for all input parameters for the `updateNlpPool` function with signature `updateNlpPool((uint64,address,uint128))` and selector `0x98c5b549`
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
        name = "updateNlpPool",
        abi = "updateNlpPool((uint64,address,uint128))"
    )]
    pub struct UpdateNlpPoolCall {
        pub p: UpdateNlpPool,
    }
    ///Container type for all input parameters for the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `0x1d9eeda5`
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
    #[ethcall(name = "updatePrice", abi = "updatePrice((uint32,int128))")]
    pub struct UpdatePriceCall {
        pub p: UpdatePrice,
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
    pub enum EndpointCalls {
        AddNlpPool(AddNlpPoolCall),
        AssertCode(AssertCodeCall),
        AssertProduct(AssertProductCall),
        ChainlinkFullReport(ChainlinkFullReportCall),
        ChainlinkReportBlob(ChainlinkReportBlobCall),
        CheckSlowModeTxInner(CheckSlowModeTxInnerCall),
        CheckSlowModeTxLinkSigner(CheckSlowModeTxLinkSignerCall),
        Clearinghouse(ClearinghouseCall),
        CloseIsolatedSubaccount(CloseIsolatedSubaccountCall),
        CreateIsolatedSubaccount(CreateIsolatedSubaccountCall),
        DeleteNlpPool(DeleteNlpPoolCall),
        DepositCollateral(DepositCollateralCall),
        DepositCollateralWithReferral(DepositCollateralWithReferralCall),
        ExecuteSlowModeTransaction(ExecuteSlowModeTransactionCall),
        GetHealthCheckFee(GetHealthCheckFeeCall),
        GetLinkedSigner(GetLinkedSignerCall),
        GetLiquidationFee(GetLiquidationFeeCall),
        GetNlpPools(GetNlpPoolsCall),
        GetNonce(GetNonceCall),
        GetNumSubaccounts(GetNumSubaccountsCall),
        GetOffchainExchange(GetOffchainExchangeCall),
        GetPriceX18(GetPriceX18Call),
        GetSequencer(GetSequencerCall),
        GetSequencerFee(GetSequencerFeeCall),
        GetSlots(GetSlotsCall),
        GetSlowModeTx(GetSlowModeTxCall),
        GetSubaccountById(GetSubaccountByIdCall),
        GetSubaccountId(GetSubaccountIdCall),
        GetTakerSequencerFee(GetTakerSequencerFeeCall),
        GetTime(GetTimeCall),
        GetTimes(GetTimesCall),
        IncrementSubmissions(IncrementSubmissionsCall),
        Initialize(InitializeCall),
        LiquidationStart(LiquidationStartCall),
        ManualAssert(ManualAssertCall),
        MatchOrders(MatchOrdersCall),
        MatchOrdersWithAmount(MatchOrdersWithAmountCall),
        Nsubmissions(NsubmissionsCall),
        NlpPools(NlpPoolsCall),
        Owner(OwnerCall),
        PerpTick(PerpTickCall),
        ProcessSlowModeTransaction(ProcessSlowModeTransactionCall),
        RebalanceXWithdraw(RebalanceXWithdrawCall),
        Rebate(RebateCall),
        ReferralCodes(ReferralCodesCall),
        RenounceOwnership(RenounceOwnershipCall),
        ResyncSlowModeTxs(ResyncSlowModeTxsCall),
        SetInitialPrice(SetInitialPriceCall),
        SetPriceX18(SetPriceX18Call),
        SetSlowModeConfig(SetSlowModeConfigCall),
        SetSlowModeTx(SetSlowModeTxCall),
        SettlePnl(SettlePnlCall),
        SignedBurnNlp(SignedBurnNlpCall),
        SignedCancellation(SignedCancellationCall),
        SignedCancellationProducts(SignedCancellationProductsCall),
        SignedLinkSigner(SignedLinkSignerCall),
        SignedLiquidateSubaccount(SignedLiquidateSubaccountCall),
        SignedMintNlp(SignedMintNlpCall),
        SignedOrder(SignedOrderCall),
        SignedTransferQuote(SignedTransferQuoteCall),
        SignedWithdrawCollateral(SignedWithdrawCollateralCall),
        SpotTick(SpotTickCall),
        SubmitSlowModeTransaction(SubmitSlowModeTransactionCall),
        SubmitTransactions(SubmitTransactionsCall),
        SubmitTransactionsChecked(SubmitTransactionsCheckedCall),
        SubmitTransactionsCheckedWithGasLimit(SubmitTransactionsCheckedWithGasLimitCall),
        TransferOwnership(TransferOwnershipCall),
        TransferQuote(TransferQuoteCall),
        UnsignedBurnNlp(UnsignedBurnNlpCall),
        UnsignedDelistProduct(UnsignedDelistProductCall),
        UnsignedDepositCollateral(UnsignedDepositCollateralCall),
        UnsignedDepositInsurance(UnsignedDepositInsuranceCall),
        UnsignedLinkSigner(UnsignedLinkSignerCall),
        UnsignedLiquidateSubaccount(UnsignedLiquidateSubaccountCall),
        UnsignedMintNlp(UnsignedMintNlpCall),
        UnsignedTransferQuote(UnsignedTransferQuoteCall),
        UnsignedUpdateTierFeeRates(UnsignedUpdateTierFeeRatesCall),
        UnsignedWithdrawCollateral(UnsignedWithdrawCollateralCall),
        UnsignedWithdrawInsurance(UnsignedWithdrawInsuranceCall),
        UpdateFeeTier(UpdateFeeTierCall),
        UpdateNlpPool(UpdateNlpPoolCall),
        UpdatePrice(UpdatePriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for EndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddNlpPool(decoded));
            }
            if let Ok(decoded) = <AssertCodeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssertCode(decoded));
            }
            if let Ok(decoded) = <AssertProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssertProduct(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkFullReportCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainlinkFullReport(decoded));
            }
            if let Ok(decoded) =
                <ChainlinkReportBlobCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainlinkReportBlob(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxInnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSlowModeTxInner(decoded));
            }
            if let Ok(decoded) =
                <CheckSlowModeTxLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSlowModeTxLinkSigner(decoded));
            }
            if let Ok(decoded) = <ClearinghouseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Clearinghouse(decoded));
            }
            if let Ok(decoded) =
                <CloseIsolatedSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CloseIsolatedSubaccount(decoded));
            }
            if let Ok(decoded) =
                <CreateIsolatedSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateIsolatedSubaccount(decoded));
            }
            if let Ok(decoded) = <DeleteNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeleteNlpPool(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <DepositCollateralWithReferralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositCollateralWithReferral(decoded));
            }
            if let Ok(decoded) =
                <ExecuteSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetHealthCheckFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHealthCheckFee(decoded));
            }
            if let Ok(decoded) =
                <GetLinkedSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLinkedSigner(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidationFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidationFee(decoded));
            }
            if let Ok(decoded) = <GetNlpPoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNlpPools(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetNumSubaccountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNumSubaccounts(decoded));
            }
            if let Ok(decoded) =
                <GetOffchainExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOffchainExchange(decoded));
            }
            if let Ok(decoded) = <GetPriceX18Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPriceX18(decoded));
            }
            if let Ok(decoded) = <GetSequencerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSequencer(decoded));
            }
            if let Ok(decoded) =
                <GetSequencerFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSequencerFee(decoded));
            }
            if let Ok(decoded) = <GetSlotsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlots(decoded));
            }
            if let Ok(decoded) = <GetSlowModeTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSlowModeTx(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountByIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountById(decoded));
            }
            if let Ok(decoded) =
                <GetSubaccountIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSubaccountId(decoded));
            }
            if let Ok(decoded) =
                <GetTakerSequencerFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTakerSequencerFee(decoded));
            }
            if let Ok(decoded) = <GetTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTime(decoded));
            }
            if let Ok(decoded) = <GetTimesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetTimes(decoded));
            }
            if let Ok(decoded) =
                <IncrementSubmissionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncrementSubmissions(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LiquidationStartCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidationStart(decoded));
            }
            if let Ok(decoded) = <ManualAssertCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManualAssert(decoded));
            }
            if let Ok(decoded) = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersWithAmountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MatchOrdersWithAmount(decoded));
            }
            if let Ok(decoded) = <NsubmissionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Nsubmissions(decoded));
            }
            if let Ok(decoded) = <NlpPoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NlpPools(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PerpTickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PerpTick(decoded));
            }
            if let Ok(decoded) =
                <ProcessSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <RebalanceXWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceXWithdraw(decoded));
            }
            if let Ok(decoded) = <RebateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rebate(decoded));
            }
            if let Ok(decoded) = <ReferralCodesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReferralCodes(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ResyncSlowModeTxsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ResyncSlowModeTxs(decoded));
            }
            if let Ok(decoded) =
                <SetInitialPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetInitialPrice(decoded));
            }
            if let Ok(decoded) = <SetPriceX18Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPriceX18(decoded));
            }
            if let Ok(decoded) =
                <SetSlowModeConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSlowModeConfig(decoded));
            }
            if let Ok(decoded) = <SetSlowModeTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSlowModeTx(decoded));
            }
            if let Ok(decoded) = <SettlePnlCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SettlePnl(decoded));
            }
            if let Ok(decoded) = <SignedBurnNlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedBurnNlp(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedCancellation(decoded));
            }
            if let Ok(decoded) =
                <SignedCancellationProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedCancellationProducts(decoded));
            }
            if let Ok(decoded) =
                <SignedLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <SignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) = <SignedMintNlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedMintNlp(decoded));
            }
            if let Ok(decoded) = <SignedOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SignedOrder(decoded));
            }
            if let Ok(decoded) =
                <SignedTransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedTransferQuote(decoded));
            }
            if let Ok(decoded) =
                <SignedWithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) = <SpotTickCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SpotTick(decoded));
            }
            if let Ok(decoded) =
                <SubmitSlowModeTransactionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitSlowModeTransaction(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitTransactions(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCheckedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitTransactionsChecked(decoded));
            }
            if let Ok(decoded) = <SubmitTransactionsCheckedWithGasLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitTransactionsCheckedWithGasLimit(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferQuote(decoded));
            }
            if let Ok(decoded) =
                <UnsignedBurnNlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedBurnNlp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDelistProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDelistProduct(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <UnsignedDepositInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedDepositInsurance(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLinkSignerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedLinkSigner(decoded));
            }
            if let Ok(decoded) =
                <UnsignedLiquidateSubaccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedLiquidateSubaccount(decoded));
            }
            if let Ok(decoded) =
                <UnsignedMintNlpCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedMintNlp(decoded));
            }
            if let Ok(decoded) =
                <UnsignedTransferQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedTransferQuote(decoded));
            }
            if let Ok(decoded) =
                <UnsignedUpdateTierFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedUpdateTierFeeRates(decoded));
            }
            if let Ok(decoded) =
                <UnsignedWithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedWithdrawCollateral(decoded));
            }
            if let Ok(decoded) =
                <UnsignedWithdrawInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsignedWithdrawInsurance(decoded));
            }
            if let Ok(decoded) = <UpdateFeeTierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateFeeTier(decoded));
            }
            if let Ok(decoded) = <UpdateNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateNlpPool(decoded));
            }
            if let Ok(decoded) = <UpdatePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssertCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssertProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainlinkFullReport(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainlinkReportBlob(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSlowModeTxInner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSlowModeTxLinkSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Clearinghouse(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CloseIsolatedSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateIsolatedSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositCollateral(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositCollateralWithReferral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHealthCheckFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLinkedSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidationFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNlpPools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNumSubaccounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOffchainExchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPriceX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSequencer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSequencerFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlowModeTx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountById(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSubaccountId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTakerSequencerFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTimes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncrementSubmissions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidationStart(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManualAssert(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchOrdersWithAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nsubmissions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NlpPools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerpTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RebalanceXWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rebate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReferralCodes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResyncSlowModeTxs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetInitialPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPriceX18(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSlowModeConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSlowModeTx(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettlePnl(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedBurnNlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedCancellation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedCancellationProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedLinkSigner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedMintNlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SignedTransferQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SignedWithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpotTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitSlowModeTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactionsChecked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactionsCheckedWithGasLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferQuote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedBurnNlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedDelistProduct(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedDepositCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedDepositInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedLinkSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedLiquidateSubaccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedMintNlp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnsignedTransferQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedUpdateTierFeeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedWithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsignedWithdrawInsurance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFeeTier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EndpointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssertProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkFullReport(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainlinkReportBlob(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxInner(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSlowModeTxLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clearinghouse(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseIsolatedSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateIsolatedSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositCollateralWithReferral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealthCheckFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLinkedSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidationFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNlpPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNumSubaccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOffchainExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlots(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlowModeTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountById(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSubaccountId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTakerSequencerFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimes(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementSubmissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidationStart(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManualAssert(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchOrdersWithAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nsubmissions(element) => ::core::fmt::Display::fmt(element, f),
                Self::NlpPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerpTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceXWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rebate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReferralCodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResyncSlowModeTxs(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInitialPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPriceX18(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSlowModeConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSlowModeTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettlePnl(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedBurnNlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellation(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedCancellationProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedMintNlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedTransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::SignedWithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitSlowModeTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactions(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsChecked(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactionsCheckedWithGasLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedBurnNlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDelistProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedDepositInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLinkSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedLiquidateSubaccount(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedMintNlp(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedTransferQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedUpdateTierFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedWithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsignedWithdrawInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFeeTier(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddNlpPoolCall> for EndpointCalls {
        fn from(value: AddNlpPoolCall) -> Self {
            Self::AddNlpPool(value)
        }
    }
    impl ::core::convert::From<AssertCodeCall> for EndpointCalls {
        fn from(value: AssertCodeCall) -> Self {
            Self::AssertCode(value)
        }
    }
    impl ::core::convert::From<AssertProductCall> for EndpointCalls {
        fn from(value: AssertProductCall) -> Self {
            Self::AssertProduct(value)
        }
    }
    impl ::core::convert::From<ChainlinkFullReportCall> for EndpointCalls {
        fn from(value: ChainlinkFullReportCall) -> Self {
            Self::ChainlinkFullReport(value)
        }
    }
    impl ::core::convert::From<ChainlinkReportBlobCall> for EndpointCalls {
        fn from(value: ChainlinkReportBlobCall) -> Self {
            Self::ChainlinkReportBlob(value)
        }
    }
    impl ::core::convert::From<CheckSlowModeTxInnerCall> for EndpointCalls {
        fn from(value: CheckSlowModeTxInnerCall) -> Self {
            Self::CheckSlowModeTxInner(value)
        }
    }
    impl ::core::convert::From<CheckSlowModeTxLinkSignerCall> for EndpointCalls {
        fn from(value: CheckSlowModeTxLinkSignerCall) -> Self {
            Self::CheckSlowModeTxLinkSigner(value)
        }
    }
    impl ::core::convert::From<ClearinghouseCall> for EndpointCalls {
        fn from(value: ClearinghouseCall) -> Self {
            Self::Clearinghouse(value)
        }
    }
    impl ::core::convert::From<CloseIsolatedSubaccountCall> for EndpointCalls {
        fn from(value: CloseIsolatedSubaccountCall) -> Self {
            Self::CloseIsolatedSubaccount(value)
        }
    }
    impl ::core::convert::From<CreateIsolatedSubaccountCall> for EndpointCalls {
        fn from(value: CreateIsolatedSubaccountCall) -> Self {
            Self::CreateIsolatedSubaccount(value)
        }
    }
    impl ::core::convert::From<DeleteNlpPoolCall> for EndpointCalls {
        fn from(value: DeleteNlpPoolCall) -> Self {
            Self::DeleteNlpPool(value)
        }
    }
    impl ::core::convert::From<DepositCollateralCall> for EndpointCalls {
        fn from(value: DepositCollateralCall) -> Self {
            Self::DepositCollateral(value)
        }
    }
    impl ::core::convert::From<DepositCollateralWithReferralCall> for EndpointCalls {
        fn from(value: DepositCollateralWithReferralCall) -> Self {
            Self::DepositCollateralWithReferral(value)
        }
    }
    impl ::core::convert::From<ExecuteSlowModeTransactionCall> for EndpointCalls {
        fn from(value: ExecuteSlowModeTransactionCall) -> Self {
            Self::ExecuteSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<GetHealthCheckFeeCall> for EndpointCalls {
        fn from(value: GetHealthCheckFeeCall) -> Self {
            Self::GetHealthCheckFee(value)
        }
    }
    impl ::core::convert::From<GetLinkedSignerCall> for EndpointCalls {
        fn from(value: GetLinkedSignerCall) -> Self {
            Self::GetLinkedSigner(value)
        }
    }
    impl ::core::convert::From<GetLiquidationFeeCall> for EndpointCalls {
        fn from(value: GetLiquidationFeeCall) -> Self {
            Self::GetLiquidationFee(value)
        }
    }
    impl ::core::convert::From<GetNlpPoolsCall> for EndpointCalls {
        fn from(value: GetNlpPoolsCall) -> Self {
            Self::GetNlpPools(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for EndpointCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetNumSubaccountsCall> for EndpointCalls {
        fn from(value: GetNumSubaccountsCall) -> Self {
            Self::GetNumSubaccounts(value)
        }
    }
    impl ::core::convert::From<GetOffchainExchangeCall> for EndpointCalls {
        fn from(value: GetOffchainExchangeCall) -> Self {
            Self::GetOffchainExchange(value)
        }
    }
    impl ::core::convert::From<GetPriceX18Call> for EndpointCalls {
        fn from(value: GetPriceX18Call) -> Self {
            Self::GetPriceX18(value)
        }
    }
    impl ::core::convert::From<GetSequencerCall> for EndpointCalls {
        fn from(value: GetSequencerCall) -> Self {
            Self::GetSequencer(value)
        }
    }
    impl ::core::convert::From<GetSequencerFeeCall> for EndpointCalls {
        fn from(value: GetSequencerFeeCall) -> Self {
            Self::GetSequencerFee(value)
        }
    }
    impl ::core::convert::From<GetSlotsCall> for EndpointCalls {
        fn from(value: GetSlotsCall) -> Self {
            Self::GetSlots(value)
        }
    }
    impl ::core::convert::From<GetSlowModeTxCall> for EndpointCalls {
        fn from(value: GetSlowModeTxCall) -> Self {
            Self::GetSlowModeTx(value)
        }
    }
    impl ::core::convert::From<GetSubaccountByIdCall> for EndpointCalls {
        fn from(value: GetSubaccountByIdCall) -> Self {
            Self::GetSubaccountById(value)
        }
    }
    impl ::core::convert::From<GetSubaccountIdCall> for EndpointCalls {
        fn from(value: GetSubaccountIdCall) -> Self {
            Self::GetSubaccountId(value)
        }
    }
    impl ::core::convert::From<GetTakerSequencerFeeCall> for EndpointCalls {
        fn from(value: GetTakerSequencerFeeCall) -> Self {
            Self::GetTakerSequencerFee(value)
        }
    }
    impl ::core::convert::From<GetTimeCall> for EndpointCalls {
        fn from(value: GetTimeCall) -> Self {
            Self::GetTime(value)
        }
    }
    impl ::core::convert::From<GetTimesCall> for EndpointCalls {
        fn from(value: GetTimesCall) -> Self {
            Self::GetTimes(value)
        }
    }
    impl ::core::convert::From<IncrementSubmissionsCall> for EndpointCalls {
        fn from(value: IncrementSubmissionsCall) -> Self {
            Self::IncrementSubmissions(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for EndpointCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LiquidationStartCall> for EndpointCalls {
        fn from(value: LiquidationStartCall) -> Self {
            Self::LiquidationStart(value)
        }
    }
    impl ::core::convert::From<ManualAssertCall> for EndpointCalls {
        fn from(value: ManualAssertCall) -> Self {
            Self::ManualAssert(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for EndpointCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<MatchOrdersWithAmountCall> for EndpointCalls {
        fn from(value: MatchOrdersWithAmountCall) -> Self {
            Self::MatchOrdersWithAmount(value)
        }
    }
    impl ::core::convert::From<NsubmissionsCall> for EndpointCalls {
        fn from(value: NsubmissionsCall) -> Self {
            Self::Nsubmissions(value)
        }
    }
    impl ::core::convert::From<NlpPoolsCall> for EndpointCalls {
        fn from(value: NlpPoolsCall) -> Self {
            Self::NlpPools(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for EndpointCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerpTickCall> for EndpointCalls {
        fn from(value: PerpTickCall) -> Self {
            Self::PerpTick(value)
        }
    }
    impl ::core::convert::From<ProcessSlowModeTransactionCall> for EndpointCalls {
        fn from(value: ProcessSlowModeTransactionCall) -> Self {
            Self::ProcessSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<RebalanceXWithdrawCall> for EndpointCalls {
        fn from(value: RebalanceXWithdrawCall) -> Self {
            Self::RebalanceXWithdraw(value)
        }
    }
    impl ::core::convert::From<RebateCall> for EndpointCalls {
        fn from(value: RebateCall) -> Self {
            Self::Rebate(value)
        }
    }
    impl ::core::convert::From<ReferralCodesCall> for EndpointCalls {
        fn from(value: ReferralCodesCall) -> Self {
            Self::ReferralCodes(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for EndpointCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ResyncSlowModeTxsCall> for EndpointCalls {
        fn from(value: ResyncSlowModeTxsCall) -> Self {
            Self::ResyncSlowModeTxs(value)
        }
    }
    impl ::core::convert::From<SetInitialPriceCall> for EndpointCalls {
        fn from(value: SetInitialPriceCall) -> Self {
            Self::SetInitialPrice(value)
        }
    }
    impl ::core::convert::From<SetPriceX18Call> for EndpointCalls {
        fn from(value: SetPriceX18Call) -> Self {
            Self::SetPriceX18(value)
        }
    }
    impl ::core::convert::From<SetSlowModeConfigCall> for EndpointCalls {
        fn from(value: SetSlowModeConfigCall) -> Self {
            Self::SetSlowModeConfig(value)
        }
    }
    impl ::core::convert::From<SetSlowModeTxCall> for EndpointCalls {
        fn from(value: SetSlowModeTxCall) -> Self {
            Self::SetSlowModeTx(value)
        }
    }
    impl ::core::convert::From<SettlePnlCall> for EndpointCalls {
        fn from(value: SettlePnlCall) -> Self {
            Self::SettlePnl(value)
        }
    }
    impl ::core::convert::From<SignedBurnNlpCall> for EndpointCalls {
        fn from(value: SignedBurnNlpCall) -> Self {
            Self::SignedBurnNlp(value)
        }
    }
    impl ::core::convert::From<SignedCancellationCall> for EndpointCalls {
        fn from(value: SignedCancellationCall) -> Self {
            Self::SignedCancellation(value)
        }
    }
    impl ::core::convert::From<SignedCancellationProductsCall> for EndpointCalls {
        fn from(value: SignedCancellationProductsCall) -> Self {
            Self::SignedCancellationProducts(value)
        }
    }
    impl ::core::convert::From<SignedLinkSignerCall> for EndpointCalls {
        fn from(value: SignedLinkSignerCall) -> Self {
            Self::SignedLinkSigner(value)
        }
    }
    impl ::core::convert::From<SignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: SignedLiquidateSubaccountCall) -> Self {
            Self::SignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<SignedMintNlpCall> for EndpointCalls {
        fn from(value: SignedMintNlpCall) -> Self {
            Self::SignedMintNlp(value)
        }
    }
    impl ::core::convert::From<SignedOrderCall> for EndpointCalls {
        fn from(value: SignedOrderCall) -> Self {
            Self::SignedOrder(value)
        }
    }
    impl ::core::convert::From<SignedTransferQuoteCall> for EndpointCalls {
        fn from(value: SignedTransferQuoteCall) -> Self {
            Self::SignedTransferQuote(value)
        }
    }
    impl ::core::convert::From<SignedWithdrawCollateralCall> for EndpointCalls {
        fn from(value: SignedWithdrawCollateralCall) -> Self {
            Self::SignedWithdrawCollateral(value)
        }
    }
    impl ::core::convert::From<SpotTickCall> for EndpointCalls {
        fn from(value: SpotTickCall) -> Self {
            Self::SpotTick(value)
        }
    }
    impl ::core::convert::From<SubmitSlowModeTransactionCall> for EndpointCalls {
        fn from(value: SubmitSlowModeTransactionCall) -> Self {
            Self::SubmitSlowModeTransaction(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCall) -> Self {
            Self::SubmitTransactions(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCheckedCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCheckedCall) -> Self {
            Self::SubmitTransactionsChecked(value)
        }
    }
    impl ::core::convert::From<SubmitTransactionsCheckedWithGasLimitCall> for EndpointCalls {
        fn from(value: SubmitTransactionsCheckedWithGasLimitCall) -> Self {
            Self::SubmitTransactionsCheckedWithGasLimit(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for EndpointCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferQuoteCall> for EndpointCalls {
        fn from(value: TransferQuoteCall) -> Self {
            Self::TransferQuote(value)
        }
    }
    impl ::core::convert::From<UnsignedBurnNlpCall> for EndpointCalls {
        fn from(value: UnsignedBurnNlpCall) -> Self {
            Self::UnsignedBurnNlp(value)
        }
    }
    impl ::core::convert::From<UnsignedDelistProductCall> for EndpointCalls {
        fn from(value: UnsignedDelistProductCall) -> Self {
            Self::UnsignedDelistProduct(value)
        }
    }
    impl ::core::convert::From<UnsignedDepositCollateralCall> for EndpointCalls {
        fn from(value: UnsignedDepositCollateralCall) -> Self {
            Self::UnsignedDepositCollateral(value)
        }
    }
    impl ::core::convert::From<UnsignedDepositInsuranceCall> for EndpointCalls {
        fn from(value: UnsignedDepositInsuranceCall) -> Self {
            Self::UnsignedDepositInsurance(value)
        }
    }
    impl ::core::convert::From<UnsignedLinkSignerCall> for EndpointCalls {
        fn from(value: UnsignedLinkSignerCall) -> Self {
            Self::UnsignedLinkSigner(value)
        }
    }
    impl ::core::convert::From<UnsignedLiquidateSubaccountCall> for EndpointCalls {
        fn from(value: UnsignedLiquidateSubaccountCall) -> Self {
            Self::UnsignedLiquidateSubaccount(value)
        }
    }
    impl ::core::convert::From<UnsignedMintNlpCall> for EndpointCalls {
        fn from(value: UnsignedMintNlpCall) -> Self {
            Self::UnsignedMintNlp(value)
        }
    }
    impl ::core::convert::From<UnsignedTransferQuoteCall> for EndpointCalls {
        fn from(value: UnsignedTransferQuoteCall) -> Self {
            Self::UnsignedTransferQuote(value)
        }
    }
    impl ::core::convert::From<UnsignedUpdateTierFeeRatesCall> for EndpointCalls {
        fn from(value: UnsignedUpdateTierFeeRatesCall) -> Self {
            Self::UnsignedUpdateTierFeeRates(value)
        }
    }
    impl ::core::convert::From<UnsignedWithdrawCollateralCall> for EndpointCalls {
        fn from(value: UnsignedWithdrawCollateralCall) -> Self {
            Self::UnsignedWithdrawCollateral(value)
        }
    }
    impl ::core::convert::From<UnsignedWithdrawInsuranceCall> for EndpointCalls {
        fn from(value: UnsignedWithdrawInsuranceCall) -> Self {
            Self::UnsignedWithdrawInsurance(value)
        }
    }
    impl ::core::convert::From<UpdateFeeTierCall> for EndpointCalls {
        fn from(value: UpdateFeeTierCall) -> Self {
            Self::UpdateFeeTier(value)
        }
    }
    impl ::core::convert::From<UpdateNlpPoolCall> for EndpointCalls {
        fn from(value: UpdateNlpPoolCall) -> Self {
            Self::UpdateNlpPool(value)
        }
    }
    impl ::core::convert::From<UpdatePriceCall> for EndpointCalls {
        fn from(value: UpdatePriceCall) -> Self {
            Self::UpdatePrice(value)
        }
    }
    ///Container type for all return fields from the `addNlpPool` function with signature `addNlpPool((address,uint128))` and selector `0xba8d8181`
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
    pub struct AddNlpPoolReturn(pub AddNlpPool);
    ///Container type for all return fields from the `assertCode` function with signature `assertCode((string[],bytes32[],uint256))` and selector `0xf968c7f4`
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
    pub struct AssertCodeReturn(pub AssertCode);
    ///Container type for all return fields from the `assertProduct` function with signature `assertProduct((uint32,bool,uint32,int128,int128,bytes32))` and selector `0x0bb9c3a2`
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
    pub struct AssertProductReturn(pub AssertProduct);
    ///Container type for all return fields from the `chainlinkFullReport` function with signature `chainlinkFullReport((bytes32[3],bytes,bytes32[],bytes32[],bytes32))` and selector `0xdb5a5021`
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
    pub struct ChainlinkFullReportReturn(pub ChainlinkFullReport);
    ///Container type for all return fields from the `chainlinkReportBlob` function with signature `chainlinkReportBlob((bytes32,uint32,uint32,uint192,uint192,uint64,int192))` and selector `0x96c47c6f`
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
    pub struct ChainlinkReportBlobReturn(pub ChainlinkReportBlob);
    ///Container type for all return fields from the `checkSlowModeTxInner` function with signature `checkSlowModeTxInner(address,bytes)` and selector `0xb70eb263`
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
    pub struct CheckSlowModeTxInnerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `checkSlowModeTxLinkSigner` function with signature `checkSlowModeTxLinkSigner()` and selector `0x5bb4c126`
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
    pub struct CheckSlowModeTxLinkSignerReturn(pub [u8; 32]);
    ///Container type for all return fields from the `clearinghouse` function with signature `clearinghouse()` and selector `0x5d4f5f97`
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
    pub struct ClearinghouseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `closeIsolatedSubaccount` function with signature `closeIsolatedSubaccount((bytes32))` and selector `0xdb3aa846`
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
    pub struct CloseIsolatedSubaccountReturn(pub CloseIsolatedSubaccount);
    ///Container type for all return fields from the `createIsolatedSubaccount` function with signature `createIsolatedSubaccount(((bytes32,int128,int128,uint64,uint64,uint128),uint32,bytes))` and selector `0x690349cf`
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
    pub struct CreateIsolatedSubaccountReturn(pub CreateIsolatedSubaccount);
    ///Container type for all return fields from the `deleteNlpPool` function with signature `deleteNlpPool((uint64))` and selector `0x338a7e56`
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
    pub struct DeleteNlpPoolReturn(pub DeleteNlpPool);
    ///Container type for all return fields from the `getHealthCheckFee` function with signature `getHealthCheckFee()` and selector `0xd4de8d9d`
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
    pub struct GetHealthCheckFeeReturn(pub i128);
    ///Container type for all return fields from the `getLinkedSigner` function with signature `getLinkedSigner(bytes32)` and selector `0x91c1e3d7`
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
    pub struct GetLinkedSignerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLiquidationFee` function with signature `getLiquidationFee()` and selector `0xfbf41984`
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
    pub struct GetLiquidationFeeReturn(pub i128);
    ///Container type for all return fields from the `getNlpPools` function with signature `getNlpPools()` and selector `0x1c886d0b`
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
    pub struct GetNlpPoolsReturn(pub ::std::vec::Vec<NlpPool>);
    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    pub struct GetNonceReturn(pub u64);
    ///Container type for all return fields from the `getNumSubaccounts` function with signature `getNumSubaccounts()` and selector `0xc4f9b25f`
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
    pub struct GetNumSubaccountsReturn(pub u64);
    ///Container type for all return fields from the `getOffchainExchange` function with signature `getOffchainExchange()` and selector `0x8f4f8ecc`
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
    pub struct GetOffchainExchangeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPriceX18` function with signature `getPriceX18(uint32)` and selector `0x368e4686`
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
    pub struct GetPriceX18Return {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
    }
    ///Container type for all return fields from the `getSequencer` function with signature `getSequencer()` and selector `0x4d96a90a`
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
    pub struct GetSequencerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSequencerFee` function with signature `getSequencerFee(uint32)` and selector `0x4fcfae58`
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
    pub struct GetSequencerFeeReturn(pub i128);
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
        pub n_submissions_slot: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSlowModeTx` function with signature `getSlowModeTx(uint64)` and selector `0xee525526`
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
    pub struct GetSlowModeTxReturn(pub SlowModeTx, pub u64, pub u64);
    ///Container type for all return fields from the `getSubaccountById` function with signature `getSubaccountById(uint64)` and selector `0xef64ed0e`
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
    pub struct GetSubaccountByIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getSubaccountId` function with signature `getSubaccountId(bytes32)` and selector `0x22d4a82d`
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
    pub struct GetSubaccountIdReturn(pub u64);
    ///Container type for all return fields from the `getTakerSequencerFee` function with signature `getTakerSequencerFee()` and selector `0xc510359f`
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
    pub struct GetTakerSequencerFeeReturn(pub i128);
    ///Container type for all return fields from the `getTime` function with signature `getTime()` and selector `0x557ed1ba`
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
    pub struct GetTimeReturn(pub u128);
    ///Container type for all return fields from the `getTimes` function with signature `getTimes()` and selector `0xe9ab77e5`
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
    pub struct GetTimesReturn(pub Times);
    ///Container type for all return fields from the `incrementSubmissions` function with signature `incrementSubmissions()` and selector `0x22006046`
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
    pub struct IncrementSubmissionsReturn(pub u64);
    ///Container type for all return fields from the `manualAssert` function with signature `manualAssert((int128,bytes[],bytes[]))` and selector `0xe7c80751`
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
    pub struct ManualAssertReturn(pub ManualAssert);
    ///Container type for all return fields from the `matchOrders` function with signature `matchOrders((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)))` and selector `0x69fddcec`
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
    pub struct MatchOrdersReturn(pub MatchOrders);
    ///Container type for all return fields from the `matchOrdersWithAmount` function with signature `matchOrdersWithAmount(((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),int128))` and selector `0xf8089d9c`
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
    pub struct MatchOrdersWithAmountReturn(pub MatchOrdersWithAmount);
    ///Container type for all return fields from the `nSubmissions` function with signature `nSubmissions()` and selector `0x18ed16eb`
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
    pub struct NsubmissionsReturn(pub u64);
    ///Container type for all return fields from the `nlpPools` function with signature `nlpPools(uint256)` and selector `0xfe00842c`
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
    pub struct NlpPoolsReturn {
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
    ///Container type for all return fields from the `perpTick` function with signature `perpTick((uint128,int128[]))` and selector `0x5a00923b`
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
    pub struct PerpTickReturn(pub PerpTick);
    ///Container type for all return fields from the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw((uint32,uint128,address))` and selector `0x9a08e535`
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
    pub struct RebalanceXWithdrawReturn(pub RebalanceXWithdraw);
    ///Container type for all return fields from the `rebate` function with signature `rebate((bytes32[],int128[]))` and selector `0x42c74d1d`
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
    pub struct RebateReturn(pub Rebate);
    ///Container type for all return fields from the `referralCodes` function with signature `referralCodes(address)` and selector `0x9534dd3e`
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
    pub struct ReferralCodesReturn(pub ::std::string::String);
    ///Container type for all return fields from the `settlePnl` function with signature `settlePnl((bytes32[],uint256[]))` and selector `0xb2bb6367`
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
    pub struct SettlePnlReturn(pub SettlePnl);
    ///Container type for all return fields from the `signedBurnNlp` function with signature `signedBurnNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))` and selector `0xf8531a64`
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
    pub struct SignedBurnNlpReturn(pub SignedBurnNlp);
    ///Container type for all return fields from the `signedCancellation` function with signature `signedCancellation(((bytes32,uint32[],bytes32[],uint64),bytes))` and selector `0x3edf2c5b`
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
    pub struct SignedCancellationReturn(pub SignedCancellation);
    ///Container type for all return fields from the `signedCancellationProducts` function with signature `signedCancellationProducts(((bytes32,uint32[],uint64),bytes))` and selector `0xa082c5aa`
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
    pub struct SignedCancellationProductsReturn(pub SignedCancellationProducts);
    ///Container type for all return fields from the `signedLinkSigner` function with signature `signedLinkSigner(((bytes32,bytes32,uint64),bytes))` and selector `0x85c83e9d`
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
    pub struct SignedLinkSignerReturn(pub SignedLinkSigner);
    ///Container type for all return fields from the `signedLiquidateSubaccount` function with signature `signedLiquidateSubaccount(((bytes32,bytes32,uint32,bool,int128,uint64),bytes))` and selector `0x9171d08b`
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
    pub struct SignedLiquidateSubaccountReturn(pub SignedLiquidateSubaccount);
    ///Container type for all return fields from the `signedMintNlp` function with signature `signedMintNlp(((bytes32,uint128,uint64),bytes,int128,int128[]))` and selector `0x5c5b34ef`
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
    pub struct SignedMintNlpReturn(pub SignedMintNlp);
    ///Container type for all return fields from the `signedOrder` function with signature `signedOrder(((bytes32,int128,int128,uint64,uint64,uint128),bytes))` and selector `0x41a09bb6`
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
    pub struct SignedOrderReturn(pub SignedOrder);
    ///Container type for all return fields from the `signedTransferQuote` function with signature `signedTransferQuote(((bytes32,bytes32,uint128,uint64),bytes))` and selector `0x6f3b0a72`
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
    pub struct SignedTransferQuoteReturn(pub SignedTransferQuote);
    ///Container type for all return fields from the `signedWithdrawCollateral` function with signature `signedWithdrawCollateral(((bytes32,uint32,uint128,uint64),bytes))` and selector `0x0d55e26b`
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
    pub struct SignedWithdrawCollateralReturn(pub SignedWithdrawCollateral);
    ///Container type for all return fields from the `spotTick` function with signature `spotTick((uint128))` and selector `0xd9768695`
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
    pub struct SpotTickReturn(pub SpotTick);
    ///Container type for all return fields from the `transferQuote` function with signature `transferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x1d97d22f`
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
    pub struct TransferQuoteReturn(pub TransferQuote);
    ///Container type for all return fields from the `unsignedBurnNlp` function with signature `unsignedBurnNlp((bytes32,uint128,uint64))` and selector `0x9f9a35e1`
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
    pub struct UnsignedBurnNlpReturn(pub BurnNlp);
    ///Container type for all return fields from the `unsignedDelistProduct` function with signature `unsignedDelistProduct((uint32,int128,bytes32[]))` and selector `0xb3147d17`
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
    pub struct UnsignedDelistProductReturn(pub DelistProduct);
    ///Container type for all return fields from the `unsignedDepositCollateral` function with signature `unsignedDepositCollateral((bytes32,uint32,uint128))` and selector `0x3cec4b93`
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
    pub struct UnsignedDepositCollateralReturn(pub DepositCollateral);
    ///Container type for all return fields from the `unsignedDepositInsurance` function with signature `unsignedDepositInsurance((uint128))` and selector `0x94faefe5`
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
    pub struct UnsignedDepositInsuranceReturn(pub DepositInsurance);
    ///Container type for all return fields from the `unsignedLinkSigner` function with signature `unsignedLinkSigner((bytes32,bytes32,uint64))` and selector `0x05e42dc7`
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
    pub struct UnsignedLinkSignerReturn(pub LinkSigner);
    ///Container type for all return fields from the `unsignedLiquidateSubaccount` function with signature `unsignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64))` and selector `0x9e851424`
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
    pub struct UnsignedLiquidateSubaccountReturn(pub LiquidateSubaccount);
    ///Container type for all return fields from the `unsignedMintNlp` function with signature `unsignedMintNlp((bytes32,uint128,uint64))` and selector `0x8f393838`
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
    pub struct UnsignedMintNlpReturn(pub MintNlp);
    ///Container type for all return fields from the `unsignedTransferQuote` function with signature `unsignedTransferQuote((bytes32,bytes32,uint128,uint64))` and selector `0x0edaacce`
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
    pub struct UnsignedTransferQuoteReturn(pub TransferQuote);
    ///Container type for all return fields from the `unsignedUpdateTierFeeRates` function with signature `unsignedUpdateTierFeeRates((uint32,uint32,int128,int128))` and selector `0xfe72d8b7`
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
    pub struct UnsignedUpdateTierFeeRatesReturn(pub UpdateTierFeeRates);
    ///Container type for all return fields from the `unsignedWithdrawCollateral` function with signature `unsignedWithdrawCollateral((bytes32,uint32,uint128,uint64))` and selector `0xbc85ca86`
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
    pub struct UnsignedWithdrawCollateralReturn(pub WithdrawCollateral);
    ///Container type for all return fields from the `unsignedWithdrawInsurance` function with signature `unsignedWithdrawInsurance((uint128,address))` and selector `0x14735755`
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
    pub struct UnsignedWithdrawInsuranceReturn(pub WithdrawInsurance);
    ///Container type for all return fields from the `updateFeeTier` function with signature `updateFeeTier((address,uint32))` and selector `0x70f6821c`
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
    pub struct UpdateFeeTierReturn(pub UpdateFeeTier);
    ///Container type for all return fields from the `updateNlpPool` function with signature `updateNlpPool((uint64,address,uint128))` and selector `0x98c5b549`
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
    pub struct UpdateNlpPoolReturn(pub UpdateNlpPool);
    ///Container type for all return fields from the `updatePrice` function with signature `updatePrice((uint32,int128))` and selector `0x1d9eeda5`
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
    pub struct UpdatePriceReturn(pub UpdatePrice);
    ///`Times(uint128,uint128)`
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
    pub struct Times {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub perp_time: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub spot_time: u128,
    }
    ///`Cancellation(bytes32,uint32[],bytes32[],uint64)`
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
    pub struct Cancellation {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub digests: ::std::vec::Vec<[u8; 32]>,
        pub nonce: u64,
    }
    ///`CancellationProducts(bytes32,uint32[],uint64)`
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
    pub struct CancellationProducts {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_ids: ::std::vec::Vec<u32>,
        pub nonce: u64,
    }
    ///`ChainlinkFullReport(bytes32[3],bytes,bytes32[],bytes32[],bytes32)`
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
    pub struct ChainlinkFullReport {
        pub report_context: [[u8; 32]; 3],
        pub report_blob: ::ethers::core::types::Bytes,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub raw_rs: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub raw_ss: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub raw_vs: [u8; 32],
    }
    ///`ChainlinkReportBlob(bytes32,uint32,uint32,uint192,uint192,uint64,int192)`
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
    pub struct ChainlinkReportBlob {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub feed_id: [u8; 32],
        pub valid_from_timestamp: u32,
        pub observations_timestamp: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub native_fee: ::ethers::core::types::U256,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub link_fee: ::ethers::core::types::U256,
        pub expires_at: u64,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i256",
            deserialize_with = "crate::serialize_utils::deserialize_i256"
        )]
        pub price: ::ethers::core::types::I256,
    }
    ///`SignedCancellation((bytes32,uint32[],bytes32[],uint64),bytes)`
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
    pub struct SignedCancellation {
        pub cancellation: Cancellation,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedCancellationProducts((bytes32,uint32[],uint64),bytes)`
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
    pub struct SignedCancellationProducts {
        pub cancellation_products: CancellationProducts,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`AddNlpPool(address,uint128)`
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
    pub struct AddNlpPool {
        pub owner: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub balance_weight_x18: u128,
    }
    ///`AssertCode(string[],bytes32[],uint256)`
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
    pub struct AssertCode {
        pub contract_names: ::std::vec::Vec<::std::string::String>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub code_hashes: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u256",
            deserialize_with = "crate::serialize_utils::deserialize_u256"
        )]
        pub spreads: ::ethers::core::types::U256,
    }
    ///`AssertProduct(uint32,bool,uint32,int128,int128,bytes32)`
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
    pub struct AssertProduct {
        pub product_id: u32,
        pub is_spot: bool,
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
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub others_hash: [u8; 32],
    }
    ///`BurnNlp(bytes32,uint128,uint64)`
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
    pub struct BurnNlp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub nlp_amount: u128,
        pub nonce: u64,
    }
    ///`CloseIsolatedSubaccount(bytes32)`
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
    pub struct CloseIsolatedSubaccount {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub subaccount: [u8; 32],
    }
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
    ///`DeleteNlpPool(uint64)`
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
    pub struct DeleteNlpPool {
        pub pool_id: u64,
    }
    ///`DelistProduct(uint32,int128,bytes32[])`
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
    pub struct DelistProduct {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
    }
    ///`DepositCollateral(bytes32,uint32,uint128)`
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
    pub struct DepositCollateral {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///`DepositInsurance(uint128)`
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
    pub struct DepositInsurance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
    }
    ///`LinkSigner(bytes32,bytes32,uint64)`
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
    pub struct LinkSigner {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub signer: [u8; 32],
        pub nonce: u64,
    }
    ///`LiquidateSubaccount(bytes32,bytes32,uint32,bool,int128,uint64)`
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
    pub struct LiquidateSubaccount {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub liquidatee: [u8; 32],
        pub product_id: u32,
        pub is_encoded_spread: bool,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub amount: i128,
        pub nonce: u64,
    }
    ///`ManualAssert(int128,bytes[],bytes[])`
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
    pub struct ManualAssert {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub insurance: i128,
        pub spot_states: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub perp_states: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    ///`MatchOrdersWithAmount((uint32,((bytes32,int128,int128,uint64,uint64,uint128),bytes),((bytes32,int128,int128,uint64,uint64,uint128),bytes)),int128)`
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
    pub struct MatchOrdersWithAmount {
        pub match_orders: MatchOrders,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub taker_amount_delta: i128,
    }
    ///`MintNlp(bytes32,uint128,uint64)`
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
    pub struct MintNlp {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub quote_amount: u128,
        pub nonce: u64,
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
    ///`PerpTick(uint128,int128[])`
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
    pub struct PerpTick {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub time: u128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub avg_price_diffs: ::std::vec::Vec<i128>,
    }
    ///`RebalanceXWithdraw(uint32,uint128,address)`
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
    pub struct RebalanceXWithdraw {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///`Rebate(bytes32[],int128[])`
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
    pub struct Rebate {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub amounts: ::std::vec::Vec<i128>,
    }
    ///`SettlePnl(bytes32[],uint256[])`
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
    pub struct SettlePnl {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_vec_bytes32"
        )]
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
        pub product_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`SignedBurnNlp((bytes32,uint128,uint64),bytes,int128,int128[])`
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
    pub struct SignedBurnNlp {
        pub tx: BurnNlp,
        pub signature: ::ethers::core::types::Bytes,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub nlp_pool_rebalance_x18: ::std::vec::Vec<i128>,
    }
    ///`SignedLinkSigner((bytes32,bytes32,uint64),bytes)`
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
    pub struct SignedLinkSigner {
        pub tx: LinkSigner,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedLiquidateSubaccount((bytes32,bytes32,uint32,bool,int128,uint64),bytes)`
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
    pub struct SignedLiquidateSubaccount {
        pub tx: LiquidateSubaccount,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedMintNlp((bytes32,uint128,uint64),bytes,int128,int128[])`
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
    pub struct SignedMintNlp {
        pub tx: MintNlp,
        pub signature: ::ethers::core::types::Bytes,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub oracle_price_x18: i128,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_vec_i128",
            deserialize_with = "crate::serialize_utils::deserialize_vec_i128"
        )]
        pub nlp_pool_rebalance_x18: ::std::vec::Vec<i128>,
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
    ///`SignedTransferQuote((bytes32,bytes32,uint128,uint64),bytes)`
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
    pub struct SignedTransferQuote {
        pub tx: TransferQuote,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SignedWithdrawCollateral((bytes32,uint32,uint128,uint64),bytes)`
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
    pub struct SignedWithdrawCollateral {
        pub tx: WithdrawCollateral,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SlowModeConfig(uint64,uint64,uint64)`
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
    pub struct SlowModeConfig {
        pub timeout: u64,
        pub tx_count: u64,
        pub tx_up_to: u64,
    }
    ///`SlowModeTx(uint64,address,bytes)`
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
    pub struct SlowModeTx {
        pub executable_at: u64,
        pub sender: ::ethers::core::types::Address,
        pub tx: ::ethers::core::types::Bytes,
    }
    ///`SpotTick(uint128)`
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
    pub struct SpotTick {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub time: u128,
    }
    ///`TransferQuote(bytes32,bytes32,uint128,uint64)`
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
    pub struct TransferQuote {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub recipient: [u8; 32],
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
    ///`UpdateFeeTier(address,uint32)`
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
    pub struct UpdateFeeTier {
        pub user: ::ethers::core::types::Address,
        pub new_tier: u32,
    }
    ///`UpdateNlpPool(uint64,address,uint128)`
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
    pub struct UpdateNlpPool {
        pub pool_id: u64,
        pub owner: ::ethers::core::types::Address,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub balance_weight_x18: u128,
    }
    ///`UpdatePrice(uint32,int128)`
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
    pub struct UpdatePrice {
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_i128",
            deserialize_with = "crate::serialize_utils::deserialize_i128"
        )]
        pub price_x18: i128,
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
    ///`WithdrawCollateral(bytes32,uint32,uint128,uint64)`
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
    pub struct WithdrawCollateral {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_bytes32",
            deserialize_with = "crate::serialize_utils::deserialize_bytes32"
        )]
        pub sender: [u8; 32],
        pub product_id: u32,
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub nonce: u64,
    }
    ///`WithdrawInsurance(uint128,address)`
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
    pub struct WithdrawInsurance {
        #[serde(
            serialize_with = "crate::serialize_utils::serialize_u128",
            deserialize_with = "crate::serialize_utils::deserialize_u128"
        )]
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
}
