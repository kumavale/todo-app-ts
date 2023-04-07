# Todoアプリ

Reactアプリに入門した with TypeScript

## 仕様

- 完了／未完了別にリスト化して表示(Read)
- 追加ができる(Create)
- 状態（完了／未完了）をボタンクリックで変更できる(Update)
- 削除ができる(Delete)
- モックサーバーでTODO情報を更新管理できる

## 実行

1. モックサーバーの起動

```
$ cd client
$ npx json-server --watch db.json --port 3100
```

2. Reactアプリの起動

```
$ cd client
$ npm start
```

3. ブラウザから [http://localhost:3000](http://localhost:3000) へアクセス

## 参考

- [React + TypeScript で Todoアプリの作成](https://qiita.com/tseno/items/b7133d73966c405b7249)

