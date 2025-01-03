import base64
import textwrap
from typing import Self

import httpx
from config import DatabaseConfig
from loguru import logger


class Database:
    def __init__(self, config: DatabaseConfig, client: httpx.AsyncClient) -> None:
        self.cfg = config
        self.client = client

    @classmethod
    async def initialize(cls, config: DatabaseConfig) -> Self:
        client = httpx.AsyncClient(
            base_url=f"http://{config.dsn}",
            headers={
                "Accept": "application/json",
                "surreal-ns": config.namespace,
                "surreal-db": config.database,
                "Authorization": f"Basic {base64.b64encode(f'{config.username}:{config.password}'.encode()).decode()}",
            },
            # event_hooks={"request": [httpx_log_event_hook]},
        )

        ping_response = await client.post(
            "sql",
            content=textwrap.dedent(
                """
                RETURN 1;
                """,
            ),
        )
        ping_response.raise_for_status()

        return cls(config, client)

    async def create_emoji(self, emoji_id: int, emojipack_id: int, description: str, file: str) -> None:
        description = description.replace("'", "\\'")

        resp = await self.client.post(
            "sql",
            content=textwrap.dedent(
                f"""
                CREATE emoji:{emoji_id} SET
                    code = '{emoji_id}',
                    description = '{description}',
                    file = '{file}',
                    hash = '',
                    pack = emojipack:{emojipack_id};
                """
            ),
            timeout=300,
        )
        logger.debug("database response: {}", resp.json())
        _ = resp.raise_for_status()

    async def create_emojipack(self, emojipack_id: int, short_name: str, description: str) -> None:
        resp = await self.client.post(
            f"key/emojipack/{emojipack_id}",
            json={
                "name": short_name,
                "description": description,
                "hash": "",
            },
        )
        logger.debug("database response: {}", resp.json())
        _ = resp.raise_for_status()
