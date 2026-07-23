use alloy::sol;

sol! {
    #[sol(rpc)]
    #[allow(missing_docs)]
    contract MockERC20 {
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
        function decimals() external view returns (uint8);
        function decreaseAllowance(address spender, uint256 subtractedValue) external returns (bool);
        function increaseAllowance(address spender, uint256 addedValue) external returns (bool);
        function mint(address account, uint256 amount) external;
        function name() external view returns (string);
        function symbol() external view returns (string);
        function totalSupply() external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
}
