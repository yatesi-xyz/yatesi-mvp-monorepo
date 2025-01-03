FROM node:22-alpine AS base

FROM base AS builder
WORKDIR /app

COPY package.json package-lock.json* pnpm-lock.yaml* ./
RUN corepack enable pnpm && pnpm i --frozen-lockfile;

COPY src ./src
# COPY public ./public
COPY next.config.ts .
COPY tsconfig.json .
COPY postcss.config.mjs .
COPY tailwind.config.ts .

ARG NEXT_PUBLIC_WEBSOCKET_URL
ENV NEXT_PUBLIC_WEBSOCKET_URL=${NEXT_PUBLIC_WEBSOCKET_URL}

ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production
RUN pnpm build

FROM base AS runner
WORKDIR /app

RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
USER nextjs

COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./.next/standalone
# COPY --from=builder --chown=nextjs:nodejs /app/public ./.next/standalone/public
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/standalone/.next/static

ENV NEXT_TELEMETRY_DISABLED=1
ENV NODE_ENV=production
ENV PORT=8080
ENV HOSTNAME=0.0.0.0
CMD ["node", ".next/standalone/server.js"]
