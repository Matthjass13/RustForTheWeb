# ─── Stage 1 : Install deps & run dev server ───────────────────────────────────
FROM node:20-alpine

WORKDIR /app

# 1. Copy manifests and install dependencies
COPY package.json package-lock.json* ./
RUN npm install

# 2. Copy the rest of the source
COPY . .

EXPOSE 5173

# Vite must listen on 0.0.0.0 to be reachable from outside the container
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
