import WebSocket from 'ws';
import { env } from '@/config/env';

export interface StatsData {
  total_emoji_count: number;
  total_emojipack_count: number;
  indexed_emoji_count: number;
  indexed_emojipack_count: number;
}

let ws: WebSocket | null = null;

export function getWebSocketConnection() {
  if (!ws || ws.readyState === WebSocket.CLOSED) {
    ws = new WebSocket(env.websocket.url);
  }
  return ws;
}

export async function getStatsData(): Promise<StatsData> {
  return new Promise((resolve, reject) => {
    const ws = getWebSocketConnection();

    const handleMessage = (data: WebSocket.Data) => {
      try {
        const stats: StatsData = JSON.parse(data.toString());
        if (!Object.values(stats).every(val => val === 0)) {
          ws.removeListener('message', handleMessage);
          resolve(stats);
        }
      } catch (error) {
        reject(error);
      }
    };

    ws.on('message', handleMessage);
    ws.on('error', reject);
  });
}
