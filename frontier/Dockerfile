FROM python:3.12.7-slim-bookworm AS base
COPY --from=ghcr.io/astral-sh/uv:0.5.8 /uv /bin/uv
ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy
WORKDIR /app
COPY uv.lock pyproject.toml /app/
RUN --mount=type=cache,target=/root/.cache/uv uv sync --frozen --no-install-project --no-dev
COPY . /app
RUN --mount=type=cache,target=/root/.cache/uv uv sync --frozen --no-dev
CMD ["uv", "run", "--frozen", "--no-dev", "frontier"]
