import React, { useCallback, useEffect, useState, useRef } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

import { ethers } from "ethers";

const canister = "rrkah-fqaaa-aaaaa-aaaaq-cai";

const idleServiceOptions = (IDL) => {
  const public_key_info = IDL.Record({
    public_key: IDL.Vec(IDL.Nat8),
  });
  const sign_info = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
    signature: IDL.Vec(IDL.Nat8),
    msg_hash: IDL.Vec(IDL.Nat8),
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

const App = () => {
  const [actor, setActor] = useState(null);
  const [provider, setProvider] = useState(null);
  const [signedTx, setSignedTx] = useState(null);
  const [senderAddress, setSenderAddress] = useState(null);
  const [balance, setBalance] = useState(null);

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
      nonce: 0,
      gasPrice: "0x09184e72a000",
      gasLimit: "0x7530",
      to: e.target.address.value,
      value: ethers.utils.parseEther(e.target.amount.value).toHexString(),
      data: "0x7f7465737432000000000000000000000000000000000000000000000000000000600057",
    };
    const serializeTx = Buffer.from(
      ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
      "hex"
    );

    const res = await actor.sign_evm_tx([...serializeTx]);

    const signedTx = Buffer.from(res.Ok.sign_tx, "hex");
    setSignedTx(signedTx);
  };

  const handleTopUp = async () => {
    const signer = await provider.getSigner(
      "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"
    );

    const from = ethers.utils.parseTransaction(signedTx).from;

    await signer.sendTransaction({
      value: ethers.utils.parseEther("2"),
      to: from,
    });

    setSenderAddress(from);

    const balance = await provider.getBalance(from);

    setBalance(balance);
  };

  const handleSendTx = async () => {
    const { hash } = await provider.sendTransaction(
      "0x" + signedTx.toString("hex")
    );

    await provider.waitForTransaction(hash);

    alert("yesss");
  };

  return (
    <div>
      {!signedTx ? (
        <form onSubmit={handleSignTx}>
          <input name="amount" placeholder="value" value="1" />
          <input
            name="address"
            placeholder="To address"
            value="0x1CBd3b2770909D4e10f157cABC84C7264073C9Ec"
          />
          <button type="submit">Create TX</button>
        </form>
      ) : !senderAddress ? (
        <button onClick={handleTopUp}>Top up</button>
      ) : (
        <button onClick={handleSendTx}>Send TX</button>
      )}
    </div>
  );
};

export default App;
