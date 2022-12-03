// https://github.com/ethereum-lists/chains/tree/master/_data

const mainnets = [
{
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
  "name": "Binance Smart Chain",
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
}
]
export { mainnets }

const testnets = [
{
	"name": "Ethereum Sepolia",
	"title": "Ethereum Testnet Sepolia",
	"chain": "ETH",
	"rpc": ["https://rpc.sepolia.org", "https://rpc-sepolia.rockx.com"],
	"faucets": ["http://fauceth.komputing.org?chain=11155111"],
	"nativeCurrency": {
		"name": "Sepolia Ether",
		"symbol": "SEP",
		"decimals": 18
	},
	"infoURL": "https://sepolia.otterscan.io",
	"shortName": "sep",
	"chainId": 11155111,
	"networkId": 11155111,
	"explorers": [
		{
			"name": "etherscan-sepolia",
			"url": "https://sepolia.etherscan.io",
			"standard": "EIP3091"
		},
		{
			"name": "otterscan-sepolia",
			"url": "https://sepolia.otterscan.io",
			"standard": "EIP3091"
		}
	]
},
{
  "name": "Ethereum Goerli",
  "title": "Ethereum Testnet Goerli",
  "chain": "ETH",
  "rpc": [
    "https://rpc.goerli.mudit.blog/"
  ],
  "faucets": [
    "http://fauceth.komputing.org?chain=5",
    "https://goerli-faucet.slock.it",
    "https://faucet.goerli.mudit.blog"
  ],
  "nativeCurrency": {
    "name": "Goerli Ether",
    "symbol": "ETH",
    "decimals": 18
  },
  "infoURL": "https://goerli.net/#about",
  "shortName": "gor",
  "chainId": 5,
  "networkId": 5,
  "ens": {
    "registry": "0x112234455c3a32fd11230c42e7bccd4a84e02010"
  },
  "explorers": [
    {
      "name": "etherscan-goerli",
      "url": "https://goerli.etherscan.io",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Smart Chain Testnet",
  "chain": "BSC",
  "rpc": [
    "https://data-seed-prebsc-1-s1.binance.org:8545",
    "https://data-seed-prebsc-2-s1.binance.org:8545",
    "https://data-seed-prebsc-1-s2.binance.org:8545",
    "https://data-seed-prebsc-2-s2.binance.org:8545",
    "https://data-seed-prebsc-1-s3.binance.org:8545",
    "https://data-seed-prebsc-2-s3.binance.org:8545"
  ],
  "faucets": ["https://testnet.binance.org/faucet-smart"],
  "nativeCurrency": {
    "name": "Binance Chain Native Token",
    "symbol": "tBNB",
    "decimals": 18
  },
  "infoURL": "https://testnet.binance.org/",
  "shortName": "bnbt",
  "chainId": 97,
  "networkId": 97,
  "explorers": [
    {
      "name": "bscscan-testnet",
      "url": "https://testnet.bscscan.com",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Polygon Mumbai",
  "title": "Polygon Testnet Mumbai",
  "chain": "Polygon",
  "icon": "polygon",
  "rpc": [
    "https://matic-mumbai.chainstacklabs.com",
    "https://rpc-mumbai.maticvigil.com",
    "https://matic-testnet-archive-rpc.bwarelabs.com"
  ],
  "faucets": ["https://faucet.polygon.technology/"],
  "nativeCurrency": {
    "name": "MATIC",
    "symbol": "MATIC",
    "decimals": 18
  },
  "infoURL": "https://polygon.technology/",
  "shortName": "maticmum",
  "chainId": 80001,
  "networkId": 80001,
  "explorers": [
    {
      "name": "polygonscan",
      "url": "https://mumbai.polygonscan.com",
      "standard": "EIP3091"
    }
  ]
},
{
  "name": "Arbitrum Nova",
  "chainId": 42170,
  "shortName": "arb-nova",
  "chain": "ETH",
  "networkId": 42170,
  "nativeCurrency": {
    "name": "Ether",
    "symbol": "ETH",
    "decimals": 18
  },
  "rpc": ["https://nova.arbitrum.io/rpc"],
  "faucets": [],
  "explorers": [
    {
      "name": "Arbitrum Nova Chain Explorer",
      "url": "https://nova-explorer.arbitrum.io",
      "icon": "blockscout",
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
	"name": "Avalanche Fuji Testnet",
	"chain": "AVAX",
	"icon": "avax",
	"rpc": ["https://api.avax-test.network/ext/bc/C/rpc"],
	"faucets": ["https://faucet.avax-test.network/"],
	"nativeCurrency": {
		"name": "Avalanche",
		"symbol": "AVAX",
		"decimals": 18
	},
	"infoURL": "https://cchain.explorer.avax-test.network",
	"shortName": "Fuji",
	"chainId": 43113,
	"networkId": 1,
	"explorers": [
		{
			"name": "snowtrace",
			"url": "https://testnet.snowtrace.io",
			"standard": "EIP3091"
		}
	]
},
{
  "name": "Fantom Testnet",
  "chain": "FTM",
  "rpc": ["https://rpc.testnet.fantom.network"],
  "faucets": ["https://faucet.fantom.network"],
  "nativeCurrency": {
    "name": "Fantom",
    "symbol": "FTM",
    "decimals": 18
  },

  "infoURL": "https://docs.fantom.foundation/quick-start/short-guide#fantom-testnet",
  "shortName": "tftm",
  "chainId": 4002,
  "networkId": 4002,
  "icon": "fantom",
  "explorers": [
    {
      "name": "ftmscan",
      "url": "https://testnet.ftmscan.com",
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
  "chainId": 31337,
  "networkId": 31337,
  "icon": "ethereum",
  "explorers": []
}
]
export { testnets }