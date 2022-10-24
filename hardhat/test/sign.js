const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");

const { Chain, Common, Hardfork } = require("@ethereumjs/common");
const { Transaction } = require("@ethereumjs/tx");

const ethereumjs_rlp_1 = require("@nomicfoundation/ethereumjs-rlp");
const ethereumjs_util_1 = require("@nomicfoundation/ethereumjs-util");
const keccak_1 = require("ethereum-cryptography/keccak");

const { assert } = require("chai");
const { ethers } = require("hardhat");

const path = require("path");
const fetch = require("node-fetch");
global.fetch = fetch;

describe("sign traduction", function () {
  let actor;

  before(async () => {
    const idleServiceOptions = (IDL) => {
      const public_key_info = IDL.Record({
        public_key: IDL.Vec(IDL.Nat8),
      });
      const sign_info = IDL.Record({
        sign_tx: IDL.Vec(IDL.Nat8),
      });

      return {
        get_public_key: IDL.Func(
          [],
          [IDL.Variant({ Ok: public_key_info, Err: IDL.Text })],
          []
        ),
        sign_evm_tx: IDL.Func(
          [IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
          [IDL.Variant({ Ok: sign_info, Err: IDL.Text })],
          []
        ),
      };
    };
    const idlFactory = ({ IDL }) => IDL.Service(idleServiceOptions(IDL));

    const canisters = require(path.resolve(
      "..",
      "IC",
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

  it("completes the withdrawal", async function () {
    const [owner, user2] = await ethers.getSigners();
    const value = "1";

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

    const tx2 = Transaction.fromSerializedTx(
      Buffer.from(signedTx.slice(2), "hex")
    );
    const receiverAddress = tx2.getSenderAddress().toString("hex");

    await owner.sendTransaction({
      to: receiverAddress,
      value: ethers.utils.parseEther("15"),
    });

    const icBefore = await ethers.provider.getBalance(receiverAddress);
    const user2Before = await user2.getBalance();

    const { hash } = await ethers.provider.sendTransaction(signedTx);

    await ethers.provider.waitForTransaction(hash);

    const icAfter = await ethers.provider.getBalance(receiverAddress);
    const user2After = await user2.getBalance();
    console.log("user2After", user2After);
    console.log("user2Before", user2Before);

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
  const msgHash = getMessageToSign(rawTX.serialize());

  const serializedTx = rawTX.serialize();

  const signedTX = await actor.sign_evm_tx([...serializedTx], [...msgHash]);

  return "0x" + Buffer.from(signedTX.Ok.sign_tx, "hex").toString("hex");
};

const getMessageToSign = (rawTxHex) => {
  const tx = Transaction.fromSerializedTx(rawTxHex);

  const rawTx = tx.raw();

  rawTx[6] = Buffer.from("01", "hex");

  const bufArrToArr = rawTx.map((item) => Uint8Array.from(item ?? []));

  console.log(bufArrToArr);
  const encode = ethereumjs_rlp_1.RLP.encode(bufArrToArr);

  const convertedToHex = Buffer.from(encode, "hex");

  console.log(convertedToHex.toString("hex"));

  const hexHash = keccak_1.keccak256(convertedToHex);

  return hexHash;
};
