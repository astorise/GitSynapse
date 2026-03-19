# Proposal: Correctif du pipeline CI — bugs découverts au premier run

## Objectif
Corriger les six bugs identifiés lors du premier run du pipeline CI mis en place par `2026-03-18-setup-ci-pipeline`, et mettre à jour les spécifications pour qu'elles reflètent fidèlement l'implémentation réelle.

## Contexte
Immédiatement après la mise en place du workflow GitHub Actions, le pipeline a échoué en cascade sur chacun des jobs, révélant des incohérences entre la spec initiale et la réalité du projet :

1. **working-directory manquant** : les steps cargo (`fmt`, `clippy`, `test`) s'exécutaient à la racine du dépôt, où il n'y a pas de `Cargo.toml`.
2. **Code non formaté** : plusieurs fichiers source ne respectaient pas le style imposé par `rustfmt` (ordre des imports, longueur de ligne, etc.).
3. **Warnings Clippy bloquants** : `unused_mut`, `dead_code` sur des variantes d'enum, et `redundant_locals`.
4. **Contexte Docker incorrect** : `context: .` pointait vers la racine du repo, sans `Dockerfile`.
5. **Version Rust incompatible** : `rust:1.80` ne supporte pas `edition = "2024"` (requis ≥ 1.85).
6. **`perl` absent** : la build OpenSSL vendorisée (feature `vendored-openssl` de `git2`) nécessite `perl`, absent des images `slim`.

## Portée
Correctifs purement techniques, sans modification fonctionnelle. Le comportement observable du pipeline reste identique à ce qui était spécifié, seule l'implémentation est corrigée.
