'use client'

import { useState, useEffect } from 'react';
import { fetchStats } from '@/lib/actions';
import type { StatsData } from '@/lib/websocket-server';

const initialData: StatsData = {
  total_emoji_count: 0,
  total_emojipack_count: 0,
  indexed_emoji_count: 0,
  indexed_emojipack_count: 0,
};

export function useStatsData() {
  const [data, setData] = useState<StatsData>(initialData);
  const [previousData, setPreviousData] = useState<StatsData>(initialData);

  useEffect(() => {
    const updateStats = async () => {
      try {
        const newStats = await fetchStats();
        setPreviousData(data);
        setData(newStats);
      } catch (error) {
        console.error('Error updating stats:', error);
      }
    };

    // Initial fetch
    updateStats();

    // Set up interval for updates
    const interval = setInterval(updateStats, 500);

    return () => clearInterval(interval);
  }, [data]);

  return { current: data, previous: previousData };
}
