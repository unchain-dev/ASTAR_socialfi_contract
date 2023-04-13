## ✅Astar-SocialFi(prototype)

本レポジトリは Astar-SocialFi の完成版を示したものになります。

以下の手順を実行することで Astar-SocialFi の挙動を確認できます。

## レポジトリのクローン

[Astar-SocialFi のリポジトリ](https://github.com/unchain-tech/ASTAR-SocialFi)から Astar-SocialFi をクローンします。

### コントラクトとフロントの準備

1. コントラクトのデプロイ

[Astar-SocialFi の教材](https://app.unchain.tech/learn/ASTAR-SocialFi/ja/0/2/)のうち section0-Lesson2 の polkadot.js の説明箇所を参考にコントラクトのデプロイを行います。

デプロイ後はコントラクトアドレスをどこかへ控えておいてください。

2. ウォレットの作成

[Astar-SocialFi の教材](https://app.unchain.tech/learn/ASTAR-SocialFi/ja/1/1/)のうち section1-Lesson4 の polkadot.js という拡張機能を追加してウォレットを作成する部分を参考にしてウォレットを作成しましょう。

作成したら、Alice という元々あるアカウントから 10 トークンほど作成したアドレスへ送金しておきましょう。ガス代として使用します。

3. フロントエンドのパッケージをインストール

トップディレクトリで下のコマンドを実行することで必要なパッケージをインストールしましょう。

```
yarn install
```

4. .env ファイルの作成、記述

1.で得たコントラクトアドレスと、ユーザーの初期プロフィール画像を下の変数名で設定しましょう。画像 URL は[unsplash のサイト](https://unsplash.com/)から取得しましょう。

```
NEXT_PUBLIC_CONTRACT_ADDRESS=WCpkcJenKkFZwk2tot1yhyvZuwFXD2xdzb7dyn2WMebKtC6
NEXT_PUBLIC_UNKNOWN_IMAGE_URL="https://images.unsplash.com/..."
```

5. フロントエンドの立ち上げ

最後に下のコマンドを実行することでフロントエンドを立ち上げます。その後は[Astar-SocialFi の教材](https://app.unchain.tech/learn/ASTAR-SocialFi/ja/3/1/)のうち section3-Lesson1 での動作確認の工程を参考にフロントの様子を確認しましょう。
