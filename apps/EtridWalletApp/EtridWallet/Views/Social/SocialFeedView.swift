//
//  SocialFeedView.swift
//  EtridWallet
//
//  Social feed showing activity, friend requests, and payment requests
//

import SwiftUI

struct SocialFeedView: View {
    @StateObject private var viewModel = SocialFeedViewModel()
    @State private var selectedTab = 0
    @State private var showingFilter = false
    @State private var selectedActivity: ActivityItem?
    @State private var showingActivityDetail = false

    var body: some View {
        NavigationView {
            ZStack {
                LinearGradient(
                    colors: [Color(red: 0.1, green: 0.05, blue: 0.2), Color(red: 0.2, green: 0.1, blue: 0.3)],
                    startPoint: .topLeading,
                    endPoint: .bottomTrailing
                )
                .ignoresSafeArea()

                VStack(spacing: 0) {
                    // Stats Card
                    statsCard

                    // Tabs
                    Picker("Feed", selection: $selectedTab) {
                        Text("Activity").tag(0)
                        Text("Requests").tag(1)
                        Text("Friends").tag(2)
                    }
                    .pickerStyle(.segmented)
                    .padding()

                    // Content
                    TabView(selection: $selectedTab) {
                        activityFeed
                            .tag(0)

                        requestsFeed
                            .tag(1)

                        friendRequestsFeed
                            .tag(2)
                    }
                    .tabViewStyle(.page(indexDisplayMode: .never))
                }
            }
            .navigationTitle("Social")
            .navigationBarTitleDisplayMode(.large)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingFilter = true
                    } label: {
                        Image(systemName: "line.3.horizontal.decrease.circle")
                            .foregroundColor(.white)
                    }
                }
            }
            .refreshable {
                await viewModel.refresh()
            }
        }
    }

    private var statsCard: some View {
        HStack(spacing: 20) {
            StatItem(
                title: "Friends",
                value: "\(viewModel.stats.totalFriends)",
                icon: "person.2.fill",
                color: .blue
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Transactions",
                value: "\(viewModel.stats.totalTransactions)",
                icon: "arrow.left.arrow.right",
                color: .green
            )

            Divider()
                .background(Color.white.opacity(0.3))
                .frame(height: 40)

            StatItem(
                title: "Active Bills",
                value: "\(viewModel.stats.activeBills)",
                icon: "doc.text",
                color: .purple
            )
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
        .padding()
    }

    private var activityFeed: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if viewModel.activities.isEmpty {
                    emptyStateView(
                        icon: "tray",
                        title: "No Activity Yet",
                        message: "Your transaction and social activity will appear here"
                    )
                } else {
                    ForEach(viewModel.activities) { activity in
                        ActivityCard(activity: activity)
                            .onTapGesture {
                                selectedActivity = activity
                                showingActivityDetail = true
                            }
                    }
                }
            }
            .padding()
        }
    }

    private var requestsFeed: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if viewModel.paymentRequests.isEmpty {
                    emptyStateView(
                        icon: "tray.fill",
                        title: "No Payment Requests",
                        message: "Payment requests will appear here"
                    )
                } else {
                    ForEach(viewModel.paymentRequests) { request in
                        PaymentRequestCard(request: request, viewModel: viewModel)
                    }
                }
            }
            .padding()
        }
    }

    private var friendRequestsFeed: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                if viewModel.friendRequests.isEmpty {
                    emptyStateView(
                        icon: "person.2.slash",
                        title: "No Friend Requests",
                        message: "Friend requests will appear here"
                    )
                } else {
                    ForEach(viewModel.friendRequests) { request in
                        FriendRequestCard(request: request, viewModel: viewModel)
                    }
                }
            }
            .padding()
        }
    }

    private func emptyStateView(icon: String, title: String, message: String) -> some View {
        VStack(spacing: 20) {
            Image(systemName: icon)
                .font(.system(size: 60))
                .foregroundColor(.gray)

            Text(title)
                .font(.title2)
                .foregroundColor(.white)

            Text(message)
                .font(.subheadline)
                .foregroundColor(.gray)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(60)
    }
}

// MARK: - Activity Card
struct ActivityCard: View {
    let activity: ActivityItem

    var body: some View {
        HStack(spacing: 12) {
            // Icon
            ZStack {
                Circle()
                    .fill(Color(activity.type.color).opacity(0.2))
                    .frame(width: 50, height: 50)

                Image(systemName: activity.type.icon)
                    .foregroundColor(Color(activity.type.color))
            }

            // Content
            VStack(alignment: .leading, spacing: 4) {
                Text(activity.title)
                    .font(.body)
                    .fontWeight(.semibold)
                    .foregroundColor(.white)

                Text(activity.description)
                    .font(.caption)
                    .foregroundColor(.gray)
                    .lineLimit(2)

                Text(activity.timeAgo)
                    .font(.caption2)
                    .foregroundColor(.gray)
            }

            Spacer()

            // Amount (if applicable)
            if let formattedAmount = activity.formattedAmount {
                Text(formattedAmount)
                    .font(.subheadline)
                    .fontWeight(.semibold)
                    .foregroundColor(.white)
            }

            // Unread Badge
            if !activity.isRead {
                Circle()
                    .fill(Color.blue)
                    .frame(width: 8, height: 8)
            }
        }
        .padding()
        .background(Color.white.opacity(activity.isRead ? 0.05 : 0.1))
        .cornerRadius(12)
    }
}

// MARK: - Payment Request Card
struct PaymentRequestCard: View {
    let request: PaymentRequest
    @ObservedObject var viewModel: SocialFeedViewModel

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 4) {
                    Text(isIncoming ? "Payment Request From" : "Payment Request To")
                        .font(.caption)
                        .foregroundColor(.gray)

                    Text(isIncoming ? request.displayFromName : request.displayToName)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)
                }

                Spacer()

                VStack(alignment: .trailing, spacing: 4) {
                    Text("\(request.formattedAmount) \(request.tokenSymbol)")
                        .font(.title3)
                        .fontWeight(.bold)
                        .foregroundColor(.white)

                    HStack(spacing: 4) {
                        Circle()
                            .fill(statusColor)
                            .frame(width: 6, height: 6)

                        Text(request.status.rawValue)
                            .font(.caption)
                            .foregroundColor(statusColor)
                    }
                }
            }

            if let message = request.message {
                Text(message)
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .padding(8)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(8)
            }

            HStack {
                Text(request.createdAt.formatted(date: .abbreviated, time: .shortened))
                    .font(.caption)
                    .foregroundColor(.gray)

                if let expiresAt = request.expiresAt {
                    Spacer()
                    Text("Expires: \(expiresAt.formatted(date: .abbreviated, time: .omitted))")
                        .font(.caption)
                        .foregroundColor(request.isExpired ? .red : .gray)
                }
            }

            // Actions
            if request.status == .pending && isIncoming {
                HStack(spacing: 12) {
                    Button {
                        viewModel.declinePaymentRequest(request)
                    } label: {
                        Text("Decline")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.red.opacity(0.3))
                            .cornerRadius(8)
                    }

                    Button {
                        viewModel.payRequest(request)
                    } label: {
                        Text("Pay Now")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(
                                LinearGradient(
                                    colors: [Color.green, Color.blue],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .cornerRadius(8)
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var isIncoming: Bool {
        request.toAddress.lowercased() == viewModel.userAddress.lowercased()
    }

    private var statusColor: Color {
        switch request.status {
        case .paid: return .green
        case .pending: return .orange
        case .declined: return .red
        case .expired: return .gray
        }
    }
}

// MARK: - Friend Request Card
struct FriendRequestCard: View {
    let request: FriendRequest
    @ObservedObject var viewModel: SocialFeedViewModel

    var body: some View {
        VStack(spacing: 12) {
            HStack {
                // Avatar
                ZStack {
                    Circle()
                        .fill(
                            LinearGradient(
                                colors: [Color.blue.opacity(0.6), Color.purple.opacity(0.6)],
                                startPoint: .topLeading,
                                endPoint: .bottomTrailing
                            )
                        )
                        .frame(width: 50, height: 50)

                    if let avatar = request.avatar {
                        Text(avatar)
                            .font(.title2)
                    } else {
                        Text(request.displayFromName.prefix(1).uppercased())
                            .font(.title3)
                            .foregroundColor(.white)
                    }
                }

                VStack(alignment: .leading, spacing: 4) {
                    Text(request.displayFromName)
                        .font(.body)
                        .fontWeight(.semibold)
                        .foregroundColor(.white)

                    Text(isIncoming ? "wants to connect" : "Sent \(request.createdAt.formatted(date: .abbreviated, time: .omitted))")
                        .font(.caption)
                        .foregroundColor(.gray)
                }

                Spacer()

                if request.status != .pending {
                    HStack(spacing: 4) {
                        Circle()
                            .fill(Color(request.status.color))
                            .frame(width: 6, height: 6)

                        Text(request.status.rawValue)
                            .font(.caption)
                            .foregroundColor(Color(request.status.color))
                    }
                }
            }

            if let message = request.message {
                Text("\"\(message)\"")
                    .font(.subheadline)
                    .foregroundColor(.gray)
                    .italic()
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(8)
                    .background(Color.white.opacity(0.05))
                    .cornerRadius(8)
            }

            // Actions for incoming pending requests
            if request.status == .pending && isIncoming {
                HStack(spacing: 12) {
                    Button {
                        viewModel.declineFriendRequest(request)
                    } label: {
                        Text("Decline")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.gray.opacity(0.3))
                            .cornerRadius(8)
                    }

                    Button {
                        viewModel.acceptFriendRequest(request)
                    } label: {
                        Text("Accept")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                            .foregroundColor(.white)
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(
                                LinearGradient(
                                    colors: [Color.blue, Color.purple],
                                    startPoint: .leading,
                                    endPoint: .trailing
                                )
                            )
                            .cornerRadius(8)
                    }
                }
            }
        }
        .padding()
        .background(Color.white.opacity(0.05))
        .cornerRadius(16)
    }

    private var isIncoming: Bool {
        request.toAddress.lowercased() == viewModel.userAddress.lowercased()
    }
}

// MARK: - View Model
class SocialFeedViewModel: ObservableObject {
    @Published var activities: [ActivityItem] = []
    @Published var paymentRequests: [PaymentRequest] = []
    @Published var friendRequests: [FriendRequest] = []
    @Published var stats: SocialStats = SocialStats()
    @Published var notifications: [WalletNotification] = []

    let userAddress = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"

    init() {
        loadData()
    }

    private func loadData() {
        // Load activities
        activities = [
            ActivityItem(
                type: .sent,
                title: "Sent to Alice",
                description: "Payment sent successfully",
                amount: String(UInt64(5 * 1e18)),
                tokenSymbol: "ETR",
                toName: "Alice",
                timestamp: Date().addingTimeInterval(-3600),
                isRead: true
            ),
            ActivityItem(
                type: .received,
                title: "Received from Bob",
                description: "Payment received",
                amount: String(UInt64(3 * 1e18)),
                tokenSymbol: "ETR",
                fromName: "Bob",
                timestamp: Date().addingTimeInterval(-7200),
                isRead: false
            ),
            ActivityItem(
                type: .billCreated,
                title: "Created Bill Split",
                description: "Dinner at Restaurant - 4 participants",
                timestamp: Date().addingTimeInterval(-86400),
                isRead: true
            )
        ]

        // Load payment requests
        paymentRequests = [
            PaymentRequest(
                fromAddress: "0x1234567890123456789012345678901234567890",
                toAddress: userAddress,
                fromName: "Alice",
                amount: String(UInt64(2 * 1e18)),
                tokenSymbol: "ETR",
                message: "Coffee this morning",
                expiresAt: Date().addingTimeInterval(86400)
            ),
            PaymentRequest(
                fromAddress: userAddress,
                toAddress: "0x0987654321098765432109876543210987654321",
                toName: "Bob",
                amount: String(UInt64(10 * 1e18)),
                tokenSymbol: "ETR",
                message: "Rent payment for December",
                expiresAt: Date().addingTimeInterval(172800)
            )
        ]

        // Load friend requests
        friendRequests = [
            FriendRequest(
                fromAddress: "0x1111111111111111111111111111111111111111",
                toAddress: userAddress,
                fromName: "Charlie",
                message: "Hey! Let's connect on Etrid",
                avatar: "👨‍💼"
            )
        ]

        // Load stats
        stats = SocialStats(
            totalFriends: 12,
            totalTransactions: 47,
            totalAmountSent: String(UInt64(10 * 1e18)),
            totalAmountReceived: String(UInt64(8 * 1e18)),
            activeBills: 3,
            pendingRequests: 2
        )
    }

    func refresh() async {
        // Refresh data from API/storage
        try? await Task.sleep(nanoseconds: 1_000_000_000)
        loadData()
    }

    func acceptFriendRequest(_ request: FriendRequest) {
        if let index = friendRequests.firstIndex(where: { $0.id == request.id }) {
            friendRequests[index].status = .accepted
            friendRequests[index].respondedAt = Date()
            // Add to contacts and update storage
        }
    }

    func declineFriendRequest(_ request: FriendRequest) {
        if let index = friendRequests.firstIndex(where: { $0.id == request.id }) {
            friendRequests[index].status = .declined
            friendRequests[index].respondedAt = Date()
            // Update storage
        }
    }

    func payRequest(_ request: PaymentRequest) {
        // Initiate payment flow
        if let index = paymentRequests.firstIndex(where: { $0.id == request.id }) {
            paymentRequests[index].status = .paid
            paymentRequests[index].paidAt = Date()
            // Update storage and process transaction
        }
    }

    func declinePaymentRequest(_ request: PaymentRequest) {
        if let index = paymentRequests.firstIndex(where: { $0.id == request.id }) {
            paymentRequests[index].status = .declined
            // Update storage
        }
    }

    func markActivityAsRead(_ activity: ActivityItem) {
        if let index = activities.firstIndex(where: { $0.id == activity.id }) {
            activities[index].isRead = true
            // Update storage
        }
    }
}

// MARK: - Helper Views
private struct StatItem: View {
    let title: String
    let value: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundColor(color)

            Text(value)
                .font(.headline)
                .foregroundColor(.white)

            Text(title)
                .font(.caption2)
                .foregroundColor(.gray)
        }
        .frame(maxWidth: .infinity)
    }
}

struct SocialFeedView_Previews: PreviewProvider {
    static var previews: some View {
        SocialFeedView()
    }
}
