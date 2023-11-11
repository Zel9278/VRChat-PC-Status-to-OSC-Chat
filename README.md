# PC Status to OSC VRChat Chatbox - PCOSCVRC

[PC Status Client - Rust](https://github.com/kazukazu123123/pcsc-rs)を使用した、VRChatのチャットボックスにPC情報を表示するOSC サーバーアプリケーションです。

![Sample](https://cdn.discordapp.com/attachments/1023650218597691442/1172897736228614278/image.png?ex=6561fd23&is=654f8823&hm=b5ac954285d588e966fce2beeb8bcac39a98fbc3b1bb4bc05d5035b12db0c677&)

## 必要なもの

1. [Node.js LTS](https://nodejs.org/en)
2. corepack (Node.jsをインストール後 `$ corepack enable` で有効化)

## 使い方

1. このリポジトリをクローンします。
2. [PC Status Client - Rustのreleasesページ](https://github.com/kazukazu123123/pcsc-rs/releases)から最新の**MSVCリリース**をダウンロードします。 (`x86_64-pc-windows-msvc`)
3. ダウンロードしたexeを適当な場所に保存して起動します。
4. `pnpm i` を実行後、 `node .` で起動します。
5. VRChatのチャットボックスに自動的にPC情報が送信されます。
