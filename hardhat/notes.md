## todo

- createRawTx() -> hexRaw
- getTxSignature(hexRaw) -> signature
  - getMessageToSign(hexRaw) ->msgHash
  - signIC(msgHash) -> signature
- signTx(signature, hexRaw) -> hexSigned
- clean up
§


## evm tx 

0x
f8
size massge: 93 => 294
82
nonce: 0147
86
gasPrice: 09184e72a000
82
gasLimit: 7530
94
to: 70997970c51812dc3a010c7d01b50e0d17dc79c8
88
value: 8ac7230489e80000
a4
data: 7f7465737432000000000000000000000000000000000000000000000000000000600057
v: 25
a0
r: 3eb5dc8a86c2b0cb79e948b26904f72a1648f54b065a67c209768347ca03a151
a0
s: 34d45214ab378f45a40a14b4c26342bf94266035a15559170323e793a4a92f5b