# PC Status to OSC VRChat Chatbox - PCOSCVRC

[PC Status Client - Rust](https://github.com/kazukazu123123/pcsc-rs)を使用した、VRChatのチャットボックスにPC情報を表示するOSC サーバーアプリケーションです。

![Sample](https://camo.githubusercontent.com/d2ab77e1af8f81a46d650127a8fec1c560b9bd2a72ab477058f41f439294023e/68747470733a2f2f63646e2e646973636f72646170702e636f6d2f6174746163686d656e74732f313032333635303231383539373639313434322f313137323839373733363232383631343237382f696d6167652e706e673f65783d36353631666432332669733d363534663838323326686d3d6235616339353432383564353838653936366663653262656562386263616333396139386662633362316262346263303564353033356231326462306336373726)

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
