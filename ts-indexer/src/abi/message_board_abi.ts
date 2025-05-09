export const ABI = {
  address: "0x296d2e48f6c649e16fec0aa75f5e42174f2ee36fed10823b7e1722d1f42c7f55",
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
        "0x1::object::Object<0x296d2e48f6c649e16fec0aa75f5e42174f2ee36fed10823b7e1722d1f42c7f55::message_board::Message>",
      ],
      return: ["0x1::string::String", "address", "u64", "u64"],
    },
    {
      name: "update_message",
      visibility: "public",
      is_entry: true,
      is_view: false,
      generic_type_params: [],
      params: [
        "&signer",
        "0x1::object::Object<0x296d2e48f6c649e16fec0aa75f5e42174f2ee36fed10823b7e1722d1f42c7f55::message_board::Message>",
        "0x1::string::String",
      ],
      return: [],
    },
  ],
  structs: [
    {
      name: "CreateMessageEvent",
      is_native: false,
      is_event: true,
      abilities: ["drop", "store"],
      generic_type_params: [],
      fields: [
        { name: "message_obj_addr", type: "address" },
        {
          name: "message",
          type: "0x296d2e48f6c649e16fec0aa75f5e42174f2ee36fed10823b7e1722d1f42c7f55::message_board::Message",
        },
      ],
    },
    {
      name: "Message",
      is_native: false,
      is_event: false,
      abilities: ["copy", "drop", "store", "key"],
      generic_type_params: [],
      fields: [
        { name: "creator", type: "address" },
        { name: "content", type: "0x1::string::String" },
        { name: "creation_timestamp", type: "u64" },
        { name: "last_update_timestamp", type: "u64" },
      ],
    },
    {
      name: "UpdateMessageEvent",
      is_native: false,
      is_event: true,
      abilities: ["drop", "store"],
      generic_type_params: [],
      fields: [
        { name: "message_obj_addr", type: "address" },
        {
          name: "message",
          type: "0x296d2e48f6c649e16fec0aa75f5e42174f2ee36fed10823b7e1722d1f42c7f55::message_board::Message",
        },
      ],
    },
  ],
} as const;
