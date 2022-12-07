pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ExampleToken is ERC20 {
    constructor() ERC20("ExampleToken", "MTK") {
        _mint(msg.sender, 100_000 * 10**18); // 100,000 tokens
    }
}