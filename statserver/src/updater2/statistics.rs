pub trait Statistic<T>: From<T>
where
    T: Sized,
{
    const SOURCE_TABLE: &'static str;
    const FIELD_NAME: &'static str;

    fn field_name(&self) -> &'static str;
    fn source_table(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct TotalEmojiCount(usize);

impl From<usize> for TotalEmojiCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Statistic<usize> for TotalEmojiCount {
    const SOURCE_TABLE: &'static str = "total_emoji_count";
    const FIELD_NAME: &'static str = "count";

    fn source_table(&self) -> &'static str {
        Self::SOURCE_TABLE
    }

    fn field_name(&self) -> &'static str {
        Self::FIELD_NAME
    }
}
#[derive(Debug, Clone)]
pub struct TotalEmojipackCount(usize);

impl From<usize> for TotalEmojipackCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Statistic<usize> for TotalEmojipackCount {
    const SOURCE_TABLE: &'static str = "total_emojipack_count";
    const FIELD_NAME: &'static str = "count";

    fn source_table(&self) -> &'static str {
        Self::SOURCE_TABLE
    }

    fn field_name(&self) -> &'static str {
        Self::FIELD_NAME
    }
}

#[derive(Debug, Clone)]
pub struct IndexedEmojiCount(usize);

impl From<usize> for IndexedEmojiCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Statistic<usize> for IndexedEmojiCount {
    const SOURCE_TABLE: &'static str = "indexed_emoji_count";
    const FIELD_NAME: &'static str = "count";

    fn source_table(&self) -> &'static str {
        Self::SOURCE_TABLE
    }

    fn field_name(&self) -> &'static str {
        Self::FIELD_NAME
    }
}

#[derive(Debug, Clone)]
pub struct IndexedEmojipackCount(usize);

impl From<usize> for IndexedEmojipackCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Statistic<usize> for IndexedEmojipackCount {
    const SOURCE_TABLE: &'static str = "indexed_emojipack_count";
    const FIELD_NAME: &'static str = "count";

    fn source_table(&self) -> &'static str {
        Self::SOURCE_TABLE
    }

    fn field_name(&self) -> &'static str {
        Self::FIELD_NAME
    }
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub total_emoji_count: TotalEmojiCount,
    pub total_emojipack_count: TotalEmojipackCount,
    pub indexed_emoji_count: IndexedEmojiCount,
    pub indexed_emojipack_count: IndexedEmojipackCount,
}
