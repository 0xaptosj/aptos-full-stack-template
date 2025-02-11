import { expect } from "chai";
import {
  publishMovePackage,
  getTestSigners,
  workspace,
} from "@aptos-labs/workspace";

let packageObjectAddress: string;

describe("my first test", () => {
  let signer;

  it("publish the contract", async () => {
    const [signer1] = await getTestSigners();
    signer = signer1;

    // publish the package, getting back the package object address
    packageObjectAddress = await publishMovePackage({
      publisher: signer,
      namedAddresses: {
        message_board_addr: signer.accountAddress,
      },
      addressName: "message_board_addr",
      packageName: "message-board",
    });

    // get the object account modules
    const accountModules = await workspace.getAccountModules({
      accountAddress: packageObjectAddress,
    });
    // expect the account modules to have at least one module
    expect(accountModules).to.have.length.at.least(1);
  });

  it("create message", async () => {
    // execute entry function `message::set_message(signer, "foobar")`
    const transaction = await workspace.transaction.build.simple({
      sender: signer.accountAddress,
      data: {
        function: `${packageObjectAddress}::message_board::create_message`,
        functionArguments: ["foobar"],
      },
    });

    const response = await workspace.signAndSubmitTransaction({
      signer: signer,
      transaction
    });

    // wait for the transaction to complete
    const committedTransactionResponse = await workspace.waitForTransaction({
      transactionHash: response.hash,
    });
    // the transaction should succeed
    expect(committedTransactionResponse.success).true;
  })
});
