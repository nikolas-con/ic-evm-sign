import React, { useCallback, useEffect, useState } from "react";

import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { DelegationChain, Ed25519KeyIdentity, DelegationIdentity } from "@dfinity/identity";
import { Principal } from "@dfinity/principal";
import {
  Box,
  Flex,
  Button,
  Heading,
  Text,
  Input,
  Badge,
  UnorderedList,
  ListItem,
  Divider,
  useToast
} from "@chakra-ui/react";

import { ethers } from "ethers";

import {
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  ModalCloseButton,
  useDisclosure
} from '@chakra-ui/react'

import { HiClock, HiPlusCircle, HiArrowLeftCircle } from "react-icons/hi2";

const IcLogo = ({ width = 36, height = 16 }) => {
  return (
    <svg viewBox="0 0 233 111" width={width} height={height}>
      <defs>
        <linearGradient id="grad-o-y" x1="145.304" x2="221.385" y1="7.174" y2="85.958" gradientUnits="userSpaceOnUse">
          <stop offset=".21" stopColor="#F15A24"></stop>
          <stop offset=".684" stopColor="#FBB03B"></stop>
        </linearGradient>
        <linearGradient id="grad-p-p" x1="85.087" x2="9.006" y1="101.622" y2="22.838" gradientUnits="userSpaceOnUse">
          <stop offset=".21" stopColor="#ED1E79"></stop>
          <stop offset=".893" stopColor="#522785"></stop>
        </linearGradient>
      </defs>
      <g transform="translate(0 2)">
        <path fill="url(#grad-o-y)" d="M174.433 0c-12.879 0-26.919 6.6-41.758 19.6-7.04 6.159-13.12 12.759-17.679 18.038l.04.04v-.04s7.199 7.84 15.159 16.24c4.28-5.08 10.44-12 17.519-18.24 13.2-11.559 21.799-13.999 26.719-13.999 18.52 0 33.559 14.68 33.559 32.719 0 17.92-15.079 32.599-33.559 32.719-.84 0-1.92-.12-3.28-.4 5.4 2.32 11.2 4 16.72 4 33.918 0 40.558-22.12 40.998-23.72 1-4.04 1.52-8.28 1.52-12.64C230.391 24.4 205.272 0 174.433 0Z"></path>
        <path fill="url(#grad-p-p)" d="M55.958 108.796c12.88 0 26.919-6.6 41.758-19.6 7.04-6.16 13.12-12.759 17.679-18.039l-.04-.04v.04s-7.199-7.84-15.159-16.24c-4.28 5.08-10.44 12-17.52 18.24-13.199 11.56-21.798 14-26.718 14-18.52-.04-33.559-14.72-33.559-32.76C22.4 36.48 37.48 21.8 55.958 21.68c.84 0 1.92.12 3.28.4-5.4-2.32-11.2-4-16.72-4C8.6 18.08 2 40.2 1.52 41.76A52.8 52.8 0 0 0 0 54.397c0 29.999 25.119 54.398 55.958 54.398Z"></path>
        <path fill="#29ABE2" d="M187.793 90.197c-17.36-.44-35.399-14.12-39.079-17.52-9.519-8.8-31.479-32.599-33.198-34.479C99.436 20.16 77.637 0 55.958 0h-.08C29.558.12 7.44 17.96 1.52 41.758c.44-1.56 9.12-24.119 40.958-23.319 17.36.44 35.479 14.32 39.199 17.72 9.52 8.8 31.479 32.598 33.199 34.478 16.079 18 37.878 38.159 59.557 38.159h.08c26.319-.12 48.478-17.96 54.358-41.759-.48 1.56-9.2 23.92-41.078 23.16Z"></path>
      </g>
    </svg>
  )
}

const SendEthModal = ({ provider, setTransactions, setBalance, actor, chainId, address, onClose, isOpen }) => {
  const [amount, setAmount] = useState("");
  const [destination, setDestination] = useState("");
  const toast = useToast()

  const handleSignTx = async (e) => {
    e.preventDefault();

    onClose()

    const nonce = await provider.getTransactionCount(address)
    const gasPrice = await provider.getGasPrice().then((s) => s.toHexString())
    const value = ethers.utils.parseEther(amount).toHexString()
    const data = "0x000000000000000000000000000000000000000000000000000000000000000000000000"
    const gasLimit = "0x5dc0"
    const transaction = { nonce, gasPrice, gasLimit, to: destination, value, data, };

    const serializeTx = Buffer.from(
      ethers.utils.serializeTransaction(transaction).slice(2) + "808080",
      "hex"
    );

    toast({ title: "Signing transaction..." });

    const res = await actor.sign_evm_tx([...serializeTx], Number(chainId));

    const signedTx = Buffer.from(res.Ok.sign_tx, "hex");

    toast({ title: "Sending transaction..." });

    const { hash } = await provider.sendTransaction(
      "0x" + signedTx.toString("hex")
    );

    await provider.waitForTransaction(hash);
    toast({ title: `Transfered ${amount} ETH` });

    const balance = await provider.getBalance(address);
    setBalance(ethers.utils.formatEther(balance));
    setTransactions((txs) => [...txs, { data: signedTx }]);
  };

  return (
    <>
      <form>
        <Modal isOpen={isOpen} onClose={onClose} isCentered>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>Transfer Funds</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              <Flex>
                <Input onChange={e => setDestination(e.target.value)} placeholder="Destination (Address)" />
                <Input onChange={e => setAmount(e.target.value)} placeholder="Amount" type="number" ml="10px" width="120px" />
              </Flex>
            </ModalBody>
            <ModalFooter>
              <Button variant='ghost' mr={3} onClick={onClose}>Close</Button>
              <Button onClick={handleSignTx}>Send</Button>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </form>
    </>
  )
}

const TransactionsModal = ({ onClose, isOpen, actor, transactions, setTransactions, chainId }) => {

  const handleCleanTxHistory = async () => {
    await actor.clear_caller_history(Number(chainId));
    setTransactions([]);
  };

  return (
    <>
      <form>
        <Modal isOpen={isOpen} onClose={onClose} isCentered>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>Transaction History</ModalHeader>
            <ModalCloseButton />
            <ModalBody>
              {transactions.length > 0 ?
                <UnorderedList>
                  {transactions.map((tx, index) => (
                    <ListItem key={index}>
                      {ethers.utils.parseTransaction(tx.data).hash}
                    </ListItem>
                  ))}
                </UnorderedList> : 'No transactions yet'}
            </ModalBody>
            <ModalFooter>
              <Button variant='ghost' mr={'auto'} onClick={handleCleanTxHistory} disabled={transactions.length === 0}>Clean History</Button>
              <Button type="submit" onClick={onClose}>Close</Button>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </form>
    </>
  )
}

/* global BigInt */

const days = BigInt(1);
const hours = BigInt(24);
const nanoseconds = BigInt(3600000000000);

const BACKEND_CANISTER_ID = "rrkah-fqaaa-aaaaa-aaaaq-cai";
const IDENTITY_CANISTER_ID = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const FAUCET_ON_LOCAL_NODE = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
const RPC_URL = process.env.REACT_APP_RPC_URL ?? "http://127.0.0.1:8545/";

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
    clear_caller_history: IDL.Func(
      [IDL.Nat64],
      [IDL.Variant({ Ok: IDL.Null, Err: IDL.Text })],
      ["update"]
    ),
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
  const [loggedIn, setLoggedIn] = useState(false);
  const [transactions, setTransactions] = useState([]);
  const { isOpen: isSendOpen, onOpen: onSendOpen, onClose: onSendClose } = useDisclosure()
  const { isOpen: isHistoryOpen, onOpen: onHistoryOpen, onClose: onHistoryClose } = useDisclosure()

  const loadUser = useCallback(async (_actor) => {
    try {
      const res = await (_actor ?? actor).get_caller_data(Number(chainId));
      const { address, transactions } = res.Ok;
      setAddress(address);
      setTransactions(transactions.transactions);
      const balance = await provider.getBalance(address);
      setBalance(ethers.utils.formatEther(balance));
    } catch (error) {
      console.log(error);
    }
  }, [provider, actor, chainId])

  const onLogin = async () => {
    setLoggedIn(true);

    const identity = authClient.getIdentity()
    localStorage.setItem("identity", JSON.stringify(identity));
    localStorage.setItem("key", JSON.stringify(authClient._key));

    await loadUser()
  };

  const onLogout = () => {
    setLoggedIn(false);

    localStorage.removeItem("identity");
    localStorage.removeItem("key");
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
      const _actor = Actor.createActor(idlFactory, createActorOptions);
      setActor(_actor);
      return _actor
    }
  }, [actor])

  const initIdentity = useCallback(async () => {

    if (!authClient) {
      const identity = localStorage.getItem("identity");
      const key = localStorage.getItem("key");

      if (identity && key) {

        const _identity = JSON.parse(identity);
        const chain = DelegationChain.fromDelegations(_identity._delegation.delegations, _identity._delegation.publicKey)

        const _key = JSON.parse(key);
        const keyIdenity = Ed25519KeyIdentity.fromParsedJson(_key)

        const delegationIdentity = DelegationIdentity.fromDelegation(keyIdenity, chain)

        const _authClient = await AuthClient.create({ identity: delegationIdentity });
        setAuthClient(_authClient);

        setLoggedIn(true);
        const _actor = initICP();

        await loadUser(_actor)
      } else {
        const _authClient = await AuthClient.create({});
        setAuthClient(_authClient);
      }
    }
  }, [authClient, initICP, loadUser]);

  const intEth = useCallback(async () => {
    const rpcProvider = new ethers.providers.JsonRpcProvider(RPC_URL);
    if (!provider) {
      setProvider(rpcProvider);
      const { chainId } = await rpcProvider.getNetwork();
      setChainId(chainId);
    }
  }, [provider]);

  useEffect(() => {
    intEth();
  }, [intEth]);

  useEffect(() => {
    if (provider) initIdentity();
  }, [provider, initIdentity]);

  const login = async () => {
    // expires in 8 days
    const identityProvider = `http://localhost:8000?canisterId=${IDENTITY_CANISTER_ID}`;
    authClient.login({
      onSuccess: onLogin,
      identityProvider,
      maxTimeToLive: days * hours * nanoseconds,
    });
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

  return (
    <Flex justifyContent={'center'} margin="auto">
      <Box minW="sm" minH="xs" borderWidth='1px' borderRadius='lg' overflow='hidden' padding="16px">
        <Flex justifyContent={'center'} flexDir="column" h="100%">
          <Heading as="h2" size="lg" mt="16px" textAlign={'center'}>No Key Wallet</Heading>

          <Flex flexDirection={"column"} alignItems={"center"} h="100%">
            
            <Box mt="auto">
            {loggedIn ? (
              <Box>

                {!address && (
                  <Button onClick={handleCreateEVMWallet}>Create EVM Wallet</Button>
                )}

                <Box mb="40px">
                  {balance && <Text textAlign="center" fontSize="3xl">{parseFloat(balance).toFixed(3)} <Box as="span" fontSize="20px">ETH</Box></Text>}
                </Box>
                <Box mb="12px">
                  {address && <Text><Badge>Address:</Badge> {address.slice(0, 8)}...{address.slice(-6)}</Text>}
                </Box>
              </Box>
            ) : (
              <Button onClick={login} rightIcon={<IcLogo />}>
                Login with
              </Button>
            )}
            </Box>

            {/* <Box>
              <Button onClick={handleTopUp}>Top up</Button>
            </Box> */}

            <Divider mb="16px" mt="auto"/>
            <Box>
              <Button variant="ghost" onClick={onHistoryOpen} leftIcon={<HiClock />} disabled={!loggedIn}>History</Button>
              <Button ml="8px" onClick={onSendOpen} leftIcon={<HiPlusCircle />} disabled={!loggedIn}>Transfer</Button>
              <Button variant="ghost" ml="8px" onClick={logout} leftIcon={<HiArrowLeftCircle />} disabled={!loggedIn}>Logout</Button>
            </Box>

            <SendEthModal provider={provider} setTransactions={setTransactions} setBalance={setBalance} actor={actor} chainId={chainId} address={address} isOpen={isSendOpen} onClose={onSendClose} />
            <TransactionsModal chainId={chainId} actor={actor} setTransactions={setTransactions} transactions={transactions} isOpen={isHistoryOpen} onClose={onHistoryClose} />
          </Flex>
        </Flex>
      </Box>
    </Flex>
  );
};

export default App;
