require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

const ALCHEMY_KEY = process.env.ALCHEMY_KEY;

module.exports = {
  solidity: "0.8.17",
  networks: {
    hardhat: {
      chainId: 1,
      forking: {
        url: `https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_KEY}`,
      },
    },
  },
};
