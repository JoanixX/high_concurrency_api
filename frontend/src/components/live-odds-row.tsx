'use client';

import React, { useEffect, useRef } from 'react';
import { TableCell, TableRow } from '@/components/ui/table';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { useLiveOdds } from '@/hooks/use-live-odds';
import { useSelectionsStore } from '@/store/selections-store';
import { cn } from '@/lib/utils';
import { Plus, Check } from 'lucide-react';
import type { MatchStatus } from '@/types/domain';
import { MATCH_STATUS } from '@/lib/constants';

interface LiveOddsRowProps {
  matchId: string;
  homeTeam: string;
  awayTeam: string;
  status: MatchStatus;
}

// fila memoizada, este solo re-renderiza cuando cambian los odds de ESTE match
// o cuando cambia su selección en el betting slip
const LiveOddsRow = React.memo(function LiveOddsRow({
  matchId,
  homeTeam,
  awayTeam,
  status,
}: LiveOddsRowProps) {
  const { odds } = useLiveOdds(matchId);
  const rowRef = useRef<HTMLTableRowElement>(null);
  const priceRef = useRef<HTMLSpanElement>(null);
  
  // guardamos referencias mutables sin causar renders
  const prevOddsRef = useRef<number | undefined>(odds);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // efecto flash O(1)
  useEffect(() => {
    // si no hay odds previos o no ha cambiado  no hacemos nada
    if (prevOddsRef.current === undefined || odds === undefined || prevOddsRef.current === odds) {
      prevOddsRef.current = odds;
      return;
    }

    const isUp = odds > prevOddsRef.current;
    
    // Cleanup previo
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    // removemos clases de animaciones previas preventivamente
    rowRef.current?.classList.remove('bg-green-500/10', 'bg-red-500/10');
    priceRef.current?.classList.remove('text-green-500', 'text-red-500');

    // nuevas clases
    const baseColorClass = isUp ? 'bg-green-500/10' : 'bg-red-500/10';
    const textColorClass = isUp ? 'text-green-500' : 'text-red-500';

    rowRef.current?.classList.add(baseColorClass, 'transition-colors', 'duration-75');
    priceRef.current?.classList.add(textColorClass, 'transition-colors', 'duration-75');

    // Cleanup
    timeoutRef.current = setTimeout(() => {
      rowRef.current?.classList.remove(baseColorClass);
      priceRef.current?.classList.remove(textColorClass);
      // forzar que vuelvan sutiles usando clases default 
      // con el tailwind base del componente
    }, 500);

    // guardamos el valor
    prevOddsRef.current = odds;

    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, [odds]);

  const selection = useSelectionsStore((s) => s.selection);
  const setSelection = useSelectionsStore((s) => s.setSelection);
  const clearSelection = useSelectionsStore((s) => s.clearSelection);

  const isSelected = selection?.matchId === matchId;

  const handleToggleSelection = () => {
    if (isSelected) {
      clearSelection();
    } else if (odds !== undefined) {
      setSelection({
        matchId,
        homeTeam,
        awayTeam,
        odds,
        amount: 10, // Monto default
      });
    }
  };

  const statusBadgeVariant = status === MATCH_STATUS.IN_PLAY
    ? 'default'
    : status === MATCH_STATUS.SUSPENDED
      ? 'destructive'
      : 'secondary';

  return (
    <TableRow ref={rowRef}>
      <TableCell className="font-medium">
        <div className="flex flex-col">
          <span>{homeTeam}</span>
          <span className="text-xs text-muted-foreground">vs</span>
          <span>{awayTeam}</span>
        </div>
      </TableCell>

      <TableCell>
        <Badge variant={statusBadgeVariant} className="capitalize">
          {status === MATCH_STATUS.IN_PLAY && (
            <span className="mr-1 inline-block h-2 w-2 animate-pulse rounded-full bg-green-400" />
          )}
          {status}
        </Badge>
      </TableCell>

      <TableCell className="text-right tabular-nums text-lg font-bold">
        <span ref={priceRef}>
          {odds?.toFixed(2) ?? '—'}
        </span>
      </TableCell>

      <TableCell className="text-right">
        <Button
          variant={isSelected ? 'default' : 'outline'}
          size="sm"
          onClick={handleToggleSelection}
          disabled={status !== MATCH_STATUS.IN_PLAY || odds === undefined}
        >
          {isSelected ? (
            <>
              <Check className="mr-1 h-4 w-4" />
              Agregado
            </>
          ) : (
            <>
              <Plus className="mr-1 h-4 w-4" />
              Apostar
            </>
          )}
        </Button>
      </TableCell>
    </TableRow>
  );
});

export { LiveOddsRow };
export type { LiveOddsRowProps };