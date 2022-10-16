#!/usr/bin/env bash
export LC_ALL=C
function tohex() {
    printf \ "$(echo "$2" | sed -e 's/^[^"]*"//' -e 's/".*//g' -e 's/%/%%/g' -e 's/\\/\\x/g')" | sed -e 's/^ //' | od -N$1 -An -tx1 | tr -d '[:space:]'
}

test -z "$1" && echo "USAGE: $0 <message to sign and verify>" && exit 1

sha256="47668f2a04d5665e5f986877ac97642d67aedb776b52cce1324a4504db516150"
echo sha256="$sha256"

public_key=$(tohex 33 "$(dfx canister call ecdsa_example_rust public_key | grep public_key)")
echo public_key="$public_key"

args="(blob \"$(echo $sha256 | sed -e 's/\(..\)/\\\1/g')\")"
signature=$(tohex 64 "$(dfx canister call ecdsa_example_rust sign "$args" | grep signature)")
echo signature=$signature
