const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");

const { Chain, Common, Hardfork } = require("@ethereumjs/common");
const { Transaction, FeeMarketEIP1559Transaction } = require("@ethereumjs/tx");

const { assert } = require("chai");
const { ethers } = require("hardhat");

const path = require("path");
const fetch = require("node-fetch");
global.fetch = fetch;

describe("sign traduction", function () {
  let actor;

  before(async () => {
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
          []
        ),
        sign_evm_tx: IDL.Func(
          [IDL.Vec(IDL.Nat8), IDL.Nat8],
          [IDL.Variant({ Ok: sign_tx_response, Err: IDL.Text })],
          []
        ),
        get_caller_data: IDL.Func(
          [],
          [IDL.Variant({ Ok: caller_response, Err: IDL.Text })],
          ["query"]
        ),
      };
    };
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

  it("sign traduction e2e legacy", async function () {
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
      value: ethers.utils.parseEther("15"),
    });

    const txParams = {
      nonce: 0,
      gasPrice: "0x09184e72a000",
      gasLimit: "0x7530",
      to: await user2.getAddress(),
      value: ethers.utils.parseEther(value).toHexString(),
      data: "0x7f7465737432000000000000000000000000000000000000000000000000000000600057",
    };

    const tx = createRawTxLegacy(txParams);

    const signedTx = await signTx(tx, actor);

    const user2Before = await user2.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const icAfter = await ethers.provider.getBalance(address);

    const user2After = await user2.getBalance();

    console.log("user2After", ethers.utils.formatEther(user2After));
    console.log("user2Before", ethers.utils.formatEther(user2Before));

    assert.ok(user2After.sub(user2Before).eq(ethers.utils.parseEther(value)));
  });
  it.only("sign traduction e2e 1559", async function () {
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
    console.log(address);

    await owner.sendTransaction({
      to: address,
      value: ethers.utils.parseEther("200"),
    });
    // 1396406304565822362002;
    // 200000000000000000000;

    const txData = {
      data: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
      gasLimit: "0x7A28",
      maxPriorityFeePerGas: "0x59682f00",
      maxFeePerGas: await ethers.provider
        .getGasPrice()
        .then((s) => s.toHexString()),
      nonce: "0x00",
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

    const icAfter = await ethers.provider.getBalance(address);

    const user2After = await user2.getBalance();

    console.log("user2After", ethers.utils.formatEther(user2After));
    console.log("user2Before", ethers.utils.formatEther(user2Before));

    assert.ok(user2After.sub(user2Before).eq(ethers.utils.parseEther(value)));
  });
});

const createRawTxLegacy = (txParams) => {
  const common = new Common({
    chain: Chain.Mainnet,
    hardfork: Hardfork.SpuriousDragon,
  });

  const tx = Transaction.fromTxData(txParams, { common });

  return tx;
};
const createRawTx1559 = (txParams) => {
  const common = new Common({
    chain: Chain.Mainnet,
    hardfork: Hardfork.London,
  });

  const tx = FeeMarketEIP1559Transaction.fromTxData(txParams, { common });

  return tx;
};

const signTx = async (rawTX, actor) => {
  const serializedTx = rawTX.serialize();
  const { chainId } = await ethers.provider.getNetwork();

  const res = await actor.sign_evm_tx([...serializedTx], Number(chainId));

  return "0x" + Buffer.from(res.Ok.sign_tx, "hex").toString("hex");
};
