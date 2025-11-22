"use client"

import { useState } from 'react'
import Image from 'next/image'
import { ArrowLeft, ExternalLink, Share2, MoreVertical } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { NFTAttributeList } from '@/components/nft/nft-attribute-list'
import { BidHistory } from '@/components/nft/bid-history'
import { useNFTDetail } from '@/hooks/use-nft-detail'

interface NFTDetailScreenProps {
  contractAddress: string
  tokenId: string
  userAddress?: string
  onBack: () => void
}

export function NFTDetailScreen({
  contractAddress,
  tokenId,
  userAddress,
  onBack,
}: NFTDetailScreenProps) {
  const { nft, offers, isListed, listNFT, cancelListing, transferNFT, makeOffer, acceptOffer } =
    useNFTDetail(contractAddress, tokenId)

  const [showListDialog, setShowListDialog] = useState(false)
  const [listPrice, setListPrice] = useState('')

  if (!nft) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin w-8 h-8 border-4 border-primary border-t-transparent rounded-full" />
      </div>
    )
  }

  const isOwner = nft.owner === userAddress

  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 bg-background/80 backdrop-blur-lg border-b p-4">
        <div className="flex items-center justify-between">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex gap-2">
            <Button variant="ghost" size="icon">
              <Share2 className="w-5 h-5" />
            </Button>
            <Button variant="ghost" size="icon">
              <MoreVertical className="w-5 h-5" />
            </Button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main>
        {/* NFT Image */}
        <div className="relative aspect-square bg-muted">
          <Image
            src={nft.image_url || '/placeholder-nft.png'}
            alt={nft.name}
            fill
            className="object-cover"
            priority
          />
          <Badge className="absolute top-4 left-4">{nft.chain}</Badge>
        </div>

        {/* NFT Info */}
        <div className="p-4 space-y-6">
          {/* Collection & Name */}
          {nft.collection && (
            <div>
              <p className="text-sm text-muted-foreground">{nft.collection.name}</p>
              {nft.collection.verified && (
                <Badge variant="secondary" className="ml-2 text-xs">
                  Verified
                </Badge>
              )}
            </div>
          )}

          <div>
            <h1 className="text-2xl font-bold">{nft.name}</h1>
            {nft.description && (
              <p className="text-muted-foreground mt-2">{nft.description}</p>
            )}
          </div>

          {/* Owner Info */}
          <Card className="p-4">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-xs text-muted-foreground">Owned by</p>
                <p className="font-semibold text-sm mt-1">
                  {isOwner ? 'You' : `${nft.owner.slice(0, 6)}...${nft.owner.slice(-4)}`}
                </p>
              </div>
              {isListed && (
                <div className="text-right">
                  <p className="text-xs text-muted-foreground">Listed for</p>
                  <p className="font-bold text-lg">2.5 ETRID</p>
                </div>
              )}
            </div>
          </Card>

          {/* Actions */}
          {isOwner ? (
            <div className="grid grid-cols-2 gap-3">
              {isListed ? (
                <>
                  <Button variant="outline" onClick={() => cancelListing()}>
                    Cancel Listing
                  </Button>
                  <Button>View Offers</Button>
                </>
              ) : (
                <>
                  <Button variant="outline" onClick={() => transferNFT('0x...')}>
                    Transfer
                  </Button>
                  <Button onClick={() => setShowListDialog(true)}>List for Sale</Button>
                </>
              )}
            </div>
          ) : (
            <div className="grid grid-cols-2 gap-3">
              {isListed ? (
                <>
                  <Button variant="outline" onClick={() => makeOffer(2.0)}>
                    Make Offer
                  </Button>
                  <Button>Buy Now</Button>
                </>
              ) : (
                <Button className="col-span-2" disabled>
                  Not Listed
                </Button>
              )}
            </div>
          )}

          {/* Tabs */}
          <Tabs defaultValue="attributes" className="w-full">
            <TabsList className="w-full grid grid-cols-3">
              <TabsTrigger value="attributes">Attributes</TabsTrigger>
              <TabsTrigger value="offers">Offers</TabsTrigger>
              <TabsTrigger value="activity">Activity</TabsTrigger>
            </TabsList>

            <TabsContent value="attributes" className="mt-4">
              <NFTAttributeList
                attributes={nft.metadata.attributes}
                showRarity
                showFloorPrice
              />
            </TabsContent>

            <TabsContent value="offers" className="mt-4">
              {offers.length > 0 ? (
                <div className="space-y-2">
                  {offers.map((offer) => (
                    <Card key={offer.id} className="p-4 flex items-center justify-between">
                      <div>
                        <p className="font-semibold">{offer.amount} ETRID</p>
                        <p className="text-sm text-muted-foreground">
                          From {offer.bidder.slice(0, 6)}...{offer.bidder.slice(-4)}
                        </p>
                      </div>
                      {isOwner && (
                        <Button size="sm" onClick={() => acceptOffer(offer.id)}>
                          Accept
                        </Button>
                      )}
                    </Card>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  No offers yet
                </div>
              )}
            </TabsContent>

            <TabsContent value="activity" className="mt-4">
              {nft.transfer_history && nft.transfer_history.length > 0 ? (
                <div className="space-y-3">
                  {nft.transfer_history.map((activity, index) => (
                    <div key={index} className="flex items-start gap-3">
                      <div className="flex-1">
                        <p className="font-semibold text-sm">
                          {activity.price ? 'Sale' : 'Transfer'}
                        </p>
                        <p className="text-xs text-muted-foreground mt-1">
                          {new Date(activity.timestamp).toLocaleDateString()}
                        </p>
                      </div>
                      {activity.price && (
                        <p className="font-bold">{activity.price} ETRID</p>
                      )}
                      <a
                        href={`https://explorer.etrid.io/tx/${activity.tx_hash}`}
                        target="_blank"
                        rel="noopener noreferrer"
                      >
                        <ExternalLink className="w-4 h-4 text-muted-foreground" />
                      </a>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 text-muted-foreground">
                  No activity yet
                </div>
              )}
            </TabsContent>
          </Tabs>
        </div>
      </main>
    </div>
  )
}
