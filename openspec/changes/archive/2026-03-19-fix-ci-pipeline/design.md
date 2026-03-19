# Design: Analyse des causes racines et décisions techniques

## Bug 1 — `working-directory` manquant

**Cause** : Le dépôt est une monorepo avec la crate Rust dans `git-faas-layer/`. Sans `working-directory`, `cargo` cherche un `Cargo.toml` à la racine et échoue avec `cargo metadata exited with an error`.

**Décision** : Ajouter `working-directory: git-faas-layer` à chaque step cargo dans le job `lint-and-test`.

## Bug 2 — Code source non formaté

**Cause** : Les fichiers sources avaient été écrits sans passer par `cargo fmt`. Rustfmt réordonne les imports (`use` statements) et reformate les expressions longues.

**Décision** : Appliquer `cargo fmt` localement sur tous les fichiers concernés (`error.rs`, `git_ops.rs`, `handlers.rs`, `main.rs`) avant de relancer le check.

## Bug 3 — Warnings Clippy traités comme erreurs

**Cause** : Le flag `-D warnings` élève tous les warnings en erreurs. Trois cas identifiés :
- `unused_mut` sur `fetch_opts` (ligne 194 de `git_ops.rs`) — variable déclarée `mut` sans mutation.
- `dead_code` sur les variantes `NotFound` et `BadRequest` de `AppError` — non construites dans le code actuel mais part de l'API publique.
- `redundant_locals` sur `let req = req` — binding identique commenté "move into blocking closure", inutile depuis Rust 2021.

**Décisions** :
- Supprimer `mut` sur `fetch_opts`.
- Ajouter `#[allow(dead_code)]` sur `AppError` pour préserver les variantes d'API.
- Supprimer le rebinding `let req = req`.

## Bug 4 — Contexte Docker incorrect

**Cause** : `context: .` dans `docker/build-push-action` pointe vers la racine du repo, qui ne contient pas de `Dockerfile`. Celui-ci se trouve dans `git-faas-layer/`.

**Décision** : Remplacer `context: .` par `context: git-faas-layer`.

## Bug 5 — Version Rust incompatible avec `edition = "2024"`

**Cause** : `Cargo.toml` utilise `edition = "2024"`, introduite dans Rust 1.85. L'image `rust:1.80-slim-bookworm` échoue lors du parsing du manifest.

**Décision** : Mettre à jour l'image vers `rust:1.85-slim-bookworm`.

## Bug 6 — `perl` absent pour la build OpenSSL vendorisée

**Cause** : La feature `vendored-openssl` de `git2` compile OpenSSL depuis les sources. Le script de configuration d'OpenSSL (`./Configure`) est un script Perl — absent des images `slim` Debian.

**Décision** : Ajouter `perl` à la commande `apt-get install` du stage `builder`.
