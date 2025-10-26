# Video Tutorial 05: Building DApps on Etrid

**Duration:** 15 minutes
**Target Audience:** Web developers, developers who completed Tutorial 04
**Prerequisites:** Tutorial 04 completed (smart contract deployed), JavaScript/React knowledge, Node.js installed

---

## Script Overview

This tutorial guides developers through building a complete decentralized application (DApp) that interacts with the Message Board smart contract deployed in Tutorial 04. We'll use React and Polkadot.js API to create a beautiful, functional web interface.

---

## Time Markers & Script

### 00:00 - 00:45 | Introduction & Demo

**NARRATION:**
"Welcome to the final tutorial in our Etrid series! Today, we're building a complete decentralized application - a DApp - that brings your smart contract to life with a beautiful web interface.

In Tutorial 04, we deployed a Message Board smart contract. Now we'll build a React web app where users can connect their wallet, post messages to the blockchain, and read messages from other users - all with a clean, modern UI.

Here's what we're building: a single-page application with wallet connection, real-time blockchain queries, transaction signing, and responsive design. By the end of this 15-minute tutorial, you'll have a fully functional DApp that you can customize and deploy.

Let's see it in action! Users click 'Connect Wallet' and authorize with their Etrid account. They can then type a message and post it to the blockchain. The transaction is signed with their wallet, submitted to the network, and within seconds, their message appears on-chain. Other users can browse all messages posted by any account address.

Ready to build this? Let's dive in!"

**VISUAL CUES:**
- Demo of final DApp running
- Walkthrough of all features:
  - Wallet connection flow
  - Message posting with transaction confirmation
  - Message reading by address
  - Responsive design on mobile/desktop
- Animated architecture diagram: React → Polkadot.js API → Etrid RPC → Smart Contract
- Feature list with checkmarks
- Side-by-side: Before (Polkadot.js Apps) vs. After (custom UI)

**KEY POINTS TO EMPHASIZE:**
- Build on Tutorial 04's smart contract
- Complete, production-ready DApp
- Modern React best practices
- Polkadot.js API for blockchain interaction
- No backend server needed - fully decentralized

---

### 00:45 - 02:30 | DApp Architecture & Setup

**NARRATION:**
"Let's understand DApp architecture first. Traditional web apps have three layers: frontend, backend, and database. DApps replace the backend and database with a blockchain and smart contracts. Your React frontend talks directly to the blockchain using the Polkadot.js API library. No centralized server needed!

Here's the data flow: User clicks a button in React → App calls Polkadot.js API → API sends JSON-RPC request to Etrid node → Node queries smart contract → Result returns to your app → UI updates. For write operations, we add wallet signing: React triggers transaction → Wallet extension shows confirmation → User signs → Transaction submitted to blockchain → Contract state updates.

Now let's set up our project. Open your terminal and create a new React app with TypeScript: npx create-react-app etrid-message-board --template typescript. This gives us a modern React setup with TypeScript support for better type safety.

While that installs, let's talk about dependencies. We need three key libraries: @polkadot/api for blockchain connection and queries, @polkadot/extension-dapp for wallet integration, and @polkadot/api-contract for smart contract interaction. We'll also use a UI library - let's use Chakra UI for beautiful, accessible components.

The installation just finished. Change into the directory: cd etrid-message-board. Now install our blockchain dependencies: npm install @polkadot/api @polkadot/extension-dapp @polkadot/api-contract. And add Chakra UI: npm install @chakra-ui/react @emotion/react @emotion/styled framer-motion. Perfect! Our development environment is ready."

**VISUAL CUES:**
- Architecture comparison diagram:
  - Traditional: Frontend ↔ Backend ↔ Database
  - DApp: Frontend ↔ Blockchain (Smart Contracts)
- Data flow diagram with icons:
  - React logo → Polkadot.js → Etrid logo → Contract
- Sequence diagram for read vs. write operations
- Terminal: create-react-app command with progress
- Package.json preview showing dependencies
- Project structure tree view

**DEMO STEPS:**
1. Open terminal
2. Navigate to projects directory: `cd ~/projects`
3. Run: `npx create-react-app etrid-message-board --template typescript`
4. Wait for installation (2-3 minutes)
5. Run: `cd etrid-message-board`
6. Run: `npm install @polkadot/api @polkadot/extension-dapp @polkadot/api-contract`
7. Run: `npm install @chakra-ui/react @emotion/react @emotion/styled framer-motion`
8. Show successful installation
9. Run: `ls -la` to show project structure

**CODE TO DISPLAY:**

**Project Setup:**
```bash
# Create React app with TypeScript
npx create-react-app etrid-message-board --template typescript

cd etrid-message-board

# Install Polkadot.js dependencies
npm install @polkadot/api \
            @polkadot/extension-dapp \
            @polkadot/api-contract

# Install UI library
npm install @chakra-ui/react \
            @emotion/react \
            @emotion/styled \
            framer-motion

# Verify installation
npm list --depth=0
```

**Dependencies (package.json):**
```json
{
  "name": "etrid-message-board",
  "version": "1.0.0",
  "dependencies": {
    "@polkadot/api": "^10.11.1",
    "@polkadot/extension-dapp": "^0.46.6",
    "@polkadot/api-contract": "^10.11.1",
    "@chakra-ui/react": "^2.8.2",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "typescript": "^5.0.0"
  }
}
```

**Project Structure:**
```
etrid-message-board/
├── public/
├── src/
│   ├── components/      # React components
│   ├── hooks/           # Custom React hooks
│   ├── contracts/       # Contract metadata and ABI
│   ├── App.tsx          # Main application
│   └── index.tsx        # Entry point
├── package.json
└── tsconfig.json
```

**KEY POINTS TO EMPHASIZE:**
- DApps are frontend-only (no backend server)
- Blockchain replaces traditional backend + database
- Polkadot.js API is the bridge to blockchain
- TypeScript provides better development experience
- Component-based architecture for maintainability

**COMMON MISTAKES TO MENTION:**
- Don't forget TypeScript template flag
- Install all Polkadot.js packages (api, extension-dapp, api-contract)
- Emotion packages are required for Chakra UI

---

### 02:30 - 05:00 | Connecting to Etrid Blockchain

**NARRATION:**
"Let's build our first component: the blockchain connection hook. Create a new file: src/hooks/useEtridApi.ts. This custom hook will manage our connection to the Etrid blockchain.

We'll use React's useState and useEffect hooks to manage the connection lifecycle. First, we create an API instance by connecting to our RPC endpoint. We'll use the Etrid testnet: wss://rpc-testnet.etrid.io. Once connected, we save the API instance to state and expose it to our components.

Here's the complete hook. We import ApiPromise and WsProvider from Polkadot.js. We define state variables for the API instance, loading status, and any errors. In useEffect, we create an async function that initializes the connection. We create a WsProvider pointing to our RPC endpoint, then await ApiPromise.create with that provider. Once ready, we save the API and set loading to false.

Don't forget cleanup! When the component unmounts, we disconnect the API to prevent memory leaks. Return an object with our API, loading, and error states.

Now let's use this hook in our App component. Import useEtridApi at the top. Call the hook to get our API instance: const { api, loading, error } = useEtridApi(). Add simple conditional rendering: if loading, show 'Connecting...'. If error, show the error message. Otherwise, show our main app content.

Let's test it! Run npm start. The development server starts and opens your browser. You should see 'Connecting to Etrid...' briefly, then your main content. Check the console - you'll see the API connection confirmed. Perfect! We're connected to the blockchain."

**VISUAL CUES:**
- VS Code: Create new file with path highlighted
- Code editor split view:
  - Left: Hook implementation (full)
  - Right: App.tsx usage
- Highlight key parts:
  - Import statements
  - useState declarations
  - useEffect with async function
  - Cleanup function
  - Return object
- Browser: Show app loading states
- Browser console: Show API connection logs
- Network tab: Show WebSocket connection
- State diagram: Connecting → Connected → Error (if fails)

**DEMO STEPS:**
1. In VS Code, create `src/hooks/useEtridApi.ts`
2. Type/paste hook implementation
3. Save file
4. Open `src/App.tsx`
5. Import hook at top
6. Use hook in component
7. Add conditional rendering
8. Save all files
9. Terminal: `npm start`
10. Browser opens automatically
11. Show loading state briefly
12. Show connected state
13. Open DevTools console
14. Point out connection confirmation logs
15. Open Network tab
16. Show WebSocket connection (Status: 101 Switching Protocols)

**CODE TO DISPLAY:**

**useEtridApi.ts (complete):**
```typescript
import { useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';

const RPC_ENDPOINT = 'wss://rpc-testnet.etrid.io';

export const useEtridApi = () => {
  const [api, setApi] = useState<ApiPromise | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const connectToBlockchain = async () => {
      try {
        console.log('Connecting to Etrid testnet...');

        // Create WebSocket provider
        const provider = new WsProvider(RPC_ENDPOINT);

        // Create API instance
        const apiInstance = await ApiPromise.create({ provider });

        // Wait for API to be ready
        await apiInstance.isReady;

        console.log('Connected to Etrid!', {
          chain: await apiInstance.rpc.system.chain(),
          version: await apiInstance.rpc.system.version(),
        });

        setApi(apiInstance);
        setLoading(false);
      } catch (err) {
        console.error('Connection error:', err);
        setError(err instanceof Error ? err.message : 'Unknown error');
        setLoading(false);
      }
    };

    connectToBlockchain();

    // Cleanup: disconnect when component unmounts
    return () => {
      if (api) {
        api.disconnect();
      }
    };
  }, []);

  return { api, loading, error };
};
```

**App.tsx (usage):**
```typescript
import React from 'react';
import { ChakraProvider, Container, Heading, Text, Spinner, Alert } from '@chakra-ui/react';
import { useEtridApi } from './hooks/useEtridApi';

function App() {
  const { api, loading, error } = useEtridApi();

  if (loading) {
    return (
      <ChakraProvider>
        <Container centerContent mt={20}>
          <Spinner size="xl" color="purple.500" />
          <Text mt={4}>Connecting to Etrid testnet...</Text>
        </Container>
      </ChakraProvider>
    );
  }

  if (error) {
    return (
      <ChakraProvider>
        <Container mt={20}>
          <Alert status="error">
            <Text>Connection Error: {error}</Text>
          </Alert>
        </Container>
      </ChakraProvider>
    );
  }

  return (
    <ChakraProvider>
      <Container maxW="container.lg" py={10}>
        <Heading mb={8}>Etrid Message Board</Heading>
        <Text>Connected to blockchain! API ready.</Text>
      </Container>
    </ChakraProvider>
  );
}

export default App;
```

**Console Output:**
```
Connecting to Etrid testnet...
Connected to Etrid! {
  chain: "Etrid Testnet",
  version: "etrid-testnet/v1.0.0"
}
```

**KEY POINTS TO EMPHASIZE:**
- Custom hooks encapsulate blockchain logic
- Connection is asynchronous (use async/await)
- Always clean up connections in useEffect return
- Loading states improve user experience
- Error handling is crucial for production apps
- WebSocket connection is persistent (stays open)

**COMMON MISTAKES TO MENTION:**
- Don't forget to await API.isReady
- Must return cleanup function from useEffect
- Handle both loading and error states in UI
- RPC endpoint must use wss:// (not https://)

---

### 05:00 - 07:30 | Wallet Integration

**NARRATION:**
"Now let's add wallet integration so users can sign transactions. We'll support the Polkadot.js browser extension, which most Substrate users already have installed.

Create another custom hook: src/hooks/useWallet.ts. This hook will handle wallet connection, account selection, and signing capabilities.

The logic works like this: First, we request access to the user's wallet extension using web3Enable. This shows a permission popup. If they approve, we get access to their accounts using web3Accounts. We save the available accounts to state and let the user choose which one to use.

Here's the implementation. We import the extension functions from Polkadot.js. We define state for accounts and the selected account. Our connect function calls web3Enable with our app name - this is what shows in the extension popup. Then we fetch all accounts and save them to state.

For signing transactions, we need to create a Signer object from the extension. We'll add a getInjector function that retrieves the extension for a specific account. This is what we'll pass to the API when signing transactions.

Now integrate it into our App. Import useWallet, call the hook, and add a 'Connect Wallet' button. When clicked, call the connect function. Once connected, show the user's account address and a dropdown to switch accounts if they have multiple.

Let's test it! Refresh the page. Click 'Connect Wallet'. The Polkadot.js extension pops up asking for permission. Click 'Yes, allow this application'. Your account appears! Try switching between accounts if you have multiple. Perfect - wallet integration is working!"

**VISUAL CUES:**
- Code editor: useWallet.ts hook implementation
- Browser extension permission popup (animated)
- Extension icon changing color (disconnected → connected)
- UI showing account address with truncation
- Account switcher dropdown
- Identicon (account avatar) display
- Flow diagram: Connect button → Extension popup → Approval → Accounts loaded
- Before/after: No wallet vs. Connected wallet UI

**DEMO STEPS:**
1. Create `src/hooks/useWallet.ts`
2. Implement wallet hook
3. Update App.tsx with wallet UI
4. Add connect button
5. Add account display
6. Save files
7. Browser auto-refreshes
8. Click "Connect Wallet" button
9. Extension popup appears
10. Click "Yes, allow this application"
11. Account address appears in UI
12. Show account switcher (if multiple accounts)
13. Test switching accounts
14. Show disconnect functionality

**CODE TO DISPLAY:**

**useWallet.ts:**
```typescript
import { useState } from 'react';
import { web3Enable, web3Accounts, web3FromAddress } from '@polkadot/extension-dapp';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

export const useWallet = () => {
  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<InjectedAccountWithMeta | null>(null);

  const connect = async () => {
    try {
      // Request access to wallet extension
      const extensions = await web3Enable('Etrid Message Board');

      if (extensions.length === 0) {
        throw new Error('No extension found. Please install Polkadot.js extension.');
      }

      // Get all accounts from extension
      const allAccounts = await web3Accounts();

      if (allAccounts.length === 0) {
        throw new Error('No accounts found. Please create an account in your wallet.');
      }

      setAccounts(allAccounts);
      setSelectedAccount(allAccounts[0]); // Auto-select first account

      console.log('Wallet connected:', {
        accountsFound: allAccounts.length,
        selectedAddress: allAccounts[0].address,
      });
    } catch (err) {
      console.error('Wallet connection error:', err);
      throw err;
    }
  };

  const disconnect = () => {
    setAccounts([]);
    setSelectedAccount(null);
  };

  const getInjector = async (address: string) => {
    const injector = await web3FromAddress(address);
    return injector.signer;
  };

  return {
    accounts,
    selectedAccount,
    connect,
    disconnect,
    selectAccount: setSelectedAccount,
    getInjector,
  };
};
```

**Updated App.tsx (wallet section):**
```typescript
import { useWallet } from './hooks/useWallet';

function App() {
  const { api, loading, error } = useEtridApi();
  const {
    accounts,
    selectedAccount,
    connect: connectWallet,
    disconnect: disconnectWallet,
    selectAccount,
  } = useWallet();

  const handleConnect = async () => {
    try {
      await connectWallet();
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <ChakraProvider>
      <Container maxW="container.lg" py={10}>
        <Heading mb={8}>Etrid Message Board</Heading>

        {!selectedAccount ? (
          <Button colorScheme="purple" onClick={handleConnect}>
            Connect Wallet
          </Button>
        ) : (
          <Box>
            <Text>Connected: {selectedAccount.address}</Text>
            {accounts.length > 1 && (
              <Select
                value={selectedAccount.address}
                onChange={(e) => {
                  const account = accounts.find((acc) => acc.address === e.target.value);
                  if (account) selectAccount(account);
                }}
              >
                {accounts.map((acc) => (
                  <option key={acc.address} value={acc.address}>
                    {acc.meta.name || acc.address}
                  </option>
                ))}
              </Select>
            )}
            <Button size="sm" onClick={disconnectWallet}>Disconnect</Button>
          </Box>
        )}
      </Container>
    </ChakraProvider>
  );
}
```

**KEY POINTS TO EMPHASIZE:**
- Polkadot.js extension required (users must install)
- Permission popup is browser security feature
- Users can have multiple accounts
- Signer object needed for transaction signing
- Extension handles private keys securely
- Never expose or transmit private keys

**COMMON MISTAKES TO MENTION:**
- Check if extension is installed before calling web3Enable
- Handle case where user rejects permission
- Handle case where user has no accounts
- Don't hardcode account selection - let user choose

---

### 07:30 - 10:30 | Reading & Writing to Smart Contract

**NARRATION:**
"Now the exciting part - interacting with our Message Board smart contract! We'll build functions to read messages from the blockchain and post new messages.

First, we need our contract's metadata and address. Remember the .contract file we generated in Tutorial 04? Copy it to src/contracts/message_board.json. Also note your contract address from the deployment - we'll need that.

Create a new file: src/hooks/useContract.ts. This hook will encapsulate all contract interactions. We need to import ContractPromise from @polkadot/api-contract - this class represents our deployed contract.

Here's how it works: We create a ContractPromise instance using three things: our API connection, the contract metadata, and the contract address. This gives us an object we can use to call contract methods.

For reading messages, we use the contract's query methods. These are free - they don't cost gas because they don't modify blockchain state. We call contract.query.getMessage, pass the caller address, and await the result. The return value includes success status and the actual message data.

For posting messages, we use the contract's tx methods. These create real transactions that must be signed and cost gas. We call contract.tx.postMessage, pass the message text, and use signAndSend to submit it to the blockchain with the user's signature.

Let's build the UI components. Create PostMessage.tsx for the posting interface. Add a textarea for the message input and a button to submit. On submit, call our post message function, show a loading spinner during submission, and display success or error notifications.

Then create MessageList.tsx to display messages. Add an input for entering an account address and a button to fetch their message. Show the retrieved message in a nice card component with the poster's address and timestamp.

Wire everything together in App.tsx. Import both components and render them side by side. Connected wallet users can post messages. Anyone can read messages by entering an address.

Let's test the complete flow! Connect your wallet. Type a message: 'Hello from my DApp!' Click Post. The wallet extension shows the transaction details. Sign it. The transaction is submitted and you see a pending status. After 15 seconds, success! Now try reading your message. Enter your address and click Fetch. Your message appears! You've just built a fully functional DApp!"

**VISUAL CUES:**
- File structure showing contracts/ folder
- Contract metadata JSON preview
- Code editor: useContract.ts implementation
- Split screen: PostMessage and MessageList components
- UI mockup: Message board layout
- Transaction flow diagram:
  - User types message
  - Click Post
  - Extension popup (sign transaction)
  - Submitted to network
  - Pending confirmation
  - Confirmed in block
  - UI updates
- Live demo of posting and reading messages
- Loading states, success states, error states
- Beautiful card components with messages

**DEMO STEPS:**

**Setup:**
1. Copy contract metadata: `cp ~/contracts/message_board/target/ink/message_board.json src/contracts/`
2. Note contract address from Tutorial 04

**useContract Hook:**
3. Create `src/hooks/useContract.ts`
4. Implement contract initialization
5. Implement query (read) function
6. Implement transaction (write) function
7. Export hook

**PostMessage Component:**
8. Create `src/components/PostMessage.tsx`
9. Add form with textarea
10. Add submit handler with loading state
11. Add success/error notifications
12. Style with Chakra UI

**MessageList Component:**
13. Create `src/components/MessageList.tsx`
14. Add address input
15. Add fetch button
16. Add message display with cards
17. Handle empty states

**Integration:**
18. Import components in App.tsx
19. Arrange in grid layout
20. Pass necessary props

**Testing:**
21. Save all files
22. Browser refreshes
23. Connect wallet
24. Type message in textarea
25. Click "Post Message"
26. Extension shows transaction
27. Click "Sign and Submit"
28. Show pending indicator
29. Show success notification
30. Enter address in read section
31. Click "Fetch Message"
32. Message displays

**CODE TO DISPLAY:**

**useContract.ts:**
```typescript
import { useEffect, useState } from 'react';
import { ApiPromise } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import contractMetadata from '../contracts/message_board.json';

const CONTRACT_ADDRESS = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';

export const useContract = (api: ApiPromise | null) => {
  const [contract, setContract] = useState<ContractPromise | null>(null);

  useEffect(() => {
    if (api) {
      const contractInstance = new ContractPromise(api, contractMetadata, CONTRACT_ADDRESS);
      setContract(contractInstance);
    }
  }, [api]);

  const getMessage = async (address: string): Promise<string | null> => {
    if (!contract) return null;

    try {
      // Query is free (read-only)
      const { result, output } = await contract.query.getMessage(
        address, // caller
        { gasLimit: -1 }, // auto-calculate gas
        address // parameter: account to fetch message for
      );

      if (result.isOk && output) {
        const message = output.toHuman() as any;
        return message?.Ok || null;
      }

      return null;
    } catch (err) {
      console.error('Query error:', err);
      return null;
    }
  };

  const postMessage = async (message: string, signer: any, address: string) => {
    if (!contract) throw new Error('Contract not initialized');

    // Create transaction
    const gasLimit = api.registry.createType('WeightV2', {
      refTime: 10000000000,
      proofSize: 100000,
    });

    // Sign and send
    return new Promise((resolve, reject) => {
      contract.tx
        .postMessage({ gasLimit }, message)
        .signAndSend(address, { signer }, (result) => {
          if (result.status.isInBlock) {
            console.log('In block:', result.status.asInBlock.toHex());
          }

          if (result.status.isFinalized) {
            console.log('Finalized:', result.status.asFinalized.toHex());
            resolve(result);
          }

          if (result.isError) {
            reject(new Error('Transaction failed'));
          }
        });
    });
  };

  return { contract, getMessage, postMessage };
};
```

**PostMessage.tsx:**
```typescript
import React, { useState } from 'react';
import { Box, Textarea, Button, useToast, VStack, Heading } from '@chakra-ui/react';
import { useContract } from '../hooks/useContract';

interface PostMessageProps {
  api: any;
  account: any;
  getInjector: (address: string) => Promise<any>;
}

export const PostMessage: React.FC<PostMessageProps> = ({ api, account, getInjector }) => {
  const [message, setMessage] = useState('');
  const [posting, setPosting] = useState(false);
  const { postMessage } = useContract(api);
  const toast = useToast();

  const handlePost = async () => {
    if (!message.trim()) {
      toast({ title: 'Please enter a message', status: 'warning' });
      return;
    }

    try {
      setPosting(true);
      const signer = await getInjector(account.address);
      await postMessage(message, signer, account.address);

      toast({
        title: 'Message posted!',
        description: 'Your message has been saved to the blockchain.',
        status: 'success',
      });

      setMessage('');
    } catch (err) {
      console.error(err);
      toast({
        title: 'Error posting message',
        description: err instanceof Error ? err.message : 'Unknown error',
        status: 'error',
      });
    } finally {
      setPosting(false);
    }
  };

  return (
    <Box borderWidth={1} borderRadius="lg" p={6}>
      <VStack spacing={4} align="stretch">
        <Heading size="md">Post a Message</Heading>
        <Textarea
          placeholder="Enter your message..."
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          rows={4}
        />
        <Button
          colorScheme="purple"
          onClick={handlePost}
          isLoading={posting}
          loadingText="Posting..."
        >
          Post to Blockchain
        </Button>
      </VStack>
    </Box>
  );
};
```

**MessageList.tsx:**
```typescript
import React, { useState } from 'react';
import { Box, Input, Button, VStack, HStack, Text, Heading, Card, CardBody } from '@chakra-ui/react';
import { useContract } from '../hooks/useContract';

interface MessageListProps {
  api: any;
}

export const MessageList: React.FC<MessageListProps> = ({ api }) => {
  const [address, setAddress] = useState('');
  const [message, setMessage] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const { getMessage } = useContract(api);

  const handleFetch = async () => {
    if (!address) return;

    try {
      setLoading(true);
      const msg = await getMessage(address);
      setMessage(msg);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box borderWidth={1} borderRadius="lg" p={6}>
      <VStack spacing={4} align="stretch">
        <Heading size="md">Read Messages</Heading>
        <HStack>
          <Input
            placeholder="Enter account address..."
            value={address}
            onChange={(e) => setAddress(e.target.value)}
          />
          <Button onClick={handleFetch} isLoading={loading} colorScheme="blue">
            Fetch
          </Button>
        </HStack>

        {message !== null && (
          <Card>
            <CardBody>
              <Text fontWeight="bold">From: {address.slice(0, 8)}...{address.slice(-8)}</Text>
              <Text mt={2}>{message || 'No message posted yet'}</Text>
            </CardBody>
          </Card>
        )}
      </VStack>
    </Box>
  );
};
```

**KEY POINTS TO EMPHASIZE:**
- Queries are free (read-only)
- Transactions cost gas and require signatures
- Gas limits must be specified for transactions
- signAndSend handles wallet integration automatically
- Transaction status goes: InBlock → Finalized
- Always handle loading and error states
- Toast notifications improve UX

**COMMON MISTAKES TO MENTION:**
- Don't forget to set proper gas limits
- Handle Option/Result types from contract (may be None/Err)
- Wait for isFinalized, not just isInBlock, for certainty
- Contract address must match deployed address exactly
- Metadata JSON must match deployed contract version

---

### 10:30 - 12:30 | Styling & Polish

**NARRATION:**
"Let's add some polish to make our DApp look professional. We'll improve the layout, add responsive design, and create a cohesive visual identity.

First, update our main App layout. Use Chakra UI's Grid component to create a responsive two-column layout: post on the left, read on the right. On mobile devices, these stack vertically. Add a proper header with the Etrid logo, app title, and wallet connection status in the top nav bar.

Let's add a color theme. Chakra UI makes this easy with theme customization. Create a theme object with Etrid's brand colors: purple as primary, with complementary gradients. Apply this to the ChakraProvider to give our entire app a consistent look.

For better UX, add loading skeletons while data fetches. Replace empty states with helpful illustrations and call-to-action messages. Show transaction history so users can see their recent posts. Add a timestamp to messages using blockchain block numbers.

Security indicators are important too. Show a green checkmark when connected to the correct network. Display gas estimates before transactions so users know the cost. Add a warning banner if the user is on mainnet instead of testnet.

Accessibility matters! Ensure all interactive elements are keyboard navigable. Add ARIA labels for screen readers. Use proper contrast ratios for text. Test with the keyboard only - can you navigate the entire app?

Finally, add some nice animations. Use Framer Motion for smooth transitions when components mount. Animate the transaction pending state with a pulsing indicator. Show a celebration effect when messages post successfully!

Let's see the finished product. Beautiful, responsive, accessible, and fully functional! This is a production-ready DApp."

**VISUAL CUES:**
- Before/after comparison: Basic layout vs. polished design
- Desktop vs. mobile responsive views
- Color theme palette display (Etrid purple scheme)
- Component library showcase:
  - Header with logo and wallet
  - Grid layout (2-column → stacked)
  - Loading skeletons
  - Toast notifications
  - Transaction history list
  - Empty states with illustrations
- Accessibility testing: keyboard navigation demo
- Animation showcase: transitions, loading states, success effects
- Dark mode toggle (bonus feature)
- Final DApp tour showing all features

**DEMO STEPS:**

**Layout Improvements:**
1. Update App.tsx with Grid layout
2. Add Header component
3. Add Footer component
4. Test responsive behavior (resize browser)

**Theme Customization:**
5. Create `src/theme.ts`
6. Define custom colors
7. Apply to ChakraProvider
8. Show color changes throughout app

**UX Enhancements:**
9. Add loading skeletons to MessageList
10. Add empty state illustrations
11. Add transaction history component
12. Add block timestamp display

**Security & Accessibility:**
13. Add network indicator
14. Add gas estimate display
15. Test keyboard navigation (Tab through everything)
16. Add ARIA labels
17. Test with screen reader (Voice Over demo)

**Animations:**
18. Install framer-motion (if not already)
19. Add fade-in animation to messages
20. Add pulse animation to pending state
21. Add success confetti effect (optional library)

**Final Testing:**
22. Walk through complete user flow
23. Test on mobile device or responsive mode
24. Verify all features work
25. Check console for errors (none!)

**CODE TO DISPLAY:**

**theme.ts:**
```typescript
import { extendTheme } from '@chakra-ui/react';

const theme = extendTheme({
  colors: {
    brand: {
      50: '#f5e9ff',
      100: '#d9c2ff',
      200: '#be9aff',
      300: '#a372ff',
      400: '#884aff',
      500: '#6e22e6', // Primary purple
      600: '#5519b3',
      700: '#3c1180',
      800: '#24094d',
      900: '#0b001a',
    },
  },
  fonts: {
    heading: '"Inter", -apple-system, BlinkMacSystemFont, sans-serif',
    body: '"Inter", -apple-system, BlinkMacSystemFont, sans-serif',
  },
  styles: {
    global: {
      body: {
        bg: 'gray.50',
      },
    },
  },
  components: {
    Button: {
      defaultProps: {
        colorScheme: 'brand',
      },
    },
  },
});

export default theme;
```

**Updated App.tsx (with layout):**
```typescript
import { Box, Grid, Heading, HStack, Container, Badge } from '@chakra-ui/react';
import { ChakraProvider } from '@chakra-ui/react';
import theme from './theme';

function App() {
  // ... hooks

  return (
    <ChakraProvider theme={theme}>
      {/* Header */}
      <Box bg="brand.500" color="white" px={4} py={3}>
        <Container maxW="container.xl">
          <HStack justify="space-between">
            <HStack>
              <Heading size="md">📝 Etrid Message Board</Heading>
              <Badge colorScheme="green">Testnet</Badge>
            </HStack>
            {selectedAccount ? (
              <Box fontSize="sm">
                {selectedAccount.address.slice(0, 6)}...{selectedAccount.address.slice(-6)}
              </Box>
            ) : (
              <Button size="sm" onClick={handleConnect}>Connect Wallet</Button>
            )}
          </HStack>
        </Container>
      </Box>

      {/* Main Content */}
      <Container maxW="container.xl" py={8}>
        <Grid templateColumns={{ base: '1fr', lg: 'repeat(2, 1fr)' }} gap={8}>
          {selectedAccount && (
            <PostMessage api={api} account={selectedAccount} getInjector={getInjector} />
          )}
          <MessageList api={api} />
        </Grid>
      </Container>

      {/* Footer */}
      <Box as="footer" textAlign="center" py={6} borderTop="1px" borderColor="gray.200">
        <Text fontSize="sm" color="gray.600">
          Built on Etrid • <Link href="https://docs.etrid.io">Docs</Link>
        </Text>
      </Box>
    </ChakraProvider>
  );
}
```

**Animation Example:**
```typescript
import { motion } from 'framer-motion';

const MotionCard = motion(Card);

<MotionCard
  initial={{ opacity: 0, y: 20 }}
  animate={{ opacity: 1, y: 0 }}
  transition={{ duration: 0.3 }}
>
  <CardBody>
    {message}
  </CardBody>
</MotionCard>
```

**KEY POINTS TO EMPHASIZE:**
- Professional design builds user trust
- Responsive design is essential (mobile-first!)
- Theme consistency across all components
- Loading states prevent confusion
- Accessibility is not optional
- Animations should enhance, not distract
- Test on real devices, not just browser resize

**COMMON MISTAKES TO MENTION:**
- Don't sacrifice accessibility for aesthetics
- Don't over-animate (can be distracting)
- Don't forget to test keyboard navigation
- Don't ignore mobile users (60%+ of traffic)
- Don't use automatic animations for critical actions

---

### 12:30 - 14:00 | Deployment & Best Practices

**NARRATION:**
"Your DApp is ready! Let's deploy it so anyone can use it. We'll use Vercel for free, automated deployments with CI/CD.

First, push your code to GitHub. Initialize git in your project: git init. Create a new repository on GitHub. Add your files: git add -A. Commit: git commit -m 'Initial DApp implementation'. Push to GitHub: git push -u origin main.

Now deploy to Vercel. Go to vercel.com and sign in with GitHub. Click 'New Project' and import your repository. Vercel automatically detects it's a React app. The default settings are perfect - no configuration needed! Click 'Deploy'.

Vercel builds your app, running npm run build. This compiles your TypeScript, bundles all JavaScript, and optimizes assets. After a minute or two, your deployment is live! Vercel gives you a URL: your-app.vercel.app. Click it to see your DApp running!

Every time you push to GitHub, Vercel automatically deploys the new version. This is called continuous deployment. It's incredibly powerful for rapid iteration.

Now let's discuss best practices for production DApps. First, environment variables: never hardcode RPC endpoints or contract addresses. Use .env files and environment variables. Create separate configs for testnet and mainnet.

Second, error handling: wrap all blockchain calls in try-catch blocks. Show meaningful error messages to users, not technical jargon. Log errors to a service like Sentry for monitoring.

Third, transaction state management: persist pending transactions to localStorage so users can refresh without losing track. Show clear status updates: pending, confirming, confirmed, or failed.

Fourth, performance: cache blockchain queries using React Query or SWR. Don't query the blockchain on every render! Use websocket subscriptions for real-time updates instead of polling.

Fifth, security: validate all inputs before sending to blockchain. Warn users about transaction costs. Never execute transactions without explicit user confirmation. Verify you're connected to the correct network.

Finally, testing: write unit tests for your components using Jest and React Testing Library. Test wallet connection flows, transaction submissions, and error states. Mock blockchain responses for predictable tests.

You now have a complete, production-ready DApp! Congratulations!"

**VISUAL CUES:**
- Git workflow diagram: Local → GitHub → Vercel → Live
- GitHub repository creation screen recording
- Vercel deployment dashboard
- Build logs scrolling (successful)
- Live DApp URL with celebration confetti
- Environment variables configuration panel
- Error handling flow chart
- Performance comparison: Before (slow) vs. After (cached)
- Security checklist with checkmarks
- Testing pyramid visualization
- Final production DApp showcase

**DEMO STEPS:**

**Git & GitHub:**
1. Terminal: `git init`
2. Create `.gitignore` (add node_modules, .env)
3. Terminal: `git add -A`
4. Terminal: `git commit -m "Initial commit"`
5. Browser: Create new GitHub repo
6. Terminal: `git remote add origin <repo-url>`
7. Terminal: `git push -u origin main`
8. Show GitHub repository with code

**Vercel Deployment:**
9. Browser: Go to vercel.com
10. Click "Sign up with GitHub"
11. Authorize Vercel
12. Click "New Project"
13. Import your repository
14. Show auto-detected settings (Framework: Create React App)
15. Click "Deploy"
16. Watch build logs
17. Show "Congratulations" screen
18. Click "Visit" to see live app
19. Copy deployment URL
20. Test app in production

**Environment Variables:**
21. Create `.env.local` file
22. Add: `REACT_APP_CONTRACT_ADDRESS=5F...`
23. Add: `REACT_APP_RPC_ENDPOINT=wss://...`
24. Update code to use process.env values
25. Add environment variables in Vercel dashboard
26. Redeploy

**Best Practices Examples:**
27. Show error boundary component
28. Show React Query usage for caching
29. Show transaction persistence with localStorage
30. Show network validation logic

**CODE TO DISPLAY:**

**Environment Variables (.env.local):**
```bash
REACT_APP_CONTRACT_ADDRESS=5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
REACT_APP_RPC_ENDPOINT=wss://rpc-testnet.etrid.io
REACT_APP_NETWORK_NAME=Etrid Testnet
```

**Using Environment Variables:**
```typescript
const CONTRACT_ADDRESS = process.env.REACT_APP_CONTRACT_ADDRESS!;
const RPC_ENDPOINT = process.env.REACT_APP_RPC_ENDPOINT || 'wss://rpc-testnet.etrid.io';

// Validate network
if (api) {
  const chain = await api.rpc.system.chain();
  if (chain.toString() !== process.env.REACT_APP_NETWORK_NAME) {
    throw new Error('Connected to wrong network!');
  }
}
```

**Error Boundary:**
```typescript
class ErrorBoundary extends React.Component {
  state = { hasError: false };

  static getDerivedStateFromError(error: Error) {
    return { hasError: true };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    console.error('DApp error:', error, errorInfo);
    // Log to Sentry or other service
  }

  render() {
    if (this.state.hasError) {
      return <Alert status="error">Something went wrong. Please refresh.</Alert>;
    }
    return this.props.children;
  }
}
```

**Caching with React Query:**
```typescript
import { useQuery } from '@tanstack/react-query';

const useMessage = (address: string) => {
  return useQuery(['message', address], () => getMessage(address), {
    staleTime: 30000, // Cache for 30 seconds
    cacheTime: 300000, // Keep in cache for 5 minutes
  });
};
```

**Deployment Commands:**
```bash
# Build for production
npm run build

# Test production build locally
npx serve -s build

# Deploy to Vercel
vercel --prod

# Or push to GitHub for automatic deployment
git push origin main
```

**KEY POINTS TO EMPHASIZE:**
- Vercel offers free hosting for DApps
- Automatic deployments save time
- Environment variables separate testnet/mainnet configs
- Error handling is critical for UX
- Caching improves performance dramatically
- Testing catches bugs before users do
- Security should be paranoid - validate everything

**COMMON MISTAKES TO MENTION:**
- Don't commit .env files to git (use .gitignore)
- Don't deploy without testing build locally first
- Don't skip error handling to save time
- Don't query blockchain on every render (use caching)
- Don't forget to test on mainnet config before launch

---

### 14:00 - 15:00 | Advanced Features & Next Steps

**NARRATION:**
"You've built a complete DApp! Let's explore advanced features you can add and where to go from here.

First, real-time updates. Right now, users must manually refresh to see new messages. We can add WebSocket subscriptions to listen for blockchain events. When someone posts a message, our app automatically updates for all users. This uses the Polkadot.js subscription API.

Second, transaction history. Display all messages posted by the connected user with timestamps and transaction hashes. Link to a block explorer so users can verify their transactions on-chain. This builds trust and transparency.

Third, gas estimation. Before submitting transactions, query the contract to estimate gas costs. Show the user the expected fee in ETR and USD. Let them adjust gas limits if needed. This prevents failed transactions and surprises.

Fourth, notifications. Integrate browser notifications to alert users when their transactions confirm. This is especially helpful for slow networks where confirmation takes minutes.

Fifth, internationalization. Use libraries like react-i18n to support multiple languages. Blockchain is global - your DApp should be too! Start with English, but add Spanish, Chinese, Japanese, and others based on your user base.

Sixth, analytics. Track how users interact with your DApp using privacy-respecting tools like Plausible or Fathom. Know which features are popular and where users struggle. Never track wallet addresses or personal data.

For more advanced projects, consider: building a DAO governance interface where users vote on proposals, creating an NFT marketplace with minting and trading features, developing a DeFi protocol with liquidity pools and staking, or making a decentralized social network with profiles and messaging.

Resources for continued learning: The Polkadot.js documentation has comprehensive API references. Substrate Stack Exchange is perfect for Q&A. The ink! community publishes example contracts you can learn from. And join hackathons to build with other developers!

Check out these related tutorials: 'Advanced ink! Patterns' covers complex contract architectures. 'Optimizing DApp Performance' teaches you to build blazing-fast interfaces. 'DApp Security Audit Checklist' helps you prepare for professional audits. 'Multi-Chain DApps' shows how to interact with multiple blockchains.

You're now a full-stack blockchain developer! You can write smart contracts, deploy them, build beautiful interfaces, and ship production DApps. The possibilities are endless.

Thank you for following along with this tutorial series! From creating your first wallet to deploying full DApps, you've learned the complete Etrid development stack. We can't wait to see what you build. Share your projects in our Discord community and help others on their journey.

Welcome to the future of decentralized applications! Happy building!"

**VISUAL CUES:**
- Feature showcase: real-time updates animation
- Transaction history component mockup
- Gas estimation UI with slider
- Browser notification example
- Multi-language selector
- Analytics dashboard (anonymized data)
- Advanced project gallery:
  - DAO governance interface
  - NFT marketplace
  - DeFi protocol
  - Social network
- Learning resource cards with links
- Hackathon event showcase
- Related tutorials carousel
- Community showcase (user-submitted DApps)
- Call-to-action: Join Discord, Follow Twitter, Star GitHub
- Final thank you screen with Etrid logo

**CODE EXAMPLES:**

**Real-Time Updates (WebSocket Subscription):**
```typescript
useEffect(() => {
  if (!api || !contract) return;

  const unsubscribe = contract.events.MessagePosted((event) => {
    const [author, message] = event.data;
    console.log('New message:', { author: author.toString(), message: message.toString() });

    // Update UI
    queryClient.invalidateQueries(['messages']);
  });

  return () => {
    if (unsubscribe) unsubscribe();
  };
}, [api, contract]);
```

**Gas Estimation:**
```typescript
const estimateGas = async (message: string) => {
  const gasLimit = await contract.query.postMessage(
    account.address,
    { gasLimit: -1 },
    message
  ).then(result => result.gasRequired);

  const gasCost = api.registry.createType('Balance', gasLimit.refTime.toBn());
  const costInETR = gasCost.div(1_000_000_000_000).toNumber();

  return { gasLimit, costInETR };
};
```

**Browser Notifications:**
```typescript
const notifyUser = (title: string, message: string) => {
  if ('Notification' in window && Notification.permission === 'granted') {
    new Notification(title, {
      body: message,
      icon: '/etrid-logo.png',
    });
  }
};

// Request permission on load
useEffect(() => {
  if ('Notification' in window && Notification.permission === 'default') {
    Notification.requestPermission();
  }
}, []);
```

**RESOURCES TO DISPLAY:**
```
📚 Documentation:
- Polkadot.js API Docs: https://polkadot.js.org/docs/
- ink! Documentation: https://ink.substrate.io
- Etrid Developer Portal: https://docs.etrid.io/developers
- Substrate Stack Exchange: https://substrate.stackexchange.com

🎓 Advanced Tutorials:
- Advanced ink! Patterns
- Optimizing DApp Performance
- DApp Security Audit Checklist
- Multi-Chain DApps
- Building DAOs on Etrid

💡 Project Ideas:
- DAO Governance Interface
- NFT Marketplace
- DeFi Lending Protocol
- Decentralized Social Network
- On-Chain Gaming Platform

🏆 Hackathons:
- Etrid Global Hackathon (Annual)
- Polkadot Hackathons (Quarterly)
- ETHGlobal (Multi-chain tracks)

💬 Community:
- Discord: https://discord.gg/etrid (#dapp-devs channel)
- Twitter: @EtridDevs
- GitHub: https://github.com/etrid
- Dev Office Hours: Tuesdays & Thursdays 2pm UTC
```

**KEY POINTS TO EMPHASIZE:**
- Real-time features greatly improve UX
- Transparency builds user trust
- Always estimate gas before transactions
- Internationalization expands your reach
- Analytics help you improve
- Endless possibilities for building
- Community is here to support you
- Keep learning and experimenting

---

## Production Notes

### Visual Assets Needed

**Static Graphics:**
1. DApp architecture diagram
2. Data flow visualization
3. Before/after UI comparison
4. Responsive design showcase
5. Component library
6. Color theme palette
7. Feature roadmap
8. Project ideas gallery
9. Resource links end card
10. Community showcase

**Screen Recordings:**
1. Complete dev environment setup
2. VS Code development workflow
3. Browser console debugging
4. Polkadot.js extension interaction
5. Transaction flow (complete)
6. Responsive design testing
7. Accessibility testing (keyboard navigation)
8. Vercel deployment process
9. Live DApp demonstration
10. Advanced features preview

**Code Examples:**
1. All hooks (useEtridApi, useWallet, useContract)
2. All components (PostMessage, MessageList)
3. Theme configuration
4. Environment variables
5. Error handling patterns
6. Performance optimizations
7. Testing examples

### Demo Requirements

**Development Environment:**
- Node.js 18+ installed
- VS Code with extensions:
  - ESLint
  - Prettier
  - ES7+ React snippets
- Polkadot.js extension installed
- Git installed

**Accounts & Access:**
- GitHub account
- Vercel account
- Etrid wallet with testnet ETR
- Contract deployed from Tutorial 04

**Network Requirements:**
- Stable internet connection
- Access to Etrid testnet RPC
- WebSocket support

### Editing Notes

**Pacing:**
- Code walkthroughs: slightly faster than real-time
- Deployment waiting: timelapse with progress indicator
- Transaction confirmations: real-time (builds anticipation)
- Final demo: normal speed, showcase all features

**Graphics:**
- Split screen for code + browser
- Picture-in-picture for terminal when needed
- Zoom on important UI interactions
- Highlight code changes with subtle animation
- Use consistent color coding throughout

**Audio:**
- Enthusiastic but professional narration
- Background music: modern, techy, upbeat
- Sound effects: subtle success sounds
- Keyboard typing sounds during code sections (optional)

**Accessibility:**
- Closed captions for all narration
- Code captions showing what's being typed
- High contrast code themes
- Large, readable fonts (min 14pt)
- Audio descriptions of visual elements

---

## Target Metrics

**Engagement Goals:**
- Watch time: 70%+ (complete tutorial)
- DApp deployments: 30%+ of viewers
- Community joins: 25%+ join Discord
- GitHub stars: 15%+ star example repo

**Educational Goals:**
- 95%+ understand DApp architecture
- 90%+ successfully connect to blockchain
- 80%+ integrate wallet connection
- 70%+ read from smart contract
- 60%+ write to smart contract
- 50%+ deploy to production

---

**Tutorial Series Complete!**

You've completed all 5 tutorials:
1. ✅ Getting Started with Etrid
2. ✅ Running a Validator
3. ✅ Staking as a Nominator
4. ✅ Deploying Smart Contracts
5. ✅ Building DApps

**Congratulations on becoming an Etrid developer!**
