/* eslint-disable react-hooks/rules-of-hooks */
import { ApiPromise } from '@polkadot/api';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import React, { SetStateAction, useEffect, useState } from 'react';

import BottomNavigation from '../../components/bottomNavigation';
import MessageMember from '../../components/message_member';
import MessageRoom from '../../components/messageRoom';
import TopBar from '../../components/topBar';
import { connectToContract } from '../../hooks/connect';
import { balanceOf } from '../../hooks/FT';
import { getLastMessage, getMessageList } from '../../hooks/messageFunction';
import {
  checkCreatedInfo,
  createProfile,
  getProfileForMessage,
  getSimpleProfileForMessage,
} from '../../hooks/profileFunction';
import type { ProfileType } from '../../hooks/profileFunction';

export default function message() {
  // variable related to contract
  const [api, setApi] = useState<ApiPromise>();
  const [accountList, setAccountList] = useState<InjectedAccountWithMeta[]>([]);
  const [actingAccount, setActingAccount] = useState<InjectedAccountWithMeta>();

  const [imgUrl, setImgUrl] = useState('');
  const [isCreatedProfile, setIsCreatedProfile] = useState(true);
  const [isCreatedFnRun, setIsCreatedFnRun] = useState(false);
  const [friendList, setFriendList] = useState([]);
  const [messageList, setMessageList] = useState([]);
  const [showMessageModal, setShowMessageModal] = useState(false);
  const [userName, setUserName] = useState('');
  const [userImgUrl, setUserImgUrl] = useState('');
  const [myImgUrl, setMyImgUrl] = useState('');
  const [messageListId, setMessageListId] = useState<string>('');
  const [messageMemberList, setMessageMemberList] = useState([]);
  const [myUserId, setMyUserId] = useState('');
  const [isSetup, setIsSetup] = useState(false);
  const [profile, setProfile] = useState<ProfileType>();
  const [balance, setBalance] = useState<string>('0');

  useEffect(() => {
    // connect to contract
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

    // get profile
    getProfileForMessage({
      api,
      userId: actingAccount?.address,
      setImgUrl,
      setMyImgUrl,
      setFriendList,
      setProfile,
    });
    // create message member list UI
    createMessageMemberList();

    balanceOf({
      api,
      actingAccount: actingAccount!,
      setBalance,
    });

    // check if already created profile in frontend
    if (isCreatedFnRun) return;

    // check if already created profile in contract
    checkCreatedInfo({
      api,
      userId: actingAccount?.address,
      setIsCreatedProfile,
    });
    if (isCreatedProfile) return;
    // create profile
    createProfile({ api, actingAccount: actingAccount! });
    setIsCreatedFnRun(true);
  });

  // create message member list UI
  const createMessageMemberList = async () => {
    const memberList: SetStateAction<never[]> = [];
    for (let i = 0; i < friendList.length; i++) {
      const friendProfile = await getSimpleProfileForMessage({
        api,
        userId: friendList[i],
      });
      const idList = profile?.messageListIdList;
      let lastMessage: string;
      const messageList = await getMessageList({
        api,
        id: idList![i],
      });
      if (idList !== null) {
        lastMessage = await getLastMessage({ api, id: idList![i] });
      }
      const memberListFactor = (
        <MessageMember
          name={friendProfile?.name}
          img_url={friendProfile?.imgUrl}
          last_message={lastMessage}
          setShowMessageModal={setShowMessageModal}
          setUserName={setUserName}
          setUserImgUrl={setUserImgUrl}
          setMyImgUrl={setMyImgUrl}
          messageListId={idList![i]}
          setMessageListId={setMessageListId}
          setMessageList={setMessageList}
          messageList={messageList}
          getMessageList={getMessageList}
          setMyUserId={setMyUserId}
          myUserId={profile?.userId}
          api={api}
        />
      );
      memberList.push(memberListFactor);
    }
    setMessageMemberList(memberList);
  };

  return !showMessageModal ? (
    <div className="flex justify-center items-center bg-gray-200 w-screen h-screen relative">
      <main className="items-center h-screen w-1/3 flex bg-white flex-col">
        <TopBar
          idList={accountList}
          imgUrl={imgUrl}
          setActingAccount={setActingAccount}
          balance={balance}
        />
        <div className="flex-1 overflow-scroll w-full mt-1">
          {messageMemberList}
        </div>
        <div className="w-full">
          <BottomNavigation />
        </div>
      </main>
    </div>
  ) : (
    <MessageRoom
      setShowMessageModal={setShowMessageModal}
      userName={userName}
      userImgUrl={userImgUrl}
      myImgUrl={myImgUrl}
      myUserId={myUserId}
      api={api!}
      actingAccount={actingAccount!}
      messageListId={messageListId!}
      messageList={messageList!}
    />
  );
}
