"use client"

import { useState } from 'react'
import { posService } from '@/lib/services/POSService'
import type { SaleInput, Sale, CartItem, Discount } from '@/lib/types/merchant'

export function usePOS() {
  const [cart, setCart] = useState<CartItem[]>([])
  const [discount, setDiscount] = useState<Discount | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<Error | null>(null)

  const addToCart = (item: CartItem) => {
    const existingItem = cart.find(
      (i) => i.product_id === item.product_id && i.variant === item.variant
    )

    if (existingItem) {
      setCart(
        cart.map((i) =>
          i.product_id === item.product_id && i.variant === item.variant
            ? {
                ...i,
                quantity: i.quantity + item.quantity,
                total: (i.quantity + item.quantity) * i.unit_price,
              }
            : i
        )
      )
    } else {
      setCart([...cart, item])
    }
  }

  const removeFromCart = (productId: string, variant?: string) => {
    setCart(
      cart.filter(
        (i) => !(i.product_id === productId && i.variant === variant)
      )
    )
  }

  const updateQuantity = (
    productId: string,
    quantity: number,
    variant?: string
  ) => {
    setCart(
      cart.map((i) =>
        i.product_id === productId && i.variant === variant
          ? { ...i, quantity, total: quantity * i.unit_price }
          : i
      )
    )
  }

  const clearCart = () => {
    setCart([])
    setDiscount(null)
  }

  const calculateSubtotal = () => {
    return cart.reduce((sum, item) => sum + item.total, 0)
  }

  const calculateTotal = (taxRate: number = 0) => {
    let subtotal = calculateSubtotal()

    if (discount) {
      if (discount.type === 'percentage') {
        subtotal -= (subtotal * discount.value) / 100
      } else {
        subtotal -= discount.value
      }
    }

    const tax = (subtotal * taxRate) / 100
    return subtotal + tax
  }

  const createSale = async (
    saleInput: Omit<SaleInput, 'items' | 'subtotal'>
  ) => {
    try {
      setLoading(true)
      setError(null)

      const sale: SaleInput = {
        items: cart,
        subtotal: calculateSubtotal(),
        discount: discount || undefined,
        ...saleInput,
      }

      const result = await posService.createSale(sale)
      clearCart()
      return result
    } catch (err) {
      setError(err as Error)
      throw err
    } finally {
      setLoading(false)
    }
  }

  const voidSale = async (saleId: string) => {
    try {
      setError(null)
      await posService.voidSale(saleId)
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  return {
    cart,
    discount,
    loading,
    error,
    addToCart,
    removeFromCart,
    updateQuantity,
    clearCart,
    setDiscount,
    calculateSubtotal,
    calculateTotal,
    createSale,
    voidSale,
  }
}
