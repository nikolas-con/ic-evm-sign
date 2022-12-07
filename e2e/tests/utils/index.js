const {
  Transaction,
  FeeMarketEIP1559Transaction,
  AccessListEIP2930Transaction,
} = require("@ethereumjs/tx");

const { Chain, Common, Hardfork } = require("@ethereumjs/common");

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

const createRawTx2930 = (txParams) => {
  const common = new Common({
    chain: Chain.Mainnet,
    hardfork: Hardfork.Berlin,
  });

  const tx = AccessListEIP2930Transaction.fromTxData(txParams, { common });

  return tx;
};

const signTx = async (rawTX, actor) => {
  const serializedTx = rawTX.serialize();
  const { chainId } = await ethers.provider.getNetwork();

  const res = await actor.sign_evm_tx([...serializedTx], chainId);

  return "0x" + Buffer.from(res.Ok.sign_tx, "hex").toString("hex");
};

module.exports = {
  createRawTxLegacy,
  createRawTx1559,
  createRawTx2930,
  signTx,
};
