import re

from loguru import logger
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


def log_request(request):
    logger.info(f"Request: {request.method} {request.url}")
    logger.debug(f"Request headers: {request.headers}")
    if request.content:
        logger.debug(f"Request body: {request.content}")


def log_response(response):
    logger.info(f"Response: {response.status_code} {response.reason_phrase}")
    logger.debug(f"Response headers: {response.headers}")
    logger.debug(f"Response body: {response.text}")
    logger.info(f"Response time: {response.elapsed.total_seconds():.2f}s")


async def httpx_log_event_hook(request):
    log_request(request)

    async def response_hook(response):
        log_response(response)
        return response

    return response_hook
