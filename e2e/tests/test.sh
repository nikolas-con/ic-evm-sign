#!/usr/bin/env bash

dfx start --clean --background

cd ../../ && dfx deploy

cd tests/e2e && npm run test

dfx stop