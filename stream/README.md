# ストリームサーバ

一定間隔で購買データをTCPに送信し続けるストリームサーバ。

## 設定

- `STREAM_SERVER_IP`
- `STREAM_SERVER_PORT`

## 実行

`cargo run --release`

## 送信データの確認

`python tcp_client_for_test.py`

## クレジット

### データセット

- **Description**: This is a transnational data set which contains all the transactions occurring between 01/12/2010 and 09/12/2011 for a UK-based and registered non-store online retail.
- **Source**: <https://archive.ics.uci.edu/dataset/352/online+retail>
- **License**: [CCBY 4.0](https://creativecommons.org/licenses/by/4.0/legalcode)
