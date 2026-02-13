import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import { QueryProvider } from '@/components/providers';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Betting Validation Engine | High Concurrency',
  description: 'Motor de validación de apuestas en tiempo real, diseñado para alta concurrencia con Rust + Next.js',
};

// root layout — Server Component
// envuelve toda la app con el provider de TanStack Query
export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="es">
      <body className={inter.className}>
        <QueryProvider>
          {children}
        </QueryProvider>
      </body>
    </html>
  );
}
