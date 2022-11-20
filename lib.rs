#![cfg_attr(not(feature = "std"), no_std)]

mod FT;
mod follow;
mod message;
mod metadata;
mod post;
mod profile;

use ink_lang as ink;

#[ink::contract]
mod astar_socialfi {

    use ink_env::debug_println;
    use ink_lang::codegen::Env;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use openbrush::storage::Mapping;

    // use openbrush::test_utils::accounts; // uncomment for testing

    pub use crate::follow::*;
    pub use crate::message::*;
    pub use crate::metadata::*;
    pub use crate::post::*;

    #[ink(storage)]
    pub struct AstarSocialfi {
        pub profile_map: Mapping<AccountId, Profile>,
        pub post_map: Mapping<u128, Post>,
        pub post_map_counter: u128,
        pub message_list_map: Mapping<u128, Vec<Message>>,
        pub message_list_map_counter: u128,
        pub asset_mapping: Mapping<AccountId, u128>,
    }

    impl AstarSocialfi {
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            Self {
                profile_map: Mapping::default(),
                post_map: Mapping::default(),
                post_map_counter: 0,
                message_list_map: Mapping::default(),
                message_list_map_counter: 0,
                asset_mapping: Mapping::default(),
            }
        }

        // post function
        #[ink(message)]
        pub fn release_post(
            &mut self,
            description: String,
            created_time: String,
            post_img_url: String,
        ) {
            let caller: AccountId = self.env().caller();
            self.release_post_fn(caller, description, created_time, post_img_url);
        }

        // get all posts function
        #[ink(message)]
        pub fn get_all_posts(&self, num: u128) -> Vec<Post> {
            let general_post_list: Vec<Post> = self.get_all_posts_fn(num);
            general_post_list
        }

        // get post from specific user
        #[ink(message)]
        pub fn get_user_posts(&self, num: u128, account_id: AccountId) -> Vec<Post> {
            let user_posts_list: Vec<Post> = self.get_user_posts_fn(num, account_id);
            user_posts_list
        }

        // add likes function
        #[ink(message)]
        pub fn add_likes(&mut self, post_id: u128) {
            self.add_likes_fn(post_id);
        }

        // send message function
        #[ink(message)]
        pub fn send_message(
            &mut self,
            message: String,
            message_list_id: u128,
            created_time: String,
        ) {
            let caller: AccountId = self.env().caller();
            self.send_message_fn(message, message_list_id, caller, created_time);
        }

        // get message list from id
        #[ink(message)]
        pub fn get_message_list(&self, message_list_id: u128, num: u128) -> Vec<Message> {
            let message_list: Vec<Message> =
                self.get_message_list_fn(message_list_id, num as usize);
            message_list
        }

        // get last message from id
        #[ink(message)]
        pub fn get_last_message(&self, message_list_id: u128) -> Message {
            let message: Message = self.get_last_message_fn(message_list_id);
            message
        }

        // create profile function
        #[ink(message)]
        pub fn create_profile(&mut self) {
            let caller: AccountId = self.env().caller();
            self.create_profile_fn(caller);
        }

        // set profile info function
        #[ink(message)]
        pub fn set_profile_info(&mut self, name: String, img_url: String) {
            let caller: AccountId = self.env().caller();
            self.set_profile_info_fn(caller, name, img_url);
        }

        // follow function
        #[ink(message)]
        pub fn follow(&mut self, followed_account_id: AccountId) {
            let caller: AccountId = self.env().caller();
            self.follow_fn(caller, followed_account_id);
        }

        // get following list function
        #[ink(message)]
        pub fn get_following_list(&self, account_id: AccountId) -> Vec<AccountId> {
            let following_list: Vec<AccountId> = self.get_following_list_fn(account_id);
            following_list
        }

        // get follower list function
        #[ink(message)]
        pub fn get_follower_list(&self, account_id: AccountId) -> Vec<AccountId> {
            let follower_list: Vec<AccountId> = self.get_follower_list_fn(account_id);
            follower_list
        }

        // get profile info function
        #[ink(message)]
        pub fn get_profile_info(&self, account_id: AccountId) -> Profile {
            let profile: Profile = self.get_profile_info_fn(account_id);
            profile
        }

        // check if profile is already created
        #[ink(message)]
        pub fn check_created_info(&self, account_id: AccountId) -> bool {
            let is_already_connected: bool = self.check_created_profile_fn(account_id);
            is_already_connected
        }

        // get total likes a user received
        #[ink(message)]
        pub fn get_total_likes(&self, account_id: AccountId) -> u128 {
            let total_likes: u128 = self.get_total_likes_fn(account_id);
            total_likes
        }

        // get token balance of a user
        #[ink(message)]
        pub fn balance_of(&self, account_id: AccountId) -> u128 {
            let balance: u128 = self.balance_of_fn(account_id);
            balance
        }

        // transfer token function
        #[ink(message)]
        pub fn transfer(&mut self, to_id: AccountId, amount: u128) {
            let caller: AccountId = self.env().caller();
            self.transfer_fn(caller, to_id, amount);
        }

        // distribute token according to likes
        #[ink(message)]
        pub fn distribute_refer_likes(&mut self) {
            let caller: AccountId = self.env().caller();
            let total_likes: u128 = self.get_total_likes_fn(caller);
            let balance: u128 = self.balance_of_fn(caller);
            let amount: u128 = 20 * total_likes;
            if balance < amount {
                self.distribute_fn(caller, amount - balance);
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::debug_println;
        use ink_lang as ink;

        fn init_contract() -> (AstarSocialfi, AccountId, AccountId, AccountId) {
            // init contract
            let mut socialfi_contract: AstarSocialfi = AstarSocialfi::new();

            // get new account
            let alice_account_id: AccountId = accounts().alice;
            let bob_account_id: AccountId = accounts().bob;
            let charlie_account_id: AccountId = accounts().charlie;

            // create profile
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            socialfi_contract.create_profile();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            socialfi_contract.create_profile();

            // set profile info
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract
                .set_profile_info("Alice".to_string(), "https://alice.com".to_string());
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            socialfi_contract
                .set_profile_info("Bob".to_string(), "https://www.bobmarley.com".to_string());
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            socialfi_contract
                .set_profile_info("Charlie".to_string(), "https://charlie.com".to_string());

            return (
                socialfi_contract,
                alice_account_id,
                bob_account_id,
                charlie_account_id,
            );
        }

        // test profile creation function
        #[ink::test]
        fn test_profile_fn() {
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let alice_account_id: AccountId = init_obj.1;

            debug_println!(
                "profile_list: {:?}",
                socialfi_contract
                    .profile_map
                    .get(&alice_account_id)
                    .unwrap()
            );
        }

        // test post function
        #[ink::test]
        fn test_post_relase_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;

            // make post
            socialfi_contract.release_post(
                "Hello World from Alice!".to_string(),
                "12:30".to_string(),
                "https://alice.com".to_string(),
            );

            // check post details
            let post_info: Post = socialfi_contract
                .post_map
                .get(&(socialfi_contract.post_map_counter - 1))
                .unwrap();
            debug_println!("post :{:?}\n", post_info);
        }

        // test get all posts function
        #[ink::test]
        fn test_get_all_posts_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;

            // make posts
            socialfi_contract.release_post(
                "Hello World from Alice!".to_string(),
                "12:30".to_string(),
                "https://alice.com".to_string(),
            );
            socialfi_contract.release_post(
                "Today was very rainy.".to_string(),
                "12:35".to_string(),
                "https://example.org".to_string(),
            );
            socialfi_contract.release_post(
                "I love Astar Network.".to_string(),
                "12:40".to_string(),
                "https://example.org".to_string(),
            );
            socialfi_contract.release_post(
                "Carpe Diem.".to_string(),
                "12:45".to_string(),
                "https://example.org".to_string(),
            );

            // check post list
            let post_list: Vec<Post> = socialfi_contract.get_all_posts_fn(1);
            debug_println!("Get all posts test\n",);
            for n in 0..(post_list.len()) {
                debug_println!("{:?}\n", post_list[n]);
            }
        }

        // test get user posts function
        #[ink::test]
        fn test_get_user_posts_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let (alice_account_id, bob_account_id, charlie_account_id): (
                AccountId,
                AccountId,
                AccountId,
            ) = (init_obj.1, init_obj.2, init_obj.3);

            // post from multiple accounts
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.release_post(
                "Hello World from Alice!".to_string(),
                "12:30".to_string(),
                "https://alice.com".to_string(),
            );
            socialfi_contract.release_post(
                "I love Astar Network.".to_string(),
                "12:40".to_string(),
                "https://example.org".to_string(),
            );

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            socialfi_contract.release_post(
                "Today was very rainy.".to_string(),
                "12:35".to_string(),
                "https://example.org".to_string(),
            );
            socialfi_contract.release_post(
                "I love Polygon.".to_string(),
                "12:42".to_string(),
                "https://example.org".to_string(),
            );

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            socialfi_contract.release_post(
                "Carpe Diem.".to_string(),
                "12:45".to_string(),
                "https://example.org".to_string(),
            );
            socialfi_contract.release_post(
                "Take a look at my chocolate factory!".to_string(),
                "12:50".to_string(),
                "https://example.org".to_string(),
            );

            let alice_post_list: Vec<Post> =
                socialfi_contract.get_user_posts_fn(1, alice_account_id);
            let bob_post_list: Vec<Post> = socialfi_contract.get_user_posts_fn(1, bob_account_id);
            let charlie_post_list: Vec<Post> =
                socialfi_contract.get_user_posts_fn(1, charlie_account_id);

            debug_println!("Get individual user posts test\n",);
            for n in 0..(alice_post_list.len()) {
                debug_println!("{:?}", alice_post_list[n]);
            }
            for n in 0..(bob_post_list.len()) {
                debug_println!("{:?}", bob_post_list[n]);
            }
            for n in 0..(charlie_post_list.len()) {
                debug_println!("{:?}", charlie_post_list[n]);
            }
        }

        // test like function
        #[ink::test]
        fn test_add_likes_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let (alice_account_id, bob_account_id, charlie_account_id): (
                AccountId,
                AccountId,
                AccountId,
            ) = (init_obj.1, init_obj.2, init_obj.3);

            // post
            socialfi_contract.release_post(
                "Hello World from Alice!".to_string(),
                "12:30".to_string(),
                "https://alice.com".to_string(),
            );
            socialfi_contract.release_post(
                "Today was very rainy.".to_string(),
                "12:35".to_string(),
                "https://example.org".to_string(),
            );
            socialfi_contract.release_post(
                "I love Astar Network.".to_string(),
                "12:40".to_string(),
                "https://example.org".to_string(),
            );

            // like
            socialfi_contract.add_likes(0);
            debug_println!(
                "Number of likes: {}",
                socialfi_contract.post_map.get(&0).unwrap().total_likes
            );
            socialfi_contract.add_likes(0);
            socialfi_contract.add_likes(1);
            socialfi_contract.add_likes(2);
            debug_println!(
                "Number of likes: {}",
                socialfi_contract.post_map.get(&0).unwrap().total_likes
            );

            let total_likes = socialfi_contract.get_total_likes(alice_account_id);
            debug_println!("Total likes to Alice: {}", total_likes);
        }

        // test follow function
        #[ink::test]
        fn test_follow_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let (alice_account_id, bob_account_id, charlie_account_id): (
                AccountId,
                AccountId,
                AccountId,
            ) = (init_obj.1, init_obj.2, init_obj.3);

            // check message list id counter
            debug_println!(
                "message_list_map_counter: {}",
                socialfi_contract.message_list_map_counter
            );

            // test follow function
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.follow(bob_account_id);
            debug_println!(
                "following_list: {:?}\nfollower_list: {:?}",
                socialfi_contract.get_following_list(alice_account_id),
                socialfi_contract.get_follower_list(bob_account_id)
            );

            // check message list id counter
            debug_println!(
                "message_list_map_id: {}",
                socialfi_contract.message_list_map_counter
            );

            // check follow function (bob -> alice)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            socialfi_contract.follow(alice_account_id);
            debug_println!(
                "message_list_map_counter: {}",
                socialfi_contract.message_list_map_counter
            );

            // check follow function (alice -> bob)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.follow(bob_account_id);
            debug_println!(
                "message_list_map_counter: {}",
                socialfi_contract.message_list_map_counter
            );

            // check follow function (alice -> cherlie)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.follow(charlie_account_id);
            debug_println!(
                "following_list: {:?}\nfollower_list {:?}",
                socialfi_contract.get_following_list(alice_account_id),
                socialfi_contract.get_follower_list(charlie_account_id)
            );
            debug_println!(
                "message_list_map_counter: {}",
                socialfi_contract.message_list_map_counter
            );
        }

        // test message function
        #[ink::test]
        fn test_message_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let (alice_account_id, bob_account_id, charlie_account_id): (
                AccountId,
                AccountId,
                AccountId,
            ) = (init_obj.1, init_obj.2, init_obj.3);

            // follow accounts to create message list
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.follow(bob_account_id);
            socialfi_contract.follow(charlie_account_id);

            // send message
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.send_message(
                "Sorry bro, I can't make it today.".to_string(),
                0,
                "12:30".to_string(),
            );
            socialfi_contract.send_message(
                "Let's go tomorrow instead.".to_string(),
                0,
                "12:33".to_string(),
            );
            socialfi_contract.send_message(
                "Let's also ask Charlie if he wants to join.".to_string(),
                0,
                "12:35".to_string(),
            );

            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(bob_account_id);
            socialfi_contract.send_message("Good idea.".to_string(), 0, "12:38".to_string());
            socialfi_contract.send_message(
                "Hey bro, Alice and I are gonna hit the gym tomorrow. Wanna join?".to_string(),
                1,
                "12:40".to_string(),
            );

            debug_println!(
                "message_list_alice_bob: {:?}\n",
                socialfi_contract.get_message_list(0, 1)
            );
            debug_println!(
                "last_message_alice_bob: {:?}\n",
                socialfi_contract.get_last_message(0)
            );
            debug_println!(
                "message_list_alice_charlie: {:?}\n",
                socialfi_contract.get_message_list(1, 1)
            );
        }

        #[ink::test]
        fn test_FT_fn() {
            // init contract
            let init_obj: (AstarSocialfi, AccountId, AccountId, AccountId) = init_contract();
            let mut socialfi_contract: AstarSocialfi = init_obj.0;
            let (alice_account_id, bob_account_id, charlie_account_id): (
                AccountId,
                AccountId,
                AccountId,
            ) = (init_obj.1, init_obj.2, init_obj.3);

            // distribute 100 tokens to alice bob charlie
            socialfi_contract.distribute_fn(alice_account_id, 100);
            socialfi_contract.distribute_fn(bob_account_id, 100);
            socialfi_contract.distribute_fn(charlie_account_id, 100);

            // alice and charlie sends bob 50
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(alice_account_id);
            socialfi_contract.transfer_fn(alice_account_id, bob_account_id, 50);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(charlie_account_id);
            socialfi_contract.transfer_fn(charlie_account_id, bob_account_id, 50);

            // check balance
            let alice_balance: u128 = socialfi_contract.balance_of(alice_account_id);
            let bob_balance: u128 = socialfi_contract.balance_of(bob_account_id);
            let charlie_balance: u128 = socialfi_contract.balance_of(charlie_account_id);
            debug_println!(
                "alice_balance: {}\nbob_balance: {}\ncharlie_balance: {}",
                alice_balance,
                bob_balance,
                charlie_balance
            );
        }
    }
}
