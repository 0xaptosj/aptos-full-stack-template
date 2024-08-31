script {
    use std::string;

    use aptos_framework::object;

    use message_board_addr::message_board;

    // This Move script runs atomically
    fun update_message(sender: &signer) {
        let message_obj_addr_1 = @0x378cb22ce6481072489c693d1cd713554d7ecee5381f642aa427d9f49977bc34;
        message_board::update_message(sender, object::address_to_object(message_obj_addr_1), string::utf8(b"updated message 3"));

        let message_obj_addr_2 = @0xc2309cb1fbe604485323cb6d799aeecd5a347d290af13fee68eacb6be0007036;
        message_board::update_message(sender, object::address_to_object(message_obj_addr_2), string::utf8(b"updated message 4"));
        message_board::update_message(sender, object::address_to_object(message_obj_addr_2), string::utf8(b"updated message 5"));

    }
}
