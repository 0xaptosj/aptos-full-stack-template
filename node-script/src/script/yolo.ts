import { getAptosClient } from "../lib/utils";

const run = async () => {
  const USDT_FA_ADDR =
    "0x357b0b74bc833e95a115ad22604854d6b0fca151cecd94111770e5d6ffc9dc2b";
  getAptosClient()
    .getCurrentFungibleAssetBalances({
      options: {
        where: {
          asset_type: {
            _eq: USDT_FA_ADDR,
          },
        },
        limit: 100,
      },
    })
    .then((res) => {
      console.log(res);
    });
};

run();
