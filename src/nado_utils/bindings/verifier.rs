pub use verifier::*;
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
pub mod verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Q"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("Q"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_P"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_P"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assignPubKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("assignPubKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
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
                    ::std::borrow::ToOwned::to_owned("checkIndividualSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkIndividualSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signerIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("computeDigest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeDigest"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IEndpoint.TransactionType",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transactionBody"),
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
                    ::std::borrow::ToOwned::to_owned("deletePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deletePubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Verifier.Point"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPubkeyAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPubkeyAddress"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("initialSet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                                8usize,
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Verifier.Point[8]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("requireValidSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requireValidSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("message"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("requireValidTxSignatures"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requireValidTxSignatures",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signatures"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revertGasInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("revertGasInfo"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasUsed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("validateCompactSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateCompactSignature",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("linkedSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IEndpoint.CompactSignature",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateSignature"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSignature"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("linkedSigner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("digest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssignPubKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AssignPubKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("i"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DeletePubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DeletePubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
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
    pub static VERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1Cb\0\0\"V[b\0\0\xE4V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\0\xE2W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[a+\xEF\x80b\0\0\xF4`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x1BW`\x005`\xE0\x1C\x80c_\xCB}X\x11a\0\xB2W\x80c\xBE\x13\xBA\xC4\x11a\0\x81W\x80c\xE4\x93\xEF\x8C\x11a\0fW\x80c\xE4\x93\xEF\x8C\x14a\x02_W\x80c\xEBvJ&\x14a\x02xW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9BW`\0\x80\xFD[\x80c\xBE\x13\xBA\xC4\x14a\x029W\x80c\xC9E\xD3Z\x14a\x02LW`\0\x80\xFD[\x80c_\xCB}X\x14a\x01\xFAW\x80cqP\x18\xA6\x14a\x02\rW\x80c\x8D\xA5\xCB[\x14a\x02\x15W\x80c\xBB\xEF\x84\xB4\x14a\x02&W`\0\x80\xFD[\x80cU\xE7g;\x11a\0\xEEW\x80cU\xE7g;\x14a\x01{W\x80cY\x02\xCA\x1B\x14a\x01\xA9W\x80c[P\xF3E\x14a\x01\xBCW\x80c]\x18\x16\xD9\x14a\x01\xCFW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\x01 W\x80c5Ob*\x14a\x015W\x80c<d\xC2\x15\x14a\x01UW\x80cUu}\xBF\x14a\x01hW[`\0\x80\xFD[a\x013a\x01.6`\x04a\x1C\x1CV[a\x02\xAEV[\0[a\x01Bd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x013a\x01c6`\x04a\x1C[V[a\x03RV[a\x013a\x01v6`\x04a\x1C\xD7V[a\x03\x9BV[a\x01\x8Ea\x01\x896`\x04a\x1D\x88V[a\x05\xE5V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01LV[a\x013a\x01\xB76`\x04a\x1E\xEEV[a\x069V[a\x01Ba\x01\xCA6`\x04a\x1F*V[a\x06\xD0V[a\x01\xE2a\x01\xDD6`\x04a\x1D\x88V[a\x0BDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01LV[a\x013a\x02\x086`\x04a\x1F\x83V[a\x0B\x92V[a\x013a\r/V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xE2V[a\x013a\x0246`\x04a \x0BV[a\rCV[a\x013a\x02G6`\x04a $V[a\x0E\x04V[a\x013a\x02Z6`\x04a \xC0V[a\x0E\x1CV[a\x01Bp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02\x8Ba\x02\x866`\x04a!!V[a\x0E(V[`@Q\x90\x15\x15\x81R` \x01a\x01LV[a\x013a\x02\xA96`\x04a!xV[a\x0E\\V[a\x02\xB7\x81a\x0E\xECV[a\x02\xC0W`\0\x80\xFD[`\0a\x02\xCB\x82a\x0FhV[\x90Pa\x02\xFA`\x02\x82` \x01Qa\x02\xE1\x91\x90a!\xA9V[\x15a\x02\xEDW`\x1Ca\x02\xF0V[`\x1B[\x82Q\x87\x87\x87a\x11\x05V[a\x03KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03[\x82a\x12\xB6V[a\x03d\x82a\x12\xB6V[`@Q` \x01a\x03u\x92\x91\x90a!\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03B\x91`\x04\x01a\"7V[a\x01\0\x81\x11\x15a\x03\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Ftoo many signatures\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x04\x10\x94\x93\x92\x91\x90a\"jV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x04g\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x05\x88W`\0\x86\x86\x83\x81\x81\x10a\x04\x9FWa\x04\x9Fa\"\x8BV[\x90P` \x02\x81\x01\x90a\x04\xB1\x91\x90a\"\xA1V[\x90P\x11\x15a\x05vWa\x04\xC4`\x01\x83a\"\xFEV[\x91Pa\x05*\x83\x87\x87\x84\x81\x81\x10a\x04\xDCWa\x04\xDCa\"\x8BV[\x90P` \x02\x81\x01\x90a\x04\xEE\x91\x90a\"\xA1V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x0E(\x91PPV[a\x05vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[\x80a\x05\x80\x81a#\x16V[\x91PPa\x04\x83V[Pa\x02\xB1T\x81\x14a\x05\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x06\x0FWa\x06\x0Fa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0a\x06N\x83\x83`\0\x01Q\x84` \x01Qa\x13\xFCV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x06\x8FWP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x06\x8FWP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x06\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03B\x91\x90a\"7V[PPPPPPV[`\0\x80\x80\x85` \x81\x11\x15a\x06\xE6Wa\x06\xE6a#/V[\x03a\x07\xA3W`\0a\x06\xF9\x84\x86\x01\x86a#kV[\x90P`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a*\x86`w\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PPa\x0B<V[`\x02\x85` \x81\x11\x15a\x07\xB7Wa\x07\xB7a#/V[\x03a\x08CW`\0a\x07\xCA\x84\x86\x01\x86a$kV[\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a+k`O\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x07\x85V[` \x85` \x81\x11\x15a\x08WWa\x08Wa#/V[\x03a\t\rW`\0a\x08j\x84\x86\x01\x86a%\x17V[\x90P`@Q\x80`\xA0\x01`@R\x80`q\x81R` \x01a)\x94`q\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x92\x85\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x84\x01R\x16`\xE0\x82\x01Ra\x01\0\x01a\x07\x85V[`\x15\x85` \x81\x11\x15a\t!Wa\t!a#/V[\x03a\t\x97W`\0a\t4\x84\x86\x01\x86a' V[\x90P`@Q\x80``\x01`@R\x80`8\x81R` \x01a+3`8\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\x16\x85` \x81\x11\x15a\t\xABWa\t\xABa#/V[\x03a\n!W`\0a\t\xBE\x84\x86\x01\x86a' V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a*\xFD`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\r\x85` \x81\x11\x15a\n5Wa\n5a#/V[\x03a\n\xA2W`\0a\nH\x84\x86\x01\x86a'UV[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a*P`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\x0F\x85` \x81\x11\x15a\n\xB6Wa\n\xB6a#/V[\x03a\x01\x1BW`\0a\n\xC9\x84\x86\x01\x86a'\xE9V[\x90P`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a*\x05`K\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x07\x85V[\x94\x93PPPPV[`\0\x80a\x0BP\x83a\x05\xE5V[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x0Bs\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0B\xB2WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0B\xCCWP0;\x15\x80\x15a\x0B\xCCWP`\0T`\xFF\x16`\x01\x14[a\x0C>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03BV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0CaW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0Cia\x14\"V[`\0[`\x08\x81\x10\x15a\x0C\xE4Wa\x0C\x94\x83\x82`\x08\x81\x10a\x0C\x8AWa\x0C\x8Aa\"\x8BV[` \x02\x01Qa\x14\x95V[a\x0C\xD4Wa\x0C\xD4\x81\x84\x83`\x08\x81\x10a\x0C\xAEWa\x0C\xAEa\"\x8BV[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x0C\xC6Wa\x0C\xC6a\"\x8BV[` \x02\x01Q` \x01Qa\x14\xAFV[a\x0C\xDD\x81a#\x16V[\x90Pa\x0ClV[P\x80\x15a\r+W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\r7a\x15\xD8V[a\rA`\0a\x162V[V[a\rKa\x15\xD8V[a\r\x89`\x99\x82`\x08\x81\x10a\raWa\raa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x14\x95V[a\r\xCEW`\x01a\x02\xB1`\0\x82\x82Ta\r\xA1\x91\x90a(bV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\r\xBAWa\r\xBAa\"\x8BV[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[`@Q\x81\x81R\x7F\n\x17\xADC\xB5\x08^U\x1CpSB\x9F\x99h\xAC\xD9\x86\x94\x9FN\x90uZ\x16\xD9\xD6\xA4\xCC\xB0\xE1Z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0E\x0Ca\x15\xD8V[a\x0E\x17\x83\x83\x83a\x14\xAFV[PPPV[`\0a\x06N\x83\x83a\x16\x9CV[`\0\x80a\x0E4\x83a\x0BDV[\x90P`\0a\x0EB\x86\x86a\x16\x9CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\x0Eda\x15\xD8V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03BV[a\x0E\xE9\x81a\x162V[PV[`\0\x80\x80[`\x08\x81\x10\x15a\x0FPW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\x0F?Wa\x0F!`\x99\x83`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x0F1WP`\0\x94\x93PPPPV[a\x0F<`\x01\x84a\"\xFEV[\x92P[Pa\x0FI\x81a#\x16V[\x90Pa\x0E\xF1V[Pa\x02\xB1Ta\x0F`\x82`\x02a(yV[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\x0F\xB6WPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x0F\x9FWa\x0F\x9Fa\"\x8BV[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\x0F\xD2W`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x06\x0FWa\x06\x0Fa\"\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x10\x8FWa\x10\x02`\x02`\xFF\x86\x16\x83\x1Ca(\x98V[`\xFF\x16`\x01\x03a\x10\x7FWa\x10\"`\x99\x82`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x10,W`\0\x80\xFD[a\x10xa\x10=`\x01\x83\x1B\x86\x18a\x0FhV[`\x99\x83`\x08\x81\x10a\x10PWa\x10Pa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x16\xC0V[\x91Pa\x10\x8FV[a\x10\x88\x81a#\x16V[\x90Pa\x0F\xE9V[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x10\xA8Wa\x10\xA8a\"\x8BV[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x10\xDDWa\x10\xDDa\"\x8BV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x11:\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a(bV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x11p\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a(bV[\x90P`\0\x82\x90\x03a\x11\x80W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\xD4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x127W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x12\xE6WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x13\x19W\x80a\x13\x03\x81a(\xBAV[\x91Pa\x13\x12\x90P`\n\x83a(\xE0V[\x91Pa\x12\xEAV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13=Wa\x13=a\x1D\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13gW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0B<Wa\x13\x85`\x01\x83a)\x06V[\x91Pa\x13\x92`\n\x86a).V[a\x13\x9D\x90`0a)TV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x13\xBBWa\x13\xBBa\"\x8BV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x13\xF5`\n\x86a(\xE0V[\x94Pa\x13kV[`\0\x80`\0a\x14\x0C\x86\x86\x86a\x18\x97V[\x91P\x91Pa\x14\x19\x81a\x18\xE9V[P\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x14\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03BV[a\rAa\x1A3V[\x80Q`\0\x90\x15\x80\x15a\x14\xA9WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x14\xBCW`\0\x80\xFD[a\x14\xD2`\x99\x84`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x14\xF1W`\x01a\x02\xB1`\0\x82\x82Ta\x14\xEB\x91\x90a\"\xFEV[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x15\x19Wa\x15\x19a\"\x8BV[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x15\x91W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x15XWa\x15Xa\"\x8BV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x15\x89\x91\x90a\"\xFEV[\x17\x90Pa\x157V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\xDB\x88%\xFD\x9F\x88\xB7\xE1\xE4\xBD\xFD\x13\xC9\x17`\xF4\xA1\xD3X\x9Aa!\x1C5A1J-h``@\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03BV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x16\xAB\x85\x85a\x1A\xA7V[\x91P\x91Pa\x16\xB8\x81a\x18\xE9V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x16\xDD\x83a\x14\x95V[\x15a\x16\xE9WP\x80a\x14\xA9V[a\x16\xF2\x82a\x14\x95V[\x15a\x16\xFEWP\x81a\x14\xA9V[\x81Q\x83Q`\0\x91\x90\x03a\x17\x8FW\x82` \x01Q\x84` \x01Q\x14a\x175WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14\xA9V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x17\x86d\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x17z`\x02d\x01\0\0\x03\xD0\x19a(bV[d\x01\0\0\x03\xD0\x19a\x1A\xECV[\x82\t\x90Pa\x17\xECV[d\x01\0\0\x03\xD0\x19a\x17\xC6d\x01\0\0\x03\xD0\x19\x86Qa\x17\xB2\x90d\x01\0\0\x03\xD0\x19a(bV[\x86Q\x08a\x17z`\x02d\x01\0\0\x03\xD0\x19a(bV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x17\xE2\x90d\x01\0\0\x03\xD0\x19a(bV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x18\x13\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x180\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x18M\x83d\x01\0\0\x03\xD0\x19a(bV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x18z\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81a\x18\xCD`\xFF\x86\x90\x1C`\x1Ba\"\xFEV[\x90Pa\x18\xDB\x87\x82\x88\x85a\x1BBV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x81`\x04\x81\x11\x15a\x18\xFDWa\x18\xFDa#/V[\x03a\x19\x05WPV[`\x01\x81`\x04\x81\x11\x15a\x19\x19Wa\x19\x19a#/V[\x03a\x19fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`\x02\x81`\x04\x81\x11\x15a\x19zWa\x19za#/V[\x03a\x19\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x03BV[`\x03\x81`\x04\x81\x11\x15a\x19\xDBWa\x19\xDBa#/V[\x03a\x0E\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x03BV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03BV[a\rA3a\x162V[`\0\x80\x82Q`A\x03a\x1A\xDDW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1A\xD1\x87\x82\x85\x85a\x1BBV[\x94P\x94PPPPa\x1A\xE5V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0`\x01[\x83\x15a\x0B<Wa\x1B\x02`\x02\x85a!\xA9V[`\x01\x03a\x1B\x1DW\x82\x80a\x1B\x17Wa\x1B\x17a!\x93V[\x85\x82\t\x90P[\x82\x80a\x1B+Wa\x1B+a!\x93V[\x85\x86\t\x94Pa\x1B;`\x02\x85a)\x7FV[\x93Pa\x1A\xF1V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1ByWP`\0\x90P`\x03a\x1B\xFDV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\xCDW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xF6W`\0`\x01\x92P\x92PPa\x1B\xFDV[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C2W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x1CP``\x86\x01a\x1C\x06V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1CnW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1C\x8FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xA7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1A\xE5W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1C\xEFW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\x07W`\0\x80\xFD[a\x1D\x13\x89\x83\x8A\x01a\x1C}V[\x90\x97P\x95P\x85\x91Pa\x1D'` \x89\x01a\x1C\xBFV[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1D=W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1DQW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1D`W`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1DuW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9AW`\0\x80\xFD[a\x1D\xA3\x82a\x1C\x06V[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xB6Wa\x1E\xB6a\x1D\xC1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1E\xD0W`\0\x80\xFD[a\x1E\xD8a\x1D\xD7V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1F\x04W`\0\x80\xFD[\x845\x93Pa\x1F\x14` \x86\x01a\x1D\xAAV[\x92P`@\x85\x015\x91Pa\x1CP\x86``\x87\x01a\x1E\xBEV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F?W`\0\x80\xFD[\x835`!\x81\x10a\x1FNW`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FjW`\0\x80\xFD[a\x1Fv\x86\x82\x87\x01a\x1C}V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x1F\x97W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x1F\xA6W`\0\x80\xFD[a\x1F\xAEa\x1E\0V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x1F\xC0W`\0\x80\xFD[\x84[\x83\x81\x10\x15a \x01W`@\x81\x88\x03\x12\x15a\x1F\xDBW`\0\x80\x81\xFD[a\x1F\xE3a\x1D\xD7V[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x1F\xC2V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a \x1DW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a 9W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a aW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a {Wa {a\x1D\xC1V[a \x8E`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x8DV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \xA3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a \xD6W`\0\x80\xFD[\x845\x93Pa \xE6` \x86\x01a\x1D\xAAV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\tW`\0\x80\xFD[a!\x15\x87\x82\x88\x01a PV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!TW`\0\x80\xFD[a!`\x86\x82\x87\x01a PV[\x92PPa!o`@\x85\x01a\x1C\x06V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\x8AW`\0\x80\xFD[a\x1D\xA3\x82a\x1D\xAAV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!\xB8Wa!\xB8a!\x93V[P\x06\x90V[`\0[\x83\x81\x10\x15a!\xD8W\x81\x81\x01Q\x83\x82\x01R` \x01a!\xC0V[\x83\x81\x11\x15a!\xE7W`\0\x84\x84\x01R[PPPPV[a\x029`\xF5\x1B\x81R`\0\x83Qa\"\n\x81`\x02\x85\x01` \x88\x01a!\xBDV[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\"+\x81`\x03\x84\x01` \x88\x01a!\xBDV[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\"V\x81`@\x85\x01` \x87\x01a!\xBDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\"\xB8W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xD3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1A\xE5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a#\x11Wa#\x11a\"\xE8V[P\x01\x90V[`\0`\x01\x82\x01a#(Wa#(a\"\xE8V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[\x805`\x0F\x81\x90\x0B\x81\x14a\x1C\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#}W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\x95W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a#\xAAW`\0\x80\xFD[a#\xB2a\x1D\xD7V[`\xC0\x82\x12\x15a#\xC0W`\0\x80\xFD[a#\xC8a\x1E$V[\x91P\x835\x82R` \x84\x015` \x83\x01Ra#\xE4`@\x85\x01a#EV[`@\x83\x01R``\x84\x015\x80\x15\x15\x81\x14a#\xFCW`\0\x80\xFD[``\x83\x01Ra$\r`\x80\x85\x01a#YV[`\x80\x83\x01Ra$\x1E`\xA0\x85\x01a\x1C\xBFV[`\xA0\x83\x01R\x90\x81R`\xC0\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[a$E\x87\x83\x86\x01a PV[` \x82\x01R\x96\x95PPPPPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$}W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\x95W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a$\xAAW`\0\x80\xFD[a$\xB2a\x1D\xD7V[`\x80\x82\x12\x15a$\xC0W`\0\x80\xFD[a$\xC8a\x1EGV[\x91P\x835\x82Ra$\xDA` \x85\x01a#EV[` \x83\x01Ra$\xEB`@\x85\x01a$TV[`@\x83\x01Ra$\xFC``\x85\x01a\x1C\xBFV[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[`\0\x81\x83\x03a\x01 \x81\x12\x15a%+W`\0\x80\xFD[a%3a\x1EjV[`\xC0\x82\x12\x15a%AW`\0\x80\xFD[a%Ia\x1E$V[\x91P\x835\x82Ra%[` \x85\x01a#EV[` \x83\x01Ra%l`@\x85\x01a$TV[`@\x83\x01Ra%}``\x85\x01a\x1C\xBFV[``\x83\x01Ra%\x8E`\x80\x85\x01a\x1D\xAAV[`\x80\x83\x01Ra%\x9F`\xA0\x85\x01a$TV[`\xA0\x83\x01R\x81\x81Ra%\xB4\x85`\xC0\x86\x01a\x1E\xBEV[` \x82\x01Ra%\xC6a\x01\0\x85\x01a#YV[`@\x82\x01R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a%\xE4W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\0Wa&\0a\x1D\xC1V[\x81`\x05\x1Ba&\x0F\x82\x82\x01a\x1E\x8DV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a&)W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a&OWa&@\x83a#YV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a&/V[\x97\x96PPPPPPPV[`\0\x81\x83\x03`\xC0\x81\x12\x15a&mW`\0\x80\xFD[a&ua\x1EGV[\x91P``\x81\x12\x15a&\x85W`\0\x80\xFD[Pa&\x8Ea\x1EjV[\x825\x81Ra&\x9E` \x84\x01a$TV[` \x82\x01Ra&\xAF`@\x84\x01a\x1C\xBFV[`@\x82\x01R\x81R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xD1W`\0\x80\xFD[a&\xDD\x85\x83\x86\x01a PV[` \x84\x01Ra&\xEE`\x80\x85\x01a#YV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a'\x07W`\0\x80\xFD[Pa'\x14\x84\x82\x85\x01a%\xD3V[``\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'IW`\0\x80\xFD[a\x0B<\x84\x82\x85\x01a&ZV[`\0` \x82\x84\x03\x12\x15a'gW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'\x7FW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\x80\x81\x12\x15a'\x94W`\0\x80\xFD[a'\x9Ca\x1D\xD7V[``\x82\x12\x15a'\xAAW`\0\x80\xFD[a'\xB2a\x1EjV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra'\xCE`@\x85\x01a\x1C\xBFV[`@\x83\x01R\x90\x81R``\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a'\xFBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x13W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a((W`\0\x80\xFD[a(0a\x1D\xD7V[`\x80\x82\x12\x15a(>W`\0\x80\xFD[a(Fa\x1EGV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra$\xEB`@\x85\x01a$TV[`\0\x82\x82\x10\x15a(tWa(ta\"\xE8V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a(\x93Wa(\x93a\"\xE8V[P\x02\x90V[`\0`\xFF\x83\x16\x80a(\xABWa(\xABa!\x93V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a(\xD6Wa(\xD6a\"\xE8V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a(\xFAWa(\xFAa!\x93V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)&Wa)&a\"\xE8V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a)HWa)Ha!\x93V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a)vWa)va\"\xE8V[\x01\x94\x93PPPPV[`\0\x82a)\x8EWa)\x8Ea!\x93V[P\x04\x90V\xFEWithdrawCollateralV2(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce,address sendTo,uint128 appendix)TransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)BurnNlp(bytes32 sender,uint128 nlpAmount,uint64 nonce)MintNlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x90UJ6\x0B\x11g\xC8\xC3h\x7F\x97\xD2\xB6>^M\x07Z\xD67M\x1Dy=)\x1A\xAB\xB6\xD4\"\xE7dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static VERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x1BW`\x005`\xE0\x1C\x80c_\xCB}X\x11a\0\xB2W\x80c\xBE\x13\xBA\xC4\x11a\0\x81W\x80c\xE4\x93\xEF\x8C\x11a\0fW\x80c\xE4\x93\xEF\x8C\x14a\x02_W\x80c\xEBvJ&\x14a\x02xW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9BW`\0\x80\xFD[\x80c\xBE\x13\xBA\xC4\x14a\x029W\x80c\xC9E\xD3Z\x14a\x02LW`\0\x80\xFD[\x80c_\xCB}X\x14a\x01\xFAW\x80cqP\x18\xA6\x14a\x02\rW\x80c\x8D\xA5\xCB[\x14a\x02\x15W\x80c\xBB\xEF\x84\xB4\x14a\x02&W`\0\x80\xFD[\x80cU\xE7g;\x11a\0\xEEW\x80cU\xE7g;\x14a\x01{W\x80cY\x02\xCA\x1B\x14a\x01\xA9W\x80c[P\xF3E\x14a\x01\xBCW\x80c]\x18\x16\xD9\x14a\x01\xCFW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\x01 W\x80c5Ob*\x14a\x015W\x80c<d\xC2\x15\x14a\x01UW\x80cUu}\xBF\x14a\x01hW[`\0\x80\xFD[a\x013a\x01.6`\x04a\x1C\x1CV[a\x02\xAEV[\0[a\x01Bd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x013a\x01c6`\x04a\x1C[V[a\x03RV[a\x013a\x01v6`\x04a\x1C\xD7V[a\x03\x9BV[a\x01\x8Ea\x01\x896`\x04a\x1D\x88V[a\x05\xE5V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x01LV[a\x013a\x01\xB76`\x04a\x1E\xEEV[a\x069V[a\x01Ba\x01\xCA6`\x04a\x1F*V[a\x06\xD0V[a\x01\xE2a\x01\xDD6`\x04a\x1D\x88V[a\x0BDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01LV[a\x013a\x02\x086`\x04a\x1F\x83V[a\x0B\x92V[a\x013a\r/V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xE2V[a\x013a\x0246`\x04a \x0BV[a\rCV[a\x013a\x02G6`\x04a $V[a\x0E\x04V[a\x013a\x02Z6`\x04a \xC0V[a\x0E\x1CV[a\x01Bp\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02\x8Ba\x02\x866`\x04a!!V[a\x0E(V[`@Q\x90\x15\x15\x81R` \x01a\x01LV[a\x013a\x02\xA96`\x04a!xV[a\x0E\\V[a\x02\xB7\x81a\x0E\xECV[a\x02\xC0W`\0\x80\xFD[`\0a\x02\xCB\x82a\x0FhV[\x90Pa\x02\xFA`\x02\x82` \x01Qa\x02\xE1\x91\x90a!\xA9V[\x15a\x02\xEDW`\x1Ca\x02\xF0V[`\x1B[\x82Q\x87\x87\x87a\x11\x05V[a\x03KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03[\x82a\x12\xB6V[a\x03d\x82a\x12\xB6V[`@Q` \x01a\x03u\x92\x91\x90a!\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03B\x91`\x04\x01a\"7V[a\x01\0\x81\x11\x15a\x03\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Ftoo many signatures\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x04\x10\x94\x93\x92\x91\x90a\"jV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x04g\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x05\x88W`\0\x86\x86\x83\x81\x81\x10a\x04\x9FWa\x04\x9Fa\"\x8BV[\x90P` \x02\x81\x01\x90a\x04\xB1\x91\x90a\"\xA1V[\x90P\x11\x15a\x05vWa\x04\xC4`\x01\x83a\"\xFEV[\x91Pa\x05*\x83\x87\x87\x84\x81\x81\x10a\x04\xDCWa\x04\xDCa\"\x8BV[\x90P` \x02\x81\x01\x90a\x04\xEE\x91\x90a\"\xA1V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x0E(\x91PPV[a\x05vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[\x80a\x05\x80\x81a#\x16V[\x91PPa\x04\x83V[Pa\x02\xB1T\x81\x14a\x05\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x06\x0FWa\x06\x0Fa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0a\x06N\x83\x83`\0\x01Q\x84` \x01Qa\x13\xFCV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x06\x8FWP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x06\x8FWP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x06\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03B\x91\x90a\"7V[PPPPPPV[`\0\x80\x80\x85` \x81\x11\x15a\x06\xE6Wa\x06\xE6a#/V[\x03a\x07\xA3W`\0a\x06\xF9\x84\x86\x01\x86a#kV[\x90P`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a*\x86`w\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PPa\x0B<V[`\x02\x85` \x81\x11\x15a\x07\xB7Wa\x07\xB7a#/V[\x03a\x08CW`\0a\x07\xCA\x84\x86\x01\x86a$kV[\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a+k`O\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x07\x85V[` \x85` \x81\x11\x15a\x08WWa\x08Wa#/V[\x03a\t\rW`\0a\x08j\x84\x86\x01\x86a%\x17V[\x90P`@Q\x80`\xA0\x01`@R\x80`q\x81R` \x01a)\x94`q\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x90\x94\x16\x90\x88\x01R`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x87\x01\x93\x90\x93Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x92\x85\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x84\x01R\x16`\xE0\x82\x01Ra\x01\0\x01a\x07\x85V[`\x15\x85` \x81\x11\x15a\t!Wa\t!a#/V[\x03a\t\x97W`\0a\t4\x84\x86\x01\x86a' V[\x90P`@Q\x80``\x01`@R\x80`8\x81R` \x01a+3`8\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\x16\x85` \x81\x11\x15a\t\xABWa\t\xABa#/V[\x03a\n!W`\0a\t\xBE\x84\x86\x01\x86a' V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a*\xFD`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\r\x85` \x81\x11\x15a\n5Wa\n5a#/V[\x03a\n\xA2W`\0a\nH\x84\x86\x01\x86a'UV[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a*P`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x07\x85V[`\x0F\x85` \x81\x11\x15a\n\xB6Wa\n\xB6a#/V[\x03a\x01\x1BW`\0a\n\xC9\x84\x86\x01\x86a'\xE9V[\x90P`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a*\x05`K\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x07\x85V[\x94\x93PPPPV[`\0\x80a\x0BP\x83a\x05\xE5V[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\x0Bs\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0B\xB2WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0B\xCCWP0;\x15\x80\x15a\x0B\xCCWP`\0T`\xFF\x16`\x01\x14[a\x0C>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03BV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0CaW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0Cia\x14\"V[`\0[`\x08\x81\x10\x15a\x0C\xE4Wa\x0C\x94\x83\x82`\x08\x81\x10a\x0C\x8AWa\x0C\x8Aa\"\x8BV[` \x02\x01Qa\x14\x95V[a\x0C\xD4Wa\x0C\xD4\x81\x84\x83`\x08\x81\x10a\x0C\xAEWa\x0C\xAEa\"\x8BV[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x0C\xC6Wa\x0C\xC6a\"\x8BV[` \x02\x01Q` \x01Qa\x14\xAFV[a\x0C\xDD\x81a#\x16V[\x90Pa\x0ClV[P\x80\x15a\r+W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\r7a\x15\xD8V[a\rA`\0a\x162V[V[a\rKa\x15\xD8V[a\r\x89`\x99\x82`\x08\x81\x10a\raWa\raa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x14\x95V[a\r\xCEW`\x01a\x02\xB1`\0\x82\x82Ta\r\xA1\x91\x90a(bV[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\r\xBAWa\r\xBAa\"\x8BV[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[`@Q\x81\x81R\x7F\n\x17\xADC\xB5\x08^U\x1CpSB\x9F\x99h\xAC\xD9\x86\x94\x9FN\x90uZ\x16\xD9\xD6\xA4\xCC\xB0\xE1Z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0E\x0Ca\x15\xD8V[a\x0E\x17\x83\x83\x83a\x14\xAFV[PPPV[`\0a\x06N\x83\x83a\x16\x9CV[`\0\x80a\x0E4\x83a\x0BDV[\x90P`\0a\x0EB\x86\x86a\x16\x9CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\x0Eda\x15\xD8V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03BV[a\x0E\xE9\x81a\x162V[PV[`\0\x80\x80[`\x08\x81\x10\x15a\x0FPW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\x0F?Wa\x0F!`\x99\x83`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x0F1WP`\0\x94\x93PPPPV[a\x0F<`\x01\x84a\"\xFEV[\x92P[Pa\x0FI\x81a#\x16V[\x90Pa\x0E\xF1V[Pa\x02\xB1Ta\x0F`\x82`\x02a(yV[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\x0F\xB6WPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x0F\x9FWa\x0F\x9Fa\"\x8BV[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\x0F\xD2W`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x06\x0FWa\x06\x0Fa\"\x8BV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x10\x8FWa\x10\x02`\x02`\xFF\x86\x16\x83\x1Ca(\x98V[`\xFF\x16`\x01\x03a\x10\x7FWa\x10\"`\x99\x82`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x10,W`\0\x80\xFD[a\x10xa\x10=`\x01\x83\x1B\x86\x18a\x0FhV[`\x99\x83`\x08\x81\x10a\x10PWa\x10Pa\"\x8BV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x16\xC0V[\x91Pa\x10\x8FV[a\x10\x88\x81a#\x16V[\x90Pa\x0F\xE9V[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x10\xA8Wa\x10\xA8a\"\x8BV[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x10\xDDWa\x10\xDDa\"\x8BV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x11:\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a(bV[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x11p\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a(bV[\x90P`\0\x82\x90\x03a\x11\x80W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x11\xD4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x127W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x12\xE6WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x13\x19W\x80a\x13\x03\x81a(\xBAV[\x91Pa\x13\x12\x90P`\n\x83a(\xE0V[\x91Pa\x12\xEAV[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13=Wa\x13=a\x1D\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13gW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\x0B<Wa\x13\x85`\x01\x83a)\x06V[\x91Pa\x13\x92`\n\x86a).V[a\x13\x9D\x90`0a)TV[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x13\xBBWa\x13\xBBa\"\x8BV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x13\xF5`\n\x86a(\xE0V[\x94Pa\x13kV[`\0\x80`\0a\x14\x0C\x86\x86\x86a\x18\x97V[\x91P\x91Pa\x14\x19\x81a\x18\xE9V[P\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x14\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03BV[a\rAa\x1A3V[\x80Q`\0\x90\x15\x80\x15a\x14\xA9WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x14\xBCW`\0\x80\xFD[a\x14\xD2`\x99\x84`\x08\x81\x10a\raWa\raa\"\x8BV[\x15a\x14\xF1W`\x01a\x02\xB1`\0\x82\x82Ta\x14\xEB\x91\x90a\"\xFEV[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x15\x19Wa\x15\x19a\"\x8BV[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x15\x91W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x15XWa\x15Xa\"\x8BV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x15\x89\x91\x90a\"\xFEV[\x17\x90Pa\x157V[P`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x7F\xDB\x88%\xFD\x9F\x88\xB7\xE1\xE4\xBD\xFD\x13\xC9\x17`\xF4\xA1\xD3X\x9Aa!\x1C5A1J-h``@\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03BV[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x16\xAB\x85\x85a\x1A\xA7V[\x91P\x91Pa\x16\xB8\x81a\x18\xE9V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x16\xDD\x83a\x14\x95V[\x15a\x16\xE9WP\x80a\x14\xA9V[a\x16\xF2\x82a\x14\x95V[\x15a\x16\xFEWP\x81a\x14\xA9V[\x81Q\x83Q`\0\x91\x90\x03a\x17\x8FW\x82` \x01Q\x84` \x01Q\x14a\x175WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14\xA9V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x17\x86d\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x17z`\x02d\x01\0\0\x03\xD0\x19a(bV[d\x01\0\0\x03\xD0\x19a\x1A\xECV[\x82\t\x90Pa\x17\xECV[d\x01\0\0\x03\xD0\x19a\x17\xC6d\x01\0\0\x03\xD0\x19\x86Qa\x17\xB2\x90d\x01\0\0\x03\xD0\x19a(bV[\x86Q\x08a\x17z`\x02d\x01\0\0\x03\xD0\x19a(bV[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x17\xE2\x90d\x01\0\0\x03\xD0\x19a(bV[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x18\x13\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x180\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x18M\x83d\x01\0\0\x03\xD0\x19a(bV[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x18z\x90d\x01\0\0\x03\xD0\x19a(bV[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81a\x18\xCD`\xFF\x86\x90\x1C`\x1Ba\"\xFEV[\x90Pa\x18\xDB\x87\x82\x88\x85a\x1BBV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x81`\x04\x81\x11\x15a\x18\xFDWa\x18\xFDa#/V[\x03a\x19\x05WPV[`\x01\x81`\x04\x81\x11\x15a\x19\x19Wa\x19\x19a#/V[\x03a\x19fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03BV[`\x02\x81`\x04\x81\x11\x15a\x19zWa\x19za#/V[\x03a\x19\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x03BV[`\x03\x81`\x04\x81\x11\x15a\x19\xDBWa\x19\xDBa#/V[\x03a\x0E\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x03BV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03BV[a\rA3a\x162V[`\0\x80\x82Q`A\x03a\x1A\xDDW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x1A\xD1\x87\x82\x85\x85a\x1BBV[\x94P\x94PPPPa\x1A\xE5V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0`\x01[\x83\x15a\x0B<Wa\x1B\x02`\x02\x85a!\xA9V[`\x01\x03a\x1B\x1DW\x82\x80a\x1B\x17Wa\x1B\x17a!\x93V[\x85\x82\t\x90P[\x82\x80a\x1B+Wa\x1B+a!\x93V[\x85\x86\t\x94Pa\x1B;`\x02\x85a)\x7FV[\x93Pa\x1A\xF1V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x1ByWP`\0\x90P`\x03a\x1B\xFDV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\xCDW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B\xF6W`\0`\x01\x92P\x92PPa\x1B\xFDV[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C2W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x1CP``\x86\x01a\x1C\x06V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1CnW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1C\x8FW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xA7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1A\xE5W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1C\xEFW`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1D\x07W`\0\x80\xFD[a\x1D\x13\x89\x83\x8A\x01a\x1C}V[\x90\x97P\x95P\x85\x91Pa\x1D'` \x89\x01a\x1C\xBFV[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1D=W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1DQW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1D`W`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1DuW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9AW`\0\x80\xFD[a\x1D\xA3\x82a\x1C\x06V[\x93\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@R\x90V[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1D\xFAWa\x1D\xFAa\x1D\xC1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xB6Wa\x1E\xB6a\x1D\xC1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1E\xD0W`\0\x80\xFD[a\x1E\xD8a\x1D\xD7V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\xA0\x85\x87\x03\x12\x15a\x1F\x04W`\0\x80\xFD[\x845\x93Pa\x1F\x14` \x86\x01a\x1D\xAAV[\x92P`@\x85\x015\x91Pa\x1CP\x86``\x87\x01a\x1E\xBEV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F?W`\0\x80\xFD[\x835`!\x81\x10a\x1FNW`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1FjW`\0\x80\xFD[a\x1Fv\x86\x82\x87\x01a\x1C}V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x1F\x97W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x1F\xA6W`\0\x80\xFD[a\x1F\xAEa\x1E\0V[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x1F\xC0W`\0\x80\xFD[\x84[\x83\x81\x10\x15a \x01W`@\x81\x88\x03\x12\x15a\x1F\xDBW`\0\x80\x81\xFD[a\x1F\xE3a\x1D\xD7V[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x1F\xC2V[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a \x1DW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a 9W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a aW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a {Wa {a\x1D\xC1V[a \x8E`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x8DV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a \xA3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a \xD6W`\0\x80\xFD[\x845\x93Pa \xE6` \x86\x01a\x1D\xAAV[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\tW`\0\x80\xFD[a!\x15\x87\x82\x88\x01a PV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!TW`\0\x80\xFD[a!`\x86\x82\x87\x01a PV[\x92PPa!o`@\x85\x01a\x1C\x06V[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\x8AW`\0\x80\xFD[a\x1D\xA3\x82a\x1D\xAAV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!\xB8Wa!\xB8a!\x93V[P\x06\x90V[`\0[\x83\x81\x10\x15a!\xD8W\x81\x81\x01Q\x83\x82\x01R` \x01a!\xC0V[\x83\x81\x11\x15a!\xE7W`\0\x84\x84\x01R[PPPPV[a\x029`\xF5\x1B\x81R`\0\x83Qa\"\n\x81`\x02\x85\x01` \x88\x01a!\xBDV[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\"+\x81`\x03\x84\x01` \x88\x01a!\xBDV[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\"V\x81`@\x85\x01` \x87\x01a!\xBDV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\"\xB8W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xD3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1A\xE5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a#\x11Wa#\x11a\"\xE8V[P\x01\x90V[`\0`\x01\x82\x01a#(Wa#(a\"\xE8V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[\x805`\x0F\x81\x90\x0B\x81\x14a\x1C\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a#}W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\x95W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a#\xAAW`\0\x80\xFD[a#\xB2a\x1D\xD7V[`\xC0\x82\x12\x15a#\xC0W`\0\x80\xFD[a#\xC8a\x1E$V[\x91P\x835\x82R` \x84\x015` \x83\x01Ra#\xE4`@\x85\x01a#EV[`@\x83\x01R``\x84\x015\x80\x15\x15\x81\x14a#\xFCW`\0\x80\xFD[``\x83\x01Ra$\r`\x80\x85\x01a#YV[`\x80\x83\x01Ra$\x1E`\xA0\x85\x01a\x1C\xBFV[`\xA0\x83\x01R\x90\x81R`\xC0\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[a$E\x87\x83\x86\x01a PV[` \x82\x01R\x96\x95PPPPPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1C\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$}W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\x95W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a$\xAAW`\0\x80\xFD[a$\xB2a\x1D\xD7V[`\x80\x82\x12\x15a$\xC0W`\0\x80\xFD[a$\xC8a\x1EGV[\x91P\x835\x82Ra$\xDA` \x85\x01a#EV[` \x83\x01Ra$\xEB`@\x85\x01a$TV[`@\x83\x01Ra$\xFC``\x85\x01a\x1C\xBFV[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[`\0\x81\x83\x03a\x01 \x81\x12\x15a%+W`\0\x80\xFD[a%3a\x1EjV[`\xC0\x82\x12\x15a%AW`\0\x80\xFD[a%Ia\x1E$V[\x91P\x835\x82Ra%[` \x85\x01a#EV[` \x83\x01Ra%l`@\x85\x01a$TV[`@\x83\x01Ra%}``\x85\x01a\x1C\xBFV[``\x83\x01Ra%\x8E`\x80\x85\x01a\x1D\xAAV[`\x80\x83\x01Ra%\x9F`\xA0\x85\x01a$TV[`\xA0\x83\x01R\x81\x81Ra%\xB4\x85`\xC0\x86\x01a\x1E\xBEV[` \x82\x01Ra%\xC6a\x01\0\x85\x01a#YV[`@\x82\x01R\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a%\xE4W`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\0Wa&\0a\x1D\xC1V[\x81`\x05\x1Ba&\x0F\x82\x82\x01a\x1E\x8DV[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a&)W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a&OWa&@\x83a#YV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a&/V[\x97\x96PPPPPPPV[`\0\x81\x83\x03`\xC0\x81\x12\x15a&mW`\0\x80\xFD[a&ua\x1EGV[\x91P``\x81\x12\x15a&\x85W`\0\x80\xFD[Pa&\x8Ea\x1EjV[\x825\x81Ra&\x9E` \x84\x01a$TV[` \x82\x01Ra&\xAF`@\x84\x01a\x1C\xBFV[`@\x82\x01R\x81R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xD1W`\0\x80\xFD[a&\xDD\x85\x83\x86\x01a PV[` \x84\x01Ra&\xEE`\x80\x85\x01a#YV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a'\x07W`\0\x80\xFD[Pa'\x14\x84\x82\x85\x01a%\xD3V[``\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'IW`\0\x80\xFD[a\x0B<\x84\x82\x85\x01a&ZV[`\0` \x82\x84\x03\x12\x15a'gW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'\x7FW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\x80\x81\x12\x15a'\x94W`\0\x80\xFD[a'\x9Ca\x1D\xD7V[``\x82\x12\x15a'\xAAW`\0\x80\xFD[a'\xB2a\x1EjV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra'\xCE`@\x85\x01a\x1C\xBFV[`@\x83\x01R\x90\x81R``\x83\x015\x90\x82\x82\x11\x15a$9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a'\xFBW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(\x13W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a((W`\0\x80\xFD[a(0a\x1D\xD7V[`\x80\x82\x12\x15a(>W`\0\x80\xFD[a(Fa\x1EGV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra$\xEB`@\x85\x01a$TV[`\0\x82\x82\x10\x15a(tWa(ta\"\xE8V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a(\x93Wa(\x93a\"\xE8V[P\x02\x90V[`\0`\xFF\x83\x16\x80a(\xABWa(\xABa!\x93V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a(\xD6Wa(\xD6a\"\xE8V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a(\xFAWa(\xFAa!\x93V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a)&Wa)&a\"\xE8V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a)HWa)Ha!\x93V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a)vWa)va\"\xE8V[\x01\x94\x93PPPPV[`\0\x82a)\x8EWa)\x8Ea!\x93V[P\x04\x90V\xFEWithdrawCollateralV2(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce,address sendTo,uint128 appendix)TransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)BurnNlp(bytes32 sender,uint128 nlpAmount,uint64 nonce)MintNlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \x90UJ6\x0B\x11g\xC8\xC3h\x7F\x97\xD2\xB6>^M\x07Z\xD67M\x1Dy=)\x1A\xAB\xB6\xD4\"\xE7dsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static VERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Verifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Verifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Verifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Verifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Verifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Verifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Verifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                VERIFIER_ABI.clone(),
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
                VERIFIER_ABI.clone(),
                VERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `Q` (0xe493ef8c) function
        pub fn q(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 147, 239, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_P` (0x354f622a) function
        pub fn p(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([53, 79, 98, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assignPubKey` (0xbe13bac4) function
        pub fn assign_pub_key(
            &self,
            i: ::ethers::core::types::U256,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 19, 186, 196], (i, x, y))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkIndividualSignature` (0xeb764a26) function
        pub fn check_individual_signature(
            &self,
            digest: [u8; 32],
            signature: ::ethers::core::types::Bytes,
            signer_index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 118, 74, 38], (digest, signature, signer_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeDigest` (0x5b50f345) function
        pub fn compute_digest(
            &self,
            tx_type: u8,
            transaction_body: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([91, 80, 243, 69], (tx_type, transaction_body))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deletePubkey` (0xbbef84b4) function
        pub fn delete_pubkey(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 239, 132, 180], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPubkey` (0x55e7673b) function
        pub fn get_pubkey(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, Point> {
            self.0
                .method_hash([85, 231, 103, 59], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPubkeyAddress` (0x5d1816d9) function
        pub fn get_pubkey_address(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 24, 22, 217], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x5fcb7d58) function
        pub fn initialize(
            &self,
            initial_set: [Point; 8],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 203, 125, 88], initial_set)
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
        ///Calls the contract's `requireValidSignature` (0x2ac478b6) function
        pub fn require_valid_signature(
            &self,
            message: [u8; 32],
            e: [u8; 32],
            s: [u8; 32],
            signer_bitmask: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 196, 120, 182], (message, e, s, signer_bitmask))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requireValidTxSignatures` (0x55757dbf) function
        pub fn require_valid_tx_signatures(
            &self,
            txn: ::ethers::core::types::Bytes,
            idx: u64,
            signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 117, 125, 191], (txn, idx, signatures))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertGasInfo` (0x3c64c215) function
        pub fn revert_gas_info(
            &self,
            i: ::ethers::core::types::U256,
            gas_used: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 100, 194, 21], (i, gas_used))
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
        ///Calls the contract's `validateCompactSignature` (0x5902ca1b) function
        pub fn validate_compact_signature(
            &self,
            sender: [u8; 32],
            linked_signer: ::ethers::core::types::Address,
            digest: [u8; 32],
            signature: CompactSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 2, 202, 27], (sender, linked_signer, digest, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSignature` (0xc945d35a) function
        pub fn validate_signature(
            &self,
            sender: [u8; 32],
            linked_signer: ::ethers::core::types::Address,
            digest: [u8; 32],
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 69, 211, 90],
                    (sender, linked_signer, digest, signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssignPubKey` event
        pub fn assign_pub_key_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AssignPubKeyFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DeletePubkey` event
        pub fn delete_pubkey_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeletePubkeyFilter>
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VerifierEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Verifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "AssignPubKey", abi = "AssignPubKey(uint256,uint256,uint256)")]
    pub struct AssignPubKeyFilter {
        pub i: ::ethers::core::types::U256,
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "DeletePubkey", abi = "DeletePubkey(uint256)")]
    pub struct DeletePubkeyFilter {
        pub index: ::ethers::core::types::U256,
    }
    #[derive(
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
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifierEvents {
        AssignPubKeyFilter(AssignPubKeyFilter),
        DeletePubkeyFilter(DeletePubkeyFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for VerifierEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssignPubKeyFilter::decode_log(log) {
                return Ok(VerifierEvents::AssignPubKeyFilter(decoded));
            }
            if let Ok(decoded) = DeletePubkeyFilter::decode_log(log) {
                return Ok(VerifierEvents::DeletePubkeyFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(VerifierEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VerifierEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VerifierEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssignPubKeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkeyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssignPubKeyFilter> for VerifierEvents {
        fn from(value: AssignPubKeyFilter) -> Self {
            Self::AssignPubKeyFilter(value)
        }
    }
    impl ::core::convert::From<DeletePubkeyFilter> for VerifierEvents {
        fn from(value: DeletePubkeyFilter) -> Self {
            Self::DeletePubkeyFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for VerifierEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for VerifierEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `Q` function with signature `Q()` and selector `0xe493ef8c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "Q", abi = "Q()")]
    pub struct QCall;
    ///Container type for all input parameters for the `_P` function with signature `_P()` and selector `0x354f622a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "_P", abi = "_P()")]
    pub struct PCall;
    ///Container type for all input parameters for the `assignPubKey` function with signature `assignPubKey(uint256,uint256,uint256)` and selector `0xbe13bac4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "assignPubKey", abi = "assignPubKey(uint256,uint256,uint256)")]
    pub struct AssignPubKeyCall {
        pub i: ::ethers::core::types::U256,
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkIndividualSignature` function with signature `checkIndividualSignature(bytes32,bytes,uint8)` and selector `0xeb764a26`
    #[derive(
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
        name = "checkIndividualSignature",
        abi = "checkIndividualSignature(bytes32,bytes,uint8)"
    )]
    pub struct CheckIndividualSignatureCall {
        pub digest: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
        pub signer_index: u8,
    }
    ///Container type for all input parameters for the `computeDigest` function with signature `computeDigest(uint8,bytes)` and selector `0x5b50f345`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "computeDigest", abi = "computeDigest(uint8,bytes)")]
    pub struct ComputeDigestCall {
        pub tx_type: u8,
        pub transaction_body: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deletePubkey` function with signature `deletePubkey(uint256)` and selector `0xbbef84b4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deletePubkey", abi = "deletePubkey(uint256)")]
    pub struct DeletePubkeyCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPubkey` function with signature `getPubkey(uint8)` and selector `0x55e7673b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPubkey", abi = "getPubkey(uint8)")]
    pub struct GetPubkeyCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `getPubkeyAddress` function with signature `getPubkeyAddress(uint8)` and selector `0x5d1816d9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPubkeyAddress", abi = "getPubkeyAddress(uint8)")]
    pub struct GetPubkeyAddressCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize((uint256,uint256)[8])` and selector `0x5fcb7d58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize((uint256,uint256)[8])")]
    pub struct InitializeCall {
        pub initial_set: [Point; 8],
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
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
    ///Container type for all input parameters for the `requireValidSignature` function with signature `requireValidSignature(bytes32,bytes32,bytes32,uint8)` and selector `0x2ac478b6`
    #[derive(
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
        name = "requireValidSignature",
        abi = "requireValidSignature(bytes32,bytes32,bytes32,uint8)"
    )]
    pub struct RequireValidSignatureCall {
        pub message: [u8; 32],
        pub e: [u8; 32],
        pub s: [u8; 32],
        pub signer_bitmask: u8,
    }
    ///Container type for all input parameters for the `requireValidTxSignatures` function with signature `requireValidTxSignatures(bytes,uint64,bytes[])` and selector `0x55757dbf`
    #[derive(
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
        name = "requireValidTxSignatures",
        abi = "requireValidTxSignatures(bytes,uint64,bytes[])"
    )]
    pub struct RequireValidTxSignaturesCall {
        pub txn: ::ethers::core::types::Bytes,
        pub idx: u64,
        pub signatures: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `revertGasInfo` function with signature `revertGasInfo(uint256,uint256)` and selector `0x3c64c215`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "revertGasInfo", abi = "revertGasInfo(uint256,uint256)")]
    pub struct RevertGasInfoCall {
        pub i: ::ethers::core::types::U256,
        pub gas_used: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
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
    ///Container type for all input parameters for the `validateCompactSignature` function with signature `validateCompactSignature(bytes32,address,bytes32,(bytes32,bytes32))` and selector `0x5902ca1b`
    #[derive(
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
        name = "validateCompactSignature",
        abi = "validateCompactSignature(bytes32,address,bytes32,(bytes32,bytes32))"
    )]
    pub struct ValidateCompactSignatureCall {
        pub sender: [u8; 32],
        pub linked_signer: ::ethers::core::types::Address,
        pub digest: [u8; 32],
        pub signature: CompactSignature,
    }
    ///Container type for all input parameters for the `validateSignature` function with signature `validateSignature(bytes32,address,bytes32,bytes)` and selector `0xc945d35a`
    #[derive(
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
        name = "validateSignature",
        abi = "validateSignature(bytes32,address,bytes32,bytes)"
    )]
    pub struct ValidateSignatureCall {
        pub sender: [u8; 32],
        pub linked_signer: ::ethers::core::types::Address,
        pub digest: [u8; 32],
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifierCalls {
        Q(QCall),
        P(PCall),
        AssignPubKey(AssignPubKeyCall),
        CheckIndividualSignature(CheckIndividualSignatureCall),
        ComputeDigest(ComputeDigestCall),
        DeletePubkey(DeletePubkeyCall),
        GetPubkey(GetPubkeyCall),
        GetPubkeyAddress(GetPubkeyAddressCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequireValidSignature(RequireValidSignatureCall),
        RequireValidTxSignatures(RequireValidTxSignaturesCall),
        RevertGasInfo(RevertGasInfoCall),
        TransferOwnership(TransferOwnershipCall),
        ValidateCompactSignature(ValidateCompactSignatureCall),
        ValidateSignature(ValidateSignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for VerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <QCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Q(decoded));
            }
            if let Ok(decoded) = <PCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::P(decoded));
            }
            if let Ok(decoded) = <AssignPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <CheckIndividualSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckIndividualSignature(decoded));
            }
            if let Ok(decoded) = <ComputeDigestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeDigest(decoded));
            }
            if let Ok(decoded) = <DeletePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeletePubkey(decoded));
            }
            if let Ok(decoded) = <GetPubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPubkey(decoded));
            }
            if let Ok(decoded) =
                <GetPubkeyAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPubkeyAddress(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequireValidSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequireValidSignature(decoded));
            }
            if let Ok(decoded) =
                <RequireValidTxSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequireValidTxSignatures(decoded));
            }
            if let Ok(decoded) = <RevertGasInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertGasInfo(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ValidateCompactSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateCompactSignature(decoded));
            }
            if let Ok(decoded) =
                <ValidateSignatureCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Q(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::P(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssignPubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckIndividualSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeDigest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeletePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPubkeyAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequireValidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequireValidTxSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertGasInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateCompactSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for VerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Q(element) => ::core::fmt::Display::fmt(element, f),
                Self::P(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckIndividualSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeDigest(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPubkeyAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequireValidTxSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertGasInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateCompactSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<QCall> for VerifierCalls {
        fn from(value: QCall) -> Self {
            Self::Q(value)
        }
    }
    impl ::core::convert::From<PCall> for VerifierCalls {
        fn from(value: PCall) -> Self {
            Self::P(value)
        }
    }
    impl ::core::convert::From<AssignPubKeyCall> for VerifierCalls {
        fn from(value: AssignPubKeyCall) -> Self {
            Self::AssignPubKey(value)
        }
    }
    impl ::core::convert::From<CheckIndividualSignatureCall> for VerifierCalls {
        fn from(value: CheckIndividualSignatureCall) -> Self {
            Self::CheckIndividualSignature(value)
        }
    }
    impl ::core::convert::From<ComputeDigestCall> for VerifierCalls {
        fn from(value: ComputeDigestCall) -> Self {
            Self::ComputeDigest(value)
        }
    }
    impl ::core::convert::From<DeletePubkeyCall> for VerifierCalls {
        fn from(value: DeletePubkeyCall) -> Self {
            Self::DeletePubkey(value)
        }
    }
    impl ::core::convert::From<GetPubkeyCall> for VerifierCalls {
        fn from(value: GetPubkeyCall) -> Self {
            Self::GetPubkey(value)
        }
    }
    impl ::core::convert::From<GetPubkeyAddressCall> for VerifierCalls {
        fn from(value: GetPubkeyAddressCall) -> Self {
            Self::GetPubkeyAddress(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for VerifierCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VerifierCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VerifierCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequireValidSignatureCall> for VerifierCalls {
        fn from(value: RequireValidSignatureCall) -> Self {
            Self::RequireValidSignature(value)
        }
    }
    impl ::core::convert::From<RequireValidTxSignaturesCall> for VerifierCalls {
        fn from(value: RequireValidTxSignaturesCall) -> Self {
            Self::RequireValidTxSignatures(value)
        }
    }
    impl ::core::convert::From<RevertGasInfoCall> for VerifierCalls {
        fn from(value: RevertGasInfoCall) -> Self {
            Self::RevertGasInfo(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VerifierCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ValidateCompactSignatureCall> for VerifierCalls {
        fn from(value: ValidateCompactSignatureCall) -> Self {
            Self::ValidateCompactSignature(value)
        }
    }
    impl ::core::convert::From<ValidateSignatureCall> for VerifierCalls {
        fn from(value: ValidateSignatureCall) -> Self {
            Self::ValidateSignature(value)
        }
    }
    ///Container type for all return fields from the `Q` function with signature `Q()` and selector `0xe493ef8c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct QReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `_P` function with signature `_P()` and selector `0x354f622a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `checkIndividualSignature` function with signature `checkIndividualSignature(bytes32,bytes,uint8)` and selector `0xeb764a26`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckIndividualSignatureReturn(pub bool);
    ///Container type for all return fields from the `computeDigest` function with signature `computeDigest(uint8,bytes)` and selector `0x5b50f345`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ComputeDigestReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getPubkey` function with signature `getPubkey(uint8)` and selector `0x55e7673b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPubkeyReturn(pub Point);
    ///Container type for all return fields from the `getPubkeyAddress` function with signature `getPubkeyAddress(uint8)` and selector `0x5d1816d9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPubkeyAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
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
    ///`CompactSignature(bytes32,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CompactSignature {
        pub r: [u8; 32],
        pub vs: [u8; 32],
    }
    ///`Point(uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Point {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
}
