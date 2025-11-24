import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"

/**
 * Combines class names using clsx and tailwind-merge
 * Ensures Tailwind classes don't conflict
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
