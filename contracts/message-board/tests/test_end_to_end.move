#[test_only]
module message_board_addr::test_end_to_end {
    use std::string;

    use message_board_addr::message_board;

    #[test(aptos_framework = @aptos_framework, sender = @message_board_addr)]
    fun test_end_to_end(aptos_framework: &signer, sender: &signer) {
        message_board::init_module_for_test(aptos_framework, sender);

        message_board::post_message(
            sender,
            string::utf8(b"hello world"),
        );
    }
}
