/**
 * TransactionExport - Transaction Data Export Component
 * Allows exporting transaction data in various formats (JSON, CSV)
 */

'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Separator } from '@/components/ui/separator';
import { Download, FileJson, FileText, CheckCircle2, Copy } from 'lucide-react';

export interface ExportTransaction {
  hash: string;
  type: string;
  chainId: string;
  timestamp: number;
  status: string;
  blockNumber?: number;
  from: string;
  to?: string;
  amount?: string;
  fee: string;
  details: any;
}

export interface TransactionExportProps {
  transactions: ExportTransaction[];
  onClose?: () => void;
}

export function TransactionExport({ transactions, onClose }: TransactionExportProps) {
  const [exportFormat, setExportFormat] = useState<'json' | 'csv'>('json');
  const [copied, setCopied] = useState(false);

  // Convert transactions to JSON
  const exportToJSON = (): string => {
    return JSON.stringify(transactions, null, 2);
  };

  // Convert transactions to CSV
  const exportToCSV = (): string => {
    const headers = [
      'Hash',
      'Type',
      'Chain',
      'Timestamp',
      'Status',
      'Block',
      'From',
      'To',
      'Amount',
      'Fee',
    ];

    const rows = transactions.map((tx) => [
      tx.hash,
      tx.type,
      tx.chainId,
      new Date(tx.timestamp).toISOString(),
      tx.status,
      tx.blockNumber?.toString() || '',
      tx.from,
      tx.to || '',
      tx.amount || '',
      tx.fee,
    ]);

    const csvContent = [
      headers.join(','),
      ...rows.map((row) => row.map((cell) => `"${cell}"`).join(',')),
    ].join('\n');

    return csvContent;
  };

  // Download file
  const handleDownload = () => {
    const content = exportFormat === 'json' ? exportToJSON() : exportToCSV();
    const blob = new Blob([content], {
      type: exportFormat === 'json' ? 'application/json' : 'text/csv',
    });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `etrid-transactions-${Date.now()}.${exportFormat}`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  // Copy to clipboard
  const handleCopy = () => {
    const content = exportFormat === 'json' ? exportToJSON() : exportToCSV();
    navigator.clipboard.writeText(content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  // Generate preview
  const getPreview = (): string => {
    const content = exportFormat === 'json' ? exportToJSON() : exportToCSV();
    const lines = content.split('\n');
    return lines.slice(0, 20).join('\n') + (lines.length > 20 ? '\n...' : '');
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <Card>
        <CardHeader>
          <CardTitle>Export Transaction Data</CardTitle>
          <CardDescription>
            Export {transactions.length} transaction{transactions.length !== 1 ? 's' : ''} to JSON
            or CSV format
          </CardDescription>
        </CardHeader>
      </Card>

      {/* Format Selection */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Select Export Format</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="grid grid-cols-2 gap-3">
            <Card
              className={`cursor-pointer transition-all ${
                exportFormat === 'json'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setExportFormat('json')}
            >
              <CardContent className="pt-6 pb-6 text-center">
                <FileJson className="w-8 h-8 mx-auto mb-2 text-primary" />
                <h4 className="font-medium">JSON</h4>
                <p className="text-xs text-muted-foreground mt-1">
                  Structured data format
                </p>
              </CardContent>
            </Card>

            <Card
              className={`cursor-pointer transition-all ${
                exportFormat === 'csv'
                  ? 'border-primary ring-2 ring-primary/20'
                  : 'hover:border-primary/50'
              }`}
              onClick={() => setExportFormat('csv')}
            >
              <CardContent className="pt-6 pb-6 text-center">
                <FileText className="w-8 h-8 mx-auto mb-2 text-primary" />
                <h4 className="font-medium">CSV</h4>
                <p className="text-xs text-muted-foreground mt-1">
                  Spreadsheet compatible
                </p>
              </CardContent>
            </Card>
          </div>
        </CardContent>
      </Card>

      {/* Preview */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <CardTitle className="text-base">Preview</CardTitle>
            <Badge variant="secondary">{exportFormat.toUpperCase()}</Badge>
          </div>
        </CardHeader>
        <CardContent>
          <div className="relative">
            <pre className="bg-muted/50 p-4 rounded-lg text-xs font-mono overflow-x-auto max-h-80 overflow-y-auto">
              {getPreview()}
            </pre>
            <Button
              variant="ghost"
              size="sm"
              className="absolute top-2 right-2"
              onClick={handleCopy}
            >
              {copied ? (
                <>
                  <CheckCircle2 className="w-4 h-4 mr-2" />
                  Copied!
                </>
              ) : (
                <>
                  <Copy className="w-4 h-4 mr-2" />
                  Copy
                </>
              )}
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Export Stats */}
      <Card className="bg-muted/30">
        <CardContent className="pt-6">
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <p className="text-2xl font-bold">{transactions.length}</p>
              <p className="text-xs text-muted-foreground">Transactions</p>
            </div>
            <div>
              <p className="text-2xl font-bold">
                {transactions.filter((tx) => tx.status === 'confirmed').length}
              </p>
              <p className="text-xs text-muted-foreground">Confirmed</p>
            </div>
            <div>
              <p className="text-2xl font-bold">
                {new Set(transactions.map((tx) => tx.chainId)).size}
              </p>
              <p className="text-xs text-muted-foreground">Chains</p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Info */}
      <Alert>
        <Download className="h-4 w-4" />
        <AlertDescription className="text-xs">
          <p className="font-medium mb-1">Export Information:</p>
          <ul className="list-disc list-inside space-y-0.5">
            <li>JSON format includes full transaction details</li>
            <li>CSV format is compatible with Excel and Google Sheets</li>
            <li>Exported data can be used for accounting and record keeping</li>
            <li>Sensitive information is included - store securely</li>
          </ul>
        </AlertDescription>
      </Alert>

      {/* Action Buttons */}
      <div className="flex gap-3">
        {onClose && (
          <Button variant="outline" className="flex-1" onClick={onClose}>
            Close
          </Button>
        )}
        <Button className="flex-1 gap-2" onClick={handleDownload}>
          <Download className="w-4 h-4" />
          Download {exportFormat.toUpperCase()}
        </Button>
      </div>
    </div>
  );
}
