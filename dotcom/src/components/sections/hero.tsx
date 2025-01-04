'use client';

import { useWebSocketData } from '@/lib/hooks/useWebSocketData';
import { AnimatedCounter } from '@/components/ui/animated-counter';
import { Button } from '@/components/ui/button';
import { BackgroundBeams } from '@/components/ui/background-beams';
import { Spotlight } from "@/components/ui/spotlight";
import { ThemeToggle } from '../ui/theme-toggle';
export function Hero() {
  const { current, previous } = useWebSocketData();

  return (
    <section className="h-screen w-full overflow-hidden relative antialiased bg-background">
      <BackgroundBeams className="opacity-20" />
      <Spotlight
        className="-top-40 left-0 md:left-60 md:-top-20"
        fill="rgba(255, 255, 255, 0.1)"
      />

      <div className="relative z-10 h-full w-full container mx-auto grid grid-cols-1 md:grid-cols-2 items-center -mt-16">
        {/* Left Column */}
        <div className="flex flex-col justify-center items-center text-center h-full">
          <div className="max-w-2xl flex flex-col items-center">
            <h1 className="text-6xl md:text-8xl font-black text-foreground">
              YATESI
            </h1>

            <p className="text-xl md:text-2xl text-muted-foreground mt-8 mb-12">
              The <span className="font-bold">smartest ever</span> Telegram Emoji Search
            </p>

            <div className="flex flex-col sm:flex-row gap-4 w-full justify-center">
              <Button
                size="lg"
                variant="outline"
                className="font-semibold backdrop-blur-sm"
              >
                Get Started
              </Button>
              <Button
                size="lg"
                variant="outline"
                className="font-semibold backdrop-blur-sm"
              >
                How it works?
              </Button>
              <ThemeToggle />
            </div>
          </div>
        </div>

        {/* Right Column - Stats */}
        <div className="flex flex-col justify-center items-center h-full">
          <div className="w-full flex flex-col items-center justify-center space-y-12">
            <AnimatedCounter
              from={previous.total_emojipack_count}
              to={current.total_emojipack_count}
              label="Total Emojipacks Known"
              size="small"
            />

            <AnimatedCounter
              from={previous.total_emoji_count}
              to={current.total_emoji_count}
              label="Total Emoji Count"
              size="large"
            />

            <AnimatedCounter
              from={previous.indexed_emoji_count}
              to={current.indexed_emoji_count}
              label="Indexed Emoji Count"
              size="small"
            />
          </div>
        </div>
      </div>
    </section>
  );
}
