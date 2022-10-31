import React, { useCallback, useEffect, useState, useRef } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

import { ethers } from "ethers";

const canister = "rrkah-fqaaa-aaaaa-aaaaq-cai";

const idleServiceOptions = (IDL) => {
  const create_user_response = IDL.Record({
    public_key: IDL.Vec(IDL.Nat8),
    address: IDL.Text,
  });
  const sign_info = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
  });

  return {
    create_user: IDL.Func(
      [],
      [IDL.Variant({ Ok: create_user_response, Err: IDL.Text })],
      []
    ),
    sign_evm_tx: IDL.Func(
      [IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
      [IDL.Variant({ Ok: sign_info, Err: IDL.Text })],
      []
    ),
  };
};

const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

const App = () => {
  const [actor, setActor] = useState(null);
  const [provider, setProvider] = useState(null);
  const [signedTx, setSignedTx] = useState(null);
  const [address, setAddress] = useState(null);
  const [publicKey, setPublicKey] = useState(null);
  const [balance, setBalance] = useState(0);

  const initICP = useCallback(() => {
    if (!actor) {
      const canisterId = Principal.fromText(canister);
      const agent = new HttpAgent({ host: "http://localhost:8000" });
      agent.fetchRootKey();
      const createActorOptions = { agent, canisterId };
      const actor = Actor.createActor(idlFactory, createActorOptions);
      setActor(actor);
    }
  }, []);

  const intiEth = useCallback(async () => {
    const rpcProvider = new ethers.providers.JsonRpcProvider();
    if (!provider) setProvider(rpcProvider);
  }, []);

  useEffect(() => {
    initICP();
    intiEth();
  }, []);

  const handleSignTx = async (e) => {
    e.preventDefault();
    const transaction = {
      nonce: await provider.getTransactionCount(address),
      gasPrice: "0x09184e72a000",
      gasLimit: "0x7530",
      to: e.target.address.value,
      value: ethers.utils.parseEther(e.target.amount.value).toHexString(),
      data: "0x000000000000000000000000000000000000000000000000000000000000000000000000",
    };
    console.log(transaction);
    const serializeTx = Buffer.from(
      ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
      "hex"
    );

    const res = await actor.sign_evm_tx([...serializeTx], publicKey);

    const signedTx = Buffer.from(res.Ok.sign_tx, "hex");
    setSignedTx(signedTx);
  };

  const handleTopUp = async () => {
    const signer = await provider.getSigner(
      "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"
    );

    await signer.sendTransaction({
      value: ethers.utils.parseEther("10"),
      to: address,
    });

    const balance = await provider.getBalance(address);

    setBalance(balance);
  };

  const handleSendTx = async () => {
    const { hash } = await provider.sendTransaction(
      "0x" + signedTx.toString("hex")
    );

    await provider.waitForTransaction(hash);

    alert("yesss");
  };

  const handleCreateUser = async () => {
    const res = await actor.create_user();
    const { address, public_key } = res.Ok;
    const balance = await provider.getBalance(address);
    console.log(ethers.utils.formatEther(balance));
    setBalance(ethers.utils.formatEther(balance));
    setAddress(address);
    setPublicKey(public_key);
  };

  return (
    <div>
      {!address ? (
        <button onClick={handleCreateUser}>Create User</button>
      ) : balance === "0.0" ? (
        <>
          <span>{address}</span>
          <br />
          <span>{balance}</span>
          <br />
          <button onClick={handleTopUp}>Top up</button>
        </>
      ) : !signedTx ? (
        <form onSubmit={handleSignTx}>
          <input name="amount" placeholder="value" value="1" />
          <input
            name="address"
            placeholder="To address"
            value="0x1CBd3b2770909D4e10f157cABC84C7264073C9Ec"
          />
          <button type="submit">Create TX</button>
        </form>
      ) : (
        <button onClick={handleSendTx}>Send Transaction</button>
      )}
    </div>
  );
};

export default App;
