const fetch = require('node-fetch');

// 查询资产转账信息的方法
async function getAssetTransfers() {
  const url = 'https://arb-sepolia.g.alchemy.com/v2/1_KbrbrHrOysh4UFkto_lA-y3PpJoi8l';
  const data = {
    id: 1,
    jsonrpc: "2.0",
    method: "alchemy_getAssetTransfers",
    params: [
      {
        fromBlock: "0x0",
        toBlock: "latest",
        toAddress: "0x6Df01209c6bFb652B8a1F00fAae229a317Dd5dE3",
        withMetadata: false,
        excludeZeroValue: true,
        maxCount: "0x3e8",
        category: [
          "internal"
        ]
      }
    ]
  };

  try {
    const res = await fetch(url, {
      method: 'POST',
      headers: {
        'accept': 'application/json',
        'content-type': 'application/json'
      },
      body: JSON.stringify(data)
    });
    const json = await res.json();
    console.log('返回结果:', JSON.stringify(json, null, 2));
  } catch (err) {
    console.error('请求出错:', err);
  }
}

// 查询指定 token 的价格信息的方法
async function getTokenPrice() {
  const priceUrl = 'https://api.g.alchemy.com/prices/v1/x1K8UJjH9h8iPoVx5EgR3NGvOlS_G_uH/tokens/by-address';
  const priceData = {
    addresses: [
      { network: "eth-mainnet", address: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" }, // ETH
      { network: "eth-mainnet", address: "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599" }, // WBTC
      { network: "eth-mainnet", address: "0xae7ab96520de3a18e5e111b5eaab095312d7fe84" }, // StETH
      { network: "eth-mainnet", address: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" }, // USDC
      { network: "eth-mainnet", address: "0xdac17f958d2ee523a2206206994597c13d831ec7" }, // USDT
      { network: "polygon-mainnet", address: "0x1bfd67037b42cf73acf2047067bd4f2c47d9bfd6" }, // WBTC (Polygon)
      { network: "polygon-mainnet", address: "0x2791bca1f2de4661ed88a30c99a7a9449aa84174" }, // USDC (Polygon)
      { network: "polygon-mainnet", address: "0xc2132d05d31c914a87c6611c10748aeb04b58e8f" }  // USDT (Polygon)
    ]
  };

  try {
    const res = await fetch(priceUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(priceData)
    });
    const json = await res.json();
    // 输出所有地址的价格
    if (json.data && Array.isArray(json.data)) {
      json.data.forEach(item => {
        if (item.error) {
          console.log(`地址: ${item.address} 查询出错: ${item.error}`);
        } else {
          const priceInfo = item.prices && item.prices[0];
          if (priceInfo) {
            console.log(`地址: ${item.address} 价格: ${priceInfo.value} ${priceInfo.currency} 更新时间: ${priceInfo.lastUpdatedAt}`);
          } else {
            console.log(`地址: ${item.address} 没有价格信息`);
          }
        }
      });
    } else {
      console.log('返回数据格式异常:', JSON.stringify(json, null, 2));
    }
  } catch (err) {
    console.error('价格查询出错:', err);
  }
}

// 查询某地址资金来源的方法
async function getFundedBy(address, apiKey) {
  const url = `https://api.etherscan.io/v2/api?chainid=1&module=account&action=fundedby&address=${address}&apikey=${apiKey}`;
  try {
    const res = await fetch(url, {
      method: 'GET',
      headers: {
        'accept': 'application/json'
      }
    });
    const json = await res.json();
    console.log('fundedby 查询结果:', JSON.stringify(json, null, 2));
  } catch (err) {
    console.error('fundedby 查询出错:', err);
  }
}

// 主流程调用
// getAssetTransfers();
// getTokenPrice();
getFundedBy('0x8f5419c8797cbdecaf3f2f1910d192f4306d527d', '7DB3XUDMGCRF9D1W4ZGUE8JZRIURN3RFH6');