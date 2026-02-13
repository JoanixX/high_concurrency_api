'use client';

import { useEffect, useRef, useCallback, useState } from 'react';
import { getSocket, disconnectSocket } from '@/lib/socket';
import type { WSEvent } from '@/types/domain';

interface UseSocketOptions {
  // si se conecta automáticamente al montar
  autoConnect?: boolean;
  // handler para todos los eventos del backend
  onEvent?: (event: WSEvent) => void;
  // handlers de conexión
  onConnect?: () => void;
  onDisconnect?: () => void;
  onError?: (error: Error) => void;
}

export function useSocket(options: UseSocketOptions = {}) {
  const {
    autoConnect = true,
    onEvent,
    onConnect,
    onDisconnect,
    onError,
  } = options;

  const [isConnected, setIsConnected] = useState(false);
  const eventHandlerRef = useRef(onEvent);
  eventHandlerRef.current = onEvent;

  const connect = useCallback(() => {
    const socket = getSocket();
    if (!socket.connected) {
      socket.connect();
    }
  }, []);

  const disconnect = useCallback(() => {
    disconnectSocket();
    setIsConnected(false);
  }, []);

  useEffect(() => {
    const socket = getSocket();

    const handleConnect = () => {
      setIsConnected(true);
      onConnect?.();
    };

    const handleDisconnect = () => {
      setIsConnected(false);
      onDisconnect?.();
    };

    const handleError = (error: Error) => {
      onError?.(error);
    };

    // evento genérico del backend — todos los eventos pasan por acá
    const handleEvent = (event: WSEvent) => {
      eventHandlerRef.current?.(event);
    };

    socket.on('connect', handleConnect);
    socket.on('disconnect', handleDisconnect);
    socket.on('connect_error', handleError);
    socket.on('event', handleEvent);

    if (autoConnect) {
      socket.connect();
    }

    return () => {
      socket.off('connect', handleConnect);
      socket.off('disconnect', handleDisconnect);
      socket.off('connect_error', handleError);
      socket.off('event', handleEvent);
    };
  }, [autoConnect, onConnect, onDisconnect, onError]);

  return { isConnected, connect, disconnect };
}
