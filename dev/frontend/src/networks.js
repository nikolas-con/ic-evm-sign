// const networks = [{ name: 'Ethereum Mainnet', short: 'Ethereum' }, { name: 'Binance Smart Chain', short: 'BSC' }, { name: 'Polygon', short: 'Polygon' }, { name: 'Arbitrum', short: 'Arbitrum' }, { name: 'Avalanche', short: 'Avalanche' }, { name: 'Fantom', short: 'Fantom' }]

const networks = [{
	"name": "Ethereum Mainnet",
	"chain": "ETH",
	"icon": "ethereum",
	"rpc": [
		"https://cloudflare-eth.com",
		"https://api.mycryptoapi.com/eth"
	],
	"features": [{ "name": "EIP155" }, { "name": "EIP1559" }],
	"faucets": [],
	"nativeCurrency": {
		"name": "Ether",
		"symbol": "ETH",
		"decimals": 18
	},
	"infoURL": "https://ethereum.org",
	"shortName": "eth",
	"chainId": 1,
	"networkId": 1,
	"slip44": 60,
	"ens": {
		"registry": "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e"
	},
	"explorers": [
		{
			"name": "etherscan",
			"url": "https://etherscan.io",
			"standard": "EIP3091"
		}
	]
},
{
  "name": "Binance Smart Chain Mainnet",
  "chain": "BSC",
  "rpc": [
    "https://bsc-dataseed1.binance.org",
    "https://bsc-dataseed2.binance.org",
    "https://bsc-dataseed3.binance.org"
  ],
  "faucets": ["https://free-online-app.com/faucet-for-eth-evm-chains/"],
  "nativeCurrency": {
    "name": "Binance Chain Native Token",
    "symbol": "BNB",
    "decimals": 18
  },
  "infoURL": "https://www.binance.org",
  "shortName": "bnb",
  "chainId": 56,
  "networkId": 56,
  "slip44": 714,
  "explorers": [
    {
      "name": "bscscan",
      "url": "https://bscscan.com",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Polygon Mainnet",
  "chain": "Polygon",
  "icon": "polygon",
  "rpc": [
    "https://polygon-rpc.com/",
    "https://rpc-mainnet.matic.network",
    "https://matic-mainnet.chainstacklabs.com",
    "https://rpc-mainnet.maticvigil.com",
    "https://rpc-mainnet.matic.quiknode.pro",
    "https://matic-mainnet-full-rpc.bwarelabs.com",
    "https://polygon-bor.publicnode.com"
  ],
  "faucets": [],
  "nativeCurrency": {
    "name": "MATIC",
    "symbol": "MATIC",
    "decimals": 18
  },
  "infoURL": "https://polygon.technology/",
  "shortName": "matic",
  "chainId": 137,
  "networkId": 137,
  "slip44": 966,
  "explorers": [
    {
      "name": "polygonscan",
      "url": "https://polygonscan.com",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Arbitrum One",
  "chainId": 42161,
  "shortName": "arb1",
  "chain": "ETH",
  "networkId": 42161,
  "nativeCurrency": {
    "name": "Ether",
    "symbol": "ETH",
    "decimals": 18
  },
  "rpc": [
    "https://arb1.arbitrum.io/rpc"
  ],
  "faucets": [],
  "explorers": [
    {
      "name": "Arbiscan",
      "url": "https://arbiscan.io",
      "standard": "EIP3091"
    },
    {
      "name": "Arbitrum Explorer",
      "url": "https://explorer.arbitrum.io",
      "standard": "EIP3091"
    }
  ],
  "infoURL": "https://arbitrum.io",
  "parent": {
    "type": "L2",
    "chain": "eip155-1",
    "bridges": [{ "url": "https://bridge.arbitrum.io" }]
  }
},
{
  "name": "Avalanche C-Chain",
  "chain": "AVAX",
  "icon": "avax",
  "rpc": ["https://api.avax.network/ext/bc/C/rpc"],
  "features": [{ "name": "EIP1559" }],
  "faucets": ["https://free-online-app.com/faucet-for-eth-evm-chains/"],
  "nativeCurrency": {
    "name": "Avalanche",
    "symbol": "AVAX",
    "decimals": 18
  },
  "infoURL": "https://www.avax.network/",
  "shortName": "avax",
  "chainId": 43114,
  "networkId": 43114,
  "slip44": 9005,
  "explorers": [
    {
      "name": "snowtrace",
      "url": "https://snowtrace.io",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Fantom Opera",
  "chain": "FTM",
  "rpc": ["https://rpc.ftm.tools"],
  "faucets": ["https://free-online-app.com/faucet-for-eth-evm-chains/"],
  "nativeCurrency": {
    "name": "Fantom",
    "symbol": "FTM",
    "decimals": 18
  },
  "infoURL": "https://fantom.foundation",
  "shortName": "ftm",
  "chainId": 250,
  "networkId": 250,
  "icon": "fantom",
  "explorers": [
    {
      "name": "ftmscan",
      "url": "https://ftmscan.com",
      "icon": "ftmscan",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Localhost 8545",
  "chain": "ETH",
  "rpc": ["http://localhost:8545/"],
  "faucets": [],
  "nativeCurrency": {
    "name": "Ethereum",
    "symbol": "ETH",
    "decimals": 18
  },
  "infoURL": "",
  "shortName": "eth",
  "chainId": 1337,
  "networkId": 1337,
  "icon": "ethereum",
  "explorers": []
}]
export default networks
