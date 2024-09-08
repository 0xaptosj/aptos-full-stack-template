export type MessageInDb = {
  id: number;
  message_obj_addr: string;
  creator_addr: string;
  creation_timestamp: number;
  last_update_timestamp: number;
  last_update_event_idx: number;
  content: string;
};

export type MessageInUi = {
  id: number;
  message_obj_addr: `0x${string}`;
  creator_addr: string;
  creation_timestamp: Date;
  last_update_timestamp: Date;
  content: string;
};

export type MessageBoardColumns = {
  id: number;
  creation_timestamp: Date;
};
