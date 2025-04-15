// Standardizes an address / table handle to be a string with length 66 (0x+64 length hex string).
export const standardizeAddress = (handle: string): string => {
  if (handle.startsWith("0x")) {
    return `0x${handle.slice(2).padStart(64, "0")}`;
  } else {
    return `0x${handle.padStart(64, "0")}`;
  }
};
