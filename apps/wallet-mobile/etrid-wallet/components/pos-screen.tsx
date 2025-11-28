"use client"

import { ArrowLeft, Plus, Minus, ShoppingCart, Trash2, CreditCard, QrCode, Smartphone } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { usePOS } from "@/hooks/merchant/use-pos"
import { useProducts } from "@/hooks/merchant/use-products"
import { POSKeypad } from "@/components/merchant/POSKeypad"
import { ProductItem } from "@/components/merchant/ProductItem"
import { useState } from "react"
import type { Product, PaymentMethod } from "@/lib/types/merchant"

interface POSScreenProps {
  onBack: () => void
}

export function POSScreen({ onBack }: POSScreenProps) {
  const {
    cart,
    addToCart,
    removeFromCart,
    updateQuantity,
    clearCart,
    calculateSubtotal,
    calculateTotal,
    createSale,
  } = usePOS()

  const { products } = useProducts()
  const [showKeypad, setShowKeypad] = useState(false)
  const [customAmount, setCustomAmount] = useState("")
  const [taxRate] = useState(8.5)

  const handleProductSelect = (product: Product) => {
    addToCart({
      product_id: product.id,
      product_name: product.name,
      quantity: 1,
      unit_price: product.sale_price || product.price,
      total: product.sale_price || product.price,
    })
  }

  const handleCheckout = async (paymentMethod: PaymentMethod) => {
    try {
      await createSale({
        tax_rate: taxRate,
        tax_amount: (calculateSubtotal() * taxRate) / 100,
        total: calculateTotal(taxRate),
        payment_method: paymentMethod,
      })
      alert("Sale completed successfully!")
    } catch (error) {
      console.error("Failed to create sale:", error)
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
            <h1 className="text-xl font-bold text-foreground">Point of Sale</h1>
          </div>
          <Button variant="outline" size="sm" onClick={() => setShowKeypad(!showKeypad)}>
            {showKeypad ? "Products" : "Keypad"}
          </Button>
        </div>
      </header>

      <main className="px-4 py-6 space-y-6">
        {showKeypad ? (
          <POSKeypad value={customAmount} onChange={setCustomAmount} />
        ) : (
          <div className="grid grid-cols-2 gap-3">
            {products.slice(0, 10).map((product) => (
              <ProductItem
                key={product.id}
                product={product}
                onSelect={handleProductSelect}
              />
            ))}
          </div>
        )}

        {/* Cart */}
        {cart.length > 0 && (
          <div className="glass-strong rounded-lg border border-border overflow-hidden">
            <div className="p-4 border-b border-border flex items-center justify-between">
              <div className="flex items-center gap-2">
                <ShoppingCart className="w-5 h-5 text-accent" />
                <h3 className="font-semibold text-foreground">Cart ({cart.length})</h3>
              </div>
              <Button variant="ghost" size="sm" onClick={clearCart}>
                <Trash2 className="w-4 h-4" />
                Clear
              </Button>
            </div>

            <div className="divide-y divide-border max-h-60 overflow-y-auto">
              {cart.map((item) => (
                <div key={`${item.product_id}-${item.variant}`} className="p-4">
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex-1">
                      <p className="font-medium text-foreground">{item.product_name}</p>
                      <p className="text-sm text-muted-foreground">
                        ${item.unit_price.toFixed(2)} each
                      </p>
                    </div>
                    <p className="text-lg font-semibold text-foreground">
                      ${item.total.toFixed(2)}
                    </p>
                  </div>

                  <div className="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="icon-sm"
                      onClick={() => {
                        if (item.quantity === 1) {
                          removeFromCart(item.product_id, item.variant)
                        } else {
                          updateQuantity(item.product_id, item.quantity - 1, item.variant)
                        }
                      }}
                    >
                      <Minus className="w-3 h-3" />
                    </Button>
                    <span className="text-sm font-medium text-foreground w-8 text-center">
                      {item.quantity}
                    </span>
                    <Button
                      variant="outline"
                      size="icon-sm"
                      onClick={() =>
                        updateQuantity(item.product_id, item.quantity + 1, item.variant)
                      }
                    >
                      <Plus className="w-3 h-3" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon-sm"
                      onClick={() => removeFromCart(item.product_id, item.variant)}
                    >
                      <Trash2 className="w-3 h-3" />
                    </Button>
                  </div>
                </div>
              ))}
            </div>

            <div className="p-4 bg-accent/5 border-t border-border space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Subtotal</span>
                <span className="text-foreground font-medium">
                  ${calculateSubtotal().toFixed(2)}
                </span>
              </div>
              <div className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Tax ({taxRate}%)</span>
                <span className="text-foreground font-medium">
                  ${((calculateSubtotal() * taxRate) / 100).toFixed(2)}
                </span>
              </div>
              <div className="flex items-center justify-between pt-2 border-t border-border">
                <span className="text-foreground font-semibold">Total</span>
                <span className="text-2xl font-bold text-accent">
                  ${calculateTotal(taxRate).toFixed(2)}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Payment Methods */}
        {cart.length > 0 && (
          <div>
            <h3 className="text-lg font-semibold text-foreground mb-3">
              Payment Method
            </h3>
            <div className="grid grid-cols-3 gap-3">
              <Button
                variant="outline"
                className="flex-col h-20 gap-2"
                onClick={() => handleCheckout("qr_code")}
              >
                <QrCode className="w-5 h-5" />
                <span className="text-xs">Scan QR</span>
              </Button>

              <Button
                variant="outline"
                className="flex-col h-20 gap-2"
                onClick={() => handleCheckout("nfc")}
              >
                <Smartphone className="w-5 h-5" />
                <span className="text-xs">NFC</span>
              </Button>

              <Button
                variant="outline"
                className="flex-col h-20 gap-2"
                onClick={() => handleCheckout("manual")}
              >
                <CreditCard className="w-5 h-5" />
                <span className="text-xs">Manual</span>
              </Button>
            </div>
          </div>
        )}
      </main>
    </div>
  )
}
