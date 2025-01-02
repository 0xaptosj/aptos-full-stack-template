import { IndexerStatus } from "@/components/IndexerStatus";

export const RootFooter = () => {
  return (
    <div className="flex items-center justify-center gap-6 pb-5">
      <a
        href="https://github.com/0xaptosj/aptos-full-stack-template/"
        target="_blank"
        rel="noreferrer"
        className="text-base text-muted-foreground font-medium leading-none"
      >
        Source Code
      </a>
      <IndexerStatus />
    </div>
  );
};
