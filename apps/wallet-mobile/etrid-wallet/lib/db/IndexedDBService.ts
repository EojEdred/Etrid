export class IndexedDBService {
  private dbName = 'etrid-wallet-db';
  private version = 1;
  private db: IDBDatabase | null = null;

  async initialize(): Promise<void> {
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(this.dbName, this.version);

      request.onerror = () => reject(request.error);
      request.onsuccess = () => {
        this.db = request.result;
        resolve();
      };

      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;

        // Transactions store
        if (!db.objectStoreNames.contains('transactions')) {
          const txStore = db.createObjectStore('transactions', { keyPath: 'id' });
          txStore.createIndex('timestamp', 'timestamp');
          txStore.createIndex('status', 'status');
        }

        // Balances store
        if (!db.objectStoreNames.contains('balances')) {
          db.createObjectStore('balances', { keyPath: 'asset' });
        }

        // Pending operations store
        if (!db.objectStoreNames.contains('pending')) {
          const pendingStore = db.createObjectStore('pending', { keyPath: 'id', autoIncrement: true });
          pendingStore.createIndex('type', 'type');
        }
      };
    });
  }

  async savePendingTransaction(tx: any): Promise<void> {
    if (!this.db) await this.initialize();

    const transaction = this.db!.transaction(['pending'], 'readwrite');
    const store = transaction.objectStore('pending');
    store.add({ ...tx, timestamp: Date.now() });
  }

  async getPendingTransactions(): Promise<any[]> {
    if (!this.db) await this.initialize();

    return new Promise((resolve, reject) => {
      const transaction = this.db!.transaction(['pending'], 'readonly');
      const store = transaction.objectStore('pending');
      const request = store.getAll();

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async saveTransaction(tx: any): Promise<void> {
    if (!this.db) await this.initialize();

    const transaction = this.db!.transaction(['transactions'], 'readwrite');
    const store = transaction.objectStore('transactions');
    store.put(tx);
  }

  async getTransactions(): Promise<any[]> {
    if (!this.db) await this.initialize();

    return new Promise((resolve, reject) => {
      const transaction = this.db!.transaction(['transactions'], 'readonly');
      const store = transaction.objectStore('transactions');
      const request = store.getAll();

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async saveBalance(asset: string, balance: any): Promise<void> {
    if (!this.db) await this.initialize();

    const transaction = this.db!.transaction(['balances'], 'readwrite');
    const store = transaction.objectStore('balances');
    store.put({ asset, ...balance, lastUpdated: Date.now() });
  }

  async getBalance(asset: string): Promise<any | null> {
    if (!this.db) await this.initialize();

    return new Promise((resolve, reject) => {
      const transaction = this.db!.transaction(['balances'], 'readonly');
      const store = transaction.objectStore('balances');
      const request = store.get(asset);

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async getAllBalances(): Promise<any[]> {
    if (!this.db) await this.initialize();

    return new Promise((resolve, reject) => {
      const transaction = this.db!.transaction(['balances'], 'readonly');
      const store = transaction.objectStore('balances');
      const request = store.getAll();

      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async clearPendingTransaction(id: number): Promise<void> {
    if (!this.db) await this.initialize();

    const transaction = this.db!.transaction(['pending'], 'readwrite');
    const store = transaction.objectStore('pending');
    store.delete(id);
  }

  async clearAllData(): Promise<void> {
    if (!this.db) await this.initialize();

    const transaction = this.db!.transaction(['transactions', 'balances', 'pending'], 'readwrite');

    transaction.objectStore('transactions').clear();
    transaction.objectStore('balances').clear();
    transaction.objectStore('pending').clear();
  }
}

// Singleton instance
export const indexedDBService = new IndexedDBService();
