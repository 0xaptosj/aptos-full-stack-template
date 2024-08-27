script {
    use std::string;

    use aptos_framework::object;

    use message_board_addr::message_board;

    // This Move script runs atomically
    fun update_message(sender: &signer) {
        let message_obj_addr = @0xcdbbb6c1c7a054ff876b0478f20e0585edabf2cec28ea8376287727dbbfaa79e;
        message_board::update_message(sender, object::address_to_object(message_obj_addr), string::utf8(b"updated message"));
    }
}
