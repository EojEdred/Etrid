'use client';

import { useState, useEffect, useRef } from 'react';
import { Terminal as TerminalIcon, X, Maximize2, Minimize2 } from 'lucide-react';
import { usePolkadotApi } from '@/hooks/usePolkadotApi';
import { useNetworkStats } from '@/hooks/useNetworkStats';
import { useWallet } from '@/contexts/WalletContext';
import { fetchNodePeers } from '@/hooks/useNodePeers';
import type { BalanceData } from '@/hooks/useBalance';

interface TerminalLine {
  type: 'input' | 'output' | 'error' | 'info';
  content: string;
}

interface TerminalProps {
  balance?: BalanceData | null;
}

export function Terminal({ balance = null }: TerminalProps = {}) {
  const [isOpen, setIsOpen] = useState(false);
  const [isMaximized, setIsMaximized] = useState(false);
  const [input, setInput] = useState('');
  const [history, setHistory] = useState<TerminalLine[]>([
    { type: 'info', content: 'Etrid Console v1.0.0' },
    { type: 'info', content: 'Type "help" for available commands.' },
  ]);
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);

  const bottomRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const { isConnected, currentBlock, chainInfo } = usePolkadotApi();
  const { stats, nodes } = useNetworkStats();
  const { account } = useWallet();

  useEffect(() => {
    if (isOpen && bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: 'smooth' });
      inputRef.current?.focus();
    }
  }, [history, isOpen]);

  const handleCommand = async (cmd: string) => {
    const trimmed = cmd.trim();
    if (!trimmed) return;

    setHistory(prev => [...prev, { type: 'input', content: `> ${trimmed}` }]);
    setCommandHistory(prev => [...prev, trimmed]);
    setHistoryIndex(-1);

    const parts = trimmed.split(' ');
    const command = parts[0].toLowerCase();
    const args = parts.slice(1);

    try {
      switch (command) {
        case 'help':
          setHistory(prev => [...prev, 
            { type: 'info', content: 'Available commands:' },
            { type: 'info', content: '  status       Show network status' },
            { type: 'info', content: '  network      Show network topology info' },
            { type: 'info', content: '  peers        List connected peers' },
            { type: 'info', content: '  balance      Show current wallet balance' },
            { type: 'info', content: '  clear        Clear terminal' },
            { type: 'info', content: '  whoami       Show current account' },
            { type: 'info', content: '  reset        Reset onboarding' },
          ]);
          break;

        case 'clear':
          setHistory([]);
          break;

        case 'reset':
          localStorage.removeItem('etrid_onboarding_completed');
          setHistory(prev => [...prev, { type: 'info', content: 'Onboarding reset. Refresh the page to see it again.' }]);
          setTimeout(() => window.location.reload(), 1000);
          break;

        case 'status':
          setHistory(prev => [...prev, 
            { type: 'output', content: `Chain: ${chainInfo?.chain || 'Unknown'}` },
            { type: 'output', content: `Block Height: ${currentBlock}` },
            { type: 'output', content: `Finalized: ${stats.finalizedHeight}` },
            { type: 'output', content: `TPS: ${stats.tps}` },
            { type: 'output', content: `Connection: ${isConnected ? 'Online' : 'Offline'}` },
          ]);
          break;

        case 'network':
          setHistory(prev => [...prev, 
            { type: 'output', content: `Validator Count: ${stats.validatorCount}` },
            { type: 'output', content: `Total Nodes: ${stats.nodeCount}` },
            { type: 'output', content: `Peers: ${stats.peerCount}` },
            { type: 'output', content: `Block Time: ${stats.blockTime}s` },
          ]);
          break;

        case 'peers':
          if (!isConnected) {
            setHistory(prev => [...prev, { type: 'error', content: 'Not connected to the node.' }]);
            break;
          }

          setHistory(prev => [...prev, { type: 'info', content: 'Fetching peers…' }]);
          {
            const peers = await fetchNodePeers();
            const lines: TerminalLine[] = [
              { type: 'info', content: `Connected peers (${peers.length}):` },
            ];

            if (peers.length === 0) {
              lines.push({ type: 'output', content: '  (none)' });
            } else {
              peers.slice(0, 25).forEach((peer) => {
                lines.push({
                  type: 'output',
                  content: `  ${peer.peerId} roles=${peer.roles} best=${peer.bestNumber ?? '—'} proto=${peer.protocolVersion ?? '—'}`,
                });
              });
              if (peers.length > 25) {
                lines.push({ type: 'info', content: 'Showing first 25 peers.' });
              }
            }

            setHistory(prev => [...prev, ...lines]);
          }
          break;

        case 'whoami':
          if (account) {
            setHistory(prev => [...prev, { type: 'output', content: `Address: ${account.address}` }]);
            setHistory(prev => [...prev, { type: 'output', content: `Name: ${account.name}` }]);
          } else {
            setHistory(prev => [...prev, { type: 'error', content: 'Not logged in.' }]);
          }
          break;

        case 'balance':
          if (account) {
            if (balance && balance.formatted) {
              setHistory(prev => [...prev, { type: 'output', content: `Balance: ${balance.formatted}` }]);
            } else {
              setHistory(prev => [...prev, { type: 'output', content: 'Balance: Loading...' }]);
            }
          } else {
            setHistory(prev => [...prev, { type: 'error', content: 'No wallet connected. Please connect your wallet first.' }]);
          }
          break;

        default:
          setHistory(prev => [...prev, { type: 'error', content: `Unknown command: ${command}` }]);
      }
    } catch (error) {
      setHistory(prev => [...prev, { type: 'error', content: `Error: ${error instanceof Error ? error.message : String(error)}` }]);
    }
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleCommand(input);
      setInput('');
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (commandHistory.length > 0) {
        const newIndex = historyIndex + 1;
        if (newIndex < commandHistory.length) {
          setHistoryIndex(newIndex);
          setInput(commandHistory[commandHistory.length - 1 - newIndex]);
        }
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIndex > 0) {
        const newIndex = historyIndex - 1;
        setHistoryIndex(newIndex);
        setInput(commandHistory[commandHistory.length - 1 - newIndex]);
      } else if (historyIndex === 0) {
        setHistoryIndex(-1);
        setInput('');
      }
    }
  };

  if (!isOpen) {
    return (
      <button 
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 left-4 z-50 bg-black/80 backdrop-blur border border-white/20 p-3 rounded-full text-white hover:bg-black hover:border-cyan-500 transition-all shadow-lg shadow-cyan-500/20 group"
      >
        <TerminalIcon className="w-5 h-5 group-hover:text-cyan-400" />
      </button>
    );
  }

  return (
    <div className={`fixed z-50 bg-black/90 backdrop-blur-md border-t border-r border-white/20 shadow-2xl transition-all duration-300 flex flex-col font-mono text-sm ${
      isMaximized ? 'inset-0' : 'bottom-0 left-0 right-0 h-[300px]'
    }`}>
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 py-2 bg-white/5 border-b border-white/10">
        <div className="flex items-center gap-2">
          <TerminalIcon className="w-4 h-4 text-cyan-400" />
          <span className="font-bold text-white/80">Etrid Terminal</span>
        </div>
        <div className="flex items-center gap-2">
          <button 
            onClick={() => setIsMaximized(!isMaximized)}
            className="p-1 hover:bg-white/10 rounded text-white/60 hover:text-white"
          >
            {isMaximized ? <Minimize2 className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
          </button>
          <button 
            onClick={() => setIsOpen(false)}
            className="p-1 hover:bg-red-500/20 rounded text-white/60 hover:text-red-400"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      </div>

      {/* Output Area */}
      <div className="flex-1 overflow-y-auto p-4 space-y-1" onClick={() => inputRef.current?.focus()}>
        {history.map((line, i) => (
          <div key={i} className={`${
            line.type === 'input' ? 'text-white/60' :
            line.type === 'error' ? 'text-red-400' :
            line.type === 'info' ? 'text-cyan-400' :
            'text-green-400'
          }`}>
            {line.content}
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      {/* Input Area */}
      <div className="p-4 bg-white/5 flex items-center gap-2">
        <span className="text-cyan-400 font-bold">❯</span>
        <input
          ref={inputRef}
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={onKeyDown}
          className="flex-1 bg-transparent border-none outline-none text-white placeholder-white/20"
          placeholder="Type a command..."
          autoFocus
        />
      </div>
    </div>
  );
}
