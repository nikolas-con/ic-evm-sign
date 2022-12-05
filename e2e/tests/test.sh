#!/usr/bin/env bash

dfx start --clean --background && clear

cd ../../ && dfx deploy && clear

cd e2e/tests && npm run test

dfx stop