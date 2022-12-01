import React, { useCallback, useEffect, useState } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";
import {
  Box,
  Button,
  Text,
  Heading,
  Input,
  UnorderedList,
  ListItem,
} from "@chakra-ui/react";

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
  const chainData = IDL.Record({
    nonce: IDL.Nat64,
    transactions: IDL.Vec(transactions),
  });
  const create_response = IDL.Record({
    address: IDL.Text,
  });
  const sign_tx_response = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
  });

  const caller_response = IDL.Record({
    address: IDL.Text,
    transactions: chainData,
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
      [IDL.Nat64],
      [IDL.Variant({ Ok: caller_response, Err: IDL.Text })],
      ["query"]
    ),
  };
};

const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

const App = () => {
  const [actor, setActor] = useState(null);
  const [chainId, setChainId] = useState(null);
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
    if (!provider) {
      setProvider(rpcProvider);
      const { chainId } = await rpcProvider.getNetwork();
      setChainId(chainId);
    }
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
      const res = await actor.get_caller_data(Number(chainId));
      const { address, transactions } = res.Ok;
      setAddress(address);
      setTransactions(transactions.transactions);
      const balance = await provider.getBalance(address);
      setBalance(ethers.utils.formatEther(balance));
    } catch (error) {
      console.log("ss");
      console.log(error);
    }
  };

  const handleSignTx = async (e) => {
    e.preventDefault();
    console.log(typeof e.target.address.value);
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
    <Box>
      {loggedIn ? (
        <>
          <Box>
            <Button onClick={logout}>log out</Button>
            {address && <Text>EVM Address: {address}</Text>}
            {balance && <Text>Balance: {balance}</Text>}
          </Box>

          {!address ? (
            <Button onClick={handleCreateEVMWallet}>Create EVM Wallet</Button>
          ) : balance === "0.0" ? (
            <Box>
              <Button onClick={handleTopUp}>Top up</Button>
            </Box>
          ) : (
            <>
              <form onSubmit={handleSignTx}>
                <Input name="address" placeholder="To address" />
                <Input name="amount" placeholder="value" />
                <Button type="submit">Send ETH</Button>
              </form>
              <Box>
                <Text>{stage}</Text>
              </Box>
              <Box>
                <Text>Transactions History</Text>
                {transactions.length > 0 && (
                  <Button onClick={handleCleanTxHistory}>
                    Clean Transaction History
                  </Button>
                )}

                <UnorderedList>
                  {transactions.map((tx, index) => (
                    <ListItem key={index}>
                      {ethers.utils.parseTransaction(tx.data).hash}
                    </ListItem>
                  ))}
                </UnorderedList>
              </Box>
            </>
          )}
        </>
      ) : (
        <Box>
          <Heading as="h1">Not authenticated</Heading>
          <Button colorScheme="blue" onClick={login}>
            Log in
          </Button>
        </Box>
      )}
    </Box>
  );
};

export default App;
