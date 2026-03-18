# Design Document: Architecture de la CI GitHub Actions

## Stratégie des Jobs
Le workflow sera divisé en deux jobs distincts pour un retour rapide (Fast Feedback) :
1. `lint-and-test` : Job rapide. Si le code ne compile pas ou si les tests échouent, on arrête tout immédiatement.
2. `docker-build` : Job plus lent. Ne s'exécute que si `lint-and-test` a réussi (`needs: lint-and-test`).

## Optimisation des performances (Caching)
La compilation Rust et de ses dépendances (surtout `libgit2` via `git2-rs`) est très consommatrice en temps CPU.
- L'action `Swatinem/rust-cache@v2` DOIT être utilisée pour mettre en cache les dossiers `target/` et `~/.cargo/`. Cela réduira le temps de CI de plusieurs minutes à quelques secondes lors des builds itératifs.

## Sécurité
Pour ce pipeline de validation (CI pure), le job Docker se contentera de construire l'image locale (`load: true` ou juste un build sans push) pour s'assurer que le `Dockerfile` n'est pas cassé. Le déploiement continu (CD) fera l'objet d'une autre spécification.