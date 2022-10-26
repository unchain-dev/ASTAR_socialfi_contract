use crate::metadata::*;
use ink_env::AccountId;
// use ink_lang as ink;
use ink_prelude::vec::Vec;

use crate::astar_sns::AstarSns;

impl AstarSns {
    // release post function
    pub fn release_post(
        &mut self,
        account_id: AccountId,
        description: String,
        created_time: String,
        post_img_url: String,
    ) {
        // get specified account id's profile
        let profile_info: Profile = self.profile_map.get(&account_id).unwrap();

        // insert post info
        // get info of account id's user
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

        // get specified account id's profile
        let mut profile: Profile = self.profile_map.get(&account_id).unwrap();

        // add post_map_counter to account id's profile
        profile.post_id_list.push(self.post_map_counter);

        // override new profile info to profile_map
        self.profile_map.insert(&account_id, &profile);

        // increment post_map_counter
        self.post_map_counter = &self.post_map_counter + 1;
    }

    // get latest post list in all users' posts list
    // variable `num` is to get specified posts
    pub fn get_general_post(&self, num: u128) -> Vec<Post> {
        // create list for returning needed posts list
        let mut post_list: Vec<Post> = Vec::new();

        // get post_map_counter that represent post_map size
        let length: u128 = self.post_map_counter;

        // this is index to specify how many post to get
        // for frontend, make it flexible how many post you can query
        let amount_index: u128 = 5;

        // judge number of posts is less than specified number or not
        // this is because how to get is different
        if length < amount_index + 1 {
            for m in 0..(length + 1) {
                // get specified post and add to returning list
                let post: Option<Post> = self.post_map.get(&m);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // get specified post and add to returning list
                // this is done from latest posts
                let post: Option<Post> = self.post_map.get(&(length - n - 1));
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        post_list
    }

    // get latest post list in specified users' posts list
    // variable `num` is to get specified posts
    pub fn get_individual_post(&self, num: u128, account_id: AccountId) -> Vec<Post> {
        // create list for returning needed posts list
        let mut post_list: Vec<Post> = Vec::new();

        // get specified account id's post id list
        let post_id_list: Vec<u128> = self.profile_map.get(&account_id).unwrap().post_id_list;

        // this is index to specify how many post to get
        // for frontend, make it flexible how many post you can query
        let amount_index: u128 = 5;

        // get post id list's length
        let length: u128 = post_id_list.len() as u128;

        // judge number of posts is less than specified number or not
        // this is because how to get is different
        if length < amount_index + 1 {
            for m in 0..(length) {
                // get specified post and add to returning list
                let post: Option<Post> = self.post_map.get(&post_id_list[m as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        } else {
            for n in (amount_index * (num - 1))..(amount_index * num) {
                // get specified post and add to returning list
                // this is done from latest posts
                let post: Option<Post> =
                    self.post_map.get(&post_id_list[(length - n - 1) as usize]);
                if post.is_some() {
                    post_list.push(post.unwrap());
                }
            }
        }
        post_list
    }

    // add likes to specified post
    pub fn add_likes(&mut self, post_id: u128) {
        // get current info of specified post
        let mut post: Post = self.post_map.get(&post_id).unwrap();

        // increment number of likes of copied post
        post.num_of_likes = &post.num_of_likes + 1;

        // override post_map with updated post
        self.post_map.insert(&post_id, &post);
    }
}
