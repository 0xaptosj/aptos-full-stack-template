#[test_only]
module message_board_addr::test_end_to_end {
    use std::option;
    use std::signer;
    use std::string;
    use std::vector;

    use aptos_std::debug;
    use aptos_std::string_utils;

    use aptos_framework::object::{Self, ObjectCore};

    use message_board_addr::message_board;

    #[test(sender = @message_board_addr)]
    fun test_end_to_end<>(sender: &signer) {
        let sende_addr = signer::address_of(sender);

        message_board::init_module_for_test(sender);

        let obj_constructor_ref = &object::create_object(sende_addr);
        let obj = object::object_from_constructor_ref<ObjectCore>(obj_constructor_ref);

        message_board::post_message(
            sender,
            true,
            string::utf8(b"hello world"),
            42,
            @0x1,
            obj,
            vector[string::utf8(b"hello")],
            option::some(true),
            option::some(string::utf8(b"hello")),
            option::some(42),
            option::some(@0x1),
            option::some(obj),
            option::some(vector[string::utf8(b"hello")]),
        );

        let message_objects = message_board::get_message_objects(option::none(), option::none());
        debug::print(&string_utils::format1(&b"message_objects = {}", message_objects));

        let (
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
        ) = message_board::get_message_content(*vector::borrow(&message_objects, 0));

        assert!(boolean_content == true, 2);
        assert!(string_content == string::utf8(b"hello world"), 3);
        assert!(number_content == 42, 4);
        assert!(address_content == @0x1, 5);
        assert!(object_content == obj, 6);
        assert!(vector_content == vector[string::utf8(b"hello")], 7);
        assert!(optional_boolean_content == option::some(true), 8);
        assert!(optional_string_content == option::some(string::utf8(b"hello")), 9);
        assert!(optional_number_content == option::some(42), 10);
        assert!(optional_address_content == option::some(@0x1), 11);
        assert!(optional_object_content == option::some(obj), 12);
        assert!(
            optional_vector_content == option::some(vector[string::utf8(b"hello")]),
            13,
        );

        // Post again
        message_board::post_message(
            sender,
            true,
            string::utf8(b"hello aptos"),
            42,
            @0x1,
            obj,
            vector[string::utf8(b"yoho")],
            option::some(false),
            option::some(string::utf8(b"hello")),
            option::some(40),
            option::some(@0x2),
            option::some(obj),
            option::some(vector[string::utf8(b"hello")]),
        );

        let message_objects = message_board::get_message_objects(option::none(), option::none());
        debug::print(&string_utils::format1(&b"message_objects = {}", message_objects));

        let (
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
        ) = message_board::get_message_content(*vector::borrow(&message_objects, 1));

        assert!(boolean_content == true, 15);
        assert!(string_content == string::utf8(b"hello aptos"), 16);
        assert!(number_content == 42, 17);
        assert!(address_content == @0x1, 18);
        assert!(object_content == obj, 19);
        assert!(vector_content == vector[string::utf8(b"yoho")], 20);
        assert!(optional_boolean_content == option::some(false), 21);
        assert!(optional_string_content == option::some(string::utf8(b"hello")), 22);
        assert!(optional_number_content == option::some(40), 23);
        assert!(optional_address_content == option::some(@0x2), 24);
        assert!(optional_object_content == option::some(obj), 25);
        assert!(
            optional_vector_content == option::some(vector[string::utf8(b"hello")]),
            26,
        );
    }
}
