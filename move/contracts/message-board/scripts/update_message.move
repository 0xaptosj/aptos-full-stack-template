script {
    use std::string;

    use aptos_framework::object;

    use message_board_addr::message_board;

    // This Move script runs atomically
    fun update_message(sender: &signer) {
        let message_obj_addr_1 =
            @0x54656506630d8eccfd6cc3b594d5cad4c0179ca0311b3aeafd913b88d61c4a4c;
        message_board::update_message(
            sender,
            object::address_to_object(message_obj_addr_1),
            string::utf8(b"updated message 3")
        );

        let message_obj_addr_2 =
            @0x1de5719525732b482a0062d5054995329c7ae094f6502630deec187d5761314f;
        message_board::update_message(
            sender,
            object::address_to_object(message_obj_addr_2),
            string::utf8(b"updated message 4")
        );
        message_board::update_message(
            sender,
            object::address_to_object(message_obj_addr_2),
            string::utf8(b"updated message 5")
        );

    }
}
