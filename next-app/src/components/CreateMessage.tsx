"use client";

import { useWalletClient } from "@thalalabs/surf/hooks";
import { useWallet } from "@aptos-labs/wallet-adapter-react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { getAptosClient } from "@/lib/aptos";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useToast } from "@/components/ui/use-toast";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { TransactionOnExplorer } from "@/components/ExplorerLink";
import { ABI } from "@/lib/abi/message_board_abi";
import { useQueryClient } from "@tanstack/react-query";
import { sponsorAndSubmitTxOnServer } from "@/app/actions";

const FormSchema = z.object({
  stringContent: z.string(),
});

export function CreateMessage() {
  const { toast } = useToast();
  const { connected, account, signTransaction } = useWallet();
  const { client: walletClient } = useWalletClient();
  const queryClient = useQueryClient();

  const form = useForm<z.infer<typeof FormSchema>>({
    resolver: zodResolver(FormSchema),
    defaultValues: {
      stringContent: "hello world",
    },
  });

  const onSignAndSubmitTransaction = async (
    data: z.infer<typeof FormSchema>
  ) => {
    if (!account || !walletClient) {
      console.error("Account or wallet client not available");
      return;
    }

    const transaction = await getAptosClient().transaction.build.simple({
      sender: account.address,
      withFeePayer: true,
      data: {
        function: `${ABI.address}::${ABI.name}::create_message`,
        functionArguments: [data.stringContent],
      },
      options: {
        maxGasAmount: 10_000,
      },
    });

    const senderAuthenticator = await signTransaction({
      transactionOrPayload: transaction,
    });

    //   await walletClient
    //     .useABI(ABI)
    //     .create_message({
    //       type_arguments: [],
    //       arguments: [data.stringContent],
    //     })
    await sponsorAndSubmitTxOnServer({
      transactionBytes: Array.from(transaction.bcsToBytes()),
      senderAuthenticatorBytes: Array.from(
        senderAuthenticator.authenticator.bcsToBytes()
      ),
    })
      .then((executedTransaction) => {
        toast({
          title: "Success, tx is sponsored haha",
          description: (
            <TransactionOnExplorer hash={executedTransaction.hash} />
          ),
        });
        return new Promise((resolve) => setTimeout(resolve, 3000));
      })
      .then(() => {
        return queryClient.invalidateQueries({ queryKey: ["messages"] });
      })
      .catch((error) => {
        console.error("Error", error);
        toast({
          title: "Error",
          description: "Failed to create a message",
        });
      });
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Create a new message</CardTitle>
        <CardDescription>
          This is a gasless tx so you can call it without having APT in your
          wallet
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-wrap">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSignAndSubmitTransaction)}
            className="flex flex-col justify-between gap-4 w-1/2"
          >
            <FormField
              control={form.control}
              name="stringContent"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>String Content</FormLabel>
                  <FormControl>
                    <Input {...field} />
                  </FormControl>
                  <FormDescription>Store a string content</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button
              type="submit"
              disabled={!connected}
              className="w-40 self-start col-span-2"
            >
              Create
            </Button>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
}
