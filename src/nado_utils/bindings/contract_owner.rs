pub use contract_owner::*;
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
pub mod contract_owner {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addEngine"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addEngine"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("engine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("offchainExchange"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("engineType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.EngineType",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addNlpPool"),
                        inputs: ::std::vec![
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addOrUpdateProducts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addOrUpdateProducts",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spotIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("perpIds"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("clearPerpAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearPerpAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("clearSpotAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("clearSpotAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createDirectDepositV1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createDirectDepositV1",),
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
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("creditDepositV1"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("creditDepositV1"),
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
                    ::std::borrow::ToOwned::to_owned("deleteNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deleteNlpPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("delistProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delistProduct"),
                        inputs: ::std::vec![
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pricesX18"),
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
                                name: ::std::borrow::ToOwned::to_owned("subaccounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositInsurance"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
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
                (
                    ::std::borrow::ToOwned::to_owned("directDepositV1Address"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("directDepositV1Address",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address payable"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("getDirectDepositV1BytecodeHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDirectDepositV1BytecodeHash",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("hasPendingAddOrUpdateProductCalls"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasPendingAddOrUpdateProductCalls",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("multisig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_deployer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_spotEngine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_perpEngine"),
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
                                name: ::std::borrow::ToOwned::to_owned("_clearinghouse"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_verifier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_wrappedNative"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address payable"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDirectDepositV1Ready"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDirectDepositV1Ready",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isFirstDeposit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("pendingPerpAddOrUpdateProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingPerpAddOrUpdateProductIds",),
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
                    ::std::borrow::ToOwned::to_owned("pendingSpotAddOrUpdateProductIds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pendingSpotAddOrUpdateProductIds",),
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
                    ::std::borrow::ToOwned::to_owned("perpUpdateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("perpUpdateRisk"),
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
                    ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("rebalanceXWithdraw"),
                        inputs: ::std::vec![
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
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
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
                    ::std::borrow::ToOwned::to_owned("removeWithdrawPoolLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeWithdrawPoolLiquidity",),
                        inputs: ::std::vec![
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
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
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
                    ::std::borrow::ToOwned::to_owned("setSpreads"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSpreads"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("spreads"),
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setWithdrawPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_withdrawPool"),
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
                    ::std::borrow::ToOwned::to_owned("spotUpdateRisk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("spotUpdateRisk"),
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
                    ::std::borrow::ToOwned::to_owned("submitPerpAddOrUpdateProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitPerpAddOrUpdateProductCall",),
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
                    ::std::borrow::ToOwned::to_owned("submitSpotAddOrUpdateProductCall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitSpotAddOrUpdateProductCall",),
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
                    ::std::borrow::ToOwned::to_owned("updateNlpPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateNlpPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateTierFeeRates"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("makerRateX18"),
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
                                name: ::std::borrow::ToOwned::to_owned("takerRateX18"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawInsurance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawInsurance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sendTo"),
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
    pub static CONTRACTOWNER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[Pb\0\0\x1Cb\0\0\"V[b\0\0\xE4V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\0\xE2W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[aL\xDB\x80b\0\0\xF4`\09`\0\xF3\xFE`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x02SW`\x005`\xE0\x1C\x80c\x8A)\xE2\xDE\x11b\0\x01BW\x80c\xBE\x05\xA6\x9D\x11b\0\0\xCCW\x80c\xCCtT|\x11b\0\0\x97W\x80c\xD9\xFB\x99\xC1\x11b\0\0zW\x80c\xD9\xFB\x99\xC1\x14b\0\x05\x17W\x80c\xEB\xD6\xC2\x94\x14b\0\x05.W\x80c\xF2\xFD\xE3\x8B\x14b\0\x05EW`\0\x80\xFD[\x80c\xCCtT|\x14b\0\x04\xE9W\x80c\xD4\xD54O\x14b\0\x05\0W`\0\x80\xFD[\x80c\xBE\x05\xA6\x9D\x14b\0\x04\x98W\x80c\xBE\x13\xBA\xC4\x14b\0\x04\xB1W\x80c\xC2'\xDB\x96\x14b\0\x04\xC8W\x80c\xC9\xC5\xEF\xAA\x14b\0\x04\xDFW`\0\x80\xFD[\x80c\x94\x91+\x80\x11b\0\x01\rW\x80c\x94\x91+\x80\x14b\0\x04<W\x80c\x9Bj\xBA\x8F\x14b\0\x04SW\x80c\xA7\xE6\x9D\xA3\x14b\0\x04jW\x80c\xBB\xEF\x84\xB4\x14b\0\x04\x81W`\0\x80\xFD[\x80c\x8A)\xE2\xDE\x14b\0\x03\xE5W\x80c\x8A\xB3\xDA\xAE\x14b\0\x03\xFCW\x80c\x8D\xA5\xCB[\x14b\0\x04\x13W\x80c\x90\x86\xA2\xA5\x14b\0\x04%W`\0\x80\xFD[\x80c=\xAB\xE0\xD9\x11b\0\x01\xE1W\x80clleN\x11b\0\x01\xACW\x80cp|\x8BX\x11b\0\x01\x8FW\x80cp|\x8BX\x14b\0\x03\xBAW\x80cqP\x18\xA6\x14b\0\x03\xC4W\x80c\x86\x0E\x96t\x14b\0\x03\xCEW`\0\x80\xFD[\x80clleN\x14b\0\x03\x8CW\x80cn\x13\xCB\xF3\x14b\0\x03\xA3W`\0\x80\xFD[\x80c=\xAB\xE0\xD9\x14b\0\x030W\x80cS\x12\xB9\x1F\x14b\0\x03GW\x80cV\xE4\x9E\xF3\x14b\0\x03^W\x80clU[\x1B\x14b\0\x03uW`\0\x80\xFD[\x80c\x19b\xC3\x84\x11b\0\x02\"W\x80c\x19b\xC3\x84\x14b\0\x02\xDFW\x80c+\x12l_\x14b\0\x02\xE9W\x80c/\xB0R:\x14b\0\x03\0W\x80c3\x92\xC5\x85\x14b\0\x03\x19W`\0\x80\xFD[\x80c\x12\xEF\xA7\x1B\x14b\0\x02XW\x80c\x14\\\xA3\x80\x14b\0\x02\xA1W\x80c\x14o\xE5\xB5\x14b\0\x02\xBCW\x80c\x17K\x8D^\x14b\0\x02\xD5W[`\0\x80\xFD[b\0\x02\x84b\0\x02i6`\x04b\0)8V[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02\xABb\0\x05\\V[`@Q\x90\x15\x15\x81R` \x01b\0\x02\x98V[b\0\x02\xD3b\0\x02\xCD6`\x04b\0)\x9EV[b\0\x05vV[\0[b\0\x02\xD3b\0\x06\x96V[b\0\x02\xD3b\0\x07\0V[b\0\x02\xD3b\0\x02\xFA6`\x04b\0+dV[b\0\x07dV[b\0\x03\nb\0\x0BhV[`@Q\x90\x81R` \x01b\0\x02\x98V[b\0\x02\xD3b\0\x03*6`\x04b\0,\x84V[b\0\x0B\x9CV[b\0\x02\xD3b\0\x03A6`\x04b\0-\x0EV[b\0\x0C\x12V[b\0\x02\xD3b\0\x03X6`\x04b\0-\xB2V[b\0\x0E4V[b\0\x02\xD3b\0\x03o6`\x04b\0.\x02V[b\0\x0F2V[b\0\x02\xD3b\0\x03\x866`\x04b\0._V[b\0\x0F\xA2V[b\0\x02\xD3b\0\x03\x9D6`\x04b\0.\xC4V[b\0\x11AV[b\0\x02\x84b\0\x03\xB46`\x04b\0)8V[b\0\x11\xF2V[b\0\x02\xD3b\0\x13MV[b\0\x02\xD3b\0\x13\xDDV[b\0\x02\xD3b\0\x03\xDF6`\x04b\0/\0V[b\0\x13\xF3V[b\0\x02\xD3b\0\x03\xF66`\x04b\0/%V[b\0\x15)V[b\0\x02\xD3b\0\x04\r6`\x04b\0,\x84V[b\0\x179V[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02\x84V[b\0\x02\xD3b\0\x0466`\x04b\0)8V[b\0\x17wV[b\0\x02\xD3b\0\x04M6`\x04b\0)8V[b\0\x17\xDFV[b\0\x02\xD3b\0\x04d6`\x04b\x000LV[b\0\x18\x1BV[b\0\x02\xABb\0\x04{6`\x04b\x001\x15V[b\0\x1A\x96V[b\0\x02\xD3b\0\x04\x926`\x04b\0)8V[b\0\x1E_V[b\0\x04\xA2b\0\x1E\x9BV[`@Qb\0\x02\x98\x91\x90b\x001HV[b\0\x02\xD3b\0\x04\xC26`\x04b\x001\x94V[b\0 \x0FV[b\0\x02\xD3b\0\x04\xD96`\x04b\x001\xC1V[b\0 YV[b\0\x04\xA2b\0 \x96V[b\0\x02\xD3b\0\x04\xFA6`\x04b\x001\xE1V[b\0\"\x04V[b\0\x02\xD3b\0\x05\x116`\x04b\x002\x12V[b\0\"YV[b\0\x02\xD3b\0\x05(6`\x04b\x0020V[b\0\"\xF6V[b\0\x02\xD3b\0\x05?6`\x04b\0-\xB2V[b\0$\xB7V[b\0\x02\xD3b\0\x05V6`\x04b\x001\xC1V[b\0%8V[`\xA5T`\0\x90\x15\x15\x80b\0\x05qWP`\xA4T\x15\x15[\x90P\x90V[b\0\x05\x80b\0%\xCEV[`@\x80Q``\x81\x01\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x01`\x01`\x80\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x1A`@\x80Q\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16``\x82\x01R`\x80\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x06\x1A\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x06[\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x06vW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\x8BW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x06\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[b\0\x06\xFE`\xA5`\0b\0(\x04V[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[b\0\x06\xFE`\xA4`\0b\0(\x04V[b\0\x07nb\0%\xCEV[`\0[`\xA4T\x81\x10\x15b\0\t`W`\0`\xA4\x82\x81T\x81\x10b\0\x07\x94Wb\0\x07\x94b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\x07\xAB\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\xD9\x90b\x003\x84V[\x80\x15b\0\x08*W\x80`\x1F\x10b\0\x07\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x08*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x08\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x08D\x91\x90b\x004DV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x08fWb\0\x08fb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x08\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc\xDF\x14O\xD5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c\xDF\x14O\xD5\x96b\0\t\x15\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\x005\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t0W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\tEW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\tW\x90b\x006sV[\x91PPb\0\x07qV[Pb\0\to`\xA4`\0b\0(\x04V[`\0[`\xA5T\x81\x10\x15b\0\x0BUW`\0`\xA5\x82\x81T\x81\x10b\0\t\x95Wb\0\t\x95b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\t\xAC\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\t\xDA\x90b\x003\x84V[\x80\x15b\0\n+W\x80`\x1F\x10b\0\t\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\n+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\n\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\nE\x91\x90b\x006\x8FV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\ngWb\0\ngb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\n\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xA4Y\x89\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xA4Y\x89\xAB\x94b\0\x0B\n\x94\x90\x93\x90\x92\x91`\x04\x01b\x007 V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B%W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B:W=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x0BL\x90b\x006sV[\x91PPb\0\trV[Pb\0\x0Bd`\xA5`\0b\0(\x04V[PPV[`\0`@Q\x80` \x01b\0\x0B|\x90b\0($V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 \x90P\x90V[b\0\x0B\xA6b\0%\xCEV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x0B\xDA\x90\x85\x90\x85\x90`\x04\x01b\x007\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xF5W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\nW=`\0\x80>=`\0\xFD[PPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x0ChW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x84\x83\x14b\0\x0C\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0[\x85\x81\x10\x15b\0\x0E+W`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\x0C\xD8Wb\0\x0C\xD8b\x003nV[\x90P` \x02\x01` \x81\x01\x90b\0\x0C\xEF\x91\x90b\x007\xE6V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\r\x0FWb\0\r\x0Fb\x003nV[\x90P` \x02\x01` \x81\x01\x90b\0\r&\x91\x90b\08\x06V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\r~\x91\x90b\08&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\r\x9E\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\r\xDF\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\r\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E\x0FW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x0E\"\x90b\x006sV[\x91PPb\0\x0C\xADV[PPPPPPPV[b\0\x0E>b\0%\xCEV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x0E\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\xAF\x91\x90b\08\x8CV[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0F\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0F(W=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x0F<b\0%\xCEV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x0Fr\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\08\xACV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0F\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E+W=`\0\x80>=`\0\xFD[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x0F\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0b\0\x10\x04b\0\x1E\x9BV[\x90P`\0[\x81Q\x81\x10\x15b\0\x10\xB7W\x81\x81\x81Q\x81\x10b\0\x10(Wb\0\x10(b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x10\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a perp p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[\x80b\0\x10\xAE\x81b\x006sV[\x91PPb\0\x10\tV[P`\xA5`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x10\xF6\x91\x90b\08\xF1V[\x90R`@Qb\0\x11\n\x91\x90` \x01b\09\x10V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x0C\n\x94\x91\x90\x92\x01\x92\x01\x90b\0(2V[b\0\x11Kb\0%\xCEV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x92\x81\x01\x92\x90\x92R\x90`\0\x90`\x12\x90``\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x11\xB1\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0E\xF8\x90\x84\x90`\x04\x01b\x003YV[`\0b\0\x11\xFEb\0\x0BhV[\x7Fyt\xDFA\xBD\xCA+\xE1S\x9F\xA7\xD0\x1FA'\x7F\rr\x88#\xB2\x020\xA1\x8A1\xE4\x0Cpxt\xE7\x14b\0\x12\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FInvalid DirectDepositV1 bytecode`D\x82\x01R\x7F hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[`\x9CT`\x9AT`\x9FT`@Q`\0\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x82\x16\x92\x88\x92\x90\x91\x16\x90b\0\x12\xC7\x90b\0($V[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x13\x10W=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x13Wb\0%\xCEV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x13\xA6\x90\x84\x90`%\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x13\xC1W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x13\xD6W=`\0\x80>=`\0\xFD[PPPPPV[b\0\x13\xE7b\0%\xCEV[b\0\x06\xFE`\0b\0&*V[b\0\x13\xFDb\0%\xCEV[`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x14HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14n\x91\x90b\08\x8CV[`\x9CT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x14\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14\xF2\x91\x90b\09\x91V[P`@\x80Q` \x81\x01\x90\x91R`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`\0`\x07`@\x80Q\x84Q`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R\x01b\0\x11\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x15JWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x15fWP0;\x15\x80\x15b\0\x15fWP`\0T`\xFF\x16`\x01\x14[b\0\x15\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x15\xFEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x16XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x06\xE7V[b\0\x16bb\0&\x89V[b\0\x16m\x89b\0%8V[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x06\x8BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[b\0\x17Cb\0%\xCEV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x0B\xDA\x90\x85\x90\x85\x90`\x04\x01b\x007\x91V[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17\xA3Wb\0\x17\xA0\x82b\0\x11\xF2V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xF5W`\0\x80\xFD[b\0\x17\xE9b\0%\xCEV[`\x9DT`@Qc\x01)\"W`\xE7\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\x91+\x80\x90`$\x01b\0\x13\xA6V[b\0\x18%b\0%\xCEV[\x82Q\x84Q\x14b\0\x18iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x81Q\x84Q\x14b\0\x18\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x80Q\x84Q\x14b\0\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0[\x84Q\x81\x10\x15b\0\x13\xD6W`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x19 Wb\0\x19 b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x19HWb\0\x19Hb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0\x19pWb\0\x19pb\x003nV[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0\x19\x95Wb\0\x19\x95b\x003nV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1A\t\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x1AJ\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1AeW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1AzW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x1A\x8D\x90b\x006sV[\x91PPb\0\x18\xF4V[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1A\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x1B\x17\x91\x90\x81\x01\x90b\09\xB1V[\x90P`\0[\x81Q\x81\x10\x15b\0\x1ERW`\0\x82\x82\x81Q\x81\x10b\0\x1B=Wb\0\x1B=b\x003nV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1B\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1B\xC3\x91\x90b\08\x8CV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x1C\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1C\x8F\x91\x90b\0:KV[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x1C\xC2Wb\0\x1C\xBF`\x01`\x01`\xA0\x1B\x03\x8A\x161\x82b\0:eV[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1D\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1D'\x91\x90b\0:\x80V[b\0\x1D4\x90`\x12b\0:\xA5V[b\0\x1DA\x90`\nb\0;\xC8V[b\0\x1DM\x90\x82b\0;\xD9V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x1D\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1D\xCA\x91\x90b\0;\xFBV[`\x80\x01Q\x90P\x88b\0\x1D\xF4Wb\0\x1D\xEB`\ng\r\xE0\xB6\xB3\xA7d\0\0b\0<\xA1V[`\x0F\x0Bb\0\x1E\rV[b\0\x1E\tg\r\xE0\xB6\xB3\xA7d\0\0`\x05b\0<\xFAV[`\x0F\x0B[b\0\x1E\x1D`\x0F\x83\x90\x0B\x84b\0'\0V[`\x0F\x0B\x12b\0\x1E7W`\x01\x97PPPPPPPPb\0\x1EYV[PPPPP\x80\x80b\0\x1EI\x90b\x006sV[\x91PPb\0\x1B\x1CV[P`\0\x91PP[\x92\x91PPV[b\0\x1Eib\0%\xCEV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x13\xA6V[`\xA5T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1E\xBFWb\0\x1E\xBFb\0)\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1E\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA5T\x81\x10\x15b\0 \tW`\0`\xA5\x82\x81T\x81\x10b\0\x1F\x12Wb\0\x1F\x12b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\x1F)\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1FW\x90b\x003\x84V[\x80\x15b\0\x1F\xA8W\x80`\x1F\x10b\0\x1F|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1F\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1F\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1F\xC2\x91\x90b\x006\x8FV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1F\xDEWb\0\x1F\xDEb\x003nV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0 \0\x81b\x006sV[\x91PPb\0\x1E\xEFV[P\x91\x90PV[b\0 \x19b\0%\xCEV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x0FrV[b\0 cb\0%\xCEV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x13\xA6V[`\xA4T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0 \xBAWb\0 \xBAb\0)\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0 \xE4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA4T\x81\x10\x15b\0 \tW`\0`\xA4\x82\x81T\x81\x10b\0!\rWb\0!\rb\x003nV[\x90`\0R` `\0 \x01\x80Tb\0!$\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0!R\x90b\x003\x84V[\x80\x15b\0!\xA3W\x80`\x1F\x10b\0!wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0!\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0!\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0!\xBD\x91\x90b\x004DV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0!\xD9Wb\0!\xD9b\x003nV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0!\xFB\x81b\x006sV[\x91PPb\0 \xEAV[b\0\"\x0Eb\0%\xCEV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16\x80\x82R`\x01`\x01`\x80\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x92\x81\x01\x92\x90\x92R\x90`\0\x90`\x19\x90``\x01b\0\x11\x91V[b\0\"cb\0%\xCEV[`@\x80Q` \x81\x01\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\0`\x1B`@\x80Q\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\"\xB5\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0Fr\x90\x84\x90`\x04\x01b\x003YV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0#LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0b\0#Xb\0 \x96V[\x90P`\0[\x81Q\x81\x10\x15b\0$\x0BW\x81\x81\x81Q\x81\x10b\0#|Wb\0#|b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0#\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a spot p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[\x80b\0$\x02\x81b\x006sV[\x91PPb\0#]V[P`\xA4`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0$V\x91\x90b\0=\xADV[\x81R` \x01b\0$l6\x86\x90\x03\x86\x01\x86b\08\xF1V[\x90R`@Qb\0$\x80\x91\x90` \x01b\0>wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x0F(\x94\x91\x90\x92\x01\x92\x01\x90b\0(2V[b\0$\xC1b\0%\xCEV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`\x80\x1B\x03\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01b\0\x05\xFAV[b\0%Bb\0%\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0%\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0%\xCB\x81b\0&*V[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x06\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x06\xE7V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0&\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0\x06\xFEb\0'\x8CV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0'HWPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0'\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x06\xE7\x91\x90b\x003YV[P\x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0'\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0\x06\xFE3b\0&*V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0%\xCB\x91\x90b\0(\xC1V[a\r9\x80b\0?m\x839\x01\x90V[\x82\x80Tb\0(@\x90b\x003\x84V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0(dW`\0\x85Ub\0(\xAFV[\x82`\x1F\x10b\0(\x7FW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0(\xAFV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0(\xAFW\x91\x82\x01[\x82\x81\x11\x15b\0(\xAFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0(\x92V[Pb\0(\xBD\x92\x91Pb\0(\xE2V[P\x90V[\x80\x82\x11\x15b\0(\xBDW`\0b\0(\xD8\x82\x82b\0(\xF9V[P`\x01\x01b\0(\xC1V[[\x80\x82\x11\x15b\0(\xBDW`\0\x81U`\x01\x01b\0(\xE3V[P\x80Tb\0)\x07\x90b\x003\x84V[`\0\x82U\x80`\x1F\x10b\0)\x18WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0%\xCB\x91\x90b\0(\xE2V[`\0` \x82\x84\x03\x12\x15b\0)KW`\0\x80\xFD[P5\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0)kW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0%\xCBW`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14b\0)kW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)\xB4W`\0\x80\xFD[b\0)\xBF\x84b\0)RV[\x92P` \x84\x015b\0)\xD1\x81b\0)pV[\x91Pb\0)\xE1`@\x85\x01b\0)\x86V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*\xA4Wb\0*\xA4b\0)\xEAV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0*\xC9Wb\0*\xC9b\0)\xEAV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0%\xCBW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12b\0*\xF8W`\0\x80\xFD[\x815` b\0+\x11b\0+\x0B\x83b\0*\xACV[b\0*xV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0+1W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0+YW\x805b\0+K\x81b\0*\xD3V[\x83R\x91\x83\x01\x91\x83\x01b\0+5V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0+xW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\x91W`\0\x80\xFD[b\0+\x9F\x86\x83\x87\x01b\0*\xE6V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0+\xB6W`\0\x80\xFD[Pb\0+\xC5\x85\x82\x86\x01b\0*\xE6V[\x91PP\x92P\x92\x90PV[\x80`\x03\x0B\x81\x14b\0%\xCBW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0%\xCBW`\0\x80\xFD[\x805b\0)k\x81b\0+\xDFV[`\0`\xA0\x82\x84\x03\x12\x15b\0,\x0FW`\0\x80\xFD[b\0,\x19b\0*\0V[\x90P\x815b\0,(\x81b\0+\xCFV[\x81R` \x82\x015b\0,:\x81b\0+\xCFV[` \x82\x01R`@\x82\x015b\0,O\x81b\0+\xCFV[`@\x82\x01R``\x82\x015b\0,d\x81b\0+\xCFV[``\x82\x01R`\x80\x82\x015b\0,y\x81b\0+\xDFV[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0,\x98W`\0\x80\xFD[\x825b\0,\xA5\x81b\0*\xD3V[\x91Pb\0,\xB6\x84` \x85\x01b\0+\xFCV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0,\xD2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0,\xEBW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0-\x07W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0-(W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0-AW`\0\x80\xFD[b\0-O\x8A\x83\x8B\x01b\0,\xBFV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0-iW`\0\x80\xFD[b\0-w\x8A\x83\x8B\x01b\0,\xBFV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0-\x91W`\0\x80\xFD[Pb\0-\xA0\x89\x82\x8A\x01b\0,\xBFV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0-\xC8W`\0\x80\xFD[\x835b\0-\xD5\x81b\0*\xD3V[\x92Pb\0-\xE5` \x85\x01b\0)\x86V[\x91P`@\x84\x015b\0-\xF7\x81b\0)pV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0.\x18W`\0\x80\xFD[\x835b\0.%\x81b\0)pV[\x92P` \x84\x015b\0.7\x81b\0)pV[\x91P`@\x84\x015`\x02\x81\x10b\0-\xF7W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0 \tW`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0.wW`\0\x80\xFD[\x845b\0.\x84\x81b\0*\xD3V[\x93P` \x85\x015b\0.\x96\x81b\0+\xDFV[\x92P`@\x85\x015b\0.\xA8\x81b\0+\xDFV[\x91Pb\0.\xB9\x86``\x87\x01b\0.LV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15b\0.\xD8W`\0\x80\xFD[b\0.\xE3\x83b\0)\x86V[\x91P` \x83\x015b\0.\xF5\x81b\0)pV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15b\0/\x13W`\0\x80\xFD[b\0/\x1E\x82b\0)\x86V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0/CW`\0\x80\xFD[\x885b\0/P\x81b\0)pV[\x97P` \x89\x015b\0/b\x81b\0)pV[\x96P`@\x89\x015b\0/t\x81b\0)pV[\x95P``\x89\x015b\0/\x86\x81b\0)pV[\x94P`\x80\x89\x015b\0/\x98\x81b\0)pV[\x93P`\xA0\x89\x015b\0/\xAA\x81b\0)pV[\x92P`\xC0\x89\x015b\0/\xBC\x81b\0)pV[\x91P`\xE0\x89\x015b\0/\xCE\x81b\0)pV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x82`\x1F\x83\x01\x12b\0/\xF1W`\0\x80\xFD[\x815` b\x000\x04b\0+\x0B\x83b\0*\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\x000$W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0+YW\x805b\x000>\x81b\0+\xDFV[\x83R\x91\x83\x01\x91\x83\x01b\x000(V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\x000cW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x000|W`\0\x80\xFD[b\x000\x8A\x88\x83\x89\x01b\0*\xE6V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\x000\xA1W`\0\x80\xFD[b\x000\xAF\x88\x83\x89\x01b\0*\xE6V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\x000\xC6W`\0\x80\xFD[b\x000\xD4\x88\x83\x89\x01b\0/\xDFV[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\x000\xEBW`\0\x80\xFD[Pb\x000\xFA\x87\x82\x88\x01b\0/\xDFV[\x91PP\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14b\0%\xCBW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\x001)W`\0\x80\xFD[\x825b\x0016\x81b\0)pV[\x91P` \x83\x015b\0.\xF5\x81b\x001\x06V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\x001\x88W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\x001dV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\x001\xAAW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\x001\xD4W`\0\x80\xFD[\x815b\0/\x1E\x81b\0)pV[`\0\x80`@\x83\x85\x03\x12\x15b\x001\xF5W`\0\x80\xFD[\x825b\x002\x02\x81b\0)pV[\x91Pb\0,\xB6` \x84\x01b\0)\x86V[`\0` \x82\x84\x03\x12\x15b\x002%W`\0\x80\xFD[b\0/\x1E\x82b\0)RV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\x002LW`\0\x80\xFD[\x875b\x002Y\x81b\0*\xD3V[\x96P` \x88\x015b\x002k\x81b\0*\xD3V[\x95P`@\x88\x015b\x002}\x81b\0+\xDFV[\x94P``\x88\x015b\x002\x8F\x81b\0+\xDFV[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\x002\xA4W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\x002\xBB\x88a\x01`\x89\x01b\0.LV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0[\x83\x81\x10\x15b\x002\xE4W\x81\x81\x01Q\x83\x82\x01R` \x01b\x002\xCAV[\x83\x81\x11\x15b\x002\xF4W`\0\x84\x84\x01R[PPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x003\x1D\x81`\x01\x85\x01` \x87\x01b\x002\xC7V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x81Q\x80\x84Rb\x003E\x81` \x86\x01` \x86\x01b\x002\xC7V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0/\x1E` \x83\x01\x84b\x003+V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\x003\x99W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0 \tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80Qb\0)k\x81b\0+\xDFV[`\0`\xA0\x82\x84\x03\x12\x15b\x003\xDAW`\0\x80\xFD[b\x003\xE4b\0*\0V[\x90P\x81Qb\x003\xF3\x81b\0+\xCFV[\x81R` \x82\x01Qb\x004\x05\x81b\0+\xCFV[` \x82\x01R`@\x82\x01Qb\x004\x1A\x81b\0+\xCFV[`@\x82\x01R``\x82\x01Qb\x004/\x81b\0+\xCFV[``\x82\x01R`\x80\x82\x01Qb\0,y\x81b\0+\xDFV[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\x004YW`\0\x80\xFD[b\x004cb\0*,V[\x83Qb\x004p\x81b\0*\xD3V[\x81R` \x84\x01Qb\x004\x82\x81b\0*\xD3V[` \x82\x01R`@\x84\x01Qb\x004\x97\x81b\0+\xDFV[`@\x82\x01R``\x84\x01Qb\x004\xAC\x81b\0+\xDFV[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\x004\xC4W`\0\x80\xFD[b\x004\xCEb\0*RV[\x91P`\x80\x84\x01Qb\x004\xE0\x81b\0)pV[\x82R`\xA0\x84\x01Qb\x004\xF2\x81b\0+\xDFV[` \x83\x01R`\xC0\x84\x01Qb\x005\x07\x81b\0+\xDFV[`@\x83\x01R`\xE0\x84\x01Qb\x005\x1C\x81b\0+\xDFV[``\x83\x01Rb\x0050a\x01\0\x85\x01b\x003\xBAV[`\x80\x83\x01Rb\x005Da\x01 \x85\x01b\x003\xBAV[`\xA0\x83\x01Rb\x005Xa\x01@\x85\x01b\x003\xBAV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\x005s\x85a\x01`\x86\x01b\x003\xC7V[`\xA0\x82\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\x006\x0F`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01R[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x006\x88Wb\x006\x88b\x006]V[P`\x01\x01\x90V[`\0a\x01\0\x82\x84\x03\x12\x15b\x006\xA3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x006\xC9Wb\x006\xC9b\0)\xEAV[`@R\x82Qb\x006\xD9\x81b\0*\xD3V[\x81R` \x83\x01Qb\x006\xEB\x81b\0+\xDFV[` \x82\x01R`@\x83\x01Qb\x007\0\x81b\0+\xDFV[`@\x82\x01Rb\x007\x14\x84``\x85\x01b\x003\xC7V[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\x007\x88``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0/\x1E` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15b\x007\xF9W`\0\x80\xFD[\x815b\0/\x1E\x81b\0*\xD3V[`\0` \x82\x84\x03\x12\x15b\08\x19W`\0\x80\xFD[\x815b\0/\x1E\x81b\0+\xDFV[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0+YW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\08jV[`\0` \x82\x84\x03\x12\x15b\08\x9FW`\0\x80\xFD[\x81Qb\0/\x1E\x81b\0)pV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\08\xE3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\09\x04W`\0\x80\xFD[b\0/\x1E\x83\x83b\0+\xFCV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\09\x8A``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\09\xA4W`\0\x80\xFD[\x81Qb\0/\x1E\x81b\x001\x06V[`\0` \x80\x83\x85\x03\x12\x15b\09\xC5W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\09\xDDW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\09\xEFW`\0\x80\xFD[\x80Qb\0:\0b\0+\x0B\x82b\0*\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\0: W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x006RW\x83Qb\0:;\x81b\0*\xD3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\0:%V[`\0` \x82\x84\x03\x12\x15b\0:^W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\0:{Wb\0:{b\x006]V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0:\x93W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0/\x1EW`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0:\xC2Wb\0:\xC2b\x006]V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\0;\x0CW\x81`\0\x19\x04\x82\x11\x15b\0:\xF0Wb\0:\xF0b\x006]V[\x80\x85\x16\x15b\0:\xFEW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\0:\xD0V[P\x92P\x92\x90PV[`\0\x82b\0;%WP`\x01b\0\x1EYV[\x81b\0;4WP`\0b\0\x1EYV[\x81`\x01\x81\x14b\0;MW`\x02\x81\x14b\0;XWb\0;xV[`\x01\x91PPb\0\x1EYV[`\xFF\x84\x11\x15b\0;lWb\0;lb\x006]V[PP`\x01\x82\x1Bb\0\x1EYV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0;\x9DWP\x81\x81\nb\0\x1EYV[b\0;\xA9\x83\x83b\0:\xCBV[\x80`\0\x19\x04\x82\x11\x15b\0;\xC0Wb\0;\xC0b\x006]V[\x02\x93\x92PPPV[`\0b\0/\x1E`\xFF\x84\x16\x83b\0;\x14V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0;\xF6Wb\0;\xF6b\x006]V[P\x02\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\0<\x0EW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0<4Wb\0<4b\0)\xEAV[`@R\x82Qb\0<D\x81b\0+\xDFV[\x81R` \x83\x01Qb\0<V\x81b\0+\xDFV[` \x82\x01R`@\x83\x01Qb\0<k\x81b\0+\xDFV[`@\x82\x01R``\x83\x01Qb\0<\x80\x81b\0+\xDFV[``\x82\x01R`\x80\x83\x01Qb\0<\x95\x81b\0+\xDFV[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80b\0<\xC7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15b\0<\xF1Wb\0<\xF1b\x006]V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\0=6Wb\0=6b\x006]V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\0=eWb\0=eb\x006]V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\0=\x84Wb\0=\x84b\x006]V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\0=\x9DWb\0=\x9Db\x006]V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15b\0=\xC0W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0=\xE6Wb\0=\xE6b\0)\xEAV[`@R\x825b\0=\xF6\x81b\0)pV[\x81R` \x83\x015b\0>\x08\x81b\0+\xDFV[` \x82\x01R`@\x83\x015b\0>\x1D\x81b\0+\xDFV[`@\x82\x01R``\x83\x015b\0>2\x81b\0+\xDFV[``\x82\x01Rb\0>E`\x80\x84\x01b\0+\xEFV[`\x80\x82\x01Rb\0>X`\xA0\x84\x01b\0+\xEFV[`\xA0\x82\x01Rb\0>k`\xC0\x84\x01b\0+\xEFV[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\0?\x1E`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\09\x8AV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r98\x03\x80a\r9\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xBEV[a\083a\x01VV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90UG\x80\x15a\x01\x12W`\x04T`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xD3V[``\x91P[PP\x90P\x80a\x01\x10W`@Q\x82\x81R\x7F\t\xE9\xD3\x9E)\xBF\x10\xAEZ\xBC\x1C@\xBE.\xEF\x8E\x82\xEDw\xC5\x95\x81J\x1E\xC9_\xC3\xBEa?24\x90` \x01`@Q\x80\x91\x03\x90\xA1[P[`\x03T`@\x80Q0\x81R\x90Q`\x01\x91\x7F\xBB\xC0\x9A\xD7\xAE}ZC~\xEA)\x01{\x8B1\x9D\xA6\xB6\xE07\x11s\x1B[\xCD\xD4;\xEC<\x19K\x17\x91\x90\x81\x90\x03` \x01\x90\xA3PPPPPa\x02\x13V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBBW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\xD4W`\0\x80\xFD[\x84Qa\x01\xDF\x81a\x01\xA6V[` \x86\x01Q\x90\x94Pa\x01\xF0\x81a\x01\xA6V[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x02\x08\x81a\x01\xA6V[\x93\x96\x92\x95P\x90\x93PPV[a\x0B\x17\x80a\x02\"`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0NW\x80cT\xFDMP\x14a\x01rW\x80cqP\x18\xA6\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01\xA8W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD0W`\0\x80\xFD[\x80c&\x08o\x07\x14a\x01(W\x80cPC\x1C\xE4\x14a\x01=W\x80cQ\xCF\xF8\xD9\x14a\x01RW`\0\x80\xFD[6a\x01#W`\x04T`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x904\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xC6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xCBV[``\x91P[PP\x90P\x80a\x01!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFailed to wrap native token.\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x014W`\0\x80\xFD[Pa\x01!a\x01\xF0V[4\x80\x15a\x01IW`\0\x80\xFD[Pa\x01!a\x05\x06V[4\x80\x15a\x01^W`\0\x80\xFD[Pa\x01!a\x01m6`\x04a\x08\xF9V[a\x05\xCEV[4\x80\x15a\x01~W`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x9FW`\0\x80\xFD[Pa\x01!a\x06NV[4\x80\x15a\x01\xB4W`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8AV[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\x01!a\x01\xEB6`\x04a\x08\xF9V[a\x06bV[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x029W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02a\x91\x90\x81\x01\x90a\tLV[\x90P`\0[\x81Q\x81\x10\x15a\x05\x02W`\0\x82\x82\x81Q\x81\x10a\x02\x83Wa\x02\x83a\n\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x06\x91\x90a\n'V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x18V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCB\x91\x90a\nDV[\x90P\x80\x15a\x04\xEBW`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04J\x91\x90a\n]V[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xD2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE6W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04\xFA\x90a\n\x7FV[\x91PPa\x02fV[PPV[a\x05\x0Ea\x06\xF2V[`@QG\x90`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05RW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05WV[``\x91P[PP\x90P\x80a\x05\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FFailed to transfer native token `D\x82\x01R\x7Fto owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x18V[a\x05\xD6a\x06\xF2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06A\x91\x90a\nDV[\x90Pa\x05\x02\x823\x83a\x07LV[a\x06Va\x06\xF2V[a\x06``\0a\x08|V[V[a\x06ja\x06\xF2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x18V[a\x06\xEF\x81a\x08|V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\x18V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x07\xBD\x91\x90a\n\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xFFV[``\x91P[P\x91P\x91P\x81\x80\x15a\x08)WP\x80Q\x15\x80a\x08)WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08)\x91\x90a\n]V[a\x08uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTransfer failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x18V[PPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\x0BW`\0\x80\xFD[\x815a\t\x16\x81a\x08\xE4V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\tGW`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\t_W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\twW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t\x8BW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\t\x9DWa\t\x9Da\t\x1DV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\t\xC2Wa\t\xC2a\t\x1DV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\t\xE0W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\n\x05Wa\t\xF6\x85a\t3V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\t\xE5V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\n9W`\0\x80\xFD[\x81Qa\t\x16\x81a\x08\xE4V[`\0` \x82\x84\x03\x12\x15a\nVW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\noW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x16W`\0\x80\xFD[`\0`\x01\x82\x01a\n\x9FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x82Q`\0[\x81\x81\x10\x15a\n\xC7W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\n\xADV[\x81\x81\x11\x15a\n\xD6W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE0\xDD\x03\xCF\tMG\xE3'\x1A\x02\xF0df:\n\x8F\x0C\x86:\xBAn\x9C\x8D\xAB\xCCZ\x8F/\xE4\xFE\xA6dsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x16\x8B).\r\x8B\xA2?Iqg{C\x8B\x83\xAB\x90LpY\xC0g(\xD0\x0C+|i\x96Qn\x9BdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static CONTRACTOWNER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`\x046\x10b\0\x02SW`\x005`\xE0\x1C\x80c\x8A)\xE2\xDE\x11b\0\x01BW\x80c\xBE\x05\xA6\x9D\x11b\0\0\xCCW\x80c\xCCtT|\x11b\0\0\x97W\x80c\xD9\xFB\x99\xC1\x11b\0\0zW\x80c\xD9\xFB\x99\xC1\x14b\0\x05\x17W\x80c\xEB\xD6\xC2\x94\x14b\0\x05.W\x80c\xF2\xFD\xE3\x8B\x14b\0\x05EW`\0\x80\xFD[\x80c\xCCtT|\x14b\0\x04\xE9W\x80c\xD4\xD54O\x14b\0\x05\0W`\0\x80\xFD[\x80c\xBE\x05\xA6\x9D\x14b\0\x04\x98W\x80c\xBE\x13\xBA\xC4\x14b\0\x04\xB1W\x80c\xC2'\xDB\x96\x14b\0\x04\xC8W\x80c\xC9\xC5\xEF\xAA\x14b\0\x04\xDFW`\0\x80\xFD[\x80c\x94\x91+\x80\x11b\0\x01\rW\x80c\x94\x91+\x80\x14b\0\x04<W\x80c\x9Bj\xBA\x8F\x14b\0\x04SW\x80c\xA7\xE6\x9D\xA3\x14b\0\x04jW\x80c\xBB\xEF\x84\xB4\x14b\0\x04\x81W`\0\x80\xFD[\x80c\x8A)\xE2\xDE\x14b\0\x03\xE5W\x80c\x8A\xB3\xDA\xAE\x14b\0\x03\xFCW\x80c\x8D\xA5\xCB[\x14b\0\x04\x13W\x80c\x90\x86\xA2\xA5\x14b\0\x04%W`\0\x80\xFD[\x80c=\xAB\xE0\xD9\x11b\0\x01\xE1W\x80clleN\x11b\0\x01\xACW\x80cp|\x8BX\x11b\0\x01\x8FW\x80cp|\x8BX\x14b\0\x03\xBAW\x80cqP\x18\xA6\x14b\0\x03\xC4W\x80c\x86\x0E\x96t\x14b\0\x03\xCEW`\0\x80\xFD[\x80clleN\x14b\0\x03\x8CW\x80cn\x13\xCB\xF3\x14b\0\x03\xA3W`\0\x80\xFD[\x80c=\xAB\xE0\xD9\x14b\0\x030W\x80cS\x12\xB9\x1F\x14b\0\x03GW\x80cV\xE4\x9E\xF3\x14b\0\x03^W\x80clU[\x1B\x14b\0\x03uW`\0\x80\xFD[\x80c\x19b\xC3\x84\x11b\0\x02\"W\x80c\x19b\xC3\x84\x14b\0\x02\xDFW\x80c+\x12l_\x14b\0\x02\xE9W\x80c/\xB0R:\x14b\0\x03\0W\x80c3\x92\xC5\x85\x14b\0\x03\x19W`\0\x80\xFD[\x80c\x12\xEF\xA7\x1B\x14b\0\x02XW\x80c\x14\\\xA3\x80\x14b\0\x02\xA1W\x80c\x14o\xE5\xB5\x14b\0\x02\xBCW\x80c\x17K\x8D^\x14b\0\x02\xD5W[`\0\x80\xFD[b\0\x02\x84b\0\x02i6`\x04b\0)8V[`\xA3` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[b\0\x02\xABb\0\x05\\V[`@Q\x90\x15\x15\x81R` \x01b\0\x02\x98V[b\0\x02\xD3b\0\x02\xCD6`\x04b\0)\x9EV[b\0\x05vV[\0[b\0\x02\xD3b\0\x06\x96V[b\0\x02\xD3b\0\x07\0V[b\0\x02\xD3b\0\x02\xFA6`\x04b\0+dV[b\0\x07dV[b\0\x03\nb\0\x0BhV[`@Q\x90\x81R` \x01b\0\x02\x98V[b\0\x02\xD3b\0\x03*6`\x04b\0,\x84V[b\0\x0B\x9CV[b\0\x02\xD3b\0\x03A6`\x04b\0-\x0EV[b\0\x0C\x12V[b\0\x02\xD3b\0\x03X6`\x04b\0-\xB2V[b\0\x0E4V[b\0\x02\xD3b\0\x03o6`\x04b\0.\x02V[b\0\x0F2V[b\0\x02\xD3b\0\x03\x866`\x04b\0._V[b\0\x0F\xA2V[b\0\x02\xD3b\0\x03\x9D6`\x04b\0.\xC4V[b\0\x11AV[b\0\x02\x84b\0\x03\xB46`\x04b\0)8V[b\0\x11\xF2V[b\0\x02\xD3b\0\x13MV[b\0\x02\xD3b\0\x13\xDDV[b\0\x02\xD3b\0\x03\xDF6`\x04b\0/\0V[b\0\x13\xF3V[b\0\x02\xD3b\0\x03\xF66`\x04b\0/%V[b\0\x15)V[b\0\x02\xD3b\0\x04\r6`\x04b\0,\x84V[b\0\x179V[`gT`\x01`\x01`\xA0\x1B\x03\x16b\0\x02\x84V[b\0\x02\xD3b\0\x0466`\x04b\0)8V[b\0\x17wV[b\0\x02\xD3b\0\x04M6`\x04b\0)8V[b\0\x17\xDFV[b\0\x02\xD3b\0\x04d6`\x04b\x000LV[b\0\x18\x1BV[b\0\x02\xABb\0\x04{6`\x04b\x001\x15V[b\0\x1A\x96V[b\0\x02\xD3b\0\x04\x926`\x04b\0)8V[b\0\x1E_V[b\0\x04\xA2b\0\x1E\x9BV[`@Qb\0\x02\x98\x91\x90b\x001HV[b\0\x02\xD3b\0\x04\xC26`\x04b\x001\x94V[b\0 \x0FV[b\0\x02\xD3b\0\x04\xD96`\x04b\x001\xC1V[b\0 YV[b\0\x04\xA2b\0 \x96V[b\0\x02\xD3b\0\x04\xFA6`\x04b\x001\xE1V[b\0\"\x04V[b\0\x02\xD3b\0\x05\x116`\x04b\x002\x12V[b\0\"YV[b\0\x02\xD3b\0\x05(6`\x04b\x0020V[b\0\"\xF6V[b\0\x02\xD3b\0\x05?6`\x04b\0-\xB2V[b\0$\xB7V[b\0\x02\xD3b\0\x05V6`\x04b\x001\xC1V[b\0%8V[`\xA5T`\0\x90\x15\x15\x80b\0\x05qWP`\xA4T\x15\x15[\x90P\x90V[b\0\x05\x80b\0%\xCEV[`@\x80Q``\x81\x01\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x01`\x01`\x80\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x1A`@\x80Q\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\x80\x1B\x03\x16``\x82\x01R`\x80\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x06\x1A\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x06[\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x06vW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x06\x8BW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x06\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[b\0\x06\xFE`\xA5`\0b\0(\x04V[V[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[b\0\x06\xFE`\xA4`\0b\0(\x04V[b\0\x07nb\0%\xCEV[`\0[`\xA4T\x81\x10\x15b\0\t`W`\0`\xA4\x82\x81T\x81\x10b\0\x07\x94Wb\0\x07\x94b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\x07\xAB\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x07\xD9\x90b\x003\x84V[\x80\x15b\0\x08*W\x80`\x1F\x10b\0\x07\xFEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x08*V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x08\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x08D\x91\x90b\x004DV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84\x83\x81Q\x81\x10b\0\x08fWb\0\x08fb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\x08\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fspot id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`\x9AT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Qc\xDF\x14O\xD5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x96c\xDF\x14O\xD5\x96b\0\t\x15\x96\x90\x95\x90\x94\x93\x92\x91`\x04\x01b\x005\x80V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\t0W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\tEW=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\tW\x90b\x006sV[\x91PPb\0\x07qV[Pb\0\to`\xA4`\0b\0(\x04V[`\0[`\xA5T\x81\x10\x15b\0\x0BUW`\0`\xA5\x82\x81T\x81\x10b\0\t\x95Wb\0\t\x95b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\t\xAC\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\t\xDA\x90b\x003\x84V[\x80\x15b\0\n+W\x80`\x1F\x10b\0\t\xFFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\n+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\n\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\nE\x91\x90b\x006\x8FV[\x90P\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10b\0\ngWb\0\ngb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x14b\0\n\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01R\x7Fperp id doesn't match.\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`\x9BT\x81Q` \x83\x01Q`@\x80\x85\x01Q``\x86\x01Q\x91Qc\xA4Y\x89\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x94c\xA4Y\x89\xAB\x94b\0\x0B\n\x94\x90\x93\x90\x92\x91`\x04\x01b\x007 V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B%W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0B:W=`\0\x80>=`\0\xFD[PPPPP\x80\x80b\0\x0BL\x90b\x006sV[\x91PPb\0\trV[Pb\0\x0Bd`\xA5`\0b\0(\x04V[PPV[`\0`@Q\x80` \x01b\0\x0B|\x90b\0($V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP\x80Q\x90` \x01 \x90P\x90V[b\0\x0B\xA6b\0%\xCEV[`\x9BT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x0B\xDA\x90\x85\x90\x85\x90`\x04\x01b\x007\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xF5W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0C\nW=`\0\x80>=`\0\xFD[PPPPPPV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x0ChW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x84\x83\x14b\0\x0C\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0[\x85\x81\x10\x15b\0\x0E+W`\0`@Q\x80``\x01`@R\x80\x89\x89\x85\x81\x81\x10b\0\x0C\xD8Wb\0\x0C\xD8b\x003nV[\x90P` \x02\x01` \x81\x01\x90b\0\x0C\xEF\x91\x90b\x007\xE6V[c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x87\x85\x81\x81\x10b\0\r\x0FWb\0\r\x0Fb\x003nV[\x90P` \x02\x01` \x81\x01\x90b\0\r&\x91\x90b\08\x06V[`\x0F\x0B\x81R` \x01\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91P`\x14\x90P\x82`@Q` \x01b\0\r~\x91\x90b\08&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\r\x9E\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\r\xDF\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\r\xFAW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E\x0FW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x0E\"\x90b\x006sV[\x91PPb\0\x0C\xADV[PPPPPPPV[b\0\x0E>b\0%\xCEV[`\x9DT`@\x80Qc\x1Ft\xAC\x01`\xE3\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFB\xA5`\x08\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15b\0\x0E\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x0E\xAF\x91\x90b\08\x8CV[`@Qc\xFD\x8CR\xCD`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`D\x83\x01R\x91\x92P\x90\x82\x16\x90c\xFD\x8CR\xCD\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0F\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0F(W=`\0\x80>=`\0\xFD[PPPPPPPPV[b\0\x0F<b\0%\xCEV[`\x9DT`@QcV\xE4\x9E\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cV\xE4\x9E\xF3\x90b\0\x0Fr\x90\x86\x90\x86\x90\x86\x90`\x04\x01b\08\xACV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0F\x8DW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x0E+W=`\0\x80>=`\0\xFD[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x0F\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0b\0\x10\x04b\0\x1E\x9BV[\x90P`\0[\x81Q\x81\x10\x15b\0\x10\xB7W\x81\x81\x81Q\x81\x10b\0\x10(Wb\0\x10(b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x86c\xFF\xFF\xFF\xFF\x16\x03b\0\x10\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a perp p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[\x80b\0\x10\xAE\x81b\x006sV[\x91PPb\0\x10\tV[P`\xA5`@Q\x80`\x80\x01`@R\x80\x87c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85`\x0F\x0B\x81R` \x01\x84\x806\x03\x81\x01\x90b\0\x10\xF6\x91\x90b\08\xF1V[\x90R`@Qb\0\x11\n\x91\x90` \x01b\09\x10V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x0C\n\x94\x91\x90\x92\x01\x92\x01\x90b\0(2V[b\0\x11Kb\0%\xCEV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\x80\x1B\x03\x84\x16\x80\x82R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x92\x81\x01\x92\x90\x92R\x90`\0\x90`\x12\x90``\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x11\xB1\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0E\xF8\x90\x84\x90`\x04\x01b\x003YV[`\0b\0\x11\xFEb\0\x0BhV[\x7Fyt\xDFA\xBD\xCA+\xE1S\x9F\xA7\xD0\x1FA'\x7F\rr\x88#\xB2\x020\xA1\x8A1\xE4\x0Cpxt\xE7\x14b\0\x12\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FInvalid DirectDepositV1 bytecode`D\x82\x01R\x7F hash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[`\x9CT`\x9AT`\x9FT`@Q`\0\x93`\x01\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x90\x82\x16\x92\x88\x92\x90\x91\x16\x90b\0\x12\xC7\x90b\0($V[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x84\x16` \x84\x01R`@\x83\x01\x91\x90\x91R\x90\x91\x16``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15b\0\x13\x10W=`\0\x80>=`\0\xFD[P`\0\x93\x84R`\xA3` R`@\x90\x93 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90UP\x90\x91\x90PV[b\0\x13Wb\0%\xCEV[`@\x80Q`\t`\xF8\x1B` \x82\x01R\x81Q`\x01\x81\x83\x03\x01\x81R`!\x82\x01\x92\x83\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x90\x93R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x13\xA6\x90\x84\x90`%\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x13\xC1W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x13\xD6W=`\0\x80>=`\0\xFD[PPPPPV[b\0\x13\xE7b\0%\xCEV[b\0\x06\xFE`\0b\0&*V[b\0\x13\xFDb\0%\xCEV[`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x14HW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14n\x91\x90b\08\x8CV[`\x9CT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03\x85\x16`$\x82\x01R\x91\x92P\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x14\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x14\xF2\x91\x90b\09\x91V[P`@\x80Q` \x81\x01\x90\x91R`\x01`\x01`\x80\x1B\x03\x83\x16\x81R`\0`\x07`@\x80Q\x84Q`\x01`\x01`\x80\x1B\x03\x16` \x82\x01R\x01b\0\x11\x91V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15b\0\x15JWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80b\0\x15fWP0;\x15\x80\x15b\0\x15fWP`\0T`\xFF\x16`\x01\x14[b\0\x15\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01R\x7Fdy initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15b\0\x15\xFEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x88\x163\x14b\0\x16XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7Fexpected deployed to initialize\0`D\x82\x01R`d\x01b\0\x06\xE7V[b\0\x16bb\0&\x89V[b\0\x16m\x89b\0%8V[`\x99\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x17\x90\x92U`\x9A\x80T\x82\x16\x8A\x84\x16\x17\x90U`\x9B\x80T\x82\x16\x89\x84\x16\x17\x90U`\x9C\x80T\x82\x16\x88\x84\x16\x17\x90U`\x9D\x80T\x82\x16\x87\x84\x16\x17\x90U`\x9E\x80T\x82\x16\x86\x84\x16\x17\x90U`\x9F\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15b\0\x06\x8BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[b\0\x17Cb\0%\xCEV[`\x9AT`@Qc\xC5V\x07\xB5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC5V\x07\xB5\x90b\0\x0B\xDA\x90\x85\x90\x85\x90`\x04\x01b\x007\x91V[`\0\x81\x81R`\xA3` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80b\0\x17\xA3Wb\0\x17\xA0\x82b\0\x11\xF2V[\x90P[\x80`\x01`\x01`\xA0\x1B\x03\x16c&\x08o\x07`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x0B\xF5W`\0\x80\xFD[b\0\x17\xE9b\0%\xCEV[`\x9DT`@Qc\x01)\"W`\xE7\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x94\x91+\x80\x90`$\x01b\0\x13\xA6V[b\0\x18%b\0%\xCEV[\x82Q\x84Q\x14b\0\x18iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x81Q\x84Q\x14b\0\x18\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[\x80Q\x84Q\x14b\0\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rminvalid inputs`\x90\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0[\x84Q\x81\x10\x15b\0\x13\xD6W`\0`@Q\x80`\x80\x01`@R\x80\x87\x84\x81Q\x81\x10b\0\x19 Wb\0\x19 b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x84\x81Q\x81\x10b\0\x19HWb\0\x19Hb\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85\x84\x81Q\x81\x10b\0\x19pWb\0\x19pb\x003nV[` \x02` \x01\x01Q`\x0F\x0B\x81R` \x01\x84\x84\x81Q\x81\x10b\0\x19\x95Wb\0\x19\x95b\x003nV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0F\x0B\x90R\x90P`\0`\x18`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x90\x81\x16` \x80\x84\x01\x91\x90\x91R\x86\x01Q\x16\x81\x83\x01R\x90\x84\x01Q`\x0F\x90\x81\x0B``\x80\x84\x01\x91\x90\x91R\x85\x01Q\x90\x0B`\x80\x82\x01R`\xA0\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\x1A\t\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x1AJ\x90\x84\x90`\x04\x01b\x003YV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x1AeW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\x1AzW=`\0\x80>=`\0\xFD[PPPPPP\x80\x80b\0\x1A\x8D\x90b\x006sV[\x91PPb\0\x18\xF4V[`\0\x80`\x9A`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cGB\x8E{`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1A\xEDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x1B\x17\x91\x90\x81\x01\x90b\09\xB1V[\x90P`\0[\x81Q\x81\x10\x15b\0\x1ERW`\0\x82\x82\x81Q\x81\x10b\0\x1B=Wb\0\x1B=b\x003nV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x9AT`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1B\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1B\xC3\x91\x90b\08\x8CV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x1C\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x06\xE7V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x82\x91`\0\x91\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1C\x8F\x91\x90b\0:KV[`\x9FT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x84\x16\x03b\0\x1C\xC2Wb\0\x1C\xBF`\x01`\x01`\xA0\x1B\x03\x8A\x161\x82b\0:eV[\x90P[\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x1D\x01W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1D'\x91\x90b\0:\x80V[b\0\x1D4\x90`\x12b\0:\xA5V[b\0\x1DA\x90`\nb\0;\xC8V[b\0\x1DM\x90\x82b\0;\xD9V[`\x9AT`@Qc\x1D\x9B9u`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEC\xD9\xCB\xA8\x90`$\x01`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x1D\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x1D\xCA\x91\x90b\0;\xFBV[`\x80\x01Q\x90P\x88b\0\x1D\xF4Wb\0\x1D\xEB`\ng\r\xE0\xB6\xB3\xA7d\0\0b\0<\xA1V[`\x0F\x0Bb\0\x1E\rV[b\0\x1E\tg\r\xE0\xB6\xB3\xA7d\0\0`\x05b\0<\xFAV[`\x0F\x0B[b\0\x1E\x1D`\x0F\x83\x90\x0B\x84b\0'\0V[`\x0F\x0B\x12b\0\x1E7W`\x01\x97PPPPPPPPb\0\x1EYV[PPPPP\x80\x80b\0\x1EI\x90b\x006sV[\x91PPb\0\x1B\x1CV[P`\0\x91PP[\x92\x91PPV[b\0\x1Eib\0%\xCEV[`\x9ET`@Qc.\xFB\xE1-`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBB\xEF\x84\xB4\x90`$\x01b\0\x13\xA6V[`\xA5T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x1E\xBFWb\0\x1E\xBFb\0)\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0\x1E\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA5T\x81\x10\x15b\0 \tW`\0`\xA5\x82\x81T\x81\x10b\0\x1F\x12Wb\0\x1F\x12b\x003nV[\x90`\0R` `\0 \x01\x80Tb\0\x1F)\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0\x1FW\x90b\x003\x84V[\x80\x15b\0\x1F\xA8W\x80`\x1F\x10b\0\x1F|Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0\x1F\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0\x1F\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0\x1F\xC2\x91\x90b\x006\x8FV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0\x1F\xDEWb\0\x1F\xDEb\x003nV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0 \0\x81b\x006sV[\x91PPb\0\x1E\xEFV[P\x91\x90PV[b\0 \x19b\0%\xCEV[`\x9ET`@Qc/\x84\xEE\xB1`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBE\x13\xBA\xC4\x90`d\x01b\0\x0FrV[b\0 cb\0%\xCEV[`\x9DT`@Qca\x13\xED\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\xC2'\xDB\x96\x90`$\x01b\0\x13\xA6V[`\xA4T``\x90`\0\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0 \xBAWb\0 \xBAb\0)\xEAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15b\0 \xE4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\xA4T\x81\x10\x15b\0 \tW`\0`\xA4\x82\x81T\x81\x10b\0!\rWb\0!\rb\x003nV[\x90`\0R` `\0 \x01\x80Tb\0!$\x90b\x003\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Tb\0!R\x90b\x003\x84V[\x80\x15b\0!\xA3W\x80`\x1F\x10b\0!wWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91b\0!\xA3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11b\0!\x85W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x80` \x01\x90Q\x81\x01\x90b\0!\xBD\x91\x90b\x004DV[\x90P\x80`\0\x01Q\x83\x83\x81Q\x81\x10b\0!\xD9Wb\0!\xD9b\x003nV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RP\x80b\0!\xFB\x81b\x006sV[\x91PPb\0 \xEAV[b\0\"\x0Eb\0%\xCEV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x84\x16\x80\x82R`\x01`\x01`\x80\x1B\x03\x84\x81\x16` \x80\x85\x01\x91\x82R\x85Q\x90\x81\x01\x93\x90\x93RQ\x16\x92\x81\x01\x92\x90\x92R\x90`\0\x90`\x19\x90``\x01b\0\x11\x91V[b\0\"cb\0%\xCEV[`@\x80Q` \x81\x01\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R`\0`\x1B`@\x80Q\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x82\x01R\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Rb\0\"\xB5\x92\x91` \x01b\x002\xFAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\x9CTcs\x02v\xCF`\xE1\x1B\x83R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE6\x04\xED\x9E\x90b\0\x0Fr\x90\x84\x90`\x04\x01b\x003YV[`\x99T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0#LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01Rv9\xB2\xB722\xB9\x106\xBA\xB9\xBA\x1012\x9022\xB867\xBC\xB2\xB9`I\x1B`D\x82\x01R`d\x01b\0\x06\xE7V[`\0b\0#Xb\0 \x96V[\x90P`\0[\x81Q\x81\x10\x15b\0$\x0BW\x81\x81\x81Q\x81\x10b\0#|Wb\0#|b\x003nV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x88c\xFF\xFF\xFF\xFF\x16\x03b\0#\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7Ftrying to add or update a spot p`D\x82\x01Rl97\xB2:\xB1\xBA\x10:;\xB4\xB1\xB2\x97`\x99\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[\x80b\0$\x02\x81b\x006sV[\x91PPb\0#]V[P`\xA4`@Q\x80`\xC0\x01`@R\x80\x89c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\x0F\x0B\x81R` \x01\x86`\x0F\x0B\x81R` \x01\x85\x806\x03\x81\x01\x90b\0$V\x91\x90b\0=\xADV[\x81R` \x01b\0$l6\x86\x90\x03\x86\x01\x86b\08\xF1V[\x90R`@Qb\0$\x80\x91\x90` \x01b\0>wV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x81T`\x01\x81\x01\x83U`\0\x92\x83R` \x92\x83\x90 \x82Qb\0\x0F(\x94\x91\x90\x92\x01\x92\x01\x90b\0(2V[b\0$\xC1b\0%\xCEV[`@\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`\x80\x1B\x03\x84\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R`\0`\x10`@\x80Q\x84Qc\xFF\xFF\xFF\xFF\x16` \x80\x83\x01\x91\x90\x91R\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x81\x83\x01R\x90\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01b\0\x05\xFAV[b\0%Bb\0%\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0%\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0%\xCB\x81b\0&*V[PV[`gT`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x06\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x06\xE7V[`g\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0&\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0\x06\xFEb\0'\x8CV[`\0g\r\xE0\xB6\xB3\xA7d\0\0`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x02\x05o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12\x80\x15\x90b\0'HWPo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x15[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a'\xA3`\xF1\x1B\x81RP\x90b\0'\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x06\xE7\x91\x90b\x003YV[P\x93\x92PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16b\0'\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01b\0\x06\xE7V[b\0\x06\xFE3b\0&*V[P\x80T`\0\x82U\x90`\0R` `\0 \x90\x81\x01\x90b\0%\xCB\x91\x90b\0(\xC1V[a\r9\x80b\0?m\x839\x01\x90V[\x82\x80Tb\0(@\x90b\x003\x84V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82b\0(dW`\0\x85Ub\0(\xAFV[\x82`\x1F\x10b\0(\x7FW\x80Q`\xFF\x19\x16\x83\x80\x01\x17\x85Ub\0(\xAFV[\x82\x80\x01`\x01\x01\x85U\x82\x15b\0(\xAFW\x91\x82\x01[\x82\x81\x11\x15b\0(\xAFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90b\0(\x92V[Pb\0(\xBD\x92\x91Pb\0(\xE2V[P\x90V[\x80\x82\x11\x15b\0(\xBDW`\0b\0(\xD8\x82\x82b\0(\xF9V[P`\x01\x01b\0(\xC1V[[\x80\x82\x11\x15b\0(\xBDW`\0\x81U`\x01\x01b\0(\xE3V[P\x80Tb\0)\x07\x90b\x003\x84V[`\0\x82U\x80`\x1F\x10b\0)\x18WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90b\0%\xCB\x91\x90b\0(\xE2V[`\0` \x82\x84\x03\x12\x15b\0)KW`\0\x80\xFD[P5\x91\x90PV[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0)kW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0%\xCBW`\0\x80\xFD[\x805`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14b\0)kW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15b\0)\xB4W`\0\x80\xFD[b\0)\xBF\x84b\0)RV[\x92P` \x84\x015b\0)\xD1\x81b\0)pV[\x91Pb\0)\xE1`@\x85\x01b\0)\x86V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@R\x90V[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*&Wb\0*&b\0)\xEAV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15b\0*\xA4Wb\0*\xA4b\0)\xEAV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0*\xC9Wb\0*\xC9b\0)\xEAV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0%\xCBW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12b\0*\xF8W`\0\x80\xFD[\x815` b\0+\x11b\0+\x0B\x83b\0*\xACV[b\0*xV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0+1W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0+YW\x805b\0+K\x81b\0*\xD3V[\x83R\x91\x83\x01\x91\x83\x01b\0+5V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15b\0+xW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0+\x91W`\0\x80\xFD[b\0+\x9F\x86\x83\x87\x01b\0*\xE6V[\x93P` \x85\x015\x91P\x80\x82\x11\x15b\0+\xB6W`\0\x80\xFD[Pb\0+\xC5\x85\x82\x86\x01b\0*\xE6V[\x91PP\x92P\x92\x90PV[\x80`\x03\x0B\x81\x14b\0%\xCBW`\0\x80\xFD[\x80`\x0F\x0B\x81\x14b\0%\xCBW`\0\x80\xFD[\x805b\0)k\x81b\0+\xDFV[`\0`\xA0\x82\x84\x03\x12\x15b\0,\x0FW`\0\x80\xFD[b\0,\x19b\0*\0V[\x90P\x815b\0,(\x81b\0+\xCFV[\x81R` \x82\x015b\0,:\x81b\0+\xCFV[` \x82\x01R`@\x82\x015b\0,O\x81b\0+\xCFV[`@\x82\x01R``\x82\x015b\0,d\x81b\0+\xCFV[``\x82\x01R`\x80\x82\x015b\0,y\x81b\0+\xDFV[`\x80\x82\x01R\x92\x91PPV[`\0\x80`\xC0\x83\x85\x03\x12\x15b\0,\x98W`\0\x80\xFD[\x825b\0,\xA5\x81b\0*\xD3V[\x91Pb\0,\xB6\x84` \x85\x01b\0+\xFCV[\x90P\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12b\0,\xD2W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0,\xEBW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15b\0-\x07W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15b\0-(W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\0-AW`\0\x80\xFD[b\0-O\x8A\x83\x8B\x01b\0,\xBFV[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15b\0-iW`\0\x80\xFD[b\0-w\x8A\x83\x8B\x01b\0,\xBFV[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15b\0-\x91W`\0\x80\xFD[Pb\0-\xA0\x89\x82\x8A\x01b\0,\xBFV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0-\xC8W`\0\x80\xFD[\x835b\0-\xD5\x81b\0*\xD3V[\x92Pb\0-\xE5` \x85\x01b\0)\x86V[\x91P`@\x84\x015b\0-\xF7\x81b\0)pV[\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15b\0.\x18W`\0\x80\xFD[\x835b\0.%\x81b\0)pV[\x92P` \x84\x015b\0.7\x81b\0)pV[\x91P`@\x84\x015`\x02\x81\x10b\0-\xF7W`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15b\0 \tW`\0\x80\xFD[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15b\0.wW`\0\x80\xFD[\x845b\0.\x84\x81b\0*\xD3V[\x93P` \x85\x015b\0.\x96\x81b\0+\xDFV[\x92P`@\x85\x015b\0.\xA8\x81b\0+\xDFV[\x91Pb\0.\xB9\x86``\x87\x01b\0.LV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15b\0.\xD8W`\0\x80\xFD[b\0.\xE3\x83b\0)\x86V[\x91P` \x83\x015b\0.\xF5\x81b\0)pV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15b\0/\x13W`\0\x80\xFD[b\0/\x1E\x82b\0)\x86V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15b\0/CW`\0\x80\xFD[\x885b\0/P\x81b\0)pV[\x97P` \x89\x015b\0/b\x81b\0)pV[\x96P`@\x89\x015b\0/t\x81b\0)pV[\x95P``\x89\x015b\0/\x86\x81b\0)pV[\x94P`\x80\x89\x015b\0/\x98\x81b\0)pV[\x93P`\xA0\x89\x015b\0/\xAA\x81b\0)pV[\x92P`\xC0\x89\x015b\0/\xBC\x81b\0)pV[\x91P`\xE0\x89\x015b\0/\xCE\x81b\0)pV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x82`\x1F\x83\x01\x12b\0/\xF1W`\0\x80\xFD[\x815` b\x000\x04b\0+\x0B\x83b\0*\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\x000$W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0+YW\x805b\x000>\x81b\0+\xDFV[\x83R\x91\x83\x01\x91\x83\x01b\x000(V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\x000cW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15b\x000|W`\0\x80\xFD[b\x000\x8A\x88\x83\x89\x01b\0*\xE6V[\x95P` \x87\x015\x91P\x80\x82\x11\x15b\x000\xA1W`\0\x80\xFD[b\x000\xAF\x88\x83\x89\x01b\0*\xE6V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15b\x000\xC6W`\0\x80\xFD[b\x000\xD4\x88\x83\x89\x01b\0/\xDFV[\x93P``\x87\x015\x91P\x80\x82\x11\x15b\x000\xEBW`\0\x80\xFD[Pb\x000\xFA\x87\x82\x88\x01b\0/\xDFV[\x91PP\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14b\0%\xCBW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15b\x001)W`\0\x80\xFD[\x825b\x0016\x81b\0)pV[\x91P` \x83\x015b\0.\xF5\x81b\x001\x06V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15b\x001\x88W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\x001dV[P\x90\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\x001\xAAW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\x001\xD4W`\0\x80\xFD[\x815b\0/\x1E\x81b\0)pV[`\0\x80`@\x83\x85\x03\x12\x15b\x001\xF5W`\0\x80\xFD[\x825b\x002\x02\x81b\0)pV[\x91Pb\0,\xB6` \x84\x01b\0)\x86V[`\0` \x82\x84\x03\x12\x15b\x002%W`\0\x80\xFD[b\0/\x1E\x82b\0)RV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x02\0\x81\x12\x15b\x002LW`\0\x80\xFD[\x875b\x002Y\x81b\0*\xD3V[\x96P` \x88\x015b\x002k\x81b\0*\xD3V[\x95P`@\x88\x015b\x002}\x81b\0+\xDFV[\x94P``\x88\x015b\x002\x8F\x81b\0+\xDFV[\x93P`\xE0`\x7F\x19\x82\x01\x12\x15b\x002\xA4W`\0\x80\xFD[P`\x80\x87\x01\x91Pb\x002\xBB\x88a\x01`\x89\x01b\0.LV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0[\x83\x81\x10\x15b\x002\xE4W\x81\x81\x01Q\x83\x82\x01R` \x01b\x002\xCAV[\x83\x81\x11\x15b\x002\xF4W`\0\x84\x84\x01R[PPPPV[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R`\0\x82Qb\x003\x1D\x81`\x01\x85\x01` \x87\x01b\x002\xC7V[\x91\x90\x91\x01`\x01\x01\x93\x92PPPV[`\0\x81Q\x80\x84Rb\x003E\x81` \x86\x01` \x86\x01b\x002\xC7V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0b\0/\x1E` \x83\x01\x84b\x003+V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\x003\x99W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0 \tWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x80Qb\0)k\x81b\0+\xDFV[`\0`\xA0\x82\x84\x03\x12\x15b\x003\xDAW`\0\x80\xFD[b\x003\xE4b\0*\0V[\x90P\x81Qb\x003\xF3\x81b\0+\xCFV[\x81R` \x82\x01Qb\x004\x05\x81b\0+\xCFV[` \x82\x01R`@\x82\x01Qb\x004\x1A\x81b\0+\xCFV[`@\x82\x01R``\x82\x01Qb\x004/\x81b\0+\xCFV[``\x82\x01R`\x80\x82\x01Qb\0,y\x81b\0+\xDFV[`\0\x81\x83\x03a\x02\0\x81\x12\x15b\x004YW`\0\x80\xFD[b\x004cb\0*,V[\x83Qb\x004p\x81b\0*\xD3V[\x81R` \x84\x01Qb\x004\x82\x81b\0*\xD3V[` \x82\x01R`@\x84\x01Qb\x004\x97\x81b\0+\xDFV[`@\x82\x01R``\x84\x01Qb\x004\xAC\x81b\0+\xDFV[``\x82\x01R`\xE0`\x7F\x19\x83\x01\x12\x15b\x004\xC4W`\0\x80\xFD[b\x004\xCEb\0*RV[\x91P`\x80\x84\x01Qb\x004\xE0\x81b\0)pV[\x82R`\xA0\x84\x01Qb\x004\xF2\x81b\0+\xDFV[` \x83\x01R`\xC0\x84\x01Qb\x005\x07\x81b\0+\xDFV[`@\x83\x01R`\xE0\x84\x01Qb\x005\x1C\x81b\0+\xDFV[``\x83\x01Rb\x0050a\x01\0\x85\x01b\x003\xBAV[`\x80\x83\x01Rb\x005Da\x01 \x85\x01b\x003\xBAV[`\xA0\x83\x01Rb\x005Xa\x01@\x85\x01b\x003\xBAV[`\xC0\x83\x01R\x81`\x80\x82\x01Rb\x005s\x85a\x01`\x86\x01b\x003\xC7V[`\xA0\x82\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x0F\x85\x81\x0B`@\x83\x01R\x84\x90\x0B``\x82\x01Ra\x02\0\x81\x01b\x006\x0F`\x80\x83\x01\x85`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[\x82Q`\x03\x90\x81\x0Ba\x01`\x84\x01R` \x84\x01Q\x81\x0Ba\x01\x80\x84\x01R`@\x84\x01Q\x81\x0Ba\x01\xA0\x84\x01R``\x84\x01Q\x90\x0Ba\x01\xC0\x83\x01R`\x80\x83\x01Q`\x0F\x0Ba\x01\xE0\x83\x01R[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\x006\x88Wb\x006\x88b\x006]V[P`\x01\x01\x90V[`\0a\x01\0\x82\x84\x03\x12\x15b\x006\xA3W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\x006\xC9Wb\x006\xC9b\0)\xEAV[`@R\x82Qb\x006\xD9\x81b\0*\xD3V[\x81R` \x83\x01Qb\x006\xEB\x81b\0+\xDFV[` \x82\x01R`@\x83\x01Qb\x007\0\x81b\0+\xDFV[`@\x82\x01Rb\x007\x14\x84``\x85\x01b\x003\xC7V[``\x82\x01R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x85\x16\x81R`\x0F\x84\x81\x0B` \x83\x01R\x83\x90\x0B`@\x82\x01Ra\x01\0\x81\x01b\x007\x88``\x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[\x95\x94PPPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`\xC0\x81\x01b\0/\x1E` \x83\x01\x84\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[`\0` \x82\x84\x03\x12\x15b\x007\xF9W`\0\x80\xFD[\x815b\0/\x1E\x81b\0*\xD3V[`\0` \x82\x84\x03\x12\x15b\08\x19W`\0\x80\xFD[\x815b\0/\x1E\x81b\0+\xDFV[`\0` \x80\x83R`\x80\x83\x01c\xFF\xFF\xFF\xFF\x85Q\x16\x82\x85\x01R\x81\x85\x01Q`\x0F\x0B`@\x85\x01R`@\x85\x01Q``\x80\x86\x01R\x81\x81Q\x80\x84R`\xA0\x87\x01\x91P\x84\x83\x01\x93P`\0\x92P[\x80\x83\x10\x15b\0+YW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90b\08jV[`\0` \x82\x84\x03\x12\x15b\08\x9FW`\0\x80\xFD[\x81Qb\0/\x1E\x81b\0)pV[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R\x83\x16` \x82\x01R``\x81\x01`\x02\x83\x10b\08\xE3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x82`@\x83\x01R\x94\x93PPPPV[`\0`\xA0\x82\x84\x03\x12\x15b\09\x04W`\0\x80\xFD[b\0/\x1E\x83\x83b\0+\xFCV[`\0a\x01\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x83Q\x16\x82R` \x83\x01Q`\x0F\x0B` \x83\x01R`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Qb\09\x8A``\x84\x01\x82\x80Q`\x03\x0B\x82R` \x81\x01Q`\x03\x0B` \x83\x01R`@\x81\x01Q`\x03\x0B`@\x83\x01R``\x81\x01Q`\x03\x0B``\x83\x01R`\x80\x81\x01Q`\x0F\x0B`\x80\x83\x01RPPV[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\09\xA4W`\0\x80\xFD[\x81Qb\0/\x1E\x81b\x001\x06V[`\0` \x80\x83\x85\x03\x12\x15b\09\xC5W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\09\xDDW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13b\09\xEFW`\0\x80\xFD[\x80Qb\0:\0b\0+\x0B\x82b\0*\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15b\0: W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15b\x006RW\x83Qb\0:;\x81b\0*\xD3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90b\0:%V[`\0` \x82\x84\x03\x12\x15b\0:^W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15b\0:{Wb\0:{b\x006]V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0:\x93W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0/\x1EW`\0\x80\xFD[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0:\xC2Wb\0:\xC2b\x006]V[\x90\x03\x93\x92PPPV[`\x01\x81\x81[\x80\x85\x11\x15b\0;\x0CW\x81`\0\x19\x04\x82\x11\x15b\0:\xF0Wb\0:\xF0b\x006]V[\x80\x85\x16\x15b\0:\xFEW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\0:\xD0V[P\x92P\x92\x90PV[`\0\x82b\0;%WP`\x01b\0\x1EYV[\x81b\0;4WP`\0b\0\x1EYV[\x81`\x01\x81\x14b\0;MW`\x02\x81\x14b\0;XWb\0;xV[`\x01\x91PPb\0\x1EYV[`\xFF\x84\x11\x15b\0;lWb\0;lb\x006]V[PP`\x01\x82\x1Bb\0\x1EYV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0;\x9DWP\x81\x81\nb\0\x1EYV[b\0;\xA9\x83\x83b\0:\xCBV[\x80`\0\x19\x04\x82\x11\x15b\0;\xC0Wb\0;\xC0b\x006]V[\x02\x93\x92PPPV[`\0b\0/\x1E`\xFF\x84\x16\x83b\0;\x14V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0;\xF6Wb\0;\xF6b\x006]V[P\x02\x90V[`\0`\xA0\x82\x84\x03\x12\x15b\0<\x0EW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0<4Wb\0<4b\0)\xEAV[`@R\x82Qb\0<D\x81b\0+\xDFV[\x81R` \x83\x01Qb\0<V\x81b\0+\xDFV[` \x82\x01R`@\x83\x01Qb\0<k\x81b\0+\xDFV[`@\x82\x01R``\x83\x01Qb\0<\x80\x81b\0+\xDFV[``\x82\x01R`\x80\x83\x01Qb\0<\x95\x81b\0+\xDFV[`\x80\x82\x01R\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B\x80b\0<\xC7WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x14`\0\x19\x82\x14\x16\x15b\0<\xF1Wb\0<\xF1b\x006]V[\x90\x05\x93\x92PPPV[`\0\x81`\x0F\x0B\x83`\x0F\x0Bo\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\0\x82\x13`\0\x84\x13\x83\x83\x04\x85\x11\x82\x82\x16\x16\x15b\0=6Wb\0=6b\x006]V[o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19`\0\x85\x12\x82\x81\x16\x87\x83\x05\x87\x12\x16\x15b\0=eWb\0=eb\x006]V[`\0\x87\x12\x92P\x85\x82\x05\x87\x12\x84\x84\x16\x16\x15b\0=\x84Wb\0=\x84b\x006]V[\x85\x85\x05\x87\x12\x81\x84\x16\x16\x15b\0=\x9DWb\0=\x9Db\x006]V[PPP\x92\x90\x91\x02\x95\x94PPPPPV[`\0`\xE0\x82\x84\x03\x12\x15b\0=\xC0W`\0\x80\xFD[`@Q`\xE0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0=\xE6Wb\0=\xE6b\0)\xEAV[`@R\x825b\0=\xF6\x81b\0)pV[\x81R` \x83\x015b\0>\x08\x81b\0+\xDFV[` \x82\x01R`@\x83\x015b\0>\x1D\x81b\0+\xDFV[`@\x82\x01R``\x83\x015b\0>2\x81b\0+\xDFV[``\x82\x01Rb\0>E`\x80\x84\x01b\0+\xEFV[`\x80\x82\x01Rb\0>X`\xA0\x84\x01b\0+\xEFV[`\xA0\x82\x01Rb\0>k`\xC0\x84\x01b\0+\xEFV[`\xC0\x82\x01R\x93\x92PPPV[`\0a\x02\0\x82\x01\x90Pc\xFF\xFF\xFF\xFF\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01RP`@\x83\x01Q`\x0F\x0B`@\x83\x01R``\x83\x01Q`\x0F\x0B``\x83\x01R`\x80\x83\x01Qb\0?\x1E`\x80\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R` \x81\x01Q`\x0F\x81\x81\x0B` \x85\x01R`@\x83\x01Q\x81\x0B`@\x85\x01R``\x83\x01Q\x81\x0B``\x85\x01R`\x80\x83\x01Q\x81\x0B`\x80\x85\x01R`\xA0\x83\x01Q\x81\x0B`\xA0\x85\x01R`\xC0\x83\x01Q\x81\x0B`\xC0\x85\x01RPPPPV[P`\xA0\x83\x01Q\x80Q`\x03\x90\x81\x0Ba\x01`\x85\x01R` \x82\x01Q\x81\x0Ba\x01\x80\x85\x01R`@\x82\x01Q\x81\x0Ba\x01\xA0\x85\x01R``\x82\x01Q\x90\x0Ba\x01\xC0\x84\x01R`\x80\x81\x01Q`\x0F\x0Ba\x01\xE0\x84\x01Rb\09\x8AV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r98\x03\x80a\r9\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xBEV[a\083a\x01VV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x02\x80T\x86\x84\x16\x90\x83\x16\x17\x90U`\x03\x84\x90U`\x04\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90UG\x80\x15a\x01\x12W`\x04T`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xCEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xD3V[``\x91P[PP\x90P\x80a\x01\x10W`@Q\x82\x81R\x7F\t\xE9\xD3\x9E)\xBF\x10\xAEZ\xBC\x1C@\xBE.\xEF\x8E\x82\xEDw\xC5\x95\x81J\x1E\xC9_\xC3\xBEa?24\x90` \x01`@Q\x80\x91\x03\x90\xA1[P[`\x03T`@\x80Q0\x81R\x90Q`\x01\x91\x7F\xBB\xC0\x9A\xD7\xAE}ZC~\xEA)\x01{\x8B1\x9D\xA6\xB6\xE07\x11s\x1B[\xCD\xD4;\xEC<\x19K\x17\x91\x90\x81\x90\x03` \x01\x90\xA3PPPPPa\x02\x13V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBBW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\xD4W`\0\x80\xFD[\x84Qa\x01\xDF\x81a\x01\xA6V[` \x86\x01Q\x90\x94Pa\x01\xF0\x81a\x01\xA6V[`@\x86\x01Q``\x87\x01Q\x91\x94P\x92Pa\x02\x08\x81a\x01\xA6V[\x93\x96\x92\x95P\x90\x93PPV[a\x0B\x17\x80a\x02\"`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80cT\xFDMP\x11a\0NW\x80cT\xFDMP\x14a\x01rW\x80cqP\x18\xA6\x14a\x01\x93W\x80c\x8D\xA5\xCB[\x14a\x01\xA8W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD0W`\0\x80\xFD[\x80c&\x08o\x07\x14a\x01(W\x80cPC\x1C\xE4\x14a\x01=W\x80cQ\xCF\xF8\xD9\x14a\x01RW`\0\x80\xFD[6a\x01#W`\x04T`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x904\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\0\xC6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\0\xCBV[``\x91P[PP\x90P\x80a\x01!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFailed to wrap native token.\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x014W`\0\x80\xFD[Pa\x01!a\x01\xF0V[4\x80\x15a\x01IW`\0\x80\xFD[Pa\x01!a\x05\x06V[4\x80\x15a\x01^W`\0\x80\xFD[Pa\x01!a\x01m6`\x04a\x08\xF9V[a\x05\xCEV[4\x80\x15a\x01~W`\0\x80\xFD[P`@Q`\x01\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x9FW`\0\x80\xFD[Pa\x01!a\x06NV[4\x80\x15a\x01\xB4W`\0\x80\xFD[P`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x8AV[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\x01!a\x01\xEB6`\x04a\x08\xF9V[a\x06bV[`\x02T`@\x80QcGB\x8E{`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cGB\x8E{\x91`\x04\x80\x83\x01\x92\x86\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x029W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02a\x91\x90\x81\x01\x90a\tLV[\x90P`\0[\x81Q\x81\x10\x15a\x05\x02W`\0\x82\x82\x81Q\x81\x10a\x02\x83Wa\x02\x83a\n\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x02T`@Qc\"\xDF?k`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x91\x92P`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cE\xBE~\xD6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x06\x91\x90a\n'V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x03^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01R\x7FInvalid productId.\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x18V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCB\x91\x90a\nDV[\x90P\x80\x15a\x04\xEBW`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x90\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04J\x91\x90a\n]V[P`\x01T`\x03T`@Qc\"\x1F\t9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x16`$\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`D\x82\x01R`\x80`d\x82\x01R`\x02`\x84\x82\x01Ra-1`\xF0\x1B`\xA4\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\"\x1F\t9\x90`\xC4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\xD2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xE6W=`\0\x80>=`\0\xFD[PPPP[PPPP\x80\x80a\x04\xFA\x90a\n\x7FV[\x91PPa\x02fV[PPV[a\x05\x0Ea\x06\xF2V[`@QG\x90`\0\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05RW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05WV[``\x91P[PP\x90P\x80a\x05\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FFailed to transfer native token `D\x82\x01R\x7Fto owner\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x18V[a\x05\xD6a\x06\xF2V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06A\x91\x90a\nDV[\x90Pa\x05\x02\x823\x83a\x07LV[a\x06Va\x06\xF2V[a\x06``\0a\x08|V[V[a\x06ja\x06\xF2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x06\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x01\x18V[a\x06\xEF\x81a\x08|V[PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x01\x18V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90R\x91Q`\0\x92\x83\x92\x90\x87\x16\x91a\x07\xBD\x91\x90a\n\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\xFAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xFFV[``\x91P[P\x91P\x91P\x81\x80\x15a\x08)WP\x80Q\x15\x80a\x08)WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08)\x91\x90a\n]V[a\x08uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTransfer failed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x18V[PPPPPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\t\x0BW`\0\x80\xFD[\x815a\t\x16\x81a\x08\xE4V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\tGW`\0\x80\xFD[\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\t_W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\twW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t\x8BW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\t\x9DWa\t\x9Da\t\x1DV[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\t\xC2Wa\t\xC2a\t\x1DV[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x88\x83\x11\x15a\t\xE0W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\n\x05Wa\t\xF6\x85a\t3V[\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\t\xE5V[\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\n9W`\0\x80\xFD[\x81Qa\t\x16\x81a\x08\xE4V[`\0` \x82\x84\x03\x12\x15a\nVW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\noW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\t\x16W`\0\x80\xFD[`\0`\x01\x82\x01a\n\x9FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0\x82Q`\0[\x81\x81\x10\x15a\n\xC7W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\n\xADV[\x81\x81\x11\x15a\n\xD6W`\0\x82\x85\x01R[P\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE0\xDD\x03\xCF\tMG\xE3'\x1A\x02\xF0df:\n\x8F\x0C\x86:\xBAn\x9C\x8D\xAB\xCCZ\x8F/\xE4\xFE\xA6dsolcC\0\x08\r\x003\xA2dipfsX\"\x12 \x16\x8B).\r\x8B\xA2?Iqg{C\x8B\x83\xAB\x90LpY\xC0g(\xD0\x0C+|i\x96Qn\x9BdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static CONTRACTOWNER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ContractOwner<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ContractOwner<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ContractOwner<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ContractOwner<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ContractOwner<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ContractOwner))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ContractOwner<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CONTRACTOWNER_ABI.clone(),
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
                CONTRACTOWNER_ABI.clone(),
                CONTRACTOWNER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addEngine` (0x56e49ef3) function
        pub fn add_engine(
            &self,
            engine: ::ethers::core::types::Address,
            offchain_exchange: ::ethers::core::types::Address,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [86, 228, 158, 243],
                    (engine, offchain_exchange, engine_type),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addNlpPool` (0xcc74547c) function
        pub fn add_nlp_pool(
            &self,
            owner: ::ethers::core::types::Address,
            balance_weight_x18: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 116, 84, 124], (owner, balance_weight_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addOrUpdateProducts` (0x2b126c5f) function
        pub fn add_or_update_products(
            &self,
            spot_ids: ::std::vec::Vec<u32>,
            perp_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 18, 108, 95], (spot_ids, perp_ids))
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
        ///Calls the contract's `clearPerpAddOrUpdateProductCalls` (0x174b8d5e) function
        pub fn clear_perp_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 75, 141, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearSpotAddOrUpdateProductCalls` (0x1962c384) function
        pub fn clear_spot_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 98, 195, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDirectDepositV1` (0x6e13cbf3) function
        pub fn create_direct_deposit_v1(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([110, 19, 203, 243], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `creditDepositV1` (0x9086a2a5) function
        pub fn credit_deposit_v1(
            &self,
            subaccount: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 134, 162, 165], subaccount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deleteNlpPool` (0xd4d5344f) function
        pub fn delete_nlp_pool(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 213, 52, 79], pool_id)
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
        ///Calls the contract's `delistProduct` (0x3dabe0d9) function
        pub fn delist_product(
            &self,
            product_ids: ::std::vec::Vec<u32>,
            prices_x18: ::std::vec::Vec<i128>,
            subaccounts: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 171, 224, 217], (product_ids, prices_x18, subaccounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositInsurance` (0x860e9674) function
        pub fn deposit_insurance(
            &self,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 14, 150, 116], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `directDepositV1Address` (0x12efa71b) function
        pub fn direct_deposit_v1_address(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([18, 239, 167, 27], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dumpFees` (0x707c8b58) function
        pub fn dump_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 124, 139, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDirectDepositV1BytecodeHash` (0x2fb0523a) function
        pub fn get_direct_deposit_v1_bytecode_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 176, 82, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPendingAddOrUpdateProductCalls` (0x145ca380) function
        pub fn has_pending_add_or_update_product_calls(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 92, 163, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x8a29e2de) function
        pub fn initialize(
            &self,
            multisig: ::ethers::core::types::Address,
            deployer: ::ethers::core::types::Address,
            spot_engine: ::ethers::core::types::Address,
            perp_engine: ::ethers::core::types::Address,
            endpoint: ::ethers::core::types::Address,
            clearinghouse: ::ethers::core::types::Address,
            verifier: ::ethers::core::types::Address,
            wrapped_native: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [138, 41, 226, 222],
                    (
                        multisig,
                        deployer,
                        spot_engine,
                        perp_engine,
                        endpoint,
                        clearinghouse,
                        verifier,
                        wrapped_native,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDirectDepositV1Ready` (0xa7e69da3) function
        pub fn is_direct_deposit_v1_ready(
            &self,
            recipient: ::ethers::core::types::Address,
            is_first_deposit: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([167, 230, 157, 163], (recipient, is_first_deposit))
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
        ///Calls the contract's `pendingPerpAddOrUpdateProductIds` (0xbe05a69d) function
        pub fn pending_perp_add_or_update_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([190, 5, 166, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingSpotAddOrUpdateProductIds` (0xc9c5efaa) function
        pub fn pending_spot_add_or_update_product_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([201, 197, 239, 170], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `perpUpdateRisk` (0x3392c585) function
        pub fn perp_update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 146, 197, 133], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalanceXWithdraw` (0xebd6c294) function
        pub fn rebalance_x_withdraw(
            &self,
            product_id: u32,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 214, 194, 148], (product_id, amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeWithdrawPoolLiquidity` (0x5312b91f) function
        pub fn remove_withdraw_pool_liquidity(
            &self,
            product_id: u32,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 18, 185, 31], (product_id, amount, send_to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSpreads` (0x94912b80) function
        pub fn set_spreads(
            &self,
            spreads: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 145, 43, 128], spreads)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawPool` (0xc227db96) function
        pub fn set_withdraw_pool(
            &self,
            withdraw_pool: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 39, 219, 150], withdraw_pool)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spotUpdateRisk` (0x8ab3daae) function
        pub fn spot_update_risk(
            &self,
            product_id: u32,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 179, 218, 174], (product_id, risk_store))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitPerpAddOrUpdateProductCall` (0x6c555b1b) function
        pub fn submit_perp_add_or_update_product_call(
            &self,
            product_id: u32,
            size_increment: i128,
            min_size: i128,
            risk_store: RiskStore,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [108, 85, 91, 27],
                    (product_id, size_increment, min_size, risk_store),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitSpotAddOrUpdateProductCall` (0xd9fb99c1) function
        pub fn submit_spot_add_or_update_product_call(
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
                    [217, 251, 153, 193],
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateNlpPool` (0x146fe5b5) function
        pub fn update_nlp_pool(
            &self,
            pool_id: u64,
            owner: ::ethers::core::types::Address,
            balance_weight_x18: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 111, 229, 181], (pool_id, owner, balance_weight_x18))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateTierFeeRates` (0x9b6aba8f) function
        pub fn update_tier_fee_rates(
            &self,
            tier: ::std::vec::Vec<u32>,
            product_id: ::std::vec::Vec<u32>,
            maker_rate_x18: ::std::vec::Vec<i128>,
            taker_rate_x18: ::std::vec::Vec<i128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 106, 186, 143],
                    (tier, product_id, maker_rate_x18, taker_rate_x18),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawInsurance` (0x6c6c654e) function
        pub fn withdraw_insurance(
            &self,
            amount: u128,
            send_to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 108, 101, 78], (amount, send_to))
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ContractOwnerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ContractOwner<M>
    {
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
    pub enum ContractOwnerEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for ContractOwnerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(ContractOwnerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ContractOwnerEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ContractOwnerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for ContractOwnerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ContractOwnerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `addEngine` function with signature `addEngine(address,address,uint8)` and selector `0x56e49ef3`
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
    #[ethcall(name = "addEngine", abi = "addEngine(address,address,uint8)")]
    pub struct AddEngineCall {
        pub engine: ::ethers::core::types::Address,
        pub offchain_exchange: ::ethers::core::types::Address,
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `addNlpPool` function with signature `addNlpPool(address,uint128)` and selector `0xcc74547c`
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
    #[ethcall(name = "addNlpPool", abi = "addNlpPool(address,uint128)")]
    pub struct AddNlpPoolCall {
        pub owner: ::ethers::core::types::Address,
        pub balance_weight_x18: u128,
    }
    ///Container type for all input parameters for the `addOrUpdateProducts` function with signature `addOrUpdateProducts(uint32[],uint32[])` and selector `0x2b126c5f`
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
        name = "addOrUpdateProducts",
        abi = "addOrUpdateProducts(uint32[],uint32[])"
    )]
    pub struct AddOrUpdateProductsCall {
        pub spot_ids: ::std::vec::Vec<u32>,
        pub perp_ids: ::std::vec::Vec<u32>,
    }
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
    ///Container type for all input parameters for the `clearPerpAddOrUpdateProductCalls` function with signature `clearPerpAddOrUpdateProductCalls()` and selector `0x174b8d5e`
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
        name = "clearPerpAddOrUpdateProductCalls",
        abi = "clearPerpAddOrUpdateProductCalls()"
    )]
    pub struct ClearPerpAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `clearSpotAddOrUpdateProductCalls` function with signature `clearSpotAddOrUpdateProductCalls()` and selector `0x1962c384`
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
        name = "clearSpotAddOrUpdateProductCalls",
        abi = "clearSpotAddOrUpdateProductCalls()"
    )]
    pub struct ClearSpotAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `createDirectDepositV1` function with signature `createDirectDepositV1(bytes32)` and selector `0x6e13cbf3`
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
    #[ethcall(name = "createDirectDepositV1", abi = "createDirectDepositV1(bytes32)")]
    pub struct CreateDirectDepositV1Call {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `creditDepositV1` function with signature `creditDepositV1(bytes32)` and selector `0x9086a2a5`
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
    #[ethcall(name = "creditDepositV1", abi = "creditDepositV1(bytes32)")]
    pub struct CreditDepositV1Call {
        pub subaccount: [u8; 32],
    }
    ///Container type for all input parameters for the `deleteNlpPool` function with signature `deleteNlpPool(uint64)` and selector `0xd4d5344f`
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
    #[ethcall(name = "deleteNlpPool", abi = "deleteNlpPool(uint64)")]
    pub struct DeleteNlpPoolCall {
        pub pool_id: u64,
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
    ///Container type for all input parameters for the `delistProduct` function with signature `delistProduct(uint32[],int128[],bytes32[])` and selector `0x3dabe0d9`
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
        name = "delistProduct",
        abi = "delistProduct(uint32[],int128[],bytes32[])"
    )]
    pub struct DelistProductCall {
        pub product_ids: ::std::vec::Vec<u32>,
        pub prices_x18: ::std::vec::Vec<i128>,
        pub subaccounts: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `depositInsurance` function with signature `depositInsurance(uint128)` and selector `0x860e9674`
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
    #[ethcall(name = "depositInsurance", abi = "depositInsurance(uint128)")]
    pub struct DepositInsuranceCall {
        pub amount: u128,
    }
    ///Container type for all input parameters for the `directDepositV1Address` function with signature `directDepositV1Address(bytes32)` and selector `0x12efa71b`
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
        name = "directDepositV1Address",
        abi = "directDepositV1Address(bytes32)"
    )]
    pub struct DirectDepositV1AddressCall(pub [u8; 32]);
    ///Container type for all input parameters for the `dumpFees` function with signature `dumpFees()` and selector `0x707c8b58`
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
    #[ethcall(name = "dumpFees", abi = "dumpFees()")]
    pub struct DumpFeesCall;
    ///Container type for all input parameters for the `getDirectDepositV1BytecodeHash` function with signature `getDirectDepositV1BytecodeHash()` and selector `0x2fb0523a`
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
        name = "getDirectDepositV1BytecodeHash",
        abi = "getDirectDepositV1BytecodeHash()"
    )]
    pub struct GetDirectDepositV1BytecodeHashCall;
    ///Container type for all input parameters for the `hasPendingAddOrUpdateProductCalls` function with signature `hasPendingAddOrUpdateProductCalls()` and selector `0x145ca380`
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
        name = "hasPendingAddOrUpdateProductCalls",
        abi = "hasPendingAddOrUpdateProductCalls()"
    )]
    pub struct HasPendingAddOrUpdateProductCallsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,address,address,address,address)` and selector `0x8a29e2de`
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
        name = "initialize",
        abi = "initialize(address,address,address,address,address,address,address,address)"
    )]
    pub struct InitializeCall {
        pub multisig: ::ethers::core::types::Address,
        pub deployer: ::ethers::core::types::Address,
        pub spot_engine: ::ethers::core::types::Address,
        pub perp_engine: ::ethers::core::types::Address,
        pub endpoint: ::ethers::core::types::Address,
        pub clearinghouse: ::ethers::core::types::Address,
        pub verifier: ::ethers::core::types::Address,
        pub wrapped_native: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isDirectDepositV1Ready` function with signature `isDirectDepositV1Ready(address,bool)` and selector `0xa7e69da3`
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
        name = "isDirectDepositV1Ready",
        abi = "isDirectDepositV1Ready(address,bool)"
    )]
    pub struct IsDirectDepositV1ReadyCall {
        pub recipient: ::ethers::core::types::Address,
        pub is_first_deposit: bool,
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
    ///Container type for all input parameters for the `pendingPerpAddOrUpdateProductIds` function with signature `pendingPerpAddOrUpdateProductIds()` and selector `0xbe05a69d`
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
        name = "pendingPerpAddOrUpdateProductIds",
        abi = "pendingPerpAddOrUpdateProductIds()"
    )]
    pub struct PendingPerpAddOrUpdateProductIdsCall;
    ///Container type for all input parameters for the `pendingSpotAddOrUpdateProductIds` function with signature `pendingSpotAddOrUpdateProductIds()` and selector `0xc9c5efaa`
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
        name = "pendingSpotAddOrUpdateProductIds",
        abi = "pendingSpotAddOrUpdateProductIds()"
    )]
    pub struct PendingSpotAddOrUpdateProductIdsCall;
    ///Container type for all input parameters for the `perpUpdateRisk` function with signature `perpUpdateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0x3392c585`
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
        name = "perpUpdateRisk",
        abi = "perpUpdateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct PerpUpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `rebalanceXWithdraw` function with signature `rebalanceXWithdraw(uint32,uint128,address)` and selector `0xebd6c294`
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
        name = "rebalanceXWithdraw",
        abi = "rebalanceXWithdraw(uint32,uint128,address)"
    )]
    pub struct RebalanceXWithdrawCall {
        pub product_id: u32,
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeWithdrawPoolLiquidity` function with signature `removeWithdrawPoolLiquidity(uint32,uint128,address)` and selector `0x5312b91f`
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
        name = "removeWithdrawPoolLiquidity",
        abi = "removeWithdrawPoolLiquidity(uint32,uint128,address)"
    )]
    pub struct RemoveWithdrawPoolLiquidityCall {
        pub product_id: u32,
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `setSpreads` function with signature `setSpreads(uint256)` and selector `0x94912b80`
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
    #[ethcall(name = "setSpreads", abi = "setSpreads(uint256)")]
    pub struct SetSpreadsCall {
        pub spreads: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setWithdrawPool` function with signature `setWithdrawPool(address)` and selector `0xc227db96`
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
    #[ethcall(name = "setWithdrawPool", abi = "setWithdrawPool(address)")]
    pub struct SetWithdrawPoolCall {
        pub withdraw_pool: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `spotUpdateRisk` function with signature `spotUpdateRisk(uint32,(int32,int32,int32,int32,int128))` and selector `0x8ab3daae`
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
        name = "spotUpdateRisk",
        abi = "spotUpdateRisk(uint32,(int32,int32,int32,int32,int128))"
    )]
    pub struct SpotUpdateRiskCall {
        pub product_id: u32,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitPerpAddOrUpdateProductCall` function with signature `submitPerpAddOrUpdateProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))` and selector `0x6c555b1b`
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
        name = "submitPerpAddOrUpdateProductCall",
        abi = "submitPerpAddOrUpdateProductCall(uint32,int128,int128,(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitPerpAddOrUpdateProductCallCall {
        pub product_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub risk_store: RiskStore,
    }
    ///Container type for all input parameters for the `submitSpotAddOrUpdateProductCall` function with signature `submitSpotAddOrUpdateProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))` and selector `0xd9fb99c1`
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
        name = "submitSpotAddOrUpdateProductCall",
        abi = "submitSpotAddOrUpdateProductCall(uint32,uint32,int128,int128,(address,int128,int128,int128,int128,int128,int128),(int32,int32,int32,int32,int128))"
    )]
    pub struct SubmitSpotAddOrUpdateProductCallCall {
        pub product_id: u32,
        pub quote_id: u32,
        pub size_increment: i128,
        pub min_size: i128,
        pub config: Config,
        pub risk_store: RiskStore,
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
    ///Container type for all input parameters for the `updateNlpPool` function with signature `updateNlpPool(uint64,address,uint128)` and selector `0x146fe5b5`
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
    #[ethcall(name = "updateNlpPool", abi = "updateNlpPool(uint64,address,uint128)")]
    pub struct UpdateNlpPoolCall {
        pub pool_id: u64,
        pub owner: ::ethers::core::types::Address,
        pub balance_weight_x18: u128,
    }
    ///Container type for all input parameters for the `updateTierFeeRates` function with signature `updateTierFeeRates(uint32[],uint32[],int128[],int128[])` and selector `0x9b6aba8f`
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
        name = "updateTierFeeRates",
        abi = "updateTierFeeRates(uint32[],uint32[],int128[],int128[])"
    )]
    pub struct UpdateTierFeeRatesCall {
        pub tier: ::std::vec::Vec<u32>,
        pub product_id: ::std::vec::Vec<u32>,
        pub maker_rate_x18: ::std::vec::Vec<i128>,
        pub taker_rate_x18: ::std::vec::Vec<i128>,
    }
    ///Container type for all input parameters for the `withdrawInsurance` function with signature `withdrawInsurance(uint128,address)` and selector `0x6c6c654e`
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
    #[ethcall(name = "withdrawInsurance", abi = "withdrawInsurance(uint128,address)")]
    pub struct WithdrawInsuranceCall {
        pub amount: u128,
        pub send_to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ContractOwnerCalls {
        AddEngine(AddEngineCall),
        AddNlpPool(AddNlpPoolCall),
        AddOrUpdateProducts(AddOrUpdateProductsCall),
        AssignPubKey(AssignPubKeyCall),
        ClearPerpAddOrUpdateProductCalls(ClearPerpAddOrUpdateProductCallsCall),
        ClearSpotAddOrUpdateProductCalls(ClearSpotAddOrUpdateProductCallsCall),
        CreateDirectDepositV1(CreateDirectDepositV1Call),
        CreditDepositV1(CreditDepositV1Call),
        DeleteNlpPool(DeleteNlpPoolCall),
        DeletePubkey(DeletePubkeyCall),
        DelistProduct(DelistProductCall),
        DepositInsurance(DepositInsuranceCall),
        DirectDepositV1Address(DirectDepositV1AddressCall),
        DumpFees(DumpFeesCall),
        GetDirectDepositV1BytecodeHash(GetDirectDepositV1BytecodeHashCall),
        HasPendingAddOrUpdateProductCalls(HasPendingAddOrUpdateProductCallsCall),
        Initialize(InitializeCall),
        IsDirectDepositV1Ready(IsDirectDepositV1ReadyCall),
        Owner(OwnerCall),
        PendingPerpAddOrUpdateProductIds(PendingPerpAddOrUpdateProductIdsCall),
        PendingSpotAddOrUpdateProductIds(PendingSpotAddOrUpdateProductIdsCall),
        PerpUpdateRisk(PerpUpdateRiskCall),
        RebalanceXWithdraw(RebalanceXWithdrawCall),
        RemoveWithdrawPoolLiquidity(RemoveWithdrawPoolLiquidityCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetSpreads(SetSpreadsCall),
        SetWithdrawPool(SetWithdrawPoolCall),
        SpotUpdateRisk(SpotUpdateRiskCall),
        SubmitPerpAddOrUpdateProductCall(SubmitPerpAddOrUpdateProductCallCall),
        SubmitSpotAddOrUpdateProductCall(SubmitSpotAddOrUpdateProductCallCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateNlpPool(UpdateNlpPoolCall),
        UpdateTierFeeRates(UpdateTierFeeRatesCall),
        WithdrawInsurance(WithdrawInsuranceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ContractOwnerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddEngineCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddEngine(decoded));
            }
            if let Ok(decoded) = <AddNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddNlpPool(decoded));
            }
            if let Ok(decoded) =
                <AddOrUpdateProductsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddOrUpdateProducts(decoded));
            }
            if let Ok(decoded) = <AssignPubKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AssignPubKey(decoded));
            }
            if let Ok(decoded) =
                <ClearPerpAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ClearPerpAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) =
                <ClearSpotAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ClearSpotAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) =
                <CreateDirectDepositV1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateDirectDepositV1(decoded));
            }
            if let Ok(decoded) =
                <CreditDepositV1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreditDepositV1(decoded));
            }
            if let Ok(decoded) = <DeleteNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeleteNlpPool(decoded));
            }
            if let Ok(decoded) = <DeletePubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeletePubkey(decoded));
            }
            if let Ok(decoded) = <DelistProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelistProduct(decoded));
            }
            if let Ok(decoded) =
                <DepositInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositInsurance(decoded));
            }
            if let Ok(decoded) =
                <DirectDepositV1AddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DirectDepositV1Address(decoded));
            }
            if let Ok(decoded) = <DumpFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DumpFees(decoded));
            }
            if let Ok(decoded) =
                <GetDirectDepositV1BytecodeHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDirectDepositV1BytecodeHash(decoded));
            }
            if let Ok(decoded) =
                <HasPendingAddOrUpdateProductCallsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::HasPendingAddOrUpdateProductCalls(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsDirectDepositV1ReadyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsDirectDepositV1Ready(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingPerpAddOrUpdateProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingPerpAddOrUpdateProductIds(decoded));
            }
            if let Ok(decoded) =
                <PendingSpotAddOrUpdateProductIdsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PendingSpotAddOrUpdateProductIds(decoded));
            }
            if let Ok(decoded) =
                <PerpUpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PerpUpdateRisk(decoded));
            }
            if let Ok(decoded) =
                <RebalanceXWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RebalanceXWithdraw(decoded));
            }
            if let Ok(decoded) =
                <RemoveWithdrawPoolLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveWithdrawPoolLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetSpreadsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSpreads(decoded));
            }
            if let Ok(decoded) =
                <SetWithdrawPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetWithdrawPool(decoded));
            }
            if let Ok(decoded) =
                <SpotUpdateRiskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SpotUpdateRisk(decoded));
            }
            if let Ok(decoded) =
                <SubmitPerpAddOrUpdateProductCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SubmitPerpAddOrUpdateProductCall(decoded));
            }
            if let Ok(decoded) =
                <SubmitSpotAddOrUpdateProductCallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SubmitSpotAddOrUpdateProductCall(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateNlpPoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateNlpPool(decoded));
            }
            if let Ok(decoded) =
                <UpdateTierFeeRatesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateTierFeeRates(decoded));
            }
            if let Ok(decoded) =
                <WithdrawInsuranceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawInsurance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ContractOwnerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddEngine(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddOrUpdateProducts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssignPubKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClearPerpAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClearSpotAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateDirectDepositV1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreditDepositV1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeletePubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelistProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositInsurance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DirectDepositV1Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DumpFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDirectDepositV1BytecodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasPendingAddOrUpdateProductCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDirectDepositV1Ready(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingPerpAddOrUpdateProductIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingSpotAddOrUpdateProductIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PerpUpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RebalanceXWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveWithdrawPoolLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetSpreads(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetWithdrawPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SpotUpdateRisk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitPerpAddOrUpdateProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitSpotAddOrUpdateProductCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateNlpPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateTierFeeRates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawInsurance(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ContractOwnerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddEngine(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddOrUpdateProducts(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssignPubKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearPerpAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClearSpotAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateDirectDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditDepositV1(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeletePubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelistProduct(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositInsurance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DirectDepositV1Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::DumpFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDirectDepositV1BytecodeHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasPendingAddOrUpdateProductCalls(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDirectDepositV1Ready(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingPerpAddOrUpdateProductIds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingSpotAddOrUpdateProductIds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PerpUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceXWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveWithdrawPoolLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSpreads(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotUpdateRisk(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitPerpAddOrUpdateProductCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SubmitSpotAddOrUpdateProductCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateNlpPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateTierFeeRates(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawInsurance(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddEngineCall> for ContractOwnerCalls {
        fn from(value: AddEngineCall) -> Self {
            Self::AddEngine(value)
        }
    }
    impl ::core::convert::From<AddNlpPoolCall> for ContractOwnerCalls {
        fn from(value: AddNlpPoolCall) -> Self {
            Self::AddNlpPool(value)
        }
    }
    impl ::core::convert::From<AddOrUpdateProductsCall> for ContractOwnerCalls {
        fn from(value: AddOrUpdateProductsCall) -> Self {
            Self::AddOrUpdateProducts(value)
        }
    }
    impl ::core::convert::From<AssignPubKeyCall> for ContractOwnerCalls {
        fn from(value: AssignPubKeyCall) -> Self {
            Self::AssignPubKey(value)
        }
    }
    impl ::core::convert::From<ClearPerpAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearPerpAddOrUpdateProductCallsCall) -> Self {
            Self::ClearPerpAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<ClearSpotAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: ClearSpotAddOrUpdateProductCallsCall) -> Self {
            Self::ClearSpotAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<CreateDirectDepositV1Call> for ContractOwnerCalls {
        fn from(value: CreateDirectDepositV1Call) -> Self {
            Self::CreateDirectDepositV1(value)
        }
    }
    impl ::core::convert::From<CreditDepositV1Call> for ContractOwnerCalls {
        fn from(value: CreditDepositV1Call) -> Self {
            Self::CreditDepositV1(value)
        }
    }
    impl ::core::convert::From<DeleteNlpPoolCall> for ContractOwnerCalls {
        fn from(value: DeleteNlpPoolCall) -> Self {
            Self::DeleteNlpPool(value)
        }
    }
    impl ::core::convert::From<DeletePubkeyCall> for ContractOwnerCalls {
        fn from(value: DeletePubkeyCall) -> Self {
            Self::DeletePubkey(value)
        }
    }
    impl ::core::convert::From<DelistProductCall> for ContractOwnerCalls {
        fn from(value: DelistProductCall) -> Self {
            Self::DelistProduct(value)
        }
    }
    impl ::core::convert::From<DepositInsuranceCall> for ContractOwnerCalls {
        fn from(value: DepositInsuranceCall) -> Self {
            Self::DepositInsurance(value)
        }
    }
    impl ::core::convert::From<DirectDepositV1AddressCall> for ContractOwnerCalls {
        fn from(value: DirectDepositV1AddressCall) -> Self {
            Self::DirectDepositV1Address(value)
        }
    }
    impl ::core::convert::From<DumpFeesCall> for ContractOwnerCalls {
        fn from(value: DumpFeesCall) -> Self {
            Self::DumpFees(value)
        }
    }
    impl ::core::convert::From<GetDirectDepositV1BytecodeHashCall> for ContractOwnerCalls {
        fn from(value: GetDirectDepositV1BytecodeHashCall) -> Self {
            Self::GetDirectDepositV1BytecodeHash(value)
        }
    }
    impl ::core::convert::From<HasPendingAddOrUpdateProductCallsCall> for ContractOwnerCalls {
        fn from(value: HasPendingAddOrUpdateProductCallsCall) -> Self {
            Self::HasPendingAddOrUpdateProductCalls(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for ContractOwnerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsDirectDepositV1ReadyCall> for ContractOwnerCalls {
        fn from(value: IsDirectDepositV1ReadyCall) -> Self {
            Self::IsDirectDepositV1Ready(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ContractOwnerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingPerpAddOrUpdateProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingPerpAddOrUpdateProductIdsCall) -> Self {
            Self::PendingPerpAddOrUpdateProductIds(value)
        }
    }
    impl ::core::convert::From<PendingSpotAddOrUpdateProductIdsCall> for ContractOwnerCalls {
        fn from(value: PendingSpotAddOrUpdateProductIdsCall) -> Self {
            Self::PendingSpotAddOrUpdateProductIds(value)
        }
    }
    impl ::core::convert::From<PerpUpdateRiskCall> for ContractOwnerCalls {
        fn from(value: PerpUpdateRiskCall) -> Self {
            Self::PerpUpdateRisk(value)
        }
    }
    impl ::core::convert::From<RebalanceXWithdrawCall> for ContractOwnerCalls {
        fn from(value: RebalanceXWithdrawCall) -> Self {
            Self::RebalanceXWithdraw(value)
        }
    }
    impl ::core::convert::From<RemoveWithdrawPoolLiquidityCall> for ContractOwnerCalls {
        fn from(value: RemoveWithdrawPoolLiquidityCall) -> Self {
            Self::RemoveWithdrawPoolLiquidity(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for ContractOwnerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetSpreadsCall> for ContractOwnerCalls {
        fn from(value: SetSpreadsCall) -> Self {
            Self::SetSpreads(value)
        }
    }
    impl ::core::convert::From<SetWithdrawPoolCall> for ContractOwnerCalls {
        fn from(value: SetWithdrawPoolCall) -> Self {
            Self::SetWithdrawPool(value)
        }
    }
    impl ::core::convert::From<SpotUpdateRiskCall> for ContractOwnerCalls {
        fn from(value: SpotUpdateRiskCall) -> Self {
            Self::SpotUpdateRisk(value)
        }
    }
    impl ::core::convert::From<SubmitPerpAddOrUpdateProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitPerpAddOrUpdateProductCallCall) -> Self {
            Self::SubmitPerpAddOrUpdateProductCall(value)
        }
    }
    impl ::core::convert::From<SubmitSpotAddOrUpdateProductCallCall> for ContractOwnerCalls {
        fn from(value: SubmitSpotAddOrUpdateProductCallCall) -> Self {
            Self::SubmitSpotAddOrUpdateProductCall(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ContractOwnerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateNlpPoolCall> for ContractOwnerCalls {
        fn from(value: UpdateNlpPoolCall) -> Self {
            Self::UpdateNlpPool(value)
        }
    }
    impl ::core::convert::From<UpdateTierFeeRatesCall> for ContractOwnerCalls {
        fn from(value: UpdateTierFeeRatesCall) -> Self {
            Self::UpdateTierFeeRates(value)
        }
    }
    impl ::core::convert::From<WithdrawInsuranceCall> for ContractOwnerCalls {
        fn from(value: WithdrawInsuranceCall) -> Self {
            Self::WithdrawInsurance(value)
        }
    }
    ///Container type for all return fields from the `createDirectDepositV1` function with signature `createDirectDepositV1(bytes32)` and selector `0x6e13cbf3`
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
    pub struct CreateDirectDepositV1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `directDepositV1Address` function with signature `directDepositV1Address(bytes32)` and selector `0x12efa71b`
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
    pub struct DirectDepositV1AddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDirectDepositV1BytecodeHash` function with signature `getDirectDepositV1BytecodeHash()` and selector `0x2fb0523a`
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
    pub struct GetDirectDepositV1BytecodeHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasPendingAddOrUpdateProductCalls` function with signature `hasPendingAddOrUpdateProductCalls()` and selector `0x145ca380`
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
    pub struct HasPendingAddOrUpdateProductCallsReturn(pub bool);
    ///Container type for all return fields from the `isDirectDepositV1Ready` function with signature `isDirectDepositV1Ready(address,bool)` and selector `0xa7e69da3`
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
    pub struct IsDirectDepositV1ReadyReturn(pub bool);
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
    ///Container type for all return fields from the `pendingPerpAddOrUpdateProductIds` function with signature `pendingPerpAddOrUpdateProductIds()` and selector `0xbe05a69d`
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
    pub struct PendingPerpAddOrUpdateProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `pendingSpotAddOrUpdateProductIds` function with signature `pendingSpotAddOrUpdateProductIds()` and selector `0xc9c5efaa`
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
    pub struct PendingSpotAddOrUpdateProductIdsReturn(pub ::std::vec::Vec<u32>);
    ///`Config(address,int128,int128,int128,int128,int128,int128)`
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
    pub struct Config {
        pub token: ::ethers::core::types::Address,
        pub interest_inflection_util_x18: i128,
        pub interest_floor_x18: i128,
        pub interest_small_cap_x18: i128,
        pub interest_large_cap_x18: i128,
        pub withdraw_fee_x18: i128,
        pub min_deposit_rate_x18: i128,
    }
    ///`RiskStore(int32,int32,int32,int32,int128)`
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
    pub struct RiskStore {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub price_x18: i128,
    }
}
