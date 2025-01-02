import asyncio

from config import load_toml
from loguru import logger
from scrapper import Scrapper


async def main():
    logger.debug("reading config file")
    with open("config.toml", "rb") as f:
        config = load_toml(f.read())

    scapper = await Scrapper.initialize(config)
    await scapper.run()


if __name__ == "__main__":
    asyncio.run(main())
