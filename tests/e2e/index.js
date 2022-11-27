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
  let otherUser;

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

    const res = await actor.create();
    // const res = await actor.get_caller_data();

    const address = res.Ok.address;
    const [owner, user] = await ethers.getSigners();

    otherUser = user;

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("10"),
    });
  });

  it("Sign Legacy Transaction", async function () {
    const res = await actor.get_caller_data();
    const address = res.Ok.address;

    const nonce = await ethers.provider.getTransactionCount(address);
    const gasPrice = await ethers.provider
      .getGasPrice()
      .then((s) => s.toHexString());
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();

    const txParams = {
      nonce,
      gasPrice,
      gasLimit,
      to,
      value: value_hex,
      data,
    };

    const tx = createRawTxLegacy(txParams);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(
      otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value))
    );
  });
  it("Sign EIP1559 Transaction", async function () {
    const res = await actor.get_caller_data();
    const address = res.Ok.address;

    const nonce = await ethers.provider.getTransactionCount(address);
    const { maxFeePerGas, maxPriorityFeePerGas } =
      await ethers.provider.getFeeData();
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();
    const chainId = await ethers.provider
      .getNetwork()
      .then((v) => ethers.BigNumber.from(v.chainId.toString()));
    const type = ethers.BigNumber.from("2").toHexString();

    const txData = {
      data,
      gasLimit,
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to,
      value: value_hex,
      chainId: chainId.toHexString(),
      accessList: [],
      type,
    };

    const tx = createRawTx1559(txData);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(
      otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value))
    );
  });
  it("Sign EIP2930 Transaction", async function () {
    const res = await actor.get_caller_data();
    const address = res.Ok.address;

    const nonce = await ethers.provider.getTransactionCount(address);
    const { maxPriorityFeePerGas, gasPrice } =
      await ethers.provider.getFeeData();
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();
    const chainId = await ethers.provider
      .getNetwork()
      .then((v) => ethers.BigNumber.from(v.chainId.toString()));
    const type = ethers.BigNumber.from("1").toHexString();

    const txData = {
      data,
      gasLimit,
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      gasPrice: gasPrice.toHexString(),
      nonce,
      to,
      value: value_hex,
      chainId: chainId.toHexString(),
      accessList: [],
      type,
    };

    const tx = createRawTx2930(txData);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(
      otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value))
    );
  });
  it.only("Deploy and used a contract hil", async function () {
    const res = await actor.get_caller_data();
    const address = res.Ok.address;

    const contract = await ethers.getContractFactory("ExampleToken");

    const estimatedGasDeploy = await ethers.provider.estimateGas({
      data: contract.getDeployTransaction().data,
    });

    const bytecode = Buffer.from(contract.bytecode.substring(2), "hex");

    const { maxFeePerGas, maxPriorityFeePerGas } =
      await ethers.provider.getFeeData();

    const chainId = await ethers.provider.getNetwork().then((v) => v.chainId);
    maxFeePerGas.toNumber();

    const res1 = await actor.deploy_evm_contract(
      [...bytecode],
      chainId,
      maxPriorityFeePerGas.toNumber(),
      estimatedGasDeploy.toNumber(),
      maxFeePerGas.toNumber()
    );

    const tx = "0x" + Buffer.from(res1.Ok.tx, "hex").toString("hex");

    const { hash } = await ethers.provider.sendTransaction(tx);

    const receiptDeployContract = await ethers.provider.waitForTransaction(
      hash
    );

    const contractAddress = receiptDeployContract.contractAddress;

    const deployedContract = contract.attach(contractAddress);

    const balance = await deployedContract.balanceOf(address);
    assert.ok(balance.eq(ethers.utils.parseUnits("100000", 18)));

    const addressOtherUser = await otherUser.getAddress();
    const res2 = await actor.transfer_erc_20(
      chainId,
      maxPriorityFeePerGas.toNumber(),
      estimatedGasDeploy.toNumber(),
      maxFeePerGas.toNumber(),
      addressOtherUser,
      1000000000000000000,
      contractAddress
    );

    const tx2 = "0x" + Buffer.from(res2.Ok.tx, "hex").toString("hex");

    const { hash: hash2 } = await ethers.provider.sendTransaction(tx2);

    await ethers.provider.waitForTransaction(hash2);

    const balanceOtherUser = await deployedContract.balanceOf(addressOtherUser);

    assert.ok(balanceOtherUser.eq(ethers.utils.parseUnits("1", 18)));
  });
  it("Deploy and used a contract", async function () {
    const res = await actor.get_caller_data();
    const address = res.Ok.address;

    const contract = await ethers.getContractFactory("Example");

    const estimatedGasDeploy = await ethers.provider.estimateGas({
      data: contract.getDeployTransaction().data,
    });

    const data = contract.bytecode;

    const { maxFeePerGas, maxPriorityFeePerGas } =
      await ethers.provider.getFeeData();

    let nonce = await ethers.provider.getTransactionCount(address);

    const value = ethers.BigNumber.from("0");

    const chainId = await ethers.provider
      .getNetwork()
      .then((v) => ethers.BigNumber.from(v.chainId.toString()));

    const type = ethers.BigNumber.from("2");

    const txDataDeployContract = {
      data,
      gasLimit: estimatedGasDeploy.toHexString(),
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to: null,
      value: value.toHexString(),
      chainId: chainId.toHexString(),
      accessList: [],
      type: type.toHexString(),
    };

    const deployContractTx = createRawTx1559(txDataDeployContract);

    const deployContractSignedTx = await signTx(deployContractTx, actor);

    const { hash } = await ethers.provider.sendTransaction(
      deployContractSignedTx
    );

    const receiptDeployContractTx = await ethers.provider.waitForTransaction(
      hash
    );

    const deployedContract = contract.attach(
      receiptDeployContractTx.contractAddress
    );

    const nameBefore = await deployedContract.name();

    assert.ok(nameBefore === "foo");

    const ABI = ["function setName(string memory _name)"];
    const iface = new ethers.utils.Interface(ABI);

    const setNameEncoded = iface.encodeFunctionData("setName", ["bar"]);
    const gasLimit = await deployedContract.estimateGas.setName("bar");
    nonce = await ethers.provider.getTransactionCount(address);

    const txData = {
      data: setNameEncoded,
      gasLimit: gasLimit.toHexString(),
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to: deployedContract.address,
      value: value.toHexString(),
      chainId: chainId.toHexString(),
      accessList: [],
      type: type.toHexString(),
    };

    const tx = createRawTx1559(txData);

    const signedTx = await signTx(tx, actor);

    const { hash: hash2 } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash2);

    const nameAfter = await deployedContract.name();
    assert.ok(nameAfter === "bar");
  });
});
