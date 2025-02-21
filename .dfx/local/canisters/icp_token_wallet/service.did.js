export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'get_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'receive_tokens' : IDL.Func([IDL.Nat64], [], []),
    'send_tokens' : IDL.Func(
        [IDL.Text, IDL.Nat64],
        [IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
