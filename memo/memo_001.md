# 手順

1. `cargo create-tauri-app`
2. `nodenv local 22.4.0`
3. 以下のコマンドを実行

   ```bash
   cd plot451
   npm install
   npm run tauri dev
   ```

4. ドメインモデル管理用クレートの作成

   ```bash
   cargo new src-model --vcs none --lib
   ```

5. アプリケーションサービス管理用のクレートの作成

    ```bash
    cargo new src-application --vcs none --lib
    ```

6. in memory リポジトリ管理用のクレートの作成

    ```bash
    cargo new src-in-memory-infrastructure --vcs none --lib
    ```

7. diesel リポジトリ管理用のクレートの作成

   ```bash
   cargo new src-diesel-infrastructure --vcs none --lib
   ```

8. sqlx リポジトリ管理用のクレートの作成

   ```bash
   cargo new src-sqlx-infrastructure --vcs none --lib
   ```
