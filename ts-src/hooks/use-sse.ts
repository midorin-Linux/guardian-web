import { useEffect, useRef, useState, useCallback } from 'react';

interface UseSSEReturn<T> {
    data: T | null;
    error: Event | null;
    isConnected: boolean;
    reconnect: () => void;
}

export function useSSE<T = any>(
    url: string,
    options?: {
        withCredentials?: boolean;
        headers?: Record<string, string>;
        onOpen?: () => void;
        onError?: (event: Event) => void;
    }
): UseSSEReturn<T> {
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<Event | null>(null);
    const [isConnected, setIsConnected] = useState(false);
    const eventSourceRef = useRef<EventSource | null>(null);

    const connect = useCallback(() => {
        if (eventSourceRef.current) {
            eventSourceRef.current.close();
        }

        const es = new EventSource(url, {
            withCredentials: options?.withCredentials ?? false,
        });

        es.onopen = () => {
            setIsConnected(true);
            setError(null);
            options?.onOpen?.();
        };

        es.onmessage = (event: MessageEvent) => {
            try {
                const parsed = JSON.parse(event.data) as T;
                setData(parsed);
            } catch (e) {
                setData(event.data as any);
            }
        };

        es.onerror = (event: Event) => {
            setIsConnected(false);
            setError(event);
            options?.onError?.(event);
        };

        eventSourceRef.current = es;
    }, [url, options]);

    const reconnect = useCallback(() => {
        connect();
    }, [connect]);

    useEffect(() => {
        connect();

        return () => {
            if (eventSourceRef.current) {
                eventSourceRef.current.close();
                eventSourceRef.current = null;
            }
        };
    }, [connect]);

    return { data, error, isConnected, reconnect };
}