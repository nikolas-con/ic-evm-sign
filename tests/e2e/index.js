const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");
const {
  createRawTx1559,
  createRawTx2930,
  createRawTxLegacy,
  signTx,
} = require("./utils");
const { idleServiceOptions } = require("./utils/idleService");

const path = require("path");
const fetch = require("node-fetch");
global.fetch = fetch;

const { assert } = require("chai");
const { ethers } = require("hardhat");

describe("Sign EVM Transactions", function () {
  let actor;

  before(async () => {
    const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

    const canisters = require(path.resolve(
      "..",
      "..",
      ".dfx",
      "local",
      "canister_ids.json"
    ));

    const canisterId = Principal.fromText(canisters.IC_backend.local);

    const agent = new HttpAgent({ host: "http://localhost:8000" });
    agent.fetchRootKey();

    const createActorOptions = { agent, canisterId };
    actor = Actor.createActor(idlFactory, createActorOptions);
  });

  it("Sign Legacy Transaction", async function () {
    const [owner, user2] = await ethers.getSigners();
    const value = "1";
    let address;
    try {
      const res = await actor.create();
      address = res.Ok.address;
    } catch (error) {
      const res = await actor.get_caller_data();
      address = res.Ok.address;
    }

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("2"),
    });

    const txParams = {
      nonce: await ethers.provider.getTransactionCount(address),
      gasPrice: await ethers.provider
        .getGasPrice()
        .then((s) => s.toHexString()),
      gasLimit: "0x7530",
      to: await user2.getAddress(),
      value: ethers.utils.parseEther(value).toHexString(),
      data: "0x000000000000000000000000000000000000000000000000000000000000000000000000",
    };

    const tx = createRawTxLegacy(txParams);

    const signedTx = await signTx(tx, actor);

    const user2Before = await user2.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const user2After = await user2.getBalance();

    assert.ok(user2After.sub(user2Before).eq(ethers.utils.parseEther(value)));
  });
  it("Sign EIP1559 Transaction", async function () {
    const [owner, user2] = await ethers.getSigners();
    const value = "0.5";
    let address;
    try {
      const res = await actor.create();
      address = res.Ok.address;
    } catch (error) {
      const res = await actor.get_caller_data();
      address = res.Ok.address;
    }

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("2"),
    });

    const txData = {
      data: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      gasLimit: "0x7A28",
      maxPriorityFeePerGas: "0x59682f00",
      maxFeePerGas: await ethers.provider
        .getGasPrice()
        .then((s) => s.toHexString()),
      nonce: await ethers.provider.getTransactionCount(address),
      to: await user2.getAddress(),
      value: ethers.utils.parseEther(value).toHexString(),
      chainId: "0x01",
      accessList: [],
      type: "0x02",
    };

    const tx = createRawTx1559(txData);

    const signedTx = await signTx(tx, actor);

    const user2Before = await user2.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const user2After = await user2.getBalance();

    assert.ok(user2After.sub(user2Before).eq(ethers.utils.parseEther(value)));
  });
  it("Sign EIP2930 Transaction", async function () {
    const [owner, user2] = await ethers.getSigners();
    const value = "1";
    let address;
    try {
      const res = await actor.create();
      address = res.Ok.address;
    } catch (error) {
      const res = await actor.get_caller_data();
      address = res.Ok.address;
    }

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("200"),
    });

    const txData = {
      data: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      gasLimit: "0x7A28",
      maxPriorityFeePerGas: "0x59682f00",
      gasPrice: await ethers.provider
        .getGasPrice()
        .then((s) => s.toHexString()),
      nonce: await ethers.provider.getTransactionCount(address),
      to: await user2.getAddress(),
      value: ethers.utils.parseEther(value).toHexString(),
      chainId: "0x01",
      accessList: [],
      type: "0x01",
    };

    const tx = createRawTx2930(txData);

    const signedTx = await signTx(tx, actor);

    const user2Before = await user2.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const user2After = await user2.getBalance();

    assert.ok(user2After.sub(user2Before).eq(ethers.utils.parseEther(value)));
  });
});
