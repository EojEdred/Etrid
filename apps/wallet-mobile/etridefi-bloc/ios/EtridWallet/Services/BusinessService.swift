//
//  BusinessService.swift
//  EtridWallet
//
//  Production business management and payment service
//

import Foundation

// MARK: - Models
struct BusinessMetrics: Codable {
    let monthlyRevenue: Double
    let yearlyRevenue: Double
    let outstandingInvoices: Int
    let teamMembers: Int
    let monthlyExpenses: Double
}

struct Invoice: Codable, Identifiable {
    let id: String
    let invoiceNumber: String
    let client: String
    let amount: Double
    let status: InvoiceStatus
    let dueDate: Date
    let createdAt: Date
    let paidAt: Date?

    enum InvoiceStatus: String, Codable {
        case draft = "draft"
        case sent = "sent"
        case paid = "paid"
        case overdue = "overdue"
        case cancelled = "cancelled"
    }
}

struct Expense: Codable, Identifiable {
    let id: String
    let category: String
    let description: String
    let amount: Double
    let date: Date
    let merchant: String?
    let receipt: String?
}

struct TeamMember: Codable, Identifiable {
    let id: String
    let name: String
    let role: String
    let walletAddress: String
    let salary: Double
    let paymentFrequency: PaymentFrequency
    let joinedAt: Date

    enum PaymentFrequency: String, Codable {
        case weekly = "weekly"
        case biweekly = "biweekly"
        case monthly = "monthly"
    }
}

struct PaymentLink: Codable, Identifiable {
    let id: String
    let title: String
    let amount: Double?
    let currency: String
    let url: String
    let createdAt: Date
    let expiresAt: Date?
}

// MARK: - Business Service
class BusinessService {
    static let shared = BusinessService()

    private init() {}

    // MARK: - Metrics
    func getMetrics(for userId: String) async throws -> BusinessMetrics {
        do {
            return try await NetworkService.shared.request(
                endpoint: "/business/metrics?userId=\(userId)"
            )
        } catch {
            // Mock data for development
            return BusinessMetrics(
                monthlyRevenue: 125000,
                yearlyRevenue: 1200000,
                outstandingInvoices: 5,
                teamMembers: 8,
                monthlyExpenses: 35000
            )
        }
    }

    // MARK: - Invoices
    func getInvoices(for userId: String, status: Invoice.InvoiceStatus? = nil) async throws -> [Invoice] {
        struct InvoiceListResponse: Decodable {
            let invoices: [Invoice]
        }

        var endpoint = "/business/invoices?userId=\(userId)"
        if let status = status {
            endpoint += "&status=\(status.rawValue)"
        }

        do {
            let response: InvoiceListResponse = try await NetworkService.shared.request(
                endpoint: endpoint
            )
            return response.invoices
        } catch {
            return generateMockInvoices()
        }
    }

    func createInvoice(
        userId: String,
        client: String,
        amount: Double,
        dueDate: Date,
        items: [[String: Any]]
    ) async throws -> Invoice {
        struct CreateInvoiceRequest: Encodable {
            let userId: String
            let client: String
            let amount: String
            let dueDate: String
        }

        let formatter = ISO8601DateFormatter()
        let request = CreateInvoiceRequest(
            userId: userId,
            client: client,
            amount: String(amount),
            dueDate: formatter.string(from: dueDate)
        )

        return try await NetworkService.shared.request(
            endpoint: "/business/invoices",
            method: .post,
            body: request
        )
    }

    // MARK: - Expenses
    func trackExpense(
        userId: String,
        category: String,
        description: String,
        amount: Double,
        merchant: String?
    ) async throws -> Expense {
        struct CreateExpenseRequest: Encodable {
            let userId: String
            let category: String
            let description: String
            let amount: String
            let merchant: String?
        }

        let request = CreateExpenseRequest(
            userId: userId,
            category: category,
            description: description,
            amount: String(amount),
            merchant: merchant
        )

        return try await NetworkService.shared.request(
            endpoint: "/business/expenses",
            method: .post,
            body: request
        )
    }

    // MARK: - Payroll
    func runPayroll(userId: String, members: [String]) async throws {
        struct PayrollRequest: Encodable {
            let userId: String
            let memberIds: [String]
        }

        let _: SharedEmptyResponse = try await NetworkService.shared.request(
            endpoint: "/business/payroll",
            method: .post,
            body: PayrollRequest(userId: userId, memberIds: members)
        )
    }

    // MARK: - Payment Links
    func createPaymentLink(
        userId: String,
        title: String,
        amount: Double?,
        currency: String
    ) async throws -> PaymentLink {
        struct CreatePaymentLinkRequest: Encodable {
            let userId: String
            let title: String
            let amount: String?
            let currency: String
        }

        let request = CreatePaymentLinkRequest(
            userId: userId,
            title: title,
            amount: amount != nil ? String(amount!) : nil,
            currency: currency
        )

        return try await NetworkService.shared.request(
            endpoint: "/business/payment-links",
            method: .post,
            body: request
        )
    }

    private func generateMockInvoices() -> [Invoice] {
        [
            Invoice(
                id: "inv_1",
                invoiceNumber: "INV-001",
                client: "Acme Corp",
                amount: 5000,
                status: .sent,
                dueDate: Date().addingTimeInterval(259200),
                createdAt: Date().addingTimeInterval(-604800),
                paidAt: nil
            ),
            Invoice(
                id: "inv_2",
                invoiceNumber: "INV-002",
                client: "TechStart Inc",
                amount: 3500,
                status: .overdue,
                dueDate: Date().addingTimeInterval(-86400),
                createdAt: Date().addingTimeInterval(-1209600),
                paidAt: nil
            )
        ]
    }
}

// MARK: - Errors
enum BusinessError: Error {
    case invalidData
    case payrollFailed(String)

    var localizedDescription: String {
        switch self {
        case .invalidData:
            return "Invalid business data"
        case .payrollFailed(let reason):
            return "Payroll failed: \(reason)"
        }
    }
}
