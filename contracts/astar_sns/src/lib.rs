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
        ////Test
        talk: TalkTest,
        value: bool,
        message_room: Option<MessageRoomTest>,
        post_list: Option<Vec<PostTest>>,
        pub map: Mapping<String, u128>,
        // pub map_2: HashMap<u128, u128>,
        ////Test
        pub profile_map: Mapping<AccountId, Profile>,
        pub post_map: Mapping<u128, Post>,
        pub post_map_counter: u128,
        pub message_list_map: Mapping<u128, Vec<Message>>,
        pub message_list_map_counter: u128,
    }

    impl AstarSns {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, message_1: String, message_2: String) -> Self {
            Self {
                value: init_value,
                talk: TalkTest {
                    message_1,
                    message_2,
                },
                message_room: None,
                post_list: None,
                map: Mapping::default(),
                profile_map: Mapping::default(),
                post_map: Mapping::default(),
                post_map_counter: 0,
                message_list_map: Mapping::default(),
                message_list_map_counter: 0,
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), Default::default(), Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_message_1(&self) {
            debug_println!("{} bro!", self.talk.message_1);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_env::debug_println;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let astar_sns = AstarSns::default();
        //     assert_eq!(astar_sns.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     assert_eq!(astar_sns.get(), false);
        //     astar_sns.flip();
        //     assert_eq!(astar_sns.get(), true);
        //     astar_sns.get_message_1();
        //     astar_sns.get_post_list();
        //     astar_sns.insert_info_to_map();
        //     debug_println!("{} bro!", astar_sns.talk.message_1);
        //     astar_sns.show_map();
        // }

        // // profile creation and setting function
        // #[ink::test]
        // fn test_profile_fn_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let account_id = AccountId::default();
        //     astar_sns.create_profile(account_id);
        //     astar_sns.set_profile_info(
        //         account_id,
        //         "Tony Stark".to_string(),
        //         "https//ff...".to_string(),
        //     );
        // }

        // test for release post function
        // #[ink::test]
        // fn test_post_release_fn_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let account_id = AccountId::default();
        //     astar_sns.create_profile(account_id.clone());
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
        //     astar_sns.set_profile_info(
        //         account_id,
        //         "Tony Stark".to_string(),
        //         "https//ff...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "I come to Thailand".to_string(),
        //         "12:35".to_string(),
        //         "https://gsdef...".to_string(),
        //     );
        //     let post_info: Post = astar_sns
        //         .post_map
        //         .get(&(astar_sns.post_map_counter - 1))
        //         .unwrap();
        //     debug_println!(
        //         "name :{}\nuser_id: {:?}\ncreated_time: {}\nimg_url: {},\ndescription: {}\n",
        //         &post_info.name,
        //         &post_info.user_id,
        //         &post_info.created_time,
        //         &post_info.img_url,
        //         &post_info.description
        //     );
        // }

        // test for get general post
        // #[ink::test]
        // fn test_general_post_get_fn_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let account_id = AccountId::default();
        //     astar_sns.create_profile(account_id.clone());
        //     astar_sns.set_profile_info(
        //         account_id,
        //         "Tony Stark".to_string(),
        //         "https//ff...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "I come to Thailand".to_string(),
        //         "12:35".to_string(),
        //         "https://gsdef...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Hello YouTube".to_string(),
        //         "12:40".to_string(),
        //         "https://fafds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Oh baby, come on!".to_string(),
        //         "12:45".to_string(),
        //         "https://fsdfee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Don't mention it!".to_string(),
        //         "12:50".to_string(),
        //         "https://fasee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Do what you want".to_string(),
        //         "12:55".to_string(),
        //         "https://fasdfgeg...".to_string(),
        //     );
        //     let post_list: Vec<Post> = astar_sns.get_general_post(1);
        //     debug_println!("General post get test\n",);
        //     for n in 0..(post_list.len()) {
        //         debug_println!("{:?}\n", post_list[n]);
        //     }

        //     let post_id_list: Vec<u128> =
        //         astar_sns.profile_map.get(&account_id).unwrap().post_id_list;
        //     for n in 0..(post_id_list.len()) {
        //         debug_println!("{:?}", post_id_list[n]);
        //     }
        // }

        // #[ink::test]
        // fn test_individual_post_get_fn_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let account_id = AccountId::default();
        //     astar_sns.create_profile(account_id.clone());
        //     astar_sns.set_profile_info(
        //         account_id,
        //         "Tony Stark".to_string(),
        //         "https//ff...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "I come to Thailand".to_string(),
        //         "12:35".to_string(),
        //         "https://gsdef...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Hello YouTube".to_string(),
        //         "12:40".to_string(),
        //         "https://fafds...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Oh baby, come on!".to_string(),
        //         "12:45".to_string(),
        //         "https://fsdfee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Don't mention it!".to_string(),
        //         "12:50".to_string(),
        //         "https://fasee...".to_string(),
        //     );
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Do what you want".to_string(),
        //         "12:55".to_string(),
        //         "https://fasdfgeg...".to_string(),
        //     );
        //     let post_list: Vec<Post> = astar_sns.get_individual_post(1, account_id);
        //     debug_println!("Individual post get test",);
        //     for n in 0..(post_list.len()) {
        //         debug_println!("{:?}", post_list[n]);
        //     }
        // }

        // #[ink::test]
        // fn test_add_likes_fn_works() {
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let account_id = AccountId::default();
        //     astar_sns.create_profile(account_id.clone());
        //     astar_sns.release_post(
        //         account_id.clone(),
        //         "Today, it was so rainy!".to_string(),
        //         "12:30".to_string(),
        //         "https://sdfds...".to_string(),
        //     );
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
        //     let mut astar_sns =
        //         AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
        //     let alice_account_id = accounts().alice;
        //     let bob_account_id = accounts().bob;
        //     let charlie_account_id = accounts().charlie;
        //     astar_sns.create_profile(alice_account_id.clone());
        //     astar_sns.create_profile(bob_account_id.clone());
        //     astar_sns.create_profile(charlie_account_id.clone());
        //     debug_println!(
        //         "message_list_map_id: {}",
        //         astar_sns.message_list_map_counter
        //     );
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
        //         "following_list: {:?}\nfollower_list: {:?}",
        //         astar_sns.get_following_list(alice_account_id),
        //         astar_sns.get_follower_list(bob_account_id)
        //     );
        //     debug_println!(
        //         "message_list_map_id: {}",
        //         astar_sns.message_list_map_counter
        //     );
        //     astar_sns.follow(alice_account_id, charlie_account_id);
        //     debug_println!(
        //         "following_list: {:?}\nfollower_list: {:?}",
        //         astar_sns.get_following_list(alice_account_id),
        //         astar_sns.get_follower_list(charlie_account_id)
        //     );
        //     debug_println!(
        //         "message_list_map_id: {}",
        //         astar_sns.message_list_map_counter
        //     );
        // }

        #[ink::test]
        fn test_message_fn_works() {
            let mut astar_sns =
                AstarSns::new(false, "Hello".to_string(), "nice to meet you".to_string());
            let alice_account_id = accounts().alice;
            let bob_account_id = accounts().bob;
            let charlie_account_id = accounts().charlie;
            astar_sns.create_profile(alice_account_id.clone());
            astar_sns.create_profile(bob_account_id.clone());
            astar_sns.create_profile(charlie_account_id.clone());
            astar_sns.follow(alice_account_id, bob_account_id);
            astar_sns.follow(alice_account_id, charlie_account_id);
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
