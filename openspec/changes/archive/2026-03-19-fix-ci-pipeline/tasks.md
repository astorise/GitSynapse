# Tasks: Correctifs CI Pipeline

- [x] 1. Ajouter `working-directory: git-faas-layer` aux steps `cargo fmt`, `cargo clippy` et `cargo test` dans `.github/workflows/ci.yml`.
- [x] 2. Formater les fichiers source (`error.rs`, `git_ops.rs`, `handlers.rs`, `main.rs`) avec `cargo fmt`.
- [x] 3. Supprimer `mut` sur `fetch_opts` dans `git_ops.rs:194`.
- [x] 4. Ajouter `#[allow(dead_code)]` sur `pub enum AppError` dans `error.rs`.
- [x] 5. Supprimer le rebinding `let req = req;` dans `git_ops.rs` (fonction `extract_context`).
- [x] 6. Remplacer `context: .` par `context: git-faas-layer` dans le job `docker-build` du workflow CI.
- [x] 7. Mettre à jour l'image Docker de `rust:1.80-slim-bookworm` vers `rust:1.85-slim-bookworm` dans `git-faas-layer/Dockerfile`.
- [x] 8. Ajouter `perl` aux dépendances système dans la commande `apt-get install` du `Dockerfile`.
