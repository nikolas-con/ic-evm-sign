// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Example {
    string public name = "foo";

    constructor()  {}

    function setName(string memory _name) public {
       name = _name;
    }
}
