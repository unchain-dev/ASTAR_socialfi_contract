use crate::metadata::*;
use ink_env::AccountId;

use crate::astar_sns::AstarSns;

impl AstarSns {
    pub fn follow(&mut self, following_account_id: AccountId, followed_account_id: AccountId) {
        let mut following_user_profile: Profile =
            self.profile_map.get(&following_account_id).unwrap();
        let mut followed_user_profile: Profile =
            self.profile_map.get(&followed_account_id).unwrap();
        following_user_profile
            .following_list
            .push(followed_account_id);
        followed_user_profile
            .follower_list
            .push(following_account_id);

        // devise how to make message list id as unique one
        let length: usize = following_user_profile.message_list_id_list.len();
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
            let is_contained = followed_user_profile
                .message_list_id_list
                .contains(&following_user_profile.message_list_id_list[n]);
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
        self.profile_map
            .insert(&following_account_id, &following_user_profile);
        self.profile_map
            .insert(&followed_account_id, &followed_user_profile);
    }

    pub fn get_following_list(&self, account_id: AccountId) -> Vec<AccountId> {
        let following_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().following_list;
        following_list
    }

    pub fn get_follower_list(&self, account_id: AccountId) -> Vec<AccountId> {
        let follower_list: Vec<AccountId> =
            self.profile_map.get(&account_id).unwrap().follower_list;
        follower_list
    }
}
