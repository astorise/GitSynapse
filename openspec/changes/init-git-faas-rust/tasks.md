# Tasks: Implémentation Détaillée du Git FaaS

## Phase 1 : Setup et Typage
- [x] 1.1. Initialiser le projet : `cargo new git-faas-layer --bin`.
- [x] 1.2. Mettre à jour `Cargo.toml` avec les dépendances strictes :
  - `axum = "0.7"`
  - `tokio = { version = "1", features = ["full"] }`
  - `git2 = { version = "0.19", features = ["vendored-openssl", "vendored-libgit2"] }`
  - `serde = { version = "1.0", features = ["derive"] }`
  - `serde_json = "1.0"`
  - `tracing` et `tracing-subscriber` pour les logs.
- [x] 1.3. Créer `src/models.rs` et définir les structures DTO (Data Transfer Objects) : `ExtractRequest`, `ExtractResponse`, `FileContent`, `CommitRequest`, `CommitResponse`. Dériver `Serialize` et `Deserialize`.
- [x] 1.4. Créer `src/error.rs` : Implémenter l'enum `AppError` avec `thiserror` (si désiré) ou manuellement, et implémenter `IntoResponse` d'Axum pour mapper `git2::Error` vers un `StatusCode::INTERNAL_SERVER_ERROR` ou `BAD_REQUEST` avec un body JSON.

## Phase 2 : Le Moteur Git (`src/git_ops.rs`)
- [x] 2.1. Implémenter la fonction asynchrone (ou blocking via `tokio::task::spawn_blocking` car `git2` est synchrone) `extract_context(req: ExtractRequest) -> Result<ExtractResponse, AppError>`.
  - *Logique :* Ouvrir le repo, trouver la branche, récupérer le `Tree`, utiliser `tree.get_path()` pour récupérer le `Blob` du fichier, et convertir son contenu en `String` (UTF-8).
- [x] 2.2. Implémenter la fonction `create_in_memory_commit(req: CommitRequest) -> Result<CommitResponse, AppError>`.
  - *Logique :* Suivre strictement l'algorithme "In-Memory Commit" détaillé dans le `design.md` (étapes 1 à 8 impliquant `TreeBuilder` et `repo.blob()`). Gérer l'authentification remote avec des `git2::RemoteCallbacks` (via token passé en variable d'environnement `GITLAB_TOKEN`).
- [x] 2.3. Implémenter la fonction `generate_diff(base_sha: &str, target_sha: &str) -> Result<DiffResponse, AppError>`.
  - *Logique :* Récupérer les deux `Tree` correspondants aux SHAs, utiliser `repo.diff_tree_to_tree()`, et itérer sur les deltas pour construire un JSON de retour.

## Phase 3 : Le Serveur Web (`src/handlers.rs` & `src/main.rs`)
- [x] 3.1. Créer `src/handlers.rs` : Écrire les fonctions de routing axum (`async fn extract_handler(Json(payload): Json<ExtractRequest>) -> Result<Json<ExtractResponse>, AppError>`, etc.) qui appellent les fonctions de `git_ops.rs`.
- [x] 3.2. Dans `src/main.rs` : Initialiser `tracing_subscriber::fmt::init()`.
- [x] 3.3. Dans `src/main.rs` : Créer le `axum::Router`, attacher les routes (`/api/v1/context`, `/api/v1/commit`, `/api/v1/diff`), et lancer le serveur `tokio::net::TcpListener` sur `0.0.0.0:8080`.

## Phase 4 : Déploiement Cloud-Native
- [x] 4.1. Créer un `Dockerfile` optimisé (Multi-stage).
  - *Stage Builder :* Image `rust:1.80-slim-bookworm`, installer `pkg-config` et `libssl-dev` (bien que vendored soit utilisé, c'recommandé pour certains environnements de build), faire le `cargo build --release`.
  - *Stage Runtime :* Utiliser `debian:bookworm-slim` (éviter distroless pur si on lie dynamiquement la libc), copier le binaire, exposer le port 8080, et définir la commande de démarrage.
- [x] 4.2. Créer `k8s/knative-service.yaml` :
  - Définir un `Service` de `apiVersion: serving.knative.dev/v1`.
  - Dans `spec.template.metadata.annotations`, AJOUTER OBLIGATOIREMENT : `linkerd.io/inject: "enabled"`.
  - Configurer les variables d'environnement (ex: `GITLAB_TOKEN` venant d'un K8s Secret).
