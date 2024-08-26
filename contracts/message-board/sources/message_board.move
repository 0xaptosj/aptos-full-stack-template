module message_board_addr::message_board {
    use std::signer;
    use std::string::String;

    use aptos_framework::event;
    use aptos_framework::object::{Self, Object};
    use aptos_framework::timestamp;

    /// Only the message creator can update the message content
    const ERR_ONLY_MESSAGE_CREATOR_CAN_UPDATE: u64 = 1;

    struct Message has copy, drop, key, store {
        creator: address,
        creation_timestamp: u64,
        content: String,
    }

    #[event]
    struct CreateMessageEvent has drop, store {
        message_obj: Object<Message>,
        message: Message,
    }

    #[event]
    struct UpdateMessageEvent has drop, store {
        message_obj: Object<Message>,
        message: Message,
    }

    // This function is only called once when the module is published for the first time.
    // init_module is optional, you can also have an entry function as the initializer.
    fun init_module(_sender: &signer) {}

    // ======================== Write functions ========================

    /// Create a new message
    public entry fun craete_message(sender: &signer, content: String) {
        let message_obj_constructor_ref = &object::create_object(@message_board_addr);
        let message_obj_signer = &object::generate_signer(message_obj_constructor_ref);
        let message = Message {
            creator: signer::address_of(sender),
            creation_timestamp: timestamp::now_seconds(),
            content,
        };
        move_to(message_obj_signer, message);

        event::emit(CreateMessageEvent {
            message_obj: object::object_from_constructor_ref(message_obj_constructor_ref),
            message,
        });
    }

    /// Update the content of an existing message, only message creator can call
    public entry fun update_message(sender: &signer, message_obj: Object<Message>, new_content: String) acquires Message {
        let message = borrow_global_mut<Message>(object::object_address(&message_obj));
        assert!(message.creator == signer::address_of(sender), ERR_ONLY_MESSAGE_CREATOR_CAN_UPDATE);
        message.content = new_content;

        event::emit(UpdateMessageEvent {
            message_obj,
            message: *message,
        });
    }

    // ======================== Read Functions ========================

    #[view]
    /// Get the content of a message
    public fun get_message_content(message_obj: Object<Message>): (String, address, u64) acquires Message {
        let message = borrow_global<Message>(object::object_address(&message_obj));
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

    #[test_only]
    public fun get_message_obj_from_create_message_event<T: drop + store>(event: T): Object<Message> {
        (event as CreateMessageEvent).message_obj
    }

    #[test_only]
    public fun get_message_obj_from_update_message_event<T: drop + store>(event: T): Object<Message> {
        (event as UpdateMessageEvent).message_obj
    }
}
