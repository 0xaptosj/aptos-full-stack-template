module message_board_addr::message_board {
    use std::signer;
    use std::string::String;

    use aptos_framework::event;
    use aptos_framework::object::{Self, Object};
    use aptos_framework::timestamp;

    struct Message has copy, drop, key, store {
        creator: address,
        creation_timestamp: u64,
        content: String,
    }

    #[event]
    struct PostMessageEvent has drop, store {
        message_obj: Object<Message>,
        message: Message,
    }

    // This function is only called once when the module is published for the first time.
    // init_module is optional, you can also have an entry function as the initializer.
    fun init_module(_sender: &signer) {}

    // ======================== Write functions ========================

    /// Post a message
    public entry fun post_message(sender: &signer, content: String) {
        let message_obj_constructor_ref = &object::create_object(@message_board_addr);
        let message_obj_signer = &object::generate_signer(message_obj_constructor_ref);
        let message = Message {
            creator: signer::address_of(sender),
            creation_timestamp: timestamp::now_seconds(),
            content,
        };
        move_to(message_obj_signer, message);

        event::emit(PostMessageEvent {
            message_obj: object::object_from_constructor_ref(message_obj_constructor_ref),
            message,
        });
    }

    // ======================== Read Functions ========================

    #[view]
    /// Get the content of a message
    public fun get_message_content(message_object: Object<Message>): (String, address, u64) acquires Message {
        let message = borrow_global<Message>(object::object_address(&message_object));
        (
            message.content,
            message.creator,
            message.creation_timestamp,
        )
    }

    // ================================= Uint Tests Helper ================================== //

    #[test_only]
    public fun init_module_for_test(aptos_framework: &signer, sender: &signer) {
        init_module(sender);
        timestamp::set_time_has_started_for_testing(aptos_framework);
    }
}
