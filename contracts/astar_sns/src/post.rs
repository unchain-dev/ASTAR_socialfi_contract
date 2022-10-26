use crate::metadata::*;
use ink_env::{debug_println, AccountId};
// use ink_lang as ink;
use ink_prelude::vec::Vec;

use crate::astar_sns::AstarSns;

impl AstarSns {
    ////Test
    pub fn get_post_list(&self) {
        let mut post_list: Vec<PostTest> = Vec::new();
        post_list.push(PostTest {
            description: "Sorry Bro".to_string(),
            img_url: "https//...".to_string(),
            num_of_likes: 3,
            created_time: "12:20".to_string(),
        });
        post_list.push(PostTest {
            description: "Fuck off bro".to_string(),
            img_url: "https//...".to_string(),
            num_of_likes: 12,
            created_time: "12:30".to_string(),
        });
        debug_println!("{}", post_list[0].description);
    }

    pub fn insert_info_to_map(&mut self) {
        self.map.insert(&("Yuji".to_string()), &22);
    }

    pub fn show_map(&self) {
        debug_println!("{}", self.map.get(&("Yuji".to_string())).unwrap());
    }
    ////Test

    pub fn release_post(
        &mut self,
        account_id: AccountId,
        description: String,
        created_time: String,
        post_img_url: String,
    ) {
        let profile_info: Profile = self.profile_map.get(&account_id).unwrap();
        self.post_map.insert(
            &(self.post_map_counter),
            &Post {
                name: profile_info.name.unwrap_or("unknown".to_string()),
                user_id: profile_info.user_id,
                created_time,
                img_url: post_img_url,
                description: description,
                num_of_likes: 0,
            },
        );

        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();
        profile.post_id_list.push(self.post_map_counter);
        self.profile_map.insert(&account_id, &profile);

        self.post_map_counter = &self.post_map_counter + 1;
    }

    pub fn get_general_post(&self, num: u128) -> Vec<Post> {
        let mut post_list: Vec<Post> = Vec::new();
        let length: u128 = self.post_map_counter;
        let amount_index: u128 = 5;

        if length < amount_index + 1 {
            for m in 0..(length + 1) {
                let post: Option<Post> = self.post_map.get(&m);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num + 1) {
                let post: Option<Post> = self.post_map.get(&(length - n));
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        post_list
    }

    pub fn get_individual_post(&self, num: u128, account_id: AccountId) -> Vec<Post> {
        let mut post_list: Vec<Post> = Vec::new();
        let post_id_list: Vec<u128> = self.profile_map.get(&account_id).unwrap().post_id_list;
        let amount_index: u128 = 5;
        let length: u128 = post_id_list.len() as u128;

        if length < amount_index + 1 {
            for m in 0..(length) {
                let post: Option<Post> = self.post_map.get(&post_id_list[m as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num + 1) {
                let post: Option<Post> = self.post_map.get(&post_id_list[(length - n) as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        post_list
    }

    pub fn add_likes(&mut self, post_id: u128) {
        let mut post: Post = self.post_map.get(&post_id).unwrap();
        post.num_of_likes = &post.num_of_likes + 1;
        self.post_map.insert(&post_id, &post);
    }
}
