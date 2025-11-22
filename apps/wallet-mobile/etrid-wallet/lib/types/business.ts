// Business Account Types

export type Role = 'owner' | 'admin' | 'manager' | 'viewer'

export interface BusinessInfo {
  name: string
  email: string
  address: string
  logo_url?: string
  tax_id?: string
  phone?: string
}

export interface BusinessAccount {
  id: string
  name: string
  email: string
  address: string
  logo_url?: string
  tax_id?: string
  phone?: string
  wallet_address: string
  created_at: Date
  updated_at: Date
}

export interface DashboardStats {
  revenue_this_month: number
  revenue_change: number
  pending_invoices: number
  team_member_count: number
  total_expenses: number
  expenses_change: number
  invoice_count: {
    draft: number
    sent: number
    paid: number
    overdue: number
  }
}

// Team Management Types

export interface TeamMemberInput {
  user_id: string
  email: string
  name: string
  role: Role
  permissions: Permission[]
}

export interface TeamMember {
  id: string
  business_id: string
  user_id: string
  email: string
  name: string
  avatar_url?: string
  role: Role
  permissions: Permission[]
  added_at: Date
  last_active?: Date
}

export interface Permission {
  resource: 'invoices' | 'payroll' | 'expenses' | 'team' | 'settings'
  actions: ('create' | 'read' | 'update' | 'delete')[]
}

export interface Activity {
  id: string
  member_id: string
  action: string
  resource: string
  timestamp: Date
  details?: any
}

// Invoice Types

export type InvoiceStatus = 'draft' | 'sent' | 'paid' | 'overdue' | 'cancelled'

export interface InvoiceLineItem {
  description: string
  quantity: number
  unit_price: number
  total: number
}

export interface InvoiceInput {
  client_name: string
  client_email: string
  client_address?: string
  line_items: InvoiceLineItem[]
  tax_rate: number
  notes?: string
  due_date: Date
}

export interface Invoice {
  id: string
  business_id: string
  invoice_number: string
  client_name: string
  client_email: string
  client_address?: string
  line_items: InvoiceLineItem[]
  subtotal: number
  tax_rate: number
  tax_amount: number
  total: number
  status: InvoiceStatus
  issued_date: Date
  due_date: Date
  paid_date?: Date
  notes?: string
  created_at: Date
  updated_at: Date
}

export interface InvoiceFilter {
  status?: InvoiceStatus
  start_date?: Date
  end_date?: Date
  client?: string
}

// Payroll Types

export type PayrollStatus = 'pending' | 'processing' | 'completed' | 'failed'

export interface PayrollEmployee {
  employee_id: string
  name: string
  email: string
  wallet_address: string
  amount: number
  tax_withholding?: number
  net_amount: number
}

export interface PayrollInput {
  pay_period_start: Date
  pay_period_end: Date
  employees: PayrollEmployee[]
  notes?: string
}

export interface Payroll {
  id: string
  business_id: string
  pay_period_start: Date
  pay_period_end: Date
  employees: PayrollEmployee[]
  total_gross: number
  total_tax: number
  total_net: number
  status: PayrollStatus
  scheduled_date?: Date
  executed_at?: Date
  transaction_ids?: string[]
  notes?: string
  created_at: Date
}

export interface PayrollSchedule {
  frequency: 'weekly' | 'bi-weekly' | 'monthly'
  day_of_week?: number
  day_of_month?: number
  auto_execute: boolean
}

// Expense Types

export type ExpenseCategory =
  | 'office'
  | 'travel'
  | 'software'
  | 'marketing'
  | 'equipment'
  | 'utilities'
  | 'professional_services'
  | 'other'

export interface ExpenseInput {
  amount: number
  category: ExpenseCategory
  description: string
  date: Date
  receipt_url?: string
  team_member_id?: string
  reimbursable?: boolean
}

export interface Expense {
  id: string
  business_id: string
  amount: number
  category: ExpenseCategory
  description: string
  date: Date
  receipt_url?: string
  team_member_id?: string
  team_member_name?: string
  reimbursable: boolean
  reimbursed: boolean
  reimbursed_date?: Date
  created_at: Date
  updated_at: Date
}

export interface ExpenseFilter {
  category?: ExpenseCategory
  start_date?: Date
  end_date?: Date
  team_member_id?: string
  reimbursable?: boolean
}

export interface CategoryBreakdown {
  category: ExpenseCategory
  total: number
  count: number
  percentage: number
}
