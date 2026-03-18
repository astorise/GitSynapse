# Proposal: Mise en place du pipeline d'Intégration Continue (CI)

## Objectif
Garantir la qualité, la sécurité et la constructibilité du code de la couche Git FaaS à chaque modification, avant toute fusion sur la branche principale.

## Portée (Scope)
Création d'un workflow GitHub Actions automatisé qui s'exécutera à chaque `push` ou `pull request` vers la branche `main`. Ce pipeline devra accomplir trois missions principales :
1. **Validation statique** : Vérifier le formatage du code Rust et exécuter le linter (`clippy`).
2. **Tests** : Compiler le projet et exécuter la suite de tests unitaires.
3. **Build Docker** : Vérifier que l'image Docker multi-stage (définie dans le MVP) se construit correctement (sans nécessairement la pousser sur un registre dans un premier temps).