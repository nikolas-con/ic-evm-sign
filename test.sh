#!/usr/bin/env bash

dfx start --clean --background

dfx deploy

cd tests/e2e && npx hardhat test ./sign.js

dfx stop