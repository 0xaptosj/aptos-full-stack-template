script {
    use std::string;

    use aptos_framework::object;

    use message_board_addr::message_board;

    // This Move script runs atomically
    fun update_message(sender: &signer) {
        let message_obj_addr = @0xfcf360032ac5f5e3771afe5b669d8d04c99c5e69b58e531c903388e93968ab94;
        message_board::update_message(sender, object::address_to_object(message_obj_addr), string::utf8(b"updated message"));
    }
}
