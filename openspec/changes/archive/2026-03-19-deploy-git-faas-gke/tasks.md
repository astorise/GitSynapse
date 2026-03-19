# Tasks: Implémentation du CD et de l'Infra GKE

- [x] 1. Créer le fichier `k8s/crd-git-workspace.yaml` définissant la structure OpenAPI v3 du CRD `GitWorkspace` (groupe: `gitsynapse.dev`, version: `v1alpha1`).
- [x] 2. Modifier `git-faas-layer/k8s/knative-service.yaml` :
  - Ajouter les annotations d'autoscaling dans `spec.template.metadata.annotations` : `autoscaling.knative.dev/window: "120s"` et `autoscaling.knative.dev/target: "10"`.
  - Mettre à jour le nom de l'image pour utiliser le format GAR (ex: `europe-west9-docker.pkg.dev/gitsynapse/gitsynapse-repo/git-faas-layer:latest`).
- [x] 3. Modifier `git-faas-layer/k8s/knative-service.yaml` pour le stockage :
  - Ajouter l'annotation pod `gke-gcsfuse/volumes: "true"`.
  - Ajouter un `volume` de type `csi` (driver: `gcsfuse.csi.storage.gke.io`) pointant vers un bucket existant (ex: `gitsynapse-workspaces`).
  - Ajouter le `volumeMount` au conteneur vers le chemin `/mnt/workspaces`.
- [x] 4. Créer le fichier `.github/workflows/cd.yml` :
  - Se déclenche uniquement sur `push` vers `main` (ou après la réussite du workflow CI existant).
  - Utiliser `google-github-actions/auth@v2` avec un `workload_identity_provider`.
  - Configurer Docker pour Artifact Registry.
  - Builder et Pousser l'image.
  - Utiliser `google-github-actions/get-gke-credentials@v2` pour se connecter au cluster `gitsynapse-cluster` (`europe-west9-b`).
  - Exécuter `kubectl apply -f k8s/crd-git-workspace.yaml` puis `kubectl apply -f git-faas-layer/k8s/knative-service.yaml`.
- [x] 5. Créer un fichier `docs/GCP_SETUP.md` pour documenter les commandes `gcloud` manuelles que l'administrateur devra passer une fois pour créer le bucket GCS, l'Artifact Registry et le Workload Identity.