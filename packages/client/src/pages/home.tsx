/* eslint-disable react-hooks/rules-of-hooks */
import { ApiPromise } from '@polkadot/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import React, { useEffect, useState } from 'react';

import { PostButton } from '../../components/atoms/postButton';
import BottomNavigation from '../../components/bottomNavigation';
import Post from '../../components/post';
import PostModal from '../../components/postModal';
import TopBar from '../../components/topBar';
import { balanceOf, distributeReferLikes, transfer } from '../../hooks/FT';
import { connectToContract } from '../../hooks/connect';
import type { PostType } from '../../hooks/postFunction';
import { getGeneralPost } from '../../hooks/postFunction';
import {
  checkCreatedInfo,
  createProfile,
  getProfileForHome,
} from '../../hooks/profileFunction';

export default function home() {
  const [api, setApi] = useState<ApiPromise>();

  const [isCreatedProfile, setIsCreatedProfile] = useState(true);
  const [isCreatedFnRun, setIsCreatedFnRun] = useState(false);
  const [showNewPostModal, setShowNewPostModal] = useState(false);
  const [isSetup, setIsSetup] = useState(false);
  const [isDistributed, setIsDistributed] = useState(false);

  const [imgUrl, setImgUrl] = useState('');
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAccount, setActingAccount] = useState<InjectedAccountWithMeta>();
  const [generalPostList, setGeneralPostList] = useState<PostType[]>([]);
  const [balance, setBalance] = useState<string>('0');

  useEffect(() => {
    connectToContract({
      api,
      accountList,
      actingAccount: actingAccount!,
      isSetup,
      setApi,
      setAccountList,
      setActingAccount: setActingAccount!,
      setIsSetup,
    });
    if (!isSetup) return;
    getProfileForHome({
      api: api!,
      userId: actingAccount?.address,
      setImgUrl,
    });
    balanceOf({
      api,
      actingAccount: actingAccount!,
      setBalance,
    });
    getGeneralPost({ api: api!, setGeneralPostList });
    if (isDistributed) return;
    distributeReferLikes({
      api,
      actingAccount: actingAccount!,
    });
    setIsDistributed(true);
    if (isCreatedFnRun) return;
    checkCreatedInfo({
      api,
      userId: actingAccount?.address,
      setIsCreatedProfile,
    });
    if (isCreatedProfile) return;
    createProfile({ api, actingAccount: actingAccount! });
    setIsCreatedFnRun(true);
  });

  return (
    <div className="flex justify-center items-center bg-gray-200 w-screen h-screen relative">
      <main className="items-center justify-center h-screen w-1/3 flex bg-white flex-col">
        <PostModal
          isOpen={showNewPostModal}
          afterOpenFn={setShowNewPostModal}
          api={api!}
          actingAccount={actingAccount!}
        />
        <TopBar
          idList={accountList}
          imgUrl={imgUrl}
          setActingAccount={setActingAccount}
          balance={balance}
        />
        <div className="flex-1 overflow-scroll">
          {generalPostList.map((post) => (
            <Post
              name={post.name}
              time={post.createdTime}
              description={post.description}
              num_of_likes={post.numOfLikes}
              user_img_url={post.userImgUrl}
              post_img_url={post.imgUrl}
              userId={post.userId}
              postId={post.postId}
              actingAccount={actingAccount}
              api={api}
              key={post}
            />
          ))}
        </div>
        <div className="w-full">
          <BottomNavigation api={api} />
        </div>
        <PostButton setShowNewPostModal={setShowNewPostModal} />
      </main>
    </div>
  );
}
