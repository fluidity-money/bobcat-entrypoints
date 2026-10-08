#![no_std]

use bobcat_cd::{U, EvmCdAddress, EvmCdBytes, EvmCdSerialise, EvmCdDeserialise};

#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip20 {
    TotalSupply,
    BalanceOf {
        owner: EvmCdAddress,
    },
    Transfer {
        to: EvmCdAddress,
        value: U,
    },
    Allowance {
        owner: EvmCdAddress,
        spender: EvmCdAddress,
    },
    Approve {
        spender: EvmCdAddress,
        value: U,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        value: U,
    },
    Name,
    Symbol,
    Decimals,
}

#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip20Permit {
    TotalSupply,
    BalanceOf {
        owner: EvmCdAddress,
    },
    Transfer {
        to: EvmCdAddress,
        value: U,
    },
    Allowance {
        owner: EvmCdAddress,
        spender: EvmCdAddress,
    },
    Approve {
        spender: EvmCdAddress,
        value: U,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        value: U,
    },
    Name,
    Symbol,
    Decimals,

    Permit {
        owner: EvmCdAddress,
        spender: EvmCdAddress,
        value: U,
        deadline: U,
        v: u8,
        r: [u8; 32],
        s: [u8; 32],
    },
    Nonces {
        owner: EvmCdAddress,
    },
    DomainSeparator,
}

#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip721DataSlice<const MAX_DATA: usize> {
    BalanceOf {
        owner: EvmCdAddress,
    },
    OwnerOf {
        token_id: U,
    },
    SafeTransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    #[evm_selector("safeTransferFrom(address,address,uint256,bytes)")]
    SafeTransferFromWithData {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
        data: EvmCdBytes<MAX_DATA>,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    Approve {
        approved: EvmCdAddress,
        token_id: U,
    },
    SetApprovalForAll {
        operator: EvmCdAddress,
        approved: bool,
    },
    GetApproved {
        token_id: U,
    },
    IsApprovedForAll {
        owner: EvmCdAddress,
        operator: EvmCdAddress,
    },
}

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip721DataVec {
    BalanceOf {
        owner: EvmCdAddress,
    },
    OwnerOf {
        token_id: U,
    },
    SafeTransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    #[evm_selector("safeTransferFrom(address,address,uint256,bytes)")]
    SafeTransferFromWithData {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
        data: Vec<u8>,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    Approve {
        approved: EvmCdAddress,
        token_id: U,
    },
    SetApprovalForAll {
        operator: EvmCdAddress,
        approved: bool,
    },
    GetApproved {
        token_id: U,
    },
    IsApprovedForAll {
        owner: EvmCdAddress,
        operator: EvmCdAddress,
    },
}

#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip721MetadataDataSlice<const MAX_DATA: usize> {
    BalanceOf {
        owner: EvmCdAddress,
    },
    OwnerOf {
        token_id: U,
    },
    SafeTransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    #[evm_selector("safeTransferFrom(address,address,uint256,bytes)")]
    SafeTransferFromWithData {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
        data: EvmCdBytes<MAX_DATA>,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    Approve {
        approved: EvmCdAddress,
        token_id: U,
    },
    SetApprovalForAll {
        operator: EvmCdAddress,
        approved: bool,
    },
    GetApproved {
        token_id: U,
    },
    IsApprovedForAll {
        owner: EvmCdAddress,
        operator: EvmCdAddress,
    },
    Name,
    Symbol,
    #[evm_selector("tokenURI(uint256)")]
    TokenUri { token_uri: U }
}

#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, EvmCdSerialise, EvmCdDeserialise)]
#[evm_selector]
pub enum Eip721MetadataDataVec<const MAX_DATA: usize> {
    BalanceOf {
        owner: EvmCdAddress,
    },
    OwnerOf {
        token_id: U,
    },
    SafeTransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    #[evm_selector("safeTransferFrom(address,address,uint256,bytes)")]
    SafeTransferFromWithData {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
        data: Vec<u8>,
    },
    TransferFrom {
        from: EvmCdAddress,
        to: EvmCdAddress,
        token_id: U,
    },
    Approve {
        approved: EvmCdAddress,
        token_id: U,
    },
    SetApprovalForAll {
        operator: EvmCdAddress,
        approved: bool,
    },
    GetApproved {
        token_id: U,
    },
    IsApprovedForAll {
        owner: EvmCdAddress,
        operator: EvmCdAddress,
    },
    Name,
    Symbol,
    #[evm_selector("tokenURI(uint256)")]
    TokenUri { token_uri: U }
}
