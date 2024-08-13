import { getSurfClient } from "../utils";

const run = async () => {
  getSurfClient()
    .view.get_message_content({
      typeArguments: [],
      functionArguments: [],
    })
    .then(console.log);
};

run();
