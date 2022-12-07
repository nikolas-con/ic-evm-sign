const {
  Transaction,
  FeeMarketEIP1559Transaction,
  AccessListEIP2930Transaction,
} = require("@ethereumjs/tx");
const { Common, Hardfork } = require("@ethereumjs/common");


const createRawTxLegacy = (txParams, chainId) => {
  const common = Common.custom({ chainId, hardfork: Hardfork.SpuriousDragon})
  
  const tx = Transaction.fromTxData(txParams, { common });
  
  return tx;
};

const createRawTx1559 = (txParams, chainId) => {
  const common = Common.custom({ chainId ,hardfork: Hardfork.London})
  
  const tx = FeeMarketEIP1559Transaction.fromTxData(txParams, { common });
  
  return tx;
};

const createRawTx2930 = (txParams, chainId) => {
  const common = Common.custom({ chainId, hardfork: Hardfork.Berlin})

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
