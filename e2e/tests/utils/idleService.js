const idleServiceOptions = (IDL) => {
  const transactions = IDL.Record({
    data: IDL.Vec(IDL.Nat8),
    timestamp: IDL.Nat64,
  });
  const chainData = IDL.Record({
    nonce: IDL.Nat64,
    transactions: IDL.Vec(transactions),
  });
  const create_response = IDL.Record({
    address: IDL.Text,
  });
  const sign_tx_response = IDL.Record({
    sign_tx: IDL.Vec(IDL.Nat8),
  });
  const caller_response = IDL.Record({
    address: IDL.Text,
    transactions: chainData,
  });

  const deploy_response = IDL.Record({
    tx: IDL.Vec(IDL.Nat8),
  });

  const transfer_erc_20_response = IDL.Record({
    tx: IDL.Vec(IDL.Nat8),
  });

  return {
    create: IDL.Func(
      [],
      [IDL.Variant({ Ok: create_response, Err: IDL.Text })],
      []
    ),
    sign_evm_tx: IDL.Func(
      [IDL.Vec(IDL.Nat8), IDL.Nat64],
      [IDL.Variant({ Ok: sign_tx_response, Err: IDL.Text })],
      []
    ),
    get_caller_data: IDL.Func(
      [IDL.Nat64],
      [IDL.Opt(caller_response)],
      ["query"]
    ),
    deploy_evm_contract: IDL.Func(
      [IDL.Vec(IDL.Nat8), IDL.Nat64, IDL.Nat64, IDL.Nat64, IDL.Nat64],
      [IDL.Variant({ Ok: deploy_response, Err: IDL.Text })],
      ["update"]
    ),
    clear_caller_history: IDL.Func([IDL.Nat64], [], ["update"]),
    transfer_erc_20: IDL.Func(
      [
        IDL.Nat64,
        IDL.Nat64,
        IDL.Nat64,
        IDL.Nat64,
        IDL.Text,
        IDL.Nat64,
        IDL.Text,
      ],
      [IDL.Variant({ Ok: transfer_erc_20_response, Err: IDL.Text })],
      ["update"]
    ),
  };
};

module.exports = {
  idleServiceOptions,
};
