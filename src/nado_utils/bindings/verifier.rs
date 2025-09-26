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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1Cb\0\0\"V[b\0\0\xE4V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\0\xE2W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[a(V\x80b\0\0\xF4`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xC9E\xD3Z\x11a\0fW\x80c\xC9E\xD3Z\x14a\x02\x1EW\x80c\xE4\x93\xEF\x8C\x14a\x021W\x80c\xEBvJ&\x14a\x02JW\x80c\xF2\xFD\xE3\x8B\x14a\x02mW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xDFW\x80c\x8D\xA5\xCB[\x14a\x01\xE7W\x80c\xBB\xEF\x84\xB4\x14a\x01\xF8W\x80c\xBE\x13\xBA\xC4\x14a\x02\x0BW`\0\x80\xFD[\x80cU\xE7g;\x11a\0\xD3W\x80cU\xE7g;\x14a\x01`W\x80c[P\xF3E\x14a\x01\x8EW\x80c]\x18\x16\xD9\x14a\x01\xA1W\x80c_\xCB}X\x14a\x01\xCCW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\x01\x05W\x80c5Ob*\x14a\x01\x1AW\x80c<d\xC2\x15\x14a\x01:W\x80cUu}\xBF\x14a\x01MW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\x1A V[a\x02\x80V[\0[a\x01'd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x18a\x01H6`\x04a\x1A_V[a\x03$V[a\x01\x18a\x01[6`\x04a\x1A\xDBV[a\x03mV[a\x01sa\x01n6`\x04a\x1B\x8CV[a\x05\xB7V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x011V[a\x01'a\x01\x9C6`\x04a\x1B\xAEV[a\x06\x0BV[a\x01\xB4a\x01\xAF6`\x04a\x1B\x8CV[a\t\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x011V[a\x01\x18a\x01\xDA6`\x04a\x1D\x04V[a\n\x03V[a\x01\x18a\x0B\xA0V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xB4V[a\x01\x18a\x02\x066`\x04a\x1D\x8CV[a\x0B\xB4V[a\x01\x18a\x02\x196`\x04a\x1D\xA5V[a\x0CBV[a\x01\x18a\x02,6`\x04a\x1EXV[a\x0CZV[a\x01'p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02]a\x02X6`\x04a\x1E\xB9V[a\x0C\xE8V[`@Q\x90\x15\x15\x81R` \x01a\x011V[a\x01\x18a\x02{6`\x04a\x1F\x10V[a\r\x1CV[a\x02\x89\x81a\r\xA9V[a\x02\x92W`\0\x80\xFD[`\0a\x02\x9D\x82a\x0E%V[\x90Pa\x02\xCC`\x02\x82` \x01Qa\x02\xB3\x91\x90a\x1FAV[\x15a\x02\xBFW`\x1Ca\x02\xC2V[`\x1B[\x82Q\x87\x87\x87a\x0F\xC2V[a\x03\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03-\x82a\x11sV[a\x036\x82a\x11sV[`@Q` \x01a\x03G\x92\x91\x90a\x1F\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\x14\x91`\x04\x01a\x1F\xCBV[a\x01\0\x81\x11\x15a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Ftoo many signatures\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03\xE2\x94\x93\x92\x91\x90a\x1F\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x049\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x05ZW`\0\x86\x86\x83\x81\x81\x10a\x04qWa\x04qa \x1FV[\x90P` \x02\x81\x01\x90a\x04\x83\x91\x90a 5V[\x90P\x11\x15a\x05HWa\x04\x96`\x01\x83a \x92V[\x91Pa\x04\xFC\x83\x87\x87\x84\x81\x81\x10a\x04\xAEWa\x04\xAEa \x1FV[\x90P` \x02\x81\x01\x90a\x04\xC0\x91\x90a 5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x0C\xE8\x91PPV[a\x05HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[\x80a\x05R\x81a \xAAV[\x91PPa\x04UV[Pa\x02\xB1T\x81\x14a\x05\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05\xE1Wa\x05\xE1a \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80\x80\x85`\x1D\x81\x11\x15a\x06!Wa\x06!a \xC3V[\x03a\x06\xDEW`\0a\x064\x84\x86\x01\x86a \xFFV[\x90P`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a&\xED`w\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PPa\t\xADV[`\x02\x85`\x1D\x81\x11\x15a\x06\xF2Wa\x06\xF2a \xC3V[\x03a\x07~W`\0a\x07\x05\x84\x86\x01\x86a!\xFFV[\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a'\xD2`O\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x06\xC0V[`\x15\x85`\x1D\x81\x11\x15a\x07\x92Wa\x07\x92a \xC3V[\x03a\x08\x08W`\0a\x07\xA5\x84\x86\x01\x86a#\xF8V[\x90P`@Q\x80``\x01`@R\x80`8\x81R` \x01a'\x9A`8\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\x16\x85`\x1D\x81\x11\x15a\x08\x1CWa\x08\x1Ca \xC3V[\x03a\x08\x92W`\0a\x08/\x84\x86\x01\x86a#\xF8V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a'd`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\r\x85`\x1D\x81\x11\x15a\x08\xA6Wa\x08\xA6a \xC3V[\x03a\t\x13W`\0a\x08\xB9\x84\x86\x01\x86a$-V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a&\xB7`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\x0F\x85`\x1D\x81\x11\x15a\t'Wa\t'a \xC3V[\x03a\x01\0W`\0a\t:\x84\x86\x01\x86a$\xC1V[\x90P`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a&l`K\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x06\xC0V[\x94\x93PPPPV[`\0\x80a\t\xC1\x83a\x05\xB7V[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\t\xE4\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n#WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\n=WP0;\x15\x80\x15a\n=WP`\0T`\xFF\x16`\x01\x14[a\n\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x14V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\n\xD2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\xDAa\x12\xB9V[`\0[`\x08\x81\x10\x15a\x0BUWa\x0B\x05\x83\x82`\x08\x81\x10a\n\xFBWa\n\xFBa \x1FV[` \x02\x01Qa\x13,V[a\x0BEWa\x0BE\x81\x84\x83`\x08\x81\x10a\x0B\x1FWa\x0B\x1Fa \x1FV[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x0B7Wa\x0B7a \x1FV[` \x02\x01Q` \x01Qa\x13FV[a\x0BN\x81a \xAAV[\x90Pa\n\xDDV[P\x80\x15a\x0B\x9CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x0B\xA8a\x14.V[a\x0B\xB2`\0a\x14\x88V[V[a\x0B\xBCa\x14.V[a\x0B\xFA`\x99\x82`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x13,V[a\x0C?W`\x01a\x02\xB1`\0\x82\x82Ta\x0C\x12\x91\x90a%:V[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x0C+Wa\x0C+a \x1FV[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x0CJa\x14.V[a\x0CU\x83\x83\x83a\x13FV[PPPV[`\0a\x0Cf\x83\x83a\x14\xF2V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0C\xA7WP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x0C\xA7WP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x0C\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x14\x91\x90a\x1F\xCBV[PPPPPPV[`\0\x80a\x0C\xF4\x83a\t\xB5V[\x90P`\0a\r\x02\x86\x86a\x14\xF2V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\r$a\x14.V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x14V[a\x0C?\x81a\x14\x88V[`\0\x80\x80[`\x08\x81\x10\x15a\x0E\rW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\r\xFCWa\r\xDE`\x99\x83`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\r\xEEWP`\0\x94\x93PPPPV[a\r\xF9`\x01\x84a \x92V[\x92P[Pa\x0E\x06\x81a \xAAV[\x90Pa\r\xAEV[Pa\x02\xB1Ta\x0E\x1D\x82`\x02a%QV[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\x0EsWPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x0E\\Wa\x0E\\a \x1FV[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\x0E\x8FW`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05\xE1Wa\x05\xE1a \x1FV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x0FLWa\x0E\xBF`\x02`\xFF\x86\x16\x83\x1Ca%pV[`\xFF\x16`\x01\x03a\x0F<Wa\x0E\xDF`\x99\x82`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\x0E\xE9W`\0\x80\xFD[a\x0F5a\x0E\xFA`\x01\x83\x1B\x86\x18a\x0E%V[`\x99\x83`\x08\x81\x10a\x0F\rWa\x0F\ra \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x15\x16V[\x91Pa\x0FLV[a\x0FE\x81a \xAAV[\x90Pa\x0E\xA6V[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0FeWa\x0Fea \x1FV[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0F\x9AWa\x0F\x9Aa \x1FV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0F\xF7\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a%:V[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x10-\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a%:V[\x90P`\0\x82\x90\x03a\x10=W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x10\x91W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x11\xA3WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x11\xD6W\x80a\x11\xC0\x81a%\x92V[\x91Pa\x11\xCF\x90P`\n\x83a%\xB8V[\x91Pa\x11\xA7V[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xFAWa\x11\xFAa\x1C\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12$W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\t\xADWa\x12B`\x01\x83a%\xDEV[\x91Pa\x12O`\n\x86a&\x06V[a\x12Z\x90`0a&,V[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x12xWa\x12xa \x1FV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x12\xB2`\n\x86a%\xB8V[\x94Pa\x12(V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x14V[a\x0B\xB2a\x16\xEDV[\x80Q`\0\x90\x15\x80\x15a\x13@WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x13SW`\0\x80\xFD[a\x13i`\x99\x84`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\x13\x88W`\x01a\x02\xB1`\0\x82\x82Ta\x13\x82\x91\x90a \x92V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x13\xB0Wa\x13\xB0a \x1FV[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x14(W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x13\xEFWa\x13\xEFa \x1FV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x14 \x91\x90a \x92V[\x17\x90Pa\x13\xCEV[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x14V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x15\x01\x85\x85a\x17aV[\x91P\x91Pa\x15\x0E\x81a\x17\xA6V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x153\x83a\x13,V[\x15a\x15?WP\x80a\x13@V[a\x15H\x82a\x13,V[\x15a\x15TWP\x81a\x13@V[\x81Q\x83Q`\0\x91\x90\x03a\x15\xE5W\x82` \x01Q\x84` \x01Q\x14a\x15\x8BWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13@V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x15\xDCd\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x15\xD0`\x02d\x01\0\0\x03\xD0\x19a%:V[d\x01\0\0\x03\xD0\x19a\x18\xF0V[\x82\t\x90Pa\x16BV[d\x01\0\0\x03\xD0\x19a\x16\x1Cd\x01\0\0\x03\xD0\x19\x86Qa\x16\x08\x90d\x01\0\0\x03\xD0\x19a%:V[\x86Q\x08a\x15\xD0`\x02d\x01\0\0\x03\xD0\x19a%:V[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x168\x90d\x01\0\0\x03\xD0\x19a%:V[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x16i\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x16\x86\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x16\xA3\x83d\x01\0\0\x03\xD0\x19a%:V[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x16\xD0\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x14V[a\x0B\xB23a\x14\x88V[`\0\x80\x82Q`A\x03a\x17\x97W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x17\x8B\x87\x82\x85\x85a\x19FV[\x94P\x94PPPPa\x17\x9FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x17\xBAWa\x17\xBAa \xC3V[\x03a\x17\xC2WPV[`\x01\x81`\x04\x81\x11\x15a\x17\xD6Wa\x17\xD6a \xC3V[\x03a\x18#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`\x02\x81`\x04\x81\x11\x15a\x187Wa\x187a \xC3V[\x03a\x18\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x03\x14V[`\x03\x81`\x04\x81\x11\x15a\x18\x98Wa\x18\x98a \xC3V[\x03a\x0C?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\x14V[`\0`\x01[\x83\x15a\t\xADWa\x19\x06`\x02\x85a\x1FAV[`\x01\x03a\x19!W\x82\x80a\x19\x1BWa\x19\x1Ba\x1F+V[\x85\x82\t\x90P[\x82\x80a\x19/Wa\x19/a\x1F+V[\x85\x86\t\x94Pa\x19?`\x02\x85a&WV[\x93Pa\x18\xF5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x19}WP`\0\x90P`\x03a\x1A\x01V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19\xD1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xFAW`\0`\x01\x92P\x92PPa\x1A\x01V[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A6W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x1AT``\x86\x01a\x1A\nV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1ArW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\x93W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x17\x9FW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1A\xF3W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x0BW`\0\x80\xFD[a\x1B\x17\x89\x83\x8A\x01a\x1A\x81V[\x90\x97P\x95P\x85\x91Pa\x1B+` \x89\x01a\x1A\xC3V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1BAW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1BUW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1BdW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1ByW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x9EW`\0\x80\xFD[a\x1B\xA7\x82a\x1A\nV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xC3W`\0\x80\xFD[\x835`\x1E\x81\x10a\x1B\xD2W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xEEW`\0\x80\xFD[a\x1B\xFA\x86\x82\x87\x01a\x1A\x81V[\x94\x97\x90\x96P\x93\x94PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1C\xFCWa\x1C\xFCa\x1C\x07V[`@R\x91\x90PV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x1D\x18W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x1D'W`\0\x80\xFD[a\x1D/a\x1C\x1DV[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x1DAW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x1D\x82W`@\x81\x88\x03\x12\x15a\x1D\\W`\0\x80\x81\xFD[a\x1Dda\x1CGV[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x1DCV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9EW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xBAW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x13Wa\x1E\x13a\x1C\x07V[a\x1E&`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xD3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E;W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1EnW`\0\x80\xFD[\x845\x93Pa\x1E~` \x86\x01a\x1D\xD1V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xA1W`\0\x80\xFD[a\x1E\xAD\x87\x82\x88\x01a\x1D\xE8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\xCEW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xECW`\0\x80\xFD[a\x1E\xF8\x86\x82\x87\x01a\x1D\xE8V[\x92PPa\x1F\x07`@\x85\x01a\x1A\nV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1F\"W`\0\x80\xFD[a\x1B\xA7\x82a\x1D\xD1V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1FPWa\x1FPa\x1F+V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1FpW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FXV[\x83\x81\x11\x15a\x14(WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x1F\x9E\x81`\x02\x85\x01` \x88\x01a\x1FUV[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x1F\xBF\x81`\x03\x84\x01` \x88\x01a\x1FUV[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1F\xEA\x81`@\x85\x01` \x87\x01a\x1FUV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a LW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a gW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17\x9FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a \xA5Wa \xA5a |V[P\x01\x90V[`\0`\x01\x82\x01a \xBCWa \xBCa |V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[\x805`\x0F\x81\x90\x0B\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!)W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a!>W`\0\x80\xFD[a!Fa\x1CGV[`\xC0\x82\x12\x15a!TW`\0\x80\xFD[a!\\a\x1CjV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra!x`@\x85\x01a \xD9V[`@\x83\x01R``\x84\x015\x80\x15\x15\x81\x14a!\x90W`\0\x80\xFD[``\x83\x01Ra!\xA1`\x80\x85\x01a \xEDV[`\x80\x83\x01Ra!\xB2`\xA0\x85\x01a\x1A\xC3V[`\xA0\x83\x01R\x90\x81R`\xC0\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[a!\xD9\x87\x83\x86\x01a\x1D\xE8V[` \x82\x01R\x96\x95PPPPPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\")W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a\">W`\0\x80\xFD[a\"Fa\x1CGV[`\x80\x82\x12\x15a\"TW`\0\x80\xFD[a\"\\a\x1C\x8DV[\x91P\x835\x82Ra\"n` \x85\x01a \xD9V[` \x83\x01Ra\"\x7F`@\x85\x01a!\xE8V[`@\x83\x01Ra\"\x90``\x85\x01a\x1A\xC3V[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\"\xBCW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xD8Wa\"\xD8a\x1C\x07V[\x81`\x05\x1Ba\"\xE7\x82\x82\x01a\x1C\xD3V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a#\x01W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a#'Wa#\x18\x83a \xEDV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a#\x07V[\x97\x96PPPPPPPV[`\0\x81\x83\x03`\xC0\x81\x12\x15a#EW`\0\x80\xFD[a#Ma\x1C\x8DV[\x91P``\x81\x12\x15a#]W`\0\x80\xFD[Pa#fa\x1C\xB0V[\x825\x81Ra#v` \x84\x01a!\xE8V[` \x82\x01Ra#\x87`@\x84\x01a\x1A\xC3V[`@\x82\x01R\x81R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xA9W`\0\x80\xFD[a#\xB5\x85\x83\x86\x01a\x1D\xE8V[` \x84\x01Ra#\xC6`\x80\x85\x01a \xEDV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a#\xDFW`\0\x80\xFD[Pa#\xEC\x84\x82\x85\x01a\"\xABV[``\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$\nW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[a\t\xAD\x84\x82\x85\x01a#2V[`\0` \x82\x84\x03\x12\x15a$?W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$WW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\x80\x81\x12\x15a$lW`\0\x80\xFD[a$ta\x1CGV[``\x82\x12\x15a$\x82W`\0\x80\xFD[a$\x8Aa\x1C\xB0V[\x91P\x835\x82R` \x84\x015` \x83\x01Ra$\xA6`@\x85\x01a\x1A\xC3V[`@\x83\x01R\x90\x81R``\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\xD3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xEBW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a%\0W`\0\x80\xFD[a%\x08a\x1CGV[`\x80\x82\x12\x15a%\x16W`\0\x80\xFD[a%\x1Ea\x1C\x8DV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra\"\x7F`@\x85\x01a!\xE8V[`\0\x82\x82\x10\x15a%LWa%La |V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a%kWa%ka |V[P\x02\x90V[`\0`\xFF\x83\x16\x80a%\x83Wa%\x83a\x1F+V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a%\xAEWa%\xAEa |V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a%\xD2Wa%\xD2a\x1F+V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a%\xFEWa%\xFEa |V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a& Wa& a\x1F+V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a&NWa&Na |V[\x01\x94\x93PPPPV[`\0\x82a&fWa&fa\x1F+V[P\x04\x90V\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)BurnNlp(bytes32 sender,uint128 nlpAmount,uint64 nonce)MintNlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \xE0=\xBD\\*\x01\xF7\r\xBEE \xD5\x9E\x1B\xA2\\\xB4\xC0+\x92w=\xEBU\0Tw\x83?\xD5\x12#dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static VERIFIER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xC9E\xD3Z\x11a\0fW\x80c\xC9E\xD3Z\x14a\x02\x1EW\x80c\xE4\x93\xEF\x8C\x14a\x021W\x80c\xEBvJ&\x14a\x02JW\x80c\xF2\xFD\xE3\x8B\x14a\x02mW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xDFW\x80c\x8D\xA5\xCB[\x14a\x01\xE7W\x80c\xBB\xEF\x84\xB4\x14a\x01\xF8W\x80c\xBE\x13\xBA\xC4\x14a\x02\x0BW`\0\x80\xFD[\x80cU\xE7g;\x11a\0\xD3W\x80cU\xE7g;\x14a\x01`W\x80c[P\xF3E\x14a\x01\x8EW\x80c]\x18\x16\xD9\x14a\x01\xA1W\x80c_\xCB}X\x14a\x01\xCCW`\0\x80\xFD[\x80c*\xC4x\xB6\x14a\x01\x05W\x80c5Ob*\x14a\x01\x1AW\x80c<d\xC2\x15\x14a\x01:W\x80cUu}\xBF\x14a\x01MW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\x1A V[a\x02\x80V[\0[a\x01'd\x01\0\0\x03\xD0\x19\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x18a\x01H6`\x04a\x1A_V[a\x03$V[a\x01\x18a\x01[6`\x04a\x1A\xDBV[a\x03mV[a\x01sa\x01n6`\x04a\x1B\x8CV[a\x05\xB7V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x011V[a\x01'a\x01\x9C6`\x04a\x1B\xAEV[a\x06\x0BV[a\x01\xB4a\x01\xAF6`\x04a\x1B\x8CV[a\t\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x011V[a\x01\x18a\x01\xDA6`\x04a\x1D\x04V[a\n\x03V[a\x01\x18a\x0B\xA0V[`gT`\x01`\x01`\xA0\x1B\x03\x16a\x01\xB4V[a\x01\x18a\x02\x066`\x04a\x1D\x8CV[a\x0B\xB4V[a\x01\x18a\x02\x196`\x04a\x1D\xA5V[a\x0CBV[a\x01\x18a\x02,6`\x04a\x1EXV[a\x0CZV[a\x01'p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x81V[a\x02]a\x02X6`\x04a\x1E\xB9V[a\x0C\xE8V[`@Q\x90\x15\x15\x81R` \x01a\x011V[a\x01\x18a\x02{6`\x04a\x1F\x10V[a\r\x1CV[a\x02\x89\x81a\r\xA9V[a\x02\x92W`\0\x80\xFD[`\0a\x02\x9D\x82a\x0E%V[\x90Pa\x02\xCC`\x02\x82` \x01Qa\x02\xB3\x91\x90a\x1FAV[\x15a\x02\xBFW`\x1Ca\x02\xC2V[`\x1B[\x82Q\x87\x87\x87a\x0F\xC2V[a\x03\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7FVerification failed\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[a\x03-\x82a\x11sV[a\x036\x82a\x11sV[`@Q` \x01a\x03G\x92\x91\x90a\x1F\x81V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\x14\x91`\x04\x01a\x1F\xCBV[a\x01\0\x81\x11\x15a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01R\x7Ftoo many signatures\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`\0F\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87`@Q` \x01a\x03\xE2\x94\x93\x92\x91\x90a\x1F\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x81`@Q` \x01a\x049\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80[\x84\x81\x10\x15a\x05ZW`\0\x86\x86\x83\x81\x81\x10a\x04qWa\x04qa \x1FV[\x90P` \x02\x81\x01\x90a\x04\x83\x91\x90a 5V[\x90P\x11\x15a\x05HWa\x04\x96`\x01\x83a \x92V[\x91Pa\x04\xFC\x83\x87\x87\x84\x81\x81\x10a\x04\xAEWa\x04\xAEa \x1FV[\x90P` \x02\x81\x01\x90a\x04\xC0\x91\x90a 5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa\x0C\xE8\x91PPV[a\x05HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01R\x7Finvalid signature\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[\x80a\x05R\x81a \xAAV[\x91PPa\x04UV[Pa\x02\xB1T\x81\x14a\x05\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7Fnot enough signatures\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x99\x82`\xFF\x16`\x08\x81\x10a\x05\xE1Wa\x05\xE1a \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x80\x80\x85`\x1D\x81\x11\x15a\x06!Wa\x06!a \xC3V[\x03a\x06\xDEW`\0a\x064\x84\x86\x01\x86a \xFFV[\x90P`@Q\x80`\xA0\x01`@R\x80`w\x81R` \x01a&\xED`w\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x80\x86\x01Q`\x80\x80\x88\x01Q`\xA0\x98\x89\x01Q\x86Q\x9B\x8C\x01\x9A\x90\x9AR\x94\x8A\x01\x96\x90\x96R\x90\x88\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x16\x92\x86\x01\x92\x90\x92R\x15\x15\x91\x84\x01\x91\x90\x91R`\x0F\x0B`\xC0\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PPa\t\xADV[`\x02\x85`\x1D\x81\x11\x15a\x06\xF2Wa\x06\xF2a \xC3V[\x03a\x07~W`\0a\x07\x05\x84\x86\x01\x86a!\xFFV[\x90P`@Q\x80`\x80\x01`@R\x80`O\x81R` \x01a'\xD2`O\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92Rc\xFF\xFF\xFF\xFF\x16\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x06\xC0V[`\x15\x85`\x1D\x81\x11\x15a\x07\x92Wa\x07\x92a \xC3V[\x03a\x08\x08W`\0a\x07\xA5\x84\x86\x01\x86a#\xF8V[\x90P`@Q\x80``\x01`@R\x80`8\x81R` \x01a'\x9A`8\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\x16\x85`\x1D\x81\x11\x15a\x08\x1CWa\x08\x1Ca \xC3V[\x03a\x08\x92W`\0a\x08/\x84\x86\x01\x86a#\xF8V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a'd`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R`\x01`\x01`\x80\x1B\x03\x16``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\r\x85`\x1D\x81\x11\x15a\x08\xA6Wa\x08\xA6a \xC3V[\x03a\t\x13W`\0a\x08\xB9\x84\x86\x01\x86a$-V[\x90P`@Q\x80``\x01`@R\x80`6\x81R` \x01a&\xB7`6\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x92\x83\x01Q\x83Q\x95\x86\x01\x94\x90\x94R\x91\x84\x01R``\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x01a\x06\xC0V[`\x0F\x85`\x1D\x81\x11\x15a\t'Wa\t'a \xC3V[\x03a\x01\0W`\0a\t:\x84\x86\x01\x86a$\xC1V[\x90P`@Q\x80`\x80\x01`@R\x80`K\x81R` \x01a&l`K\x919\x80Q` \x91\x82\x01 \x82Q\x80Q\x81\x84\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q\x97\x88\x01\x96\x90\x96R\x90\x86\x01\x92\x90\x92R\x91\x84\x01\x91\x90\x91R`\x01`\x01`\x80\x1B\x03\x16`\x80\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x82\x01R`\xC0\x01a\x06\xC0V[\x94\x93PPPPV[`\0\x80a\t\xC1\x83a\x05\xB7V[\x80Q` \x80\x83\x01Q`@Q\x93\x94Pa\t\xE4\x93\x90\x91\x01\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n#WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\n=WP0;\x15\x80\x15a\n=WP`\0T`\xFF\x16`\x01\x14[a\n\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x14V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\n\xD2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\xDAa\x12\xB9V[`\0[`\x08\x81\x10\x15a\x0BUWa\x0B\x05\x83\x82`\x08\x81\x10a\n\xFBWa\n\xFBa \x1FV[` \x02\x01Qa\x13,V[a\x0BEWa\x0BE\x81\x84\x83`\x08\x81\x10a\x0B\x1FWa\x0B\x1Fa \x1FV[` \x02\x01QQ\x85\x84`\x08\x81\x10a\x0B7Wa\x0B7a \x1FV[` \x02\x01Q` \x01Qa\x13FV[a\x0BN\x81a \xAAV[\x90Pa\n\xDDV[P\x80\x15a\x0B\x9CW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[a\x0B\xA8a\x14.V[a\x0B\xB2`\0a\x14\x88V[V[a\x0B\xBCa\x14.V[a\x0B\xFA`\x99\x82`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x13,V[a\x0C?W`\x01a\x02\xB1`\0\x82\x82Ta\x0C\x12\x91\x90a%:V[\x90\x91UP`\x99\x90P\x81`\x08\x81\x10a\x0C+Wa\x0C+a \x1FV[`\0`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x81\x81U`\x01\x01U[PV[a\x0CJa\x14.V[a\x0CU\x83\x83\x83a\x13FV[PPPV[`\0a\x0Cf\x83\x83a\x14\xF2V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0C\xA7WP`\x01`\x01`\xA0\x1B\x03\x81\x16``\x86\x90\x1C\x14\x80a\x0C\xA7WP\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aIS`\xF0\x1B\x81RP\x90a\x0C\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\x14\x91\x90a\x1F\xCBV[PPPPPPV[`\0\x80a\x0C\xF4\x83a\t\xB5V[\x90P`\0a\r\x02\x86\x86a\x14\xF2V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x92\x16\x91\x90\x91\x14\x95\x94PPPPPV[a\r$a\x14.V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x03\x14V[a\x0C?\x81a\x14\x88V[`\0\x80\x80[`\x08\x81\x10\x15a\x0E\rW`\x01`\xFF\x85\x16\x82\x1C\x81\x16\x14\x80\x15a\r\xFCWa\r\xDE`\x99\x83`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\r\xEEWP`\0\x94\x93PPPPV[a\r\xF9`\x01\x84a \x92V[\x92P[Pa\x0E\x06\x81a \xAAV[\x90Pa\r\xAEV[Pa\x02\xB1Ta\x0E\x1D\x82`\x02a%QV[\x11\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x82\x16\x15\x80a\x0EsWPa\x02\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x0E\\Wa\x0E\\a \x1FV[` \x81\x04\x90\x91\x01T`\xFF`\x1F\x90\x92\x16a\x01\0\n\x90\x04\x16[\x15a\x0E\x8FW`\xA9\x82`\xFF\x16a\x01\0\x81\x10a\x05\xE1Wa\x05\xE1a \x1FV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[`\x08\x81\x10\x15a\x0FLWa\x0E\xBF`\x02`\xFF\x86\x16\x83\x1Ca%pV[`\xFF\x16`\x01\x03a\x0F<Wa\x0E\xDF`\x99\x82`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\x0E\xE9W`\0\x80\xFD[a\x0F5a\x0E\xFA`\x01\x83\x1B\x86\x18a\x0E%V[`\x99\x83`\x08\x81\x10a\x0F\rWa\x0F\ra \x1FV[`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPPa\x15\x16V[\x91Pa\x0FLV[a\x0FE\x81a \xAAV[\x90Pa\x0E\xA6V[P\x80`\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0FeWa\x0Fea \x1FV[`\x02\x02\x01`\0\x82\x01Q\x81`\0\x01U` \x82\x01Q\x81`\x01\x01U\x90PP`\x01a\x02\xA9\x84`\xFF\x16a\x01\0\x81\x10a\x0F\x9AWa\x0F\x9Aa \x1FV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x91PP\x91\x90PV[`\0\x80p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x86\x84\ta\x0F\xF7\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a%:V[\x90P`\0p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19\x87\x86\ta\x10-\x90p\x01EQ#\x19P\xB7_\xC4@-\xA1s/\xC9\xBE\xBE\x19a%:V[\x90P`\0\x82\x90\x03a\x10=W`\0\x80\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x85\x90R`\xFF\x8B\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x10\x91W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01R\x7Fecrecover failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x83\x90\x1B\x16` \x82\x01R\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xF8\x8B\x90\x1B\x16`4\x82\x01R`5\x81\x01\x89\x90R`U\x81\x01\x88\x90R`u\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x14\x93PPPP\x95\x94PPPPPV[``\x81`\x01`\x01`\x80\x1B\x03\x16`\0\x03a\x11\xA3WPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[\x81`\0[`\x01`\x01`\x80\x1B\x03\x82\x16\x15a\x11\xD6W\x80a\x11\xC0\x81a%\x92V[\x91Pa\x11\xCF\x90P`\n\x83a%\xB8V[\x91Pa\x11\xA7V[`\0\x81`\x01`\x01`\x80\x1B\x03\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xFAWa\x11\xFAa\x1C\x07V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12$W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P[`\x01`\x01`\x80\x1B\x03\x85\x16\x15a\t\xADWa\x12B`\x01\x83a%\xDEV[\x91Pa\x12O`\n\x86a&\x06V[a\x12Z\x90`0a&,V[`\xF8\x1B\x81\x83`\x01`\x01`\x80\x1B\x03\x16\x81Q\x81\x10a\x12xWa\x12xa \x1FV[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81`\0\x1A\x90SPa\x12\xB2`\n\x86a%\xB8V[\x94Pa\x12(V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x14V[a\x0B\xB2a\x16\xEDV[\x80Q`\0\x90\x15\x80\x15a\x13@WP` \x82\x01Q\x15[\x92\x91PPV[`\x08\x83\x10a\x13SW`\0\x80\xFD[a\x13i`\x99\x84`\x08\x81\x10a\x0B\xD2Wa\x0B\xD2a \x1FV[\x15a\x13\x88W`\x01a\x02\xB1`\0\x82\x82Ta\x13\x82\x91\x90a \x92V[\x90\x91UPP[`@Q\x80`@\x01`@R\x80\x83\x81R` \x01\x82\x81RP`\x99\x84`\x08\x81\x10a\x13\xB0Wa\x13\xB0a \x1FV[\x82Q`\x02\x91\x90\x91\x02\x91\x90\x91\x01\x90\x81U` \x90\x91\x01Q`\x01\x91\x82\x01U\x83\x1B[a\x01\0\x81\x10\x15a\x14(W`\0a\x02\xA9\x82a\x01\0\x81\x10a\x13\xEFWa\x13\xEFa \x1FV[` \x91\x82\x82\x04\x01\x91\x90\x06a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x90\x1B\x81`\x01a\x14 \x91\x90a \x92V[\x17\x90Pa\x13\xCEV[PPPPV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03\x14V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80`\0a\x15\x01\x85\x85a\x17aV[\x91P\x91Pa\x15\x0E\x81a\x17\xA6V[P\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x153\x83a\x13,V[\x15a\x15?WP\x80a\x13@V[a\x15H\x82a\x13,V[\x15a\x15TWP\x81a\x13@V[\x81Q\x83Q`\0\x91\x90\x03a\x15\xE5W\x82` \x01Q\x84` \x01Q\x14a\x15\x8BWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13@V[d\x01\0\0\x03\xD0\x19\x84Q`\x03\t\x90Pd\x01\0\0\x03\xD0\x19\x84Q\x82\t\x90Pd\x01\0\0\x03\xD0\x19a\x15\xDCd\x01\0\0\x03\xD0\x19\x85` \x01Q`\x02\ta\x15\xD0`\x02d\x01\0\0\x03\xD0\x19a%:V[d\x01\0\0\x03\xD0\x19a\x18\xF0V[\x82\t\x90Pa\x16BV[d\x01\0\0\x03\xD0\x19a\x16\x1Cd\x01\0\0\x03\xD0\x19\x86Qa\x16\x08\x90d\x01\0\0\x03\xD0\x19a%:V[\x86Q\x08a\x15\xD0`\x02d\x01\0\0\x03\xD0\x19a%:V[d\x01\0\0\x03\xD0\x19` \x87\x01Qa\x168\x90d\x01\0\0\x03\xD0\x19a%:V[\x86` \x01Q\x08\t\x90P[`\0d\x01\0\0\x03\xD0\x19\x82\x83\t\x90Pd\x01\0\0\x03\xD0\x19\x85Qa\x16i\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08\x90Pd\x01\0\0\x03\xD0\x19\x84Qa\x16\x86\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08\x90P`\0d\x01\0\0\x03\xD0\x19a\x16\xA3\x83d\x01\0\0\x03\xD0\x19a%:V[\x87Q\x08\x90Pd\x01\0\0\x03\xD0\x19\x83\x82\t\x90Pd\x01\0\0\x03\xD0\x19` \x87\x01Qa\x16\xD0\x90d\x01\0\0\x03\xD0\x19a%:V[\x82\x08`@\x80Q\x80\x82\x01\x90\x91R\x92\x83R` \x83\x01RP\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x17XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03\x14V[a\x0B\xB23a\x14\x88V[`\0\x80\x82Q`A\x03a\x17\x97W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa\x17\x8B\x87\x82\x85\x85a\x19FV[\x94P\x94PPPPa\x17\x9FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a\x17\xBAWa\x17\xBAa \xC3V[\x03a\x17\xC2WPV[`\x01\x81`\x04\x81\x11\x15a\x17\xD6Wa\x17\xD6a \xC3V[\x03a\x18#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03\x14V[`\x02\x81`\x04\x81\x11\x15a\x187Wa\x187a \xC3V[\x03a\x18\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x03\x14V[`\x03\x81`\x04\x81\x11\x15a\x18\x98Wa\x18\x98a \xC3V[\x03a\x0C?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x03\x14V[`\0`\x01[\x83\x15a\t\xADWa\x19\x06`\x02\x85a\x1FAV[`\x01\x03a\x19!W\x82\x80a\x19\x1BWa\x19\x1Ba\x1F+V[\x85\x82\t\x90P[\x82\x80a\x19/Wa\x19/a\x1F+V[\x85\x86\t\x94Pa\x19?`\x02\x85a&WV[\x93Pa\x18\xF5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x19}WP`\0\x90P`\x03a\x1A\x01V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x19\xD1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x19\xFAW`\0`\x01\x92P\x92PPa\x1A\x01V[\x91P`\0\x90P[\x94P\x94\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A6W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91Pa\x1AT``\x86\x01a\x1A\nV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1ArW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\x93W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x17\x9FW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\x1A\xF3W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B\x0BW`\0\x80\xFD[a\x1B\x17\x89\x83\x8A\x01a\x1A\x81V[\x90\x97P\x95P\x85\x91Pa\x1B+` \x89\x01a\x1A\xC3V[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\x1BAW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1BUW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1BdW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1ByW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B\x9EW`\0\x80\xFD[a\x1B\xA7\x82a\x1A\nV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xC3W`\0\x80\xFD[\x835`\x1E\x81\x10a\x1B\xD2W`\0\x80\xFD[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xEEW`\0\x80\xFD[a\x1B\xFA\x86\x82\x87\x01a\x1A\x81V[\x94\x97\x90\x96P\x93\x94PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@R\x90V[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1CAWa\x1CAa\x1C\x07V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1C\xFCWa\x1C\xFCa\x1C\x07V[`@R\x91\x90PV[`\0a\x02\0\x80\x83\x85\x03\x12\x15a\x1D\x18W`\0\x80\xFD[\x83`\x1F\x84\x01\x12a\x1D'W`\0\x80\xFD[a\x1D/a\x1C\x1DV[\x90\x83\x01\x90\x80\x85\x83\x11\x15a\x1DAW`\0\x80\xFD[\x84[\x83\x81\x10\x15a\x1D\x82W`@\x81\x88\x03\x12\x15a\x1D\\W`\0\x80\x81\xFD[a\x1Dda\x1CGV[\x815\x81R` \x80\x83\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x1DCV[P\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x1D\x9EW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xBAW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x1D\xF9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x13Wa\x1E\x13a\x1C\x07V[a\x1E&`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xD3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E;W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1EnW`\0\x80\xFD[\x845\x93Pa\x1E~` \x86\x01a\x1D\xD1V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xA1W`\0\x80\xFD[a\x1E\xAD\x87\x82\x88\x01a\x1D\xE8V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E\xCEW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xECW`\0\x80\xFD[a\x1E\xF8\x86\x82\x87\x01a\x1D\xE8V[\x92PPa\x1F\x07`@\x85\x01a\x1A\nV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1F\"W`\0\x80\xFD[a\x1B\xA7\x82a\x1D\xD1V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1FPWa\x1FPa\x1F+V[P\x06\x90V[`\0[\x83\x81\x10\x15a\x1FpW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FXV[\x83\x81\x11\x15a\x14(WPP`\0\x91\x01RV[a\x029`\xF5\x1B\x81R`\0\x83Qa\x1F\x9E\x81`\x02\x85\x01` \x88\x01a\x1FUV[`\x01`\xFD\x1B`\x02\x91\x84\x01\x91\x82\x01R\x83Qa\x1F\xBF\x81`\x03\x84\x01` \x88\x01a\x1FUV[\x01`\x03\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1F\xEA\x81`@\x85\x01` \x87\x01a\x1FUV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x84\x81R\x83` \x82\x01R\x81\x83`@\x83\x017`\0\x91\x01`@\x01\x90\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a LW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a gW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17\x9FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a \xA5Wa \xA5a |V[P\x01\x90V[`\0`\x01\x82\x01a \xBCWa \xBCa |V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[\x805`\x0F\x81\x90\x0B\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!)W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xE0\x81\x12\x15a!>W`\0\x80\xFD[a!Fa\x1CGV[`\xC0\x82\x12\x15a!TW`\0\x80\xFD[a!\\a\x1CjV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra!x`@\x85\x01a \xD9V[`@\x83\x01R``\x84\x015\x80\x15\x15\x81\x14a!\x90W`\0\x80\xFD[``\x83\x01Ra!\xA1`\x80\x85\x01a \xEDV[`\x80\x83\x01Ra!\xB2`\xA0\x85\x01a\x1A\xC3V[`\xA0\x83\x01R\x90\x81R`\xC0\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[a!\xD9\x87\x83\x86\x01a\x1D\xE8V[` \x82\x01R\x96\x95PPPPPPV[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x1A\x1BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\")W`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a\">W`\0\x80\xFD[a\"Fa\x1CGV[`\x80\x82\x12\x15a\"TW`\0\x80\xFD[a\"\\a\x1C\x8DV[\x91P\x835\x82Ra\"n` \x85\x01a \xD9V[` \x83\x01Ra\"\x7F`@\x85\x01a!\xE8V[`@\x83\x01Ra\"\x90``\x85\x01a\x1A\xC3V[``\x83\x01R\x90\x81R`\x80\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\"\xBCW`\0\x80\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\"\xD8Wa\"\xD8a\x1C\x07V[\x81`\x05\x1Ba\"\xE7\x82\x82\x01a\x1C\xD3V[\x92\x83R\x84\x81\x01\x82\x01\x92\x82\x81\x01\x90\x87\x85\x11\x15a#\x01W`\0\x80\xFD[\x83\x87\x01\x92P[\x84\x83\x10\x15a#'Wa#\x18\x83a \xEDV[\x82R\x91\x83\x01\x91\x90\x83\x01\x90a#\x07V[\x97\x96PPPPPPPV[`\0\x81\x83\x03`\xC0\x81\x12\x15a#EW`\0\x80\xFD[a#Ma\x1C\x8DV[\x91P``\x81\x12\x15a#]W`\0\x80\xFD[Pa#fa\x1C\xB0V[\x825\x81Ra#v` \x84\x01a!\xE8V[` \x82\x01Ra#\x87`@\x84\x01a\x1A\xC3V[`@\x82\x01R\x81R``\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xA9W`\0\x80\xFD[a#\xB5\x85\x83\x86\x01a\x1D\xE8V[` \x84\x01Ra#\xC6`\x80\x85\x01a \xEDV[`@\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a#\xDFW`\0\x80\xFD[Pa#\xEC\x84\x82\x85\x01a\"\xABV[``\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a$\nW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$!W`\0\x80\xFD[a\t\xAD\x84\x82\x85\x01a#2V[`\0` \x82\x84\x03\x12\x15a$?W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$WW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\x80\x81\x12\x15a$lW`\0\x80\xFD[a$ta\x1CGV[``\x82\x12\x15a$\x82W`\0\x80\xFD[a$\x8Aa\x1C\xB0V[\x91P\x835\x82R` \x84\x015` \x83\x01Ra$\xA6`@\x85\x01a\x1A\xC3V[`@\x83\x01R\x90\x81R``\x83\x015\x90\x82\x82\x11\x15a!\xCDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\xD3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$\xEBW`\0\x80\xFD[\x90\x83\x01\x90\x81\x85\x03`\xA0\x81\x12\x15a%\0W`\0\x80\xFD[a%\x08a\x1CGV[`\x80\x82\x12\x15a%\x16W`\0\x80\xFD[a%\x1Ea\x1C\x8DV[\x91P\x835\x82R` \x84\x015` \x83\x01Ra\"\x7F`@\x85\x01a!\xE8V[`\0\x82\x82\x10\x15a%LWa%La |V[P\x03\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a%kWa%ka |V[P\x02\x90V[`\0`\xFF\x83\x16\x80a%\x83Wa%\x83a\x1F+V[\x80`\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x81\x03a%\xAEWa%\xAEa |V[`\x01\x01\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a%\xD2Wa%\xD2a\x1F+V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a%\xFEWa%\xFEa |V[\x03\x93\x92PPPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x84\x16\x80a& Wa& a\x1F+V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0`\x01`\x01`\x80\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a&NWa&Na |V[\x01\x94\x93PPPPV[`\0\x82a&fWa&fa\x1F+V[P\x04\x90V\xFETransferQuote(bytes32 sender,bytes32 recipient,uint128 amount,uint64 nonce)LinkSigner(bytes32 sender,bytes32 signer,uint64 nonce)LiquidateSubaccount(bytes32 sender,bytes32 liquidatee,uint32 productId,bool isEncodedSpread,int128 amount,uint64 nonce)BurnNlp(bytes32 sender,uint128 nlpAmount,uint64 nonce)MintNlp(bytes32 sender,uint128 quoteAmount,uint64 nonce)WithdrawCollateral(bytes32 sender,uint32 productId,uint128 amount,uint64 nonce)\xA2dipfsX\"\x12 \xE0=\xBD\\*\x01\xF7\r\xBEE \xD5\x9E\x1B\xA2\\\xB4\xC0+\x92w=\xEBU\0Tw\x83?\xD5\x12#dsolcC\0\x08\r\x003";
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
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for VerifierEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
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
