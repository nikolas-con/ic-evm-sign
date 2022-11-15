import React, { useCallback, useEffect, useState } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";

import { ethers } from "ethers";

/* global BigInt */

const days = BigInt(1);
const hours = BigInt(24);
const nanoseconds = BigInt(3600000000000);

const BACKEND_CANISTER_ID = "rrkah-fqaaa-aaaaa-aaaaq-cai";
const IDENTITY_CANISTER_ID = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const TO_ADDRESS = "0x7b2a3598d63256D0CE33a64ed88515dD6e76Eb2A";
const FAUCET_ON_LOCAL_NODE = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
const RPC_URL = process.env.REACT_APP_RPC_URL
  ? process.env.REACT_APP_RPC_URL
  : "http://127.0.0.1:8545/";
const idleServiceOptions = (IDL) => {
  const transactions = IDL.Record({
    data: IDL.Vec(IDL.Nat8),
    timestamp: IDL.Nat64,
  });
  const create_response = IDL.Record({
    address: IDL.Text,
  });
  const sign_tx_response = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
  });

  const caller_response = IDL.Record({
    address: IDL.Text,
    transactions: IDL.Vec(transactions),
  });

  return {
    create: IDL.Func(
      [],
      [IDL.Variant({ Ok: create_response, Err: IDL.Text })],
      ["update"]
    ),
    sign_evm_tx: IDL.Func(
      [IDL.Vec(IDL.Nat8), IDL.Nat64],
      [IDL.Variant({ Ok: sign_tx_response, Err: IDL.Text })],
      ["update"]
    ),
    clear_caller_history: IDL.Func([], [], ["update"]),
    get_caller_data: IDL.Func(
      [],
      [IDL.Variant({ Ok: caller_response, Err: IDL.Text })],
      ["query"]
    ),
  };
};

const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

const App = () => {
  const [actor, setActor] = useState(null);
  const [authClient, setAuthClient] = useState(null);
  const [provider, setProvider] = useState(null);
  const [address, setAddress] = useState(null);
  const [balance, setBalance] = useState(null);
  const [stage, setStage] = useState("");
  const [loggedIn, setLoggedIn] = useState(false);
  const [transactions, setTransactions] = useState([]);

  const onLogin = () => {
    setLoggedIn(true);
    console.log("success");
  };
  const onLogout = (msg) => {
    setLoggedIn(false);
    console.log("logout", msg);
  };

  const logout = useCallback(async () => {
    await authClient.logout();
    onLogout("");
  }, [authClient]);

  const initICP = useCallback(() => {
    if (!actor) {
      const backendCanisterId = Principal.fromText(BACKEND_CANISTER_ID);
      const agent = new HttpAgent({ host: "http://localhost:8000" });
      agent.fetchRootKey();
      const createActorOptions = { agent, canisterId: backendCanisterId };
      const actor = Actor.createActor(idlFactory, createActorOptions);
      setActor(actor);
    }
  }, []);

  const initIdentity = useCallback(async () => {
    if (!authClient) {
      // timeout 30 seconds
      const onIdle = () => {
        logout("Inactivity");
      };
      const idleOptions = { idleTimeout: 30_000, disableIdle: false, onIdle };
      const _authClient = await AuthClient.create({ idleOptions });
      setAuthClient(_authClient);
    }
  }, []);

  const intiEth = useCallback(async () => {
    const rpcProvider = new ethers.providers.JsonRpcProvider(RPC_URL);
    if (!provider) setProvider(rpcProvider);
  }, []);

  useEffect(() => {
    initICP();
    initIdentity();
    intiEth();
  }, []);

  const login = async () => {
    // expires in 8 days
    const identityProvider = `http://localhost:8000?canisterId=${IDENTITY_CANISTER_ID}`;
    authClient.login({
      onSuccess: onLogin,
      identityProvider,
      maxTimeToLive: days * hours * nanoseconds,
    });

    try {
      const res = await actor.get_caller_data();
      const { address, transactions } = res.Ok;
      setAddress(address);
      setTransactions(transactions);
      const balance = await provider.getBalance(address);
      setBalance(ethers.utils.formatEther(balance));
    } catch (error) {
      console.log(error);
    }
  };

  const handleSignTx = async (e) => {
    e.preventDefault();
    const transaction = {
      nonce: await provider.getTransactionCount(address),
      gasPrice: await provider.getGasPrice().then((s) => s.toHexString()),
      gasLimit: "0x5dc0",
      to: e.target.address.value,
      value: ethers.utils.parseEther(e.target.amount.value).toHexString(),
      data: "0x000000000000000000000000000000000000000000000000000000000000000000000000",
    };

    const serializeTx = Buffer.from(
      ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
      "hex"
    );

    setStage("signing transaction...");

    const { chainId } = await provider.getNetwork();

    const res = await actor.sign_evm_tx([...serializeTx], Number(chainId));

    const signedTx = Buffer.from(res.Ok.sign_tx, "hex");

    setStage("send transaction...");

    const { hash } = await provider.sendTransaction(
      "0x" + signedTx.toString("hex")
    );

    setStage("wait for verification ...");
    await provider.waitForTransaction(hash);
    setStage("");

    const balance = await provider.getBalance(address);
    setBalance(ethers.utils.formatEther(balance));
    setTransactions((txs) => [...txs, { data: signedTx }]);
  };

  const handleTopUp = async () => {
    const signer = await provider.getSigner(FAUCET_ON_LOCAL_NODE);

    await signer.sendTransaction({
      value: ethers.utils.parseEther("10"),
      to: address,
    });

    const balance = await provider.getBalance(address);

    setBalance(ethers.utils.formatEther(balance));
  };

  const handleCreateEVMWallet = async () => {
    const res = await actor.create();
    const { address } = res.Ok;
    const balance = await provider.getBalance(address);
    setBalance(ethers.utils.formatEther(balance));
    setAddress(address);
  };
  const handleCleanTxHistory = async () => {
    await actor.clear_caller_history();
    setTransactions([]);
  };

  return (
    <div>
      {loggedIn ? (
        <>
          <div>
            <button onClick={logout}>log out</button>
            {address && <p>EVM Address: {address}</p>}
            {balance && <p>Balance: {balance}</p>}
          </div>

          {!address ? (
            <button onClick={handleCreateEVMWallet}>Create EVM Wallet</button>
          ) : balance === "0.0" ? (
            <div>
              <button onClick={handleTopUp}>Top up</button>
            </div>
          ) : (
            <>
              <form onSubmit={handleSignTx}>
                <input name="amount" placeholder="value" />
                <input name="address" placeholder="To address" />
                <button type="submit">Send ETH</button>
              </form>
              <div>
                <span>{stage}</span>
              </div>
              <div>
                <p>Transactions History</p>
                {transactions.length > 0 && (
                  <button onClick={handleCleanTxHistory}>
                    Clean Transaction History
                  </button>
                )}

                <ul>
                  {transactions.map((tx, index) => (
                    <li key={index}>
                      {ethers.utils.parseTransaction(tx.data).hash}
                    </li>
                  ))}
                </ul>
              </div>
            </>
          )}
        </>
      ) : (
        <div>
          <h2>Not authenticated</h2>
          <button type="button" onClick={login}>
            Log in
          </button>
        </div>
      )}
    </div>
  );
};

export default App;
