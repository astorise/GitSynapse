# GCP Setup — Commandes manuelles (one-time)

Ces commandes sont à exécuter **une seule fois** par l'administrateur GCP avant le premier déploiement.

## Prérequis
- `gcloud` CLI installé et authentifié (`gcloud auth login`)
- `PROJECT_ID` : votre projet GCP (ex: `gitsynapse`)
- `GITHUB_ORG` : votre organisation ou compte GitHub (ex: `mon-org`)
- `GITHUB_REPO` : le nom du dépôt (ex: `GitSynapse`)

```bash
export PROJECT_ID=gitsynapse
export REGION=europe-west9
export GITHUB_ORG=<votre-org-github>
export GITHUB_REPO=GitSynapse
```

---

## 1. Artifact Registry

Créer le registre Docker qui hébergera les images du service FaaS.

```bash
gcloud artifacts repositories create gitsynapse-repo \
  --repository-format=docker \
  --location=$REGION \
  --project=$PROJECT_ID \
  --description="Images Docker GitSynapse"
```

---

## 2. Bucket GCS pour les workspaces Git

Créer le bucket qui sera monté via GCS Fuse dans les pods Knative.

```bash
gcloud storage buckets create gs://gitsynapse-workspaces \
  --location=$REGION \
  --project=$PROJECT_ID \
  --uniform-bucket-level-access
```

---

## 3. Workload Identity Federation pour GitHub Actions

Permet au workflow CD de s'authentifier sur GCP sans clé de compte de service statique.

### 3a. Créer le pool Workload Identity

```bash
gcloud iam workload-identity-pools create github-pool \
  --location=global \
  --project=$PROJECT_ID \
  --display-name="GitHub Actions Pool"
```

### 3b. Créer le provider OIDC

```bash
gcloud iam workload-identity-pools providers create-oidc github-provider \
  --location=global \
  --workload-identity-pool=github-pool \
  --project=$PROJECT_ID \
  --display-name="GitHub OIDC Provider" \
  --issuer-uri="https://token.actions.githubusercontent.com" \
  --attribute-mapping="google.subject=assertion.sub,attribute.repository=assertion.repository" \
  --attribute-condition="assertion.repository=='${GITHUB_ORG}/${GITHUB_REPO}'"
```

### 3c. Créer le compte de service pour le CD

```bash
gcloud iam service-accounts create github-cd-sa \
  --project=$PROJECT_ID \
  --display-name="GitHub Actions CD Service Account"
```

### 3d. Accorder les rôles nécessaires

```bash
# Pousser des images sur Artifact Registry
gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:github-cd-sa@${PROJECT_ID}.iam.gserviceaccount.com" \
  --role="roles/artifactregistry.writer"

# Déployer sur GKE
gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:github-cd-sa@${PROJECT_ID}.iam.gserviceaccount.com" \
  --role="roles/container.developer"
```

### 3e. Lier le compte de service au pool Workload Identity

```bash
gcloud iam service-accounts add-iam-policy-binding \
  github-cd-sa@${PROJECT_ID}.iam.gserviceaccount.com \
  --project=$PROJECT_ID \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/projects/$(gcloud projects describe $PROJECT_ID --format='value(projectNumber)')/locations/global/workloadIdentityPools/github-pool/attribute.repository/${GITHUB_ORG}/${GITHUB_REPO}"
```

---

## 4. Secrets GitHub à configurer

Dans les **Settings → Secrets → Actions** du dépôt GitHub, ajouter :

| Secret | Valeur |
|--------|--------|
| `WIF_PROVIDER` | `projects/<PROJECT_NUMBER>/locations/global/workloadIdentityPools/github-pool/providers/github-provider` |
| `WIF_SERVICE_ACCOUNT` | `github-cd-sa@<PROJECT_ID>.iam.gserviceaccount.com` |

Récupérer le `PROJECT_NUMBER` avec :
```bash
gcloud projects describe $PROJECT_ID --format='value(projectNumber)'
```

---

## 5. Activer le driver GCS Fuse CSI sur le cluster GKE

```bash
gcloud container clusters update gitsynapse-cluster \
  --location=europe-west9-b \
  --project=$PROJECT_ID \
  --update-addons=GcsFuseCsiDriver=ENABLED
```
