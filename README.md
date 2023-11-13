# PC Status to OSC VRChat Chatbox - PCOSCVRC

[PC Status Client - Rust](https://github.com/kazukazu123123/pcsc-rs)を使用した、VRChatのチャットボックスにPC情報を表示するOSC サーバーアプリケーションです。

![Sample](https://i.imgur.com/AH8MUeq.png)

## 必要なもの

1. [Node.js LTS](https://nodejs.org/en)
2. corepack (Node.jsをインストール後 `$ corepack enable` で有効化)

## 使い方

1. このリポジトリをクローンしてください。
2. [PC Status Client - Rustのreleasesページ](https://github.com/kazukazu123123/pcsc-rs/releases)から最新の**MSVCリリース**をダウンロードしてください。 (`x86_64-pc-windows-msvc`)
3. ダウンロードしたexeを適当なフォルダに保存してください。
4. 同じフォルダに `.env` ファイルを作成し、以下のように記述してください。

```env
PASS=npU7pmkkYfuUdKfqzm2BtDfBPEe4pizrXyPVj8Fby3KaUtehNu3ToDtM8uEdGBr3AS9LRUkZixtZxuKTvsL2e4BVrfzWWG7RqqVThLWsVLHLaJJ8ekeGuHtLBkfZpBtv
```

5. ダウンロードしたexeを起動してください。
6. クローンしたリポジトリに移動し、コマンドプロンプトを開いて `pnpm i` を実行後、 `start-vrc-osc.bat` を実行します。
7. VRChatのチャットボックスに自動的にPC情報が送信されます。
