import { getSurfClient, getAccount } from "../utils";

const run = async () => {
  getSurfClient()
    .entry.post_message({
      typeArguments: [],
      functionArguments: [
        true,
        "Hello, World!",
        100,
        "0x12345",
        "0x15ca2ec4412fbeed545298b466f7df39ae412a6cba063b41bd037d5a52ebe465",
        ["yoyuo"],
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
        undefined,
      ],
      account: getAccount(),
    })
    .then(console.log);
};

run();
