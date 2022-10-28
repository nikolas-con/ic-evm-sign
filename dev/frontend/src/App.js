import React, { useCallback, useEffect, useState } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

import { ethers } from "ethers";

/* global BigInt */

const days = BigInt(1);
const hours = BigInt(24);
const nanoseconds = BigInt(3600000000000);

const canister = "rrkah-fqaaa-aaaaa-aaaaq-cai";

const idleServiceOptions = (IDL) => {
  const public_key_info = IDL.Record({
    public_key: IDL.Vec(IDL.Nat8),
  });
  const sign_info = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
  });

  return {
    get_public_key: IDL.Func(
      [],
      [IDL.Variant({ Ok: public_key_info, Err: IDL.Text })],
      []
    ),
    sign_evm_tx: IDL.Func(
      [IDL.Vec(IDL.Nat8)],
      [IDL.Variant({ Ok: sign_info, Err: IDL.Text })],
      []
    ),
  };
};

const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

const convertToHex = (buffer) => {
  return [...new Uint8Array(buffer)].map((x) => x.toString(16)).join("");
};

const App = () => {
  const [actor, setActor] = useState(null);

  const initICP = useCallback(() => {
    const canisterId = Principal.fromText(canister);
    const agent = new HttpAgent({ host: "http://localhost:8000" });
    agent.fetchRootKey();
    const createActorOptions = { agent, canisterId };
    const actor = Actor.createActor(idlFactory, createActorOptions);
    setActor(actor);
  }, []);

  const intiEth = useCallback(async () => {
    let provider = new ethers.providers.JsonRpcProvider();
    const text = await provider.getBalance(
      "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"
    );
    console.log(text);
  }, []);
  useEffect(() => {
    initICP();
    intiEth();
  }, []);

  const handle = async () => {
    const res = await actor.get_public_key();
    const publicKey = res.Ok.public_key;
    console.log("0x" + Buffer.from([...publicKey], "hex").toString("hex"));
  };

  return (
    <div>
      <button onClick={handle}>test</button>
    </div>
  );
};

export default App;
