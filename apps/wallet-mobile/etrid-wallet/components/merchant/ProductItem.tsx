"use client"

import { Package, Edit, Trash2, AlertCircle } from "lucide-react"
import { Button } from "@/components/ui/button"
import type { Product } from "@/lib/types/merchant"

interface ProductItemProps {
  product: Product
  onEdit?: (product: Product) => void
  onDelete?: (product: Product) => void
  onSelect?: (product: Product) => void
}

export function ProductItem({
  product,
  onEdit,
  onDelete,
  onSelect,
}: ProductItemProps) {
  const isLowStock = product.stock_quantity <= product.low_stock_threshold
  const price = product.sale_price || product.price

  return (
    <div
      className={`glass-strong rounded-lg border border-border overflow-hidden hover:border-accent/50 transition-colors ${
        onSelect ? "cursor-pointer" : ""
      }`}
      onClick={() => onSelect?.(product)}
    >
      <div className="aspect-square bg-accent/10 flex items-center justify-center relative">
        {product.image_url ? (
          <img
            src={product.image_url}
            alt={product.name}
            className="w-full h-full object-cover"
          />
        ) : (
          <Package className="w-12 h-12 text-accent/50" />
        )}

        {product.sale_price && (
          <div className="absolute top-2 right-2 bg-red-500 text-white text-xs font-semibold px-2 py-1 rounded">
            Sale
          </div>
        )}

        {isLowStock && (
          <div className="absolute top-2 left-2 bg-yellow-500/90 text-white text-xs font-semibold px-2 py-1 rounded flex items-center gap-1">
            <AlertCircle className="w-3 h-3" />
            Low Stock
          </div>
        )}
      </div>

      <div className="p-3">
        <div className="mb-2">
          <h3 className="font-semibold text-foreground truncate">
            {product.name}
          </h3>
          {product.category && (
            <p className="text-xs text-muted-foreground">{product.category}</p>
          )}
        </div>

        {product.description && (
          <p className="text-sm text-muted-foreground line-clamp-2 mb-2">
            {product.description}
          </p>
        )}

        <div className="flex items-center justify-between mb-3">
          <div>
            <div className="flex items-center gap-2">
              <span className="text-lg font-bold text-foreground">
                ${price.toFixed(2)}
              </span>
              {product.sale_price && (
                <span className="text-sm text-muted-foreground line-through">
                  ${product.price.toFixed(2)}
                </span>
              )}
            </div>
            {product.sku && (
              <p className="text-xs text-muted-foreground">SKU: {product.sku}</p>
            )}
          </div>

          <div className="text-right">
            <p className="text-sm font-medium text-foreground">
              {product.stock_quantity} in stock
            </p>
          </div>
        </div>

        {(onEdit || onDelete) && (
          <div className="flex items-center gap-2">
            {onEdit && (
              <Button
                variant="outline"
                size="sm"
                className="flex-1"
                onClick={(e) => {
                  e.stopPropagation()
                  onEdit(product)
                }}
              >
                <Edit className="w-3 h-3" />
                Edit
              </Button>
            )}
            {onDelete && (
              <Button
                variant="outline"
                size="sm"
                onClick={(e) => {
                  e.stopPropagation()
                  onDelete(product)
                }}
              >
                <Trash2 className="w-3 h-3" />
              </Button>
            )}
          </div>
        )}
      </div>
    </div>
  )
}
