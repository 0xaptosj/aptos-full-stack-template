script {
    use std::string;

    use aptos_framework::object;

    use message_board_addr::message_board;

    // This Move script runs atomically
    fun update_message(sender: &signer) {
        let message_obj_addr = @0x391c863de19f1e7f0640e59ec88941c34ccabdd0809af4bad4b159a25dd3d93b;
        message_board::update_message(sender, object::address_to_object(message_obj_addr), string::utf8(b"updated message"));
    }
}
