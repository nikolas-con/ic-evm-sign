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
  Divider,
  Spinner,
  IconButton,
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

import { HiClock, HiPlusCircle, HiArrowLeftCircle, HiArrowDownOnSquareStack, HiCog6Tooth, HiArrowTopRightOnSquare, HiOutlineClipboardDocument, HiOutlineClipboardDocumentCheck } from "react-icons/hi2";

import {
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
} from '@chakra-ui/react'

import { Tabs, TabList, TabPanels, Tab, TabPanel } from '@chakra-ui/react'

import { mainnets, testnets } from "./networks"

const getHostFromUrl = (hostUrl) => {
  try {
      const url = new URL(hostUrl)
      return url.host
  } catch (error) {
      return ''
  }
}

const timeSinceShort = (date) => {

  const m = date.toLocaleString('default', { month: 'short' })
  const y = date.getYear()

  const s = Math.floor((new Date() - date) / 1000)
  let i = s / 31536000
  if (i > 1) { return `${m} ${y}` }
  i = s / 86400
  if (i > 1) { const x = Math.floor(i); return `${x}d ago` }
  i = s / 3600
  if (i > 1) { const x = Math.floor(i); return `${x}h ago` }
  i = s / 60
  if (i > 1) { const x = Math.floor(i); return `${x}m ago` }
  return `now`
}

const getDelegationIdentity = () => {
  const identity = localStorage.getItem("identity");
  const key = localStorage.getItem("key");

  if (!identity || !key) return

  const _identity = JSON.parse(identity);
  const chain = DelegationChain.fromDelegations(_identity._delegation.delegations, _identity._delegation.publicKey)

  const _key = JSON.parse(key);
  const keyIdenity = Ed25519KeyIdentity.fromParsedJson(_key)

  const delegationIdentity = DelegationIdentity.fromDelegation(keyIdenity, chain)
  return delegationIdentity
}

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

const SendFundsModal = ({ provider, network, setTransactions, setBalance, actor, address, onClose, isOpen }) => {
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

    toast({ title: "Signing transaction...", variant: "subtle" });

    const res = await actor.sign_evm_tx([...serializeTx], Number(network.chainId));

    const signedTx = Buffer.from(res.Ok.sign_tx, "hex");

    toast({ title: "Sending transaction...", variant: "subtle" });

    const { hash } = await provider.sendTransaction(
      "0x" + signedTx.toString("hex")
    );

    await provider.waitForTransaction(hash);
    toast({ title: `Transfered ${amount} ${network.nativeCurrency.symbol}` });

    const balance = await provider.getBalance(address);
    setBalance(ethers.utils.formatEther(balance));
    setTransactions((txs) => [...txs, { data: signedTx, timestamp: new Date() }]);
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} isCentered size="lg">
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
          <Button onClick={handleSignTx} disabled={!amount || amount === '0'}>Send</Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}

const TransactionsModal = ({ onClose, isOpen, actor, transactions, setTransactions, network }) => {

  const toast = useToast()

  const handleClearTxHistory = async () => {

    toast({ title: "Clearing history...", variant: "subtle" });
    onClose()

    await actor.clear_caller_history(Number(network.chainId));
    setTransactions([]);

    toast({ title: "History cleared" });
  };

  const goToExplorer = (txId) => {
    if (network.explorers.length > 0) {
      window.open(`${network.explorers[0].url}/tx/${txId}`, '_blank').focus()
    } else {
      toast({ title: "There is no explorer for this network", variant: "subtle" });
    }
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose} isCentered size="lg">
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Transaction History</ModalHeader>
        <ModalCloseButton />
        <ModalBody>
          {transactions.length > 0 ?
            <TableContainer>
              <Table variant='simple'>
                <Thead>
                  <Tr>
                    <Th>Transaction Id</Th>
                    <Th>Value</Th>
                    <Th>Created</Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {transactions.map((tx, index) => (
                    <Tr key={index}>
                      <Td>
                        {ethers.utils.parseTransaction(tx.data).hash.slice(0, 8)}...{ethers.utils.parseTransaction(tx.data).hash.slice(-6)}
                        <IconButton onClick={() => goToExplorer(ethers.utils.parseTransaction(tx.data).hash)} ml="4px" fontSize="16px" size="xs" variant="ghost" icon={<HiArrowTopRightOnSquare />} />
                      </Td>
                      <Td>{ethers.utils.formatEther(ethers.utils.parseTransaction(tx.data).value)}</Td>
                      <Td>{timeSinceShort(tx.timestamp)}</Td>
                    </Tr>
                  ))}
                </Tbody>
              </Table>
            </TableContainer> :
            'No transactions yet'}
        </ModalBody>
        <ModalFooter>
          <Button variant='ghost' mr={'auto'} onClick={handleClearTxHistory} disabled={transactions.length === 0}>Clear History</Button>
          <Button type="submit" onClick={onClose}>Close</Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}

const chainId = localStorage.getItem("chain-id") ?? 0
const defaultNetwork = [].concat(testnets, testnets).find(r => r.chainId === +chainId) ?? mainnets[0]

const NetworkModal = ({ onClose, isOpen, setNetwork }) => {

  const selectNetwork = (i, isMainnet) => {

    const network = isMainnet ? mainnets[i] : testnets[i]
    setNetwork(network)
    onClose()
    localStorage.setItem("chain-id", network.chainId);

  }

  return (
    <Modal isOpen={isOpen} onClose={onClose} isCentered size="xs">
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>Select Network</ModalHeader>
        <ModalCloseButton />
        <ModalBody mb="12px">
          <Tabs>
            <TabList justifyContent="center">
              <Tab>Mainnets</Tab>
              <Tab>Testnets</Tab>
            </TabList>
            <TabPanels overflow="scroll" height="280px">
              <TabPanel>
                {mainnets.map((n, i) => <Text key={i} onClick={() => selectNetwork(i, true)} _hover={{ bgColor: '#00000010', cursor: 'pointer' }} padding="8px" borderRadius="4px" textAlign="center">{n.name}</Text>)}
              </TabPanel>
              <TabPanel>
                {testnets.map((n, i) => <Text key={i} onClick={() => selectNetwork(i, false)} _hover={{ bgColor: '#00000010', cursor: 'pointer' }} padding="8px" borderRadius="4px" textAlign="center">{n.name}</Text>)}
              </TabPanel>
            </TabPanels>
          </Tabs>
        </ModalBody>
      </ModalContent>
    </Modal>
  )
}

// internet computer
const IC_URL = "http://localhost:8000";
const BACKEND_CANISTER_ID = process.env.REACT_APP_BACKEND_CANISTER_ID ?? "rrkah-fqaaa-aaaaa-aaaaq-cai";
const IDENTITY_CANISTER_ID = process.env.REACT_APP_IDENTITY_CANISTER_ID ?? "ryjl3-tyaaa-aaaaa-aaaba-cai";

// evm chain
const LOCAL_SIGNER = process.env.LOCAL_SIGNER ?? "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";

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

const getActor = () => {
  const backendCanisterId = Principal.fromText(BACKEND_CANISTER_ID);
  const agent = new HttpAgent({ host: IC_URL });
  agent.fetchRootKey();
  const createActorOptions = { agent, canisterId: backendCanisterId };
  const _actor = Actor.createActor(idlFactory, createActorOptions);
  return _actor
}
const actor = getActor()

const App = () => {

  const toast = useToast()

  const [authClient, setAuthClient] = useState(null);
  const [provider, setProvider] = useState(null);
  const [address, setAddress] = useState(null);
  const [balance, setBalance] = useState(null);
  const [hasCopied, setHasCopied] = useState(false);
  const [network, setNetwork] = useState(defaultNetwork);
  const [loggedIn, setLoggedIn] = useState(false);
  const [transactions, setTransactions] = useState([]);
  const { isOpen: isSendOpen, onOpen: onSendOpen, onClose: onSendClose } = useDisclosure()
  const { isOpen: isHistoryOpen, onOpen: onHistoryOpen, onClose: onHistoryClose } = useDisclosure()
  const { isOpen: isNetworkOpen, onOpen: onNetworkOpen, onClose: onNetworkClose } = useDisclosure()

  const loadUser = useCallback(async (_provider) => {
    try {
      setBalance();
      const res = await actor.get_caller_data(Number(network.chainId));
      const { address, transactions } = res.Ok;
      setAddress(address);
      setTransactions(transactions.transactions.map(tx => ({...tx, timestamp: new Date(Number(tx.timestamp / 1000n / 1000n))})));
      const balance = await _provider.getBalance(address);
      setBalance(ethers.utils.formatEther(balance));
    } catch (error) {
      console.log(error);
      const message = error?.result?.reject_message ?? ''
      toast({ title: "Error", status: 'error', description: message, variant: "subtle" });
    }
  }, [network.chainId, toast])

  const onLogin = async () => {
    setLoggedIn(true);

    const identity = authClient.getIdentity()
    localStorage.setItem("identity", JSON.stringify(identity));
    localStorage.setItem("key", JSON.stringify(authClient._key));

    await loadUser(provider)
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

  const loadProviderAndUser = useCallback(async () => {

    const rpcProvider = new ethers.providers.JsonRpcProvider(network.rpc[0]);
    setProvider(rpcProvider);

    const delegationIdentity = getDelegationIdentity()

    const _authClient = await AuthClient.create({ identity: delegationIdentity });
    setAuthClient(_authClient);

    if (delegationIdentity) {
      setLoggedIn(true);

      await loadUser(rpcProvider)
    }
  }, [loadUser, network.rpc]);

  useEffect(() => {
    loadProviderAndUser();
  }, [loadProviderAndUser, network]);

  const login = async () => {

    const isLocal = getHostFromUrl(IC_URL).startsWith('localhost')
    const identityProvider = isLocal ? `${IC_URL}?canisterId=${IDENTITY_CANISTER_ID}` : "https://identity.ic0.app/#authorize";
    const maxTimeToLive = 24n * 60n * 60n * 1000n * 1000n
    authClient.login({
      onSuccess: onLogin,
      identityProvider,
      maxTimeToLive,
    });
  };

  const handleTopUp = async () => {

    const isHardhat = network.chainId === 31337
    if (isHardhat) {
      const signer = await provider.getSigner(LOCAL_SIGNER);
      await signer.sendTransaction({value: ethers.utils.parseEther("10"), to: address,});
  
      const balance = await provider.getBalance(address);
      setBalance(ethers.utils.formatEther(balance));
    } else if (network.faucets.length > 0) {
      window.open(network.faucets[0], '_blank').focus()
    } else {
      toast({ title: "There is no faucet for this network", variant: "subtle" });
    }
    
  };

  const handleCreateEVMWallet = async () => {

    toast({ title: "Creating wallet...", variant: "subtle" });

    const res = await actor.create();

    toast({ title: "New wallet created" });

    const { address } = res.Ok;
    const balance = await provider.getBalance(address);
    setBalance(ethers.utils.formatEther(balance));
    setAddress(address);
  };

  const copyToClipboard = async () => {
    setHasCopied(true)

    await navigator.clipboard.writeText(address)
    toast({ title: "Copied to clipboard", variant: "subtle" });

    setTimeout(() => setHasCopied(false), 2000)
  }

  const goToExplorer = () => {
    if (network.explorers.length > 0) {
      window.open(`${network.explorers[0].url}/address/${address}`, '_blank').focus()
    } else {
      toast({ title: "There is no explorer for this network", variant: "subtle" });
    }
  }

  return (
    <Flex justifyContent={'center'} margin="auto">
      <Box minW="sm" minH="sm" borderWidth='1px' borderRadius='lg' overflow='hidden' padding="16px">
        <Flex justifyContent={'center'} flexDir="column" h="100%">
          <Heading as="h2" size="lg" mt="16px" textAlign={'center'}>No Key Wallet</Heading>

          <Flex justifyContent="center" mt="20px">
            <Button rightIcon={<HiCog6Tooth />} size="xs" variant='solid' onClick={onNetworkOpen}>
              {network?.name}
            </Button>
          </Flex>

          <Flex flexDirection={"column"} alignItems={"center"} h="100%">

            <Box mt="auto">
              {loggedIn ? (
                <Box>

                  {!address ? (
                    <Button onClick={handleCreateEVMWallet}>Create EVM Wallet</Button>
                  ) :
                    <>
                      <Flex mb="40px" justifyContent="center">
                        {balance ? <Text fontSize="3xl">{parseFloat(balance).toPrecision(3)} <Box as="span" fontSize="20px">{network.nativeCurrency.symbol}</Box></Text> : <Spinner />}
                      </Flex>
                      <Flex mb="12px">
                        {address && <Flex alignItems="center">
                          <Text>{address.slice(0, 10)}...{address.slice(-8)}</Text>
                          <IconButton onClick={copyToClipboard} ml="8px" fontSize="16px" size="xs" variant="ghost" icon={hasCopied ? <HiOutlineClipboardDocumentCheck /> : <HiOutlineClipboardDocument />} />
                          <IconButton onClick={goToExplorer} ml="4px" fontSize="16px" size="xs" variant="ghost" icon={<HiArrowTopRightOnSquare />} />
                        </Flex>}
                      </Flex>
                    </>
                  }
                </Box>
              ) : (
                <Button onClick={login} rightIcon={<IcLogo />}>
                  Login with
                </Button>
              )}
            </Box>

            <Divider mb="16px" mt="auto" />
            <Box>
              <Button variant="ghost" onClick={onHistoryOpen} leftIcon={<HiClock />} disabled={!loggedIn}>History</Button>
              {balance > 0 ?
                <Button ml="8px" onClick={onSendOpen} leftIcon={<HiPlusCircle />} disabled={!loggedIn}>Transfer</Button> :
                <Button ml="8px" onClick={handleTopUp} leftIcon={<HiArrowDownOnSquareStack />} disabled={!loggedIn}>Top up</Button>
              }
              <Button variant="ghost" ml="8px" onClick={logout} leftIcon={<HiArrowLeftCircle />} disabled={!loggedIn}>Logout</Button>
            </Box>

            <SendFundsModal network={network} provider={provider} setTransactions={setTransactions} setBalance={setBalance} actor={actor} address={address} isOpen={isSendOpen} onClose={onSendClose} />
            <TransactionsModal network={network} actor={actor} setTransactions={setTransactions} transactions={transactions} isOpen={isHistoryOpen} onClose={onHistoryClose} />
            <NetworkModal setNetwork={setNetwork} isOpen={isNetworkOpen} onClose={onNetworkClose} />
          </Flex>
        </Flex>
      </Box>
    </Flex>
  );
};

export default App;
