use ethers::contract::abigen;
// use ethers_contract::abigen;

abigen!(
    TestContract,
    r#"[
    function foo() external view returns (uint256)
    ]"#r
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
