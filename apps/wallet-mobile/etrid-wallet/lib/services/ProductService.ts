import type { ProductInput, Product } from '@/lib/types/merchant'

export class ProductService {
  private baseUrl = '/api/products'

  async addProduct(product: ProductInput): Promise<Product> {
    try {
      const response = await fetch(this.baseUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(product),
      })

      if (!response.ok) {
        throw new Error('Failed to add product')
      }

      return await response.json()
    } catch (error) {
      console.error('Error adding product:', error)
      throw error
    }
  }

  async getProducts(category?: string): Promise<Product[]> {
    try {
      const url = category
        ? `${this.baseUrl}?category=${category}`
        : this.baseUrl
      const response = await fetch(url)

      if (!response.ok) {
        throw new Error('Failed to fetch products')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching products:', error)
      throw error
    }
  }

  async getProduct(id: string): Promise<Product> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`)

      if (!response.ok) {
        throw new Error('Failed to fetch product')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching product:', error)
      throw error
    }
  }

  async updateProduct(
    id: string,
    updates: Partial<Product>
  ): Promise<Product> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(updates),
      })

      if (!response.ok) {
        throw new Error('Failed to update product')
      }

      return await response.json()
    } catch (error) {
      console.error('Error updating product:', error)
      throw error
    }
  }

  async deleteProduct(id: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}`, {
        method: 'DELETE',
      })

      if (!response.ok) {
        throw new Error('Failed to delete product')
      }
    } catch (error) {
      console.error('Error deleting product:', error)
      throw error
    }
  }

  async updateStock(id: string, quantity: number): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/${id}/stock`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ quantity }),
      })

      if (!response.ok) {
        throw new Error('Failed to update stock')
      }
    } catch (error) {
      console.error('Error updating stock:', error)
      throw error
    }
  }

  async getLowStockProducts(): Promise<Product[]> {
    try {
      const response = await fetch(`${this.baseUrl}/low-stock`)

      if (!response.ok) {
        throw new Error('Failed to fetch low stock products')
      }

      return await response.json()
    } catch (error) {
      console.error('Error fetching low stock products:', error)
      throw error
    }
  }

  async searchProducts(query: string): Promise<Product[]> {
    try {
      const response = await fetch(
        `${this.baseUrl}/search?q=${encodeURIComponent(query)}`
      )

      if (!response.ok) {
        throw new Error('Failed to search products')
      }

      return await response.json()
    } catch (error) {
      console.error('Error searching products:', error)
      throw error
    }
  }
}

export const productService = new ProductService()
