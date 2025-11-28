//
//  BookmarksView.swift
//  EtridWallet
//
//  Bookmark management for dApp browser
//

import SwiftUI

struct BookmarksView: View {
    @StateObject private var service = DAppBrowserService.shared
    @Environment(\.dismiss) private var dismiss
    @State private var showAddBookmark = false
    @State private var newBookmarkName = ""
    @State private var newBookmarkURL = ""

    var body: some View {
        NavigationView {
            List {
                if service.bookmarks.isEmpty {
                    ContentUnavailableView(
                        "No Bookmarks",
                        systemImage: "book",
                        description: Text("Add bookmarks to quickly access your favorite dApps")
                    )
                } else {
                    ForEach(service.bookmarks) { bookmark in
                        BookmarkRow(bookmark: bookmark) {
                            // Navigate to bookmark
                            dismiss()
                        }
                    }
                    .onDelete(perform: deleteBookmarks)
                }
            }
            .navigationTitle("Bookmarks")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Done") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showAddBookmark = true }) {
                        Image(systemName: "plus")
                    }
                }
            }
            .sheet(isPresented: $showAddBookmark) {
                AddBookmarkView(
                    name: $newBookmarkName,
                    url: $newBookmarkURL,
                    onSave: addBookmark
                )
            }
        }
    }

    private func deleteBookmarks(at offsets: IndexSet) {
        for index in offsets {
            let bookmark = service.bookmarks[index]
            service.removeBookmark(bookmark)
        }
    }

    private func addBookmark() {
        guard !newBookmarkName.isEmpty, !newBookmarkURL.isEmpty else { return }
        service.addBookmark(name: newBookmarkName, url: newBookmarkURL, iconUrl: nil)
        newBookmarkName = ""
        newBookmarkURL = ""
        showAddBookmark = false
    }
}

struct BookmarkRow: View {
    let bookmark: Bookmark
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack {
                Circle()
                    .fill(Color.blue.opacity(0.1))
                    .frame(width: 40, height: 40)
                    .overlay(
                        Text(String(bookmark.name.prefix(1)))
                            .foregroundColor(.blue)
                    )

                VStack(alignment: .leading) {
                    Text(bookmark.name)
                        .font(.headline)
                    Text(bookmark.url)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .lineLimit(1)
                }

                Spacer()
            }
        }
    }
}

struct AddBookmarkView: View {
    @Binding var name: String
    @Binding var url: String
    let onSave: () -> Void
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationView {
            Form {
                Section("Bookmark Details") {
                    TextField("Name", text: $name)
                    TextField("URL", text: $url)
                        .autocapitalization(.none)
                        .keyboardType(.URL)
                }
            }
            .navigationTitle("Add Bookmark")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Save") {
                        onSave()
                        dismiss()
                    }
                    .disabled(name.isEmpty || url.isEmpty)
                }
            }
        }
    }
}
