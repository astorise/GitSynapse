# Tasks: Implémentation du Workflow GitHub Actions

- [x] 1. Créer le répertoire `.github/workflows/` à la racine du projet.
- [x] 2. Créer le fichier `.github/workflows/ci.yml`.
- [x] 3. Définir les `on:` triggers pour `push` et `pull_request` sur la branche `main`.
- [x] 4. Créer le job `lint-and-test` :
  - Utiliser l'image `ubuntu-latest`.
  - Ajouter l'étape `actions/checkout@v4`.
  - Ajouter l'étape pour installer la toolchain Rust stable (`dtolnay/rust-toolchain@stable` recommandé).
  - Ajouter l'étape de cache : `Swatinem/rust-cache@v2`.
  - Ajouter l'étape : `cargo fmt --all -- --check`.
  - Ajouter l'étape : `cargo clippy --all-targets --all-features -- -D warnings`.
  - Ajouter l'étape : `cargo test --verbose`.
- [x] 5. Créer le job `docker-build` :
  - Définir `needs: lint-and-test`.
  - Utiliser `ubuntu-latest`.
  - Ajouter l'étape `actions/checkout@v4`.
  - Configurer Buildx avec `docker/setup-buildx-action@v3`.
  - Utiliser `docker/build-push-action@v5` avec les paramètres `context: .` et `push: false`.