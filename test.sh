#!/usr/bin/env bash

dfx start --clean --background

dfx deploy

cd tests && npx hardhat test

dfx stop