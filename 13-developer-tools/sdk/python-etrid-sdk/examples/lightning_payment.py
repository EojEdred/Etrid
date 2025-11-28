"""
Example: Lightning-Bloc Payment Channel

Demonstrates opening a payment channel, sending payments, and closing the channel.
"""

import asyncio
from etrid_sdk import EtridClient
from etrid_sdk.wrappers import LightningBlocWrapper


async def main():
    # Connect to Ëtrid node
    print("Connecting to Ëtrid node...")
    client = EtridClient("ws://127.0.0.1:9944")
    
    if not client.is_connected():
        print("Error: Could not connect to node")
        return
        
    print(f"Connected to {client.get_chain()}")
    print(f"Block: {client.get_block_number()}")
    
    # Create keypairs
    alice = client.create_keypair()
    bob = client.create_keypair()
    
    print(f"\nAlice: {alice.ss58_address}")
    print(f"Bob: {bob.ss58_address}")
    
    # Initialize Lightning-Bloc wrapper
    lightning = LightningBlocWrapper(client.api)
    
    # Open payment channel
    print("\n1. Opening payment channel...")
    channel = await lightning.open_channel(
        alice,
        recipient=bob.ss58_address,
        amount=1000 * 10**18  # 1000 ÉTR
    )
    
    print(f"   Channel ID: {channel['channel_id']}")
    print(f"   Capacity: {channel['amount'] / 10**18} ÉTR")
    print(f"   TX: {channel['tx_hash']}")
    
    # Get channel info
    print("\n2. Checking channel status...")
    channel_info = await lightning.get_channel(channel['channel_id'])
    print(f"   From: {channel_info['from']}")
    print(f"   To: {channel_info['to']}")
    print(f"   Balance: {channel_info['balance'] / 10**18} ÉTR")
    print(f"   Status: {channel_info['status']}")
    
    # Send payment through channel
    print("\n3. Sending payment...")
    payment = await lightning.send_payment(
        alice,
        channel_id=channel['channel_id'],
        amount=100 * 10**18  # 100 ÉTR
    )
    print(f"   Sent 100 ÉTR")
    print(f"   TX: {payment['tx_hash']}")
    
    # Close channel
    print("\n4. Closing channel...")
    tx_hash = await lightning.close_channel(
        alice,
        channel_id=channel['channel_id']
    )
    print(f"   Channel closed")
    print(f"   TX: {tx_hash}")
    
    # Clean up
    client.close()
    print("\n✅ Example completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
