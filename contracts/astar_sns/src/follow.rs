use crate::metadata::*;
use ink_env::AccountId;

use crate::astar_sns::AstarSns;

impl AstarSns {
    // follow function
    pub fn follow(&mut self, following_account_id: AccountId, followed_account_id: AccountId) {
        // get profile info of a user who follow someone
        let mut following_user_profile: Profile =
            self.profile_map.get(&following_account_id).unwrap();

        // get profile info of a user who is followed by someone
        let mut followed_user_profile: Profile =
            self.profile_map.get(&followed_account_id).unwrap();

        // add other's account id to following user's `following_list`
        following_user_profile
            .following_list
            .push(followed_account_id);

        // add other's account id to followed user's `follower_list`
        followed_user_profile
            .follower_list
            .push(following_account_id);

        // get message list's length of a user who follow someone
        let length: usize = following_user_profile.message_list_id_list.len();

        // if one of two accounts does not have any message room, add message list
        if followed_user_profile.message_list_id_list.len() == 0
            && following_user_profile.message_list_id_list.len() == 0
        {
            followed_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            following_user_profile
                .message_list_id_list
                .push(self.message_list_map_counter);
            self.message_list_map_counter = &self.message_list_map_counter + 1;
        }

        for n in 0..length {
            // get whether one account has the same message list's id
            let is_contained = followed_user_profile
                .message_list_id_list
                .contains(&following_user_profile.message_list_id_list[n]);

            // if not, add message list's id to both of profile
            if !is_contained {
                followed_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                following_user_profile
                    .message_list_id_list
                    .push(self.message_list_map_counter);
                self.message_list_map_counter = &self.message_list_map_counter + 1;
            }
        }

        // override profile info
        self.profile_map
            .insert(&following_account_id, &following_user_profile);
        self.profile_map
            .insert(&followed_account_id, &followed_user_profile);
    }

    // get following list of specified account
    pub fn get_following_list(&self, account_id: AccountId) -> Vec<AccountId> {
        let following_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().following_list;
        following_list
    }

    // get followed list of specified account
    pub fn get_follower_list(&self, account_id: AccountId) -> Vec<AccountId> {
        let follower_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().follower_list;
        follower_list
    }
}
