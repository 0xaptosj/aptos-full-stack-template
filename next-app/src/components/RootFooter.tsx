import { IndexerStatus } from "@/components/IndexerStatus";

export const RootFooter = () => {
  return (
    <div className="flex space-x-5 items-center justify-center gap-6 pb-10">
      <IndexerStatus />
    </div>
  );
};
