'use client'

import { Input } from '@/components/ui/input'
import { useState } from 'react'

interface ColorPickerProps {
  label: string
  value: string
  onChange: (color: string) => void
}

export function ColorPicker({ label, value, onChange }: ColorPickerProps) {
  const [hexValue, setHexValue] = useState(value)

  const handleHexChange = (hex: string) => {
    setHexValue(hex)
    // Validate hex color
    if (/^#[0-9A-F]{6}$/i.test(hex)) {
      onChange(hex)
    }
  }

  return (
    <div>
      <label className="text-sm font-medium mb-2 block">{label}</label>
      <div className="flex gap-2">
        <input
          type="color"
          value={value}
          onChange={(e) => {
            onChange(e.target.value)
            setHexValue(e.target.value)
          }}
          className="w-12 h-10 rounded cursor-pointer"
        />
        <Input
          type="text"
          value={hexValue}
          onChange={(e) => handleHexChange(e.target.value)}
          placeholder="#000000"
          className="flex-1"
        />
      </div>
    </div>
  )
}
