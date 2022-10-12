// const Tx = require("ethereumjs-tx");
const { Chain, Common, Hardfork } = require("@ethereumjs/common");
const { Transaction } = require("@ethereumjs/tx");
const {
  recoverPublicKey,
  signSync,
} = require("ethereum-cryptography/secp256k1");

const ethereumjs_rlp_1 = require("@nomicfoundation/ethereumjs-rlp");
const ethereumjs_util_1 = require("@nomicfoundation/ethereumjs-util");
const keccak_1 = require("ethereum-cryptography/keccak");

const { ethers, network } = require("hardhat");

describe("sign traduction", function () {
  it("completes the withdrawal", async function () {
    const common = new Common({
      chain: Chain.Mainnet,
      hardfork: Hardfork.SpuriousDragon,
    });

    const from = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    const to = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
    const value = "1";

    const fromPrivateKey =
      "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    // const txCount = await ethers.provider.getTransactionCount(receiverAddress);

    const txParams = {
      nonce: 0,
      gasPrice: "0x09184e72a000",
      gasLimit: "0x7530",
      to,
      value: ethers.utils.parseEther(value).toHexString(),
      data: "0x7f7465737432000000000000000000000000000000000000000000000000000000600057",
    };
    const tx = Transaction.fromTxData(txParams, { common });

    // const privateKey = Buffer.from(fromPrivateKey.split("x")[1], "hex");

    // const msgHash = getMessageToSign(tx.raw(), true);

    // const { r, s, v } = ecsign(msgHash, privateKey);
    const signature = Buffer.from(
      "289380788dc6ff65a961f0f20ac686ec4d9ccdb86b04a3b15e6b88eee087448f1de0d4793a92a23227d6b5236cb01d67280033ca9510ef6932d655f68f1032bc"
    );
    const r = signature.subarray(0, 32);
    const s = signature.subarray(32, 64);
    const v = Buffer.from("25", "hex");

    const hex =
      tx.serialize().toString("hex").slice(0, -6) +
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

    const tx2 = Transaction.fromSerializedTx(Buffer.from(hex2.slice(2), "hex"));
    const receiverAddress = tx2.getSenderAddress().toString("hex");

    const [owner, user2] = await ethers.getSigners();
    await owner.sendTransaction({
      to: receiverAddress,
      value: ethers.utils.parseEther("15"),
    });

    console.log("owner", await owner.getBalance());
    console.log("ic before", await ethers.provider.getBalance(receiverAddress));
    console.log("user2 before", await user2.getBalance());

    const { hash } = await ethers.provider.sendTransaction(hex2);

    await ethers.provider.waitForTransaction(hash);

    console.log("user2 after", await user2.getBalance());
    console.log("ic after", await ethers.provider.getBalance(receiverAddress));
  });
});

// function ecsign(msgHash, privateKey, chainId) {
//   const [signature, recovery] = signSync(msgHash, privateKey, {
//     recovered: true,
//     der: false,
//   });

//   const r = Buffer.from(signature.slice(0, 32));
//   const s = Buffer.from(signature.slice(32, 64));

//   // alagi
//   const v = Buffer.from("25", "hex");

//   return { r, s, v };
// }

// const getMessageToSign = (raw, hashMessage = true) => {
//   const message = raw.slice(0, 6);
//   message.push(Buffer.from("01", "hex"));
//   message.push(Buffer.from("", "hex"));
//   message.push(Buffer.from("", "hex"));

//   if (hashMessage) {
//     return Buffer.from(
//       (0, keccak_1.keccak256)(
//         (0, ethereumjs_util_1.arrToBufArr)(
//           ethereumjs_rlp_1.RLP.encode(
//             (0, ethereumjs_util_1.bufArrToArr)(message)
//           )
//         )
//       )
//     );
//   } else {
//     return message;
//   }
// };
