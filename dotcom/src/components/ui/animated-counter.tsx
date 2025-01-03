"use client"

import { animate, useMotionValue, useTransform, motion } from 'framer-motion';
import { useEffect } from 'react';
import { cn } from '@/lib/utils';
import { Skeleton } from '@/components/ui/skeleton';

interface AnimatedCounterProps {
  from: number;
  to: number;
  label: string;
  size?: 'small' | 'large';
  isLoading?: boolean;
}

export function AnimatedCounter({
  from,
  to,
  label,
  size = 'small',
  isLoading = false
}: AnimatedCounterProps) {
  const count = useMotionValue(from);
  const rounded = useTransform(count, latest => Math.round(latest));

  useEffect(() => {
    const controls = animate(count, to, {
      duration: 0.5,
      ease: "easeOut",
    });

    return controls.stop;
  }, [count, to]);

  const numberClass = cn(
    "font-bold text-primary mb-3 tabular-nums",
    size === 'small' && "text-4xl md:text-5xl",
    size === 'large' && "text-6xl md:text-8xl"
  );

  const labelClass = cn(
    "text-muted-foreground",
    size === 'small' && "text-base md:text-lg",
    size === 'large' && "text-lg md:text-xl"
  );

  // Show skeleton when loading or when the number is 0
  if (isLoading || to === 0) {
    return (
      <div className="flex flex-col items-center justify-center text-center">
        <Skeleton
          className={cn(
            "mb-3 rounded-lg",
            size === 'small' && "h-12 md:h-14 w-32 md:w-40",
            size === 'large' && "h-16 md:h-24 w-40 md:w-60"
          )}
        />
        <Skeleton className="h-6 w-24 rounded-lg" />
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center text-center">
      <motion.span className={numberClass}>
        {rounded}
      </motion.span>
      <span className={labelClass}>
        {label}
      </span>
    </div>
  );
}
