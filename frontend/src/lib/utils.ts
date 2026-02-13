import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

// Combina clases condicionales con tailwind merge (evita conflictos)
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// formatea latencia para mostrar en el dashboard
export function formatLatency(ms: number): string {
  return ms < 1 ? '<1ms' : `${ms.toFixed(1)}ms`;
}

// Genera un UUID v4 usando crypto del browser
export function generateUUID(): string {
  return crypto.randomUUID();
}
