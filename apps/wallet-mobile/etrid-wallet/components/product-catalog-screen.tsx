"use client"

import { ArrowLeft, Plus, Package } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useProducts } from "@/hooks/merchant/use-products"
import { ProductItem } from "@/components/merchant/ProductItem"

interface ProductCatalogScreenProps {
  onBack: () => void
  onCreate: () => void
  onEdit: (product: any) => void
}

export function ProductCatalogScreen({ onBack, onCreate, onEdit }: ProductCatalogScreenProps) {
  const { products, loading, deleteProduct } = useProducts()

  const handleDelete = async (product: any) => {
    if (confirm(`Delete ${product.name}?`)) {
      try {
        await deleteProduct(product.id)
      } catch (error) {
        console.error("Failed to delete product:", error)
      }
    }
  }

  return (
    <div className="min-h-screen pb-24">
      <header className="glass-strong border-b border-border sticky top-0 z-10">
        <div className="flex items-center justify-between p-4">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="icon" onClick={onBack}>
              <ArrowLeft className="w-5 h-5" />
            </Button>
            <h1 className="text-xl font-bold text-foreground">Products</h1>
          </div>
          <Button size="sm" onClick={onCreate}>
            <Plus className="w-4 h-4" />
            Add Product
          </Button>
        </div>
      </header>

      <main className="px-4 py-6">
        {loading ? (
          <p className="text-center text-muted-foreground">Loading...</p>
        ) : products.length === 0 ? (
          <div className="glass-strong rounded-lg p-8 border border-border text-center">
            <Package className="w-12 h-12 text-muted-foreground mx-auto mb-3" />
            <p className="text-foreground font-medium mb-1">No products yet</p>
            <p className="text-sm text-muted-foreground mb-4">
              Add your first product to start selling
            </p>
            <Button onClick={onCreate}>
              <Plus className="w-4 h-4" />
              Add Product
            </Button>
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-4">
            {products.map((product) => (
              <ProductItem
                key={product.id}
                product={product}
                onEdit={onEdit}
                onDelete={handleDelete}
              />
            ))}
          </div>
        )}
      </main>
    </div>
  )
}
