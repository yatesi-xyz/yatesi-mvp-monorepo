'use client'

import { env } from '@/config/env';
import { useState, useEffect, useRef } from 'react';

interface StatsData {
  total_emoji_count: number;
  total_emojipack_count: number;
  indexed_emoji_count: number;
  indexed_emojipack_count: number;
}

const initialData: StatsData = {
  total_emoji_count: 0,
  total_emojipack_count: 0,
  indexed_emoji_count: 0,
  indexed_emojipack_count: 0,
};



export function useWebSocketData() {
  const [data, setData] = useState<StatsData>(initialData);
  const [previousData, setPreviousData] = useState<StatsData>(initialData);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>();

  useEffect(() => {
    function connect() {
      if (wsRef.current?.readyState !== WebSocket.OPEN) {
        wsRef.current = new WebSocket(env.websocket.url);

        wsRef.current.onmessage = (event) => {
          try {
            const newData: StatsData = JSON.parse(event.data);
            // Ignore if all zeros
            if (!Object.values(newData).every(val => val === 0)) {
              setPreviousData(data);
              setData(newData);
            }
          } catch (error) {
            console.error('WebSocket message error:', error);
          }
        };

        wsRef.current.onclose = () => {
          if (!reconnectTimeoutRef.current) {
            reconnectTimeoutRef.current = setTimeout(() => {
              reconnectTimeoutRef.current = undefined;
              connect();
            }, 1000);
          }
        };

        wsRef.current.onerror = (error) => {
          console.error('WebSocket error:', error);
          wsRef.current?.close();
        };
      }
    }

    connect();

    return () => {
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (wsRef.current) {
        wsRef.current.close();
        wsRef.current = null;
      }
    };
  }, [data]);

  return { current: data, previous: previousData };
}
