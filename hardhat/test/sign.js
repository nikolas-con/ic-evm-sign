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
        signature: IDL.Vec(IDL.Nat8),
      });

      return {
        get_public_key: IDL.Func(
          [],
          [IDL.Variant({ Ok: public_key_info, Err: IDL.Text })],
          []
        ),
        sign_evm_tx: IDL.Func(
          [IDL.Vec(IDL.Nat8)],
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

    const signature = await getTxSignature(tx.raw(), actor);

    const signedTx = signTx(signature, tx);

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

const getTxSignature = async (rawTX, actor) => {
  const msgHash = getMessageToSign(rawTX, true);
  const getSignature = await actor.sign_evm_tx([...msgHash]);
  const signature = Buffer.from(getSignature.Ok.signature, "hex");

  return signature;
};

const getMessageToSign = (raw, hashMessage = true) => {
  const message = raw.slice(0, 6);
  message.push(Buffer.from("01", "hex"));
  message.push(Buffer.from("", "hex"));
  message.push(Buffer.from("", "hex"));

  if (hashMessage) {
    return Buffer.from(
      (0, keccak_1.keccak256)(
        (0, ethereumjs_util_1.arrToBufArr)(
          ethereumjs_rlp_1.RLP.encode(
            (0, ethereumjs_util_1.bufArrToArr)(message)
          )
        )
      )
    );
  } else {
    return message;
  }
};
// - signTx(signature, hexRaw) -> hexSigned

const signTx = (signature, txHexRaw) => {
  const r = signature.subarray(0, 32);
  const s = signature.subarray(32, 64);
  const v = Buffer.from("25", "hex");

  const serializedTx = txHexRaw.serialize();

  const hex =
    serializedTx.toString("hex").slice(0, -6) +
    v.toString("hex") +
    "a0" +
    r.toString("hex") +
    "a0" +
    s.toString("hex");

  const hex2 =
    "0x" +
    hex.substring(0, 2) +
    (hex.substring(4).length / 2).toString(16) +
    hex.substring(4);

  return hex2;
};
