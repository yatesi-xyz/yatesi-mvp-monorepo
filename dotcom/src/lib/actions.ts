'use server'

import { getStatsData, type StatsData } from './websocket-server';

export async function fetchStats(): Promise<StatsData> {
  try {
    const stats = await getStatsData();
    return stats;
  } catch (error) {
    console.error('Error fetching stats:', error);
    return {
      total_emoji_count: 0,
      total_emojipack_count: 0,
      indexed_emoji_count: 0,
      indexed_emojipack_count: 0,
    };
  }
}
