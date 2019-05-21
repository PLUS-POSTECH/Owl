# Owl development environment setup guide

## Prerequisites

### Install system-wide dependencies

- [Node.js](https://nodejs.org/en/)
- [Docker Desktop](https://hub.docker.com/)

### Install CLI helpers

```bash
# Yarn dependency manager (faster than npm)
# https://yarnpkg.com/lang/en/
npm install -g yarn

# Check yarn bin directory and add it to your PATH
# You may want to restart the shell after setting it
yarn global bin

# Prisma database access layer
# https://www.prisma.io/
yarn global add prisma

# Parcel package bundler
# https://parceljs.org/
yarn global add parcel-bundler
```

## Development Build Guide

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/PLUS-POSTECH/Owl
cd Owl

# Install git hooks
cp hooks/pre-commit .git/hooks/pre-commit

# Install JS dependency
yarn install

# Run Prisma server
docker-compose up -d
```

Now Prisma admin server runs at [http://localhost:4466/_admin](http://localhost:4466/_admin)

### Update

```bash
# Build client code (run this after changing owl.prisma file)
yarn build

# Start watch server
yarn watch
```

## Production Build Guide

**Work In Progress**

```bash
# Clone the repository
git clone https://github.com/PLUS-POSTECH/Owl
cd Owl

# Install JS dependency
yarn install

# Set secrets
# For differences, see https://www.prisma.io/docs/faq/service-secret-vs-management-api-secret-fq01/
export PRISMA_MANAGEMENT_SECRET="REPLACE_WITH_YOUR_PASSWORD"
export PRISMA_SERVICE_SECRET="REPLACE_WITH_YOUR_PASSWORD"

# Override docker-compose setup with production setting
cp docker-compose.production.yml docker-compose.yml

# Run Prisma server
docker-compose up -d

# Override Prisma setup with production setting
cp prisma.production.yml prisma.yml

# Build client code
yarn build

# Now how should we run the client code? Docker?
echo "WIP"
```
