import asyncio

from config import load_toml
from loguru import logger
from scrapper import Scrapper


async def main():
    log_format = "<green>{time:YYYY-MM-DD HH:mm:ss.SSS zz}</green> | <level>{level: <8}</level> | <yellow>Line {line: >4} ({file}):</yellow> <b>{message}</b>"  # noqa: E501
    logger.add("file.log", level="DEBUG", format=log_format, colorize=False, backtrace=True, diagnose=True)

    logger.debug("reading config file")
    with open("config.toml", "rb") as f:
        config = load_toml(f.read())

    scapper = await Scrapper.initialize(config)
    await scapper.run()


if __name__ == "__main__":
    asyncio.run(main())
