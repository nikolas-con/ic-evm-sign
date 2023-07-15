const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");
const { createRawTx1559, createRawTx2930, createRawTxLegacy, signTx } = require("./utils");
const { idleServiceOptions } = require("./utils/idleService");
const path = require("path");
const { assert } = require("chai");
const { ethers } = require("hardhat");
const fetch = require("node-fetch");

global.fetch = fetch;

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
      "canister_ids"
    ));

    const canisterId = Principal.fromText(canisters.backend.local);

    const agent = new HttpAgent({ host: "http://127.0.0.1:8000" });
    agent.fetchRootKey();

    const createActorOptions = { agent, canisterId };
    actor = Actor.createActor(idlFactory, createActorOptions);
    const { chainId } = await ethers.provider.getNetwork();
    let address;
    const [caller] = await actor.get_caller_data(chainId);

    if (!caller) {
      const res = await actor.create_address();
      address = res.Ok.address;
    } else {
      address = caller.address;
      await actor.clear_caller_history(chainId);
    }

    const [owner, user] = await ethers.getSigners();

    otherUser = user;

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("10"),
    });
  });

  it("Sign Legacy Transaction", async function () {
    const nonce = 0;
    const gasPrice = await ethers.provider
      .getGasPrice()
      .then((s) => s.toHexString());
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();
    const { chainId } = await ethers.provider.getNetwork()

    const txParams = {
      nonce,
      gasPrice,
      gasLimit,
      to,
      value: value_hex,
      data,
    };

    const tx = createRawTxLegacy(txParams, chainId);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value)));
  });

  it("Sign EIP1559 Transaction", async function () {
    const { chainId } = await ethers.provider.getNetwork();
    const [caller] = await actor.get_caller_data(chainId);
    const nonce = Number(caller.transactions.nonce);
    const { maxFeePerGas, maxPriorityFeePerGas } =
      await ethers.provider.getFeeData();
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();
    const type = ethers.BigNumber.from("2").toHexString();
    const chainId_tx = ethers.BigNumber.from(chainId.toString()).toHexString();

    const txData = {
      data,
      gasLimit,
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to,
      value: value_hex,
      chainId: chainId_tx,
      accessList: [],
      type,
    };

    const tx = createRawTx1559(txData, chainId);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value)));
  });

  it("Sign EIP2930 Transaction", async function () {
    const { chainId } = await ethers.provider.getNetwork();
    const [caller] = await actor.get_caller_data(chainId);
    const nonce = Number(caller.transactions.nonce);
    const { maxPriorityFeePerGas, gasPrice } =
      await ethers.provider.getFeeData();
    const gasLimit = ethers.BigNumber.from("23000").toHexString();
    const to = await otherUser.getAddress();
    const value = "1";
    const value_hex = ethers.utils.parseEther(value).toHexString();
    const data = ethers.BigNumber.from("0").toHexString();
    const chainId_tx = ethers.BigNumber.from(chainId.toString()).toHexString();
    const type = ethers.BigNumber.from("1").toHexString();

    const txData = {
      data,
      gasLimit,
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      gasPrice: gasPrice.toHexString(),
      nonce,
      to,
      value: value_hex,
      chainId: chainId_tx,
      accessList: [],
      type,
    };

    const tx = createRawTx2930(txData, chainId);

    const signedTx = await signTx(tx, actor);

    const otherUserBefore = await otherUser.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const otherUserAfter = await otherUser.getBalance();

    assert.ok(otherUserAfter.sub(otherUserBefore).eq(ethers.utils.parseEther(value)));
  });

  it("Deploy and used a contract with high level functions from canister", async function () {
    const { chainId } = await ethers.provider.getNetwork();

    const [caller] = await actor.get_caller_data(chainId);
    const address = caller.address;

    const contract = await ethers.getContractFactory("ExampleToken");

    const estimatedGasDeploy = await ethers.provider.estimateGas({
      data: contract.getDeployTransaction().data,
    });

    const bytecode = Buffer.from(contract.bytecode.substring(2), "hex");

    const { maxFeePerGas, maxPriorityFeePerGas } =
      await ethers.provider.getFeeData();

    const resDeployContract = await actor.deploy_evm_contract(
      [...bytecode],
      chainId,
      maxPriorityFeePerGas.toNumber(),
      estimatedGasDeploy.toNumber(),
      maxFeePerGas.toNumber()
    );

    const txSignedDeployContract = "0x" + Buffer.from(resDeployContract.Ok.tx, "hex").toString("hex");

    const { hash } = await ethers.provider.sendTransaction(txSignedDeployContract);

    const receiptDeployContract = await ethers.provider.waitForTransaction(
      hash
    );

    const contractAddress = receiptDeployContract.contractAddress;

    const deployedContract = contract.attach(contractAddress);

    const balance = await deployedContract.balanceOf(address);
    assert.ok(balance.eq(ethers.utils.parseUnits("100000", 18)));

    const addressOtherUser = await otherUser.getAddress();
    const resTransferERC20 = await actor.transfer_erc_20(
      chainId,
      maxPriorityFeePerGas.toNumber(),
      estimatedGasDeploy.toNumber(),
      maxFeePerGas.toNumber(),
      addressOtherUser,
      1000000000000000000,
      contractAddress
    );

    const txSignedTransferERC20 = "0x" + Buffer.from(resTransferERC20.Ok.tx, "hex").toString("hex");

    const { hash: hashTransferERC20 } = await ethers.provider.sendTransaction(txSignedTransferERC20);

    await ethers.provider.waitForTransaction(hashTransferERC20);

    const balanceOtherUser = await deployedContract.balanceOf(addressOtherUser);

    assert.ok(balanceOtherUser.eq(ethers.utils.parseUnits("1", 18)));
  });

  it("Deploy and used a contract", async function () {
    const { chainId } = await ethers.provider.getNetwork();

    const [caller] = await actor.get_caller_data(chainId);
    const address = caller.address;

    const contract = await ethers.getContractFactory("Example");

    const estimatedGasDeploy = await ethers.provider.estimateGas({
      data: contract.getDeployTransaction().data,
    });

    const { maxFeePerGas, maxPriorityFeePerGas } = await ethers.provider.getFeeData();
    let nonce = Number(caller.transactions.nonce);
    const value = ethers.BigNumber.from("0");
    const type = ethers.BigNumber.from("2");
    const chainId_tx = ethers.BigNumber.from(chainId.toString()).toHexString();

    const txDataDeployContract = {
      data: contract.bytecode,
      gasLimit: estimatedGasDeploy.toHexString(),
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to: null,
      value: value.toHexString(),
      chainId: chainId_tx,
      accessList: [],
      type: type.toHexString(),
    };

    const deployContractTx = createRawTx1559(txDataDeployContract, chainId);

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
    nonce = nonce + 1;

    const txData = {
      data: setNameEncoded,
      gasLimit: gasLimit.toHexString(),
      maxPriorityFeePerGas: maxPriorityFeePerGas.toHexString(),
      maxFeePerGas: maxFeePerGas.toHexString(),
      nonce,
      to: deployedContract.address,
      value: value.toHexString(),
      chainId: chainId_tx,
      accessList: [],
      type: type.toHexString(),
    };

    const tx = createRawTx1559(txData, chainId);

    const signedTx = await signTx(tx, actor);

    const { hash: hashSigned } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hashSigned);

    const nameAfter = await deployedContract.name();
    assert.ok(nameAfter === "bar");
  });
});
