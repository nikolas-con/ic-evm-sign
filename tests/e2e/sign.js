const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");

const { Chain, Common, Hardfork } = require("@ethereumjs/common");
const { Transaction } = require("@ethereumjs/tx");

const { assert } = require("chai");
const { ethers } = require("hardhat");

const path = require("path");
const fetch = require("node-fetch");
global.fetch = fetch;

describe("sign traduction", function () {
  let actor;

  before(async () => {
    const idleServiceOptions = (IDL) => {
      const create_response = IDL.Record({
        address: IDL.Text,
      });
      const sign_tx_response = IDL.Record({
        sign_tx: IDL.Vec(IDL.Nat8),
      });

      return {
        create: IDL.Func(
          [],
          [IDL.Variant({ Ok: create_response, Err: IDL.Text })],
          []
        ),
        sign_evm_tx: IDL.Func(
          [IDL.Vec(IDL.Nat8)],
          [IDL.Variant({ Ok: sign_tx_response, Err: IDL.Text })],
          []
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

  it("sign traduction e2e", async function () {
    const [owner, user2] = await ethers.getSigners();
    const value = "1";

    const res = await actor.create();
    const { address } = res.Ok;

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

    const tx = createRawTx(txParams);

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

const createRawTx = (txParams) => {
  const common = new Common({
    chain: Chain.Mainnet,
    hardfork: Hardfork.SpuriousDragon,
  });

  const tx = Transaction.fromTxData(txParams, { common });

  return tx;
};

const signTx = async (rawTX, actor) => {
  const serializedTx = rawTX.serialize();

  const res = await actor.sign_evm_tx([...serializedTx]);

  return "0x" + Buffer.from(res.Ok.sign_tx, "hex").toString("hex");
};
