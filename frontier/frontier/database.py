import base64
import textwrap
from typing import Self

import httpx
from config import DatabaseConfig


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
                "NS": config.namespace,
                "DB": config.database,
                "Authorization": f"Basic {base64.b64encode(f'{config.username}:{config.password}'.encode()).decode()}",
            },
        )

        ping_response = await client.post(
            "sql",
            content=textwrap.dedent(
                f"""
                USE NS {config.namespace};
                USE DB {config.database};
                RETURN 1;
                """,
            ),
        )
        ping_response.raise_for_status()

        return cls(config, client)
