const { Actor, HttpAgent } = require("@dfinity/agent");
const { Principal } = require("@dfinity/principal");

const { Chain, Common, Hardfork } = require("@ethereumjs/common");
const { Transaction } = require("@ethereumjs/tx");
const { ecdsaVerify, ecdsaRecover } = require("secp256k1");
const { splitSignature } = require("@ethersproject/bytes");

const crypto = require("crypto");
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
        signature: IDL.Vec(IDL.Nat8),
        msg_hash: IDL.Vec(IDL.Nat8),
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

    const txParams = {
      nonce: 0,
      gasPrice: "0x09184e72a000",
      gasLimit: "0x7530",
      to: await user2.getAddress(),
      value: ethers.utils.parseEther(value).toHexString(),
      data: "0x7f7465737432000000000000000000000000000000000000000000000000000000600057",
    };
    // computeAddress from public_key fn
    const response = await actor.get_public_key();
    const publicKey = Buffer.from(response.Ok.public_key);
    console.log("publicKey", publicKey.toString("hex"), "\n");

    const address = ethers.utils.computeAddress(publicKey);
    console.log("address", address, "\n");

    const tx = createRawTx(txParams);

    const { txs, signature, msgHash } = await signTx(tx, actor);

    // const hash = crypto
    //   .createHash("sha256")
    //   .update(txStr.slice(0, -6) + "018080")
    //   .digest();
    // console.log([...hash]);

    const { r, s, v } = splitSignature(signature);

    console.log("tx", txs)
    console.log();
    console.log("signature", signature.toString("hex"));
    console.log();

    const verified = ecdsaVerify(signature, msgHash, publicKey);
    console.log("verified = ", verified);

    // console.log("txs", txs);
    // console.log();

    const tx2 = Transaction.fromSerializedTx(Buffer.from(txs.slice(2), "hex"));
    const receiverAddress = tx2.getSenderAddress().toString("hex");

    const hash = "0x" + tx2.getMessageToVerifySignature().toString("hex");

    console.log("unsign", msgHash);
    console.log("hash", hash);

    console.log();
    const address2 = ethers.utils.recoverAddress(hash, {
      r,
      s,
      v: 37,
    });

    // Using an expanded Signature
    // const address2 = ethers.utils.recoverAddress(msgHash, {
    //   r: "0x930956309a27f2e1579f5436eeee9368ec2929c89623add84951a003960433cd",
    //   s: "0x71f24ab18d769667eb34314233f39c3c5a9ddc3e503cd27d331fc13a08547f23",
    //   v: "0x25",
    // });

    console.log("address2", address2);
    console.log();

    console.log("receiverAddress", receiverAddress);
    console.log();

    console.log(Buffer.from(hash.slice(2), "hex"))

    console.log(signature)
    const publicKeyRecovered = Buffer.from(ecdsaRecover(signature, 0, Buffer.from(hash.slice(2), "hex")));
    console.log("recovered = ", publicKey.equals(publicKeyRecovered));

    console.log(ethers.utils.computeAddress(publicKeyRecovered));
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

  const signedTX = await actor.sign_evm_tx([...serializedTx]);

  return {
    txs: "0x" + Buffer.from(signedTX.Ok.sign_tx, "hex").toString("hex"),
    signature: Buffer.from(signedTX.Ok.signature),
    msgHash: Buffer.from(signedTX.Ok.msg_hash),
  };
};
