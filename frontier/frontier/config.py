from datetime import timedelta
from enum import StrEnum
from typing import Any

import humanize
import msgspec
import pytimeparse
from loguru import logger


def enc_hook(obj: Any) -> str:
    if isinstance(obj, HumanTimeDelta):
        return humanize.naturaldelta(obj.inner)

    raise NotImplementedError(f"type <{type(obj)}> is missing custom encoder")


def dec_hook(_type: type, obj: Any):
    if _type is HumanTimeDelta:
        return HumanTimeDelta(timedelta(seconds=pytimeparse.parse(obj) or 0))

    raise NotImplementedError(f"type <{type(obj)}> is missing custom decoder")


class HumanTimeDelta:
    inner: timedelta

    def __init__(self, inner: timedelta):
        self.inner = inner


class ApplicationMode(StrEnum):
    LISTEN = "listen"  # aggresively scan channels
    SCRAPE = "scrape"  # wait for new posts


class ApplicationConfig(msgspec.Struct):
    cooldown: HumanTimeDelta
    mode: ApplicationMode


class TelegramConfig(msgspec.Struct):
    api_id: int
    api_hash: str
    session: str
    sources: list[str]
    phone: str


class DatabaseConfig(msgspec.Struct):
    dsn: str
    username: str
    password: str
    namespace: str
    database: str


class StorageConfig(msgspec.Struct):
    endpoint: str
    id_tenant: str
    region: str
    access_key_id: str
    access_secret: str


class CacheConfig(msgspec.Struct):
    dsn: str


class Config(msgspec.Struct):
    application: ApplicationConfig
    telegram: TelegramConfig
    database: DatabaseConfig
    storage: StorageConfig
    cache: CacheConfig


def load_toml(data: bytes) -> Config:
    return msgspec.toml.decode(data, type=Config, dec_hook=dec_hook)  # type: ignore


def dump_toml(config: Config) -> bytes:
    return msgspec.toml.encode(config, enc_hook=enc_hook)


if __name__ == "__main__":
    with open("config.toml", "rb") as f:
        config = load_toml(f.read())

    logger.debug(f"loaded config {dump_toml(config).decode('utf-8')}")
