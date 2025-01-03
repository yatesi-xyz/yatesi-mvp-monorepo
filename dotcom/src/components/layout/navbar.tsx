import Link from "next/link"
import { ThemeToggle } from "@/components/ui/theme-toggle"
import { Button } from "@/components/ui/button"
import { Search } from "lucide-react"
import { cn } from "@/lib/utils"

export function Navbar() {
  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-16 items-center">
        <div className="flex items-center space-x-4">
          <Link
            href="/"
            className={cn(
              "flex items-center space-x-2",
              "transition-colors hover:text-primary"
            )}
          >
            <Search className="h-6 w-6" />
            <span className="font-bold text-xl">YATESI</span>
          </Link>
        </div>

        {/* Navigation Items */}
        <nav className="flex flex-1 items-center justify-center space-x-6">
          <Link
            href="/how-it-works"
            className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
          >
            How it works?
          </Link>
          <Link
            href="/pricing"
            className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
          >
            Pricing
          </Link>
          <Link
            href="/contacts"
            className="text-sm font-medium text-muted-foreground transition-colors hover:text-primary"
          >
            Contacts
          </Link>
        </nav>

        {/* Right Side Items */}
        <div className="flex items-center space-x-4">
          <ThemeToggle />
          <Button
            size="sm"
            className="font-medium"
          >
            Get Started
          </Button>
        </div>
      </div>
    </header>
  )
}
