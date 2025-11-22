// Merchant Account Types

export interface MerchantInfo {
  business_name: string
  category: string
  description?: string
  logo_url?: string
  contact_email: string
  contact_phone?: string
}

export interface MerchantAccount {
  id: string
  business_name: string
  category: string
  description?: string
  logo_url?: string
  contact_email: string
  contact_phone?: string
  wallet_address: string
  merchant_code: string
  created_at: Date
  updated_at: Date
}

export interface SalesStats {
  total_sales: number
  sales_change: number
  transaction_count: number
  transaction_change: number
  average_sale: number
  top_products: Array<{
    product_id: string
    product_name: string
    quantity_sold: number
    revenue: number
  }>
  sales_by_day: Array<{
    date: string
    sales: number
    transactions: number
  }>
}

// Product Types

export interface ProductInput {
  name: string
  description?: string
  price: number
  sale_price?: number
  sku?: string
  barcode?: string
  image_url?: string
  category?: string
  stock_quantity?: number
  low_stock_threshold?: number
  variants?: ProductVariant[]
}

export interface ProductVariant {
  name: string
  options: string[]
  price_modifier?: number
}

export interface Product {
  id: string
  merchant_id: string
  name: string
  description?: string
  price: number
  sale_price?: number
  sku?: string
  barcode?: string
  image_url?: string
  category?: string
  stock_quantity: number
  low_stock_threshold: number
  variants?: ProductVariant[]
  is_active: boolean
  created_at: Date
  updated_at: Date
}

// POS Types

export type PaymentMethod = 'qr_code' | 'nfc' | 'manual' | 'payment_link'

export interface CartItem {
  product_id: string
  product_name: string
  quantity: number
  unit_price: number
  total: number
  variant?: string
}

export interface Discount {
  type: 'percentage' | 'fixed'
  value: number
  reason?: string
}

export interface SaleInput {
  items: CartItem[]
  subtotal: number
  discount?: Discount
  tax_rate: number
  tax_amount: number
  total: number
  payment_method: PaymentMethod
  customer_name?: string
  customer_email?: string
  notes?: string
}

export interface Sale {
  id: string
  merchant_id: string
  sale_number: string
  items: CartItem[]
  subtotal: number
  discount?: Discount
  tax_rate: number
  tax_amount: number
  total: number
  payment_method: PaymentMethod
  customer_name?: string
  customer_email?: string
  notes?: string
  transaction_id?: string
  voided: boolean
  voided_at?: Date
  created_at: Date
}

// Payment Link Types

export type PaymentLinkStatus = 'active' | 'paid' | 'expired' | 'cancelled'

export interface PaymentLinkInput {
  amount?: number
  description: string
  expires_at?: Date
  reusable?: boolean
  max_uses?: number
  custom_fields?: Record<string, string>
}

export interface PaymentLink {
  id: string
  merchant_id: string
  link_code: string
  amount?: number
  description: string
  status: PaymentLinkStatus
  expires_at?: Date
  reusable: boolean
  max_uses?: number
  use_count: number
  custom_fields?: Record<string, string>
  paid_at?: Date
  transaction_id?: string
  created_at: Date
}

export interface PaymentStatus {
  link_id: string
  status: PaymentLinkStatus
  paid: boolean
  paid_at?: Date
  amount_paid?: number
  payer_address?: string
  transaction_id?: string
}

// Refund Types

export type RefundStatus = 'pending' | 'approved' | 'rejected' | 'processing' | 'completed' | 'failed'

export interface RefundRequest {
  sale_id: string
  amount?: number
  reason: string
  customer_notes?: string
}

export interface Refund {
  id: string
  sale_id: string
  merchant_id: string
  original_amount: number
  refund_amount: number
  reason: string
  customer_notes?: string
  merchant_notes?: string
  status: RefundStatus
  requested_at: Date
  processed_at?: Date
  transaction_id?: string
  created_at: Date
  updated_at: Date
}
