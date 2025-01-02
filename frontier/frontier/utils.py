import re

from telethon.tl.custom import Message
from telethon.tl.types import Channel, Chat, User

EMOJIPACK_REGEX = re.compile(r"t\.me/addemoji/(?P<shortname>\w+)")


def extract_emojipack_shortname(text: str) -> list[str]:
    results = []
    for match in EMOJIPACK_REGEX.finditer(text):
        results.append(match.group("shortname"))

    return results


def get_message_source_name(message: Message) -> str:  # noqa: C901
    source = message.chat

    if isinstance(source, str):
        return source

    if isinstance(source, int):
        return str(source)

    if isinstance(source, Channel):
        if source.username:
            return source.username
        if source.usernames:
            return source.usernames[0].username
        if message.chat_id:
            return str(message.chat_id)

        return "unknown"

    if isinstance(source, User):
        if source.username:
            return source.username
        if message.chat_id:
            return str(message.chat_id)

        return "unknown"

    if isinstance(source, Chat):
        return str(source.id)

    return str(message.chat_id) if message.chat_id else "unknown"
