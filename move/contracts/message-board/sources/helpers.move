module message_board_addr::helpers {
    use std::signer;

    use aptos_framework::coin;
    use aptos_framework::aptos_coin::AptosCoin;
    use aptos_framework::primary_fungible_store;

    public fun convert_coin_apt_to_fa_apt(sender: &signer, amount: u64) {
        let apt = coin::withdraw<AptosCoin>(sender, amount);
        let fa_apt = coin::coin_to_fungible_asset(apt);
        primary_fungible_store::deposit(signer::address_of(sender), fa_apt);
    }
}

