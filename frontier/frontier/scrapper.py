import asyncio
import io
import json
import pickle
from typing import Never, Self

import aiostream
import telethon.tl.functions.messages as tgreq
import telethon.tl.types as tgt
import telethon.tl.types.messages as tgres
from aiostream.core import Streamer
from config import ApplicationMode, Config
from database import Database
from loguru import logger
from redis.asyncio import Redis
from telethon import TelegramClient
from telethon.events import NewMessage
from telethon.tl.custom import Message
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
        premium_emojis_document_ids: list[int] = []

        if message.text:
            logger.debug("searching for emojipack short names in message text using regex")
            emojipacks = extract_emojipack_shortname(message.text)
            logger.debug("found {} emojipacks short names: {}", len(emojipacks), emojipacks)

            logger.debug("fetching emojipacks from emojipacks links {}", emojipacks)
            emojipacks_sets: list[list[int]] = await asyncio.gather(*[
                self._get_emojis_from_emojipack_by_name(emojipack) for emojipack in emojipacks
            ])

            counter = 0
            for pack in emojipacks_sets:
                counter += len(pack)
                premium_emojis_document_ids.extend(pack)

            logger.debug("found {} premium emojis", counter)

        # todo: premium reactions are not linked to emojipacks???
        # if message.reactions:
        #     reactions = message.reactions.results
        #     logger.debug("message contains {} reactions, search for premium emojis among them", len(reactions))

        #     counter = 0
        #     for reaction_count in reactions:
        #         if isinstance(reaction_count.reaction, tgt.ReactionCustomEmoji):
        #             counter += 1
        #             premium_emojis_document_ids.append(reaction_count.reaction.document_id)

        #     logger.debug("found {} premium emojis", counter)

        if message.entities:
            logger.debug("message contains {} entities, search for premium emojis among them", len(message.entities))

            counter = 0
            for entity in message.entities:
                if isinstance(entity, tgt.ReactionCustomEmoji):
                    counter += 1
                    premium_emojis_document_ids.append(entity.document_id)

            logger.debug("found {} premium emojis", counter)

        all_emojies = sorted(set(premium_emojis_document_ids))
        emojipack_ids = await asyncio.gather(*[self._get_parent_emojipack_id(emoji) for emoji in all_emojies])

        logger.info("in total found {} premium emojis from {} emojipacks", len(all_emojies), len(set(emojipack_ids)))
        await self._add_to_known_emojis(all_emojies)

        emojipacks_data = await asyncio.gather(*[
            self._get_emojipack(emojipack_id) if emojipack_id else asyncio.sleep(0, result=None)
            for emojipack_id in emojipack_ids
        ])
        emojipacks_data = [
            (emojipack.set.id, emojipack.set.short_name, emojipack.set.title) if emojipack is not None else None
            for emojipack in emojipacks_data
        ]

        await asyncio.gather(*[
            self.database.create_emojipack(*data) for data in set(emojipacks_data) if data is not None
        ])
        await asyncio.gather(*[
            self.database.create_emoji(emoji_id, emojipack[0], emojipack[2], f"/{emojipack[0]}/{emoji_id}.tgs")
            for emoji_id, emojipack in zip(all_emojies, emojipacks_data, strict=False)
            if emojipack is not None
        ])

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
        return int(await self.cache.get(f"source:{source}:last_processed_id") or 0)

    async def __set_last_processed_message_id(self, source: str, id: int) -> None:
        await self.cache.set(f"source:{source}:last_processed_id", id)

    async def _get_emojis_from_emojipack_by_name(self, short_name: str) -> list[int]:
        try:
            stickerset: tgres.StickerSet = await self.client(
                tgreq.GetStickerSetRequest(tgt.InputStickerSetShortName(short_name), hash=0)
            )  # type: ignore
            await self.cache.set(f"emojipack:{stickerset.set.id}:data", pickle.dumps(stickerset))
            emojis: list[int] = [d.id for d in stickerset.documents]
        except Exception:
            emojis = []

        await self.cache.set(f"emojipack:{stickerset.set.id}:emoji_ids", json.dumps(emojis))
        return emojis

    async def _get_emojipack(self, emojipack_id: int) -> tgres.StickerSet | None:
        try:
            cached = await self.cache.get(f"emojipack:{emojipack_id}:data")
            if cached:
                return pickle.loads(cached)

            stickerset: tgres.StickerSet = await self.client(
                tgreq.GetStickerSetRequest(tgt.InputStickerSetID(emojipack_id, access_hash=0), hash=0)
            )  # type: ignore
            await self.cache.set(f"emojipack:{emojipack_id}:data", stickerset.to_json() or "")

            return stickerset
        except Exception as e:
            logger.error(e)
            return None

    async def _get_emojis_from_emojipack(self, emojipack_id: int) -> list[int]:
        cached = await self.cache.get(f"emojipack:{emojipack_id}:emoji_ids")
        if cached:
            return json.loads(cached)
        try:
            stickerset: tgres.StickerSet | None = await self._get_emojipack(emojipack_id)
            if stickerset:
                emojis: list[int] = [d.id for d in stickerset.documents]
            else:
                emojis = []
        except Exception:
            emojis = []

        await self.cache.set(f"emojipack:{emojipack_id}:emoji_ids", json.dumps(emojis))
        return emojis

    async def _get_parent_emojipack_id(self, emoji_id: int) -> int | None:
        emojipack_id: bytes = await self.cache.get(f"emoji:{emoji_id}:emojipack_id")
        if emojipack_id:
            return int(emojipack_id)

        # todo: should probably cache this
        docs: list[tgt.Document] = await self.client(tgreq.GetCustomEmojiDocumentsRequest([emoji_id]))

        for attribute in docs[0].attributes:
            if isinstance(attribute, tgt.DocumentAttributeCustomEmoji):
                try:
                    # todo: should probably cache this
                    if isinstance(attribute.stickerset, tgt.InputStickerSetID):
                        cached = await self.cache.get(f"emojipack:{attribute.stickerset.id}:data")
                    else:
                        cached = None

                    if cached:
                        stickerset = pickle.loads(cached)
                    else:
                        stickerset: tgres.StickerSet = await self.client(
                            tgreq.GetStickerSetRequest(attribute.stickerset, hash=0)
                        )  # type: ignore

                        await self.cache.set(f"emojipack:{stickerset.set.id}:data", pickle.dumps(stickerset))

                except Exception as e:
                    logger.error("failed while fetching parent emojipack for emoji {}: {}", emoji_id, e)
                    return None

                stickerset_emoji_ids: list[int] = [d.id for d in stickerset.documents]
                await self.cache.mset({
                    f"emoji:{emoji_id}:emojipack_id": stickerset.set.id for emoji_id in stickerset_emoji_ids
                })
                await self.cache.set(f"emojipack:{stickerset.set.id}:emoji_ids", json.dumps(stickerset_emoji_ids))

                return stickerset.set.id

    async def _add_to_known_emojis(self, emojis: list[int]) -> None:
        if emojis:
            await self.cache.sadd("global:emoji_ids", *emojis)  # type: ignore

    async def _download_emojis(self, emoji_ids: list[int]) -> list[bytes]:
        buffers = [io.BytesIO() for _ in range(len(emoji_ids))]
        # get parent sticker set: GetCustomEmojiDocumentsRequest[].attribute(DocumentAttributeCustomEmoji).stickerset
        await asyncio.gather(*[
            self.client.download_media(emoji_ids[i], buffers[i])  # type: ignore
            for i in range(len(emoji_ids))
        ])
        return [buffer.getvalue() for buffer in buffers]
