module message_board_addr::message_board {
    use std::option::{Self, Option};
    use std::string::String;
    use std::vector;

    use aptos_std::math64;

    use aptos_framework::object::{Self, Object, ObjectCore};

    const MESSAGE_BOARD_OBJECT_SEED: vector<u8> = b"message_board";

    const DEFAULT_LIMIT: u64 = 10;
    const DEFAULT_OFFSET: u64 = 0;

    struct Message has key, drop {
        boolean_content: bool,
        string_content: String,
        number_content: u64,
        address_content: address,
        object_content: Object<ObjectCore>,
        vector_content: vector<String>,
        optional_boolean_content: Option<bool>,
        optional_string_content: Option<String>,
        optional_number_content: Option<u64>,
        optional_address_content: Option<address>,
        optional_object_content: Option<Object<ObjectCore>>,
        optional_vector_content: Option<vector<String>>,
    }

    struct MessageBoard has key {
        messages: vector<Object<Message>>,
    }

    // This function is only called once when the module is published for the first time.
    // init_module is optional, you can also have an entry function as the initializer.
    fun init_module(sender: &signer) {
        let constructor_ref = &object::create_named_object(sender, MESSAGE_BOARD_OBJECT_SEED);
        move_to(
            &object::generate_signer(constructor_ref),
            MessageBoard {
                messages: vector[],
            },
        );
    }

    // ======================== Write functions ========================

    public entry fun post_message(
        _sender: &signer,
        boolean_content: bool,
        string_content: String,
        number_content: u64,
        address_content: address,
        object_content: Object<ObjectCore>,
        vector_content: vector<String>,
        optional_boolean_content: Option<bool>,
        optional_string_content: Option<String>,
        optional_number_content: Option<u64>,
        optional_address_content: Option<address>,
        optional_object_content: Option<Object<ObjectCore>>,
        optional_vector_content: Option<vector<String>>,
    ) acquires MessageBoard {
        let message_object_constructor_ref = &object::create_object(@message_board_addr);

        move_to(
            &object::generate_signer(message_object_constructor_ref),
            Message {
                boolean_content,
                string_content,
                number_content,
                address_content,
                object_content,
                vector_content,
                optional_boolean_content,
                optional_string_content,
                optional_number_content,
                optional_address_content,
                optional_object_content,
                optional_vector_content,
            },
        );

        let messages =&mut  borrow_global_mut<MessageBoard>(get_message_board_object_address()).messages;
        vector::push_back(messages, object::object_from_constructor_ref(message_object_constructor_ref));
    }

    // ======================== Read Functions ========================

    #[view]
    /// Get message objects, old messages come first
    public fun get_message_objects(limit: Option<u64>, offset: Option<u64>): vector<Object<Message>> acquires MessageBoard {
        let messages = borrow_global<MessageBoard>(get_message_board_object_address()).messages;

        let limit = *option::borrow_with_default(&limit, &DEFAULT_LIMIT);
        let offset = *option::borrow_with_default(&offset, &DEFAULT_OFFSET);

        let message_objects = vector[];
        for (i in offset..math64::min(vector::length(&messages), offset + limit)) {
            vector::push_back(&mut message_objects, *vector::borrow(&messages, i));
        };

        message_objects
    }

    #[view]
    /// Get the content of a message
    public fun get_message_content(message_object: Object<Message>)
        : (
        bool,
        String,
        u64,
        address,
        Object<ObjectCore>,
        vector<String>,
        Option<bool>,
        Option<String>,
        Option<u64>,
        Option<address>,
        Option<Object<ObjectCore>>,
        Option<vector<String>>,
    ) acquires Message {
        let message = borrow_global<Message>(object::object_address(&message_object));
        (
            message.boolean_content,
            message.string_content,
            message.number_content,
            message.address_content,
            message.object_content,
            message.vector_content,
            message.optional_boolean_content,
            message.optional_string_content,
            message.optional_number_content,
            message.optional_address_content,
            message.optional_object_content,
            message.optional_vector_content,
        )
    }

    // ======================== Helper functions ========================

    fun get_message_board_object_address(): address {
        object::create_object_address(&@message_board_addr, MESSAGE_BOARD_OBJECT_SEED)
    }

    // ================================= Uint Tests Helper ================================== //

    #[test_only]
    public fun init_module_for_test(
        sender: &signer
    ) {
        init_module(sender);
    }
}
