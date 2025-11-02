const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider('ws://163.192.125.23:9944')
  });
  
  const operatorAddress = '5GZaEegZ4nUUeg9X6xUe5pdPgSnntdakSuykoNNr2FTsuL3m';
  const { data: balance } = await api.query.system.account(operatorAddress);
  
  const balanceStr = balance.free.toString();
  const whole = balanceStr.slice(0, -18) || '0';
  const decimal = balanceStr.slice(-18).replace(/0+$/, '') || '0';
  
  console.log('Bridge Operator Balance:', decimal ? `${whole}.${decimal}` : whole, 'ETR');
  process.exit(0);
}

main().catch(console.error);
