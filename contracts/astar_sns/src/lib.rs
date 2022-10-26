#![cfg_attr(not(feature = "std"), no_std)]

mod follow;
mod message;
mod metadata;
mod post;
mod profile;

use ink_lang as ink;

#[ink::contract]
mod astar_sns {
    use ink_env::debug_println;
    use ink_lang::codegen::Env;
    use ink_prelude::vec::Vec;
    use openbrush::storage::Mapping;
    use openbrush::test_utils::accounts;

    pub use crate::follow::*;
    pub use crate::message::*;
    pub use crate::metadata::*;
    pub use crate::post::*;

    #[ink(storage)]
    pub struct AstarSns {
        pub profile_map: Mapping<AccountId, Profile>,
        pub post_map: Mapping<u128, Post>,
        pub post_map_counter: u128,
        pub message_list_map: Mapping<u128, Vec<Message>>,
        pub message_list_map_counter: u128,
    }

    impl AstarSns {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                profile_map: Mapping::default(),
                post_map: Mapping::default(),
                post_map_counter: 0,
                message_list_map: Mapping::default(),
                message_list_map_counter: 0,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }
        // This is function for avoid error because at least a function with #[ink(message)] is needed
        #[ink(message)]
        pub fn debug(&self) {
            debug_println!("Hello World!");
        }
    }

    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_env::debug_println;

        use ink_lang as ink;

        // // profile creation and setting function
        // #[ink::test]
        // fn test_profile_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     //make new account
        //     let alice_account_id = accounts().alice;

        //     // instantiate profile
        //     astar_sns.create_profile(alice_account_id);

        //     // set profile info
        //     astar_sns.set_profile_info(
        //         alice_account_id,
        //         "Tony Stark".to_string(),
        //         "https//ff...".to_string(),
        //     );

        //     // get profile info and display on terminal
        //     debug_println!(
        //         "profile_list: {:?}",
        //         astar_sns.profile_map.get(&alice_account_id).unwrap()
        //     );
        // }

        // // test for release post function
        // #[ink::test]
        // fn test_post_release_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     //make new account
        //     let alice_account_id = accounts().alice;

        //     // instantiate profile
        //     astar_sns.create_profile(alice_account_id.clone());

        //     // release post
        //     astar_sns.release_post(
        //         alice_account_id,
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );

        //     // set profile
        //     astar_sns.set_profile_info(
        //         alice_account_id,
        //         "Alice".to_string(),
        //         "https//ff...".to_string(),
        //     );

        //     // get post and display
        //     let post_info: Post = astar_sns
        //         .post_map
        //         .get(&(astar_sns.post_map_counter - 1))
        //         .unwrap();
        //     debug_println!("post :{:?}\n", post_info);
        // }

        // // test for get general post
        // #[ink::test]
        // fn test_general_post_get_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     // make new accounts
        //     let alice_account_id = accounts().alice;
        //     let bob_account_id = accounts().bob;
        //     let charlie_account_id = accounts().charlie;

        //     // instantiate profiles
        //     astar_sns.create_profile(alice_account_id.clone());
        //     astar_sns.create_profile(bob_account_id.clone());
        //     astar_sns.create_profile(charlie_account_id.clone());

        //     // set profile
        //     astar_sns.set_profile_info(
        //         alice_account_id,
        //         "Alice".to_string(),
        //         "https//ff...".to_string(),
        //     );

        //     // release posts
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "I come to Thailand".to_string(),
        //         "12:35".to_string(),
        //         "https://gsdef...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Hello YouTube".to_string(),
        //         "12:40".to_string(),
        //         "https://fafds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Oh baby, come on!".to_string(),
        //         "12:45".to_string(),
        //         "https://fsdfee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Don't mention it!".to_string(),
        //         "12:50".to_string(),
        //         "https://fasee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Do what you want".to_string(),
        //         "12:55".to_string(),
        //         "https://fasdfgeg...".to_string(),
        //     );

        //     //get posts and display on terminal
        //     let post_list: Vec<Post> = astar_sns.get_general_post(1);
        //     debug_println!("General post get test\n",);
        //     for n in 0..(post_list.len()) {
        //         debug_println!("{:?}\n", post_list[n]);
        //     }
        // }

        // #[ink::test]
        // fn test_individual_post_get_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     //make new accounts
        //     let alice_account_id = accounts().alice;
        //     let bob_account_id = accounts().bob;
        //     let charlie_account_id = accounts().charlie;

        //     // instantiate profiles
        //     astar_sns.create_profile(alice_account_id.clone());
        //     astar_sns.create_profile(bob_account_id.clone());
        //     astar_sns.create_profile(charlie_account_id.clone());

        //     // set profiles
        //     astar_sns.set_profile_info(
        //         alice_account_id,
        //         "Alice".to_string(),
        //         "https//ff...".to_string(),
        //     );
        //     astar_sns.set_profile_info(
        //         bob_account_id,
        //         "Bob".to_string(),
        //         "https//ff...".to_string(),
        //     );
        //     astar_sns.set_profile_info(
        //         charlie_account_id,
        //         "Charlie".to_string(),
        //         "https//ff...".to_string(),
        //     );

        //     // release post by some accounts
        //     astar_sns.release_post(
        //         alice_account_id,
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         bob_account_id,
        //         "I come to Thailand".to_string(),
        //         "12:35".to_string(),
        //         "https://gsdef...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         charlie_account_id,
        //         "Hello YouTube".to_string(),
        //         "12:40".to_string(),
        //         "https://fafds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         alice_account_id,
        //         "Oh baby, come on!".to_string(),
        //         "12:45".to_string(),
        //         "https://fsdfee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         bob_account_id,
        //         "Don't mention it!".to_string(),
        //         "12:50".to_string(),
        //         "https://fasee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         charlie_account_id,
        //         "Do what you want".to_string(),
        //         "12:55".to_string(),
        //         "https://fasdfgeg...".to_string(),
        //     );

        //     // get posts individually and display on terminal
        //     let alice_post_list: Vec<Post> = astar_sns.get_individual_post(1, alice_account_id);
        //     let bob_post_list: Vec<Post> = astar_sns.get_individual_post(1, bob_account_id);
        //     let charlie_post_list: Vec<Post> = astar_sns.get_individual_post(1, charlie_account_id);
        //     debug_println!("Individual post get test");
        //     for n in 0..(alice_post_list.len()) {
        //         debug_println!("{:?}", alice_post_list[n]);
        //     }
        //     for n in 0..(bob_post_list.len()) {
        //         debug_println!("{:?}", bob_post_list[n]);
        //     }
        //     for n in 0..(charlie_post_list.len()) {
        //         debug_println!("{:?}", charlie_post_list[n]);
        //     }
        // }

        // #[ink::test]
        // fn test_add_likes_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     //make new account
        //     let alice_account_id = accounts().alice;

        //     // instantiate profile
        //     astar_sns.create_profile(alice_account_id.clone());

        //     // release post
        //     astar_sns.release_post(
        //         alice_account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );

        //     // add like to specified post and display number of likes
        //     astar_sns.add_likes(0);
        //     debug_println!(
        //         "Number of likes: {}",
        //         astar_sns.post_map.get(&0).unwrap().num_of_likes
        //     );
        //     astar_sns.add_likes(0);
        //     debug_println!(
        //         "Number of likes: {}",
        //         astar_sns.post_map.get(&0).unwrap().num_of_likes
        //     );
        // }

        // #[ink::test]
        // fn test_follow_fn_works() {
        //     // instantiate contract
        //     let mut astar_sns = AstarSns::new();

        //     //make new accounts
        //     let alice_account_id = accounts().alice;
        //     let bob_account_id = accounts().bob;
        //     let charlie_account_id = accounts().charlie;

        //     // instantiate profiles
        //     astar_sns.create_profile(alice_account_id.clone());
        //     astar_sns.create_profile(bob_account_id.clone());
        //     astar_sns.create_profile(charlie_account_id.clone());

        //     // display message_list_map_counter
        //     debug_println!(
        //         "message_list_map_counter: {}",
        //         astar_sns.message_list_map_counter
        //     );

        //     // implement follow function
        //     astar_sns.follow(alice_account_id, bob_account_id);
        //     debug_println!(
        //         "following_list: {:?}\nfollower_list: {:?}",
        //         astar_sns.get_following_list(alice_account_id),
        //         astar_sns.get_follower_list(bob_account_id)
        //     );
        //     debug_println!(
        //         "message_list_map_id: {}",
        //         astar_sns.message_list_map_counter
        //     );
        //     astar_sns.follow(bob_account_id, alice_account_id);
        //     debug_println!(
        //         "message_list_map_counter: {}",
        //         astar_sns.message_list_map_counter
        //     );
        //     astar_sns.follow(alice_account_id, charlie_account_id);
        //     debug_println!(
        //         "following_list: {:?}\nfollower_list: {:?}",
        //         astar_sns.get_following_list(alice_account_id),
        //         astar_sns.get_follower_list(charlie_account_id)
        //     );
        //     debug_println!(
        //         "message_list_map_counter: {}",
        //         astar_sns.message_list_map_counter
        //     );
        // }

        #[ink::test]
        fn test_message_fn_works() {
            // instantiate contract
            let mut astar_sns = AstarSns::new();

            //make new accounts
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;

            // instantiate profiles
            astar_sns.create_profile(alice_account_id.clone());
            astar_sns.create_profile(bob_account_id.clone());
            astar_sns.create_profile(charlie_account_id.clone());

            // implement follow function to make message room
            astar_sns.follow(alice_account_id, bob_account_id);
            astar_sns.follow(alice_account_id, charlie_account_id);

            // send messages
            astar_sns.send_message(
                "Sorry Bro, I can't go today.".to_string(),
                0,
                alice_account_id,
                "12:30".to_string(),
            );
            astar_sns.send_message(
                "Why don't we go there tomorrow?".to_string(),
                0,
                alice_account_id,
                "12:33".to_string(),
            );
            astar_sns.send_message(
                "Hey, charlie will come!".to_string(),
                0,
                alice_account_id,
                "12:35".to_string(),
            );
            astar_sns.send_message(
                "He seems so muscular, so he would teach us.".to_string(),
                0,
                alice_account_id,
                "12:36".to_string(),
            );
            astar_sns.send_message(
                "Why don't we go there tomorrow?".to_string(),
                0,
                alice_account_id,
                "12:37".to_string(),
            );
            astar_sns.send_message(
                "I'm so looking forward that!".to_string(),
                0,
                bob_account_id,
                "12:38".to_string(),
            );
            astar_sns.send_message(
                "Hey bro! Tomorrow I and Bob go to gym. Don't you join us?".to_string(),
                1,
                alice_account_id,
                "12:34".to_string(),
            );

            // display messages and last message
            debug_println!(
                "message_list_alice_bob: {:?}\n",
                astar_sns.get_message_list(0, 1)
            );
            debug_println!(
                "last_message_alice_bob: {:?}\n",
                astar_sns.get_last_message(0)
            );
            debug_println!(
                "message_list_alice_charlie: {:?}",
                astar_sns.get_message_list(1, 1)
            );
        }
    }
}
