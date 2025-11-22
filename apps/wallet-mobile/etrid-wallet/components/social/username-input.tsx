/**
 * Username Input Component
 * Real-time username availability checker with validation
 */

'use client';

import { useState, useEffect } from 'react';
import { Check, X, Loader2 } from 'lucide-react';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import { useUsername } from '@/hooks/useUsername';

interface UsernameInputProps {
  value: string;
  onChange: (value: string) => void;
  onValidationChange?: (isValid: boolean) => void;
}

export function UsernameInput({ value, onChange, onValidationChange }: UsernameInputProps) {
  const { availability, pricing, isChecking, checkAvailability, validateUsername } = useUsername();
  const [debouncedValue, setDebouncedValue] = useState(value);

  // Debounce username input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedValue(value);
    }, 500);

    return () => clearTimeout(timer);
  }, [value]);

  // Check availability when debounced value changes
  useEffect(() => {
    if (debouncedValue && debouncedValue.length > 0) {
      const validation = validateUsername(debouncedValue);
      if (validation.valid) {
        checkAvailability(debouncedValue);
      }
    }
  }, [debouncedValue, checkAvailability, validateUsername]);

  // Notify parent of validation state
  useEffect(() => {
    if (onValidationChange) {
      const validation = validateUsername(value);
      const isValid = validation.valid && availability?.available === true;
      onValidationChange(isValid);
    }
  }, [value, availability, validateUsername, onValidationChange]);

  const validation = validateUsername(value);
  const showStatus = value.length > 0 && !isChecking;
  const isAvailable = availability?.available === true;
  const isValid = validation.valid && isAvailable;

  return (
    <div className="space-y-3">
      {/* Input Field */}
      <div className="relative">
        <Input
          type="text"
          value={value}
          onChange={(e) => onChange(e.target.value.toLowerCase())}
          placeholder="username"
          className="pr-24 font-mono"
          maxLength={63}
        />

        {/* Suffix */}
        <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2">
          <span className="text-sm text-muted-foreground">.etrid</span>

          {/* Status Icon */}
          {isChecking && <Loader2 className="w-4 h-4 animate-spin text-muted-foreground" />}
          {showStatus && validation.valid && isAvailable && (
            <Check className="w-4 h-4 text-green-500" />
          )}
          {showStatus && (!validation.valid || !isAvailable) && (
            <X className="w-4 h-4 text-destructive" />
          )}
        </div>
      </div>

      {/* Validation Messages */}
      {value.length > 0 && (
        <div className="space-y-2">
          {/* Format Validation */}
          {!validation.valid && (
            <p className="text-sm text-destructive">{validation.error}</p>
          )}

          {/* Availability Status */}
          {validation.valid && availability && (
            <div>
              {isAvailable ? (
                <p className="text-sm text-green-600 dark:text-green-400">
                  ✓ {value}.etrid is available!
                </p>
              ) : (
                <div className="space-y-2">
                  <p className="text-sm text-destructive">
                    ✗ {value}.etrid is not available
                  </p>

                  {/* Suggestions */}
                  {availability.suggestions && availability.suggestions.length > 0 && (
                    <div className="space-y-1">
                      <p className="text-xs text-muted-foreground">Try these instead:</p>
                      <div className="flex flex-wrap gap-2">
                        {availability.suggestions.map((suggestion) => (
                          <Badge
                            key={suggestion}
                            variant="outline"
                            className="cursor-pointer hover:bg-accent"
                            onClick={() => onChange(suggestion)}
                          >
                            {suggestion}.etrid
                          </Badge>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          )}

          {/* Pricing */}
          {validation.valid && isAvailable && pricing && (
            <div className="flex items-center justify-between p-3 rounded-lg bg-accent/10 border border-border">
              <div className="space-y-1">
                <p className="text-sm font-medium">Registration Price</p>
                <p className="text-xs text-muted-foreground">
                  {pricing.length} characters • {pricing.tier} tier
                </p>
              </div>
              <div className="text-right">
                <p className="text-lg font-bold text-accent">{pricing.price} ÉTR</p>
                <p className="text-xs text-muted-foreground">per year</p>
              </div>
            </div>
          )}
        </div>
      )}

      {/* Character Counter */}
      <div className="flex justify-between items-center text-xs text-muted-foreground">
        <span>Alphanumeric + hyphen (1-63 chars)</span>
        <span>{value.length}/63</span>
      </div>
    </div>
  );
}
