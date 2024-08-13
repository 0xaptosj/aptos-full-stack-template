import { getSurfClient } from "../utils";

const run = async () => {
  getSurfClient()
    .view.exist_message({
      typeArguments: [],
      functionArguments: [],
    })
    .then(console.log);
};

run();
