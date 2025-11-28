"use client"

import { useState, useEffect } from 'react'
import { productService } from '@/lib/services/ProductService'
import type { Product, ProductInput } from '@/lib/types/merchant'

export function useProducts(category?: string) {
  const [products, setProducts] = useState<Product[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  const fetchProducts = async () => {
    try {
      setLoading(true)
      setError(null)
      const data = await productService.getProducts(category)
      setProducts(data)
    } catch (err) {
      setError(err as Error)
    } finally {
      setLoading(false)
    }
  }

  const addProduct = async (product: ProductInput) => {
    try {
      setError(null)
      const newProduct = await productService.addProduct(product)
      setProducts([...products, newProduct])
      return newProduct
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateProduct = async (id: string, updates: Partial<Product>) => {
    try {
      setError(null)
      const updated = await productService.updateProduct(id, updates)
      setProducts(products.map((p) => (p.id === id ? updated : p)))
      return updated
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const deleteProduct = async (id: string) => {
    try {
      setError(null)
      await productService.deleteProduct(id)
      setProducts(products.filter((p) => p.id !== id))
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const updateStock = async (id: string, quantity: number) => {
    try {
      setError(null)
      await productService.updateStock(id, quantity)
      setProducts(
        products.map((p) =>
          p.id === id ? { ...p, stock_quantity: quantity } : p
        )
      )
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  const searchProducts = async (query: string) => {
    try {
      setError(null)
      const results = await productService.searchProducts(query)
      return results
    } catch (err) {
      setError(err as Error)
      throw err
    }
  }

  useEffect(() => {
    fetchProducts()
  }, [category])

  return {
    products,
    loading,
    error,
    refetch: fetchProducts,
    addProduct,
    updateProduct,
    deleteProduct,
    updateStock,
    searchProducts,
  }
}
