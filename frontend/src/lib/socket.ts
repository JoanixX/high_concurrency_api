import { io, type Socket } from 'socket.io-client';
import { WS_BASE_URL } from './constants';
import type { WSEvent } from '@/types/domain';

// singleton del socket
// Se inicializa una sola vez
let socket: Socket | null = null;

export function getSocket(): Socket {
  if (!socket) {
    socket = io(WS_BASE_URL, {
      // Dejamos que el hook controle
      autoConnect: false,
      // reconexión automática con backoff exponencial
      reconnection: true,
      reconnectionAttempts: 10,
      reconnectionDelay: 1000,
      reconnectionDelayMax: 5000,
      // transporte: intentar websocket primero, fallback a polling
      transports: ['websocket', 'polling'],
    });
  }
  return socket;
}

// Limpia la conexión (para logout o unmount)
export function disconnectSocket(): void {
  if (socket) {
    socket.disconnect();
    socket = null;
  }
}

// tipo helper para listeners tipados
export type WSEventHandler = (event: WSEvent) => void;