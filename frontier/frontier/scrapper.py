import asyncio
import io
import json
from typing import Never, Self

import aiostream
from aiostream.core import Streamer
from config import ApplicationMode, Config
from database import Database
from loguru import logger
from redis.asyncio import Redis
from telethon import TelegramClient
from telethon.events import NewMessage
from telethon.tl.custom import Message
from telethon.tl.functions.messages import GetCustomEmojiDocumentsRequest, GetStickerSetRequest
from telethon.tl.types import Document, InputStickerSetShortName, ReactionCustomEmoji, StickerSet
from utils import extract_emojipack_shortname, get_message_source_name


class Scrapper:
    def __init__(self, config: Config, client: TelegramClient, cache: Redis, database: Database) -> None:
        self.cfg = config
        self.client = client
        self.cache = cache
        self.database = database

    @classmethod
    async def initialize(cls, config: Config) -> Self:
        logger.debug("initializing scrapper")

        logger.debug("initializing telegram client")
        client: TelegramClient = await TelegramClient(
            config.telegram.session_file, config.telegram.api_id, config.telegram.api_hash
        ).start(phone=config.telegram.phone)  # type: ignore

        logger.debug("initializing redis cache")
        cache = Redis.from_url(config.cache.dsn)
        await cache.ping()

        logger.debug("initializing database")
        database = await Database.initialize(config.database)

        return cls(config, client, cache, database)

    async def run(self) -> Never:
        if self.cfg.application.mode == ApplicationMode.LISTEN:
            await self._listen_mode()
        elif self.cfg.application.mode == ApplicationMode.SCRAPE:
            await self._scrape_mode()

    async def _listen_mode(self) -> Never:
        logger.info("starting listen mode")

        @self.client.on(NewMessage(chats=self.cfg.telegram.sources))
        async def handler(event: NewMessage.Event):
            await self._process_message(event.message)

        await self.client.run_until_disconnected()  # type: ignore

    async def _scrape_mode(self) -> Never:
        async with self._stream_messages_from_sources(self.cfg.telegram.sources) as stream:
            async for message in stream:
                await self._process_message(message)

    async def _process_message(self, message: Message):
        logger.info("processing message {}", message.pretty_format(message))

        premium_emojis_document_ids: list[int] = []

        if message.text:
            logger.debug("searching for emojipack short names in message text using regex")
            emojipacks = extract_emojipack_shortname(message.text)
            logger.debug("found {} emojipacks short names: {}", len(emojipacks), emojipacks)

            logger.debug("fetching emojipacks from emojipacks links {}", emojipacks)
            emojipacks_sets: list[list[int]] = await asyncio.gather(*[
                self.__get_emojis_from_emojipack(emojipack) for emojipack in emojipacks
            ])

            counter = 0
            for pack in emojipacks_sets:
                counter += len(pack)
                premium_emojis_document_ids.extend(pack)

            logger.debug("found {} premium emojis", counter)

        if message.reactions:
            reactions = message.reactions.results
            logger.debug("message contains {} reactions, search for premium emojis among them", len(reactions))

            counter = 0
            for reaction_count in reactions:
                if isinstance(reaction_count.reaction, ReactionCustomEmoji):
                    counter += 1
                    premium_emojis_document_ids.append(reaction_count.reaction.document_id)

            logger.debug("found {} premium emojis", counter)

        if message.entities:
            logger.debug("message contains {} entities, search for premium emojis among them", len(message.entities))

            counter = 0
            for entity in message.entities:
                if isinstance(entity, ReactionCustomEmoji):
                    counter += 1
                    premium_emojis_document_ids.append(entity.document_id)

            logger.debug("found {} premium emojis", counter)

        all_emojies = sorted(set(premium_emojis_document_ids))
        logger.info("in total found {} premium emojis from message {}", len(all_emojies), message.id)
        await self.__add_to_known_emojis(all_emojies)

        source = get_message_source_name(message)
        await self.__set_last_processed_message_id(source, message.id)

    def _stream_messages_from_sources(self, sources: list[str]) -> Streamer[Message]:
        return aiostream.stream.merge(*[
            self._stream_messages_from_single_source(source) for source in sources
        ]).stream()  # type: ignore

    async def _stream_messages_from_single_source(self, source: str) -> Streamer[Message]:  # type: ignore
        last_id = await self.__get_last_processed_message_id(source)
        message_iterator = self.client.iter_messages(
            source,
            offset_id=last_id,
            reverse=True,
        )
        async for message in message_iterator:
            yield message  # type: ignore
            logger.debug("cooldown {:.2f}s for chat {}", self.cfg.application.cooldown.inner.total_seconds(), source)
            await asyncio.sleep(self.cfg.application.cooldown.inner.total_seconds())

    async def __get_last_processed_message_id(self, source: str) -> int:
        return int(await self.cache.get(source + ".last_processed_id") or 0)

    async def __set_last_processed_message_id(self, source: str, id: int) -> None:
        await self.cache.set(source + ".last_processed_id", id)

    async def __get_emojis_from_emojipack(self, short_name: str) -> list[int]:
        cached = await self.cache.get(short_name + ".emojis")
        if cached:
            return json.loads(cached)

        stickerset: StickerSet = await self.client(GetStickerSetRequest(InputStickerSetShortName(short_name), hash=0))  # type: ignore
        docs: list[Document] = stickerset.documents  # type: ignore
        emojis: list[int] = [d.id for d in docs]
        await self.cache.set(short_name + ".emojis", json.dumps(emojis))
        return emojis

    async def __add_to_known_emojis(self, emojis: list[int]) -> None:
        if emojis:
            await self.cache.sadd("known_emojis_ids", *emojis)  # type: ignore

    async def __download_emojis(self, emoji_ids: list[int]) -> list[bytes]:
        buffers = [io.BytesIO() for _ in range(len(emoji_ids))]
        # get parent sticker set: GetCustomEmojiDocumentsRequest[].attribute(DocumentAttributeCustomEmoji).stickerset
        await asyncio.gather(*[
            self.client.download_media(emoji_ids[i], buffers[i])  # type: ignore
            for i in range(len(emoji_ids))
        ])
        return [buffer.getvalue() for buffer in buffers]
