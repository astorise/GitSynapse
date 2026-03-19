# Spécifications : Git Core API

## ADDED Requirements

### Requirement: Endpoint d'extraction de contexte
Le service SHALL fournir une route `POST /api/v1/context` permettant à un agent IA de récupérer le contenu texte de fichiers spécifiques sans cloner tout l'historique du dépôt.

#### Scenario: Extraction réussie de fichiers multiples
- **WHEN** un payload JSON valide avec une liste de chemins de fichiers est envoyé
- **THEN** le service clone superficiellement le dépôt
- **THEN** retourne un JSON contenant le texte brut de chaque fichier demandé
- **THEN** le code HTTP de retour est 200 OK

#### Scenario: Fichier inexistant
- **WHEN** l'agent IA demande un chemin de fichier qui n'existe pas dans l'arbre Git
- **THEN** le service l'ignore ou retourne un avertissement dans le JSON, mais ne crashe pas
- **THEN** le code HTTP de retour est 200 OK (partial success) ou 404 (si aucun fichier n'est trouvé)

### Requirement: Endpoint de création de commit
Le service SHALL fournir une route `POST /api/v1/commit` permettant à l'IA de proposer ses modifications de code.

#### Scenario: Commit en mémoire
- **WHEN** un payload contenant des modifications de code (`new_content`) et un message de commit est reçu
- **THEN** le service utilise `git2` pour créer un objet Tree en mémoire
- **THEN** le service pousse (push) le commit sur la branche cible
- **THEN** le service retourne le SHA du nouveau commit

### Requirement: Endpoint de révision (Diff)
Le service SHALL  fournir une route `POST /api/v1/diff` (ou GET avec query params) pour la supervision humaine et le modèle "Senior".

#### Scenario: Génération de diff structuré
- **WHEN** les SHA de deux commits (base et target) sont fournis
- **THEN** le service génère un diff en utilisant les API de diff de `git2`
- **THEN** retourne un format JSON structuré par fichier modifié, et non un simple texte unifié

### Requirement: Déploiement Knative et Linkerd
Le service SHALL être déployable sur Kubernetes en mode Serverless.

#### Scenario: Configuration des manifestes
- **WHEN** le manifeste de déploiement (Knative Service) est généré
- **THEN** il doit inclure l'annotation `linkerd.io/inject: enabled` dans les métadonnées du pod
- **THEN** il doit cibler le port 8080 du conteneur