pragma solidity ^0.8.9;

contract Example {
    string public name = "foo";

    function setName(string memory _name) public {
       name = _name;
    }
}
