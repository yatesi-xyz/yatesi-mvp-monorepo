import re
import textwrap

EMOJIPACK_REGEX = re.compile(r"t\.me/addemoji/(?P<shortname>\w+)")


def extract_emojipack_shortname(text: str) -> list[str]:
    results = []
    for match in EMOJIPACK_REGEX.finditer(text):
        results.append(match.group("shortname"))

    return results
