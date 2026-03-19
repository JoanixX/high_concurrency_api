'use client';

import { useAuthStore } from '@/store/auth-store';
import { useUserHistory } from '@/hooks/use-user-history';
import { BET_STATUS } from '@/lib/constants';
import { Badge } from '@/components/ui/badge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { History } from 'lucide-react';
import type { BetStatus } from '@/types/domain';

// formateador de moneda instanciado una sola vez fuera del render
const currencyFmt = new Intl.NumberFormat('en-US', {
  style: 'currency',
  currency: 'USD',
  minimumFractionDigits: 2,
});

// convertimos centavos enteros a string
function fmtCents(cents: number): string {
  return currencyFmt.format(cents / 100);
}

// convertimos milésimas a cuota decimal (1500 → "1.50")
function fmtOdds(millis: number): string {
  return (millis / 1000).toFixed(2);
}

// convertimos ISO 8601 a fecha corta legible
function fmtDate(iso: string): string {
  return new Date(iso).toLocaleDateString('es-PE', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function statusClasses(status: BetStatus) {
  switch (status) {
    case BET_STATUS.WON:
      return 'text-emerald-500 font-bold';
    case BET_STATUS.LOST:
      return 'line-through text-muted-foreground';
    default:
      return '';
  }
}

function StatusBadge({ status }: { status: BetStatus }) {
  switch (status) {
    case BET_STATUS.WON:
      return <Badge className="bg-emerald-500/15 text-emerald-500 hover:bg-emerald-500/25">Ganada</Badge>;
    case BET_STATUS.LOST:
      return <Badge variant="secondary" className="text-muted-foreground">Perdida</Badge>;
    case BET_STATUS.ACCEPTED:
      return <Badge className="bg-amber-500/15 text-amber-600 hover:bg-amber-500/25">Aceptada</Badge>;
    case BET_STATUS.PENDING:
      return <Badge className="bg-orange-500/15 text-orange-500 hover:bg-orange-500/25">Pendiente</Badge>;
    default:
      return <Badge variant="outline">{status}</Badge>;
  }
}

export default function HistoryPage() {
  const userId = useAuthStore((s) => s.userId);
  const { data: history = [], isLoading } = useUserHistory(userId);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="flex items-center gap-2 text-2xl font-bold tracking-tight">
          <History className="h-6 w-6" />
          Historial de Apuestas
        </h1>
        <p className="text-sm text-muted-foreground">
          Registro completo de apuestas resolutas y pendientes
        </p>
      </div>

      {isLoading ? (
        <div className="flex h-48 items-center justify-center rounded-lg border border-dashed bg-muted/20">
          <span className="animate-pulse text-muted-foreground">Cargando historial...</span>
        </div>
      ) : history.length === 0 ? (
        <div className="flex h-48 items-center justify-center rounded-lg border border-dashed bg-muted/20">
          <span className="text-muted-foreground">No hay apuestas registradas aún.</span>
        </div>
      ) : (
        <div className="rounded-lg border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Fecha</TableHead>
                <TableHead>Partido</TableHead>
                <TableHead>Selección</TableHead>
                <TableHead className="text-right">Cuota</TableHead>
                <TableHead className="text-right">Monto</TableHead>
                <TableHead className="text-center">Estado</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {history.map((entry) => (
                <TableRow key={entry.bet_id}>
                  <TableCell className="text-sm text-muted-foreground whitespace-nowrap">
                    {fmtDate(entry.created_at)}
                  </TableCell>
                  <TableCell className="font-medium">
                    {entry.home_team} vs {entry.away_team}
                  </TableCell>
                  <TableCell className="capitalize">
                    {entry.selection}
                  </TableCell>
                  <TableCell className="text-right tabular-nums">
                    {fmtOdds(entry.odds)}
                  </TableCell>
                  <TableCell className={`text-right tabular-nums ${statusClasses(entry.status)}`}>
                    {fmtCents(entry.amount)}
                  </TableCell>
                  <TableCell className="text-center">
                    <StatusBadge status={entry.status} />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}
    </div>
  );
}