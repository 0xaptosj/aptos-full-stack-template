export const ABI = {
  address: "0x4b1990191a80bed96eacb6df71ab0b0a5b16a9749dc7304edbce44db18e4b64f",
  name: "message_board",
  friends: [],
  exposed_functions: [
    {
      name: "create_message",
      visibility: "public",
      is_entry: true,
      is_view: false,
      generic_type_params: [],
      params: ["&signer", "0x1::string::String"],
      return: [],
    },
    {
      name: "get_message_content",
      visibility: "public",
      is_entry: false,
      is_view: true,
      generic_type_params: [],
      params: [
        "0x1::object::Object<0x4b1990191a80bed96eacb6df71ab0b0a5b16a9749dc7304edbce44db18e4b64f::message_board::Message>",
      ],
      return: ["0x1::string::String", "address"],
    },
    {
      name: "update_message",
      visibility: "public",
      is_entry: true,
      is_view: false,
      generic_type_params: [],
      params: [
        "&signer",
        "0x1::object::Object<0x4b1990191a80bed96eacb6df71ab0b0a5b16a9749dc7304edbce44db18e4b64f::message_board::Message>",
        "0x1::string::String",
      ],
      return: [],
    },
  ],
  structs: [
    {
      name: "CreateMessageEvent",
      is_native: false,
      abilities: ["drop", "store"],
      generic_type_params: [],
      fields: [
        { name: "message_obj_addr", type: "address" },
        {
          name: "message",
          type: "0x4b1990191a80bed96eacb6df71ab0b0a5b16a9749dc7304edbce44db18e4b64f::message_board::Message",
        },
      ],
    },
    {
      name: "Message",
      is_native: false,
      abilities: ["copy", "drop", "store", "key"],
      generic_type_params: [],
      fields: [
        { name: "creator", type: "address" },
        { name: "content", type: "0x1::string::String" },
        { name: "creation_timestamp", type: "u64" },
        { name: "last_update_timestamp", type: "0x1::option::Option<u64>" },
      ],
    },
    {
      name: "UpdateMessageEvent",
      is_native: false,
      abilities: ["drop", "store"],
      generic_type_params: [],
      fields: [
        { name: "message_obj_addr", type: "address" },
        {
          name: "message",
          type: "0x4b1990191a80bed96eacb6df71ab0b0a5b16a9749dc7304edbce44db18e4b64f::message_board::Message",
        },
      ],
    },
  ],
} as const;
